use crate::{args::Args, opt_name::OptName};
use derive_more::Display;
use exn::{Result, ResultExt as _, bail};
use hickory_client::client::{Client, ClientHandle as _};
use hickory_proto::{
    op::ResponseCode,
    rr::{DNSClass, Name, RData, Record, RecordType},
    runtime::TokioRuntimeProvider,
    tcp::TcpClientStream,
    udp::UdpClientStream,
    xfer::DnsResponse,
};
use hickory_resolver::{
    TokioResolver,
    config::{LookupIpStrategy, ResolverOpts},
    name_server::GenericConnector,
};
use itertools::Itertools as _;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt,
    future::Future,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    sync::RwLock,
};

/// A container for all resolver errors
#[derive(Debug, Display)]
pub struct ResolverError(String);

impl std::error::Error for ResolverError {}

/// Is the ip allowed with regard to what the op asked.
macro_rules! is_ip_allowed {
        ($self:expr, $ip:expr ) => {
        $ip.is_ipv4() && $self.arguments.ipv4 // if we asked for ipv4 only
                || $ip.is_ipv6() && $self.arguments.ipv6 // if we asked for ipv6 only
                || !($self.arguments.ipv4 || $self.arguments.ipv6) // if we did not ask anything
        }
    }

/// Cache key
type CacheKey = (IpAddr, Name);

/// Lookup cache
type Cache = HashSet<CacheKey>;

/// Results from one nameserver
#[derive(Clone, Debug, Default)]
struct FullResult {
    /// List of records from a nameserver
    records: BTreeSet<Record>,
    /// Response code from the answer
    response_code: ResponseCode,
}

/// Recursive resolver
pub struct RecursiveResolver<'a> {
    /// Store the results, in case we need to display them
    results: RwLock<HashMap<OptName, FullResult>>,
    /// Single resolver for everything
    resolver: TokioResolver,
    /// Copy of all the arguments for easier processing
    arguments: &'a Args,
    /// Positive answer cache
    positive_cache: Option<RwLock<Cache>>,
    /// Negative answer cache
    negative_cache: Option<RwLock<Cache>>,
}

impl fmt::Debug for RecursiveResolver<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecursiveResolver")
            .field("results", &self.results)
            .field("args", &self.arguments)
            .field("positive_cache", &self.positive_cache)
            .field("negative_cache", &self.negative_cache)
            .finish_non_exhaustive()
    }
}

impl<'a> RecursiveResolver<'a> {
    /// Create a new recursive resolver
    pub fn new(args: &'a Args) -> Result<Self, ResolverError> {
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;
        resolver_opts.attempts = args.retries;
        resolver_opts.timeout = args.timeout;
        resolver_opts.edns0 = !args.no_edns0;

        let resolver = TokioResolver::builder(GenericConnector::new(TokioRuntimeProvider::new()))
            .or_raise(|| ResolverError("build tokio resolver".to_owned()))?
            .with_options(resolver_opts)
            .build();

        Ok(Self {
            results: RwLock::new(HashMap::new()),
            resolver,
            positive_cache: (!args.no_positive_cache).then(|| RwLock::new(HashSet::new())),
            negative_cache: args.negative_cache.then(|| RwLock::new(HashSet::new())),
            arguments: args,
        })
    }
}

