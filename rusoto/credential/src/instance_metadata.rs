//! The Credentials Provider for an AWS Resource's IAM Role.

use hyper::Uri;
use std::time::Duration as StdDuration;
use tokio_core::reactor::Core;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, ProvideTimeoutableAwsCredentials,
     make_request, parse_credentials_from_aws_service};

const AWS_CREDENTIALS_PROVIDER_IP: &'static str = "169.254.169.254";
const AWS_CREDENTIALS_PROVIDER_PATH: &'static str = "latest/meta-data/iam/security-credentials";

/// Provides AWS credentials from a resource's IAM role.
#[derive(Debug)]
pub struct InstanceMetadataProvider;

impl ProvideAwsCredentials for InstanceMetadataProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        get_credentials_from_role(None)
    }
}

impl ProvideTimeoutableAwsCredentials for InstanceMetadataProvider {
    fn credentials_with_timeout(
        &self,
        timeout: StdDuration,
    ) -> Result<AwsCredentials, CredentialsError> {
        get_credentials_from_role(Some(timeout))
    }
}

/// Gets the role name to get credentials for using the IAM Metadata Service (169.254.169.254).
fn get_role_name(timeout: Option<StdDuration>) -> Result<String, CredentialsError> {
    let role_name_address = format!(
        "http://{}/{}/",
        AWS_CREDENTIALS_PROVIDER_IP,
        AWS_CREDENTIALS_PROVIDER_PATH
    );
    let core = try!(Core::new());
    let uri = try!(role_name_address.parse::<Uri>());
    match make_request(core, uri, timeout) {
        Ok(resp) => Ok(resp),
        Err(err) => {
            return Err(CredentialsError::new(
                format!("Couldn't connect to credentials provider: {}", err),
            ))
        }
    }
}

/// Gets the credentials for an EC2 Instances IAM Role.
fn get_credentials_from_role(
    timeout: Option<StdDuration>,
) -> Result<AwsCredentials, CredentialsError> {
    let role_name = try!(get_role_name(timeout.clone()));
    let credentials_provider_url = format!(
        "http://{}/{}/{}",
        AWS_CREDENTIALS_PROVIDER_IP,
        AWS_CREDENTIALS_PROVIDER_PATH,
        role_name
    );

    let core = try!(Core::new());
    let uri = try!(credentials_provider_url.parse::<Uri>());
    match make_request(core, uri, timeout) {
        Ok(resp) => parse_credentials_from_aws_service(&resp),
        Err(err) => {
            return Err(CredentialsError::new(
                format!("Couldn't connect to credentials provider: {}", err),
            ))
        }
    }
}
