### Credentials

Rusoto has the ability to source AWS access credentials in a few different ways:

1. Environment variables via `rusoto::credentials::EnvironmentCredentialsProvider`
2. AWS credentials file via `rusoto::credentials::ProfileCredentialsProvider`
3. IAM instance profile via `rusoto::credentials::IAMRoleCredentialsProvider`

There is also `rusoto::credentials::DefaultAWSCredentialsProviderChain`, which is a convenience for attempting to source access credentials using the methods above in order.
If credentials cannot be obtained through one method, it falls back to the next.
If all possibilites are exhausted, an error will be returned.

`ProfileCredentialsProvider` (and `DefaultAWSCredentialsProviderChain`) also allow you to specify a custom path to the credentials file and the name of the profile to use.
If not specified, the profile "default" is used.

It's also possible to implement your own credentials sourcing mechanism by creating a type that implements `rusoto::credentials::AWSCredentialsProvider`.

#### Credential refreshing

Credentials obtained from environment variables and credential files expire ten minutes after being acquired and are refreshed on subsequent calls to `get_credentials()` (a method from the `AWSCredentialsProvider` trait).

IAM instance profile credentials are refreshed as needed.
Upon calling `get_credentials()` it will see if they are expired or not.
If expired, it attempts to get new credentials from the metadata service.
If that fails it will return an error.
IAM credentials expiration time comes from the IAM metadata response.

#### Local integration testing of IAM credentials

Edit the `address` location in [src/credentials.rs](src/credentials.rs).
For local testing, you can use [moe](https://github.com/matthewkmayer/moe) and set the string to this:

```rust
let mut address: String = "http://localhost:8080/latest/meta-data/iam/security-credentials".to_owned();
```
