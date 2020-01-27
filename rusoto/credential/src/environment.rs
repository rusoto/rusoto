//! The Credentials Provider to read from Environment Variables.
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};

use crate::{non_empty_env_var, AwsCredentials, CredentialsError, ProvideAwsCredentials};

/// Provides AWS credentials from environment variables.
///
/// # Available Environment Variables
///
/// * `AWS_ACCESS_KEY_ID`:
///
///   [Access key ID](https://docs.aws.amazon.com/general/latest/gr/aws-sec-cred-types.html#access-keys-and-secret-access-keys)
///
/// * `AWS_SECRET_ACCESS_KEY`:
///
///   [Secret access key](https://docs.aws.amazon.com/general/latest/gr/aws-sec-cred-types.html#access-keys-and-secret-access-keys)
///
/// * `AWS_SESSION_TOKEN`:
///
///   [Session token](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html)
///
/// * `AWS_CREDENTIAL_EXPIRATION`:
///
///   Expiration time in RFC 3339 format (e.g. `1996-12-19T16:39:57-08:00`). If unset, credentials
///   won't expire.
///
/// # Example
///
/// ```rust
/// use futures::future::Future;
/// use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
/// use std::env;
///
/// env::set_var("AWS_ACCESS_KEY_ID", "ANTN35UAENTS5UIAEATD");
/// env::set_var("AWS_SECRET_ACCESS_KEY", "TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur");
/// env::set_var("AWS_SESSION_TOKEN", "DfnGs8Td4rT8r4srxAg6Td4rT8r4srxAg6GtkTir");
///
/// let creds = EnvironmentProvider::default().credentials().await.unwrap();
///
/// assert_eq!(creds.aws_access_key_id(), "ANTN35UAENTS5UIAEATD");
/// assert_eq!(creds.aws_secret_access_key(), "TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur");
/// assert_eq!(creds.token(), &Some("DfnGs8Td4rT8r4srxAg6Td4rT8r4srxAg6GtkTir".to_string()));
/// assert!(creds.expires_at().is_none()); // doesn't expire
///
/// env::set_var("AWS_CREDENTIAL_EXPIRATION", "2018-04-21T01:13:02Z");
/// let creds = EnvironmentProvider::default().credentials().await.unwrap();
/// assert_eq!(creds.expires_at().unwrap().to_rfc3339(), "2018-04-21T01:13:02+00:00");
/// ```
#[derive(Debug, Clone)]
pub struct EnvironmentProvider {
    prefix: String,
}

impl Default for EnvironmentProvider {
    fn default() -> Self {
        EnvironmentProvider {
            prefix: "AWS".to_owned(),
        }
    }
}

impl EnvironmentProvider {
    /// Create an EnvironmentProvider with a non-standard variable prefix.
    ///
    /// ```rust
    /// use std::future::Future;
    /// use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
    /// use std::env;
    ///
    /// env::set_var("MYAPP_ACCESS_KEY_ID", "ANTN35UAENTS5UIAEATD");
    /// env::set_var("MYAPP_SECRET_ACCESS_KEY", "TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur");
    /// env::set_var("MYAPP_SESSION_TOKEN", "DfnGs8Td4rT8r4srxAg6Td4rT8r4srxAg6GtkTir");
    ///
    /// let creds = EnvironmentProvider::with_prefix("MYAPP").credentials().await.unwrap();
    ///
    /// assert_eq!(creds.aws_access_key_id(), "ANTN35UAENTS5UIAEATD");
    /// assert_eq!(creds.aws_secret_access_key(), "TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur");
    /// assert_eq!(creds.token(), &Some("DfnGs8Td4rT8r4srxAg6Td4rT8r4srxAg6GtkTir".to_string()));
    /// assert!(creds.expires_at().is_none()); // doesn't expire
    ///
    /// env::set_var("MYAPP_CREDENTIAL_EXPIRATION", "2018-04-21T01:13:02Z");
    /// let creds = EnvironmentProvider::with_prefix("MYAPP").credentials().await.unwrap();
    /// assert_eq!(creds.expires_at().unwrap().to_rfc3339(), "2018-04-21T01:13:02+00:00");
    /// ```
    pub fn with_prefix(prefix: &str) -> Self {
        EnvironmentProvider {
            prefix: prefix.to_owned(),
        }
    }
}

