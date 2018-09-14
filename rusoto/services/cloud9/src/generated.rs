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
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateEnvironmentEC2Error {
    pub fn from_body(body: &str) -> CreateEnvironmentEC2Error {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateEnvironmentEC2Error::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateEnvironmentEC2Error::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateEnvironmentEC2Error::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateEnvironmentEC2Error::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateEnvironmentEC2Error::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateEnvironmentEC2Error::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateEnvironmentEC2Error::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateEnvironmentEC2Error::Validation(error_message.to_string())
                    }
                    _ => CreateEnvironmentEC2Error::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateEnvironmentEC2Error::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateEnvironmentEC2Error {
    fn from(err: serde_json::error::Error) -> CreateEnvironmentEC2Error {
        CreateEnvironmentEC2Error::Unknown(err.description().to_string())
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
            CreateEnvironmentEC2Error::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateEnvironmentMembershipError {
    pub fn from_body(body: &str) -> CreateEnvironmentMembershipError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateEnvironmentMembershipError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateEnvironmentMembershipError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateEnvironmentMembershipError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateEnvironmentMembershipError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        CreateEnvironmentMembershipError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateEnvironmentMembershipError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateEnvironmentMembershipError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateEnvironmentMembershipError::Validation(error_message.to_string())
                    }
                    _ => CreateEnvironmentMembershipError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateEnvironmentMembershipError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateEnvironmentMembershipError {
    fn from(err: serde_json::error::Error) -> CreateEnvironmentMembershipError {
        CreateEnvironmentMembershipError::Unknown(err.description().to_string())
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
            CreateEnvironmentMembershipError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteEnvironmentError {
    pub fn from_body(body: &str) -> DeleteEnvironmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteEnvironmentError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteEnvironmentError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteEnvironmentError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteEnvironmentError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteEnvironmentError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteEnvironmentError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteEnvironmentError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteEnvironmentError::Validation(error_message.to_string())
                    }
                    _ => DeleteEnvironmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEnvironmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEnvironmentError {
    fn from(err: serde_json::error::Error) -> DeleteEnvironmentError {
        DeleteEnvironmentError::Unknown(err.description().to_string())
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
            DeleteEnvironmentError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteEnvironmentMembershipError {
    pub fn from_body(body: &str) -> DeleteEnvironmentMembershipError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteEnvironmentMembershipError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteEnvironmentMembershipError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteEnvironmentMembershipError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteEnvironmentMembershipError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        DeleteEnvironmentMembershipError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteEnvironmentMembershipError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteEnvironmentMembershipError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteEnvironmentMembershipError::Validation(error_message.to_string())
                    }
                    _ => DeleteEnvironmentMembershipError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEnvironmentMembershipError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEnvironmentMembershipError {
    fn from(err: serde_json::error::Error) -> DeleteEnvironmentMembershipError {
        DeleteEnvironmentMembershipError::Unknown(err.description().to_string())
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
            DeleteEnvironmentMembershipError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentMembershipsError {
    pub fn from_body(body: &str) -> DescribeEnvironmentMembershipsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DescribeEnvironmentMembershipsError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DescribeEnvironmentMembershipsError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DescribeEnvironmentMembershipsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DescribeEnvironmentMembershipsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => DescribeEnvironmentMembershipsError::LimitExceeded(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        DescribeEnvironmentMembershipsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeEnvironmentMembershipsError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeEnvironmentMembershipsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEnvironmentMembershipsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEnvironmentMembershipsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEnvironmentMembershipsError {
    fn from(err: serde_json::error::Error) -> DescribeEnvironmentMembershipsError {
        DescribeEnvironmentMembershipsError::Unknown(err.description().to_string())
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
            DescribeEnvironmentMembershipsError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentStatusError {
    pub fn from_body(body: &str) -> DescribeEnvironmentStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DescribeEnvironmentStatusError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DescribeEnvironmentStatusError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DescribeEnvironmentStatusError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DescribeEnvironmentStatusError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        DescribeEnvironmentStatusError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeEnvironmentStatusError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeEnvironmentStatusError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEnvironmentStatusError::Validation(error_message.to_string())
                    }
                    _ => DescribeEnvironmentStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEnvironmentStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEnvironmentStatusError {
    fn from(err: serde_json::error::Error) -> DescribeEnvironmentStatusError {
        DescribeEnvironmentStatusError::Unknown(err.description().to_string())
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
            DescribeEnvironmentStatusError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentsError {
    pub fn from_body(body: &str) -> DescribeEnvironmentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DescribeEnvironmentsError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DescribeEnvironmentsError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DescribeEnvironmentsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DescribeEnvironmentsError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DescribeEnvironmentsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeEnvironmentsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeEnvironmentsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEnvironmentsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEnvironmentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEnvironmentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEnvironmentsError {
    fn from(err: serde_json::error::Error) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::Unknown(err.description().to_string())
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
            DescribeEnvironmentsError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListEnvironmentsError {
    pub fn from_body(body: &str) -> ListEnvironmentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListEnvironmentsError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        ListEnvironmentsError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListEnvironmentsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListEnvironmentsError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListEnvironmentsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListEnvironmentsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListEnvironmentsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListEnvironmentsError::Validation(error_message.to_string())
                    }
                    _ => ListEnvironmentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListEnvironmentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListEnvironmentsError {
    fn from(err: serde_json::error::Error) -> ListEnvironmentsError {
        ListEnvironmentsError::Unknown(err.description().to_string())
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
            ListEnvironmentsError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateEnvironmentError {
    pub fn from_body(body: &str) -> UpdateEnvironmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateEnvironmentError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateEnvironmentError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateEnvironmentError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateEnvironmentError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateEnvironmentError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateEnvironmentError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateEnvironmentError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateEnvironmentError::Validation(error_message.to_string())
                    }
                    _ => UpdateEnvironmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEnvironmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEnvironmentError {
    fn from(err: serde_json::error::Error) -> UpdateEnvironmentError {
        UpdateEnvironmentError::Unknown(err.description().to_string())
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
            UpdateEnvironmentError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateEnvironmentMembershipError {
    pub fn from_body(body: &str) -> UpdateEnvironmentMembershipError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateEnvironmentMembershipError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateEnvironmentMembershipError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateEnvironmentMembershipError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateEnvironmentMembershipError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        UpdateEnvironmentMembershipError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateEnvironmentMembershipError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateEnvironmentMembershipError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateEnvironmentMembershipError::Validation(error_message.to_string())
                    }
                    _ => UpdateEnvironmentMembershipError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEnvironmentMembershipError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEnvironmentMembershipError {
    fn from(err: serde_json::error::Error) -> UpdateEnvironmentMembershipError {
        UpdateEnvironmentMembershipError::Unknown(err.description().to_string())
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
            UpdateEnvironmentMembershipError::Unknown(ref cause) => cause,
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateEnvironmentEC2Error::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateEnvironmentMembershipError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEnvironmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEnvironmentMembershipError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentMembershipsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListEnvironmentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateEnvironmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateEnvironmentMembershipError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
