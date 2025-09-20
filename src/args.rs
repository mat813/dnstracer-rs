use clap::Parser;
use eyre::{Result, WrapErr as _, bail};
use hickory_proto::rr::RecordType;
use std::{net::IpAddr, str::FromStr as _, time::Duration};

// Original arguments
// -c: disable local caching, default enabled
// -C: enable negative caching, default disabled
// -e: disable EDNS0, default enabled
// TODO: -E <size>: set EDNS0 size, default 1500
// -o: enable overview of received answers, default disabled
// -q <querytype>: query-type to use for the DNS requests, default A
// -r <retries>: amount of retries for DNS requests, default 3
// -s <server>: use this server for the initial request, default localhost
//             If . is specified, A.ROOT-SERVERS.NET will be used.
// -t <maximum timeout>: Limit time to wait per try
// TODO: -v: verbose
// -S <ip address>: use this source address.
// -4: don't query IPv6 servers

/// Our command line arguments
#[expect(clippy::struct_excessive_bools, reason = "Those are flags, not states")]
#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(version, about)]
pub struct Args {
    /// The domain to query
    pub domain: String,

    /// disable positive response caching, default enabled
    #[arg(short = 'c', long)]
    pub no_positive_cache: bool,

    /// enable negative response caching, default disabled
    #[arg(short = 'C', long)]
    pub negative_cache: bool,

    /// disable EDNS0, default enabled
    #[arg(short = 'e', long)]
    pub no_edns0: bool,

    /// enable overview of received answers, default disabled
    #[arg(short = 'o', long)]
    pub overview: bool,

    /// The type of record (A, AAAA, NS ...)
    #[arg(short = 'q', long, default_value = "A", value_parser = parse_record_type)]
    pub query_type: RecordType,

    /// amount of retries for DNS requests, default 3
    #[arg(short = 'r', long, default_value = "3")]
    pub retries: usize,

    /// Start the query at the given DNS server
    /// If an ip is given, it will be used
    /// If a hostname is given, all its ip will be used
    /// If "." is specified, all root servers will be used
    #[arg(short, long, default_value = "a.root-servers.net")]
    pub server: String,

    /// Limit time to wait per try
    #[arg(short = 't', long, default_value = "5", value_parser = parse_duration)]
    pub timeout: Duration,

    /// use this source address.
    #[arg(short = 'S', long)]
    pub source_address: Option<IpAddr>,

    /// Force using IPv6 for DNS queries (no IPv4)
    #[arg(short = '6', long)]
    pub ipv6: bool,

    /// Force using IPv4 for DNS queries (no IPv6)
    #[arg(short = '4', long)]
    pub ipv4: bool,

    /// Force using TCP for DNS queries
    #[arg(short = 'T', long)]
    pub tcp: bool,
}

fn parse_record_type(s: &str) -> Result<RecordType> {
    Ok(RecordType::from_str(&s.to_ascii_uppercase())?)
}

impl Args {
    /// Perform some validation on arguments
    pub fn validate(&mut self) -> Result<()> {
        match self.source_address {
            Some(IpAddr::V4(ip)) => {
                if self.ipv6 {
                    bail!("Cannot use IPv6 only queries with an ipv4 source address ({ip})");
                }
                // Also, force IPv4 queries everywhere, otherwise we'd get protocol errors
                self.ipv4 = true;
            }
            Some(IpAddr::V6(ip)) => {
                if self.ipv4 {
                    bail!("Cannot use IPv4 only queries with an ipv6 source address ({ip})");
                }
                // Also, force IPv6 queries everywhere, otherwise we'd get protocol errors
                self.ipv6 = true;
            }
            None => (),
        }

        Ok(())
    }
}

/// Duration parser for args
fn parse_duration(src: &str) -> Result<Duration> {
    src.parse::<u64>()
        .map(Duration::from_secs)
        .wrap_err_with(|| format!("Invalid duration: {src}"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used, reason = "test")]

    use super::*;
    use insta::assert_debug_snapshot;
    use rstest::rstest;

    #[rstest]
    #[case("default_values", vec!["test", "example.com"])]
    #[case("all_flags", vec![
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
    #[case("ipv4_flag", vec!["test", "example.com", "-4"])]
    #[case("server_override", vec!["test", "-s", "1.1.1.1", "example.com"])]
    #[case("query_type", vec!["test", "example.com", "-q", "AAAA"])]
    #[case("source_v4", vec!["test", "example.com", "-S", "1.1.1.1"])]
    #[case("source_v6", vec!["test", "example.com", "-S", "2001:db8::1"])]
    #[trace]
    fn args(#[case] name: &str, #[case] input: Vec<&str>) {
        let args = Args::parse_from(input);

        assert_debug_snapshot!(format!("args_{name}"), args);
    }

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

    #[rstest]
    #[case("invalid_query_type", vec!["test", "example.com", "-q", "INVALID"])]
    #[case("invalid_retries", vec!["test", "example.com", "-r", "INVALID"])]
    #[case("invalid_ipv4", vec!["test", "example.com", "-S", "5432.5432.234.12"])]
    #[case("invalid_ipv6", vec!["test", "example.com", "-S", "2a0x::1"])]
    #[trace]
    fn bad_args(#[case] name: &str, #[case] input: Vec<&str>) {
        let args = Args::try_parse_from(input).unwrap_err();

        assert_debug_snapshot!(format!("bad_{name}"), args);
    }

    #[rstest]
    #[case("source_v4_plus_ipv6_flag", vec!["test", "example.com", "-6", "-S", "1.1.1.1"])]
    #[case("source_v6_plus_ipv4_flag", vec!["test", "example.com", "-4", "-S", "2001:db8::1"])]
    #[trace]
    fn not_valid(#[case] name: &str, #[case] input: Vec<&str>) {
        let mut args = Args::parse_from(input);
        let validated = args.validate().unwrap_err();

        assert_debug_snapshot!(format!("not_valid_{name}"), validated);
    }
}
