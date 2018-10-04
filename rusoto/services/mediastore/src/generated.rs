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
/// <p>This section describes operations that you can perform on an AWS Elemental MediaStore container.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Container {
    /// <p>The Amazon Resource Name (ARN) of the container. The ARN has the following format:</p> <p>arn:aws:&lt;region&gt;:&lt;account that owns this container&gt;:container/&lt;name of container&gt; </p> <p>For example: arn:aws:mediastore:us-west-2:111122223333:container/movies </p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Unix timestamp.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The DNS endpoint of the container. Use the endpoint to identify the specific container when sending requests to the data plane. The service assigns this value when the container is created. Once the value has been assigned, it does not change.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The name of the container.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of container creation or deletion. The status is one of the following: <code>CREATING</code>, <code>ACTIVE</code>, or <code>DELETING</code>. While the service is creating the container, the status is <code>CREATING</code>. When the endpoint is available, the status changes to <code>ACTIVE</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>A rule for a CORS policy. You can add up to 100 rules to a CORS policy. If more than one rule applies, the service uses the first applicable rule listed.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    /// <p>Specifies which headers are allowed in a preflight <code>OPTIONS</code> request through the <code>Access-Control-Request-Headers</code> header. Each header name that is specified in <code>Access-Control-Request-Headers</code> must have a corresponding entry in the rule. Only the headers that were requested are sent back. </p> <p>This element can contain only one wildcard character (*).</p>
    #[serde(rename = "AllowedHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_headers: Option<Vec<String>>,
    /// <p>Identifies an HTTP method that the origin that is specified in the rule is allowed to execute.</p> <p>Each CORS rule must contain at least one <code>AllowedMethod</code> and one <code>AllowedOrigin</code> element.</p>
    #[serde(rename = "AllowedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<Vec<String>>,
    /// <p>One or more response headers that you want users to be able to access from their applications (for example, from a JavaScript <code>XMLHttpRequest</code> object).</p> <p>Each CORS rule must have at least one <code>AllowedOrigin</code> element. The string value can include only one wildcard character (*), for example, http://*.example.com. Additionally, you can specify only one wildcard character to allow cross-origin access for all origins.</p>
    #[serde(rename = "AllowedOrigins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    /// <p>One or more headers in the response that you want users to be able to access from their applications (for example, from a JavaScript <code>XMLHttpRequest</code> object).</p> <p>This element is optional for each rule.</p>
    #[serde(rename = "ExposeHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    /// <p>The time in seconds that your browser caches the preflight response for the specified resource.</p> <p>A CORS rule can have only one <code>MaxAgeSeconds</code> element.</p>
    #[serde(rename = "MaxAgeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateContainerInput {
    /// <p>The name for the container. The name must be from 1 to 255 characters. Container names must be unique to your AWS account within a specific region. As an example, you could create a container named <code>movies</code> in every region, as long as you don’t have an existing container with that name.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateContainerOutput {
    /// <p>ContainerARN: The Amazon Resource Name (ARN) of the newly created container. The ARN has the following format: arn:aws:&lt;region&gt;:&lt;account that owns this container&gt;:container/&lt;name of container&gt;. For example: arn:aws:mediastore:us-west-2:111122223333:container/movies </p> <p>ContainerName: The container name as specified in the request.</p> <p>CreationTime: Unix time stamp.</p> <p>Status: The status of container creation or deletion. The status is one of the following: <code>CREATING</code>, <code>ACTIVE</code>, or <code>DELETING</code>. While the service is creating the container, the status is <code>CREATING</code>. When an endpoint is available, the status changes to <code>ACTIVE</code>.</p> <p>The return value does not include the container's endpoint. To make downstream requests, you must obtain this value by using <a>DescribeContainer</a> or <a>ListContainers</a>.</p>
    #[serde(rename = "Container")]
    pub container: Container,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteContainerInput {
    /// <p>The name of the container to delete. </p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteContainerOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteContainerPolicyInput {
    /// <p>The name of the container that holds the policy.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteContainerPolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCorsPolicyInput {
    /// <p>The name of the container to remove the policy from.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteCorsPolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeContainerInput {
    /// <p>The name of the container to query.</p>
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeContainerOutput {
    /// <p>The name of the queried container.</p>
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetContainerPolicyInput {
    /// <p>The name of the container. </p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetContainerPolicyOutput {
    /// <p>The contents of the access policy.</p>
    #[serde(rename = "Policy")]
    pub policy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCorsPolicyInput {
    /// <p>The name of the container that the policy is assigned to.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCorsPolicyOutput {
    #[serde(rename = "CorsPolicy")]
    pub cors_policy: Vec<CorsRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListContainersInput {
    /// <p>Enter the maximum number of containers in the response. Use from 1 to 255 characters. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Only if you used <code>MaxResults</code> in the first command, enter the token (which was included in the previous response) to obtain the next set of containers. This token is included in a response only if there actually are more containers to list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListContainersOutput {
    /// <p>The names of the containers.</p>
    #[serde(rename = "Containers")]
    pub containers: Vec<Container>,
    /// <p> <code>NextToken</code> is the token to use in the next call to <code>ListContainers</code>. This token is returned only if you included the <code>MaxResults</code> tag in the original command, and only if there are still containers to return. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutContainerPolicyInput {
    /// <p>The name of the container.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// <p><p>The contents of the policy, which includes the following: </p> <ul> <li> <p>One <code>Version</code> tag</p> </li> <li> <p>One <code>Statement</code> tag that contains the standard tags for the policy.</p> </li> </ul></p>
    #[serde(rename = "Policy")]
    pub policy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutContainerPolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutCorsPolicyInput {
    /// <p>The name of the container that you want to assign the CORS policy to.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// <p>The CORS policy to apply to the container. </p>
    #[serde(rename = "CorsPolicy")]
    pub cors_policy: Vec<CorsRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutCorsPolicyOutput {}

/// Errors returned by CreateContainer
#[derive(Debug, PartialEq)]
pub enum CreateContainerError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>A service limit has been exceeded.</p>
    LimitExceeded(String),
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

impl CreateContainerError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateContainerError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return CreateContainerError::ContainerInUse(String::from(error_message))
                }
                "InternalServerError" => {
                    return CreateContainerError::InternalServerError(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateContainerError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateContainerError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateContainerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateContainerError {
    fn from(err: serde_json::error::Error) -> CreateContainerError {
        CreateContainerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateContainerError {
    fn from(err: CredentialsError) -> CreateContainerError {
        CreateContainerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateContainerError {
    fn from(err: HttpDispatchError) -> CreateContainerError {
        CreateContainerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateContainerError {
    fn from(err: io::Error) -> CreateContainerError {
        CreateContainerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateContainerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateContainerError {
    fn description(&self) -> &str {
        match *self {
            CreateContainerError::ContainerInUse(ref cause) => cause,
            CreateContainerError::InternalServerError(ref cause) => cause,
            CreateContainerError::LimitExceeded(ref cause) => cause,
            CreateContainerError::Validation(ref cause) => cause,
            CreateContainerError::Credentials(ref err) => err.description(),
            CreateContainerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateContainerError::ParseError(ref cause) => cause,
            CreateContainerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteContainer
#[derive(Debug, PartialEq)]
pub enum DeleteContainerError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
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

impl DeleteContainerError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteContainerError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return DeleteContainerError::ContainerInUse(String::from(error_message))
                }
                "ContainerNotFoundException" => {
                    return DeleteContainerError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return DeleteContainerError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteContainerError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteContainerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteContainerError {
    fn from(err: serde_json::error::Error) -> DeleteContainerError {
        DeleteContainerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteContainerError {
    fn from(err: CredentialsError) -> DeleteContainerError {
        DeleteContainerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteContainerError {
    fn from(err: HttpDispatchError) -> DeleteContainerError {
        DeleteContainerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteContainerError {
    fn from(err: io::Error) -> DeleteContainerError {
        DeleteContainerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteContainerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteContainerError {
    fn description(&self) -> &str {
        match *self {
            DeleteContainerError::ContainerInUse(ref cause) => cause,
            DeleteContainerError::ContainerNotFound(ref cause) => cause,
            DeleteContainerError::InternalServerError(ref cause) => cause,
            DeleteContainerError::Validation(ref cause) => cause,
            DeleteContainerError::Credentials(ref err) => err.description(),
            DeleteContainerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteContainerError::ParseError(ref cause) => cause,
            DeleteContainerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteContainerPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteContainerPolicyError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on a policy that does not exist.</p>
    PolicyNotFound(String),
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

impl DeleteContainerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteContainerPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return DeleteContainerPolicyError::ContainerInUse(String::from(error_message))
                }
                "ContainerNotFoundException" => {
                    return DeleteContainerPolicyError::ContainerNotFound(String::from(
                        error_message,
                    ))
                }
                "InternalServerError" => {
                    return DeleteContainerPolicyError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "PolicyNotFoundException" => {
                    return DeleteContainerPolicyError::PolicyNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteContainerPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteContainerPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteContainerPolicyError {
    fn from(err: serde_json::error::Error) -> DeleteContainerPolicyError {
        DeleteContainerPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteContainerPolicyError {
    fn from(err: CredentialsError) -> DeleteContainerPolicyError {
        DeleteContainerPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteContainerPolicyError {
    fn from(err: HttpDispatchError) -> DeleteContainerPolicyError {
        DeleteContainerPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteContainerPolicyError {
    fn from(err: io::Error) -> DeleteContainerPolicyError {
        DeleteContainerPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteContainerPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteContainerPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteContainerPolicyError::ContainerInUse(ref cause) => cause,
            DeleteContainerPolicyError::ContainerNotFound(ref cause) => cause,
            DeleteContainerPolicyError::InternalServerError(ref cause) => cause,
            DeleteContainerPolicyError::PolicyNotFound(ref cause) => cause,
            DeleteContainerPolicyError::Validation(ref cause) => cause,
            DeleteContainerPolicyError::Credentials(ref err) => err.description(),
            DeleteContainerPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteContainerPolicyError::ParseError(ref cause) => cause,
            DeleteContainerPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteCorsPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteCorsPolicyError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>Could not perform an operation on a policy that does not exist.</p>
    CorsPolicyNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
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

impl DeleteCorsPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteCorsPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return DeleteCorsPolicyError::ContainerInUse(String::from(error_message))
                }
                "ContainerNotFoundException" => {
                    return DeleteCorsPolicyError::ContainerNotFound(String::from(error_message))
                }
                "CorsPolicyNotFoundException" => {
                    return DeleteCorsPolicyError::CorsPolicyNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return DeleteCorsPolicyError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteCorsPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteCorsPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteCorsPolicyError {
    fn from(err: serde_json::error::Error) -> DeleteCorsPolicyError {
        DeleteCorsPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCorsPolicyError {
    fn from(err: CredentialsError) -> DeleteCorsPolicyError {
        DeleteCorsPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCorsPolicyError {
    fn from(err: HttpDispatchError) -> DeleteCorsPolicyError {
        DeleteCorsPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCorsPolicyError {
    fn from(err: io::Error) -> DeleteCorsPolicyError {
        DeleteCorsPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCorsPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCorsPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteCorsPolicyError::ContainerInUse(ref cause) => cause,
            DeleteCorsPolicyError::ContainerNotFound(ref cause) => cause,
            DeleteCorsPolicyError::CorsPolicyNotFound(ref cause) => cause,
            DeleteCorsPolicyError::InternalServerError(ref cause) => cause,
            DeleteCorsPolicyError::Validation(ref cause) => cause,
            DeleteCorsPolicyError::Credentials(ref err) => err.description(),
            DeleteCorsPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteCorsPolicyError::ParseError(ref cause) => cause,
            DeleteCorsPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeContainer
#[derive(Debug, PartialEq)]
pub enum DescribeContainerError {
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
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

impl DescribeContainerError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeContainerError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerNotFoundException" => {
                    return DescribeContainerError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return DescribeContainerError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeContainerError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeContainerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeContainerError {
    fn from(err: serde_json::error::Error) -> DescribeContainerError {
        DescribeContainerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeContainerError {
    fn from(err: CredentialsError) -> DescribeContainerError {
        DescribeContainerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeContainerError {
    fn from(err: HttpDispatchError) -> DescribeContainerError {
        DescribeContainerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeContainerError {
    fn from(err: io::Error) -> DescribeContainerError {
        DescribeContainerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeContainerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeContainerError {
    fn description(&self) -> &str {
        match *self {
            DescribeContainerError::ContainerNotFound(ref cause) => cause,
            DescribeContainerError::InternalServerError(ref cause) => cause,
            DescribeContainerError::Validation(ref cause) => cause,
            DescribeContainerError::Credentials(ref err) => err.description(),
            DescribeContainerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeContainerError::ParseError(ref cause) => cause,
            DescribeContainerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetContainerPolicy
#[derive(Debug, PartialEq)]
pub enum GetContainerPolicyError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on a policy that does not exist.</p>
    PolicyNotFound(String),
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

impl GetContainerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> GetContainerPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return GetContainerPolicyError::ContainerInUse(String::from(error_message))
                }
                "ContainerNotFoundException" => {
                    return GetContainerPolicyError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetContainerPolicyError::InternalServerError(String::from(error_message))
                }
                "PolicyNotFoundException" => {
                    return GetContainerPolicyError::PolicyNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetContainerPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetContainerPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetContainerPolicyError {
    fn from(err: serde_json::error::Error) -> GetContainerPolicyError {
        GetContainerPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetContainerPolicyError {
    fn from(err: CredentialsError) -> GetContainerPolicyError {
        GetContainerPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetContainerPolicyError {
    fn from(err: HttpDispatchError) -> GetContainerPolicyError {
        GetContainerPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetContainerPolicyError {
    fn from(err: io::Error) -> GetContainerPolicyError {
        GetContainerPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetContainerPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetContainerPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetContainerPolicyError::ContainerInUse(ref cause) => cause,
            GetContainerPolicyError::ContainerNotFound(ref cause) => cause,
            GetContainerPolicyError::InternalServerError(ref cause) => cause,
            GetContainerPolicyError::PolicyNotFound(ref cause) => cause,
            GetContainerPolicyError::Validation(ref cause) => cause,
            GetContainerPolicyError::Credentials(ref err) => err.description(),
            GetContainerPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetContainerPolicyError::ParseError(ref cause) => cause,
            GetContainerPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCorsPolicy
#[derive(Debug, PartialEq)]
pub enum GetCorsPolicyError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>Could not perform an operation on a policy that does not exist.</p>
    CorsPolicyNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
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

impl GetCorsPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCorsPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return GetCorsPolicyError::ContainerInUse(String::from(error_message))
                }
                "ContainerNotFoundException" => {
                    return GetCorsPolicyError::ContainerNotFound(String::from(error_message))
                }
                "CorsPolicyNotFoundException" => {
                    return GetCorsPolicyError::CorsPolicyNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetCorsPolicyError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCorsPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCorsPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCorsPolicyError {
    fn from(err: serde_json::error::Error) -> GetCorsPolicyError {
        GetCorsPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCorsPolicyError {
    fn from(err: CredentialsError) -> GetCorsPolicyError {
        GetCorsPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCorsPolicyError {
    fn from(err: HttpDispatchError) -> GetCorsPolicyError {
        GetCorsPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCorsPolicyError {
    fn from(err: io::Error) -> GetCorsPolicyError {
        GetCorsPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCorsPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCorsPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetCorsPolicyError::ContainerInUse(ref cause) => cause,
            GetCorsPolicyError::ContainerNotFound(ref cause) => cause,
            GetCorsPolicyError::CorsPolicyNotFound(ref cause) => cause,
            GetCorsPolicyError::InternalServerError(ref cause) => cause,
            GetCorsPolicyError::Validation(ref cause) => cause,
            GetCorsPolicyError::Credentials(ref err) => err.description(),
            GetCorsPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCorsPolicyError::ParseError(ref cause) => cause,
            GetCorsPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListContainers
#[derive(Debug, PartialEq)]
pub enum ListContainersError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
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

impl ListContainersError {
    pub fn from_response(res: BufferedHttpResponse) -> ListContainersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerError" => {
                    return ListContainersError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return ListContainersError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListContainersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListContainersError {
    fn from(err: serde_json::error::Error) -> ListContainersError {
        ListContainersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListContainersError {
    fn from(err: CredentialsError) -> ListContainersError {
        ListContainersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListContainersError {
    fn from(err: HttpDispatchError) -> ListContainersError {
        ListContainersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListContainersError {
    fn from(err: io::Error) -> ListContainersError {
        ListContainersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListContainersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListContainersError {
    fn description(&self) -> &str {
        match *self {
            ListContainersError::InternalServerError(ref cause) => cause,
            ListContainersError::Validation(ref cause) => cause,
            ListContainersError::Credentials(ref err) => err.description(),
            ListContainersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListContainersError::ParseError(ref cause) => cause,
            ListContainersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutContainerPolicy
#[derive(Debug, PartialEq)]
pub enum PutContainerPolicyError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
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

impl PutContainerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> PutContainerPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return PutContainerPolicyError::ContainerInUse(String::from(error_message))
                }
                "ContainerNotFoundException" => {
                    return PutContainerPolicyError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return PutContainerPolicyError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return PutContainerPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutContainerPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutContainerPolicyError {
    fn from(err: serde_json::error::Error) -> PutContainerPolicyError {
        PutContainerPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutContainerPolicyError {
    fn from(err: CredentialsError) -> PutContainerPolicyError {
        PutContainerPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutContainerPolicyError {
    fn from(err: HttpDispatchError) -> PutContainerPolicyError {
        PutContainerPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutContainerPolicyError {
    fn from(err: io::Error) -> PutContainerPolicyError {
        PutContainerPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutContainerPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutContainerPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutContainerPolicyError::ContainerInUse(ref cause) => cause,
            PutContainerPolicyError::ContainerNotFound(ref cause) => cause,
            PutContainerPolicyError::InternalServerError(ref cause) => cause,
            PutContainerPolicyError::Validation(ref cause) => cause,
            PutContainerPolicyError::Credentials(ref err) => err.description(),
            PutContainerPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutContainerPolicyError::ParseError(ref cause) => cause,
            PutContainerPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutCorsPolicy
#[derive(Debug, PartialEq)]
pub enum PutCorsPolicyError {
    /// <p>Resource already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>Could not perform an operation on a container that does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
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

impl PutCorsPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> PutCorsPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ContainerInUseException" => {
                    return PutCorsPolicyError::ContainerInUse(String::from(error_message))
                }
                "ContainerNotFoundException" => {
                    return PutCorsPolicyError::ContainerNotFound(String::from(error_message))
                }
                "InternalServerError" => {
                    return PutCorsPolicyError::InternalServerError(String::from(error_message))
                }
                "ValidationException" => {
                    return PutCorsPolicyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutCorsPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutCorsPolicyError {
    fn from(err: serde_json::error::Error) -> PutCorsPolicyError {
        PutCorsPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutCorsPolicyError {
    fn from(err: CredentialsError) -> PutCorsPolicyError {
        PutCorsPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutCorsPolicyError {
    fn from(err: HttpDispatchError) -> PutCorsPolicyError {
        PutCorsPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutCorsPolicyError {
    fn from(err: io::Error) -> PutCorsPolicyError {
        PutCorsPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutCorsPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutCorsPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutCorsPolicyError::ContainerInUse(ref cause) => cause,
            PutCorsPolicyError::ContainerNotFound(ref cause) => cause,
            PutCorsPolicyError::InternalServerError(ref cause) => cause,
            PutCorsPolicyError::Validation(ref cause) => cause,
            PutCorsPolicyError::Credentials(ref err) => err.description(),
            PutCorsPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutCorsPolicyError::ParseError(ref cause) => cause,
            PutCorsPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the MediaStore API. MediaStore clients implement this trait.
pub trait MediaStore {
    /// <p>Creates a storage container to hold objects. A container is similar to a bucket in the Amazon S3 service.</p>
    fn create_container(
        &self,
        input: CreateContainerInput,
    ) -> RusotoFuture<CreateContainerOutput, CreateContainerError>;

    /// <p>Deletes the specified container. Before you make a <code>DeleteContainer</code> request, delete any objects in the container or in any folders in the container. You can delete only empty containers. </p>
    fn delete_container(
        &self,
        input: DeleteContainerInput,
    ) -> RusotoFuture<DeleteContainerOutput, DeleteContainerError>;

    /// <p>Deletes the access policy that is associated with the specified container.</p>
    fn delete_container_policy(
        &self,
        input: DeleteContainerPolicyInput,
    ) -> RusotoFuture<DeleteContainerPolicyOutput, DeleteContainerPolicyError>;

    /// <p>Deletes the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:DeleteCorsPolicy</code> action. The container owner has this permission by default and can grant this permission to others.</p>
    fn delete_cors_policy(
        &self,
        input: DeleteCorsPolicyInput,
    ) -> RusotoFuture<DeleteCorsPolicyOutput, DeleteCorsPolicyError>;

    /// <p>Retrieves the properties of the requested container. This request is commonly used to retrieve the endpoint of a container. An endpoint is a value assigned by the service when a new container is created. A container's endpoint does not change after it has been assigned. The <code>DescribeContainer</code> request returns a single <code>Container</code> object based on <code>ContainerName</code>. To return all <code>Container</code> objects that are associated with a specified AWS account, use <a>ListContainers</a>.</p>
    fn describe_container(
        &self,
        input: DescribeContainerInput,
    ) -> RusotoFuture<DescribeContainerOutput, DescribeContainerError>;

    /// <p>Retrieves the access policy for the specified container. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p>
    fn get_container_policy(
        &self,
        input: GetContainerPolicyInput,
    ) -> RusotoFuture<GetContainerPolicyOutput, GetContainerPolicyError>;

    /// <p>Returns the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:GetCorsPolicy</code> action. By default, the container owner has this permission and can grant it to others.</p>
    fn get_cors_policy(
        &self,
        input: GetCorsPolicyInput,
    ) -> RusotoFuture<GetCorsPolicyOutput, GetCorsPolicyError>;

    /// <p>Lists the properties of all containers in AWS Elemental MediaStore. </p> <p>You can query to receive all the containers in one response. Or you can include the <code>MaxResults</code> parameter to receive a limited number of containers in each response. In this case, the response includes a token. To get the next set of containers, send the command again, this time with the <code>NextToken</code> parameter (with the returned token as its value). The next set of responses appears, with a token if there are still more containers to receive. </p> <p>See also <a>DescribeContainer</a>, which gets the properties of one container. </p>
    fn list_containers(
        &self,
        input: ListContainersInput,
    ) -> RusotoFuture<ListContainersOutput, ListContainersError>;

    /// <p>Creates an access policy for the specified container to restrict the users and clients that can access it. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p> <p>For this release of the REST API, you can create only one policy for a container. If you enter <code>PutContainerPolicy</code> twice, the second command modifies the existing policy. </p>
    fn put_container_policy(
        &self,
        input: PutContainerPolicyInput,
    ) -> RusotoFuture<PutContainerPolicyOutput, PutContainerPolicyError>;

    /// <p>Sets the cross-origin resource sharing (CORS) configuration on a container so that the container can service cross-origin requests. For example, you might want to enable a request whose origin is http://www.example.com to access your AWS Elemental MediaStore container at my.example.container.com by using the browser's XMLHttpRequest capability.</p> <p>To enable CORS on a container, you attach a CORS policy to the container. In the CORS policy, you configure rules that identify origins and the HTTP methods that can be executed on your container. The policy can contain up to 398,000 characters. You can add up to 100 rules to a CORS policy. If more than one rule applies, the service uses the first applicable rule listed.</p>
    fn put_cors_policy(
        &self,
        input: PutCorsPolicyInput,
    ) -> RusotoFuture<PutCorsPolicyOutput, PutCorsPolicyError>;
}
/// A client for the MediaStore API.
pub struct MediaStoreClient {
    client: Client,
    region: region::Region,
}

impl MediaStoreClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaStoreClient {
        MediaStoreClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaStoreClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        MediaStoreClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl MediaStore for MediaStoreClient {
    /// <p>Creates a storage container to hold objects. A container is similar to a bucket in the Amazon S3 service.</p>
    fn create_container(
        &self,
        input: CreateContainerInput,
    ) -> RusotoFuture<CreateContainerOutput, CreateContainerError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.CreateContainer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateContainerOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateContainerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified container. Before you make a <code>DeleteContainer</code> request, delete any objects in the container or in any folders in the container. You can delete only empty containers. </p>
    fn delete_container(
        &self,
        input: DeleteContainerInput,
    ) -> RusotoFuture<DeleteContainerOutput, DeleteContainerError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DeleteContainer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteContainerOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteContainerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the access policy that is associated with the specified container.</p>
    fn delete_container_policy(
        &self,
        input: DeleteContainerPolicyInput,
    ) -> RusotoFuture<DeleteContainerPolicyOutput, DeleteContainerPolicyError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DeleteContainerPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteContainerPolicyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteContainerPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:DeleteCorsPolicy</code> action. The container owner has this permission by default and can grant this permission to others.</p>
    fn delete_cors_policy(
        &self,
        input: DeleteCorsPolicyInput,
    ) -> RusotoFuture<DeleteCorsPolicyOutput, DeleteCorsPolicyError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DeleteCorsPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteCorsPolicyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCorsPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the properties of the requested container. This request is commonly used to retrieve the endpoint of a container. An endpoint is a value assigned by the service when a new container is created. A container's endpoint does not change after it has been assigned. The <code>DescribeContainer</code> request returns a single <code>Container</code> object based on <code>ContainerName</code>. To return all <code>Container</code> objects that are associated with a specified AWS account, use <a>ListContainers</a>.</p>
    fn describe_container(
        &self,
        input: DescribeContainerInput,
    ) -> RusotoFuture<DescribeContainerOutput, DescribeContainerError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DescribeContainer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeContainerOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeContainerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the access policy for the specified container. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p>
    fn get_container_policy(
        &self,
        input: GetContainerPolicyInput,
    ) -> RusotoFuture<GetContainerPolicyOutput, GetContainerPolicyError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.GetContainerPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetContainerPolicyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetContainerPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:GetCorsPolicy</code> action. By default, the container owner has this permission and can grant it to others.</p>
    fn get_cors_policy(
        &self,
        input: GetCorsPolicyInput,
    ) -> RusotoFuture<GetCorsPolicyOutput, GetCorsPolicyError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.GetCorsPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCorsPolicyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCorsPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the properties of all containers in AWS Elemental MediaStore. </p> <p>You can query to receive all the containers in one response. Or you can include the <code>MaxResults</code> parameter to receive a limited number of containers in each response. In this case, the response includes a token. To get the next set of containers, send the command again, this time with the <code>NextToken</code> parameter (with the returned token as its value). The next set of responses appears, with a token if there are still more containers to receive. </p> <p>See also <a>DescribeContainer</a>, which gets the properties of one container. </p>
    fn list_containers(
        &self,
        input: ListContainersInput,
    ) -> RusotoFuture<ListContainersOutput, ListContainersError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.ListContainers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListContainersOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListContainersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an access policy for the specified container to restrict the users and clients that can access it. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p> <p>For this release of the REST API, you can create only one policy for a container. If you enter <code>PutContainerPolicy</code> twice, the second command modifies the existing policy. </p>
    fn put_container_policy(
        &self,
        input: PutContainerPolicyInput,
    ) -> RusotoFuture<PutContainerPolicyOutput, PutContainerPolicyError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.PutContainerPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutContainerPolicyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutContainerPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the cross-origin resource sharing (CORS) configuration on a container so that the container can service cross-origin requests. For example, you might want to enable a request whose origin is http://www.example.com to access your AWS Elemental MediaStore container at my.example.container.com by using the browser's XMLHttpRequest capability.</p> <p>To enable CORS on a container, you attach a CORS policy to the container. In the CORS policy, you configure rules that identify origins and the HTTP methods that can be executed on your container. The policy can contain up to 398,000 characters. You can add up to 100 rules to a CORS policy. If more than one rule applies, the service uses the first applicable rule listed.</p>
    fn put_cors_policy(
        &self,
        input: PutCorsPolicyInput,
    ) -> RusotoFuture<PutCorsPolicyOutput, PutCorsPolicyError> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.PutCorsPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutCorsPolicyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutCorsPolicyError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
