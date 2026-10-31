use std::{
    cell::{Cell, RefCell},
    collections::{BTreeSet, HashMap, HashSet},
    fmt,
    io::{self, Write},
    net::{IpAddr, SocketAddr},
};

use derive_more::{Debug, Display};
use exn::{Result, ResultExt as _, bail};
use hickory_net::{
    client::{Client, ClientHandle as _},
    runtime::TokioRuntimeProvider,
    tcp::TcpClientStream,
    udp::UdpClientStream,
};
use hickory_proto::{
    op::{DnsResponse, ResponseCode},
    rr::{DNSClass, Name, RData, Record, RecordType},
};
use hickory_resolver::{
    TokioResolver,
    config::{LookupIpStrategy, ResolverOpts},
};

use crate::{args::Args, opt_name::OptName};

/// The errors of the resolver
#[derive(Debug, Display)]
pub enum ResolverError {
    /// The NS lookup failed
    #[display("NS lookup failed for {name}")]
    NsLookup {
        /// The name that the lookup used
        name: String,
    },
    /// The IP lookup failed
    #[display("IP lookup failed for {name}")]
    IpLookup {
        /// The name that has no IP address
        name: String,
    },
    /// The client query failed
    #[display("Client query failed for {query_type} {name}")]
    ClientQuery {
        /// The name that the query used
        name: Name,
        /// The record type that the query used
        query_type: RecordType,
    },
    /// The client creation failed
    #[display("Client creation failed for {name}")]
    ClientNew {
        /// The server that the client connects to
        name: OptName,
    },
    /// The Tokio resolver cannot be built
    #[display("Cannot build the Tokio resolver")]
    BuildTokioResolver,
    /// The hostname has no IP address
    #[display("No IP address found for hostname {name}")]
    NoIpForHostname {
        /// The hostname
        name: String,
    },
    /// The call to `do_recurse` failed
    #[display("Recursive query failed")]
    DoRecurse,
    /// The recursion went deeper than the maximum depth
    #[display("Recursion is deeper than the maximum depth {depth}")]
    MaxDepthExceeded {
        /// The maximum depth
        depth: usize,
    },
    /// The trace sent the maximum number of queries
    #[display("The trace sent more than the maximum of {max} queries")]
    MaxQueriesExceeded {
        /// The maximum number of queries
        max: usize,
    },
    /// The output cannot be written
    #[display("Cannot write the output")]
    Write,
}

impl std::error::Error for ResolverError {}

/// The maximum DNS delegation depth. This limit stops infinite recursion.
const MAX_RECURSION_DEPTH: usize = 32;

/// The key of a cache entry
type CacheKey = (IpAddr, Name);

/// A cache of lookups
type Cache = HashSet<CacheKey>;

/// The results from one name server
#[derive(Clone, Debug, Default)]
struct FullResult {
    /// The records from the name server
    records: BTreeSet<Record>,
    /// The response code of the answer
    response_code: ResponseCode,
}

/// Resolve DNS names to IP addresses. Used to find the first servers and to resolve NS names.
#[cfg_attr(
    test,
    allow(clippy::unused_async_trait_impl, reason = "mockall"),
    mockall::automock
)]
pub trait NameResolver: Send + Sync {
    /// Find the NS records of a zone
    async fn ns_lookup(&self, name: &str) -> Result<Vec<Name>, ResolverError>;
    /// Find the IP addresses of a hostname
    async fn lookup_ip(&self, name: &str) -> Result<Vec<IpAddr>, ResolverError>;
}

/// A DNS query result. It has only the fields that this program uses.
#[derive(Clone, Debug, Default)]
pub struct QueryResult {
    /// True if the response is authoritative
    pub authoritative: bool,
    /// The records of the answer section
    pub answers: Vec<Record>,
    /// The records of the authority section
    pub name_servers: Vec<Record>,
    /// The records of the additional section
    pub additionals: Vec<Record>,
    /// The response code
    pub response_code: ResponseCode,
}

impl From<DnsResponse> for QueryResult {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(resp), ret)
    )]
    fn from(resp: DnsResponse) -> Self {
        #[cfg(feature = "tracing")]
        tracing::debug!(?resp);

        Self {
            authoritative: resp.metadata.authoritative,
            answers: resp.answers.clone(),
            name_servers: resp.authorities.clone(),
            additionals: resp.additionals.clone(),
            response_code: resp.metadata.response_code,
        }
    }
}

/// Send DNS queries directly to a name server
#[cfg_attr(
    test,
    allow(clippy::unused_async_trait_impl, reason = "mockall"),
    mockall::automock
)]
pub trait DnsQuerier: Send + Sync {
    /// Send a DNS query to the server
    async fn query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<QueryResult, ResolverError>;
}

/// A DNS name resolver that wraps the `TokioResolver` of hickory
pub struct TokioNameResolver(TokioResolver);

impl NameResolver for TokioNameResolver {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, name), ret, err(level = "debug"))
    )]
    async fn ns_lookup(&self, name: &str) -> Result<Vec<Name>, ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?name);

        let lookup = self
            .0
            .ns_lookup(name)
            .await
            .or_raise(|| ResolverError::NsLookup {
                name: name.to_owned(),
            })?;

        // A caching resolver may answer "what are the NS for ." with an
        // answer section and no authority section. In that case, use the answer section.
        if name == "." && lookup.authorities().is_empty() && !lookup.answers().is_empty() {
            Ok(lookup
                .answers()
                .iter()
                .filter_map(|ns| match ns.data {
                    RData::NS(ref ns) => Some((**ns).clone()),
                    _ => None,
                })
                .collect())
        } else {
            Ok(lookup
                .authorities()
                .iter()
                .map(|ns| ns.name.clone())
                .collect())
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, name), ret, err(level = "debug"))
    )]
    async fn lookup_ip(&self, name: &str) -> Result<Vec<IpAddr>, ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?name);

        Ok(self
            .0
            .lookup_ip(name)
            .await
            .or_raise(|| ResolverError::IpLookup {
                name: name.to_owned(),
            })?
            .iter()
            .collect())
    }
}

