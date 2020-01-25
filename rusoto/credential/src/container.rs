//! The Credentials provider to read from a task's IAM Role.

use std::time::Duration;

use async_trait::async_trait;
use hyper::{Body, Request};

use crate::request::HttpClient;
use crate::{
    non_empty_env_var, parse_credentials_from_aws_service, AwsCredentials, CredentialsError,
    ProvideAwsCredentials,
};

// The following constants are documented in AWS' ECS developers guide,
// see https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html.
const AWS_CREDENTIALS_PROVIDER_IP: &str = "169.254.170.2";
const AWS_CONTAINER_CREDENTIALS_RELATIVE_URI: &str = "AWS_CONTAINER_CREDENTIALS_RELATIVE_URI";
// I have not yet found any official documentation from AWS concerning the following two
// environment variables, but they are used by the Java, Go, JavaScript and the Python SDKs.
const AWS_CONTAINER_CREDENTIALS_FULL_URI: &str = "AWS_CONTAINER_CREDENTIALS_FULL_URI";
const AWS_CONTAINER_AUTHORIZATION_TOKEN: &str = "AWS_CONTAINER_AUTHORIZATION_TOKEN";

/// Provides AWS credentials from a task's IAM role.
///
/// The provider has a default timeout of 30 seconds. While it should work well for most setups,
/// you can change the timeout using the `set_timeout` method.
///
/// As described in Amazon's
/// [ECS developers guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html),
/// Containers started as part of Tasks using IAM Roles for Tasks will be provided with a relative
/// URL stored in the environment variable ```AWS_CONTAINER_CREDENTIALS_RELATIVE_URI```, which will
/// be used to obtain the AWS credentials. If that environment variable is not set, rusoto will use
/// the URL set in environment variable ```AWS_CONTAINER_CREDENTIALS_FULL_URI``` to obtain AWS
/// credentials and will (optionally) also set the ```Authorization``` header to the value of
/// environment variable ```AWS_CONTAINER_AUTHORIZATION_TOKEN```.
///
/// # Example
///
/// ```rust
/// extern crate rusoto_credential;
///
/// use std::time::Duration;
///
/// use rusoto_credential::ContainerProvider;
///
/// fn main() {
///   let mut provider = ContainerProvider::new();
///   // you can overwrite the default timeout like this:
///   provider.set_timeout(Duration::from_secs(60));
///
///   // ...
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ContainerProvider {
    client: HttpClient,
    timeout: Duration,
}

impl ContainerProvider {
    /// Create a new provider with the given handle.
    pub fn new() -> Self {
        ContainerProvider {
            client: HttpClient::new(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the timeout on the provider to the specified duration.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}

/// Default: create a new provider with the given handle.
impl Default for ContainerProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProvideAwsCredentials for ContainerProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let req = request_from_env_vars().map_err(|err| CredentialsError {
            message: format!(
                "Could not get request from environment: {}",
                err.to_string()
            ),
        })?;
        let resp = self
            .client
            .request(req, self.timeout)
            .await
            .map_err(|err| CredentialsError {
                message: format!(
                    "Could not get credentials from container: {}",
                    err.to_string()
                ),
            })?;
        parse_credentials_from_aws_service(&resp)
    }
}

fn request_from_env_vars() -> Result<Request<Body>, CredentialsError> {
    let relative_uri = non_empty_env_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI)
        .map(|path| format!("http://{}{}", AWS_CREDENTIALS_PROVIDER_IP, path));
    match relative_uri {
        Some(ref uri) => new_request(uri, AWS_CONTAINER_CREDENTIALS_RELATIVE_URI),
        None => match non_empty_env_var(AWS_CONTAINER_CREDENTIALS_FULL_URI) {
            Some(ref uri) => {
                let mut request = new_request(uri, AWS_CONTAINER_CREDENTIALS_FULL_URI)?;
                if let Some(token) = non_empty_env_var(AWS_CONTAINER_AUTHORIZATION_TOKEN) {
                    match token.parse() {
                        Ok(parsed_token) => {
                            request.headers_mut().insert("authorization", parsed_token);
                        }
                        Err(err) => {
                            return Err(CredentialsError::new(format!(
                                "failed to parse token: {}",
                                err
                            )));
                        }
                    }
                }
                Ok(request)
            }
            None => Err(CredentialsError::new(format!(
                "Neither environment variable '{}' nor '{}' is set",
                AWS_CONTAINER_CREDENTIALS_FULL_URI, AWS_CONTAINER_CREDENTIALS_RELATIVE_URI
            ))),
        },
    }
}

