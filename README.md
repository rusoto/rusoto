# Rusoto

[![Build Status](https://travis-ci.org/rusoto/rusoto.svg?branch=master)](https://travis-ci.org/rusoto/rusoto)

AWS SDK for Rust. [Documentation](https://rusoto.github.io/rusoto/rusoto/index.html).

IRC: #rusoto on irc.freenode.net.

## Requirements

Rust 1.8.0 or later is required.

On OS X and Windows, you may need to install the openssl runtime and headers to get the `rust-openssl` dependency to build. Instructions for that can be found [here](https://github.com/sfackler/rust-openssl#building).

## Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto).
To use Rusoto in your Rust program built with Cargo, add it as a dependency and enable the Cargo features for any AWS service you want to use.

For example, to include only S3 and SQS:

``` toml
[dependencies]
rusoto = {version = "0.15.2", features = ["s3", "sqs"]}
```

You can use the Cargo feature "all" to build Rusoto with support for every available service.

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
use rusoto::dynamodb::{DynamoDBClient, ListTablesInput};

fn main() {
  let provider = DefaultCredentialsProvider::new().unwrap();
  let mut client = DynamoDBClient::new(provider, Region::UsEast1);
  let list_tables_input: ListTablesInput = Default::default();

  match client.list_tables(&list_tables_input) {
    Ok(output) => {
      match output.TableNames {
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
For tests it uses [env_logger](https://crates.io/crates/env_logger/).
To see output of logging from integration tests, run:

`RUST_LOG=info cargo test --features all`

## Semantic versioning

Rusoto complies with [semantic versioning 2.0.0](http://semver.org/).
Until reaching 1.0.0 the API is to be considered unstable.
See [Cargo.toml](Cargo.toml) or [rusoto on crates.io](https://crates.io/crates/rusoto) for current version.

## Releases

Information on release schedules and procedures are in [RELEASING](RELEASING.md).

## Supported AWS services

Service | Cargo feature
--------|--------------
All supported services | all
[Certificate Manager](https://aws.amazon.com/certificate-manager/) | acm
[CloudHSM](https://aws.amazon.com/cloudhsm/) | cloudhsm
[CloudTrail](https://aws.amazon.com/cloudtrail/) | cloudtrail
[CloudWatch Events](http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/WhatIsCloudWatchEvents.html) | events
[CloudWatch Logs](http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CWL_GettingStarted.html) | logs
[CodeCommit](https://aws.amazon.com/codecommit/) | codecommit
[CodeDeploy](https://aws.amazon.com/codedeploy/) | codedeploy
[CodePipeline](https://aws.amazon.com/codepipeline/) | codepipeline
[Cognito Identity](http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-identity.html) | cognito-identity
[Config](https://aws.amazon.com/config/) | config
[Data Pipeline](https://aws.amazon.com/datapipeline/) | datapipeline
[Device Farm](https://aws.amazon.com/device-farm/) | devicefarm
[Direct Connect](https://aws.amazon.com/directconnect/) | directconnect
[Directory Service](https://aws.amazon.com/directoryservice/) | ds
[DynamoDB](https://aws.amazon.com/dynamodb/) | dynamodb
[DynamoDB Streams](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.html)) | dynamodbstreams
[EC2](https://aws.amazon.com/ec2/) | ec2
[EC2 Container Registry](https://aws.amazon.com/ecr/) | ecr
[ECS](https://aws.amazon.com/ecs/) | ecs
[Elastic MapReduce](https://aws.amazon.com/elasticmapreduce/) | emr
[Elastic Transcoder](https://aws.amazon.com/elastictranscoder/) | ets
[Inspector](https://aws.amazon.com/inspector/) | inspector
[Key Management Service](https://aws.amazon.com/kms/) | kms
[Kinesis](https://aws.amazon.com/kinesis/) | kinesis
[Kinesis Firehose](https://aws.amazon.com/kinesis/firehose/) | firehose
[Machine Learning](https://aws.amazon.com/machine-learning/) | machinelearning
[OpsWorks](https://aws.amazon.com/opsworks/) | opsworks
[Route53 Domains](http://docs.aws.amazon.com/Route53/latest/APIReference/actions-on-domain-registrations.html) | route53domains
[S3](https://aws.amazon.com/s3/) | s3
[Simple Systems Manager](http://docs.aws.amazon.com/ssm/latest/APIReference/Welcome.html) | ssm
[Simple Workflow Service](https://aws.amazon.com/swf/) | swf
[SQS](https://aws.amazon.com/sqs/) | sqs
[Storage Gateway](https://aws.amazon.com/storagegateway/) | storagegateway
[Web Application Firewall](https://aws.amazon.com/waf/) | waf
[WorkSpaces](https://aws.amazon.com/workspaces/) | workspaces

## Contributing

See [CONTRIBUTING](CONTRIBUTING.md).
