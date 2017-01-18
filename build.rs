extern crate rustc_version;
extern crate rusoto_codegen;

use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::thread;

use rusoto_codegen::{Service, generate};

/// Parses and generates variables used to construct a User-Agent.
///
/// This is used to create a User-Agent header string resembling
/// `rusoto/x.y.z rust/x.y.z <os>`.
fn generate_user_agent_vars(output_path: &Path) {
    let rust_version = rustc_version::version();
    let mut f = File::create(&output_path.join("user_agent_vars.rs"))
            .expect("Could not create user agent file");
    f.write_all(format!("static RUST_VERSION: &'static str = \"{}\";", rust_version).as_bytes())
            .expect("Unable to write user agent");
}

/*
gamelift/2015-10-01/service-2.json:    "protocol":"json"
support/2013-04-15/service-2.json:    "protocol":"json"
*/

// expand to use cfg!() so codegen only gets run for services
// in the features list
macro_rules! services {
    ( $( [$name:expr, $date:expr] ),* ) => {
        {
            let mut services = Vec::new();
            $(
                if cfg!(feature = $name) {
                    services.push(Service::new($name, $date));
                }
            )*
            services
        }
    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    let services = services! {
        ["acm", "2015-12-08"],
        ["cloudformation", "2010-05-15"],
        ["cloudfront", "2016-11-25"],
        ["cloudhsm", "2014-05-30"],
        ["cloudtrail", "2013-11-01"],
        ["cloudwatch", "2010-08-01"],
        ["codecommit", "2015-04-13"],
        ["codedeploy", "2014-10-06"],
        ["codepipeline", "2015-07-09"],
        ["cognito-identity", "2014-06-30"],
        ["config", "2014-11-12"],
        ["datapipeline", "2012-10-29"],
        ["devicefarm", "2015-06-23"],
        ["directconnect", "2012-10-25"],
        ["ds", "2015-04-16"],
        ["dynamodb", "2012-08-10"],
        ["dynamodbstreams", "2012-08-10"],
        ["ec2", "2015-10-01"],
        ["ecr", "2015-09-21"],
        ["ecs", "2014-11-13"],
        ["elastictranscoder", "2012-09-25"],
        ["emr", "2009-03-31"],
        ["events", "2015-10-07"],
        ["firehose", "2015-08-04"],
        ["iam", "2010-05-08"],
        ["inspector", "2016-02-16"],
        ["iot", "2015-05-28"],
        ["kinesis", "2013-12-02"],
        ["kms", "2014-11-01"],
        ["logs", "2014-03-28"],
        ["machinelearning", "2014-12-12"],
        ["marketplacecommerceanalytics", "2015-07-01"],
        ["opsworks", "2013-02-18"],
        ["route53", "2013-04-01"],
        ["route53domains", "2014-05-15"],
        ["s3", "2006-03-01"],
        ["sqs", "2012-11-05"],
        ["ssm", "2014-11-06"],
        ["storagegateway", "2013-06-30"],
        ["swf", "2012-01-25"],
        ["waf", "2015-08-24"],
        ["workspaces", "2015-04-08"]
    };

    let mut generate_threads = vec![];
    for service in services {
        let thread_outpath = out_path.clone();
        generate_threads.push(thread::spawn(move || {
            generate(service, &thread_outpath);
        }));
    }

    for child_thread in generate_threads {
        match child_thread.join() {
            Ok(_) => {},
            Err(err) => panic!(err)
        }
    }

    generate_user_agent_vars(&out_path);

    let codegen_dir = Path::new("codegen");

    // avoid unnecessary recompiles when used as a crates.io dependency
    if codegen_dir.exists() {
        println!("cargo:rerun-if-changed=codegen");
    }
}
