### Credentials

Rusoto will search for credentials in this order:

1. Environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`.
2. AWS Credentials file: `~/.aws/credentials`.  It will use `aws_access_key_id` and `aws_secret_access_key` fields.
Profiles are supported.
3. IAM instance profile.  Rusoto will query the metadata service for an instance profile/role and fetch the access key, secret access key and token to supply those for requests.

If Rusoto exhausts all three options it will return an error.

#### Credential refreshing

Credentials obtained from environment variables and credential files expire ten minutes after being acquired, and are refreshed on subsequent calls to `get_credentials()`.

IAM instance profile credentials are refreshed as needed.  Upon calling `get_credentials()` it will see if they are expired or not.  If expired, it attempts to get new credentials from the metadata service.  If that fails it will return an error.  IAM credentials expiration time comes from the IAM metadata response.

#### Local integration testing of IAM credentials

Edit the `address` location in [src/credentials.rs](src/credentials.rs).  For local testing, I use [moe](https://github.com/matthewkmayer/moe) and set the string to this:

```rust
let mut address : String = "http://localhost:8080/latest/meta-data/iam/security-credentials".to_string();
```
