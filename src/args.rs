use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    num::ParseIntError,
    time::Duration,
};

use clap::Parser;
use derive_more::Display;
use exn::{Result, bail};
use hickory_proto::{rr::RecordType, serialize::binary::DecodeError};

// Arguments of the original tool
// -c: disable local caching, default enabled
// -C: enable negative caching, default disabled
// -e: disable EDNS0, default enabled
// TODO: -E <size>: set the EDNS0 size, default 1500
// -o: enable the overview of received answers, default disabled
// -q <querytype>: set the query type of the DNS requests, default A
// -r <retries>: set the number of retries for DNS requests, default 3
// -s <server>: send the first request to this server, default localhost
//             If the value is ".", use A.ROOT-SERVERS.NET.
// -t <maximum timeout>: set the time limit for each try
// TODO: -v: verbose
// -S <ip address>: use this source address.
// -4: do not query IPv6 servers

/// The command-line arguments
#[expect(clippy::struct_excessive_bools, reason = "Those are flags, not states")]
#[derive(Clone, Debug, Eq, Parser, PartialEq)]
#[command(version, about)]
pub struct Args {
    /// The domain to query
    pub domain: String,

    /// Disable the cache of positive responses (default: enabled)
    #[arg(short = 'c', long)]
    pub no_positive_cache: bool,

    /// Enable the cache of negative responses (default: disabled)
    #[arg(short = 'C', long)]
    pub negative_cache: bool,

    /// Disable EDNS0 (default: enabled)
    #[arg(short = 'e', long)]
    pub no_edns0: bool,

    /// Show an overview of the received answers (default: disabled)
    #[arg(short = 'o', long)]
    pub overview: bool,

    /// The record type to query (A, AAAA, NS, ...)
    #[arg(short = 'q', long, default_value = "A", value_parser = parse_record_type)]
    pub query_type: RecordType,

    /// The number of retries for each DNS request
    #[arg(short = 'r', long, default_value = "3")]
    pub retries: usize,

    /// Start the query at the given DNS server.
    /// For an IP address, use that address.
    /// For a hostname, use all the IP addresses of that hostname.
    /// For ".", use all the root servers.
    #[arg(short, long, default_value = "a.root-servers.net")]
    pub server: String,

    /// The time limit, in seconds, for each try
    #[arg(short = 't', long, default_value = "5", value_parser = parse_duration)]
    pub timeout: Duration,

    /// The source address of the DNS queries
    #[arg(short = 'S', long)]
    pub source_address: Option<IpAddr>,

    /// Use only IPv6 for DNS queries
    #[arg(short = '6', long)]
    pub ipv6: bool,

    /// Use only IPv4 for DNS queries
    #[arg(short = '4', long)]
    pub ipv4: bool,

    /// Use only TCP for DNS queries
    #[arg(short = 'T', long)]
    pub tcp: bool,
}

/// The errors of argument parsing
#[derive(Debug, Display)]
pub enum ArgsError {
    /// IPv6-only mode requested but source address is IPv4
    #[display("Cannot use IPv6-only queries with the IPv4 source address {ipv4}")]
    Ipv6WithIpv4Source {
        /// The IPv4 source address
        ipv4: Ipv4Addr,
    },
    /// IPv4-only mode requested but source address is IPv6
    #[display("Cannot use IPv4-only queries with the IPv6 source address {ipv6}")]
    Ipv4WithIpv6Source {
        /// The IPv6 source address
        ipv6: Ipv6Addr,
    },
}

impl std::error::Error for ArgsError {}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", skip(s), ret, err(level = "debug"))
)]
/// Parse a record type
fn parse_record_type(s: &str) -> std::result::Result<RecordType, DecodeError> {
    #[cfg(feature = "tracing")]
    tracing::debug!(?s);
    s.to_ascii_uppercase().parse()
}

