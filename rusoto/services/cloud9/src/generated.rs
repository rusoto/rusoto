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
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEnvironmentEC2Request {
    /// <p>The number of minutes until the running instance is shut down after the environment has last been used.</p>
    #[serde(rename = "automaticStopTimeMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_stop_time_minutes: Option<i64>,
    /// <p>A unique, case-sensitive string that helps AWS Cloud9 to ensure this operation completes no more than one time.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Client Tokens</a> in the <i>Amazon EC2 API Reference</i>.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The description of the environment to create.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of instance to connect to the environment (for example, <code>t2.micro</code>).</p>
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    /// <p>The name of the environment to create.</p> <p>This name is visible to other AWS IAM users in the same AWS account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the environment owner. This ARN can be the ARN of any AWS IAM principal. If this value is not specified, the ARN defaults to this environment's creator.</p>
    #[serde(rename = "ownerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    /// <p>The ID of the subnet in Amazon VPC that AWS Cloud9 will use to communicate with the Amazon EC2 instance.</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateEnvironmentEC2Result {
    /// <p>The ID of the environment that was created.</p>
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEnvironmentMembershipRequest {
    /// <p>The ID of the environment that contains the environment member you want to add.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p><p>The type of environment member permissions you want to associate with this environment member. Available values include:</p> <ul> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul></p>
    #[serde(rename = "permissions")]
    pub permissions: String,
    /// <p>The Amazon Resource Name (ARN) of the environment member you want to add.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateEnvironmentMembershipResult {
    /// <p>Information about the environment member that was added.</p>
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<EnvironmentMember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEnvironmentMembershipRequest {
    /// <p>The ID of the environment to delete the environment member from.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p>The Amazon Resource Name (ARN) of the environment member to delete from the environment.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteEnvironmentMembershipResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEnvironmentRequest {
    /// <p>The ID of the environment to delete.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteEnvironmentResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEnvironmentMembershipsRequest {
    /// <p>The ID of the environment to get environment member information about.</p>
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// <p>The maximum number of environment members to get information about.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>During a previous call, if there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of environment member permissions to get information about. Available values include:</p> <ul> <li> <p> <code>owner</code>: Owns the environment.</p> </li> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul> <p>If no value is specified, information about all environment members are returned.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an individual environment member to get information about. If no value is specified, information about all environment members are returned.</p>
    #[serde(rename = "userArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEnvironmentMembershipsResult {
    /// <p>Information about the environment members for the environment.</p>
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<EnvironmentMember>>,
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEnvironmentStatusRequest {
    /// <p>The ID of the environment to get status information about.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEnvironmentStatusResult {
    /// <p>Any informational message about the status of the environment.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The status of the environment. Available values include:</p> <ul> <li> <p> <code>connecting</code>: The environment is connecting.</p> </li> <li> <p> <code>creating</code>: The environment is being created.</p> </li> <li> <p> <code>deleting</code>: The environment is being deleted.</p> </li> <li> <p> <code>error</code>: The environment is in an error state.</p> </li> <li> <p> <code>ready</code>: The environment is ready.</p> </li> <li> <p> <code>stopped</code>: The environment is stopped.</p> </li> <li> <p> <code>stopping</code>: The environment is stopping.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEnvironmentsRequest {
    /// <p>The IDs of individual environments to get information about.</p>
    #[serde(rename = "environmentIds")]
    pub environment_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEnvironmentsResult {
    /// <p>Information about the environments that are returned.</p>
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<Environment>>,
}

/// <p>Information about an AWS Cloud9 development environment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Environment {
    /// <p>The Amazon Resource Name (ARN) of the environment.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description for the environment.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the environment.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the environment.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the environment owner.</p>
    #[serde(rename = "ownerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    /// <p><p>The type of environment. Valid values include the following:</p> <ul> <li> <p> <code>ec2</code>: An Amazon Elastic Compute Cloud (Amazon EC2) instance connects to the environment.</p> </li> <li> <p> <code>ssh</code>: Your own server connects to the environment.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about an environment member for an AWS Cloud9 development environment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnvironmentMember {
    /// <p>The ID of the environment for the environment member.</p>
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// <p>The time, expressed in epoch time format, when the environment member last opened the environment.</p>
    #[serde(rename = "lastAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access: Option<f64>,
    /// <p><p>The type of environment member permissions associated with this environment member. Available values include:</p> <ul> <li> <p> <code>owner</code>: Owns the environment.</p> </li> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul></p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the environment member.</p>
    #[serde(rename = "userArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The user ID in AWS Identity and Access Management (AWS IAM) of the environment member.</p>
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEnvironmentsRequest {
    /// <p>The maximum number of environments to get identifiers for.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>During a previous call, if there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListEnvironmentsResult {
    /// <p>The list of environment identifiers.</p>
    #[serde(rename = "environmentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_ids: Option<Vec<String>>,
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEnvironmentMembershipRequest {
    /// <p>The ID of the environment for the environment member whose settings you want to change.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p><p>The replacement type of environment member permissions you want to associate with this environment member. Available values include:</p> <ul> <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li> <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li> </ul></p>
    #[serde(rename = "permissions")]
    pub permissions: String,
    /// <p>The Amazon Resource Name (ARN) of the environment member whose settings you want to change.</p>
    #[serde(rename = "userArn")]
    pub user_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateEnvironmentMembershipResult {
    /// <p>Information about the environment member whose settings were changed.</p>
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<EnvironmentMember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEnvironmentRequest {
    /// <p>Any new or replacement description for the environment.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the environment to change settings.</p>
    #[serde(rename = "environmentId")]
    pub environment_id: String,
    /// <p>A replacement name for the environment.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateEnvironmentResult {}

/// Errors returned by CreateEnvironmentEC2
#[derive(Debug, PartialEq)]
pub enum CreateEnvironmentEC2Error {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateEnvironmentEC2Error {
    pub fn from_response(res: BufferedHttpResponse) -> CreateEnvironmentEC2Error {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return CreateEnvironmentEC2Error::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return CreateEnvironmentEC2Error::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return CreateEnvironmentEC2Error::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateEnvironmentEC2Error::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return CreateEnvironmentEC2Error::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateEnvironmentEC2Error::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return CreateEnvironmentEC2Error::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateEnvironmentEC2Error::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateEnvironmentEC2Error::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateEnvironmentEC2Error {
    fn from(err: serde_json::error::Error) -> CreateEnvironmentEC2Error {
        CreateEnvironmentEC2Error::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEnvironmentEC2Error {
    fn from(err: CredentialsError) -> CreateEnvironmentEC2Error {
        CreateEnvironmentEC2Error::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEnvironmentEC2Error {
    fn from(err: HttpDispatchError) -> CreateEnvironmentEC2Error {
        CreateEnvironmentEC2Error::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEnvironmentEC2Error {
    fn from(err: io::Error) -> CreateEnvironmentEC2Error {
        CreateEnvironmentEC2Error::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEnvironmentEC2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEnvironmentEC2Error {
    fn description(&self) -> &str {
        match *self {
            CreateEnvironmentEC2Error::BadRequest(ref cause) => cause,
            CreateEnvironmentEC2Error::Conflict(ref cause) => cause,
            CreateEnvironmentEC2Error::Forbidden(ref cause) => cause,
            CreateEnvironmentEC2Error::InternalServerError(ref cause) => cause,
            CreateEnvironmentEC2Error::LimitExceeded(ref cause) => cause,
            CreateEnvironmentEC2Error::NotFound(ref cause) => cause,
            CreateEnvironmentEC2Error::TooManyRequests(ref cause) => cause,
            CreateEnvironmentEC2Error::Validation(ref cause) => cause,
            CreateEnvironmentEC2Error::Credentials(ref err) => err.description(),
            CreateEnvironmentEC2Error::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEnvironmentEC2Error::ParseError(ref cause) => cause,
            CreateEnvironmentEC2Error::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateEnvironmentMembership
#[derive(Debug, PartialEq)]
pub enum CreateEnvironmentMembershipError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateEnvironmentMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateEnvironmentMembershipError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return CreateEnvironmentMembershipError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return CreateEnvironmentMembershipError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return CreateEnvironmentMembershipError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateEnvironmentMembershipError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return CreateEnvironmentMembershipError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return CreateEnvironmentMembershipError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return CreateEnvironmentMembershipError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateEnvironmentMembershipError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateEnvironmentMembershipError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateEnvironmentMembershipError {
    fn from(err: serde_json::error::Error) -> CreateEnvironmentMembershipError {
        CreateEnvironmentMembershipError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEnvironmentMembershipError {
    fn from(err: CredentialsError) -> CreateEnvironmentMembershipError {
        CreateEnvironmentMembershipError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEnvironmentMembershipError {
    fn from(err: HttpDispatchError) -> CreateEnvironmentMembershipError {
        CreateEnvironmentMembershipError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEnvironmentMembershipError {
    fn from(err: io::Error) -> CreateEnvironmentMembershipError {
        CreateEnvironmentMembershipError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEnvironmentMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEnvironmentMembershipError {
    fn description(&self) -> &str {
        match *self {
            CreateEnvironmentMembershipError::BadRequest(ref cause) => cause,
            CreateEnvironmentMembershipError::Conflict(ref cause) => cause,
            CreateEnvironmentMembershipError::Forbidden(ref cause) => cause,
            CreateEnvironmentMembershipError::InternalServerError(ref cause) => cause,
            CreateEnvironmentMembershipError::LimitExceeded(ref cause) => cause,
            CreateEnvironmentMembershipError::NotFound(ref cause) => cause,
            CreateEnvironmentMembershipError::TooManyRequests(ref cause) => cause,
            CreateEnvironmentMembershipError::Validation(ref cause) => cause,
            CreateEnvironmentMembershipError::Credentials(ref err) => err.description(),
            CreateEnvironmentMembershipError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEnvironmentMembershipError::ParseError(ref cause) => cause,
            CreateEnvironmentMembershipError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteEnvironmentError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEnvironmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteEnvironmentError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteEnvironmentError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteEnvironmentError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteEnvironmentError::InternalServerError(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteEnvironmentError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteEnvironmentError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteEnvironmentError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteEnvironmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteEnvironmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEnvironmentError {
    fn from(err: serde_json::error::Error) -> DeleteEnvironmentError {
        DeleteEnvironmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEnvironmentError {
    fn from(err: CredentialsError) -> DeleteEnvironmentError {
        DeleteEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEnvironmentError {
    fn from(err: HttpDispatchError) -> DeleteEnvironmentError {
        DeleteEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEnvironmentError {
    fn from(err: io::Error) -> DeleteEnvironmentError {
        DeleteEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            DeleteEnvironmentError::BadRequest(ref cause) => cause,
            DeleteEnvironmentError::Conflict(ref cause) => cause,
            DeleteEnvironmentError::Forbidden(ref cause) => cause,
            DeleteEnvironmentError::InternalServerError(ref cause) => cause,
            DeleteEnvironmentError::LimitExceeded(ref cause) => cause,
            DeleteEnvironmentError::NotFound(ref cause) => cause,
            DeleteEnvironmentError::TooManyRequests(ref cause) => cause,
            DeleteEnvironmentError::Validation(ref cause) => cause,
            DeleteEnvironmentError::Credentials(ref err) => err.description(),
            DeleteEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEnvironmentError::ParseError(ref cause) => cause,
            DeleteEnvironmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEnvironmentMembership
#[derive(Debug, PartialEq)]
pub enum DeleteEnvironmentMembershipError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteEnvironmentMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEnvironmentMembershipError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteEnvironmentMembershipError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteEnvironmentMembershipError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteEnvironmentMembershipError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteEnvironmentMembershipError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return DeleteEnvironmentMembershipError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DeleteEnvironmentMembershipError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteEnvironmentMembershipError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteEnvironmentMembershipError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteEnvironmentMembershipError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEnvironmentMembershipError {
    fn from(err: serde_json::error::Error) -> DeleteEnvironmentMembershipError {
        DeleteEnvironmentMembershipError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEnvironmentMembershipError {
    fn from(err: CredentialsError) -> DeleteEnvironmentMembershipError {
        DeleteEnvironmentMembershipError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEnvironmentMembershipError {
    fn from(err: HttpDispatchError) -> DeleteEnvironmentMembershipError {
        DeleteEnvironmentMembershipError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEnvironmentMembershipError {
    fn from(err: io::Error) -> DeleteEnvironmentMembershipError {
        DeleteEnvironmentMembershipError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEnvironmentMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEnvironmentMembershipError {
    fn description(&self) -> &str {
        match *self {
            DeleteEnvironmentMembershipError::BadRequest(ref cause) => cause,
            DeleteEnvironmentMembershipError::Conflict(ref cause) => cause,
            DeleteEnvironmentMembershipError::Forbidden(ref cause) => cause,
            DeleteEnvironmentMembershipError::InternalServerError(ref cause) => cause,
            DeleteEnvironmentMembershipError::LimitExceeded(ref cause) => cause,
            DeleteEnvironmentMembershipError::NotFound(ref cause) => cause,
            DeleteEnvironmentMembershipError::TooManyRequests(ref cause) => cause,
            DeleteEnvironmentMembershipError::Validation(ref cause) => cause,
            DeleteEnvironmentMembershipError::Credentials(ref err) => err.description(),
            DeleteEnvironmentMembershipError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEnvironmentMembershipError::ParseError(ref cause) => cause,
            DeleteEnvironmentMembershipError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEnvironmentMemberships
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentMembershipsError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEnvironmentMembershipsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEnvironmentMembershipsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DescribeEnvironmentMembershipsError::BadRequest(String::from(
                        error_message,
                    ))
                }
                "ConflictException" => {
                    return DescribeEnvironmentMembershipsError::Conflict(String::from(
                        error_message,
                    ))
                }
                "ForbiddenException" => {
                    return DescribeEnvironmentMembershipsError::Forbidden(String::from(
                        error_message,
                    ))
                }
                "InternalServerErrorException" => {
                    return DescribeEnvironmentMembershipsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return DescribeEnvironmentMembershipsError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DescribeEnvironmentMembershipsError::NotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DescribeEnvironmentMembershipsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeEnvironmentMembershipsError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeEnvironmentMembershipsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEnvironmentMembershipsError {
    fn from(err: serde_json::error::Error) -> DescribeEnvironmentMembershipsError {
        DescribeEnvironmentMembershipsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentMembershipsError {
    fn from(err: CredentialsError) -> DescribeEnvironmentMembershipsError {
        DescribeEnvironmentMembershipsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentMembershipsError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentMembershipsError {
        DescribeEnvironmentMembershipsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentMembershipsError {
    fn from(err: io::Error) -> DescribeEnvironmentMembershipsError {
        DescribeEnvironmentMembershipsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentMembershipsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentMembershipsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentMembershipsError::BadRequest(ref cause) => cause,
            DescribeEnvironmentMembershipsError::Conflict(ref cause) => cause,
            DescribeEnvironmentMembershipsError::Forbidden(ref cause) => cause,
            DescribeEnvironmentMembershipsError::InternalServerError(ref cause) => cause,
            DescribeEnvironmentMembershipsError::LimitExceeded(ref cause) => cause,
            DescribeEnvironmentMembershipsError::NotFound(ref cause) => cause,
            DescribeEnvironmentMembershipsError::TooManyRequests(ref cause) => cause,
            DescribeEnvironmentMembershipsError::Validation(ref cause) => cause,
            DescribeEnvironmentMembershipsError::Credentials(ref err) => err.description(),
            DescribeEnvironmentMembershipsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentMembershipsError::ParseError(ref cause) => cause,
            DescribeEnvironmentMembershipsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEnvironmentStatus
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentStatusError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEnvironmentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEnvironmentStatusError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DescribeEnvironmentStatusError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DescribeEnvironmentStatusError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DescribeEnvironmentStatusError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DescribeEnvironmentStatusError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return DescribeEnvironmentStatusError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DescribeEnvironmentStatusError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DescribeEnvironmentStatusError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeEnvironmentStatusError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeEnvironmentStatusError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEnvironmentStatusError {
    fn from(err: serde_json::error::Error) -> DescribeEnvironmentStatusError {
        DescribeEnvironmentStatusError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentStatusError {
    fn from(err: CredentialsError) -> DescribeEnvironmentStatusError {
        DescribeEnvironmentStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentStatusError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentStatusError {
        DescribeEnvironmentStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentStatusError {
    fn from(err: io::Error) -> DescribeEnvironmentStatusError {
        DescribeEnvironmentStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentStatusError::BadRequest(ref cause) => cause,
            DescribeEnvironmentStatusError::Conflict(ref cause) => cause,
            DescribeEnvironmentStatusError::Forbidden(ref cause) => cause,
            DescribeEnvironmentStatusError::InternalServerError(ref cause) => cause,
            DescribeEnvironmentStatusError::LimitExceeded(ref cause) => cause,
            DescribeEnvironmentStatusError::NotFound(ref cause) => cause,
            DescribeEnvironmentStatusError::TooManyRequests(ref cause) => cause,
            DescribeEnvironmentStatusError::Validation(ref cause) => cause,
            DescribeEnvironmentStatusError::Credentials(ref err) => err.description(),
            DescribeEnvironmentStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentStatusError::ParseError(ref cause) => cause,
            DescribeEnvironmentStatusError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEnvironments
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentsError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeEnvironmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEnvironmentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DescribeEnvironmentsError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DescribeEnvironmentsError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DescribeEnvironmentsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DescribeEnvironmentsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return DescribeEnvironmentsError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DescribeEnvironmentsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DescribeEnvironmentsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeEnvironmentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeEnvironmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEnvironmentsError {
    fn from(err: serde_json::error::Error) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentsError {
    fn from(err: CredentialsError) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentsError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentsError {
    fn from(err: io::Error) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentsError::BadRequest(ref cause) => cause,
            DescribeEnvironmentsError::Conflict(ref cause) => cause,
            DescribeEnvironmentsError::Forbidden(ref cause) => cause,
            DescribeEnvironmentsError::InternalServerError(ref cause) => cause,
            DescribeEnvironmentsError::LimitExceeded(ref cause) => cause,
            DescribeEnvironmentsError::NotFound(ref cause) => cause,
            DescribeEnvironmentsError::TooManyRequests(ref cause) => cause,
            DescribeEnvironmentsError::Validation(ref cause) => cause,
            DescribeEnvironmentsError::Credentials(ref err) => err.description(),
            DescribeEnvironmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentsError::ParseError(ref cause) => cause,
            DescribeEnvironmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListEnvironments
#[derive(Debug, PartialEq)]
pub enum ListEnvironmentsError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListEnvironmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListEnvironmentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return ListEnvironmentsError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return ListEnvironmentsError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return ListEnvironmentsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return ListEnvironmentsError::InternalServerError(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListEnvironmentsError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return ListEnvironmentsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return ListEnvironmentsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return ListEnvironmentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListEnvironmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListEnvironmentsError {
    fn from(err: serde_json::error::Error) -> ListEnvironmentsError {
        ListEnvironmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListEnvironmentsError {
    fn from(err: CredentialsError) -> ListEnvironmentsError {
        ListEnvironmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEnvironmentsError {
    fn from(err: HttpDispatchError) -> ListEnvironmentsError {
        ListEnvironmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEnvironmentsError {
    fn from(err: io::Error) -> ListEnvironmentsError {
        ListEnvironmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEnvironmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEnvironmentsError {
    fn description(&self) -> &str {
        match *self {
            ListEnvironmentsError::BadRequest(ref cause) => cause,
            ListEnvironmentsError::Conflict(ref cause) => cause,
            ListEnvironmentsError::Forbidden(ref cause) => cause,
            ListEnvironmentsError::InternalServerError(ref cause) => cause,
            ListEnvironmentsError::LimitExceeded(ref cause) => cause,
            ListEnvironmentsError::NotFound(ref cause) => cause,
            ListEnvironmentsError::TooManyRequests(ref cause) => cause,
            ListEnvironmentsError::Validation(ref cause) => cause,
            ListEnvironmentsError::Credentials(ref err) => err.description(),
            ListEnvironmentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListEnvironmentsError::ParseError(ref cause) => cause,
            ListEnvironmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateEnvironmentError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEnvironmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return UpdateEnvironmentError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return UpdateEnvironmentError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateEnvironmentError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateEnvironmentError::InternalServerError(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateEnvironmentError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateEnvironmentError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateEnvironmentError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateEnvironmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateEnvironmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEnvironmentError {
    fn from(err: serde_json::error::Error) -> UpdateEnvironmentError {
        UpdateEnvironmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEnvironmentError {
    fn from(err: CredentialsError) -> UpdateEnvironmentError {
        UpdateEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEnvironmentError {
    fn from(err: HttpDispatchError) -> UpdateEnvironmentError {
        UpdateEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEnvironmentError {
    fn from(err: io::Error) -> UpdateEnvironmentError {
        UpdateEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            UpdateEnvironmentError::BadRequest(ref cause) => cause,
            UpdateEnvironmentError::Conflict(ref cause) => cause,
            UpdateEnvironmentError::Forbidden(ref cause) => cause,
            UpdateEnvironmentError::InternalServerError(ref cause) => cause,
            UpdateEnvironmentError::LimitExceeded(ref cause) => cause,
            UpdateEnvironmentError::NotFound(ref cause) => cause,
            UpdateEnvironmentError::TooManyRequests(ref cause) => cause,
            UpdateEnvironmentError::Validation(ref cause) => cause,
            UpdateEnvironmentError::Credentials(ref err) => err.description(),
            UpdateEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEnvironmentError::ParseError(ref cause) => cause,
            UpdateEnvironmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateEnvironmentMembership
#[derive(Debug, PartialEq)]
pub enum UpdateEnvironmentMembershipError {
    /// <p>The target request is invalid.</p>
    BadRequest(String),
    /// <p>A conflict occurred.</p>
    Conflict(String),
    /// <p>An access permissions issue occurred.</p>
    Forbidden(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// <p>A service limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The target resource cannot be found.</p>
    NotFound(String),
    /// <p>Too many service requests were made over the given time period.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateEnvironmentMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEnvironmentMembershipError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return UpdateEnvironmentMembershipError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return UpdateEnvironmentMembershipError::Conflict(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateEnvironmentMembershipError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateEnvironmentMembershipError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return UpdateEnvironmentMembershipError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return UpdateEnvironmentMembershipError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateEnvironmentMembershipError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateEnvironmentMembershipError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateEnvironmentMembershipError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEnvironmentMembershipError {
    fn from(err: serde_json::error::Error) -> UpdateEnvironmentMembershipError {
        UpdateEnvironmentMembershipError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEnvironmentMembershipError {
    fn from(err: CredentialsError) -> UpdateEnvironmentMembershipError {
        UpdateEnvironmentMembershipError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEnvironmentMembershipError {
    fn from(err: HttpDispatchError) -> UpdateEnvironmentMembershipError {
        UpdateEnvironmentMembershipError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEnvironmentMembershipError {
    fn from(err: io::Error) -> UpdateEnvironmentMembershipError {
        UpdateEnvironmentMembershipError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEnvironmentMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEnvironmentMembershipError {
    fn description(&self) -> &str {
        match *self {
            UpdateEnvironmentMembershipError::BadRequest(ref cause) => cause,
            UpdateEnvironmentMembershipError::Conflict(ref cause) => cause,
            UpdateEnvironmentMembershipError::Forbidden(ref cause) => cause,
            UpdateEnvironmentMembershipError::InternalServerError(ref cause) => cause,
            UpdateEnvironmentMembershipError::LimitExceeded(ref cause) => cause,
            UpdateEnvironmentMembershipError::NotFound(ref cause) => cause,
            UpdateEnvironmentMembershipError::TooManyRequests(ref cause) => cause,
            UpdateEnvironmentMembershipError::Validation(ref cause) => cause,
            UpdateEnvironmentMembershipError::Credentials(ref err) => err.description(),
            UpdateEnvironmentMembershipError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEnvironmentMembershipError::ParseError(ref cause) => cause,
            UpdateEnvironmentMembershipError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Cloud9 API. AWS Cloud9 clients implement this trait.
pub trait Cloud9 {
    /// <p>Creates an AWS Cloud9 development environment, launches an Amazon Elastic Compute Cloud (Amazon EC2) instance, and then connects from the instance to the environment.</p>
    fn create_environment_ec2(
        &self,
        input: CreateEnvironmentEC2Request,
    ) -> RusotoFuture<CreateEnvironmentEC2Result, CreateEnvironmentEC2Error>;

    /// <p>Adds an environment member to an AWS Cloud9 development environment.</p>
    fn create_environment_membership(
        &self,
        input: CreateEnvironmentMembershipRequest,
    ) -> RusotoFuture<CreateEnvironmentMembershipResult, CreateEnvironmentMembershipError>;

    /// <p>Deletes an AWS Cloud9 development environment. If an Amazon EC2 instance is connected to the environment, also terminates the instance.</p>
    fn delete_environment(
        &self,
        input: DeleteEnvironmentRequest,
    ) -> RusotoFuture<DeleteEnvironmentResult, DeleteEnvironmentError>;

    /// <p>Deletes an environment member from an AWS Cloud9 development environment.</p>
    fn delete_environment_membership(
        &self,
        input: DeleteEnvironmentMembershipRequest,
    ) -> RusotoFuture<DeleteEnvironmentMembershipResult, DeleteEnvironmentMembershipError>;

    /// <p>Gets information about environment members for an AWS Cloud9 development environment.</p>
    fn describe_environment_memberships(
        &self,
        input: DescribeEnvironmentMembershipsRequest,
    ) -> RusotoFuture<DescribeEnvironmentMembershipsResult, DescribeEnvironmentMembershipsError>;

    /// <p>Gets status information for an AWS Cloud9 development environment.</p>
    fn describe_environment_status(
        &self,
        input: DescribeEnvironmentStatusRequest,
    ) -> RusotoFuture<DescribeEnvironmentStatusResult, DescribeEnvironmentStatusError>;

    /// <p>Gets information about AWS Cloud9 development environments.</p>
    fn describe_environments(
        &self,
        input: DescribeEnvironmentsRequest,
    ) -> RusotoFuture<DescribeEnvironmentsResult, DescribeEnvironmentsError>;

    /// <p>Gets a list of AWS Cloud9 development environment identifiers.</p>
    fn list_environments(
        &self,
        input: ListEnvironmentsRequest,
    ) -> RusotoFuture<ListEnvironmentsResult, ListEnvironmentsError>;

    /// <p>Changes the settings of an existing AWS Cloud9 development environment.</p>
    fn update_environment(
        &self,
        input: UpdateEnvironmentRequest,
    ) -> RusotoFuture<UpdateEnvironmentResult, UpdateEnvironmentError>;

    /// <p>Changes the settings of an existing environment member for an AWS Cloud9 development environment.</p>
    fn update_environment_membership(
        &self,
        input: UpdateEnvironmentMembershipRequest,
    ) -> RusotoFuture<UpdateEnvironmentMembershipResult, UpdateEnvironmentMembershipError>;
}
/// A client for the AWS Cloud9 API.
pub struct Cloud9Client {
    client: Client,
    region: region::Region,
}

impl Cloud9Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Cloud9Client {
        Cloud9Client {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Cloud9Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Cloud9Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Cloud9 for Cloud9Client {
    /// <p>Creates an AWS Cloud9 development environment, launches an Amazon Elastic Compute Cloud (Amazon EC2) instance, and then connects from the instance to the environment.</p>
    fn create_environment_ec2(
        &self,
        input: CreateEnvironmentEC2Request,
    ) -> RusotoFuture<CreateEnvironmentEC2Result, CreateEnvironmentEC2Error> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.CreateEnvironmentEC2",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateEnvironmentEC2Result>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateEnvironmentEC2Error::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds an environment member to an AWS Cloud9 development environment.</p>
    fn create_environment_membership(
        &self,
        input: CreateEnvironmentMembershipRequest,
    ) -> RusotoFuture<CreateEnvironmentMembershipResult, CreateEnvironmentMembershipError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.CreateEnvironmentMembership",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateEnvironmentMembershipResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateEnvironmentMembershipError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes an AWS Cloud9 development environment. If an Amazon EC2 instance is connected to the environment, also terminates the instance.</p>
    fn delete_environment(
        &self,
        input: DeleteEnvironmentRequest,
    ) -> RusotoFuture<DeleteEnvironmentResult, DeleteEnvironmentError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DeleteEnvironment",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteEnvironmentResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEnvironmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an environment member from an AWS Cloud9 development environment.</p>
    fn delete_environment_membership(
        &self,
        input: DeleteEnvironmentMembershipRequest,
    ) -> RusotoFuture<DeleteEnvironmentMembershipResult, DeleteEnvironmentMembershipError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DeleteEnvironmentMembership",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteEnvironmentMembershipResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEnvironmentMembershipError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about environment members for an AWS Cloud9 development environment.</p>
    fn describe_environment_memberships(
        &self,
        input: DescribeEnvironmentMembershipsRequest,
    ) -> RusotoFuture<DescribeEnvironmentMembershipsResult, DescribeEnvironmentMembershipsError>
    {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DescribeEnvironmentMemberships",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEnvironmentMembershipsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentMembershipsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets status information for an AWS Cloud9 development environment.</p>
    fn describe_environment_status(
        &self,
        input: DescribeEnvironmentStatusRequest,
    ) -> RusotoFuture<DescribeEnvironmentStatusResult, DescribeEnvironmentStatusError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DescribeEnvironmentStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEnvironmentStatusResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about AWS Cloud9 development environments.</p>
    fn describe_environments(
        &self,
        input: DescribeEnvironmentsRequest,
    ) -> RusotoFuture<DescribeEnvironmentsResult, DescribeEnvironmentsError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.DescribeEnvironments",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEnvironmentsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeEnvironmentsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets a list of AWS Cloud9 development environment identifiers.</p>
    fn list_environments(
        &self,
        input: ListEnvironmentsRequest,
    ) -> RusotoFuture<ListEnvironmentsResult, ListEnvironmentsError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.ListEnvironments",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListEnvironmentsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListEnvironmentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes the settings of an existing AWS Cloud9 development environment.</p>
    fn update_environment(
        &self,
        input: UpdateEnvironmentRequest,
    ) -> RusotoFuture<UpdateEnvironmentResult, UpdateEnvironmentError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.UpdateEnvironment",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateEnvironmentResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateEnvironmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes the settings of an existing environment member for an AWS Cloud9 development environment.</p>
    fn update_environment_membership(
        &self,
        input: UpdateEnvironmentMembershipRequest,
    ) -> RusotoFuture<UpdateEnvironmentMembershipResult, UpdateEnvironmentMembershipError> {
        let mut request = SignedRequest::new("POST", "cloud9", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCloud9WorkspaceManagementService.UpdateEnvironmentMembership",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateEnvironmentMembershipResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateEnvironmentMembershipError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
