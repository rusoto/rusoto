#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]
#![deny(missing_docs)]

//! Types for loading and managing AWS access credentials for API requests.

pub use crate::container::ContainerProvider;
pub use crate::environment::EnvironmentProvider;
pub use crate::instance_metadata::InstanceMetadataProvider;
pub use crate::profile::ProfileProvider;
pub use crate::static_provider::StaticProvider;

pub mod claims;
mod container;
mod environment;
mod instance_metadata;
mod profile;
mod request;
mod static_provider;
#[cfg(test)]
pub(crate) mod test_utils;

use async_trait::async_trait;
use std::collections::BTreeMap;
use std::env::{var as env_var, VarError};
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Duration as ChronoDuration, ParseError, Utc};
use hyper::Error as HyperError;
use serde::Deserialize;
use tokio::sync::Mutex;

/// AWS API access credentials, including access key, secret key, token (for IAM profiles),
/// expiration timestamp, and claims from federated login.
#[derive(Clone, Deserialize, Default)]
pub struct AwsCredentials {
    #[serde(rename = "AccessKeyId")]
    key: String,
    #[serde(rename = "SecretAccessKey")]
    secret: String,
    #[serde(rename = "Token")]
    token: Option<String>,
    #[serde(rename = "Expiration")]
    expires_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    claims: BTreeMap<String, String>,
}

impl AwsCredentials {
    /// Create a new `AwsCredentials` from a key ID, secret key, optional access token, and expiry
    /// time.
    pub fn new<K, S>(
        key: K,
        secret: S,
        token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> AwsCredentials
    where
        K: Into<String>,
        S: Into<String>,
    {
        AwsCredentials {
            key: key.into(),
            secret: secret.into(),
            token,
            expires_at,
            claims: BTreeMap::new(),
        }
    }

    /// Get a reference to the access key ID.
    pub fn aws_access_key_id(&self) -> &str {
        &self.key
    }

    /// Get a reference to the secret access key.
    pub fn aws_secret_access_key(&self) -> &str {
        &self.secret
    }

    /// Get a reference to the expiry time.
    pub fn expires_at(&self) -> &Option<DateTime<Utc>> {
        &self.expires_at
    }

    /// Get a reference to the access token.
    pub fn token(&self) -> &Option<String> {
        &self.token
    }

    /// Determine whether or not the credentials are expired.
    fn credentials_are_expired(&self) -> bool {
        match self.expires_at {
            Some(ref e) =>
            // This is a rough hack to hopefully avoid someone requesting creds then sitting on them
            // before issuing the request:
            {
                *e < Utc::now() + ChronoDuration::seconds(20)
            }
            None => false,
        }
    }

    /// Get the token claims
    pub fn claims(&self) -> &BTreeMap<String, String> {
        &self.claims
    }

    /// Get the mutable token claims
    pub fn claims_mut(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.claims
    }
}

impl fmt::Debug for AwsCredentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AwsCredentials")
            .field("key", &self.key)
            .field("secret", &"**********")
            .field("token", &self.token.as_ref().map(|_| "**********"))
            .field("expires_at", &self.expires_at)
            .field("claims", &self.claims)
            .finish()
    }
}

/// Represents an Error that has occured during the fetching Credentials Phase.
///
/// This generally is an error message from one of our underlying libraries, however
/// we wrap it up with this type so we can export one single error type.
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialsError {
    /// The underlying error message for the credentials error.
    pub message: String,
}

impl CredentialsError {
    /// Creates a new Credentials Error.
    ///
    /// * `message` - The Error message for this CredentialsError.
    pub fn new<S>(message: S) -> CredentialsError
    where
        S: ToString,
    {
        CredentialsError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CredentialsError {}

impl From<ParseError> for CredentialsError {
    fn from(err: ParseError) -> CredentialsError {
        CredentialsError::new(err)
    }
}

impl From<IoError> for CredentialsError {
    fn from(err: IoError) -> CredentialsError {
        CredentialsError::new(err)
    }
}

impl From<HyperError> for CredentialsError {
    fn from(err: HyperError) -> CredentialsError {
        CredentialsError::new(format!("Couldn't connect to credentials provider: {}", err))
    }
}

impl From<serde_json::Error> for CredentialsError {
    fn from(err: serde_json::Error) -> CredentialsError {
        CredentialsError::new(err)
    }
}

impl From<VarError> for CredentialsError {
    fn from(err: VarError) -> CredentialsError {
        CredentialsError::new(err)
    }
}

impl From<FromUtf8Error> for CredentialsError {
    fn from(err: FromUtf8Error) -> CredentialsError {
        CredentialsError::new(err)
    }
}

/// A trait for types that produce `AwsCredentials`.
#[async_trait]
pub trait ProvideAwsCredentials {
    /// Produce a new `AwsCredentials` future.
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError>;
}

//#[async_trait]
//impl<P: ProvideAwsCredentials> ProvideAwsCredentials for Rc<P> {
//    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
//        P::credentials(self).await
//    }
//}

#[async_trait]
impl<P: ProvideAwsCredentials + Send + Sync> ProvideAwsCredentials for Arc<P> {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        P::credentials(self).await
    }
}

/// Wrapper for `ProvideAwsCredentials` that caches the credentials returned by the
/// wrapped provider.  Each time the credentials are accessed, they are checked to see if
/// they have expired, in which case they are retrieved from the wrapped provider again.
///
/// In order to access the wrapped provider, for instance to set a timeout, the `get_ref`
/// and `get_mut` methods can be used.
#[derive(Debug, Clone)]
pub struct AutoRefreshingProvider<P: ProvideAwsCredentials + 'static> {
    credentials_provider: P,
    current_credentials: Arc<Mutex<Option<Result<AwsCredentials, CredentialsError>>>>,
}

