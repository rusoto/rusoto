# Rusoto
[![Build Status](https://ci.dualspark.com/api/badge/github.com/DualSpark/rust-aws/status.svg?branch=master)](https://ci.dualspark.com/github.com/DualSpark/rust-aws)

AWS client libraries for Rust

### Current state

**Alpha**.  Rust code has been generated from JSON documentation of services from [botocore](https://github.com/boto/botocore).

#### Currently implemented

1. **SQS**: list queues, create queue, get queue url, send message, receive message, delete message, delete queue.
2. **S3**: list buckets, create bucket, delete bucket, upload object, download object, delete object.

### Installation

Rusoto is available on [crates.io](https://crates.io/crates/rusoto).

### Semantic versioning

Rusoto complies with [semantic versioning 2.0.0](http://semver.org/).  Until reaching 1.0.0 the API is to be considered unstable.  See [Cargo.toml](Cargo.toml) or [rusoto on crates.io](https://crates.io/crates/rusoto) for current version.  

Information about how releases are made is in [RELEASING](RELEASING.md).

### Contributing

1. Install Rust 1.1.0 or later - http://www.rust-lang.org/
2. Check out code from github
3. Set up AWS credentials: environment variables (export AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY), ~/.aws/credentials, or use an IAM instance profile.
4. `cargo build`
5. `cargo run` - This will create real AWS resources and you may be charged.

### Credentials

Rusoto will search for credentials in this order:

1. Environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`.
2. AWS Credentials file: `~/.aws/credentials`.  It will use the first `aws_access_key_id` and `aws_secret_access_key` it finds.  Profiles are not yet supported.
3. IAM instance profile.  Rusoto will query the metadata service for an instance profile/role and fetch the access key, secret access key and token to supply those for requests.

If Rusoto exhausts all three options it will panic.

#### Credential refreshing

Credentials obtained from environment variables and credential files expire ten minutes after being acquired, and are refreshed on subsequent calls to `get_credentials()`.

IAM instance profile credentials are refreshed as needed.  Upon calling `get_credentials()` it will see if they are expired or not.  If expired, it attempts to get new credentials from the metadata service.  If that fails it will panic.  IAM credentials expiration time comes from the IAM metadata response.


#### Output from `cargo run` should resemble:

```
Existing queue: https://sqs.us-east-1.amazonaws.com/428250473290/test1
Existing queue: https://sqs.us-east-1.amazonaws.com/428250473290/test2
Created queue test_q_1436921723 with url https://sqs.us-east-1.amazonaws.com/428250473290/test_q_1436921723
Verified queue url https://sqs.us-east-1.amazonaws.com/428250473290/test_q_1436921723 for queue name test_q_1436921723
Send message with body 'lorem ipsum dolor sit amet' and created message_id 9315712d-3e6f-4264-95d4-426fe6a6f69f
Received message 'lorem ipsum dolor sit amet' with id 9315712d-3e6f-4264-95d4-426fe6a6f69f
Message deleted by request_id 2866edd9-d7ee-534b-9b43-a3c66653ef6e
Queue https://sqs.us-east-1.amazonaws.com/428250473290/test_q_1436921723 deleted by request_id b51e12e8-03dc-547c-aa7c-5cf7b261d6e1
Everything worked.
```

#### Example code in [src/bin/main.rs](src/bin/main.rs)

```rust
let sqs = SQSHelper::new(&creds, "us-east-1");

// list existing queues
let response = try!(sqs.list_queues());
for q in response.queue_urls {
  println!("Existing queue: {}", q);
}
```

#### Code generation

```bash
./botocore_parser path/to/some.json ClientClassName > some_module.rs
```

#### Local integration testing of IAM credentials

Edit the `address` location in [src/credentials.rs](src/credentials.rs).  For local testing, I use [moe](https://github.com/matthewkmayer/moe) and set the string to this:

```rust
let mut address : String = "http://localhost:8080/latest/meta-data/iam/security-credentials".to_string();
```
