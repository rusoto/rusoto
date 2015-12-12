# Rusoto

[![Build Status](https://travis-ci.org/DualSpark/rusoto.svg?branch=master)](https://travis-ci.org/DualSpark/rusoto)

AWS SDK for Rust.

#### Documentation

Docs are available at [http://dualspark.github.io/rusoto/](http://dualspark.github.io/rusoto/).

### Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto).

### Use

Examples are available in [tests](tests) directory.

[SQS example](tests/sqs.rs):

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

### Releases

Information on release schedules and procedures are in [RELEASING](RELEASING.md).

#### Currently implemented

1. **SQS**: See available functions in [sqs.rs](src/sqs.rs) and uses in [SQS integration tests](tests/sqs.rs).
2. **S3**: See available functions in [s3.rs](src/s3.rs) and uses in [S3 integration tests](tests/s3.rs).
2. **DynamoDB**: See available functions in [dynamodb.rs](src/dynamodb.rs) and uses in [DynamoDB integration tests](tests/dynamodb.rs).

### Contributing

1. Install Rust **1.3.0** or later - http://www.rust-lang.org/
2. Check out code from github
3. Set up AWS credentials: environment variables (export AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY), ~/.aws/credentials, or use an IAM instance profile.
4. `cargo build`
5. `cargo test --verbose --features aws_integration` - This will create real AWS resources and you may be charged.

#### Rust code generation from boto core service definitions:

See [CODEGEN](codegen/CODEGEN.md).
