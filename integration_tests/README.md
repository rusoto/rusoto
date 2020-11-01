## Rusoto integration tests

#### Warning - these can incur charges on AWS

These tests talk to real AWS services and your account may be charged for the actions taken.

Also, if tests fail they can leave items in your AWS account, such as S3 buckets.

#### Running the tests against AWS

In this directory, `integration_tests`, you can run all tests with `cargo test --features all`.

Specific service tests can be run using their feature flags.  To run the S3 tests: `cargo test --features s3`.

To run multiple service tests, add the feature flags: `cargo test --features "ecs ec2"`.

There will be a lot of tests in the cargo test output. To limit this output when targetting runs of specific tests you may wish to use cargo test's `--test <testname>` filter. You can use this flag more than once for more than one test.

```sh
cargo test --features "amplify ram" --test amplify --test ram -- --nocapture
```

#### Running S3 tests against Minio or Ceph

Dependencies:
* [Docker](https://docs.docker.com/install/)
* Python3
* Python3 library requests (e.g. `apt install python3-requests` or `pip3 install --user requests`)

**Ceph**: Execute `../.semaphoreci/test_10_ceph.sh` while you're in this directory.
**Minio**: Execute `../.semaphoreci/test_20_minio.sh` while you're in this directory