fn new_request(uri: &str, env_var_name: &str) -> Result<Request<Body>, CredentialsError> {
    Request::get(uri).body(Body::empty()).map_err(|error| {
        CredentialsError::new(format!(
            "Error while parsing URI '{}' derived from environment variable '{}': {}",
            uri, env_var_name, error
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{lock, ENV_MUTEX};
    use std::env;

    #[test]
    fn request_from_relative_uri() {
        let path = "/xxx";
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI, path);
        env::set_var(AWS_CONTAINER_CREDENTIALS_FULL_URI, "dummy");
        env::set_var(AWS_CONTAINER_AUTHORIZATION_TOKEN, "dummy");
        let result = request_from_env_vars();
        env::remove_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI);
        env::remove_var(AWS_CONTAINER_CREDENTIALS_FULL_URI);
        env::remove_var(AWS_CONTAINER_AUTHORIZATION_TOKEN);
        assert!(result.is_ok());
        let request = result.ok().unwrap();
        assert_eq!(request.uri().path(), path);
        assert_eq!(request.headers().contains_key("authorization"), false);
    }

    #[test]
    fn error_from_missing_env_vars() {
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI);
        env::remove_var(AWS_CONTAINER_CREDENTIALS_FULL_URI);
        let result = request_from_env_vars();
        assert!(result.is_err());
    }

    #[test]
    fn error_from_empty_env_vars() {
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI, "");
        env::set_var(AWS_CONTAINER_CREDENTIALS_FULL_URI, "");
        env::set_var(AWS_CONTAINER_AUTHORIZATION_TOKEN, "");
        let result = request_from_env_vars();
        env::remove_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI);
        env::remove_var(AWS_CONTAINER_CREDENTIALS_FULL_URI);
        env::remove_var(AWS_CONTAINER_AUTHORIZATION_TOKEN);
        assert!(result.is_err());
    }

    #[test]
    fn request_from_full_uri_with_token() {
        let url = "http://localhost/xxx";
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI);
        env::set_var(AWS_CONTAINER_CREDENTIALS_FULL_URI, url);
        env::set_var(AWS_CONTAINER_AUTHORIZATION_TOKEN, "dummy");
        let result = request_from_env_vars();
        env::remove_var(AWS_CONTAINER_CREDENTIALS_FULL_URI);
        env::remove_var(AWS_CONTAINER_AUTHORIZATION_TOKEN);
        assert!(result.is_ok());
        let request = result.ok().unwrap();
        assert_eq!(request.uri().to_string(), url);
        assert_eq!(request.headers().contains_key("authorization"), true);
    }

    #[test]
    fn request_from_full_uri_without_token() {
        let url = "http://localhost/xxx";
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI);
        env::set_var(AWS_CONTAINER_CREDENTIALS_FULL_URI, url);
        env::remove_var(AWS_CONTAINER_AUTHORIZATION_TOKEN);
        let result = request_from_env_vars();
        env::remove_var(AWS_CONTAINER_CREDENTIALS_FULL_URI);
        assert!(result.is_ok());
        let request = result.ok().unwrap();
        assert_eq!(request.uri().to_string(), url);
        assert_eq!(request.headers().contains_key("authorization"), false);
    }

    #[test]
    fn request_from_full_uri_with_empty_token() {
        let url = "http://localhost/xxx";
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI);
        env::set_var(AWS_CONTAINER_CREDENTIALS_FULL_URI, url);
        env::set_var(AWS_CONTAINER_AUTHORIZATION_TOKEN, "");
        let result = request_from_env_vars();
        env::remove_var(AWS_CONTAINER_CREDENTIALS_FULL_URI);
        env::remove_var(AWS_CONTAINER_AUTHORIZATION_TOKEN);
        assert!(result.is_ok());
        let request = result.ok().unwrap();
        assert_eq!(request.uri().to_string(), url);
        assert_eq!(request.headers().contains_key("authorization"), false);
    }
}
