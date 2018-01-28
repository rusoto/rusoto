//! The Credentials Provider to read from Environment Variables.

use std::env::var as env_var;

use futures::{Future, Poll};
use futures::future::{FutureResult, result};

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, in_ten_minutes};

/// Provides AWS credentials from environment variables.
#[derive(Debug)]
pub struct EnvironmentProvider;

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

/// Grabs the Credentials from the environment. These credentials are good for 10 minutes.
fn credentials_from_environment() -> Result<AwsCredentials, CredentialsError> {
    let env_key = match env_var("AWS_ACCESS_KEY_ID") {
        Ok(val) => val,
        Err(_) => return Err(CredentialsError::new("No AWS_ACCESS_KEY_ID in environment")),
    };
    let env_secret = match env_var("AWS_SECRET_ACCESS_KEY") {
        Ok(val) => val,
        Err(_) => {
            return Err(CredentialsError::new(
                "No AWS_SECRET_ACCESS_KEY in environment",
            ))
        }
    };

    if env_key.is_empty() || env_secret.is_empty() {
        return Err(CredentialsError::new(
            "Couldn't find either AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY or both in environment.",
        ));
    }

    // Present when using temporary credentials, e.g. on Lambda with IAM roles
    let token = match env_var("AWS_SESSION_TOKEN") {
        Ok(val) => if val.is_empty() { None } else { Some(val) },
        Err(_) => None,
    };

    Ok(AwsCredentials::new(
        env_key,
        env_secret,
        token,
        in_ten_minutes(),
    ))
}
