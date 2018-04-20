#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate hoedown;
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate inflector;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate log;
extern crate env_logger;

mod botocore;
mod cargo;
mod commands;
mod config;
mod service;
mod util;
mod doco;

use std::path::Path;

use clap::{Arg, App, SubCommand};

use service::Service;
use config::ServiceConfig;
use botocore::ServiceDefinition;

fn main() {
    let matches = App::new("Rusoto Service Crate Generator")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("check")
            .arg(Arg::with_name("services_config")
                .long("config")
                .short("c")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("generate")
            .arg(Arg::with_name("services_config")
                .long("config")
                .short("c")
                .takes_value(true))
            .arg(Arg::with_name("out_dir")
                .long("outdir")
                .short("o")
                .takes_value(true)
                .required(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("check") {
        let services_config_path = matches.value_of("services_config").unwrap();
        let service_configs = ServiceConfig::load_all(services_config_path).expect("Unable to read services configuration file.");

        commands::check::check(service_configs);
    }

    if let Some(matches) = matches.subcommand_matches("generate") {
        let services_config_path = matches.value_of("services_config").unwrap();
        let service_configs = ServiceConfig::load_all(services_config_path).expect("Unable to read services configuration file.");

        let out_dir = Path::new(matches.value_of("out_dir").unwrap());

        commands::generate::generate_services(&service_configs, out_dir); 
    }
}
