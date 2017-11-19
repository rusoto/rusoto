## Rusoto integration tests

#### Warning - these can incur charges on AWS

These tests talk to real AWS services and your account may be charged for the actions taken.

Also, if tests fail they can leave items in your AWS account, such as S3 buckets.

#### Running the tests

In this directory, `integration_tests`, you can run all tests with `cargo test --features all`.

Specific tests can be run using their feature flags.  To run the S3 tests: `cargo test --features s3`.

To run multiple tests, add the feature flags: `cargo test --features "ecs ec2"`.