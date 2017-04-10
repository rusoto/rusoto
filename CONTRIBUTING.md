# Contributing to Rusoto

### Setting up build environment (only needed once)

See minimum version of Rust required in [README](README.md).

Check out code from GitHub.

Set up AWS credentials: environment variables (export AWS_ACCESS_KEY_ID and
AWS_SECRET_ACCESS_KEY), populate the ~/.aws/credentials file, or use an
IAM instance profile on an EC2 instance.

Rusoto codegen depends on botocore.  Update the git submodule via:

``` bash
cd rusoto
git submodule init
git submodule update
```

You are now ready to build the project with `cargo build`.
Remember to include the appropriate feature flags for the AWS services you want to use.
See [README](README.md) for a table of available services and their Cargo feature names.

### Building after initial setup

Build the project with `cargo build`.

Integration tests can be executed by running `cargo test --features FEATURE`, where FEATURE is one or more space-separated Cargo features to test as defined in `Cargo.toml`.
Each AWS service has a Cargo feature to enable it.
The feature "all" can be used to test all supported services.
The integration tests will create real AWS resources and you may be charged.
To run only the in-crate unit tests, which don't call out to AWS, include the `--lib` option to `cargo test`.

For more verbose test output, you can run `cargo test --verbose --features FEATURE -- --nocapture`.

**Warning**: When building or testing with `--features all` the build/test can require upwards of 5 GB of memory. You can limit the number of features to build at once to prevent running out of memory. That can be achieved using the `--features` flag, e.g. `cargo build --features packagea && cargo build --features packageb`. See [README](README.md) for a table of available services and their Cargo feature names. 

### Rust code generation from boto core service definitions:

See [Cargo.toml](codegen/Cargo.toml) and [build.rs](codegen/build.rs) in the
rusoto_codegen subcrate.

## Clippy

Instructions on [clippy's homepage](https://github.com/Manishearth/rust-clippy) have details on how to install and run.
A nightly version of Rust is required.  To get the latest nightly version of Rust and switch to it, you can run:

`rustup update && rustup default nightly`

To run clippy against the checked-in code, assuming nightly Rust is used:

`cargo build --no-default-features --features nightly-testing`

To run clippy against the generated code as well as the checked-in code, assuming nightly Rust is used:

`rustup run nightly cargo build --features "nightly-testing all"`
