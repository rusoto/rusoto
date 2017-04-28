use {AwsCredentials, CredentialsError, ProvideAwsCredentials};
use chrono::{Duration, UTC};

/// Provides AWS credentials from statically/programmatically provided strings.
#[derive(Debug)]
pub struct StaticProvider {
    /// The AWS Access Key ID to use for authenticating to AWS.
    pub aws_access_key_id: String,
    /// The AWS Secret Access Key to use for authenticating to AWS.
    pub aws_secret_access_key: String,
    /// The optional token to use for authenticating to aWS.
    pub token: Option<String>,
    /// The time in seconds each issued token should be valid ofr.
    pub valid_for: i64,
}

impl StaticProvider {
    /// Creates a new Static Provider. This should be used when you want to statically, or programmatically
    /// provide access to AWS.
    pub fn new(access_key: String, secret_access_key: String, token: Option<String>, valid_for: Option<i64>) -> StaticProvider {
        StaticProvider {
            aws_access_key_id: access_key,
            aws_secret_access_key: secret_access_key,
            token: token,
            valid_for: valid_for.unwrap_or(600),
        }
    }

    /// Creates a new minimal Static Provider. This will set the token as optional none,
    /// and the wait for time as 10 minutes. Which are the defaults elsewhere.
    pub fn new_minimal(access_key: String, secret_access_key: String) -> StaticProvider {
        StaticProvider {
            aws_access_key_id: access_key,
            aws_secret_access_key: secret_access_key,
            token: None,
            valid_for: 600,
        }
    }
}

impl ProvideAwsCredentials for StaticProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        Ok(AwsCredentials::new(self.aws_access_key_id.clone(), self.aws_secret_access_key.clone(),
            self.token.clone(), UTC::now() + Duration::seconds(self.valid_for)))
    }
}

#[cfg(test)]
mod tests {
    use {ProvideAwsCredentials};
    use super::*;

    #[test]
    fn test_static_provider_creation() {
        let result = StaticProvider::new("fake-key".to_owned(), "fake-secret".to_owned(), Some("token".to_owned()), Some(300)).credentials();
        assert!(result.is_ok());
    }

    #[test]
    fn test_static_provider_minimal_creation() {
        let result = StaticProvider::new_minimal("fake-key-2".to_owned(), "fake-secret-2".to_owned()).credentials();
        assert!(result.is_ok());
    }

    #[test]
    fn test_static_provider_custom_time_expiration() {
        let start_time = UTC::now();
        let result = StaticProvider::new("fake-key".to_owned(), "fake-secret".to_owned(), None, Some(10000)).credentials();
        assert!(result.is_ok());
        let finalized = result.unwrap();
        let time_diff = (finalized.expires_at().clone() - start_time).num_minutes();

        /// Give a wide range of time, just incase there's somehow an immense amount of lag.
        assert!(time_diff > 100);
        assert!(time_diff < 200);
    }
}