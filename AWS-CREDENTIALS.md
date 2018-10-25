### Credentials

Rusoto has the ability to source AWS access credentials in a few different ways:

1. Environment variables via `rusoto_core::EnvironmentProvider` (`AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`)
2. AWS credentials file via `rusoto_core::ProfileProvider`
3. IAM ECS container profile via `rusoto_core::ContainerProvider`
4. IAM EC2 instance profile via `rusoto_core::InstanceMetadataProvider`

There is also `rusoto_core::ChainProvider`, which is a convenience for attempting to source access credentials using the methods above in order.
If credentials cannot be obtained through one method, it falls back to the next.
If all possibilites are exhausted, an error will be returned.

`ProfileProvider` (and `ChainProvider`) also allow you to specify a custom path to the credentials file and the name of the profile to use.
If not explicitly provided as arguments, the values for these two parameters are computed according to the following rules:
* **location of credentials file:** if set and not empty the value of the environment variable ```AWS_SHARED_CREDENTIALS_FILE``` otherwise ```"~/.aws/credentials"```.
* **profile name:** if set and not empty the value of the environment variable ```AWS_PROFILE``` otherwise ```"default"```

It's also possible to implement your own credentials sourcing mechanism by creating a type that implements `rusoto_core::ProvideAwsCredentials`.

#### sts:AssumeRole

If your aws account belongs to an organization and you need to use sts:AssumeRole, you're probably looking for `rusoto_sts::StsAssumeRoleSessionCredentialsProvider`. A simple program that uses sts:AssumeRole looks like this:

```rust,no_run
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_ec2;
extern crate rusoto_sts;

use std::default::Default;

use rusoto_core::{Region, HttpClient};

use rusoto_ec2::{Ec2Client, Ec2, DescribeSpotInstanceRequestsRequest};
use rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};

fn main() {
    let _ = env_logger::try_init();

    let sts = StsClient::new(Region::EuWest1);

    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        "arn:aws:iam::something:role/something".to_owned(),
        "default".to_owned(),
        None, None, None, None
    );

    let client = Ec2Client::new_with(HttpClient::new().unwrap(), provider, Region::UsEast1);

    let sir_input = DescribeSpotInstanceRequestsRequest::default();
    let x = client.describe_spot_instance_requests(sir_input).sync();

    println!("{:?}", x);
}
```

#### Credential refreshing

Credentials obtained from environment variables and credential files expire ten minutes after being acquired and are refreshed on subsequent calls to `credentials()` (a method from the `ProvideAwsCredentials` trait).

IAM instance profile credentials are refreshed as needed.
Upon calling `credentials()` it will see if they are expired or not.
If expired, it attempts to get new credentials from the metadata service.
If that fails it will return an error.
IAM credentials expiration time comes from the IAM metadata response.

#### Local integration testing of IAM credentials

Edit the relevant `address`/IP locations in [credential/src/container.rs](credential/src/container.rs) and [credential/src/instance_metadata.rs](credential/src/instance_metadata.rs).
For local testing, you can use [moe](https://github.com/matthewkmayer/moe) and set the string to this:

```rust,ignore
let mut address: String = "http://localhost:8080/latest/meta-data/iam/security-credentials".to_owned();
```
