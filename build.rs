extern crate rusoto_codegen;

use std::env;
use std::path::Path;

use rusoto_codegen::{Service, generate};

/*
gamelift/2015-10-01/service-2.json:    "protocol":"json"
support/2013-04-15/service-2.json:    "protocol":"json"

waf/2015-08-24/service-2.json:    "protocol":"json",
workspaces/2015-04-08/service-2.json:    "protocol":"json"*/

// expand to use cfg!() and prevent the codegen for every service
// being run every build
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
    let out_path = Path::new(&out_dir);

    let services = services! {
        ["acm", "2015-12-08"],
        ["cloudhsm", "2014-05-30"],
        ["cloudtrail", "2013-11-01"],
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
        ["events", "2014-02-03"],
        ["firehose", "2015-08-04"],
        ["inspector", "2016-02-16"],
        ["kinesis", "2013-12-02"],
        ["kms", "2014-11-01"],
        ["logs", "2014-03-28"],
        ["machinelearning", "2014-12-12"],
        ["marketplacecommerceanalytics", "2015-07-01"],
        ["opsworks", "2013-02-18"],
        ["route53domains", "2014-05-15"],
        ["sqs", "2012-11-05"],
        ["ssm", "2014-11-06"],
        ["storagegateway", "2013-06-30"],
        ["swf", "2012-01-25"]
    };

    for service in services {
        generate(service, out_path);
    }


    println!("cargo:rerun-if-changed=codegen");
}