impl<P: ProvideAwsCredentials + 'static> AutoRefreshingProvider<P> {
    /// Create a new `AutoRefreshingProvider` around the provided base provider.
    pub fn new(provider: P) -> Result<AutoRefreshingProvider<P>, CredentialsError> {
        Ok(AutoRefreshingProvider {
            credentials_provider: provider,
            current_credentials: Arc::new(Mutex::new(None)),
        })
    }

    /// Get a shared reference to the wrapped provider.
    pub fn get_ref(&self) -> &P {
        &self.credentials_provider
    }

    /// Get a mutable reference to the wrapped provider.
    ///
    /// This can be used to call `set_timeout` on the wrapped
    /// provider.
    pub fn get_mut(&mut self) -> &mut P {
        &mut self.credentials_provider
    }
}

#[async_trait]
impl<P: ProvideAwsCredentials + Send + Sync + 'static> ProvideAwsCredentials
    for AutoRefreshingProvider<P>
{
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        loop {
            let mut guard = self.current_credentials.lock().await;
            match guard.as_ref() {
                // no result from the future yet, let's keep using it
                None => {
                    let res = self.credentials_provider.credentials().await;
                    *guard = Some(res);
                }
                Some(Err(e)) => return Err(e.clone()),
                Some(Ok(creds)) => {
                    if creds.credentials_are_expired() {
                        *guard = None;
                    } else {
                        return Ok(creds.clone());
                    };
                }
            }
        }
    }
}

/// Wraps a `ChainProvider` in an `AutoRefreshingProvider`.
///
/// The underlying `ChainProvider` checks multiple sources for credentials, and the `AutoRefreshingProvider`
/// refreshes the credentials automatically when they expire.
///
/// # Warning
///
/// This provider allows the [`credential_process`][credential_process] option in the AWS config
/// file (`~/.aws/config`), a method of sourcing credentials from an external process. This can
/// potentially be dangerous, so proceed with caution. Other credential providers should be
/// preferred if at all possible. If using this option, you should make sure that the config file
/// is as locked down as possible using security best practices for your operating system.
///
/// [credential_process]: https://docs.aws.amazon.com/cli/latest/topic/config-vars.html#sourcing-credentials-from-external-processes
#[derive(Clone)]
pub struct DefaultCredentialsProvider(AutoRefreshingProvider<ChainProvider>);

impl DefaultCredentialsProvider {
    /// Creates a new thread-safe `DefaultCredentialsProvider`.
    pub fn new() -> Result<DefaultCredentialsProvider, CredentialsError> {
        let inner = AutoRefreshingProvider::new(ChainProvider::new())?;
        Ok(DefaultCredentialsProvider(inner))
    }
}

#[async_trait]
impl ProvideAwsCredentials for DefaultCredentialsProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        self.0.credentials().await
    }
}

/// Provides AWS credentials from multiple possible sources using a priority order.
///
/// The following sources are checked in order for credentials when calling `credentials`:
///
/// 1. Environment variables: `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`
/// 2. `credential_process` command in the AWS config file, usually located at `~/.aws/config`.
/// 3. AWS credentials file. Usually located at `~/.aws/credentials`.
/// 4. IAM instance profile. Will only work if running on an EC2 instance with an instance profile/role.
///
/// If the sources are exhausted without finding credentials, an error is returned.
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
/// use rusoto_credential::ChainProvider;
///
/// fn main() {
///   let mut provider = ChainProvider::new();
///   // you can overwrite the default timeout like this:
///   provider.set_timeout(Duration::from_secs(60));
///
///   // ...
/// }
/// ```
///
/// # Warning
///
/// This provider allows the [`credential_process`][credential_process] option in the AWS config
/// file (`~/.aws/config`), a method of sourcing credentials from an external process. This can
/// potentially be dangerous, so proceed with caution. Other credential providers should be
/// preferred if at all possible. If using this option, you should make sure that the config file
/// is as locked down as possible using security best practices for your operating system.
///
/// [credential_process]: https://docs.aws.amazon.com/cli/latest/topic/config-vars.html#sourcing-credentials-from-external-processes
#[derive(Debug, Clone)]
pub struct ChainProvider {
    environment_provider: EnvironmentProvider,
    instance_metadata_provider: InstanceMetadataProvider,
    container_provider: ContainerProvider,
    profile_provider: Option<ProfileProvider>,
}

