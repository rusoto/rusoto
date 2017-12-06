//! The Credentials provider to read from a task's IAM Role.

use hyper::Uri;
use std::env::var as env_var;
use std::time::Duration as StdDuration;
use tokio_core::reactor::Core;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, ProvideTimeoutableAwsCredentials,
     make_request, parse_credentials_from_aws_service};

const AWS_CREDENTIALS_PROVIDER_IP: &'static str = "169.254.170.2";

/// Provides AWS credentials from a task's IAM role.
#[derive(Debug)]
pub struct ContainerProvider;

impl ProvideAwsCredentials for ContainerProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        credentials_from_container(None)
    }
}

impl ProvideTimeoutableAwsCredentials for ContainerProvider {
    fn credentials_with_timeout(
        &self,
        timeout: StdDuration,
    ) -> Result<AwsCredentials, CredentialsError> {
        credentials_from_container(Some(timeout))
    }
}

/// Grabs the Credentials from the AWS Container Credentials Provider. (169.254.170.2).
fn credentials_from_container(
    timeout: Option<StdDuration>,
) -> Result<AwsCredentials, CredentialsError> {
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
    let core = try!(Core::new());
    let uri = try!(address.parse::<Uri>());
    match make_request(core, uri, timeout) {
        Ok(resp) => parse_credentials_from_aws_service(&resp),
        Err(err) => {
            return Err(CredentialsError::new(
                format!("Couldn't connect to credentials provider: {}", err),
            ))
        }
    }
}
