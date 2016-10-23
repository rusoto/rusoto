# Rusoto

[![Build Status](https://travis-ci.org/rusoto/rusoto.svg?branch=master)](https://travis-ci.org/rusoto/rusoto)

[![Build status](https://ci.appveyor.com/api/projects/status/o83ruaeu7xft0ru5/branch/master?svg=true)](https://ci.appveyor.com/project/matthewkmayer/rusoto/branch/master)

AWS SDK for Rust. [Documentation](https://rusoto.github.io/rusoto/rusoto/index.html).

IRC: #rusoto on irc.freenode.net.

## Requirements

Rust 1.9.0 or later is required.

On OS X and Windows, you may need to install the openssl runtime and headers to get the `rust-openssl` dependency to build. Instructions for that can be found [here](https://github.com/sfackler/rust-openssl#building).

## Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto).
To use Rusoto in your Rust program built with Cargo, add it as a dependency and enable the Cargo features for any AWS service you want to use.

For example, to include only S3 and SQS:

``` toml
[dependencies]
rusoto = {version = "0.17", features = ["s3", "sqs"]}
```

You can use the Cargo feature "all" to build Rusoto with support for every available service. Warning: building with "all" can require upwards of 5 GB of memory. Most people do not need all 40+ services so use individual features to enable the services you use.

## Usage

Rusoto includes a public module for each AWS service it is compiled for containing Rust types for that service's API.
A full list of these services and their Cargo feature names are included at the end of this document.
All other public types are reexported to the crate root.
Consult the rustdoc documentation for full details by running `cargo doc` or visiting the online [documentation](https://rusoto.github.io/rusoto/rusoto/index.html) for the latest crates.io release.

A simple example of using Rusoto's DynamoDB API to list the names of all tables in a database:

```rust
extern crate rusoto;

use std::default::Default;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::dynamodb::{DynamoDbClient, ListTablesInput};

fn main() {
  let provider = DefaultCredentialsProvider::new().unwrap();
  let client = DynamoDbClient::new(provider, Region::UsEast1);
  let list_tables_input: ListTablesInput = Default::default();

  match client.list_tables(&list_tables_input) {
    Ok(output) => {
      match output.table_names {
        Some(table_name_list) => {
          println!("Tables in database:");

          for table_name in table_name_list {
            println!("{}", table_name);
          }
        }
        None => println!("No tables in database!"),
      }
    }
    Err(error) => {
      println!("Error: {:?}", error);
    }
  }
}
```

Rusoto exposes relatively low level types for AWS's APIs.
It may be convenient to use higher level types, which can be found in the [rusoto_helpers](helpers) crate.

### Credentials

For more information on Rusoto's use of AWS credentials such as priority and refreshing, see [AWS Credentials](AWS-CREDENTIALS.md).

### Debugging

Rusoto uses the [log](https://crates.io/crates/log/) logging facade.
For tests it uses [env_logger](https://crates.io/crates/env_logger/).  In order to see the output,
`env_logger` needs to be initialized.  For example:

```rust
#[test]
fn list_objects_test() {
    let _ = env_logger::init();
    let bare_s3 = S3Client::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);
    let mut list_request = ListObjectsRequest::default();
    list_request.bucket = "rusototester".to_string();
    let result = bare_s3.list_objects(&list_request).unwrap();
    println!("result is {:?}", result);
}
```

To see output of logging from integration tests, run:

`RUST_LOG=info cargo test --features all`

To get logging output as well as `println!()` statements, run:

`RUST_LOG=debug cargo test --features all -- --nocapture`

## Semantic versioning

Rusoto complies with [semantic versioning 2.0.0](http://semver.org/).
Until reaching 1.0.0 the API is to be considered unstable.
See [Cargo.toml](Cargo.toml) or [rusoto on crates.io](https://crates.io/crates/rusoto) for current version.

## Releases

Information on release schedules and procedures are in [RELEASING](RELEASING.md).

## Supported AWS services

Service | Cargo feature | Stability
--------|---------------|----------
All supported services | all | Under development
[Certificate Manager](https://aws.amazon.com/certificate-manager/) | acm | Under development
[CloudHSM](https://aws.amazon.com/cloudhsm/) | cloudhsm | Under development
[CloudTrail](https://aws.amazon.com/cloudtrail/) | cloudtrail | Under development
[CloudWatch Events](http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/WhatIsCloudWatchEvents.html) | events | Under development
[CloudWatch Logs](http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CWL_GettingStarted.html) | logs | Under development
[CodeCommit](https://aws.amazon.com/codecommit/) | codecommit | Under development
[CodeDeploy](https://aws.amazon.com/codedeploy/) | codedeploy | Under development
[CodePipeline](https://aws.amazon.com/codepipeline/) | codepipeline | Under development
[Cognito Identity](http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-identity.html) | cognito-identity | Under development
[Config](https://aws.amazon.com/config/) | config | Under development
[Data Pipeline](https://aws.amazon.com/datapipeline/) | datapipeline | Under development
[Device Farm](https://aws.amazon.com/device-farm/) | devicefarm | Under development
[Direct Connect](https://aws.amazon.com/directconnect/) | directconnect | Under development
[Directory Service](https://aws.amazon.com/directoryservice/) | ds | Under development
[DynamoDB](https://aws.amazon.com/dynamodb/) | dynamodb | Stable
[DynamoDB Streams](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.html) | dynamodbstreams | Under development
[EC2](https://aws.amazon.com/ec2/) | ec2 | Stable
[EC2 Container Registry](https://aws.amazon.com/ecr/) | ecr | Under development
[ECS](https://aws.amazon.com/ecs/) | ecs | Stable
[Elastic MapReduce](https://aws.amazon.com/elasticmapreduce/) | emr | Under development
[Elastic Transcoder](https://aws.amazon.com/elastictranscoder/) | ets | Stable
[IAM](https://aws.amazon.com/iam/) | iam | Under development
[Inspector](https://aws.amazon.com/inspector/) | inspector | Under development
[Key Management Service](https://aws.amazon.com/kms/) | kms | Stable
[Kinesis](https://aws.amazon.com/kinesis/) | kinesis | Under development
[Kinesis Firehose](https://aws.amazon.com/kinesis/firehose/) | firehose | Under development
[Machine Learning](https://aws.amazon.com/machine-learning/) | machinelearning | Under development
[OpsWorks](https://aws.amazon.com/opsworks/) | opsworks | Under development
[Route53 Domains](http://docs.aws.amazon.com/Route53/latest/APIReference/actions-on-domain-registrations.html) | route53domains | Under development
[S3](https://aws.amazon.com/s3/) | s3 | Stable
[Simple Systems Manager](http://docs.aws.amazon.com/ssm/latest/APIReference/Welcome.html) | ssm | Under development
[Simple Workflow Service](https://aws.amazon.com/swf/) | swf | Under development
[SQS](https://aws.amazon.com/sqs/) | sqs | Stable
[Storage Gateway](https://aws.amazon.com/storagegateway/) | storagegateway | Under development
[Web Application Firewall](https://aws.amazon.com/waf/) | waf | Under development
[WorkSpaces](https://aws.amazon.com/workspaces/) | workspaces | Under development

## Contributing

See [CONTRIBUTING](CONTRIBUTING.md).
