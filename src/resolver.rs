use crate::{args::Args, opt_name::OptName};
use derive_more::{Debug, Display};
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
    io::{self, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    sync::{Mutex, RwLock},
};

/// A container for all resolver errors
#[derive(Debug, Display)]
#[allow(clippy::missing_docs_in_private_items, reason = "self explainatory")]
pub enum ResolverError {
    #[display("NS lookup failed for {_0}")]
    NsLookup(String),
    #[display("IP lookup failed for {_0}")]
    IpLookup(String),
    #[display("Client connect failed for {_0}")]
    ClientConnect(OptName),
    #[display("Client query failed for {_1} {_0}")]
    ClientQuery(Name, RecordType),
    #[display("Client creation failed")]
    ClientNew(OptName),
    #[display("Failed to build tokio resolver")]
    BuildTokioResolver,
    #[display("No IP address found for hostname: {_0}")]
    NoIpForHostname(String),
    #[display("do recurse failed")]
    DoRecurse,
    #[display("Failed to acquire read lock")]
    ReadLock,
    #[display("Failed to acquire write lock")]
    WriteLock,
    #[display("Write error")]
    Write,
}

impl std::error::Error for ResolverError {}

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

/// Trait for resolving DNS names to IPs — used for bootstrapping and NS name resolution
#[cfg_attr(test, mockall::automock)]
pub trait NameResolver: Send + Sync {
    /// Look up the NS records for a zone
    async fn ns_lookup(&self, name: &str) -> Result<Vec<Name>, ResolverError>;
    /// Look up the IP addresses for a hostname
    async fn lookup_ip(&self, name: &str) -> Result<Vec<IpAddr>, ResolverError>;
}

/// Simplified DNS query result containing only the fields we use
#[derive(Clone, Debug)]
pub struct QueryResult {
    /// Whether the response is authoritative
    pub authoritative: bool,
    /// Answer section records
    pub answers: Vec<Record>,
    /// Authority section records
    pub name_servers: Vec<Record>,
    /// Additional section records
    pub additionals: Vec<Record>,
    /// Response code
    pub response_code: ResponseCode,
}

impl Default for QueryResult {
    fn default() -> Self {
        Self {
            authoritative: false,
            answers: vec![],
            name_servers: vec![],
            additionals: vec![],
            response_code: ResponseCode::NoError,
        }
    }
}

impl From<DnsResponse> for QueryResult {
    fn from(resp: DnsResponse) -> Self {
        Self {
            authoritative: resp.authoritative(),
            answers: resp.answers().to_vec(),
            name_servers: resp.name_servers().to_vec(),
            additionals: resp.additionals().to_vec(),
            response_code: resp.response_code(),
        }
    }
}

/// Trait for making direct DNS queries to a nameserver
#[cfg_attr(test, mockall::automock)]
pub trait DnsQuerier: Send + Sync {
    /// Send a DNS query to the given server
    async fn query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<QueryResult, ResolverError>;
}

/// Real DNS name resolver wrapping hickory's `TokioResolver`
pub struct TokioNameResolver(TokioResolver);

impl NameResolver for TokioNameResolver {
    async fn ns_lookup(&self, name: &str) -> Result<Vec<Name>, ResolverError> {
        Ok(self
            .0
            .ns_lookup(name)
            .await
            .or_raise(|| ResolverError::NsLookup(name.to_owned()))?
            .iter()
            .map(|ns| ns.0.clone())
            .collect())
    }

    async fn lookup_ip(&self, name: &str) -> Result<Vec<IpAddr>, ResolverError> {
        Ok(self
            .0
            .lookup_ip(name)
            .await
            .or_raise(|| ResolverError::IpLookup(name.to_owned()))?
            .iter()
            .collect())
    }
}

/// Configuration needed for direct DNS queries, extracted from Args at construction time
pub struct DefaultDnsQuerier {
    /// Use TCP instead of UDP
    tcp: bool,
    /// Query timeout
    timeout: std::time::Duration,
    /// Optional source address to bind to
    source_address: Option<IpAddr>,
    /// Whether EDNS0 is disabled
    no_edns0: bool,
}

