//! The Credentials Provider for an AWS Resource's IAM Role.

use async_trait::async_trait;
use hyper::Uri;
use std::time::Duration;

use crate::request::HttpClient;
use crate::{
    parse_credentials_from_aws_service, AwsCredentials, CredentialsError, ProvideAwsCredentials,
};

const AWS_CREDENTIALS_PROVIDER_IP: &str = "169.254.169.254";
const AWS_CREDENTIALS_PROVIDER_PATH: &str = "latest/meta-data/iam/security-credentials";

/// Provides AWS credentials from a resource's IAM role.
///
/// The provider has a default timeout of 30 seconds. While it should work well for most setups,
/// you can change the timeout using the `set_timeout` method.
///
/// # Example
///
/// ```rust
/// extern crate rusoto_credential;
///
/// use std::time::Duration;
///
/// use rusoto_credential::InstanceMetadataProvider;
///
/// fn main() {
///   let mut provider = InstanceMetadataProvider::new();
///   // you can overwrite the default timeout like this:
///   provider.set_timeout(Duration::from_secs(60));
///
///   // ...
/// }
/// ```
#[derive(Clone, Debug)]
pub struct InstanceMetadataProvider {
    client: HttpClient,
    timeout: Duration,
}

impl InstanceMetadataProvider {
    /// Create a new provider with the given handle.
    pub fn new() -> Self {
        InstanceMetadataProvider {
            client: HttpClient::new(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the timeout on the provider to the specified duration.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}

impl Default for InstanceMetadataProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProvideAwsCredentials for InstanceMetadataProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let role_name_address = format!(
            "http://{}/{}/",
            AWS_CREDENTIALS_PROVIDER_IP, AWS_CREDENTIALS_PROVIDER_PATH
        );
        let uri = match role_name_address.parse::<Uri>() {
            Ok(u) => u,
            Err(e) => return Err(CredentialsError::new(e)),
        };

        let role_name =
            self.client
                .get(uri, self.timeout)
                .await
                .map_err(|err| CredentialsError {
                    message: format!("Could not get credentials from iam: {}", err.to_string()),
                })?;

        let credentials_provider_url = format!(
            "http://{}/{}/{}",
            AWS_CREDENTIALS_PROVIDER_IP, AWS_CREDENTIALS_PROVIDER_PATH, role_name
        );

        let uri = match credentials_provider_url.parse::<Uri>() {
            Ok(u) => u,
            Err(e) => return Err(CredentialsError::new(e)),
        };

        let cred_str =
            self.client
                .get(uri, self.timeout)
                .await
                .map_err(|err| CredentialsError {
                    message: format!("Could not get credentials from iam: {}", err.to_string()),
                })?;
        parse_credentials_from_aws_service(&cred_str)
    }
}