impl RecursiveResolver<'_> {
    /// Figure out the server we got as an argument
    pub async fn init(&self) -> Result<Vec<OptName>, ResolverError> {
        let mut results: Vec<OptName> = vec![];

        // If it is an IP, use it directly
        if let Ok(ip) = self.arguments.server.parse::<IpAddr>() {
            results.push(OptName {
                ip,
                name: None,
                zone: None,
            });
        } else if self.arguments.server == "." {
            // It's a dot, we want to start at the root zone and iterate over all its name servers ips
            let root_ns: Vec<Name> = self
                .resolver
                .ns_lookup(".")
                .await
                .or_raise(|| ResolverError("ns lookup".to_owned()))?
                .iter()
                .cloned()
                .map(|ns| ns.0)
                .collect();

            for ns in root_ns {
                results.append(
                    &mut self
                        .resolver
                        .lookup_ip(ns.clone())
                        .await
                        .or_raise(|| ResolverError("ip lookup".to_owned()))?
                        .iter()
                        .filter(|ip| is_ip_allowed!(self, ip))
                        .map(|ip| OptName {
                            ip,
                            name: Some(ns.to_string()),
                            zone: Some(".".to_owned()),
                        })
                        .collect(),
                );
            }
        } else {
            // It's not a dot, let's try and resolve it
            results.append(
                &mut self
                    .resolver
                    .lookup_ip(&self.arguments.server)
                    .await
                    .or_raise(|| ResolverError("ip lookup".to_owned()))?
                    .iter()
                    .filter(|ip| is_ip_allowed!(self, ip))
                    .map(|ip| OptName {
                        ip,
                        name: Some(self.arguments.server.clone()),
                        zone: None,
                    })
                    .collect(),
            );
        }

        if results.is_empty() {
            bail!(ResolverError(format!(
                "no IP address found for hostname: {}",
                self.arguments.server
            )));
        }

        Ok(results)
    }

    /// Recurse through the internet looking for answers
    pub fn do_recurse<'a>(
        &'a self,
        name: &'a Name,
        server: &'a OptName,
        depth: usize,
        last: Vec<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ResolverError>> + 'a>> {
        Box::pin(async move {
            if self.cache_get(&(server.ip, name.clone())) {
                Self::print(depth, server, "(cached)", &last);
                return Ok(());
            }

            let response_res = {
                let query_type = if depth == 0 {
                    // First request is always a NS request, in case the given server is a recursive server.
                    RecordType::NS
                } else {
                    self.arguments.query_type
                };

                if self.arguments.tcp {
                    self.tcp_query(server, name, query_type).await
                } else {
                    self.udp_query(server, name, query_type).await
                }
            };

            match response_res {
                Ok(response) => {
                    let mut next_servers: Option<Vec<OptName>> = None;

                    if response.authoritative() {
                        // If the response is authoritative, we are probaby at the end of the journey.
                        let result = response.answers();
                        Self::print(depth, server, "found authoritative answer", &last);
                        self.cache_set(true, (server.ip, name.clone()));
                        self.add_result(server.clone(), response.response_code(), result)?;

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
                                .filter_map(|r| r.data().as_cname())
                            {
                                next_servers = Some(
                                    self.get_next_servers(
                                        response.name_servers(),
                                        &response,
                                        server,
                                        cname,
                                        depth,
                                        &last,
                                    )
                                    .await,
                                );
                            }
                        }
                    } else {
                        Self::print(depth, server, "", &last);

                        let records = if depth == 0 && response.answer_count() > 0 {
                            // If we're at the start and we get answers, it means it was a recursive name server, so use those answers.
                            response.answers()
                        } else {
                            // Otherwise, we get an authority section.
                            response.name_servers()
                        };

                        next_servers = Some(
                            self.get_next_servers(records, &response, server, name, depth, &last)
                                .await,
                        );
                    }

                    if let Some(next) = next_servers {
                        let len = next.len();
                        for (index, ns) in next.iter().sorted().enumerate() {
                            self.do_recurse(name, ns, depth + 1, {
                                let mut new_last = last.clone();
                                new_last.push(index == (len - 1));
                                new_last
                            })
                            .await
                            .or_raise(|| {
                                ResolverError(format!("do recurse depth {}", depth + 1))
                            })?;
                        }
                    }
                }
                Err(e) => {
                    self.cache_set(false, (server.ip, name.clone()));
                    Self::print(depth, server, format!("resolution error: {e}"), &last);
                }
            }
            Ok(())
        })
    }

    /// Make a UDP DNS query
    async fn udp_query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<DnsResponse, ResolverError> {
        let stream = UdpClientStream::builder(server.into(), TokioRuntimeProvider::new())
            .with_timeout(Some(self.arguments.timeout))
            .with_bind_addr(
                self.arguments
                    .source_address
                    .map(|ip| SocketAddr::new(ip, 0)),
            )
            .build();
        let (mut client, bg) = Client::connect(stream)
            .await
            .or_raise(|| ResolverError("client connect".to_owned()))?;

        if self.arguments.no_edns0 {
            client.disable_edns();
        } else {
            client.enable_edns();
        }

        // Spawn background task for the DNS client
        tokio::spawn(bg);

        client
            .query(name.clone(), DNSClass::IN, query_type)
            .await
            .or_raise(|| ResolverError("client query".to_owned()))
    }

    /// Make a TCP DNS query
    async fn tcp_query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<DnsResponse, ResolverError> {
        let (stream, sender) = TcpClientStream::new(
            server.into(),
            self.arguments
                .source_address
                .map(|ip| SocketAddr::new(ip, 0)),
            Some(self.arguments.timeout),
            TokioRuntimeProvider::new(),
        );

        let (mut client, bg) = Client::new(stream, sender, None)
            .await
            .or_raise(|| ResolverError("client new".to_owned()))?;

        if self.arguments.no_edns0 {
            client.disable_edns();
        } else {
            client.enable_edns();
        }

        // Spawn background task for the DNS client
        tokio::spawn(bg);

        client
            .query(name.clone(), DNSClass::IN, query_type)
            .await
            .or_raise(|| ResolverError("client query".to_owned()))
    }

    /// Figure out the next servers in the recursion
    async fn get_next_servers(
        &self,
        records: &[Record],
        response: &DnsResponse,
        server: &OptName,
        name: &Name,
        depth: usize,
        last: &[bool],
    ) -> Vec<OptName> {
        let mut next_servers: Vec<OptName> = vec![];

        for record in records {
            let mut found = false;
            // Here, we know it's a NS, so unwrap all that.
            let Some(ns) = record.data().as_ns() else {
                continue;
            };
            // Some name servers will respond with an additional section, use it
            next_servers.append(
                &mut response
                    .additionals()
                    .iter()
                    .filter(|r| *r.name() == ns.0)
                    .filter_map(|additional| match *additional.data() {
                        RData::A(ref a) => {
                            Some((additional, IpAddr::from(Into::<Ipv4Addr>::into(*a))))
                        }
                        RData::AAAA(ref a) => {
                            Some((additional, IpAddr::from(Into::<Ipv6Addr>::into(*a))))
                        }
                        _ => None,
                    })
                    .filter(|&(_, ip)| is_ip_allowed!(self, ip))
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
                if let Ok(response) = self.resolver.lookup_ip(ns_s).await {
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
                self.cache_set(true, (server.ip, name.clone()));
            } else {
                // If we cannot find an IP address, we create a fake server to give an error
                Self::print(
                    depth,
                    &OptName {
                        ip: [0, 0, 0, 0].into(),
                        name: Some(ns.to_string()),
                        zone: Some(record.name().to_string()),
                    },
                    "no ip found",
                    last,
                );
            }
        }

        next_servers
    }

    /// Print the overview
    #[expect(clippy::print_stdout, reason = "print")]
    pub fn show_overview(&self) -> Result<(), ResolverError> {
        for (key, values) in self
            .results
            .read()
            .map_err(|e| ResolverError(format!("get read lock: {e:?}")))?
            .iter()
        {
            if values.response_code != ResponseCode::NoError {
                println!(
                    "{} ({})\t{}",
                    key.name.as_deref().unwrap_or_default(),
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
                println!(
                    "{} ({}) \t{record}",
                    key.name.as_deref().unwrap_or_default(),
                    key.ip
                );
            }
        }
        Ok(())
    }

    /// Did we already ask for this, wether it turned out ok or not ?
    fn cache_get(&self, key: &CacheKey) -> bool {
        self.positive_cache
            .as_ref()
            .is_some_and(|o| o.read().ok().as_ref().and_then(|r| r.get(key)).is_some())
            || self
                .negative_cache
                .as_ref()
                .is_some_and(|o| o.read().ok().as_ref().and_then(|r| r.get(key)).is_some())
    }

    /// Set one of the caches
    #[expect(clippy::print_stderr, reason = "non fatal error")]
    fn cache_set(&self, positive: bool, key: CacheKey) {
        #[expect(clippy::needless_borrowed_reference, reason = "ok")]
        if let &Some(ref locked_cache) = if positive {
            &self.positive_cache
        } else {
            &self.negative_cache
        } {
            match locked_cache.write() {
                Ok(mut cache) => {
                    cache.insert(key);
                }
                Err(error) => {
                    eprintln!("cache set error {error}");
                }
            }
        }
    }

    /// Add a result to the pile
    #[expect(
        clippy::significant_drop_tightening,
        reason = "Scope is short enough and there should not be contentions"
    )]
    fn add_result(
        &self,
        server: OptName,
        response_code: ResponseCode,
        results: &[Record],
    ) -> Result<(), ResolverError> {
        let mut res = self
            .results
            .write()
            .map_err(|e| ResolverError(format!("get write lock: {e:?}")))?;
        let full = res.entry(server).or_default();

        full.response_code = response_code;

        for result in results {
            full.records.insert(result.clone());
        }

        Ok(())
    }

    /// Try to give a nice out, as the original did
    #[expect(clippy::print_stdout, reason = "called print")]
    fn print<S: fmt::Display>(depth: usize, server: &OptName, rest: S, last: &[bool]) {
        let mut output = String::new();

        for i in 0..depth {
            if *last.get(i).unwrap_or(&false) {
                output.push_str("  ");
            } else {
                output.push_str(" |");
            }
            if i < depth - 1 {
                output.push_str("     ");
            }
        }

        if depth > 0 {
            output.push_str(r"\___ ");
        }

        let rest = format!("{rest}");
        if rest.is_empty() {
            println!("{output}{server}");
        } else {
            println!("{output}{server} {rest}");
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::indexing_slicing,
        reason = "test"
    )]

    use super::*;
    use crate::args::Args;
    use std::{
        net::{IpAddr, Ipv4Addr},
        time::Duration,
    };

    fn default_args() -> Args {
        Args {
            domain: "example.com".to_owned(),
            no_positive_cache: false,
            negative_cache: false,
            no_edns0: true,
            overview: false,
            query_type: RecordType::A,
            retries: 3,
            server: ".".to_owned(),
            timeout: Duration::from_secs(5),
            source_address: None,
            ipv6: false,
            ipv4: true,
            tcp: false,
        }
    }

    #[test]
    fn recursive_resolver_new() {
        let args = default_args();
        let resolver = RecursiveResolver::new(&args).unwrap();

        assert_eq!(*resolver.arguments, args);
        assert!(resolver.positive_cache.is_some());
        assert!(resolver.negative_cache.is_none());
    }

    #[test]
    fn recursive_resolver_new_2() {
        let args = Args {
            no_positive_cache: true,
            negative_cache: true,
            ..default_args()
        };
        let resolver = RecursiveResolver::new(&args).unwrap();

        assert_eq!(*resolver.arguments, args);
        assert!(resolver.positive_cache.is_none());
        assert!(resolver.negative_cache.is_some());
    }

    #[tokio::test]
    async fn recursive_resolver_init_with_ip() {
        let args = Args {
            server: "8.8.8.8".to_owned(),
            ..default_args()
        };
        let resolver = RecursiveResolver::new(&args).unwrap();

        let result = resolver.init().await;
        assert!(result.is_ok());
        let servers = result.unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
        assert!(servers[0].name.is_none());
    }
}
