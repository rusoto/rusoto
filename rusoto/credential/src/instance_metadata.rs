//! The Credentials Provider for an AWS Resource's IAM Role.

use async_trait::async_trait;
use hyper::Uri;
use std::{borrow::Cow, time::Duration};

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
/// # Examples
///
/// ```rust
/// use std::time::Duration;
///
/// use rusoto_credential::InstanceMetadataProvider;
///
/// let mut provider = InstanceMetadataProvider::new();
/// // you can overwrite the default timeout like this:
/// provider.set_timeout(Duration::from_secs(60));
/// ```
///
/// The source location can be changed from the default of 169.254.169.254:
///
/// ```rust
/// use std::time::Duration;
///
/// use rusoto_credential::InstanceMetadataProvider;
///
/// let mut provider = InstanceMetadataProvider::new();
/// // you can overwrite the default endpoint like this:
/// provider.set_ip_addr_with_port("127.0.0.1", "8080");
/// ```
#[derive(Clone, Debug)]
pub struct InstanceMetadataProvider {
    client: HttpClient,
    timeout: Duration,
    metadata_ip_addr: String,
    role_arn: Option<String>,
}

impl InstanceMetadataProvider {
    /// Create a new provider with the given handle.
    pub fn new() -> Self {
        InstanceMetadataProvider {
            client: HttpClient::new(),
            timeout: Duration::from_secs(30),
            metadata_ip_addr: AWS_CREDENTIALS_PROVIDER_IP.to_string(),
            role_arn: None,
        }
    }

    /// Set the timeout on the provider to the specified duration.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Allow overriding host and port of instance metadata service.
    pub fn set_ip_addr_with_port(&mut self, ip: &str, port: &str) {
        self.metadata_ip_addr = format!("{}:{}", ip, port);
    }

    /// Allow setting the IAM role name ARN directly to avoid
    /// making two calls to the instance metadata API.
    ///
    /// It looks like arn:aws:iam::xxxxxxxxxxxx:role/<role-name>
    pub fn set_role_arn(&mut self, role_arn: String) {
        self.role_arn = Some(role_arn);
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
        let role_arn = if let Some(role_arn) = &self.role_arn {
            Cow::from(role_arn)
        } else {
            Cow::from(
                get_role_arn(&self.client, self.timeout, &self.metadata_ip_addr)
                    .await
                    .map_err(|err| CredentialsError {
                        message: format!("Could not get credentials from iam: {}", err.to_string()),
                    })?,
            )
        };

        let cred_str = get_credentials_from_role(
            &self.client,
            self.timeout,
            &role_arn,
            &self.metadata_ip_addr,
        )
        .await
        .map_err(|err| CredentialsError {
            message: format!("Could not get credentials from iam: {}", err.to_string()),
        })?;

        parse_credentials_from_aws_service(&cred_str)
    }
}

/// Gets the role name to get credentials for using the IAM Metadata Service (169.254.169.254).
async fn get_role_arn(
    client: &HttpClient,
    timeout: Duration,
    ip_addr: &str,
) -> Result<String, CredentialsError> {
    let role_arn_address = format!("http://{}/{}/", ip_addr, AWS_CREDENTIALS_PROVIDER_PATH);
    let uri = match role_arn_address.parse::<Uri>() {
        Ok(u) => u,
        Err(e) => return Err(CredentialsError::new(e)),
    };

    Ok(client.get(uri, timeout).await?)
}

/// Gets the credentials for an EC2 Instances IAM Role.
async fn get_credentials_from_role(
    client: &HttpClient,
    timeout: Duration,
    role_arn: &str,
    ip_addr: &str,
) -> Result<String, CredentialsError> {
    let credentials_provider_url = format!(
        "http://{}/{}/{}",
        ip_addr, AWS_CREDENTIALS_PROVIDER_PATH, role_arn
    );

    let uri = match credentials_provider_url.parse::<Uri>() {
        Ok(u) => u,
        Err(e) => return Err(CredentialsError::new(e)),
    };

    Ok(client.get(uri, timeout).await?)
}
