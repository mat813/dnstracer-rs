use crate::{args::Args, opt_name::OptName};
use hickory_client::{
    client::{Client, SyncClient},
    op::DnsResponse,
    rr::{Name, RecordType},
    udp::UdpClientConnection,
};
use hickory_proto::{
    op::ResponseCode,
    rr::{DNSClass, RData, Record},
};
use hickory_resolver::{
    config::{LookupIpStrategy, ResolverConfig, ResolverOpts},
    error::ResolveError,
    Resolver,
};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::RwLock,
    time::Duration,
};

/// Provide a shortcut to get the root servers
macro_rules! dot_to_a {
    ($server:expr) => {
        if $server == "." {
            String::from("A.ROOT-SERVERS.NET.")
        } else {
            $server.clone()
        }
    };
}

/// Is the ip allowed with regard to what the op asked.
macro_rules! is_ip_allowed {
        ($self:expr, $ip:expr ) => {
        $ip.is_ipv4() && $self.arguments.ipv4 // if we asked for ipv4 only
                || $ip.is_ipv6() && $self.arguments.ipv6 // if we asked for ipv6 only
                || !($self.arguments.ipv4 || $self.arguments.ipv6) // if we did not ask anything
        }
    }

/// Lookup cache
type Cache = HashSet<(IpAddr, Name)>;

/// Results from one nameserver
#[derive(Clone, Debug, Default)]
struct FullResult {
    /// List of records from a nameserver
    records: Vec<Record>,
    /// Response code from the answer
    response_code: ResponseCode,
}

/// Recursive resolver
pub struct RecursiveResolver {
    results: RwLock<HashMap<OptName, FullResult>>,
    resolver: Resolver,
    arguments: Args,
    positive_cache: Option<RwLock<Cache>>,
    negative_cache: Option<RwLock<Cache>>,
}

impl fmt::Debug for RecursiveResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecursiveResolver")
            .field("results", &self.results)
            .field("args", &self.arguments)
            .field("positive_cache", &self.positive_cache)
            .field("negative_cache", &self.negative_cache)
            .finish()
    }
}

impl RecursiveResolver {
    /// Create a new recursive resolver
    pub fn new(args: Args) -> Self {
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;
        resolver_opts.attempts = args.retries;
        resolver_opts.timeout = Duration::from_secs(args.timeout);
        resolver_opts.edns0 = !args.no_edns0;

        Self {
            results: RwLock::new(HashMap::new()),
            resolver: Resolver::new(ResolverConfig::default(), resolver_opts)
                .expect("Unable to create internal resolver"),
            positive_cache: match args.no_positive_cache {
                true => None,
                false => Some(RwLock::new(HashSet::new())),
            },
            negative_cache: match args.negative_cache {
                true => Some(RwLock::new(HashSet::new())),
                false => None,
            },
            arguments: args,
        }
    }

    /// Figure out the server we got as an argument
    pub fn init(&self) -> Result<OptName, ResolveError> {
        // If it is an IP, use it directly
        if let Ok(ip) = self.arguments.server.parse::<IpAddr>() {
            Ok(OptName {
                ip,
                name: None,
                zone: None,
            })
        } else {
            // Otherwise, we got a name server, try and resolve its ip address
            match self.resolver.lookup_ip(dot_to_a!(self.arguments.server)) {
                Ok(lookup) => match lookup.iter().find(|ip| is_ip_allowed!(self, ip)) {
                    Some(ip) => Ok(OptName {
                        ip,
                        name: Some(dot_to_a!(self.arguments.server)),
                        zone: match self.arguments.server.as_str() {
                            "." => Some(".".to_string()),
                            _ => None,
                        },
                    }),
                    None => Err(ResolveError::from(format!(
                        "no IP address found for hostname: {}",
                        self.arguments.server
                    ))),
                },
                Err(e) => Err(ResolveError::from(format!(
                    "no IP address found for hostname: {} ({e})",
                    self.arguments.server
                ))),
            }
        }
    }