impl Args {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self), err(level = "debug"))
    )]
    /// Check that the arguments are compatible
    pub fn validate(&mut self) -> Result<(), ArgsError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?self);
        match self.source_address {
            Some(IpAddr::V4(ip)) => {
                if self.ipv6 {
                    bail!(ArgsError::Ipv6WithIpv4Source { ipv4: ip });
                }
                // Use IPv4 for all queries. IPv6 queries from an IPv4 source cause protocol errors.
                self.ipv4 = true;
            },
            Some(IpAddr::V6(ip)) => {
                if self.ipv4 {
                    bail!(ArgsError::Ipv4WithIpv6Source { ipv6: ip });
                }
                // Use IPv6 for all queries. IPv4 queries from an IPv6 source cause protocol errors.
                self.ipv6 = true;
            },
            None => (),
        }

        Ok(())
    }
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", skip(src), ret, err(level = "debug"))
)]
/// Parse a number of seconds into a `Duration`
fn parse_duration(src: &str) -> std::result::Result<Duration, ParseIntError> {
    #[cfg(feature = "tracing")]
    tracing::debug!(?src);
    src.parse::<u64>().map(Duration::from_secs)
}

#[cfg(test)]
mod tests {
    #![expect(clippy::expect_used, reason = "test")]

    use insta::assert_debug_snapshot;
    use rstest::{Context, rstest};

    use super::*;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(ctx, input))
    )]
    #[rstest]
    #[case::default_values(vec!["test", "example.com"])]
    #[case::all_flags(vec![
            "test",
            "-c", // no_positive_cache
            "-C", // negative_cache
            "-e", // edns0 disabled
            "-o", // overview enabled
            "-q",
            "NS", // query_type: NS
            "-r",
            "5", // retries: 5
            "-s",
            "8.8.8.8", // server: 8.8.8.8
            "-t",
            "10", // timeout: 10 seconds
            "-S",
            "192.168.0.1", // source_address: 192.168.0.1
            "-6",          // force IPv6
            "-T",          // use TCP
            "example.com",
        ])]
    #[case::ipv4_flag(vec!["test", "example.com", "-4"])]
    #[case::server_override(vec!["test", "-s", "1.1.1.1", "example.com"])]
    #[case::query_type(vec!["test", "example.com", "-q", "AAAA"])]
    #[case::source_v4(vec!["test", "example.com", "-S", "1.1.1.1"])]
    #[case::source_v6(vec!["test", "example.com", "-S", "2001:db8::1"])]
    #[trace]
    fn args(
        #[notrace]
        #[context]
        ctx: Context,
        #[case] input: Vec<&str>,
    ) {
        let args = Args::parse_from(input);

        assert_debug_snapshot!(
            format!("{}_{}", ctx.name, ctx.description.unwrap_or_default()),
            args
        );
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(record))
    )]
    #[rstest]
    #[case("soa")]
    #[case("SoA")]
    #[case("a")]
    #[case("A")]
    #[case("aAaA")]
    #[case("Mx")]
    #[case("dS")]
    #[trace]
    fn record_type(#[case] record: &str) {
        let args = Args::parse_from(["test", "-q", record, "example.com"]);

        assert_debug_snapshot!(format!("record_{record}"), args);
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(ctx, input))
    )]
    #[rstest]
    #[case::invalid_query_type(vec!["test", "example.com", "-q", "INVALID"])]
    #[case::invalid_retries(vec!["test", "example.com", "-r", "INVALID"])]
    #[case::invalid_ipv4(vec!["test", "example.com", "-S", "5432.5432.234.12"])]
    #[case::invalid_ipv6(vec!["test", "example.com", "-S", "2a0x::1"])]
    #[trace]
    fn bad_args(
        #[notrace]
        #[context]
        ctx: Context,
        #[case] input: Vec<&str>,
    ) {
        let args = Args::try_parse_from(input).expect_err("input should be rejected by the parser");

        assert_debug_snapshot!(
            format!("{}_{}", ctx.name, ctx.description.unwrap_or_default()),
            args
        );
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(ctx, input))
    )]
    #[rstest]
    #[case::source_v4_plus_ipv6_flag(vec!["test", "example.com", "-6", "-S", "1.1.1.1"])]
    #[case::source_v6_plus_ipv4_flag(vec!["test", "example.com", "-4", "-S", "2001:db8::1"])]
    #[trace]
    fn not_valid(
        #[notrace]
        #[context]
        ctx: Context,
        #[case] input: Vec<&str>,
    ) {
        let mut args = Args::parse_from(input);
        let validated = args
            .validate()
            .expect_err("incompatible flags should fail validation");

        assert_debug_snapshot!(
            format!("{}_{}", ctx.name, ctx.description.unwrap_or_default()),
            validated
        );
    }
}
