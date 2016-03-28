# Contributing to Rusoto

### Setting up build environment (only needed once)

Install Rust **1.7.0** or later - http://www.rust-lang.org/

Check out code from github.

If you're on OSX, you'll probably need a new version of openssl.  Run `brew install openssl`.

If using pre-El Capitan OSX, run `brew link --force openssl`.

For El Capitan, these environment variables need to be set whenever building the openssl crate.
This includes rebuilding Rusoto after a `cargo clean`.
They can be added to your shell profile (EG `~/.bash_profile`):

```bash
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
```

Set up AWS credentials: environment variables (export AWS_ACCESS_KEY_ID and
AWS_SECRET_ACCESS_KEY), populate the ~/.aws/credentials file, or use an
IAM instance profile on an EC2 instance.

Initialize and fetch the git submodule for botocore definitions:

`git submodule init`

`git submodule update`

Now run `cargo build` to perform the initial build.

Initial setup is now complete, the above shouldn't be needed again unless you need to update the botocore definitions from that upstream project.

### Building after initial setup

Build the project with `cargo build`.

Integration tests can be executed by running `cargo test --features FEATURE`, where FEATURE is one or more space-separated Cargo features to test as defined in `Cargo.toml`.
Each AWS service has a Cargo feature to enable it.
The feature "all" can be used to test all supported services.
This will create real AWS resources and you may be charged.

For more verbose test output, you can run `cargo test --verbose --features FEATURE -- --nocapture`.

### Rust code generation from boto core service definitions:

See [Cargo.toml](codegen/Cargo.toml) and [build.rs](codegen/build.rs) in the
rusoto_codegen subcrate.
