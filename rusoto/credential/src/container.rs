//! The Credentials provider to read from a task's IAM Role.

use reqwest;
use std::env::var as env_var;
use std::io::Read;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, parse_credentials_from_aws_service};

const AWS_CREDENTIALS_PROVIDER_IP: &'static str = "169.254.170.2";

/// Provides AWS credentials from a task's IAM role.
#[derive(Debug)]
pub struct ContainerProvider;

impl ProvideAwsCredentials for ContainerProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        credentials_from_container()
    }
}

/// Grabs the Credentials from the AWS Container Credentials Provider. (169.254.170.2).
fn credentials_from_container() -> Result<AwsCredentials, CredentialsError> {
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
    let response = reqwest::get(&address);

    match response {
        Ok(mut resp) => {
            if resp.status().is_success() {
                let mut body = String::new();
                if resp.read_to_string(&mut body).is_err() {
                    return Err(CredentialsError::new(
                        "Didn't get a parsable response body from the credentials provider",
                    ));
                }
                parse_credentials_from_aws_service(&body)
            } else {
                return Err(CredentialsError::new(format!(
                    "Couldn't connect to credentials provider: {}",
                    resp.status()
                )));
            }
        }
        Err(err) => {
            return Err(CredentialsError::new(
                format!("Couldn't connect to credentials provider: {}", err),
            ))
        }
    }
}
