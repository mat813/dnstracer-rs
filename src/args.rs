use clap::Parser;
use hickory_client::rr::RecordType;
use std::{net::IpAddr, str::FromStr};

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
#[derive(Parser, Debug, Clone, PartialEq)]
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

    /// Start the query at the given DNS server (IP or hostname)
    /// If "." is specified, A.ROOT-SERVERS.NET will be used
    #[arg(short, long, default_value = ".")]
    pub server: String,

    /// Limit time to wait per try
    #[arg(short = 't', long, default_value = "5")]
    pub timeout: u64,

    /// use this source address.
    #[arg(short = 'S', long)]
    pub source_address: Option<IpAddr>,

    /// Force using IPv6 for DNS queries (no IPv4)
    #[arg(short = '6', long)]
    pub ipv6: bool,

    /// Force using IPv4 for DNS queries (no IPv6)
    #[arg(short = '4', long)]
    pub ipv4: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::net::IpAddr;

    #[test]
    fn test_default_values() {
        let args = Args::try_parse_from(&["test", "example.com"]).unwrap();

        assert_eq!(args.domain, "example.com");
        assert!(!args.no_positive_cache);
        assert!(!args.negative_cache);
        assert!(!args.no_edns0);
        assert!(!args.overview);
        assert_eq!(args.query_type, RecordType::A);
        assert_eq!(args.retries, 3);
        assert_eq!(args.server, ".");
        assert_eq!(args.timeout, 5);
        assert!(args.source_address.is_none());
        assert!(!args.ipv6);
        assert!(!args.ipv4);
    }

    #[test]
    fn test_all_flags() {
        let args = Args::try_parse_from(&[
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
        assert_eq!(args.timeout, 10);
        assert_eq!(
            args.source_address,
            Some(IpAddr::from_str("192.168.0.1").unwrap())
        );
        assert!(args.ipv6);
        assert!(!args.ipv4);
    }

    #[test]
    fn test_ipv4_flag() {
        let args = Args::try_parse_from(&["test", "example.com", "-4"]).unwrap();

        assert!(args.ipv4);
        assert!(!args.ipv6);
    }

    #[test]
    fn test_with_server_override() {
        let args = Args::try_parse_from(&["test", "-s", "1.1.1.1", "example.com"]).unwrap();

        assert_eq!(args.server, "1.1.1.1");
    }

    #[test]
    fn test_with_query_type() {
        let args = Args::try_parse_from(&["test", "example.com", "-q", "AAAA"]).unwrap();

        assert_eq!(args.query_type, RecordType::AAAA);
    }

    #[test]
    fn test_invalid_query_type() {
        let result = Args::try_parse_from(&["test", "example.com", "-q", "INVALID"]);
        assert!(result.is_err()); // Should fail since "INVALID" is not a valid RecordType
    }
}