/// A private trait for building the environment variable names based
/// on a provided prefix. Smallest subset of functionality needed for
/// Credentials building (see `credentials_from_environment` below).
trait EnvironmentVariableProvider {
    fn prefix(&self) -> &str;

    fn access_key_id_var(&self) -> String {
        format!("{}_ACCESS_KEY_ID", self.prefix())
    }

    fn secret_access_key_var(&self) -> String {
        format!("{}_SECRET_ACCESS_KEY", self.prefix())
    }

    fn session_token_var(&self) -> String {
        format!("{}_SESSION_TOKEN", self.prefix())
    }

    fn credential_expiration_var(&self) -> String {
        format!("{}_CREDENTIAL_EXPIRATION", self.prefix())
    }
}

impl EnvironmentVariableProvider for EnvironmentProvider {
    fn prefix(&self) -> &str {
        self.prefix.as_str()
    }
}

#[async_trait]
impl ProvideAwsCredentials for EnvironmentProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let env_key = get_critical_variable(self.access_key_id_var())?;
        let env_secret = get_critical_variable(self.secret_access_key_var())?;
        // Present when using temporary credentials, e.g. on Lambda with IAM roles
        let token = non_empty_env_var(&self.session_token_var());
        // Mimic botocore's behavior, see https://github.com/boto/botocore/pull/1187.
        let var_name = self.credential_expiration_var();
        let expires_at = match non_empty_env_var(&var_name) {
            Some(val) => Some(
                DateTime::<FixedOffset>::parse_from_rfc3339(&val)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(|e| {
                        CredentialsError::new(format!(
                            "Invalid {} in environment '{}': {}",
                            var_name, val, e
                        ))
                    })?,
            ),
            _ => None,
        };
        Ok(AwsCredentials::new(env_key, env_secret, token, expires_at))
    }
}

