//! The Credentials Provider for an AWS Resource's IAM Role.

use reqwest;
use retry::retry_exponentially;
use std::io::Read;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, parse_credentials_from_aws_service};

const AWS_CREDENTIALS_PROVIDER_IP: &'static str = "169.254.169.254";
const AWS_CREDENTIALS_PROVIDER_PATH: &'static str = "latest/meta-data/iam/security-credentials";

/// Provides AWS credentials from a resource's IAM role.
#[derive(Debug)]
pub struct InstanceMetadataProvider;

impl ProvideAwsCredentials for InstanceMetadataProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        get_credentials_from_role()
    }
}

/// Gets the role name to get credentials for using the IAM Metadata Service (169.254.169.254).
fn get_role_name() -> Result<String, CredentialsError> {
    let role_name_address = format!("http://{}/{}", AWS_CREDENTIALS_PROVIDER_IP, AWS_CREDENTIALS_PROVIDER_PATH);
    let mut response =
        match retry_exponentially(5, 7f64, || { reqwest::get(&role_name_address) },
        |response_attempt| {
            match response_attempt.as_ref() {
                Ok(response_returned) => response_returned.status().is_success(),
                Err(_) => false,
            }
        })
    {
        Ok(response_from_try) => {
            match response_from_try {
                Ok(resp) => resp,
                Err(err) => return Err(CredentialsError::new(format!("Couldn't connect to credentials role provider: {}", err))),
            }
        }
        Err(error) => return Err(CredentialsError::new(format!("Couldn't connect to credentials role provider: {}", error))),
    };

    let mut role_name = String::new();
    if response.read_to_string(&mut role_name).is_err() {
        return Err(CredentialsError::new("Didn't get a parsable role name from metadata service"));
    }

    Ok(role_name)
}

/// Gets the credentials for an EC2 Instances IAM Role.
fn get_credentials_from_role() -> Result<AwsCredentials, CredentialsError> {
    let role_name = try!(get_role_name());
    let credentials_provider_url = format!("http://{}/{}/{}", AWS_CREDENTIALS_PROVIDER_IP, AWS_CREDENTIALS_PROVIDER_PATH, role_name);
    let mut response =
        match retry_exponentially(5, 7f64, || { reqwest::get(&credentials_provider_url) },
        |response_attempt| {
            match response_attempt.as_ref() {
                Ok(response_returned) => response_returned.status().is_success(),
                Err(_) => false,
            }
        })
    {
        Ok(response_from_try) => {
            match response_from_try {
                Ok(resp) => resp,
                Err(err) => return Err(CredentialsError::new(format!("Couldn't connect to credentials provider: {}", err))),
            }
        }
        Err(error) => return Err(CredentialsError::new(format!("Couldn't connect to credentials provider: {}", error))),
    };

    let mut credentials_response = String::new();
    if response.read_to_string(&mut credentials_response).is_err() {
        return Err(CredentialsError::new("Had issues with reading iam role response: {}"));
    }

    parse_credentials_from_aws_service(&credentials_response)
}
