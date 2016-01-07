extern crate serde_codegen;
extern crate syntex;

use std::env::var_os;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

use serde_codegen::register;
use syntex::Registry;

const BOTOCORE_DIR: &'static str = "codegen/botocore/botocore/data/";

struct Service {
    name: String,
    type_name: String, 
    protocol_date: String
}

fn main() {
    let out_dir = var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir);

    let botocore_dir = var_os("BOTOCORE_DIR");
    let botocore_path = match botocore_dir {
        Some(ref dirname) => Path::new(dirname),
        None => Path::new(BOTOCORE_DIR)
    };
    
    let services = vec![
        Service::new("dynamodb", "DynamoDBClient", "2012-08-10"),
        Service::new("kms", "KMSClient", "2014-11-01"),
        Service::new("sqs", "SQSClient", "2012-11-05")
    ];

    for service in services {        
        generate(service, botocore_path, out_path);
    }
}

fn botocore_generate(input: &str, type_name: &str, destination: &Path) {
    let mut command = Command::new("codegen/botocore_parser.py");

    command.args(&[input, type_name]);

    let output = command.output().expect("couldn't get output of child process");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr[..]));
        panic!("child process was unsuccessful");
    }

    let mut file = File::create(destination).expect("couldn't open file for writing");
    copy(&mut &output.stdout[..], &mut file).expect("failed to write generated code to file");
}

fn generate(
    service: Service,
    botocore_path: &Path,
    base_destination: &Path,
) {
    let botocore_destination = base_destination.join(format!("{}_botocore.rs", service.name));
    let serde_destination = base_destination.join(format!("{}.rs", service.name));
    let input_location = botocore_path.join(format!("{}/{}/service-2.json", service.name, service.protocol_date));

    let input = input_location.to_str().expect(&format!("Invalid service definition path for {} {}", service.protocol_date, service.name));

    botocore_generate(input, &service.type_name, botocore_destination.as_path());
    serde_generate(botocore_destination.as_path(), serde_destination.as_path());
}

fn serde_generate(source: &Path, destination: &Path) {
    let mut registry = Registry::new();

    register(&mut registry);
    registry.expand("", source, destination).expect("failed to generate code with Serde");
}

impl Service {
    fn new<S: ToString>(name: S, type_name: S, protocol_date: S) -> Service {
        Service { name: name.to_string(), type_name: type_name.to_string(), protocol_date: protocol_date.to_string() }
    }
}
