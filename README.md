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

**Rusoto is an AWS SDK for Rust**

---

You may be looking for:

* [An overview of Rusoto][rusoto-overview]
* [AWS services supported by Rusoto][supported-aws-services]
* [API documentation][api-documentation]
* [Getting help with Rusoto][rusoto-help]

## Requirements

Rust 1.12.0 or later is required.

On OS X and Windows, you may need to install the openssl runtime and headers to get the `rust-openssl` dependency to build. Instructions for that can be found [here](https://github.com/sfackler/rust-openssl#building).

## Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto).
To use Rusoto in your Rust program built with Cargo, add it as a dependency and enable the Cargo features for any AWS service you want to use.

For example, to include only S3 and SQS:

``` toml
[dependencies]
rusoto = {version = "0.20", features = ["s3", "sqs"]}
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

[api-documentation]: https://rusoto.github.io/rusoto/rusoto/ "API documentation"
[license]: https://github.com/rusoto/rusoto/blob/master/LICENSE "MIT License"
[rusoto-help]: https://rusoto.github.io/help.html "Getting help with Rusoto"
[rusoto-overview]: https://rusoto.github.io/ "Rusoto overview"
[supported-aws-services]: https://rusoto.github.io/supported-aws-services.html "List of AWS services supported by Rusoto"
