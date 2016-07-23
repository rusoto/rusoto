# Contributing to Rusoto

### Setting up build environment (only needed once)

Install Rust **1.8.0** or later - http://www.rust-lang.org/

Check out code from github.

If you're on OSX, you'll probably need a new version of openssl.  Run `brew install openssl`.

Do one of the following to make rust-openssl see the Homebrew-installed OpenSSL:

1.  Run `brew link --force openssl`.
2.  Run:

    ``` bash
    export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
    export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
    ```

Set up AWS credentials: environment variables (export AWS_ACCESS_KEY_ID and
AWS_SECRET_ACCESS_KEY), populate the ~/.aws/credentials file, or use an
IAM instance profile on an EC2 instance.

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

### Rust code generation from boto core service definitions:

See [Cargo.toml](codegen/Cargo.toml) and [build.rs](codegen/build.rs) in the
rusoto_codegen subcrate.