impl DefaultDnsQuerier {
    /// Create from CLI arguments
    const fn from_args(args: &Args) -> Self {
        Self {
            tcp: args.tcp,
            timeout: args.timeout,
            source_address: args.source_address,
            no_edns0: args.no_edns0,
        }
    }

    /// Make a UDP DNS query
    async fn udp_query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<DnsResponse, ResolverError> {
        let stream = UdpClientStream::builder(server.into(), TokioRuntimeProvider::new())
            .with_timeout(Some(self.timeout))
            .with_bind_addr(self.source_address.map(|ip| SocketAddr::new(ip, 0)))
            .build();
        let (mut client, bg) = Client::connect(stream)
            .await
            .or_raise(|| ResolverError::ClientConnect(server.clone()))?;

        if self.no_edns0 {
            client.disable_edns();
        } else {
            client.enable_edns();
        }

        tokio::spawn(bg);

        client
            .query(name.clone(), DNSClass::IN, query_type)
            .await
            .or_raise(|| ResolverError::ClientQuery(name.clone(), query_type))
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
            self.source_address.map(|ip| SocketAddr::new(ip, 0)),
            Some(self.timeout),
            TokioRuntimeProvider::new(),
        );

        let (mut client, bg) = Client::new(stream, sender, None)
            .await
            .or_raise(|| ResolverError::ClientNew(server.clone()))?;

        if self.no_edns0 {
            client.disable_edns();
        } else {
            client.enable_edns();
        }

        tokio::spawn(bg);

        client
            .query(name.clone(), DNSClass::IN, query_type)
            .await
            .or_raise(|| ResolverError::ClientQuery(name.clone(), query_type))
    }
}

impl DnsQuerier for DefaultDnsQuerier {
    async fn query(
        &self,
        server: &OptName,
        name: &Name,
        query_type: RecordType,
    ) -> Result<QueryResult, ResolverError> {
        let response = if self.tcp {
            self.tcp_query(server, name, query_type).await
        } else {
            self.udp_query(server, name, query_type).await
        }?;
        Ok(QueryResult::from(response))
    }
}

/// Recursive resolver
#[derive(Debug)]
pub struct RecursiveResolver<'a, R = TokioNameResolver, Q = DefaultDnsQuerier, W = io::Stdout> {
    /// Store the results, in case we need to display them
    results: RwLock<HashMap<OptName, FullResult>>,
    /// Resolver for bootstrapping and NS name resolution
    #[debug(skip)]
    name_resolver: R,
    /// Client for making direct DNS queries
    #[debug(skip)]
    querier: Q,
    /// Copy of all the arguments for easier processing
    arguments: &'a Args,
    /// Positive answer cache
    positive_cache: Option<RwLock<Cache>>,
    /// Negative answer cache
    negative_cache: Option<RwLock<Cache>>,
    /// Output writer
    #[debug(skip)]
    output: Mutex<W>,
}

impl<'a> RecursiveResolver<'a> {
    /// Create a new recursive resolver with real DNS implementations
    pub fn new(args: &'a Args) -> Result<Self, ResolverError> {
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;
        resolver_opts.attempts = args.retries;
        resolver_opts.timeout = args.timeout;
        resolver_opts.edns0 = !args.no_edns0;

        let resolver = TokioResolver::builder(GenericConnector::new(TokioRuntimeProvider::new()))
            .or_raise(|| ResolverError::BuildTokioResolver)?
            .with_options(resolver_opts)
            .build();

        Ok(Self {
            results: RwLock::new(HashMap::new()),
            name_resolver: TokioNameResolver(resolver),
            querier: DefaultDnsQuerier::from_args(args),
            positive_cache: (!args.no_positive_cache).then(|| RwLock::new(HashSet::new())),
            negative_cache: args.negative_cache.then(|| RwLock::new(HashSet::new())),
            arguments: args,
            output: Mutex::new(io::stdout()),
        })
    }
}

