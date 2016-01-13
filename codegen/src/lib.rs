extern crate serde_codegen;
extern crate syntex;

use std::env::var_os;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_codegen::register;
use syntex::Registry;

const BOTOCORE_DIR: &'static str = "codegen/botocore/botocore/data/";

pub struct Generator {
    botocore_path: PathBuf,
    out_path: PathBuf,
}

impl Generator {
    pub fn new() -> Result<Generator, String> {
        let out_dir = try!(var_os("OUT_DIR").ok_or("OUT_DIR not specified"));
        let out_path = PathBuf::from(&out_dir);

        let botocore_dir = var_os("BOTOCORE_DIR");
        let botocore_path = match botocore_dir {
            Some(ref dirname) => PathBuf::from(dirname),
            None => PathBuf::from(BOTOCORE_DIR)
        };

        Ok(Generator {
            botocore_path: botocore_path,
            out_path: out_path,
        })
    }

    pub fn generate(&self, services: &[Service]) -> Result<(), String> {
        for service in services {
            try!(generate(service, self.botocore_path.as_path(), self.out_path.as_path()));
        }

        Ok(())
    }
}

pub struct Service {
    name: String,
    type_name: String,
    protocol_date: String
}

impl Service {
    pub fn new<S: ToString>(name: S, type_name: S, protocol_date: S) -> Service {
        Service {
            name: name.to_string(),
            type_name: type_name.to_string(),
            protocol_date: protocol_date.to_string(),
        }
    }
}

fn botocore_generate(input: &str, type_name: &str, destination: &Path) -> Result<(), String> {
    let mut command = Command::new("codegen/botocore_parser.py");

    command.args(&[input, type_name]);

    let output = try!(command.output().or(Err("Couldn't get output of child process".to_owned())));

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr[..]));
        return Err("child process was unsuccessful".to_owned());
    }

    let mut file = try!(
        File::create(destination).or(Err("Couldn't open file for writing".to_owned()))
    );

    try!(
        copy(&mut &output.stdout[..], &mut file).or(
            Err("Failed to write generated code to file".to_owned())
        )
    );

    Ok(())
}

fn generate(service: &Service, botocore_path: &Path, base_destination: &Path) -> Result<(), String> {
    let botocore_destination = base_destination.join(format!("{}_botocore.rs", service.name));
    let serde_destination = base_destination.join(format!("{}.rs", service.name));
    let input_location = botocore_path.join(
        format!("{}/{}/service-2.json", service.name, service.protocol_date)
    );

    let input = try!(
        input_location.to_str().ok_or(
            format!(
                "Invalid service definition path for {} {}",
                service.protocol_date,
                service.name,
            )
        )
    );

    try!(botocore_generate(input, &service.type_name, botocore_destination.as_path()));
    serde_generate(botocore_destination.as_path(), serde_destination.as_path())
}

fn serde_generate(source: &Path, destination: &Path) -> Result<(), String> {
    let mut registry = Registry::new();

    register(&mut registry);

    try!(
        registry.expand("", source, destination).or(
            Err("Failed to generate code with Serde".to_owned())
        )
    );

    Ok(())
}
