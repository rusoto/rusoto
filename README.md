# Rusoto

<table>
    <tr>
        <td><strong>Linux / OS X</strong></td>
        <td><a href="https://travis-ci.org/rusoto/rusoto" title="Travis Build Status"><img src="https://travis-ci.org/rusoto/rusoto.svg?branch=master" alt="travis-badge"></img></a></td>
    </tr>
    <tr>
        <td><strong>Windows</strong></td>
        <td><a href="https://ci.appveyor.com/project/matthewkmayer/rusoto/branch/master" title="Appveyor Build Status"><img src="https://ci.appveyor.com/api/projects/status/o83ruaeu7xft0ru5/branch/master?svg=true" alt="appveyor-badge"></img></a></td>
    </tr>
    <tr>
        <td colspan="2">
            <a href="https://rusoto.github.io/rusoto/rusoto/" title="API Docs"><img src="https://img.shields.io/badge/API-docs-blue.svg" alt="api-docs-badge"></img></a>
            <a href="https://crates.io/crates/rusoto" title="Crates.io"><img src="https://img.shields.io/crates/v/rusoto.svg" alt="crates-io"></img></a>
            <a href="#license" title="License: MIT"><img src="https://img.shields.io/crates/l/rusoto.svg" alt="license-badge"></img></a>
        </td>
    </tr>
</table>

AWS SDK for Rust. [Documentation](https://rusoto.github.io/rusoto/rusoto/index.html).

IRC: #rusoto on irc.freenode.net.

---

You may be looking for:

* [An overview of Rusoto][rusoto-overview]
* [AWS services supported by Rusoto][supported-aws-services]

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
A full list of these services and their Cargo feature names can be found [here][supported-aws-services].
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

## Contributing

See [CONTRIBUTING](CONTRIBUTING.md).

## License

Rusoto is distributed under the terms of the MIT license.

See [LICENSE][license] for details.

[license]: https://github.com/rusoto/rusoto/blob/master/LICENSE "MIT License"
[rusoto-overview]: https://rusoto.github.io/ "Rusoto overview"
[supported-aws-services]: https://rusoto.github.io/supported-aws-services.html "List of AWS services supported by Rusoto"