/// Force an error if we do not see the particular variable name in the env.
fn get_critical_variable(var_name: String) -> Result<String, CredentialsError> {
    non_empty_env_var(&var_name)
        .ok_or_else(|| CredentialsError::new(format!("No (or empty) {} in environment", var_name)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{lock, ENV_MUTEX};
    use chrono::Utc;
    use std::env;

    static AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
    static AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
    static AWS_SESSION_TOKEN: &str = "AWS_SESSION_TOKEN";
    static AWS_CREDENTIAL_EXPIRATION: &str = "AWS_CREDENTIAL_EXPIRATION";

    static E_NO_ACCESS_KEY_ID: &str = "No (or empty) AWS_ACCESS_KEY_ID in environment";
    static E_NO_SECRET_ACCESS_KEY: &str = "No (or empty) AWS_SECRET_ACCESS_KEY in environment";
    static E_INVALID_EXPIRATION: &str = "Invalid AWS_CREDENTIAL_EXPIRATION in environment";

    #[tokio::test]
    async fn get_temporary_credentials_from_env() {
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::set_var(AWS_SESSION_TOKEN, "token");
        let result = EnvironmentProvider::default().credentials().await;
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::remove_var(AWS_SESSION_TOKEN);
        assert!(result.is_ok());
        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "id");
        assert_eq!(creds.aws_secret_access_key(), "secret");
        assert_eq!(creds.token(), &Some("token".to_string()));
    }

    #[tokio::test]
    async fn get_non_temporary_credentials_from_env() {
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider::default().credentials().await;
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        assert!(result.is_ok());
        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "id");
        assert_eq!(creds.aws_secret_access_key(), "secret");
        assert_eq!(creds.token(), &None);
    }

    #[tokio::test]
    async fn environment_provider_missing_key_id() {
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider::default().credentials().await;
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(E_NO_ACCESS_KEY_ID))
        );
    }

    #[tokio::test]
    async fn environment_provider_missing_secret() {
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider::default().credentials().await;
        env::remove_var(AWS_ACCESS_KEY_ID);
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(E_NO_SECRET_ACCESS_KEY))
        );
    }

    #[tokio::test]
    async fn environment_provider_missing_credentials() {
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider::default().credentials().await;
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(E_NO_ACCESS_KEY_ID))
        );
    }

    #[tokio::test]
    async fn environment_provider_bad_expiration() {
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::set_var(AWS_SESSION_TOKEN, "token");
        env::set_var(AWS_CREDENTIAL_EXPIRATION, "lore ipsum");
        let result = EnvironmentProvider::default().credentials().await;
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::remove_var(AWS_SESSION_TOKEN);
        env::remove_var(AWS_CREDENTIAL_EXPIRATION);
        assert!(result.is_err());
        assert!(match &result.err() {
            &Some(CredentialsError { ref message }) => message.starts_with(E_INVALID_EXPIRATION),
            _ => false,
        });
    }

    #[tokio::test]
    async fn get_temporary_credentials_with_expiration_from_env() {
        let _guard = lock(&ENV_MUTEX);
        let now = Utc::now();
        let now_str = now.to_rfc3339();
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::set_var(AWS_SESSION_TOKEN, "token");
        env::set_var(AWS_CREDENTIAL_EXPIRATION, now_str);
        let result = EnvironmentProvider::default().credentials().await;
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::remove_var(AWS_SESSION_TOKEN);
        env::remove_var(AWS_CREDENTIAL_EXPIRATION);
        assert!(result.is_ok());
        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "id");
        assert_eq!(creds.aws_secret_access_key(), "secret");
        assert_eq!(creds.token(), &Some("token".to_string()));
        assert_eq!(creds.expires_at(), &Some(now));
    }

    #[tokio::test]
    async fn regression_test_rfc_3339_compat() {
        let _guard = lock(&ENV_MUTEX);
        // RFC 3339 expiration times with lower case 't' could not be parsed by earlier
        // implementations.
        env::set_var(AWS_CREDENTIAL_EXPIRATION, "1996-12-19t16:39:57-08:00");
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        let result = EnvironmentProvider::default().credentials().await;
        env::remove_var(AWS_CREDENTIAL_EXPIRATION);
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);

        assert_eq!(
            result.unwrap().expires_at().unwrap().to_rfc3339(),
            "1996-12-20T00:39:57+00:00"
        );
    }

    #[tokio::test]
    async fn alternative_prefix() {
        // NOTE: not strictly neccessary here, since we are using a non-standard
        // prefix, so we shouldn't collide with the other env interactions in
        // the other tests.
        let _guard = lock(&ENV_MUTEX);

        let now = Utc::now();
        let now_str = now.to_rfc3339();
        env::set_var("MYAPP_ACCESS_KEY_ID", "id");
        env::set_var("MYAPP_SECRET_ACCESS_KEY", "secret");
        env::set_var("MYAPP_SESSION_TOKEN", "token");
        env::set_var("MYAPP_CREDENTIAL_EXPIRATION", now_str);
        let result = EnvironmentProvider::with_prefix("MYAPP")
            .credentials()
            .await;
        env::remove_var("MYAPP_ACCESS_KEY_ID");
        env::remove_var("MYAPP_SECRET_ACCESS_KEY");
        env::remove_var("MYAPP_SESSION_TOKEN");
        env::remove_var("MYAPP_CREDENTIAL_EXPIRATION");
        assert!(result.is_ok());
        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "id");
        assert_eq!(creds.aws_secret_access_key(), "secret");
        assert_eq!(creds.token(), &Some("token".to_string()));
        assert_eq!(creds.expires_at(), &Some(now));
    }
}
