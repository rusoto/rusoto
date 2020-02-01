### Credentials

#### Best Practices

Please follow the [AWS documentation on best practices](https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html) for managing credentials for your account. These credentials provide anyone who has access to them access to your AWS account. You can mitigate risk that goes along with that access by following these best practices.

#### Usage

Much like the [standard AWS toolchain](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html), Rusoto has the ability to source AWS access credentials from multiple sources, either independently or in a tiered fashion.

1. Environment variables via [`rusoto_core::EnvironmentProvider`](https://docs.rs/rusoto_credential/latest/rusoto_credential/struct.EnvironmentProvider.html) (`AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`)
2. AWS credentials file via [`rusoto_core::ProfileProvider`](https://docs.rs/rusoto_credential/latest/rusoto_credential/struct.ProfileProvider.html)
3. IAM ECS container profile via [`rusoto_core::ContainerProvider`](https://docs.rs/rusoto_credential/latest/rusoto_credential/struct.ContainerProvider.html)
4. IAM EC2 instance profile via [`rusoto_core::InstanceMetadataProvider`](https://docs.rs/rusoto_credential/latest/rusoto_credential/struct.InstanceMetadataProvider.html)

The [`rusoto_core::ChainProvider`](https://docs.rs/rusoto_credential/latest/rusoto_credential/struct.ChainProvider.html) is a convenience for attempting to source access credentials using all the methods above in that order.
If credentials cannot be obtained through one method, it falls back to the next.
If all possibilites are exhausted, an error will be returned.

`ProfileProvider` (and `ChainProvider`) also allow you to specify a custom path to the credentials file and the name of the profile to use.
If not explicitly provided as arguments, the values for these two parameters are computed according to the following rules:
* **location of credentials file:** if set and not empty the value of the environment variable `AWS_SHARED_CREDENTIALS_FILE` otherwise `"~/.aws/credentials"`.
* **profile name:** if set and not empty the value of the environment variable ```AWS_PROFILE``` otherwise `"default"`

It's also possible to implement your own credentials sourcing mechanism by creating a type that implements `rusoto_core::ProvideAwsCredentials`.

`rusoto_sts` implements a replacement `ChainProvider` which adds a credential source for web identities for use with Kubernetes.

#### sts:AssumeRole

If your aws account belongs to an organization and you need to use sts:AssumeRole, you're probably looking for `rusoto_sts::StsAssumeRoleSessionCredentialsProvider`. A simple program that uses sts:AssumeRole looks like this:

```rust,no_run
use std::default::Default;

use rusoto_core::{Region, HttpClient};

use rusoto_ec2::{Ec2Client, Ec2, DescribeSpotInstanceRequestsRequest};
use rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};

#[tokio::main]
async fn main() {
    env_logger::init();

    let sts = StsClient::new(Region::EuWest1);

    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        "arn:aws:iam::something:role/something".to_owned(),
        "default".to_owned(),
        None, None, None, None
    );

    let client = Ec2Client::new_with(HttpClient::new().unwrap(), provider, Region::UsEast1);

    let sir_input = DescribeSpotInstanceRequestsRequest::default();
    let x = client.describe_spot_instance_requests(sir_input).await;

    println!("{:?}", x);
}
```

### Important note about using the StsAssumeRoleSessionCredentialsProvider

**Be careful**. The current `rusoto_sts::StsAssumeRoleSessionCredentialsProvider` should be used with `rusoto_credential::AutoRefreshingProvider` as a wrapper to take advantage of using the already cached token of AssumeRole as its only valid for 1 hour by default.
The current implementation does not reuse the cached token returned by the AssumeRole by default so it will be refreshed with every call to AWS resource.

This will affect the performance as well as the size of you AWS bill.

- https://docs.rs/rusoto_credential
- https://crates.io/crates/rusoto_credential
```
let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        "arn:aws:iam::something:role/something".to_owned(),
        "default".to_owned(),
        None, None, None, None
    );

let auto_refreshing_provider = rusoto_credential::AutoRefreshingProvider::new(provider);
```

#### Local integration testing of IAM credentials

Local testing of IAM credentials can be done using [credential_service_mock](rusoto/credential_service_mock) which runs as a local [Warp](https://crates.io/crates/warp) server.
 
See [this integration test](rusoto/credential/tests/instance-profile-test.rs) for a working example of local testing.