    /// Recurse through the internet looking for answers
    pub fn do_recurse(&self, name: &Name, server: OptName, depth: usize, last: Vec<bool>) {
        if self.cache_get(&server, name) {
            self.print(depth, &server, "(cached)", last.clone());
            return;
        }

        let address = server.clone().into();
        let conn = UdpClientConnection::with_bind_addr_and_timeout(
            address,
            self.arguments
                .source_address
                .map(|ip| SocketAddr::new(ip, 0)),
            Duration::from_secs(self.arguments.timeout),
        )
        .expect("Failed to create UDP connection");
        let client = SyncClient::new(conn);

        let response_res = client.query(
            name,
            DNSClass::IN,
            // First request is always a NS request, in case the given server is a recursive server.
            match depth {
                0 => RecordType::NS,
                _ => self.arguments.query_type,
            },
        );

        match response_res {
            Ok(response) => {
                let mut next_servers: Option<Vec<OptName>> = None;

                if response.authoritative() {
                    // If the response is authoritative, we are probaby at the end of the journey.
                    let result = response.answers();
                    self.print(depth, &server, "found authoritative answer", last.clone());
                    self.cache_set(true, &server, name);
                    self.add_result(server.clone(), response.response_code(), result);

                    // But if we get only CNAMEs and we asked for something
                    // else, we need to try and continue the recursion.
                    if self.arguments.query_type != RecordType::CNAME
                        && result.iter().all(|r| r.record_type() == RecordType::CNAME)
                        && response.name_server_count() > 0
                    {
                        for cname in response
                            .answers()
                            .iter()
                            // We already checked every entry was a CNAME
                            .map(|r| r.data().unwrap().as_cname().unwrap())
                        {
                            next_servers = Some(self.get_next_servers(
                                response.name_servers(),
                                &response,
                                &server,
                                cname,
                                depth,
                                &last,
                            ));
                        }
                    }
                } else {
                    self.print(depth, &server, "", last.clone());

                    let records = if depth == 0 && response.answer_count() > 0 {
                        // If we're at the start and we get answers, it means it was a recursive name server, so use those answers.
                        response.answers()
                    } else {
                        // Otherwise, we get an authority section.
                        response.name_servers()
                    };

                    next_servers = Some(
                        self.get_next_servers(records, &response, &server, name, depth, &last),
                    );
                }

                if let Some(next) = next_servers {
                    let len = next.len();
                    for (index, ns) in next.iter().sorted().enumerate() {
                        self.do_recurse(name, ns.clone(), depth + 1, {
                            let mut new_last = last.to_owned();
                            new_last.push(index == (len - 1));
                            new_last
                        });
                    }
                }
            }
            Err(e) => {
                self.cache_set(false, &server, name);
                self.print(depth, &server, format!("resolution error: {e}"), last);
            }
        }
    }

    /// Figure out the next servers in the recursion
    fn get_next_servers(
        &self,
        records: &[Record],
        response: &DnsResponse,
        server: &OptName,
        name: &Name,
        depth: usize,
        last: &[bool],
    ) -> Vec<OptName> {
        let mut next_servers: Vec<OptName> = vec![];

        for record in records.iter() {
            let mut found = false;
            // Here, we know it's a NS, so unwrap all that.
            let ns = record.data().unwrap().as_ns().unwrap();
            // Some name servers will respond with an additional section, use it
            next_servers.append(
                &mut response
                    .additionals()
                    .iter()
                    .filter(|r| *r.name() == ns.0)
                    .filter_map(|additional| match additional.data() {
                        Some(RData::A(a)) => Some((additional, IpAddr::from(Ipv4Addr::from(*a)))),
                        Some(RData::AAAA(a)) => {
                            Some((additional, IpAddr::from(Ipv6Addr::from(*a))))
                        }
                        _ => None,
                    })
                    .filter(|(_, ip)| is_ip_allowed!(self, ip))
                    .map(|(additional, ip)| {
                        found = true;

                        OptName {
                            ip,
                            name: Some(additional.name().to_string()),
                            zone: Some(record.name().to_string()),
                        }
                    })
                    .collect(),
            );

            // If we don't have an additional section or we had stuff
            // but not this name server, we need to resolve it
            // ourselves.
            if !found {
                let ns_s = ns.to_string();
                if let Ok(response) = self.resolver.lookup_ip(dot_to_a!(ns_s)) {
                    next_servers.append(
                        &mut response
                            .iter()
                            .filter(|ip| is_ip_allowed!(self, ip))
                            .map(|ip| {
                                found = true;

                                OptName {
                                    ip,
                                    name: Some(ns.to_string()),
                                    zone: Some(record.name().to_string()),
                                }
                            })
                            .collect(),
                    );
                }
            }

            if found {
                self.cache_set(true, server, name);
            } else {
                // If we cannot find an IP address, we create a fake server to give an error
                self.print(
                    depth,
                    &OptName {
                        ip: [0, 0, 0, 0].into(),
                        name: Some(ns.to_string()),
                        zone: Some(record.name().to_string()),
                    },
                    "no ip found",
                    last.to_owned(),
                );
            }
        }

        next_servers
    }

