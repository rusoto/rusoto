//! The Credentials Provider for an AWS Resource's IAM Role.

use std::time::Duration;

use futures::future::{result, FutureResult};
use futures::{Future, Poll};
use hyper::Uri;

use crate::request::{HttpClient, HttpClientFuture};
use crate::{parse_credentials_from_aws_service, AwsCredentials, CredentialsError, ProvideAwsCredentials};

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

enum InstanceMetadataFutureState {
    Start,
    GetRoleName(HttpClientFuture),
    GetCredentialsFromRole(HttpClientFuture),
    Done(FutureResult<AwsCredentials, CredentialsError>),
}

/// Future returned from `InstanceMetadataProvider`.
pub struct InstanceMetadataProviderFuture {
    state: InstanceMetadataFutureState,
    client: HttpClient,
    timeout: Duration,
}

impl Future for InstanceMetadataProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let new_state = match self.state {
            InstanceMetadataFutureState::Start => {
                let new_future = get_role_name(&self.client, self.timeout)?;
                InstanceMetadataFutureState::GetRoleName(new_future)
            }
            InstanceMetadataFutureState::GetRoleName(ref mut future) => {
                let role_name = try_ready!(future.poll());
                let new_future = get_credentials_from_role(&self.client, self.timeout, &role_name)?;
                InstanceMetadataFutureState::GetCredentialsFromRole(new_future)
            }
            InstanceMetadataFutureState::GetCredentialsFromRole(ref mut future) => {
                let cred_str = try_ready!(future.poll());
                let new_future = result(parse_credentials_from_aws_service(&cred_str));
                InstanceMetadataFutureState::Done(new_future)
            }
            InstanceMetadataFutureState::Done(ref mut future) => {
                return future.poll();
            }
        };
        self.state = new_state;
        self.poll()
    }
}

impl ProvideAwsCredentials for InstanceMetadataProvider {
    type Future = InstanceMetadataProviderFuture;

    fn credentials(&self) -> Self::Future {
        InstanceMetadataProviderFuture {
            state: InstanceMetadataFutureState::Start,
            client: self.client.clone(),
            timeout: self.timeout,
        }
    }
}

/// Gets the role name to get credentials for using the IAM Metadata Service (169.254.169.254).
fn get_role_name(
    client: &HttpClient,
    timeout: Duration,
) -> Result<HttpClientFuture, CredentialsError> {
    let role_name_address = format!(
        "http://{}/{}/",
        AWS_CREDENTIALS_PROVIDER_IP, AWS_CREDENTIALS_PROVIDER_PATH
    );
    let uri = role_name_address
        .parse::<Uri>()
        .map_err(|err| CredentialsError::new(err))?;
    Ok(client.get(uri, timeout))
}

/// Gets the credentials for an EC2 Instances IAM Role.
fn get_credentials_from_role(
    client: &HttpClient,
    timeout: Duration,
    role_name: &str,
) -> Result<HttpClientFuture, CredentialsError> {
    let credentials_provider_url = format!(
        "http://{}/{}/{}",
        AWS_CREDENTIALS_PROVIDER_IP, AWS_CREDENTIALS_PROVIDER_PATH, role_name
    );

    let uri = credentials_provider_url
        .parse::<Uri>()
        .map_err(|err| CredentialsError::new(err))?;

    Ok(client.get(uri, timeout))
}
