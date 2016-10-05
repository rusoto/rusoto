#![cfg_attr(feature = "unstable", feature(const_fn, core_intrinsics, drop_types_in_const))]
#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]

extern crate inflector;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate serde_json;

#[cfg(not(feature = "serde_macros"))]
extern crate serde_codegen;
#[cfg(not(feature = "serde_macros"))]
extern crate syntex;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use botocore::Service as BotocoreService;
use generator::generate_source;

mod botocore;
mod generator;
mod serialization;
mod util;

const BOTOCORE_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/botocore/botocore/data/");

pub struct Service {
    name: String,
    protocol_date: String,
}

impl Service {
    pub fn new<S>(name: S, protocol_date: S) -> Self
        where S: Into<String>
    {
        Service {
            name: name.into(),
            protocol_date: protocol_date.into(),
        }
    }
}

pub fn generate(service: Service, output_path: &Path) {
    let botocore_destination_path = output_path.join(format!("{}_botocore.rs", service.name));
    let serde_destination_path = output_path.join(format!("{}.rs", service.name));
    let botocore_service_data_path = Path::new(BOTOCORE_DIR)
        .join(format!("{}/{}/service-2.json", service.name, service.protocol_date));

    botocore_generate(botocore_service_data_path.as_path(),
                      botocore_destination_path.as_path());
    serde_generate(botocore_destination_path.as_path(),
                   serde_destination_path.as_path());
}

fn botocore_generate(input_path: &Path, output_path: &Path) {
    let mut input_file = File::open(input_path).expect(&format!(
        "{:?} not found",
        input_path,
    ));

    let mut service_data = String::new();

    input_file.read_to_string(&mut service_data).expect(&format!(
        "Failed to read {:?}",
        input_path,
    ));

    let service: BotocoreService = serde_json::from_str(&service_data).expect(&format!(
        "Could not convert JSON in {:?} to Service",
        input_path,
    ));

    let source_code = generate_source(&service);

    let mut output_file = File::create(output_path).expect(&format!(
        "Couldn't open file for writing: {:?}",
        output_path,
    ));

    output_file.write_all(source_code.as_bytes()).expect(&format!(
        "Failed to write generated source code to {:?}",
        output_path,
    ));
}

#[cfg(not(feature = "serde_macros"))]
fn serde_generate(source: &Path, destination: &Path) {
    ::serde_codegen::expand(&source, &destination).unwrap();
}

#[cfg(feature = "serde_macros")]
fn serde_generate(source: &Path, destination: &Path) {
    ::std::fs::copy(source, destination).expect(&format!(
        "Failed to copy {:?} to {:?}",
        source,
        destination,
    ));
}