impl ChainProvider {
    /// Set the timeout on the provider to the specified duration.
    pub fn set_timeout(&mut self, duration: Duration) {
        self.instance_metadata_provider.set_timeout(duration);
        self.container_provider.set_timeout(duration);
    }
}

async fn chain_provider_credentials(
    provider: ChainProvider,
) -> Result<AwsCredentials, CredentialsError> {
    if let Ok(creds) = provider.environment_provider.credentials().await {
        return Ok(creds);
    }
    if let Some(ref profile_provider) = provider.profile_provider {
        if let Ok(creds) = profile_provider.credentials().await {
            return Ok(creds);
        }
    }
    if let Ok(creds) = provider.container_provider.credentials().await {
        return Ok(creds);
    }
    if let Ok(creds) = provider.instance_metadata_provider.credentials().await {
        return Ok(creds);
    }
    Err(CredentialsError::new(
        "Couldn't find AWS credentials in environment, credentials file, or IAM role.",
    ))
}

#[async_trait]
impl ProvideAwsCredentials for ChainProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        chain_provider_credentials(self.clone()).await
    }
}

impl ChainProvider {
    /// Create a new `ChainProvider` using a `ProfileProvider` with the default settings.
    pub fn new() -> ChainProvider {
        ChainProvider {
            environment_provider: EnvironmentProvider::default(),
            profile_provider: ProfileProvider::new().ok(),
            instance_metadata_provider: InstanceMetadataProvider::new(),
            container_provider: ContainerProvider::new(),
        }
    }

    /// Create a new `ChainProvider` using the provided `ProfileProvider`.
    pub fn with_profile_provider(profile_provider: ProfileProvider) -> ChainProvider {
        ChainProvider {
            environment_provider: EnvironmentProvider::default(),
            profile_provider: Some(profile_provider),
            instance_metadata_provider: InstanceMetadataProvider::new(),
            container_provider: ContainerProvider::new(),
        }
    }
}

impl Default for ChainProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// This is a helper function as Option<T>::filter is not yet stable (see issue #45860).
/// <https://github.com/rust-lang/rfcs/issues/2036> also affects the implementation of this.
fn non_empty_env_var(name: &str) -> Option<String> {
    match env_var(name) {
        Ok(value) => {
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        }
        Err(_) => None,
    }
}

/// Parses the response from an AWS Metadata Service, either from an IAM Role, or a Container.
fn parse_credentials_from_aws_service(response: &str) -> Result<AwsCredentials, CredentialsError> {
    Ok(serde_json::from_str::<AwsCredentials>(response)?)
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Read;
    use std::path::Path;

    use crate::test_utils::{is_secret_hidden_behind_asterisks, lock, ENV_MUTEX, SECRET};
    use quickcheck::quickcheck;

    use super::*;

    #[test]
    fn providers_are_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync>() {}

        is_send_and_sync::<ChainProvider>();
        is_send_and_sync::<AutoRefreshingProvider<ChainProvider>>();
        is_send_and_sync::<DefaultCredentialsProvider>();
    }

    #[tokio::test]
    async fn profile_provider_finds_right_credentials_in_file() {
        let _guard = lock(&ENV_MUTEX);
        let profile_provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );

        let credentials = profile_provider.credentials().await.expect(
            "Failed to get credentials from profile provider using tests/sample-data/multiple_profile_credentials",
        );

        assert_eq!(credentials.aws_access_key_id(), "foo_access_key");
        assert_eq!(credentials.aws_secret_access_key(), "foo_secret_key");
    }

    #[test]
    fn parse_iam_task_credentials_sample_response() {
        fn read_file_to_string(file_path: &Path) -> String {
            match fs::metadata(file_path) {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        panic!("Couldn't open file");
                    }
                }
                Err(_) => panic!("Couldn't stat file"),
            };

            let mut file = File::open(file_path).unwrap();
            let mut result = String::new();
            file.read_to_string(&mut result).ok();

            result
        }

        let response = read_file_to_string(Path::new(
            "tests/sample-data/iam_task_credentials_sample_response",
        ));

        let credentials = parse_credentials_from_aws_service(&response);

        assert!(credentials.is_ok());
        let credentials = credentials.unwrap();

        assert_eq!(credentials.aws_access_key_id(), "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(
            credentials.aws_secret_access_key(),
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
        );

        assert_eq!(
            credentials.expires_at().expect(""),
            DateTime::parse_from_rfc3339("2016-11-18T01:50:39Z").expect("")
        );
    }

    #[cfg(test)]
    quickcheck! {
        fn test_aws_credentials_secrets_not_in_debug(
            key: String,
            valid_for: Option<i64>,
            token: Option<()>
        ) -> bool {
            let creds = AwsCredentials::new(
                key,
                SECRET.to_owned(),
                token.map(|_| test_utils::SECRET.to_owned()),
                valid_for.map(|v| Utc::now() + ChronoDuration::seconds(v)),
            );
            is_secret_hidden_behind_asterisks(&creds)
        }
    }
}
