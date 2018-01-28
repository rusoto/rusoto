//! The Credentials provider to read from a task's IAM Role.

use std::env::var as env_var;

use futures::{Async, Future, Poll};
use futures::future::err;
use hyper::{Client, Uri};
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials,
     make_request, parse_credentials_from_aws_service};

const AWS_CREDENTIALS_PROVIDER_IP: &'static str = "169.254.170.2";

/// Provides AWS credentials from a task's IAM role.
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
    let aws_container_credentials_relative_uri =
        match env_var("AWS_CONTAINER_CREDENTIALS_RELATIVE_URI") {
            Ok(v) => v,
            Err(_) => {
                return Err(CredentialsError::new(
                    "No AWS_CONTAINER_CREDENTIALS_RELATIVE_URI in environment",
                ))
            }
        };
    let address: String = format!(
        "http://{}{}",
        AWS_CREDENTIALS_PROVIDER_IP,
        aws_container_credentials_relative_uri
    );
    let uri = try!(address.parse::<Uri>());
    Ok(make_request(client, uri))
}
