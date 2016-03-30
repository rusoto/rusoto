# Rusoto

[![Build Status](https://travis-ci.org/rusoto/rusoto.svg?branch=master)](https://travis-ci.org/rusoto/rusoto)

AWS SDK for Rust. [Documentation](https://rusoto.github.io/rusoto/rusoto/index.html).

IRC: #rusoto on irc.freenode.net.

## Requirements

Rust 1.7.0 or later is required.

## Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto).
To use Rusoto in your Rust program built with Cargo, add it as a dependency and enable the Cargo features for any AWS service you want to use.

For example:

``` toml
[dependencies.rusoto]
features = ["dynamodb", "s3"]
version = "x.y.z"
```

You can use the Cargo feature "all" to build Rusoto with support for every available service.

## Usage

Rusoto includes a public module for each AWS service it is compiled for containing Rust types for that service's API.
A full list of these services and their Cargo feature names are included at the end of this document.
All other public types are reexported to the crate root.
Consult the rustdoc documenation for full details by running `cargo doc` or visiting the online [documentation](https://rusoto.github.io/rusoto/rusoto/index.html) for the latest crates.io release.

A simple example of using Rusoto's DynamoDB API to list the names of all tables in a database:

```rust
extern crate rusoto;

use std::default::Default;

use rusoto::{ChainProvider, Region}
use rusoto::dynamodb::{DynamoDBClient, ListTablesInput};

fn main() {
  let provider = ChainProvider::new().unwrap();
  let region = Region::UsEast1;
  let mut client = DynamoDBClient::new(provider, &region);
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
It may be convenient to use higher level types, which can be found in the [rusoto_helpers](https://github.com/rusoto/rusoto_helpers) crate.

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
[DynamoDB](https://aws.amazon.com/dynamodb/) | dynamodb
[ECS](https://aws.amazon.com/ecs/) | ecs
[KMS](https://aws.amazon.com/kms/) | kms
[S3](https://aws.amazon.com/s3/) | s3
[SQS](https://aws.amazon.com/sqs/) | sqs

## Contributing

See [CONTRIBUTING](CONTRIBUTING.md).
