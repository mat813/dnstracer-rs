#![doc = include_str!("../README.md")]
#![warn(
    clippy::allow_attributes,
    clippy::as_ptr_cast_mut,
    clippy::as_underscore,
    clippy::assigning_clones,
    clippy::borrow_as_ptr,
    clippy::branches_sharing_code,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::if_then_some_else_none,
    clippy::match_like_matches_macro,
    clippy::match_same_arms,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::option_as_ref_deref,
    clippy::pedantic
)]

use crate::{args::Args, opt_name::OptName, resolver::RecursiveResolver};
use clap::Parser;
use hickory_client::rr::Name;
use std::{process, str::FromStr};

/// The arguments
mod args;
/// A nameserver and the zone it is supposed to be authoritative for.
mod opt_name;
/// All the codes in there
mod resolver;

#[tokio::main]
async fn main() {
    // Parse command-line arguments into the Args struct
    let mut arguments = Args::parse();

    if let Err(err) = arguments.validate() {
        eprintln!("Error: {err}");
        process::exit(1);
    }

    let recursor = RecursiveResolver::new(arguments.clone());

    let first_servers = match recursor.init().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let name = Name::from_str(&arguments.domain).expect("Unable to parse domain name");

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

        recursor
            .do_recurse(&name, first_server.clone(), 0, Vec::new())
            .await;
    }

    if arguments.overview {
        recursor.show_overview();
    }
}
