# Rusoto

[![Build Status](https://travis-ci.org/rusoto/rusoto.svg?branch=master)](https://travis-ci.org/rusoto/rusoto)

AWS SDK for Rust.  [Documentation](http://rusoto.github.io/rusoto/rusoto/index.html).

IRC: #rusoto on irc.freenode.net.

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

#### Credentials

For more information on Rusoto's use of AWS credentials such as priority and refreshing, see [AWS Credentials](AWS-CREDENTIALS.md).

#### Debugging

Rusoto uses the [log](https://crates.io/crates/log/) logging facade.  For tests it uses [env_logger](https://crates.io/crates/env_logger/).
To see output of logging from integration tests, run:

`RUST_LOG=info cargo test --features aws_integration`

### Semantic versioning

Rusoto complies with [semantic versioning 2.0.0](http://semver.org/).  Until reaching 1.0.0 the API is to be considered unstable.  See [Cargo.toml](Cargo.toml) or [rusoto on crates.io](https://crates.io/crates/rusoto) for current version.  

### Releases

Information on release schedules and procedures are in [RELEASING](RELEASING.md).

#### Currently implemented

* DynamoDB
* KMS
* S3
* SQS

### Contributing

See [CONTRIBUTING](CONTRIBUTING.md).
