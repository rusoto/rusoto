extern crate rusoto_codegen;

use std::env;
use std::fs::copy;
use std::path::Path;

use rusoto_codegen::{AmazonService, generate};

const BOTOCORE_DIR: &'static str = "codegen/botocore/botocore/data/";

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir);

    let botocore_dir = env::var_os("BOTOCORE_DIR");
    let botocore_path = match botocore_dir {
        Some(ref dirname) => Path::new(dirname),
        None => Path::new(BOTOCORE_DIR)
    };

    let services = vec![
        AmazonService::new("dynamodb", "DynamoDBClient", "2012-08-10"),
        AmazonService::new("kms", "KMSClient", "2014-11-01"),
        AmazonService::new("ecs", "ECSClient", "2014-11-13"),
        AmazonService::new("sqs", "SQSClient", "2012-11-05"),
    ];

    for service in services {
        generate(service, botocore_path, out_path);
    }

    // Punting on S3 codegen by copying existing hand crafted code:
    let s3_out = out_path.join("s3.rs");
    copy("src/s3.rs.old", s3_out).expect("Couldn't copy existing s3 to s3.rs");
}
