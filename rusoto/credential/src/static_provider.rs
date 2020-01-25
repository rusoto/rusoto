//! Provides a way to create static/programmatically generated AWS Credentials.
//! For those who can't get them from an environment, or a file.
use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{AwsCredentials, CredentialsError, ProvideAwsCredentials};

/// Provides AWS credentials from statically/programmatically provided strings.
#[derive(Clone, Debug)]
pub struct StaticProvider {
    /// AWS credentials
    credentials: AwsCredentials,

    /// The time in seconds for which each issued token is valid.
    valid_for: Option<i64>,
}

impl StaticProvider {
    /// Creates a new Static Provider. This should be used when you want to statically, or programmatically
    /// provide access to AWS.
    ///
    /// `valid_for` is the number of seconds for which issued tokens are valid.
    pub fn new(
        access_key: String,
        secret_access_key: String,
        token: Option<String>,
        valid_for: Option<i64>,
    ) -> StaticProvider {
        StaticProvider {
            credentials: AwsCredentials::new(access_key, secret_access_key, token, None),
            valid_for,
        }
    }

    /// Creates a new minimal Static Provider. This will set the token as optional none.
    pub fn new_minimal(access_key: String, secret_access_key: String) -> StaticProvider {
        StaticProvider {
            credentials: AwsCredentials::new(access_key, secret_access_key, None, None),
            valid_for: None,
        }
    }

    /// Gets the AWS Access Key ID for this Static Provider.
    pub fn get_aws_access_key_id(&self) -> &str {
        &self.credentials.key
    }

    /// Gets the AWS Secret Access Key for this Static Provider.
    pub fn get_aws_secret_access_key(&self) -> &str {
        &self.credentials.secret
    }

    /// Determines if this Static Provider was given a Token.
    pub fn has_token(&self) -> bool {
        self.credentials.token.is_some()
    }

    /// Gets The Token this Static Provider was given.
    pub fn get_token(&self) -> &Option<String> {
        &self.credentials.token
    }

    /// Returns the length in seconds this Static Provider will be valid for.
    pub fn is_valid_for(&self) -> &Option<i64> {
        &self.valid_for
    }
}

#[async_trait]
impl ProvideAwsCredentials for StaticProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let mut creds = self.credentials.clone();
        creds.expires_at = self.valid_for.map(|v| Utc::now() + Duration::seconds(v));
        Ok(creds)
    }
}

impl From<AwsCredentials> for StaticProvider {
    fn from(credentials: AwsCredentials) -> Self {
        StaticProvider {
            credentials,
            valid_for: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time;

    use super::*;
    use crate::test_utils::{is_secret_hidden_behind_asterisks, SECRET};
    use crate::ProvideAwsCredentials;

    use quickcheck_macros::quickcheck;

    #[tokio::test]
    async fn test_static_provider_creation() {
        let result = StaticProvider::new(
            "fake-key".to_owned(),
            "fake-secret".to_owned(),
            Some("token".to_owned()),
            Some(300),
        )
        .credentials()
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_static_provider_minimal_creation() {
        let result =
            StaticProvider::new_minimal("fake-key-2".to_owned(), "fake-secret-2".to_owned())
                .credentials()
                .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_static_provider_custom_time_expiration() {
        let start_time = Utc::now();
        let result = StaticProvider::new(
            "fake-key".to_owned(),
            "fake-secret".to_owned(),
            None,
            Some(10000),
        )
        .credentials()
        .await;
        assert!(result.is_ok());
        let finalized = result.unwrap();
        let expires_at = finalized.expires_at().unwrap().clone();

        // Give a wide range of time, just in case there's somehow an immense amount of lag.
        assert!(start_time + Duration::minutes(100) < expires_at);
        assert!(expires_at < start_time + Duration::minutes(200));
    }

    #[tokio::test]
    async fn test_static_provider_expiration_time_is_recalculated() {
        let provider = StaticProvider::new(
            "fake-key".to_owned(),
            "fake-secret".to_owned(),
            None,
            Some(10000),
        );
        let creds1 = provider.credentials().await.unwrap();
        thread::sleep(time::Duration::from_secs(1));
        let creds2 = provider.credentials().await.unwrap();
        assert!(creds1.expires_at() < creds2.expires_at());
    }

    #[quickcheck]
    fn test_static_provider_secrets_not_in_debug(
        access_key: String,
        token: Option<()>,
        valid_for: Option<i64>,
    ) -> bool {
        let provider = StaticProvider::new(
            access_key,
            SECRET.to_owned(),
            token.map(|_| SECRET.to_owned()),
            valid_for,
        );
        is_secret_hidden_behind_asterisks(&provider)
    }
}
