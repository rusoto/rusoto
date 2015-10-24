# Rusoto
[![Build Status](https://ci.dualspark.com/api/badge/github.com/DualSpark/rusoto/status.svg?branch=master)](https://ci.dualspark.com/github.com/DualSpark/rusoto)

AWS client libraries for Rust.  Work in progress.

#### Documentation

Docs are available at [http://dualspark.github.io/rusoto/](http://dualspark.github.io/rusoto/).

### Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto).

### Use

More example code in [src/bin/main.rs](src/bin/main.rs).

```rust
let provider = DefaultAWSCredentialsProviderChain::new();
let region = Region::UsEast1;

let mut sqs = SQSHelper::new(provider, &region);

let response = try!(sqs.list_queues());
for q in response.queue_urls {
    println!("Existing queue url: {}", q);
}
```

For more information on Rusoto's use of AWS credentials such as priority and refreshing, see [AWS Credentials](AWS-CREDENTIALS.md).

### Semantic versioning

Rusoto complies with [semantic versioning 2.0.0](http://semver.org/).  Until reaching 1.0.0 the API is to be considered unstable.  See [Cargo.toml](Cargo.toml) or [rusoto on crates.io](https://crates.io/crates/rusoto) for current version.  

Information about how releases are made is in [RELEASING](RELEASING.md).

#### Currently implemented

1. **SQS**: See available functions in [sqs.rs](src/sqs.rs).
2. **S3**: See available functions in [s3.rs](src/s3.rs).

### Contributing

1. Install Rust 1.1.0 or later - http://www.rust-lang.org/
2. Check out code from github
3. Set up AWS credentials: environment variables (export AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY), ~/.aws/credentials, or use an IAM instance profile.
4. `cargo build`
5. `cargo run` - This will create real AWS resources and you may be charged.

#### Rust code generation from boto core service definitions:

```bash
./botocore_parser path/to/some.json ClientClassName > some_module.rs
```
