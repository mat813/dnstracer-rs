//! # dnstracer
//!
//! `dnstracer` is a Rust-based implementation of `dnstracer`, a DNS tracing tool that follows the chain of DNS servers responsible for resolving a domain name. It helps trace the delegation path from the root DNS servers down to the authoritative servers for a specific domain.
//!
//! ## Features
//!
//! - Traces DNS delegation paths for domain names.
//! - Supports both IPv4 and IPv6 addresses.
//! - Allows specifying custom DNS query types.
//! - Handles recursive and authoritative DNS queries.
//! - Provides detailed output of DNS servers and records.
//!
//! ## Installation
//!
//! ### Requirements
//!
//! - [Rust](https://www.rust-lang.org/tools/install) (version 1.XX or higher)
//! - Cargo (Rust's package manager)
//!
//! ### Installing dnstracer
//!
//! Clone the repository and build the project using `cargo`:
//!
//! ```bash
//! cargo install dnstracer
//! ```
//!
//! ### Running
//!
//! Once built, you can run `dnstracer-rs` from the target directory:
//!
//! ```bash
//! dnstracer [options] <domain>
//! ```
//!
//! ### Example
//!
//! ```bash
//! dnstracer example.com
//! ```
//!
//! This will trace the DNS delegation for the domain `example.com`, showing the path of DNS servers involved in the resolution.
//!
//! ## Usage
//!
//! ```bash
//! dnstracer [OPTIONS] <domain>
//! ```
//!
//! ### Options
//!
//! - `-c`, `--no-positive-cache`
//!   - disable positive response caching, default enabled
//! - `-C`, `--negative-cache`
//!   - enable negative response caching, default disabled
//! - `-e`, `--edns0`
//!   - disable EDNS0, default enabled
//! - `-o`, `--overview`
//!   - enable overview of received answers, default disabled
//! - `-q`, `--query-type <QUERY_TYPE>`
//!   - The type of record (A, AAAA, NS ...) [default: A]
//! - `-r`, `--retries <RETRIES>`
//!   - amount of retries for DNS requests, default 3 [default: 3]
//! - `-s`, `--server <SERVER>`
//!   - Start the query at the given DNS server (IP or hostname) If `.` is specified, A.ROOT-SERVERS.NET will be used [default: .]
//! - `-t`, `--timeout <TIMEOUT>`
//!   - Limit time to wait per try [default: 5]
//! - `-S`, `--source-address <SOURCE_ADDRESS>`
//!   - use this source address
//! - `-6`, `--ipv6`
//!   - Force using IPv6 for DNS queries (no IPv4)
//! - `-4`, `--ipv4`
//!   - Force using IPv4 for DNS queries (no IPv6)
//! - `-h`, `--help`
//!   - Print help
//! - `-V`, `--version`
//!   - Print version
//!
//! ## Output
//!
//! `dnstracer` provides detailed output for each DNS server in the delegation chain:
//!
//! - The IP address and name of each DNS server.
//! - Whether the response is authoritative or not.
//! - The DNS records associated with the query.
//!
//! ### Sample Output
//!
//! ```text
//! $ dnstracer www.example.com -o             
//! Tracing to www.example.com[A] via A.ROOT-SERVERS.NET. (198.41.0.4), maximum of 3 retries
//! A.ROOT-SERVERS.NET. [.] (198.41.0.4)
//!  |\___ a.gtld-servers.net. [com] (192.5.6.30)
//!  |      |\___ ns1.example.com. [example.com] (192.0.2.1) found authoritative answer
//!  |      |\___ ns2.example.com. [example.com] (198.51.100.1) found authoritative answer
//!  |       \___ ns2.example.com. [example.com] (2001:db8::1) found authoritative answer
//!  |\___ a.gtld-servers.net. [com] (2001:503:a83e::2:30)
//!  |      |\___ ns1.example.com. [example.com] (192.0.2.1) (cached)
//!  |      |\___ ns2.example.com. [example.com] (198.51.100.1) (cached)
//!  |       \___ ns2.example.com. [example.com] (2001:db8::1) (cached)
//!  |\___ b.gtld-servers.net. [com] (192.33.14.30)
//!  ...
//! ns1.example.com. (192.0.2.1) 	www.example.com. 86400 IN A 203.0.113.1
//! ns2.example.com. (198.51.100.1) 	www.example.com. 86400 IN A 203.0.113.1
//! ns2.example.com. (2001:db8::1) 	www.example.com. 86400 IN A 203.0.113.1
//! ```

use crate::{args::Args, opt_name::OptName, resolver::RecursiveResolver};
use clap::Parser;
use hickory_client::rr::Name;
use std::{process, str::FromStr};

mod args;
mod opt_name;
mod resolver;

fn main() {
    // Parse command-line arguments into the Args struct
    let arguments = Args::parse();

    let recursor = RecursiveResolver::new(arguments.clone());

    let first_server = match recursor.init() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let name = Name::from_str(&arguments.domain).expect("Unable to parse domain name");

    println!(
        "Tracing to {}[{}] via {}, maximum of {} retries",
        name,
        arguments.query_type,
        OptName {
            zone: None,
            ..first_server.clone()
        },
        arguments.retries
    );

    recursor.do_recurse(&name, first_server, 0, Vec::new());

    if arguments.overview {
        recursor.show_overview();
    }
}
