//! The Credentials provider to read from a task's IAM Role.

use std::error::Error;
use futures::{Async, Future, Poll};
use futures::future::err;
use hyper::{Client, Uri, Request, Method};
use hyper::header::Authorization;
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials,
     make_request, parse_credentials_from_aws_service, non_empty_env_var};

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
/// As described in Amazon's
/// [ECS developers guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html),
/// Containers started as part of Tasks using IAM Roles for Tasks will be provided with a relative
/// URL stored in the environment variable ```AWS_CONTAINER_CREDENTIALS_RELATIVE_URI```, which will
/// be used to obtain the AWS credentials. If that environment variable is not set, rusoto will use
/// the URL set in environment variable ```AWS_CONTAINER_CREDENTIALS_FULL_URI``` to obtain AWS
/// credentials and will (optionally) also set the ```Authorization``` header to the value of
/// environment variable ```AWS_CONTAINER_AUTHORIZATION_TOKEN```.
///
#[derive(Clone, Debug)]
pub struct ContainerProvider {
    client: Client<HttpConnector>
}

impl ContainerProvider {
    /// Create a new provider with the given handle.
    pub fn new(handle: &Handle) -> Self {
        let client = Client::configure().build(handle);
        ContainerProvider { client: client }
    }
}

/// Future returned from `ContainerProvider`.
pub struct ContainerProviderFuture {
    inner: Box<Future<Item=String, Error=CredentialsError>>
}

impl Future for ContainerProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let resp = try_ready!(self.inner.poll());
        let creds = parse_credentials_from_aws_service(&resp)?;
        Ok(Async::Ready(creds))
    }
}

impl ProvideAwsCredentials for ContainerProvider {
    type Future = ContainerProviderFuture;

    fn credentials(&self) -> Self::Future {
        let inner = match credentials_from_container(&self.client) {
            Ok(future) => future,
            Err(e) => Box::new(err(e))
        };
        ContainerProviderFuture { inner: inner }
    }
}

/// Grabs the Credentials from the AWS Container Credentials Provider. (169.254.170.2).
fn credentials_from_container(client: &Client<HttpConnector>) -> Result<Box<Future<Item=String, Error=CredentialsError>>, CredentialsError> {
    Ok(make_request(client, request_from_env_vars()?))
}

fn request_from_env_vars() -> Result<Request, CredentialsError> {
    let relative_uri = non_empty_env_var(AWS_CONTAINER_CREDENTIALS_RELATIVE_URI).map(
        |path| format!("http://{}{}", AWS_CREDENTIALS_PROVIDER_IP, path)
    );
    match relative_uri {
        Some(ref uri) => new_request(uri, AWS_CONTAINER_CREDENTIALS_RELATIVE_URI),
        None => match non_empty_env_var(AWS_CONTAINER_CREDENTIALS_FULL_URI) {
            Some(ref uri) => {
                let mut request = new_request(uri, AWS_CONTAINER_CREDENTIALS_FULL_URI)?;
                let token = non_empty_env_var(AWS_CONTAINER_AUTHORIZATION_TOKEN);
                token.map(|t| request.headers_mut().set(Authorization(t)));
                Ok(request)
            },
            None => Err(CredentialsError::new(
                format!("Neither environment variable '{}' nor '{}' is set", AWS_CONTAINER_CREDENTIALS_FULL_URI, AWS_CONTAINER_CREDENTIALS_RELATIVE_URI),
            ))
        }
    }
}

fn new_request(uri: &str, env_var_name: &str) -> Result<Request, CredentialsError> {
    uri.parse::<Uri>().map(
            |uri| Request::new(Method::Get, uri)
        ).or_else(|error| Err(
             CredentialsError::new(
                 format!("Error while parsing URI '{}' derived from environment variable '{}': {}",
                         uri, env_var_name, error.description())
             )
        )
    )
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::{Mutex, MutexGuard};
    use hyper::header::Authorization;
    use hyper::header::Bearer;
    use super::*;

    // cargo runs tests in parallel, which leads to race conditions when changing
    // environment variables. Therefore we use a global mutex for all tests which
    // rely on environment variables.
    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    // As failed (panic) tests will poisen the global mutex, we use a helper which
    // recovers from poisoned mutex.
    fn lock<'a, T>(mutex: &'a Mutex<T>) -> MutexGuard<'a, T> {
        match mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

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
        assert_eq!(request.headers().has::<Authorization<Bearer>>(), false);
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
        assert_eq!(request.headers().has::<Authorization<Bearer>>(), true);
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
        assert_eq!(request.headers().has::<Authorization<Bearer>>(), false);
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
        assert_eq!(request.headers().has::<Authorization<Bearer>>(), false);
    }
}
