//! The Credentials Provider to read from Environment Variables.

use chrono::{FixedOffset, Utc, DateTime};

use futures::{Future, Poll};
use futures::future::{FutureResult, result};

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, non_empty_env_var};

const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
const AWS_SESSION_TOKEN: &str = "AWS_SESSION_TOKEN";
const AWS_CREDENTIAL_EXPIRATION: &str = "AWS_CREDENTIAL_EXPIRATION";

const E_NO_ACCESS_KEY_ID: &str = "No (or empty) AWS_ACCESS_KEY_ID in environment";
const E_NO_SECRET_ACCESS_KEY: &str = "No (or empty) AWS_SECRET_ACCESS_KEY in environment";
const E_INVALID_EXPIRATION: &str = "Invalid AWS_CREDENTIAL_EXPIRATION in environment";

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
/// ```
/// # extern crate futures;
/// # extern crate rusoto_credential;
/// #
/// # fn main() {
/// use futures::future::Future;
/// use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
/// use std::env;
///
/// env::set_var("AWS_ACCESS_KEY_ID", "ANTN35UAENTS5UIAEATD");
/// env::set_var("AWS_SECRET_ACCESS_KEY", "TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur");
/// env::set_var("AWS_SESSION_TOKEN", "DfnGs8Td4rT8r4srxAg6Td4rT8r4srxAg6GtkTir");
///
/// let creds = EnvironmentProvider.credentials().wait().unwrap();
///
/// assert_eq!(creds.aws_access_key_id(), "ANTN35UAENTS5UIAEATD");
/// assert_eq!(creds.aws_secret_access_key(), "TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur");
/// assert_eq!(creds.token(), &Some("DfnGs8Td4rT8r4srxAg6Td4rT8r4srxAg6GtkTir".to_string()));
/// assert!(creds.expires_at().is_none()); // doesn't expire
///
/// env::set_var("AWS_CREDENTIAL_EXPIRATION", "2018-04-21T01:13:02Z");
/// let creds = EnvironmentProvider.credentials().wait().unwrap();
/// assert_eq!(creds.expires_at().unwrap().to_rfc3339(), "2018-04-21T01:13:02+00:00");
/// # }
/// ```
#[derive(Debug)]
pub struct EnvironmentProvider;

/// Provides AWS credentials from environment variables as a Future.
pub struct EnvironmentProviderFuture {
    inner: FutureResult<AwsCredentials, CredentialsError>
}

impl Future for EnvironmentProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

impl ProvideAwsCredentials for EnvironmentProvider {
    type Future = EnvironmentProviderFuture;

    fn credentials(&self) -> Self::Future {
        EnvironmentProviderFuture {
            inner: result(credentials_from_environment())
        }
    }
}

