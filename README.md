# ![Rusoto](./assets/logo-wide.png)

<table>
    <tr>
        <td><strong>Linux / OS X stable</strong></td>
        <td><a href="https://dev.azure.com/matthewkmayer/Rusoto/_build?definitionId=1" title="Pipelines Build Status"><img src="https://dev.azure.com/matthewkmayer/Rusoto/_apis/build/status/rusoto.rusoto?branchName=master" alt="pipelines-badge"></img></a></td>
    </tr>
    <tr>
        <td><strong>Linux / OS X beta and nightly</strong></td>
        <td><a href="https://travis-ci.org/rusoto/rusoto" title="Travis Build Status"><img src="https://travis-ci.org/rusoto/rusoto.svg?branch=master" alt="travis-badge"></img></a></td>
    </tr>
    <tr>
        <td><strong>Windows</strong></td>
        <td><a href="https://ci.appveyor.com/project/matthewkmayer/rusoto/branch/master" title="Appveyor Build Status"><img src="https://ci.appveyor.com/api/projects/status/o83ruaeu7xft0ru5/branch/master?svg=true" alt="appveyor-badge"></img></a></td>
    </tr>
    <tr>
        <td><strong>Ceph and Minio support</strong></td>
        <td><a href='https://semaphoreci.com/matthewkmayer/rusoto'> <img src='https://semaphoreci.com/api/v1/matthewkmayer/rusoto/branches/master/badge.svg' alt='Build Status'></a></td>
    </tr>
    <tr>
        <td colspan="2">
            <a href="https://rusoto.github.io/rusoto/" title="API Docs"><img src="https://img.shields.io/badge/API-docs-blue.svg" alt="api-docs-badge"></img></a>
            <a href="https://crates.io/crates/rusoto_core" title="Crates.io"><img src="https://img.shields.io/crates/v/rusoto_core.svg" alt="crates-io"></img></a>
            <a href="#license" title="License: MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="license-badge"></img></a>
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

## Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto_core).
To use Rusoto in your Rust program built with Cargo, add it as a dependency and `rusoto_$SERVICENAME` for any supported AWS service you want to use.

For example, to include only S3 and SQS:

```toml
[dependencies]
rusoto_core = "0.37.0"
rusoto_sqs = "0.37.0"
rusoto_s3 = "0.37.0"
```

## Migration notes

Breaking changes and migration details are documented at [https://rusoto.org/migrations.html](https://rusoto.org/migrations.html).

## Usage

Rusoto has a crate for each AWS service, containing Rust types for that service's API.
A full list of these services can be found [here][supported-aws-services].
All other public types are reexported to the crate root.
Consult the rustdoc documentation for full details by running `cargo doc` or visiting the online [documentation](https://rusoto.github.io/rusoto/rusoto/index.html) for the latest crates.io release.

A simple example of using Rusoto's DynamoDB API to list the names of all tables in a database:

```rust,no_run
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

fn main() {
    let client = DynamoDbClient::new(Region::UsEast1);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).sync() {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");

                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                },
                None => println!("No tables in database!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}
```

### Credentials

For more information on Rusoto's use of AWS credentials such as priority and refreshing, see [AWS Credentials](AWS-CREDENTIALS.md).

## Semantic versioning

Rusoto complies with [semantic versioning 2.0.0](http://semver.org/).
Until reaching 1.0.0 the API is to be considered unstable.
See [Cargo.toml](Cargo.toml) or [rusoto on crates.io](https://crates.io/crates/rusoto_core) for current version.

## Releases

Information on release schedules and procedures are in [RELEASING](RELEASING.md).

## Contributing

See [CONTRIBUTING](CONTRIBUTING.md).

## Supported OSs and Rust versions

Linux, OSX and Windows are supported and tested via TravisCI and Appveyor.

Rust stable is supported.  Older versions of Rust are supported and tested via TravisCI.  The minimum Rust version is incremented when it becomes inconvenient to support older versions.  The current minimum version of Rust supported can be found in [.travis.yml](.travis.yml).  If a version number is not specified in the `rust` section, only the named versions listed are supported.  This should be stable, beta and nightly.

## License

Rusoto is distributed under the terms of the MIT license.

See [LICENSE][license] for details.

[api-documentation]: https://rusoto.github.io/rusoto/rusoto/ "API documentation"
[license]: https://github.com/rusoto/rusoto/blob/master/LICENSE "MIT License"
[rusoto-help]: https://www.rusoto.org/help.html "Getting help with Rusoto"
[rusoto-overview]: https://www.rusoto.org/ "Rusoto overview"
[supported-aws-services]: https://www.rusoto.org/supported-aws-services.html "List of AWS services supported by Rusoto"
