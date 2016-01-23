extern crate serde_codegen;
extern crate syntex;

use std::env::var_os;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

use serde_codegen::register;
use syntex::Registry;

fn main() {
    let out_dir = var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir);
    let src_path = Path::new("src");

    // DynamoDB
    generate(
        "codegen/botocore/botocore/data/dynamodb/2012-08-10/service-2.json",
        "DynamoDBClient",
        out_path,
        "dynamodb",
    );
    serde_generate(
        &src_path.join("kms_helpers.in.rs"),
        &out_path.join("kms_helpers.rs"),
    );

    // ECS
    generate(
        "codegen/botocore/botocore/data/ecs/2014-11-13/service-2.json",
        "ECSClient",
        out_path,
        "ecs",
    );
    serde_generate(
        &src_path.join("ecs_helpers.in.rs"),
        &out_path.join("ecs_helpers.rs"),
    );

    // KMS
    generate(
        "codegen/botocore/botocore/data/kms/2014-11-01/service-2.json",
        "KMSClient",
        out_path,
        "kms",
    );
    serde_generate(
        &src_path.join("dynamodb_helpers.in.rs"),
        &out_path.join("dynamodb_helpers.rs"),
    );

    // SQS
    generate(
        "codegen/botocore/botocore/data/sqs/2012-11-05/service-2.json",
        "SQSClient",
        out_path,
        "sqs",
    )
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
    input: &str,
    type_name: &str,
    base_destination: &Path,
    service_name: &str,
) {
    let botocore_destination = base_destination.join(format!("{}_botocore.rs", service_name));
    let serde_destination = base_destination.join(format!("{}.rs", service_name));

    botocore_generate(input, type_name, botocore_destination.as_path());
    serde_generate(botocore_destination.as_path(), serde_destination.as_path());
}

fn serde_generate(source: &Path, destination: &Path) {
    let mut registry = Registry::new();

    register(&mut registry);
    registry.expand("", source, destination).expect("failed to generate code with Serde");
}