/// Grabs the Credentials from the environment.
fn credentials_from_environment() -> Result<AwsCredentials, CredentialsError> {
    let env_key = match non_empty_env_var(AWS_ACCESS_KEY_ID) {
        Some(val) => val,
        None => return Err(CredentialsError::new(E_NO_ACCESS_KEY_ID)),
    };
    let env_secret = match non_empty_env_var(AWS_SECRET_ACCESS_KEY) {
        Some(val) => val,
        None => {
            return Err(CredentialsError::new(E_NO_SECRET_ACCESS_KEY))
        }
    };
    // Present when using temporary credentials, e.g. on Lambda with IAM roles
    let token = non_empty_env_var(AWS_SESSION_TOKEN);
    // Mimik botocore's behavior, see https://github.com/boto/botocore/pull/1187.
    let expires_at = match non_empty_env_var(AWS_CREDENTIAL_EXPIRATION) {
        Some(val) => match DateTime::<FixedOffset>::parse_from_rfc3339(&val).map(|dt| dt.with_timezone(&Utc)) {
            Ok(e) => Some(e),
            Err(e) => return Err(CredentialsError::new(format!("{} '{}': {}", E_INVALID_EXPIRATION, val, e)))
        },
        _ => None,
    };
    Ok(AwsCredentials::new(
        env_key,
        env_secret,
        token,
        expires_at,
    ))
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::{Mutex, MutexGuard};
    use chrono::Utc;
    use super::*;

    // cargo runs tests in parallel, which leads to race conditions when changing
    // environment variables. Therefore we use a global mutex for all tests which
    // rely on environment variables.
    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    // As failed (panic) tests will poisen the global mutex, we use a helper which
    // recovers from poisoned mutex.
    fn lock<'a, T>(mutex: &'a Mutex<T>) -> MutexGuard<'a,T> {
        match mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[test]
    fn get_temporary_credentials_from_env() {
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::set_var(AWS_SESSION_TOKEN, "token");
        let result = EnvironmentProvider.credentials().wait();
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::remove_var(AWS_SESSION_TOKEN);
        assert!(result.is_ok());
        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "id");
        assert_eq!(creds.aws_secret_access_key(), "secret");
        assert_eq!(creds.token(), &Some("token".to_string()));
    }

    #[test]
    fn get_non_temporary_credentials_from_env() {
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider.credentials().wait();
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        assert!(result.is_ok());
        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "id");
        assert_eq!(creds.aws_secret_access_key(), "secret");
        assert_eq!(creds.token(), &None);
    }

    #[test]
    fn environment_provider_missing_key_id() {
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider.credentials().wait();
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(E_NO_ACCESS_KEY_ID))
        );
    }

    #[test]
    fn environment_provider_missing_secret() {
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider.credentials().wait();
        env::remove_var(AWS_ACCESS_KEY_ID);
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(E_NO_SECRET_ACCESS_KEY))
        );
    }

    #[test]
    fn environment_provider_missing_credentials() {
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SESSION_TOKEN);
        let result = EnvironmentProvider.credentials().wait();
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(E_NO_ACCESS_KEY_ID))
        );
    }

    #[test]
    fn environment_provider_bad_expiration() {
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::set_var(AWS_SESSION_TOKEN, "token");
        env::set_var(AWS_CREDENTIAL_EXPIRATION, "lore ipsum");
        let result = EnvironmentProvider.credentials().wait();
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);
        env::remove_var(AWS_SESSION_TOKEN);
        env::remove_var(AWS_CREDENTIAL_EXPIRATION);
        assert!(result.is_err());
        assert!(match &result.err() {
            &Some(CredentialsError{ref message}) => message.starts_with(E_INVALID_EXPIRATION),
            _ => false,
        });
    }

    #[test]
    fn get_temporary_credentials_with_expiration_from_env() {
        let _guard = lock(&ENV_MUTEX);
        let now = Utc::now();
        let now_str = now.to_rfc3339();
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        env::set_var(AWS_SESSION_TOKEN, "token");
        env::set_var(AWS_CREDENTIAL_EXPIRATION, now_str);
        let result = EnvironmentProvider.credentials().wait();
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

    #[test]
    fn regression_test_rfc_3339_compat() {
        let _guard = lock(&ENV_MUTEX);
        // RFC 3339 expiration times with lower case 't' could not be parsed by earlier
        // implementations.
        env::set_var(AWS_CREDENTIAL_EXPIRATION, "1996-12-19t16:39:57-08:00");
        env::set_var(AWS_ACCESS_KEY_ID, "id");
        env::set_var(AWS_SECRET_ACCESS_KEY, "secret");
        let result = EnvironmentProvider.credentials().wait();
        env::remove_var(AWS_CREDENTIAL_EXPIRATION);
        env::remove_var(AWS_ACCESS_KEY_ID);
        env::remove_var(AWS_SECRET_ACCESS_KEY);

        assert_eq!(result.unwrap().expires_at().unwrap().to_rfc3339(), "1996-12-20T00:39:57+00:00");
    }
}