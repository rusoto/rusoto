# Rusoto Core

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

Rusoto Core contains core functionality shared across AWS services, such as region definitions, credentials and common request-sending and parsing logic.

---

You may be looking for:

* [An overview of Rusoto][rusoto-overview]
* [AWS services supported by Rusoto][supported-aws-services]
* [API documentation][api-documentation]
* [Getting help with Rusoto][rusoto-help]

## Requirements

Rust 1.15.0 or later is required.

On Linux, OpenSSL is required.

## Installation

Rusoto Core is available on [crates.io](https://crates.io/crates/rusoto).
To use Rusoto in your Rust program built with Cargo, add it as a dependency in your Cargo.toml:

``` toml
[dependencies]
rusoto_core = "0.24.0"
```

## Usage

Rusoto Core should be paired with Rusoto services to able to be able to work with them. Consult the documentation for those services to see how it should be used with them.

Consult the rustdoc documentation for full details by running `cargo doc` or visiting the online [documentation](https://rusoto.github.io/rusoto/rusoto/index.html) for the latest crates.io release.

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

## Supported OSs and Rust versions

Linux, OSX and Windows are supported and tested via TravisCI and Appveyor.

Rust stable is supported.  Older versions of Rust are supported and tested via TravisCI.  The minimum Rust version is
incremented when it becomes inconvenient to support older versions.  The current minimum version of Rust supported can
be found in [.travis.yml](.travis.yml).  If a version number is not specified in the `rust` section, only the named versions
listed are supported.  This should be stable, beta and nightly.

## License

Rusoto is distributed under the terms of the MIT license.

See [LICENSE][license] for details.

[api-documentation]: https://rusoto.github.io/rusoto/rusoto/ "API documentation"
[license]: https://github.com/rusoto/rusoto/blob/master/LICENSE "MIT License"
[rusoto-help]: https://www.rusoto.org/help.html "Getting help with Rusoto"
[rusoto-overview]: https://www.rusoto.org/ "Rusoto overview"
[supported-aws-services]: https://www.rusoto.org/supported-aws-services.html "List of AWS services supported by Rusoto"
