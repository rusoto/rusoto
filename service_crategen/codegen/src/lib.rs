#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]

extern crate inflector;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate hyper;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use botocore::Service as BotocoreService;
use generator::generate_source;

pub mod botocore;
pub mod generator;
mod serialization;
mod util;

#[derive(Debug)]
pub struct Service {
    pub name: String,
    protocol_date: String,
}

impl Service {
    pub fn new<S>(name: S, protocol_date: S) -> Self
        where S: Into<String> {
        Service {
            name: name.into(),
            protocol_date: protocol_date.into(),
        }
    }
}

pub fn generate(service: Service, output_path: &Path) -> i32 {
    let service = BotocoreService::load(&service.name, &service.protocol_date)
        .expect(&format!("Failed to load service: {:?}", service));

    let output_file = File::create(output_path).expect(&format!(
        "Couldn't open file for writing: {:?}",
        output_path,
    ));

    let mut writer = BufWriter::new(output_file);

    generate_source(&service, &mut writer).expect(&format!("Failed to write file at {:?}", output_path));

    return 1;
}