    /// Print the overview
    pub fn show_overview(&self) {
        for (key, values) in self
            .results
            .read()
            .unwrap()
            .iter()
            .sorted_by_key(|(o, _)| *o)
        {
            if values.response_code != ResponseCode::NoError {
                println!(
                    "{} ({})\t{}",
                    key.name.as_deref().unwrap(),
                    key.ip,
                    values.response_code
                );
            }

            for record in values
                .records
                .iter()
                // Don't use Record's Ord impl, it sorts things in a strange way
                .sorted_by_cached_key(|r| format!("{r}"))
            {
                println!("{} ({}) \t{record}", key.name.as_deref().unwrap(), key.ip);
            }
        }
    }

    /// Did we already ask for this, wether it turned out ok or not ?
    fn cache_get(&self, server: &OptName, name: &Name) -> bool {
        (match &self.positive_cache {
            Some(ref o) => o.read().unwrap().get(&(server.ip, name.clone())).is_some(),
            None => false,
        }) || (match &self.negative_cache {
            Some(ref o) => o.read().unwrap().get(&(server.ip, name.clone())).is_some(),
            None => false,
        })
    }

    /// Set one of the caches
    fn cache_set(&self, positive: bool, server: &OptName, name: &Name) {
        if let Some(ref o) = match positive {
            true => &self.positive_cache,
            false => &self.negative_cache,
        } {
            let mut a = o.write().unwrap();
            a.insert((server.ip, name.clone()));
        }
    }

    /// Add a result to the pile
    fn add_result(&self, server: OptName, response_code: ResponseCode, results: &[Record]) {
        let mut res = self.results.write().unwrap();

        let full = res.entry(server).or_default();

        full.response_code = response_code;

        for result in results {
            if !full.records.contains(result) {
                full.records.push(result.clone());
            }
        }
    }

    /// Try to give a nice out, as the original did
    fn print<S: fmt::Display>(&self, depth: usize, server: &OptName, rest: S, last: Vec<bool>) {
        let mut output = String::new();

        for i in 0..depth {
            if *last.get(i).unwrap_or(&false) {
                output.push_str("  ")
            } else {
                output.push_str(" |")
            }
            if i < depth - 1 {
                output.push_str("     ");
            }
        }

        if depth > 0 {
            output.push_str(r"\___ ");
        }

        if format!("{}", rest) != "" {
            println!("{output}{server} {rest}");
        } else {
            println!("{output}{server}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Args;
    use std::net::{IpAddr, Ipv4Addr};

    fn default_args() -> Args {
        Args {
            domain: "example.com".to_string(),
            no_positive_cache: false,
            negative_cache: false,
            no_edns0: true,
            overview: false,
            query_type: RecordType::A,
            retries: 3,
            server: ".".to_string(),
            timeout: 5,
            source_address: None,
            ipv6: false,
            ipv4: true,
        }
    }

    #[test]
    fn test_recursive_resolver_new() {
        let args = default_args();
        let resolver = RecursiveResolver::new(args.clone());

        assert_eq!(resolver.arguments, args);
        assert!(resolver.positive_cache.is_some());
        assert!(resolver.negative_cache.is_none());
    }

    #[test]
    fn test_recursive_resolver_new_2() {
        let args = Args {
            no_positive_cache: true,
            negative_cache: true,
            ..default_args()
        };
        let resolver = RecursiveResolver::new(args.clone());

        assert_eq!(resolver.arguments, args);
        assert!(resolver.positive_cache.is_none());
        assert!(resolver.negative_cache.is_some());
    }

    #[test]
    fn test_recursive_resolver_init_with_ip() {
        let args = Args {
            server: "8.8.8.8".to_string(),
            ..default_args()
        };
        let resolver = RecursiveResolver::new(args);

        let result = resolver.init();
        assert!(result.is_ok());
        let opt_name = result.unwrap();
        assert_eq!(opt_name.ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
        assert!(opt_name.name.is_none());
    }
}
