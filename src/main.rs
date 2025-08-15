#![doc = include_str!("../README.md")]

use crate::{args::Args, opt_name::OptName, resolver::RecursiveResolver};
use clap::Parser as _;
use eyre::Result;
use hickory_proto::rr::Name;
use std::{process, str::FromStr as _};

/// The arguments
mod args;
/// A nameserver and the zone it is supposed to be authoritative for.
mod opt_name;
/// All the codes in there
mod resolver;

#[tokio::main]
#[allow(clippy::exit, reason = "error")]
#[expect(clippy::print_stderr, clippy::print_stdout, reason = "main")]
async fn main() -> Result<()> {
    // Parse command-line arguments into the Args struct
    let mut arguments = Args::parse();

    if let Err(err) = arguments.validate() {
        eprintln!("Error: {err}");
        process::exit(1);
    }

    let recursor = RecursiveResolver::new(&arguments)?;

    let first_servers = match recursor.init().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let name = Name::from_str(&arguments.domain)?;

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

    if arguments.overview
        && let Err(e) = recursor.show_overview()
    {
        eprintln!("error getting overview: {e}");
    }

    Ok(())
}
