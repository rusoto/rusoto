use std::env;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, in_ten_minutes};

/// Provides AWS credentials from environment variables.
#[derive(Debug)]
pub struct EnvironmentProvider;

impl ProvideAwsCredentials for EnvironmentProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        credentials_from_environment()
    }
}

fn credentials_from_environment() -> Result<AwsCredentials, CredentialsError> {
    let env_key = match env::var("AWS_ACCESS_KEY_ID") {
        Ok(val) => val,
        Err(_) => return Err(CredentialsError::new("No AWS_ACCESS_KEY_ID in environment"))
    };
    let env_secret = match env::var("AWS_SECRET_ACCESS_KEY") {
        Ok(val) => val,
        Err(_) => return Err(CredentialsError::new("No AWS_SECRET_ACCESS_KEY in environment"))
    };

    if env_key.is_empty() || env_secret.is_empty() {
        return Err(CredentialsError::new("Couldn't find either AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY or both in environment."));
    }

    // Present when using temporary credentials, e.g. on Lambda with IAM roles
    let token = match env::var("AWS_SESSION_TOKEN") {
        Ok(val) => {
            if val.is_empty() {
                None
            } else {
                Some(val)
            }
        }
        Err(_) => None,
    };

    Ok(AwsCredentials::new(env_key, env_secret, token, in_ten_minutes()))
}
