// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl Ec2InstanceConnectClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(
            http_method,
            "ec2-instance-connect",
            &self.region,
            request_uri,
        );

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendSSHPublicKeyRequest {
    /// <p>The Availability Zone in which the EC2 instance was launched.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>The ID of the EC2 instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
    #[serde(rename = "InstanceOSUser")]
    pub instance_os_user: String,
    /// <p>The public key material. To use the public key, you must have the matching private key.</p>
    #[serde(rename = "SSHPublicKey")]
    pub ssh_public_key: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendSSHPublicKeyResponse {
    /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Is true if the request succeeds and an error otherwise.</p>
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendSerialConsoleSSHPublicKeyRequest {
    /// <p>The ID of the EC2 instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[serde(rename = "SSHPublicKey")]
    pub ssh_public_key: String,
    /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p> <p>Default: 0</p>
    #[serde(rename = "SerialPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_port: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendSerialConsoleSSHPublicKeyResponse {
    /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Is true if the request succeeds and an error otherwise.</p>
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// Errors returned by SendSSHPublicKey
#[derive(Debug, PartialEq)]
pub enum SendSSHPublicKeyError {
    /// <p>Either your AWS credentials are not valid or you do not have access to the EC2 instance.</p>
    Auth(String),
    /// <p>The specified instance was not found.</p>
    EC2InstanceNotFound(String),
    /// <p>One of the parameters is not valid.</p>
    InvalidArgs(String),
    /// <p>The service encountered an error. Follow the instructions in the error message and try again.</p>
    Service(String),
    /// <p>The requests were made too frequently and have been throttled. Wait a while and try again. To increase the limit on your request frequency, contact AWS Support.</p>
    Throttling(String),
}

impl SendSSHPublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendSSHPublicKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::Auth(err.msg))
                }
                "EC2InstanceNotFoundException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::EC2InstanceNotFound(
                        err.msg,
                    ))
                }
                "InvalidArgsException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::InvalidArgs(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::Service(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendSSHPublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendSSHPublicKeyError::Auth(ref cause) => write!(f, "{}", cause),
            SendSSHPublicKeyError::EC2InstanceNotFound(ref cause) => write!(f, "{}", cause),
            SendSSHPublicKeyError::InvalidArgs(ref cause) => write!(f, "{}", cause),
            SendSSHPublicKeyError::Service(ref cause) => write!(f, "{}", cause),
            SendSSHPublicKeyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendSSHPublicKeyError {}
/// Errors returned by SendSerialConsoleSSHPublicKey
#[derive(Debug, PartialEq)]
pub enum SendSerialConsoleSSHPublicKeyError {
    /// <p>Either your AWS credentials are not valid or you do not have access to the EC2 instance.</p>
    Auth(String),
    /// <p>The specified instance was not found.</p>
    EC2InstanceNotFound(String),
    /// <p>The instance type is not supported for connecting via the serial console. Only Nitro instance types are currently supported.</p>
    EC2InstanceTypeInvalid(String),
    /// <p>One of the parameters is not valid.</p>
    InvalidArgs(String),
    /// <p>Your account is not authorized to use the EC2 Serial Console. To authorize your account, run the EnableSerialConsoleAccess API. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EnableSerialConsoleAccess.html">EnableSerialConsoleAccess</a> in the <i>Amazon EC2 API Reference</i>.</p>
    SerialConsoleAccessDisabled(String),
    /// <p>The instance currently has 1 active serial console session. Only 1 session is supported at a time.</p>
    SerialConsoleSessionLimitExceeded(String),
    /// <p>Unable to start a serial console session. Please try again.</p>
    SerialConsoleSessionUnavailable(String),
    /// <p>The service encountered an error. Follow the instructions in the error message and try again.</p>
    Service(String),
    /// <p>The requests were made too frequently and have been throttled. Wait a while and try again. To increase the limit on your request frequency, contact AWS Support.</p>
    Throttling(String),
}

impl SendSerialConsoleSSHPublicKeyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SendSerialConsoleSSHPublicKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthException" => {
                    return RusotoError::Service(SendSerialConsoleSSHPublicKeyError::Auth(err.msg))
                }
                "EC2InstanceNotFoundException" => {
                    return RusotoError::Service(
                        SendSerialConsoleSSHPublicKeyError::EC2InstanceNotFound(err.msg),
                    )
                }
                "EC2InstanceTypeInvalidException" => {
                    return RusotoError::Service(
                        SendSerialConsoleSSHPublicKeyError::EC2InstanceTypeInvalid(err.msg),
                    )
                }
                "InvalidArgsException" => {
                    return RusotoError::Service(SendSerialConsoleSSHPublicKeyError::InvalidArgs(
                        err.msg,
                    ))
                }
                "SerialConsoleAccessDisabledException" => {
                    return RusotoError::Service(
                        SendSerialConsoleSSHPublicKeyError::SerialConsoleAccessDisabled(err.msg),
                    )
                }
                "SerialConsoleSessionLimitExceededException" => {
                    return RusotoError::Service(
                        SendSerialConsoleSSHPublicKeyError::SerialConsoleSessionLimitExceeded(
                            err.msg,
                        ),
                    )
                }
                "SerialConsoleSessionUnavailableException" => {
                    return RusotoError::Service(
                        SendSerialConsoleSSHPublicKeyError::SerialConsoleSessionUnavailable(
                            err.msg,
                        ),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(SendSerialConsoleSSHPublicKeyError::Service(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SendSerialConsoleSSHPublicKeyError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendSerialConsoleSSHPublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendSerialConsoleSSHPublicKeyError::Auth(ref cause) => write!(f, "{}", cause),
            SendSerialConsoleSSHPublicKeyError::EC2InstanceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SendSerialConsoleSSHPublicKeyError::EC2InstanceTypeInvalid(ref cause) => {
                write!(f, "{}", cause)
            }
            SendSerialConsoleSSHPublicKeyError::InvalidArgs(ref cause) => write!(f, "{}", cause),
            SendSerialConsoleSSHPublicKeyError::SerialConsoleAccessDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            SendSerialConsoleSSHPublicKeyError::SerialConsoleSessionLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            SendSerialConsoleSSHPublicKeyError::SerialConsoleSessionUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            SendSerialConsoleSSHPublicKeyError::Service(ref cause) => write!(f, "{}", cause),
            SendSerialConsoleSSHPublicKeyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendSerialConsoleSSHPublicKeyError {}
/// Trait representing the capabilities of the EC2 Instance Connect API. EC2 Instance Connect clients implement this trait.
#[async_trait]
pub trait Ec2InstanceConnect {
    /// <p>Pushes an SSH public key to the specified EC2 instance for use by the specified user. The key remains for 60 seconds. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Connect-using-EC2-Instance-Connect.html">Connect to your Linux instance using EC2 Instance Connect</a> in the <i>Amazon EC2 User Guide</i>.</p>
    async fn send_ssh_public_key(
        &self,
        input: SendSSHPublicKeyRequest,
    ) -> Result<SendSSHPublicKeyResponse, RusotoError<SendSSHPublicKeyError>>;

    /// <p>Pushes an SSH public key to the specified EC2 instance. The key remains for 60 seconds, which gives you 60 seconds to establish a serial console connection to the instance using SSH. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-serial-console.html">EC2 Serial Console</a> in the <i>Amazon EC2 User Guide</i>.</p>
    async fn send_serial_console_ssh_public_key(
        &self,
        input: SendSerialConsoleSSHPublicKeyRequest,
    ) -> Result<
        SendSerialConsoleSSHPublicKeyResponse,
        RusotoError<SendSerialConsoleSSHPublicKeyError>,
    >;
}
/// A client for the EC2 Instance Connect API.
#[derive(Clone)]
pub struct Ec2InstanceConnectClient {
    client: Client,
    region: region::Region,
}

impl Ec2InstanceConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Ec2InstanceConnectClient {
        Ec2InstanceConnectClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Ec2InstanceConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        Ec2InstanceConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Ec2InstanceConnectClient {
        Ec2InstanceConnectClient { client, region }
    }
}

#[async_trait]
impl Ec2InstanceConnect for Ec2InstanceConnectClient {
    /// <p>Pushes an SSH public key to the specified EC2 instance for use by the specified user. The key remains for 60 seconds. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Connect-using-EC2-Instance-Connect.html">Connect to your Linux instance using EC2 Instance Connect</a> in the <i>Amazon EC2 User Guide</i>.</p>
    async fn send_ssh_public_key(
        &self,
        input: SendSSHPublicKeyRequest,
    ) -> Result<SendSSHPublicKeyResponse, RusotoError<SendSSHPublicKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSEC2InstanceConnectService.SendSSHPublicKey",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SendSSHPublicKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SendSSHPublicKeyResponse, _>()
    }

    /// <p>Pushes an SSH public key to the specified EC2 instance. The key remains for 60 seconds, which gives you 60 seconds to establish a serial console connection to the instance using SSH. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-serial-console.html">EC2 Serial Console</a> in the <i>Amazon EC2 User Guide</i>.</p>
    async fn send_serial_console_ssh_public_key(
        &self,
        input: SendSerialConsoleSSHPublicKeyRequest,
    ) -> Result<
        SendSerialConsoleSSHPublicKeyResponse,
        RusotoError<SendSerialConsoleSSHPublicKeyError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSEC2InstanceConnectService.SendSerialConsoleSSHPublicKey",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SendSerialConsoleSSHPublicKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SendSerialConsoleSSHPublicKeyResponse, _>()
    }
}