impl<R: NameResolver, Q: DnsQuerier, W: Write + Send> RecursiveResolver<'_, R, Q, W> {
    /// Is the given IP allowed given the user's IPv4/IPv6 preferences?
    const fn is_ip_allowed(&self, ip: IpAddr) -> bool {
        ip.is_ipv4() && self.arguments.ipv4 // if we asked for ipv4 only
            || ip.is_ipv6() && self.arguments.ipv6 // if we asked for ipv6 only
            || !(self.arguments.ipv4 || self.arguments.ipv6) // if we did not ask anything
    }

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
            let root_ns = self.name_resolver.ns_lookup(".").await?;

            for ns in root_ns {
                let ns_str = ns.to_string();
                results.append(
                    &mut self
                        .name_resolver
                        .lookup_ip(&ns_str)
                        .await
                        .or_raise(|| ResolverError::IpLookup(ns_str))?
                        .into_iter()
                        .filter(|ip| self.is_ip_allowed(*ip))
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
                    .name_resolver
                    .lookup_ip(&self.arguments.server)
                    .await
                    .or_raise(|| ResolverError::IpLookup(self.arguments.server.clone()))?
                    .into_iter()
                    .filter(|ip| self.is_ip_allowed(*ip))
                    .map(|ip| OptName {
                        ip,
                        name: Some(self.arguments.server.clone()),
                        zone: None,
                    })
                    .collect(),
            );
        }

        if results.is_empty() {
            bail!(ResolverError::NoIpForHostname(
                self.arguments.server.clone()
            ));
        }

        Ok(results)
    }

    /// Recurse through the internet looking for answers
    pub fn do_recurse<'b>(
        &'b self,
        name: &'b Name,
        server: &'b OptName,
        depth: usize,
        last: Vec<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ResolverError>> + 'b>> {
        Box::pin(async move {
            if self.cache_get(&(server.ip, name.clone())) {
                self.print(depth, server, "(cached)", &last);
                return Ok(());
            }

            let query_type = if depth == 0 {
                // First request is always a NS request, in case the given server is a recursive server.
                RecordType::NS
            } else {
                self.arguments.query_type
            };

            match self.querier.query(server, name, query_type).await {
                Ok(response) => {
                    let mut next_servers: Option<Vec<OptName>> = None;

                    if response.authoritative {
                        // If the response is authoritative, we are probably at the end of the journey.
                        let result = &response.answers;
                        self.print(depth, server, "found authoritative answer", &last);
                        self.cache_set(true, (server.ip, name.clone()));
                        self.add_result(server.clone(), response.response_code, result)?;

                        // But if we get only CNAMEs and we asked for something
                        // else, we need to try and continue the recursion.
                        if self.arguments.query_type != RecordType::CNAME
                            && result.iter().all(|r| r.record_type() == RecordType::CNAME)
                            && !response.name_servers.is_empty()
                        {
                            for cname in response
                                .answers
                                .iter()
                                // We already checked every entry was a CNAME
                                .filter_map(|r| r.data().as_cname())
                            {
                                next_servers = Some(
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
                        }
                    } else {
                        self.print(depth, server, "", &last);

                        let (records, additionals) = if depth == 0 && !response.answers.is_empty() {
                            // If we're at the start and we get answers, it means it was a recursive name server, so use those answers.
                            (&response.answers, &response.additionals)
                        } else {
                            // Otherwise, we get an authority section.
                            (&response.name_servers, &response.additionals)
                        };

                        next_servers = Some(
                            self.get_next_servers(records, additionals, server, name, depth, &last)
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
                            .or_raise(|| ResolverError::DoRecurse)?;
                        }
                    }
                }
                Err(e) => {
                    self.cache_set(false, (server.ip, name.clone()));
                    self.print(
                        depth,
                        server,
                        format!(
                            "{e} -> {}",
                            e.frame().children().first().map_or_else(
                                || "unknown error".to_owned(),
                                std::string::ToString::to_string
                            )
                        ),
                        &last,
                    );
                }
            }
            Ok(())
        })
    }

    /// Figure out the next servers in the recursion
    async fn get_next_servers(
        &self,
        records: &[Record],
        additionals: &[Record],
        server: &OptName,
        name: &Name,
        depth: usize,
        last: &[bool],
    ) -> Vec<OptName> {
        let mut next_servers: Vec<OptName> = vec![];

        for record in records {
            // Here, we know it's a NS, so unwrap all that.
            let Some(ns) = record.data().as_ns() else {
                continue;
            };

            // Some name servers will respond with an additional section, use it
            let before_len = next_servers.len();
            next_servers.append(
                &mut additionals
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
                    .filter(|&(_, ip)| self.is_ip_allowed(ip))
                    .map(|(additional, ip)| OptName {
                        ip,
                        name: Some(additional.name().to_string()),
                        zone: Some(record.name().to_string()),
                    })
                    .collect(),
            );

            // If we don't have an additional section or we had stuff
            // but not this name server, we need to resolve it
            // ourselves.
            if next_servers.len() == before_len {
                let ns_s = ns.to_string();
                let before_resolve_len = next_servers.len();
                if let Ok(ips) = self.name_resolver.lookup_ip(&ns_s).await {
                    next_servers.append(
                        &mut ips
                            .into_iter()
                            .filter(|ip| self.is_ip_allowed(*ip))
                            .map(|ip| OptName {
                                ip,
                                name: Some(ns.to_string()),
                                zone: Some(record.name().to_string()),
                            })
                            .collect(),
                    );
                }

                if next_servers.len() > before_resolve_len {
                    self.cache_set(true, (server.ip, name.clone()));
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
                        last,
                    );
                }
            } else {
                self.cache_set(true, (server.ip, name.clone()));
            }
        }

        next_servers
    }

    /// Print the overview
    pub fn show_overview(&self) -> Result<(), ResolverError> {
        use fmt::Write as _;

        let results = self.results.read().map_err(|_| ResolverError::ReadLock)?;
        let mut buf = String::new();
        for (key, values) in results.iter() {
            if values.response_code != ResponseCode::NoError {
                let _ = writeln!(
                    buf,
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
                let _ = writeln!(
                    buf,
                    "{} ({}) \t{record}",
                    key.name.as_deref().unwrap_or_default(),
                    key.ip
                );
            }
        }
        drop(results);
        self.output
            .lock()
            .map_err(|_| ResolverError::WriteLock)?
            .write_all(buf.as_bytes())
            .map_err(|_| ResolverError::Write)?;
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
        let cache = if positive {
            &self.positive_cache
        } else {
            &self.negative_cache
        };
        #[expect(clippy::pattern_type_mismatch, reason = "can't dereference guard")]
        if let Some(locked_cache) = cache {
            match locked_cache.write() {
                Ok(mut c) => {
                    c.insert(key);
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
        let mut res = self.results.write().map_err(|_| ResolverError::WriteLock)?;
        let full = res.entry(server).or_default();

        full.response_code = response_code;

        for result in results {
            full.records.insert(result.clone());
        }

        Ok(())
    }

    /// Try to give a nice out, as the original did
    fn print(&self, depth: usize, server: &OptName, rest: impl fmt::Display, last: &[bool]) {
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
        if let Ok(mut w) = self.output.lock() {
            let _ = w.write_all(line.as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::indexing_slicing, reason = "test")]

    use super::*;
    use crate::args::Args;
    use hickory_proto::rr::{Name, RData, Record, RecordType, rdata};
    use insta::assert_debug_snapshot;
    use mockall::predicate;
    use std::{
        net::{IpAddr, Ipv4Addr},
        str::FromStr as _,
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

    /// Build a `RecursiveResolver` with mock implementations for testing (output discarded)
    fn mock_resolver(
        args: &Args,
        name_resolver: MockNameResolver,
        querier: MockDnsQuerier,
    ) -> RecursiveResolver<'_, MockNameResolver, MockDnsQuerier, Vec<u8>> {
        RecursiveResolver {
            results: RwLock::new(HashMap::new()),
            name_resolver,
            querier,
            arguments: args,
            positive_cache: (!args.no_positive_cache).then(|| RwLock::new(HashSet::new())),
            negative_cache: args.negative_cache.then(|| RwLock::new(HashSet::new())),
            output: Mutex::new(Vec::<u8>::new()),
        }
    }

    /// Build a `QueryResult` representing a non-authoritative NS delegation with glue records
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

    /// Build a `QueryResult` representing an authoritative answer with one A record
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

    fn get_output(
        resolver: RecursiveResolver<'_, MockNameResolver, MockDnsQuerier, Vec<u8>>,
    ) -> String {
        String::from_utf8(
            resolver
                .output
                .into_inner()
                .expect("output lock should not be poisoned"),
        )
        .expect("output should be valid UTF-8")
    }

    // ── Existing tests (kept as-is) ───────────────────────────────────────────

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

    // ── New mock-based tests ──────────────────────────────────────────────────

    #[tokio::test]
    async fn init_with_dot_uses_ns_then_lookup_ip() {
        let args = default_args(); // server = ".", ipv4 = true

        let mut nr = MockNameResolver::new();
        nr.expect_ns_lookup()
            .with(predicate::eq("."))
            .once()
            .returning(|_| {
                Ok(vec![
                    Name::from_str("a.root-servers.net.")
                        .expect("a.root-servers.net. is a valid DNS name"),
                ])
            });
        nr.expect_lookup_ip()
            .with(predicate::eq("a.root-servers.net."))
            .once()
            .returning(|_| Ok(vec![IpAddr::from([198, 41, 0, 4])]));

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let servers = resolver.init().await.expect("init with dot should succeed");

        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].ip, IpAddr::from([198, 41, 0, 4]));
        assert_eq!(servers[0].name, Some("a.root-servers.net.".to_owned()));
        assert_eq!(servers[0].zone, Some(".".to_owned()));
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
        ]
        "#);
        assert_debug_snapshot!(get_output(resolver), @r#""""#);
    }

    #[tokio::test]
    async fn init_with_dot_filters_ipv6_when_ipv4_only() {
        let args = default_args(); // ipv4 = true, ipv6 = false

        let mut nr = MockNameResolver::new();
        nr.expect_ns_lookup().once().returning(|_| {
            Ok(vec![
                Name::from_str("a.root-servers.net.")
                    .expect("a.root-servers.net. is a valid DNS name"),
            ])
        });
        // Return both an IPv4 and an IPv6 address
        nr.expect_lookup_ip().once().returning(|_| {
            let ipv6: IpAddr = "2001:503:ba3e::2:30"
                .parse()
                .expect("2001:503:ba3e::2:30 is a valid IPv6 address");
            Ok(vec![IpAddr::from([198, 41, 0, 4]), ipv6])
        });

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let servers = resolver.init().await.expect("init with dot should succeed");

        // Only the IPv4 address should be kept
        assert_eq!(servers.len(), 1);
        assert!(servers[0].ip.is_ipv4());
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
        ]
        "#);
        assert_debug_snapshot!(get_output(resolver), @r#""""#);
    }

    #[tokio::test]
    async fn init_with_hostname_resolves_ips() {
        let args = Args {
            server: "ns1.example.com".to_owned(),
            ..default_args()
        };

        let mut nr = MockNameResolver::new();
        nr.expect_lookup_ip()
            .with(predicate::eq("ns1.example.com"))
            .once()
            .returning(|_| {
                Ok(vec![
                    IpAddr::from([192, 0, 2, 1]),
                    IpAddr::from([192, 0, 2, 2]),
                ])
            });

        let resolver = mock_resolver(&args, nr, MockDnsQuerier::new());
        let servers = resolver
            .init()
            .await
            .expect("init with hostname should succeed");

        assert_eq!(servers.len(), 2);
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
                ip: 192.0.2.2,
                name: Some(
                    "ns1.example.com",
                ),
                zone: None,
            },
        ]
        "#);
        assert_debug_snapshot!(get_output(resolver), @r#""""#);
    }

    #[tokio::test]
    async fn init_with_no_results_returns_error() {
        let args = Args {
            server: "ns1.example.com".to_owned(),
            ..default_args()
        };

        let mut nr = MockNameResolver::new();
        // Return only an IPv6 address but ipv4=true means it gets filtered out
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
            No IP address found for hostname: ns1.example.com, at src/resolver.rs:370:13,
        )
        ");
        assert_debug_snapshot!(get_output(resolver), @r#""""#);
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

        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
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
                        name_labels: Name("example.com."),
                        dns_class: IN,
                        ttl: 300,
                        rdata: A(
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
        assert_debug_snapshot!(get_output(resolver), @r#""  \\___ ns1.example.com. (192.0.2.1) found authoritative answer\n""#);
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
        // depth = 0 forces NS query type regardless of args.query_type
        resolver
            .do_recurse(&name, &server, 0, vec![])
            .await
            .expect("do_recurse should succeed");
        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
        assert!(results.is_empty());
        drop(results);
        assert_debug_snapshot!(get_output(resolver), @r#""ns1.example.com. (192.0.2.1)\n""#);
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
        // First call: delegation response
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
        // Second call (recursion): authoritative answer
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

        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
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
                        name_labels: Name("example.com."),
                        dns_class: IN,
                        ttl: 300,
                        rdata: A(
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
        assert_debug_snapshot!(get_output(resolver), @r#"" |\\___ ns1.example.com. (192.0.2.1)\n        |\\___ ns2.example.com. [example.com.] (192.0.2.2) found authoritative answer\n""#);
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
            .write()
            .expect("positive cache lock should not be poisoned")
            .insert((server.ip, name.clone()));

        resolver
            .do_recurse(&name, &server, 1, vec![])
            .await
            .expect("do_recurse should succeed");
        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
        assert!(results.is_empty());
        drop(results);
        // The mock verifies query was never called
        assert_debug_snapshot!(get_output(resolver), @r#"" |\\___ ns1.example.com. (192.0.2.1) (cached)\n""#);
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
            move |_, _, _| Err(ResolverError::ClientQuery(name.clone(), RecordType::A).into())
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
            .read()
            .expect("negative cache lock should not be poisoned");
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
        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
        assert!(results.is_empty());
        drop(results);
        assert_debug_snapshot!(get_output(resolver), @r#"" |\\___ ns1.example.com. (192.0.2.1) Client query failed for A example.com. -> unknown error\n""#);
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

        // No lookup_ip call expected — glue records in additionals are sufficient
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
        assert_eq!(next[0].ip, IpAddr::from([1, 2, 3, 4]));
        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
        assert!(results.is_empty());
        drop(results);
        assert_debug_snapshot!(get_output(resolver), @r#""""#);
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
        assert_eq!(next[0].ip, IpAddr::from([5, 6, 7, 8]));

        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
        assert!(results.is_empty());
        drop(results);
        assert_debug_snapshot!(get_output(resolver), @r#""""#);
    }

    #[test]
    fn query_result_default_values() {
        let qr = QueryResult::default();
        assert!(!qr.authoritative);
        assert_eq!(qr.response_code, ResponseCode::NoError);
        assert!(qr.answers.is_empty());
        assert!(qr.name_servers.is_empty());
        assert!(qr.additionals.is_empty());
    }

    #[test]
    fn show_overview_empty() {
        let args = default_args();
        let resolver = mock_resolver(&args, MockNameResolver::new(), MockDnsQuerier::new());

        resolver
            .show_overview()
            .expect("show_overview on empty results should succeed");

        assert_debug_snapshot!(get_output(resolver), @r#""""#);
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
        resolver
            .results
            .write()
            .expect("results lock should not be poisoned")
            .insert(
                server,
                FullResult {
                    records: std::collections::BTreeSet::from([record]),
                    response_code: ResponseCode::NoError,
                },
            );

        resolver
            .show_overview()
            .expect("show_overview with a record should succeed");

        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
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
                        name_labels: Name("example.com."),
                        dns_class: IN,
                        ttl: 300,
                        rdata: A(
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
        assert_debug_snapshot!(get_output(resolver), @r#""ns1.example.com. (192.0.2.1) \texample.com. 300 IN A 93.184.216.34\n""#);
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
        resolver
            .results
            .write()
            .expect("results lock should not be poisoned")
            .insert(
                server,
                FullResult {
                    records: std::collections::BTreeSet::new(),
                    response_code: ResponseCode::NXDomain,
                },
            );

        resolver
            .show_overview()
            .expect("show_overview with an error response code should succeed");

        let results = resolver
            .results
            .read()
            .expect("results lock should not be poisoned");
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
        assert_debug_snapshot!(get_output(resolver), @r#""ns1.example.com. (192.0.2.1)\tNon-Existent Domain\n""#);
    }
}
