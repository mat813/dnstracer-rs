#![doc = include_str!("../README.md")]

use crate::{args::Args, opt_name::OptName, resolver::RecursiveResolver};
use clap::Parser as _;
use derive_more::Display;
use exn::{Result, ResultExt as _};
use hickory_proto::rr::Name;
use std::str::FromStr as _;

/// The arguments
mod args;
/// A nameserver and the zone it is supposed to be authoritative for.
mod opt_name;
/// All the codes in there
mod resolver;

/// The main body error
#[derive(Debug, Display)]
struct MainError(String);

impl std::error::Error for MainError {}

#[tokio::main]
#[allow(clippy::exit, reason = "error")]
#[expect(clippy::print_stderr, clippy::print_stdout, reason = "main")]
async fn main() -> Result<(), MainError> {
    // Parse command-line arguments into the Args struct
    let mut arguments = Args::parse();

    arguments
        .validate()
        .map_err(|e| MainError(format!("Arguments validation error: {e:?}")))?;

    let recursor = RecursiveResolver::new(&arguments)
        .or_raise(|| MainError("Resolver creation".to_owned()))?;

    let first_servers = recursor
        .init()
        .await
        .or_raise(|| MainError("Getting first servers".to_owned()))?;

    let name = Name::from_str(&arguments.domain)
        .or_raise(|| MainError(format!("Converting {} to a DNS Name", arguments.domain)))?;

    for (index, first_server) in first_servers.iter().enumerate() {
        if index == 0 {
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
        }

        if let Err(error) = recursor
            .do_recurse(&name, first_server, 0, Vec::new())
            .await
        {
            eprintln!("{error}");
        }
    }

    if arguments.overview {
        recursor
            .show_overview()
            .or_raise(|| MainError("Creating overview".to_owned()))?;
    }

    Ok(())
}
