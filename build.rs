extern crate rusoto_codegen;

use std::env;
use std::path::Path;

use rusoto_codegen::{Service, generate};

/*
codedeploy/2014-10-06/service-2.json:    "protocol":"json",
codepipeline/2015-07-09/service-2.json:    "protocol":"json"
cognito-identity/2014-06-30/service-2.json:    "protocol":"json"
config/2014-11-12/service-2.json:    "protocol":"json",
datapipeline/2012-10-29/service-2.json:    "protocol":"json"
devicefarm/2015-06-23/service-2.json:    "protocol":"json",
directconnect/2012-10-25/service-2.json:    "protocol":"json",
ds/2015-04-16/service-2.json:    "protocol":"json",
dynamodb/2012-08-10/service-2.json:    "protocol":"json",
dynamodbstreams/2012-08-10/service-2.json:    "protocol":"json"
ecr/2015-09-21/service-2.json:    "protocol":"json",
ecs/2014-11-13/service-2.json:    "protocol":"json",
emr/2009-03-31/service-2.json:    "protocol":"json",
events/2014-02-03/service-2.json:    "protocol":"json"
events/2015-10-07/service-2.json:    "protocol":"json"
firehose/2015-08-04/service-2.json:    "protocol":"json"
gamelift/2015-10-01/service-2.json:    "protocol":"json"
inspector/2015-08-18/service-2.json:    "protocol":"json",
kinesis/2013-12-02/service-2.json:    "protocol":"json"
kms/2014-11-01/service-2.json:    "protocol":"json"
logs/2014-03-28/service-2.json:    "protocol":"json",
machinelearning/2014-12-12/service-2.json:    "protocol":"json"
marketplacecommerceanalytics/2015-07-01/service-2.json:    "protocol":"json",
opsworks/2013-02-18/service-2.json:    "protocol":"json",
route53domains/2014-05-15/service-2.json:    "protocol":"json"
ssm/2014-11-06/service-2.json:    "protocol":"json",
storagegateway/2013-06-30/service-2.json:    "protocol":"json",
support/2013-04-15/service-2.json:    "protocol":"json"
swf/2012-01-25/service-2.json:    "protocol":"json"
waf/2015-08-24/service-2.json:    "protocol":"json",
workspaces/2015-04-08/service-2.json:    "protocol":"json"*/

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir);

    let services = vec![
        Service::new("acm", "2015-12-08"),
        Service::new("cloudhsm","2014-05-30"),
        Service::new("cloudtrail","2013-11-01"),
        Service::new("codecommit", "2015-04-13"),
        Service::new("dynamodb", "2012-08-10"),
        Service::new("firehose", "2015-08-04"),
        Service::new("kinesis", "2013-12-02"),
        Service::new("kms", "2014-11-01"),
        Service::new("ec2", "2015-10-01"),
        Service::new("ecs", "2014-11-13"),
        Service::new("elastictranscoder", "2012-09-25"),
        Service::new("sqs", "2012-11-05"),
    ];

    for service in services {
        generate(service, out_path);
    }

    println!("cargo:rerun-if-changed=codegen");
}