/// The configuration of direct DNS queries. `from_args` copies it from `Args`.
pub struct DefaultDnsQuerier {
    /// Use TCP instead of UDP
    tcp: bool,
    /// The query timeout
    timeout: std::time::Duration,
    /// The source address to bind to, if any
    source_address: Option<IpAddr>,
    /// True if EDNS0 is disabled
    no_edns0: bool,
}

impl DefaultDnsQuerier {
    /// Create the configuration from the command-line arguments
    const fn from_args(args: &Args) -> Self {
        Self {
            tcp: args.tcp,
            timeout: args.timeout,
            source_address: args.source_address,
            no_edns0: args.no_edns0,
        }
    }

    /// Apply the EDNS0 setting and send the query on a client that exists
    async fn send(
        &self,
        mut client: Client<TokioRuntimeProvider>,
        name: &Name,
        query_type: RecordType,
    ) -> Result<DnsResponse, ResolverError> {
        if self.no_edns0 {
            client.disable_edns();
        } else {
            client.enable_edns();
        }

        client
            .query(name.clone(), DNSClass::IN, query_type)
            .await
            .or_raise(|| ResolverError::ClientQuery {
                name: name.clone(),
                query_type,
            })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            level = "trace",
            skip(self, server, name, query_type),
            ret,
            err(level = "debug")
        )
    )]
    /// Send a DNS query over UDP
    async fn udp_query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<DnsResponse, ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?server, ?name, ?query_type);

        let stream = UdpClientStream::builder(server.into(), TokioRuntimeProvider::new())
            .with_timeout(Some(self.timeout))
            .with_bind_addr(self.source_address.map(|ip| SocketAddr::new(ip, 0)))
            .build();
        let (client, bg) = Client::<TokioRuntimeProvider>::from_sender(stream);
        tokio::spawn(bg);

        self.send(client, name, query_type).await
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            level = "trace",
            skip(self, server, name, query_type),
            ret,
            err(level = "debug")
        )
    )]
    /// Send a DNS query over TCP
    async fn tcp_query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<DnsResponse, ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?server, ?name, ?query_type);

        let (future, sender) = TcpClientStream::new(
            server.into(),
            self.source_address.map(|ip| SocketAddr::new(ip, 0)),
            Some(self.timeout),
            TokioRuntimeProvider::new(),
        );

        let (client, bg) = Client::<TokioRuntimeProvider>::new(
            future.await.or_raise(|| ResolverError::ClientNew {
                name: server.clone(),
            })?,
            sender,
        );
        tokio::spawn(bg);

        self.send(client, name, query_type).await
    }
}

impl DnsQuerier for DefaultDnsQuerier {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            level = "trace",
            skip(self, server, name, query_type),
            ret,
            err(level = "debug")
        )
    )]
    async fn query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<QueryResult, ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?server, ?name, ?query_type);

        let response = if self.tcp {
            self.tcp_query(server, name, query_type).await
        } else {
            self.udp_query(server, name, query_type).await
        }?;
        Ok(QueryResult::from(response))
    }
}

/// A resolver that follows the DNS delegations from server to server
#[derive(Debug)]
pub struct RecursiveResolver<'a, R = TokioNameResolver, Q = DefaultDnsQuerier, W = io::Stdout> {
    /// The results. The overview shows them.
    results: RefCell<HashMap<OptName, FullResult>>,
    /// The resolver that finds the first servers and resolves NS names
    #[debug(skip)]
    name_resolver: R,
    /// The client that sends the DNS queries
    #[debug(skip)]
    querier: Q,
    /// The command-line arguments
    arguments: &'a Args,
    /// The cache of positive answers
    positive_cache: Option<RefCell<Cache>>,
    /// The cache of negative answers
    negative_cache: Option<RefCell<Cache>>,
    /// The number of queries that the trace sent
    queries: Cell<usize>,
    /// The writer for the output
    #[debug(skip)]
    output: RefCell<W>,
}

impl<'a> RecursiveResolver<'a> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(args), ret, err(level = "debug"))
    )]
    /// Create a resolver that uses real DNS clients
    pub fn new(args: &'a Args) -> Result<Self, ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?args);

        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;
        resolver_opts.attempts = args.retries;
        resolver_opts.timeout = args.timeout;
        resolver_opts.edns0 = !args.no_edns0;

        let resolver = TokioResolver::builder(TokioRuntimeProvider::new())
            .or_raise(|| ResolverError::BuildTokioResolver)?
            .with_options(resolver_opts)
            .build()
            .or_raise(|| ResolverError::BuildTokioResolver)?;

        Ok(Self {
            results: RefCell::new(HashMap::new()),
            name_resolver: TokioNameResolver(resolver),
            querier: DefaultDnsQuerier::from_args(args),
            positive_cache: (!args.no_positive_cache).then(|| RefCell::new(HashSet::new())),
            negative_cache: args.negative_cache.then(|| RefCell::new(HashSet::new())),
            queries: Cell::new(0),
            arguments: args,
            output: RefCell::new(io::stdout()),
        })
    }
}

