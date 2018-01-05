//! The Credentials Provider for an AWS Resource's IAM Role.

use futures::{Future, Poll};
use futures::future::{FutureResult, result};
use hyper::{Client, Uri};
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials,
     make_request, parse_credentials_from_aws_service};

const AWS_CREDENTIALS_PROVIDER_IP: &'static str = "169.254.169.254";
const AWS_CREDENTIALS_PROVIDER_PATH: &'static str = "latest/meta-data/iam/security-credentials";

/// Provides AWS credentials from a resource's IAM role.
#[derive(Clone, Debug)]
pub struct InstanceMetadataProvider {
    client: Client<HttpConnector>
}

impl InstanceMetadataProvider {
    /// Create a new provider with the given handle.
    pub fn new(handle: &Handle) -> Self {
        let client = Client::configure().build(handle);
        InstanceMetadataProvider { client: client }
    }
}

enum InstanceMetadataFutureState {
    Start,
    GetRoleName(Box<Future<Item=String, Error=CredentialsError>>),
    GetCredentialsFromRole(Box<Future<Item=String, Error=CredentialsError>>),
    Done(FutureResult<AwsCredentials, CredentialsError>)
}

/// Future returned from `InstanceMetadataProvider`.
pub struct InstanceMetadataProviderFuture {
    state: InstanceMetadataFutureState,
    client: Client<HttpConnector>
}

impl Future for InstanceMetadataProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let new_state = match self.state {
            InstanceMetadataFutureState::Start => {
                let new_future = get_role_name(&self.client)?;
                InstanceMetadataFutureState::GetRoleName(new_future)
            }
            InstanceMetadataFutureState::GetRoleName(ref mut future) => {
                let role_name = try_ready!(future.poll());
                let new_future = get_credentials_from_role(&self.client, &role_name)?;
                InstanceMetadataFutureState::GetCredentialsFromRole(new_future)
            },
            InstanceMetadataFutureState::GetCredentialsFromRole(ref mut future) => {
                let cred_str = try_ready!(future.poll());
                let new_future = result(parse_credentials_from_aws_service(&cred_str));
                InstanceMetadataFutureState::Done(new_future)
            },
            InstanceMetadataFutureState::Done(ref mut future) => {
                return future.poll();
          
            }
        };
        self.state = new_state;
        self.poll()
    }
}

impl ProvideAwsCredentials for InstanceMetadataProvider {
    type Future = InstanceMetadataProviderFuture;

    fn credentials(&self) -> Self::Future {
        InstanceMetadataProviderFuture {
            state: InstanceMetadataFutureState::Start,
            client: self.client.clone()
        }
    }
}

/// Gets the role name to get credentials for using the IAM Metadata Service (169.254.169.254).
fn get_role_name(client: &Client<HttpConnector>) -> Result<Box<Future<Item=String, Error=CredentialsError>>, CredentialsError> {
    let role_name_address = format!(
        "http://{}/{}/",
        AWS_CREDENTIALS_PROVIDER_IP,
        AWS_CREDENTIALS_PROVIDER_PATH
    );
    let uri = try!(role_name_address.parse::<Uri>());
    Ok(make_request(client, uri))
}

/// Gets the credentials for an EC2 Instances IAM Role.
fn get_credentials_from_role(
    client: &Client<HttpConnector>,
    role_name: &str
) -> Result<Box<Future<Item=String, Error=CredentialsError>>, CredentialsError> {
    let credentials_provider_url = format!(
        "http://{}/{}/{}",
        AWS_CREDENTIALS_PROVIDER_IP,
        AWS_CREDENTIALS_PROVIDER_PATH,
        role_name
    );

    let uri = try!(credentials_provider_url.parse::<Uri>());
    Ok(make_request(client, uri))
}
