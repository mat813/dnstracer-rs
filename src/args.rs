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
    #[arg(short = 'q', long, default_value = "A", value_parser = RecordType::from_str)]
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
    use std::net::IpAddr;

    #[test]
    fn default_values() {
        let args = Args::try_parse_from(["test", "example.com"]).unwrap();

        assert_eq!(args.domain, "example.com");
        assert!(!args.no_positive_cache);
        assert!(!args.negative_cache);
        assert!(!args.no_edns0);
        assert!(!args.overview);
        assert_eq!(args.query_type, RecordType::A);
        assert_eq!(args.retries, 3);
        assert_eq!(args.server, "a.root-servers.net");
        assert_eq!(args.timeout, Duration::from_secs(5));
        assert!(args.source_address.is_none());
        assert!(!args.ipv6);
        assert!(!args.ipv4);
        assert!(!args.tcp);
    }

    #[test]
    fn all_flags() {
        let args = Args::try_parse_from([
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
        ])
        .unwrap();

        assert!(args.no_positive_cache);
        assert!(args.negative_cache);
        assert!(args.no_edns0);
        assert!(args.overview);
        assert_eq!(args.query_type, RecordType::NS);
        assert_eq!(args.retries, 5);
        assert_eq!(args.server, "8.8.8.8");
        assert_eq!(args.timeout, Duration::from_secs(10));
        assert_eq!(
            args.source_address,
            Some(IpAddr::from_str("192.168.0.1").unwrap())
        );
        assert!(args.ipv6);
        assert!(!args.ipv4);
        assert!(args.tcp);
    }

    #[test]
    fn ipv4_flag() {
        let args = Args::try_parse_from(["test", "example.com", "-4"]).unwrap();

        assert!(args.ipv4);
        assert!(!args.ipv6);
    }

    #[test]
    fn with_server_override() {
        let args = Args::try_parse_from(["test", "-s", "1.1.1.1", "example.com"]).unwrap();

        assert_eq!(args.server, "1.1.1.1");
    }

    #[test]
    fn with_query_type() {
        let args = Args::try_parse_from(["test", "example.com", "-q", "AAAA"]).unwrap();

        assert_eq!(args.query_type, RecordType::AAAA);
    }

    #[test]
    fn invalid_query_type() {
        let result = Args::try_parse_from(["test", "example.com", "-q", "INVALID"]);
        assert!(result.is_err()); // Should fail since "INVALID" is not a valid RecordType
    }

    #[test]
    fn with_source_address_v4() {
        let mut args = Args::try_parse_from(["test", "example.com", "-S", "1.1.1.1"]).unwrap();
        let validated = args.validate();

        assert!(validated.is_ok());
        assert_eq!(args.source_address, Some("1.1.1.1".parse().unwrap()));
        assert!(args.ipv4);
        assert!(!args.ipv6);
    }

    #[test]
    fn with_source_address_v6() {
        let mut args = Args::try_parse_from(["test", "example.com", "-S", "2001:db8::1"]).unwrap();
        let validated = args.validate();

        assert!(validated.is_ok());
        assert_eq!(args.source_address, Some("2001:db8::1".parse().unwrap()));
        assert!(!args.ipv4);
        assert!(args.ipv6);
    }

    #[test]
    fn with_source_address_v4_and_ipv6() {
        let mut args =
            Args::try_parse_from(["test", "example.com", "-6", "-S", "1.1.1.1"]).unwrap();
        let validated = args.validate();

        assert!(validated.is_err());
        assert_eq!(
            validated.unwrap_err(),
            "Cannot use IPv6 only queries with an ipv4 source address (1.1.1.1)"
        );
    }

    #[test]
    fn with_source_address_v6_and_ipv4() {
        let mut args =
            Args::try_parse_from(["test", "example.com", "-4", "-S", "2001:db8::1"]).unwrap();
        let validated = args.validate();

        assert!(validated.is_err());
        assert_eq!(
            validated.unwrap_err(),
            "Cannot use IPv4 only queries with an ipv6 source address (2001:db8::1)"
        );
    }
}
