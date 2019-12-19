#![cfg_attr(feature = "nightly-testing", feature(plugin))]

mod botocore;
mod cargo;
mod commands;
mod config;
mod doco;
mod service;
mod util;

use std::path::Path;

use clap::{crate_authors, crate_description, crate_version, App, Arg, SubCommand};

use crate::botocore::ServiceDefinition;
use crate::config::ServiceConfig;
use crate::service::Service;

fn main() {
    let matches = App::new("Rusoto Service Crate Generator")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("check").arg(
                Arg::with_name("services_config")
                    .long("config")
                    .short("c")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .arg(
                    Arg::with_name("services_config")
                        .long("config")
                        .short("c")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("out_dir")
                        .long("outdir")
                        .short("o")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("service")
                        .long("service")
                        .short("s")
                        .takes_value(true)
                        .multiple(true)
                        .required(false),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("check") {
        let services_config_path = matches.value_of("services_config").unwrap();
        let service_configs = ServiceConfig::load_all(services_config_path)
            .expect("Unable to read services configuration file.");

        commands::check::check(service_configs);
    }

    if let Some(matches) = matches.subcommand_matches("generate") {
        let services_config_path = matches.value_of("services_config").unwrap();
        let service_configs = ServiceConfig::load_all(services_config_path)
            .expect("Unable to read services configuration file.");

        let out_dir = Path::new(matches.value_of("out_dir").unwrap());
        let service: Option<Vec<&str>> = matches
            .values_of("service")
            .map(std::iter::Iterator::collect);

        commands::generate::generate_services(&service_configs, out_dir, service.as_ref());
    }
}