impl<R: NameResolver, Q: DnsQuerier, W: Write + Send> RecursiveResolver<'_, R, Q, W> {
    /// Check if the IPv4 and IPv6 options of the user allow the IP address
    const fn is_ip_allowed(&self, ip: IpAddr) -> bool {
        ip.is_ipv4() && self.arguments.ipv4 // the user asked for IPv4 only
            || ip.is_ipv6() && self.arguments.ipv6 // the user asked for IPv6 only
            || !(self.arguments.ipv4 || self.arguments.ipv6) // the user did not select an IP version
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, host, zone), ret, err(level = "debug"))
    )]
    /// Resolve a hostname to the name servers that the IP options allow
    #[expect(clippy::future_not_send, reason = "single-threaded, RefCell state")]
    async fn lookup_servers(
        &self,
        host: &str,
        zone: Option<String>,
    ) -> Result<Vec<OptName>, ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?host, ?zone);

        Ok(self
            .name_resolver
            .lookup_ip(host)
            .await
            .or_raise(|| ResolverError::IpLookup {
                name: host.to_owned(),
            })?
            .into_iter()
            .filter(|ip| self.is_ip_allowed(*ip))
            .map(|ip| OptName {
                ip,
                name: Some(host.to_owned()),
                zone: zone.clone(),
            })
            .collect())
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self), ret, err(level = "debug"))
    )]
    /// Find the servers that the `server` argument selects
    #[expect(clippy::future_not_send, reason = "single-threaded, RefCell state")]
    pub async fn init(&self) -> Result<Vec<OptName>, ResolverError> {
        let mut results: Vec<OptName> = vec![];

        // For an IP address, use that address
        if let Ok(ip) = self.arguments.server.parse::<IpAddr>() {
            results.push(OptName {
                ip,
                name: None,
                zone: None,
            });
        } else if self.arguments.server == "." {
            // For a dot, start at the root zone and use the IP addresses of all its name servers
            let root_ns = self.name_resolver.ns_lookup(".").await?;

            for ns in root_ns {
                results.extend(
                    self.lookup_servers(&ns.to_string(), Some(".".to_owned()))
                        .await?,
                );
            }
        } else {
            // For a hostname, resolve it
            results.extend(self.lookup_servers(&self.arguments.server, None).await?);
        }

        if results.is_empty() {
            bail!(ResolverError::NoIpForHostname {
                name: self.arguments.server.clone()
            });
        }

        Ok(results)
    }

    /// Count one query. A hostile delegation can fan out without limit,
    /// so fail after the maximum number of queries.
    fn count_query(&self) -> Result<(), ResolverError> {
        let max = self.arguments.max_queries;
        if self.queries.get() >= max {
            bail!(ResolverError::MaxQueriesExceeded { max });
        }
        self.queries.set(self.queries.get() + 1);
        Ok(())
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, name, server, depth, last))
    )]
    /// Follow the delegations from server to server to find the answers
    #[expect(clippy::future_not_send, reason = "single-threaded, RefCell state")]
    #[expect(
        clippy::too_many_lines,
        reason = "one function for the whole delegation step"
    )]
    pub async fn do_recurse(
        &self,
        name: &Name,
        server: &OptName,
        depth: usize,
        last: Vec<bool>,
    ) -> Result<(), ResolverError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?name, ?server, ?depth, ?last);

        if depth > MAX_RECURSION_DEPTH {
            bail!(ResolverError::MaxDepthExceeded {
                depth: MAX_RECURSION_DEPTH
            });
        }

        if self.cache_get(&(server.ip, name.clone())) {
            self.print(depth, server, "(cached)", &last);
            return Ok(());
        }

        let query_type = if depth == 0 {
            // The first request is always an NS request, because the first server can be a recursive server.
            RecordType::NS
        } else {
            self.arguments.query_type
        };

        self.count_query()?;

        match self.querier.query(server, name, query_type).await {
            Ok(response) => {
                let mut next_servers: Option<BTreeSet<OptName>> = None;

                if response.authoritative {
                    // An authoritative response probably ends the delegation chain.
                    let result = &response.answers;
                    self.print(depth, server, "found authoritative answer", &last);
                    self.cache_set(true, (server.ip, name.clone()));
                    self.add_result(server.clone(), response.response_code, result);

                    // If the answers are only CNAMEs and the query type is not CNAME,
                    // continue the recursion.
                    if self.arguments.query_type != RecordType::CNAME
                        && result.iter().all(|r| r.record_type() == RecordType::CNAME)
                        && !response.name_servers.is_empty()
                    {
                        let mut accumulated: BTreeSet<OptName> = BTreeSet::new();
                        for cname in response
                            .answers
                            .iter()
                            // The check above showed that every record is a CNAME
                            .filter_map(|r| match r.data {
                                RData::CNAME(ref cname) => Some(cname),
                                _ => None,
                            })
                        {
                            accumulated.extend(
                                self.get_next_servers(
                                    &response.name_servers,
                                    &response.additionals,
                                    server,
                                    cname,
                                    depth,
                                    &last,
                                )
                                .await,
                            );
                        }
                        if !accumulated.is_empty() {
                            next_servers = Some(accumulated);
                        }
                    }
                } else {
                    self.print(depth, server, "", &last);

                    let (records, additionals) = if depth == 0 && !response.answers.is_empty() {
                        // At the start, answers mean that the server is a recursive name server. Use the answers.
                        (&response.answers, &response.additionals)
                    } else {
                        // Otherwise, use the authority section.
                        (&response.name_servers, &response.additionals)
                    };

                    next_servers = Some(
                        self.get_next_servers(records, additionals, server, name, depth, &last)
                            .await,
                    );
                }

                if let Some(next) = next_servers {
                    let len = next.len();
                    for (index, ns) in next.iter().enumerate() {
                        Box::pin(self.do_recurse(name, ns, depth + 1, {
                            let mut new_last = last.clone();
                            new_last.push(index == (len - 1));
                            new_last
                        }))
                        .await
                        .or_raise(|| ResolverError::DoRecurse)?;
                    }
                }
            },
            Err(e) => {
                self.cache_set(false, (server.ip, name.clone()));
                self.print(
                    depth,
                    server,
                    std::fmt::from_fn(|f| {
                        write!(
                            f,
                            "{e} -> {}",
                            e.frame().children().first().map_or_else(
                                || "unknown error".to_owned(),
                                std::string::ToString::to_string
                            )
                        )
                    }),
                    &last,
                );
            },
        }
        Ok(())
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            level = "trace",
            skip(self, records, additionals, server, name, depth, last),
            ret
        )
    )]
    /// Find the servers for the next step of the recursion
    #[expect(clippy::future_not_send, reason = "single-threaded, RefCell state")]
    async fn get_next_servers(
        &self,
        records: &[Record],
        additionals: &[Record],
        server: &OptName,
        name: &Name,
        depth: usize,
        last: &[bool],
    ) -> BTreeSet<OptName> {
        #[cfg(feature = "tracing")]
        tracing::debug!(?records, ?additionals, ?server, ?name, ?depth, ?last);

        let mut next_servers: BTreeSet<OptName> = BTreeSet::new();

        for record in records {
            // Only NS records give servers. Skip the other records.
            let RData::NS(ref ns) = record.data else {
                continue;
            };

            // Some name servers send an additional section. Use it.
            let before_len = next_servers.len();
            next_servers.extend(
                additionals
                    .iter()
                    .filter(|r| r.name == ns.0)
                    .filter_map(|additional| match additional.data {
                        RData::A(ref a) => Some((additional, IpAddr::V4(a.0))),
                        RData::AAAA(ref a) => Some((additional, IpAddr::V6(a.0))),
                        _ => None,
                    })
                    .filter(|&(_, ip)| self.is_ip_allowed(ip))
                    .map(|(additional, ip)| OptName {
                        ip,
                        name: Some(additional.name.to_string()),
                        zone: Some(record.name.to_string()),
                    }),
            );

            // If the additional section has no address for this name server,
            // resolve the name server.
            if next_servers.len() == before_len {
                let ns_s = ns.to_string();
                if let Ok(servers) = self
                    .lookup_servers(&ns_s, Some(record.name.to_string()))
                    .await
                {
                    next_servers.extend(servers);
                }

                if next_servers.len() > before_len {
                    self.cache_set(true, (server.ip, name.clone()));
                } else {
                    // If there is no IP address, print the error for a placeholder server
                    self.print(
                        depth,
                        &OptName {
                            ip: [0, 0, 0, 0].into(),
                            name: Some(ns.to_string()),
                            zone: Some(record.name.to_string()),
                        },
                        "no ip found",
                        last,
                    );
                }
            } else {
                self.cache_set(true, (server.ip, name.clone()));
            }
        }

        next_servers
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self), err(level = "debug"))
    )]
    /// Print the overview of the answers
    pub fn show_overview(&self) -> Result<(), ResolverError> {
        let results = self.results.borrow();
        let mut output = self.output.borrow_mut();
        for (key, values) in results.iter() {
            if values.response_code != ResponseCode::NoError {
                writeln!(
                    output,
                    "{} ({})\t{}",
                    key.name.as_deref().unwrap_or_default(),
                    key.ip,
                    values.response_code
                )
                .map_err(|_| ResolverError::Write)?;
            }

            // Do not use the `Ord` implementation of `Record`. It sorts the records in an unexpected order.
            let mut records: Vec<_> = values.records.iter().collect();
            records.sort_by_cached_key(|r| format!("{r}"));
            for record in records {
                writeln!(
                    output,
                    "{} ({}) \t{record}",
                    key.name.as_deref().unwrap_or_default(),
                    key.ip
                )
                .map_err(|_| ResolverError::Write)?;
            }
        }
        drop(output);
        drop(results);
        Ok(())
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, key), ret)
    )]
    /// Check if the program already sent a query for this key, with or without success
    fn cache_get(&self, key: &CacheKey) -> bool {
        #[cfg(feature = "tracing")]
        tracing::debug!(?key);

        [&self.positive_cache, &self.negative_cache]
            .into_iter()
            .flatten()
            .any(|cache| cache.borrow().contains(key))
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, positive, key))
    )]
    /// Add the key to the positive or the negative cache
    fn cache_set(&self, positive: bool, key: CacheKey) {
        #[cfg(feature = "tracing")]
        tracing::debug!(?positive, ?key);

        let cache = if positive {
            &self.positive_cache
        } else {
            &self.negative_cache
        };
        if let Some(cache) = cache.as_ref() {
            cache.borrow_mut().insert(key);
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, server, response_code, results))
    )]
    /// Add the records of a server to the results
    fn add_result(&self, server: OptName, response_code: ResponseCode, results: &[Record]) {
        #[cfg(feature = "tracing")]
        tracing::debug!(?server, ?response_code, ?results);

        let mut res = self.results.borrow_mut();
        let full = res.entry(server).or_default();

        full.response_code = response_code;

        for result in results {
            full.records.insert(result.clone());
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(self, depth, server, rest, last))
    )]
    /// Print one line of the delegation tree, in the format of the original tool
    fn print(&self, depth: usize, server: &OptName, rest: impl fmt::Display, last: &[bool]) {
        #[cfg(feature = "tracing")]
        tracing::debug!(?depth, ?server, %rest, ?last);

        let mut prefix = String::new();

        for i in 0..depth {
            if *last.get(i).unwrap_or(&false) {
                prefix.push_str("  ");
            } else {
                prefix.push_str(" |");
            }
            if i < depth - 1 {
                prefix.push_str("     ");
            }
        }

        if depth > 0 {
            prefix.push_str(r"\___ ");
        }

        let rest = format!("{rest}");
        let line = if rest.is_empty() {
            format!("{prefix}{server}\n")
        } else {
            format!("{prefix}{server} {rest}\n")
        };
        let _ = self.output.borrow_mut().write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::indexing_slicing, reason = "test")]

    use std::{net::Ipv4Addr, str::FromStr as _, time::Duration};

    use hickory_proto::rr::rdata;
    use insta::{assert_debug_snapshot, assert_snapshot};
    use mockall::predicate;

    use super::*;

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip()))]
    fn default_args() -> Args {
        Args {
            domain: "example.com".to_owned(),
            no_positive_cache: false,
            negative_cache: false,
            no_edns0: true,
            overview: false,
            query_type: RecordType::A,
            retries: 3,
            max_queries: 200,
            server: ".".to_owned(),
            timeout: Duration::from_secs(5),
            source_address: None,
            ipv6: false,
            ipv4: true,
            tcp: false,
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(args, name_resolver, querier))
    )]
    /// Build a `RecursiveResolver` with mock implementations. It discards the output.
    fn mock_resolver(
        args: &Args,
        name_resolver: MockNameResolver,
        querier: MockDnsQuerier,
    ) -> RecursiveResolver<'_, MockNameResolver, MockDnsQuerier, Vec<u8>> {
        RecursiveResolver {
            results: RefCell::new(HashMap::new()),
            name_resolver,
            querier,
            arguments: args,
            positive_cache: (!args.no_positive_cache).then(|| RefCell::new(HashSet::new())),
            negative_cache: args.negative_cache.then(|| RefCell::new(HashSet::new())),
            queries: Cell::new(0),
            output: RefCell::new(Vec::<u8>::new()),
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(ns_name, ns_ip, zone))
    )]
    /// Build a `QueryResult` for a non-authoritative NS delegation with glue records
    fn delegation_response(ns_name: &str, ns_ip: Ipv4Addr, zone: &str) -> QueryResult {
        let ns_name_parsed = Name::from_str(ns_name).expect("ns_name is a valid DNS name literal");
        let zone_name = Name::from_str(zone).expect("zone is a valid DNS name literal");

        let ns_record = Record::from_rdata(
            zone_name,
            3600,
            RData::NS(rdata::NS(ns_name_parsed.clone())),
        );
        let glue_record = Record::from_rdata(ns_name_parsed, 3600, RData::A(rdata::A(ns_ip)));

        QueryResult {
            authoritative: false,
            answers: vec![],
            name_servers: vec![ns_record],
            additionals: vec![glue_record],
            response_code: ResponseCode::NoError,
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(domain, ip))
    )]
    /// Build a `QueryResult` for an authoritative answer with one A record
    fn authoritative_a_response(domain: &str, ip: Ipv4Addr) -> QueryResult {
        let record = Record::from_rdata(
            Name::from_str(domain).expect("domain is a valid DNS name literal"),
            300,
            RData::A(rdata::A(ip)),
        );
        QueryResult {
            authoritative: true,
            answers: vec![record],
            name_servers: vec![],
            additionals: vec![],
            response_code: ResponseCode::NoError,
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip(resolver))
    )]
    fn get_output(
        resolver: RecursiveResolver<'_, MockNameResolver, MockDnsQuerier, Vec<u8>>,
    ) -> String {
        String::from_utf8(resolver.output.into_inner()).expect("output should be valid UTF-8")
    }

    // ── Tests of the creation of the resolver ─────────────────────────────────

    #[test]
    fn recursive_resolver_new() {
        let args = default_args();
        let resolver = RecursiveResolver::new(&args)
            .expect("resolver creation with default args should succeed");

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
        let resolver = RecursiveResolver::new(&args)
            .expect("resolver creation with default args should succeed");

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
        let resolver =
            RecursiveResolver::new(&args).expect("resolver creation with IP server should succeed");

        let servers = resolver
            .init()
            .await
            .expect("init with an IP server should succeed");
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
        assert!(servers[0].name.is_none());
    }

    // ── Tests with mock implementations ───────────────────────────────────────

    #[tokio::test]
    async fn init_with_dot_uses_ns_then_lookup_ip() {
        let args = Args {
            ipv6: true,
            ..default_args() // server = ".", ipv4 = true
        };

        let mut nr = MockNameResolver::new();
        nr.expect_ns_lookup()
            .with(predicate::eq("."))
            .once()
            .returning(|_| {
                Ok(vec![
                    Name::from_str("a.root-servers.net.")
                        .expect("a.root-servers.net. is a valid DNS name"),
                    Name::from_str("b.root-servers.net.")
                        .expect("b.root-servers.net. is a valid DNS name"),
                ])
            });
        nr.expect_lookup_ip()
            .with(predicate::eq("a.root-servers.net."))
            .once()
            .returning(|_| {
                Ok(vec![
                    IpAddr::from([198, 41, 0, 4]),
                    IpAddr::from([0x2001, 0x503, 0xba3e, 0, 0, 0, 0x2, 0x30]),
                ])
            });
        nr.expect_lookup_ip()
            .with(predicate::eq("b.root-servers.net."))
            .once()
            .returning(|_| {
                Ok(vec![
                    IpAddr::from([170, 247, 170, 2]),
                    IpAddr::from([0x2801, 0xb8, 0x10, 0, 0, 0, 0, 0xb]),
                ])
            });

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let servers = resolver.init().await.expect("init with dot should succeed");

        assert_eq!(servers.len(), 4);
        assert_debug_snapshot!(servers, @r#"
        [
            OptName {
                ip: 198.41.0.4,
                name: Some(
                    "a.root-servers.net.",
                ),
                zone: Some(
                    ".",
                ),
            },
            OptName {
                ip: 2001:503:ba3e::2:30,
                name: Some(
                    "a.root-servers.net.",
                ),
                zone: Some(
                    ".",
                ),
            },
            OptName {
                ip: 170.247.170.2,
                name: Some(
                    "b.root-servers.net.",
                ),
                zone: Some(
                    ".",
                ),
            },
            OptName {
                ip: 2801:b8:10::b,
                name: Some(
                    "b.root-servers.net.",
                ),
                zone: Some(
                    ".",
                ),
            },
        ]
        "#);
        assert_snapshot!(get_output(resolver), @"");
    }

    #[tokio::test]
    async fn init_with_dot_filters_ipv6_when_ipv4_only() {
        let args = default_args(); // ipv4 = true, ipv6 = false

        let mut nr = MockNameResolver::new();
        nr.expect_ns_lookup().once().returning(|_| {
            Ok(vec![
                Name::from_str("a.root-servers.net.")
                    .expect("a.root-servers.net. is a valid DNS name"),
                Name::from_str("b.root-servers.net.")
                    .expect("b.root-servers.net. is a valid DNS name"),
            ])
        });
        // Return an IPv4 address and an IPv6 address
        nr.expect_lookup_ip()
            .with(predicate::eq("a.root-servers.net."))
            .once()
            .returning(|_| {
                Ok(vec![
                    IpAddr::from([198, 41, 0, 4]),
                    IpAddr::from([0x2001, 0x503, 0xba3e, 0, 0, 0, 0x2, 0x30]),
                ])
            });
        nr.expect_lookup_ip()
            .with(predicate::eq("b.root-servers.net."))
            .once()
            .returning(|_| {
                Ok(vec![
                    IpAddr::from([170, 247, 170, 2]),
                    IpAddr::from([0x2801, 0xb8, 0x10, 0, 0, 0, 0, 0xb]),
                ])
            });

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let servers = resolver.init().await.expect("init with dot should succeed");

        // Only the IPv4 address remains
        assert_eq!(servers.len(), 2);
        assert!(servers.iter().all(|server| server.ip.is_ipv4()));
        assert_debug_snapshot!(servers, @r#"
        [
            OptName {
                ip: 198.41.0.4,
                name: Some(
                    "a.root-servers.net.",
                ),
                zone: Some(
                    ".",
                ),
            },
            OptName {
                ip: 170.247.170.2,
                name: Some(
                    "b.root-servers.net.",
                ),
                zone: Some(
                    ".",
                ),
            },
        ]
        "#);
        assert_snapshot!(get_output(resolver), @"");
    }

    #[tokio::test]
    async fn init_with_hostname_resolves_ips() {
        let args = Args {
            server: "ns1.example.com".to_owned(),
            ipv6: true,
            ..default_args()
        };

        let mut nr = MockNameResolver::new();
        nr.expect_lookup_ip()
            .with(predicate::eq("ns1.example.com"))
            .once()
            .returning(|_| {
                Ok(vec![
                    IpAddr::from([192, 0, 2, 1]),
                    IpAddr::from([0x2001, 0xDB8, 0, 0, 0, 0, 0, 1]),
                    IpAddr::from([192, 0, 2, 2]),
                    IpAddr::from([0x2001, 0xDB8, 0, 0, 0, 0, 0, 2]),
                ])
            });

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let servers = resolver
            .init()
            .await
            .expect("init with hostname should succeed");

        assert_eq!(servers.len(), 4);
        assert!(
            servers
                .iter()
                .all(|s| s.name == Some("ns1.example.com".to_owned()))
        );
        assert!(servers.iter().all(|s| s.zone.is_none()));
        assert_debug_snapshot!(servers, @r#"
        [
            OptName {
                ip: 192.0.2.1,
                name: Some(
                    "ns1.example.com",
                ),
                zone: None,
            },
            OptName {
                ip: 2001:db8::1,
                name: Some(
                    "ns1.example.com",
                ),
                zone: None,
            },
            OptName {
                ip: 192.0.2.2,
                name: Some(
                    "ns1.example.com",
                ),
                zone: None,
            },
            OptName {
                ip: 2001:db8::2,
                name: Some(
                    "ns1.example.com",
                ),
                zone: None,
            },
        ]
        "#);
        assert_snapshot!(get_output(resolver), @"");
    }

    #[tokio::test]
    async fn init_with_no_results_returns_error() {
        let args = Args {
            server: "ns1.example.com".to_owned(),
            ..default_args()
        };

        let mut nr = MockNameResolver::new();
        // Return only an IPv6 address. The option ipv4=true filters it out.
        nr.expect_lookup_ip().once().returning(|_| {
            let ipv6: IpAddr = "2001:db8::1"
                .parse()
                .expect("2001:db8::1 is a valid IPv6 address");
            Ok(vec![ipv6])
        });

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let result = resolver.init().await;

        assert!(result.is_err());
        assert_debug_snapshot!(result, @"
        Err(
            No IP address found for hostname ns1.example.com, at src/resolver.rs:499:13,
        )
        ");
        assert_snapshot!(get_output(resolver), @"");
    }

    #[tokio::test]
    async fn do_recurse_authoritative_answer_stored_in_results() {
        let args = default_args();
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };

        let mut q = MockDnsQuerier::new();
        q.expect_query().once().returning(|_, _, _| {
            Ok(authoritative_a_response(
                "example.com.",
                Ipv4Addr::new(93, 184, 216, 34),
            ))
        });

        let resolver = mock_resolver(&args, MockNameResolver::new(), q);
        resolver
            .do_recurse(&name, &server, 1, vec![true])
            .await
            .expect("do_recurse should succeed");

        let results = resolver.results.borrow();
        assert_eq!(results.len(), 1);
        let result = results
            .values()
            .next()
            .expect("results map should contain at least one entry");
        assert_eq!(result.response_code, ResponseCode::NoError);
        assert_eq!(result.records.len(), 1);
        assert_debug_snapshot!(results, @r#"
        {
            OptName {
                ip: 192.0.2.1,
                name: Some(
                    "ns1.example.com.",
                ),
                zone: None,
            }: FullResult {
                records: {
                    Record {
                        name: Name("example.com."),
                        dns_class: IN,
                        ttl: 300,
                        data: A(
                            A(
                                93.184.216.34,
                            ),
                        ),
                    },
                },
                response_code: NoError,
            },
        }
        "#);
        drop(results);
        assert_snapshot!(get_output(resolver), @r"  \___ ns1.example.com. (192.0.2.1) found authoritative answer");
    }

    #[tokio::test]
    async fn do_recurse_uses_ns_at_depth_zero() {
        let args = default_args();
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };

        let mut q = MockDnsQuerier::new();
        q.expect_query()
            .withf(|_, _, qtype| *qtype == RecordType::NS)
            .once()
            .returning(|_, _, _| Ok(QueryResult::default()));

        let resolver = mock_resolver(&args, MockNameResolver::new(), q);
        // At depth 0 the query type is NS, for any value of args.query_type
        resolver
            .do_recurse(&name, &server, 0, vec![])
            .await
            .expect("do_recurse should succeed");
        let results = resolver.results.borrow();
        assert!(results.is_empty());
        drop(results);
        assert_snapshot!(get_output(resolver), @"ns1.example.com. (192.0.2.1)");
    }

    #[tokio::test]
    async fn do_recurse_follows_ns_delegation() {
        let args = default_args();
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let first_server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };

        let mut q = MockDnsQuerier::new();
        // First call: a delegation response
        q.expect_query()
            .withf(|server, _, _| server.ip == IpAddr::from([192, 0, 2, 1]))
            .once()
            .returning(|_, _, _| {
                Ok(delegation_response(
                    "ns2.example.com.",
                    Ipv4Addr::new(192, 0, 2, 2),
                    "example.com.",
                ))
            });
        // Second call (recursion): an authoritative answer
        q.expect_query()
            .withf(|server, _, _| server.ip == IpAddr::from([192, 0, 2, 2]))
            .once()
            .returning(|_, _, _| {
                Ok(authoritative_a_response(
                    "example.com.",
                    Ipv4Addr::new(93, 184, 216, 34),
                ))
            });

        let resolver = mock_resolver(&args, MockNameResolver::new(), q);
        resolver
            .do_recurse(&name, &first_server, 1, vec![])
            .await
            .expect("do_recurse should succeed");

        let results = resolver.results.borrow();
        assert_eq!(results.len(), 1);
        assert_debug_snapshot!(results, @r#"
        {
            OptName {
                ip: 192.0.2.2,
                name: Some(
                    "ns2.example.com.",
                ),
                zone: Some(
                    "example.com.",
                ),
            }: FullResult {
                records: {
                    Record {
                        name: Name("example.com."),
                        dns_class: IN,
                        ttl: 300,
                        data: A(
                            A(
                                93.184.216.34,
                            ),
                        ),
                    },
                },
                response_code: NoError,
            },
        }
        "#);
        drop(results);
        assert_snapshot!(get_output(resolver), @r"
        |\___ ns1.example.com. (192.0.2.1)
               |\___ ns2.example.com. [example.com.] (192.0.2.2) found authoritative answer
        ");
    }

    #[tokio::test]
    async fn do_recurse_stops_at_max_queries() {
        let args = Args {
            max_queries: 5,
            ..default_args()
        };
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let first_server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };

        // A hostile server: every answer delegates to a new IP address
        let mut q = MockDnsQuerier::new();
        let mut next = 2_u8;
        q.expect_query().times(5).returning(move |_, _, _| {
            next += 1;
            Ok(delegation_response(
                "ns.example.com.",
                Ipv4Addr::new(192, 0, 2, next),
                "example.com.",
            ))
        });

        let resolver = mock_resolver(&args, MockNameResolver::new(), q);
        let error = resolver
            .do_recurse(&name, &first_server, 0, vec![])
            .await
            .expect_err("do_recurse should stop at the maximum number of queries");

        assert!(
            format!("{error:?}").contains("more than the maximum of 5 queries"),
            "the error should come from the query limit: {error:?}"
        );
        // The mock checks that the code sent exactly 5 queries
    }

    #[tokio::test]
    async fn do_recurse_skips_cached_servers() {
        let args = default_args();
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };

        let mut q = MockDnsQuerier::new();
        q.expect_query().never();

        let resolver = mock_resolver(&args, MockNameResolver::new(), q);
        resolver
            .positive_cache
            .as_ref()
            .expect("positive cache should be initialized")
            .borrow_mut()
            .insert((server.ip, name.clone()));

        resolver
            .do_recurse(&name, &server, 1, vec![])
            .await
            .expect("do_recurse should succeed");
        let results = resolver.results.borrow();
        assert!(results.is_empty());
        drop(results);
        // The mock checks that the code sends no query
        assert_snapshot!(get_output(resolver), @r" |\___ ns1.example.com. (192.0.2.1) (cached)");
    }

    #[tokio::test]
    async fn do_recurse_sets_negative_cache_on_error() {
        let args = Args {
            negative_cache: true,
            ..default_args()
        };
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };

        let mut q = MockDnsQuerier::new();
        q.expect_query().once().returning({
            let name = name.clone();
            move |_, _, _| {
                Err(ResolverError::ClientQuery {
                    name: name.clone(),
                    query_type: RecordType::A,
                }
                .into())
            }
        });

        let resolver = mock_resolver(&args, MockNameResolver::new(), q);
        resolver
            .do_recurse(&name, &server, 1, vec![])
            .await
            .expect("do_recurse should succeed even when the query errors (error is printed, not propagated)");

        let neg = resolver
            .negative_cache
            .as_ref()
            .expect("negative cache should be initialized")
            .borrow();
        assert!(neg.contains(&(server.ip, name.clone())));
        assert_debug_snapshot!(neg, @r#"
        {
            (
                192.0.2.1,
                Name("example.com."),
            ),
        }
        "#);
        drop(neg);
        let results = resolver.results.borrow();
        assert!(results.is_empty());
        drop(results);
        assert_snapshot!(get_output(resolver), @r" |\___ ns1.example.com. (192.0.2.1) Client query failed for A example.com. -> unknown error");
    }

    #[tokio::test]
    async fn get_next_servers_uses_glue_records_from_additionals() {
        let args = default_args();
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let current_server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: None,
            zone: None,
        };

        // The test expects no lookup_ip call. The glue records in the additional section are sufficient.
        let nr = MockNameResolver::new();

        let ns_name =
            Name::from_str("ns1.example.com.").expect("ns1.example.com. is a valid DNS name");
        let zone_name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let ns_record = Record::from_rdata(zone_name, 3600, RData::NS(rdata::NS(ns_name.clone())));
        let glue = Record::from_rdata(ns_name, 3600, RData::A(rdata::A(Ipv4Addr::new(1, 2, 3, 4))));

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let next = resolver
            .get_next_servers(&[ns_record], &[glue], &current_server, &name, 1, &[true])
            .await;

        assert_eq!(next.len(), 1);
        assert_eq!(
            next.iter()
                .next()
                .expect("next_servers should contain one entry")
                .ip,
            IpAddr::from([1, 2, 3, 4])
        );
        let results = resolver.results.borrow();
        assert!(results.is_empty());
        drop(results);
        assert_snapshot!(get_output(resolver), @"");
    }

    #[tokio::test]
    async fn get_next_servers_falls_back_to_lookup_when_no_glue() {
        let args = default_args();
        let name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let current_server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: None,
            zone: None,
        };

        let mut nr = MockNameResolver::new();
        nr.expect_lookup_ip()
            .with(predicate::eq("ns1.example.com."))
            .once()
            .returning(|_| Ok(vec![IpAddr::from([5, 6, 7, 8])]));

        let ns_name =
            Name::from_str("ns1.example.com.").expect("ns1.example.com. is a valid DNS name");
        let zone_name = Name::from_str("example.com.").expect("example.com. is a valid DNS name");
        let ns_record = Record::from_rdata(zone_name, 3600, RData::NS(rdata::NS(ns_name)));

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let next = resolver
            .get_next_servers(
                &[ns_record],
                &[], // no additionals
                &current_server,
                &name,
                1,
                &[true],
            )
            .await;

        assert_eq!(next.len(), 1);
        assert_eq!(
            next.iter()
                .next()
                .expect("next_servers should contain one entry")
                .ip,
            IpAddr::from([5, 6, 7, 8])
        );

        let results = resolver.results.borrow();
        assert!(results.is_empty());
        drop(results);
        assert_snapshot!(get_output(resolver), @"");
    }

    #[test]
    fn query_result_default_values() {
        let qr = QueryResult::default();
        assert!(!qr.authoritative);
        assert_eq!(qr.response_code, ResponseCode::NoError);
        assert_eq!(qr.answers, []);
        assert_eq!(qr.name_servers, []);
        assert_eq!(qr.additionals, []);
    }

    #[test]
    fn show_overview_empty() {
        let args = default_args();
        let resolver = mock_resolver(&args, MockNameResolver::new(), MockDnsQuerier::new());

        resolver
            .show_overview()
            .expect("show_overview on empty results should succeed");

        assert_snapshot!(get_output(resolver), @"");
    }

    #[test]
    fn show_overview_with_record() {
        let args = default_args();
        let resolver = mock_resolver(&args, MockNameResolver::new(), MockDnsQuerier::new());

        let server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };
        let record = Record::from_rdata(
            Name::from_str("example.com.").expect("example.com. is a valid DNS name"),
            300,
            RData::A(rdata::A(Ipv4Addr::new(93, 184, 216, 34))),
        );
        resolver.results.borrow_mut().insert(server, FullResult {
            records: std::collections::BTreeSet::from([record]),
            response_code: ResponseCode::NoError,
        });

        resolver
            .show_overview()
            .expect("show_overview with a record should succeed");

        let results = resolver.results.borrow();
        assert_debug_snapshot!(results, @r#"
        {
            OptName {
                ip: 192.0.2.1,
                name: Some(
                    "ns1.example.com.",
                ),
                zone: None,
            }: FullResult {
                records: {
                    Record {
                        name: Name("example.com."),
                        dns_class: IN,
                        ttl: 300,
                        data: A(
                            A(
                                93.184.216.34,
                            ),
                        ),
                    },
                },
                response_code: NoError,
            },
        }
        "#);
        drop(results);
        assert_snapshot!(get_output(resolver), @"ns1.example.com. (192.0.2.1) 	example.com. 300 IN A 93.184.216.34");
    }

    #[test]
    fn show_overview_with_error_response_code() {
        let args = default_args();
        let resolver = mock_resolver(&args, MockNameResolver::new(), MockDnsQuerier::new());

        let server = OptName {
            ip: IpAddr::from([192, 0, 2, 1]),
            name: Some("ns1.example.com.".to_owned()),
            zone: None,
        };
        resolver.results.borrow_mut().insert(server, FullResult {
            records: std::collections::BTreeSet::new(),
            response_code: ResponseCode::NXDomain,
        });

        resolver
            .show_overview()
            .expect("show_overview with an error response code should succeed");

        let results = resolver.results.borrow();
        assert_debug_snapshot!(results, @r#"
        {
            OptName {
                ip: 192.0.2.1,
                name: Some(
                    "ns1.example.com.",
                ),
                zone: None,
            }: FullResult {
                records: {},
                response_code: NXDomain,
            },
        }
        "#);
        drop(results);
        assert_snapshot!(get_output(resolver), @"ns1.example.com. (192.0.2.1)	Non-Existent Domain");
    }
}
