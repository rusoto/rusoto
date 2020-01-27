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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>This section describes operations that you can perform on an AWS Elemental MediaStore container.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Container {
    /// <p>The Amazon Resource Name (ARN) of the container. The ARN has the following format:</p> <p>arn:aws:&lt;region&gt;:&lt;account that owns this container&gt;:container/&lt;name of container&gt; </p> <p>For example: arn:aws:mediastore:us-west-2:111122223333:container/movies </p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The state of access logging on the container. This value is <code>false</code> by default, indicating that AWS Elemental MediaStore does not send access logs to Amazon CloudWatch Logs. When you enable access logging on the container, MediaStore changes this value to <code>true</code>, indicating that the service delivers access logs for objects stored in that container to CloudWatch Logs.</p>
    #[serde(rename = "AccessLoggingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_logging_enabled: Option<bool>,
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
    pub allowed_headers: Vec<String>,
    /// <p>Identifies an HTTP method that the origin that is specified in the rule is allowed to execute.</p> <p>Each CORS rule must contain at least one <code>AllowedMethods</code> and one <code>AllowedOrigins</code> element.</p>
    #[serde(rename = "AllowedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<Vec<String>>,
    /// <p>One or more response headers that you want users to be able to access from their applications (for example, from a JavaScript <code>XMLHttpRequest</code> object).</p> <p>Each CORS rule must have at least one <code>AllowedOrigins</code> element. The string value can include only one wildcard character (*), for example, http://*.example.com. Additionally, you can specify only one wildcard character to allow cross-origin access for all origins.</p>
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: Vec<String>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateContainerInput {
    /// <p>The name for the container. The name must be from 1 to 255 characters. Container names must be unique to your AWS account within a specific region. As an example, you could create a container named <code>movies</code> in every region, as long as you don’t have an existing container with that name.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// <p>An array of key:value pairs that you define. These values can be anything that you want. Typically, the tag key represents a category (such as "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 tags to each container. For more information about tagging, including naming and usage conventions, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/tagging.html">Tagging Resources in MediaStore</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateContainerOutput {
    /// <p>ContainerARN: The Amazon Resource Name (ARN) of the newly created container. The ARN has the following format: arn:aws:&lt;region&gt;:&lt;account that owns this container&gt;:container/&lt;name of container&gt;. For example: arn:aws:mediastore:us-west-2:111122223333:container/movies </p> <p>ContainerName: The container name as specified in the request.</p> <p>CreationTime: Unix time stamp.</p> <p>Status: The status of container creation or deletion. The status is one of the following: <code>CREATING</code>, <code>ACTIVE</code>, or <code>DELETING</code>. While the service is creating the container, the status is <code>CREATING</code>. When an endpoint is available, the status changes to <code>ACTIVE</code>.</p> <p>The return value does not include the container's endpoint. To make downstream requests, you must obtain this value by using <a>DescribeContainer</a> or <a>ListContainers</a>.</p>
    #[serde(rename = "Container")]
    pub container: Container,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteContainerInput {
    /// <p>The name of the container to delete. </p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteContainerOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteContainerPolicyInput {
    /// <p>The name of the container that holds the policy.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteContainerPolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCorsPolicyInput {
    /// <p>The name of the container to remove the policy from.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCorsPolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLifecyclePolicyInput {
    /// <p>The name of the container that holds the object lifecycle policy.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLifecyclePolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeContainerInput {
    /// <p>The name of the container to query.</p>
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeContainerOutput {
    /// <p>The name of the queried container.</p>
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContainerPolicyInput {
    /// <p>The name of the container. </p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContainerPolicyOutput {
    /// <p>The contents of the access policy.</p>
    #[serde(rename = "Policy")]
    pub policy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCorsPolicyInput {
    /// <p>The name of the container that the policy is assigned to.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCorsPolicyOutput {
    /// <p>The CORS policy assigned to the container.</p>
    #[serde(rename = "CorsPolicy")]
    pub cors_policy: Vec<CorsRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLifecyclePolicyInput {
    /// <p>The name of the container that the object lifecycle policy is assigned to.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLifecyclePolicyOutput {
    /// <p>The object lifecycle policy that is assigned to the container.</p>
    #[serde(rename = "LifecyclePolicy")]
    pub lifecycle_policy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the container.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>An array of key:value pairs that are assigned to the container.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutContainerPolicyInput {
    /// <p>The name of the container.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// <p><p>The contents of the policy, which includes the following: </p> <ul> <li> <p>One <code>Version</code> tag</p> </li> <li> <p>One <code>Statement</code> tag that contains the standard tags for the policy.</p> </li> </ul></p>
    #[serde(rename = "Policy")]
    pub policy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutContainerPolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutCorsPolicyInput {
    /// <p>The name of the container that you want to assign the CORS policy to.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// <p>The CORS policy to apply to the container. </p>
    #[serde(rename = "CorsPolicy")]
    pub cors_policy: Vec<CorsRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutCorsPolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLifecyclePolicyInput {
    /// <p>The name of the container that you want to assign the object lifecycle policy to.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    /// <p>The object lifecycle policy to apply to the container.</p>
    #[serde(rename = "LifecyclePolicy")]
    pub lifecycle_policy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutLifecyclePolicyOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartAccessLoggingInput {
    /// <p>The name of the container that you want to start access logging on.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartAccessLoggingOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopAccessLoggingInput {
    /// <p>The name of the container that you want to stop access logging on.</p>
    #[serde(rename = "ContainerName")]
    pub container_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopAccessLoggingOutput {}

/// <p>A collection of tags associated with a container. Each tag consists of a key:value pair, which can be anything you define. Typically, the tag key represents a category (such as "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 tags to each container. For more information about tagging, including naming and usage conventions, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/tagging.html">Tagging Resources in MediaStore</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Part of the key:value pair that defines a tag. You can use a tag key to describe a category of information, such as "customer." Tag keys are case-sensitive.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Part of the key:value pair that defines a tag. You can use a tag value to describe a specific value within a category, such as "companyA" or "companyB." Tag values are case-sensitive.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the container. </p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>An array of key:value pairs that you want to add to the container. You need to specify only the tags that you want to add or update. For example, suppose a container already has two tags (customer:CompanyA and priority:High). You want to change the priority tag and also add a third tag (type:Contract). For TagResource, you specify the following tags: priority:Medium, type:Contract. The result is that your container has three tags: customer:CompanyA, priority:Medium, and type:Contract.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the container.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>A comma-separated list of keys for tags that you want to remove from the container. For example, if your container has two tags (customer:CompanyA and priority:High) and you want to remove one of the tags (priority:High), you specify the key for the tag that you want to remove (priority).</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

/// Errors returned by CreateContainer
#[derive(Debug, PartialEq)]
pub enum CreateContainerError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>A service limit has been exceeded.</p>
    LimitExceeded(String),
}

impl CreateContainerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateContainerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(CreateContainerError::ContainerInUse(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateContainerError::InternalServerError(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateContainerError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateContainerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateContainerError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            CreateContainerError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateContainerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateContainerError {}
/// Errors returned by DeleteContainer
#[derive(Debug, PartialEq)]
pub enum DeleteContainerError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl DeleteContainerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteContainerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(DeleteContainerError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(DeleteContainerError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteContainerError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteContainerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteContainerError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            DeleteContainerError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            DeleteContainerError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteContainerError {}
/// Errors returned by DeleteContainerPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteContainerPolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The policy that you specified in the request does not exist.</p>
    PolicyNotFound(String),
}

impl DeleteContainerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteContainerPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(DeleteContainerPolicyError::ContainerInUse(
                        err.msg,
                    ))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(DeleteContainerPolicyError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteContainerPolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DeleteContainerPolicyError::PolicyNotFound(
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
impl fmt::Display for DeleteContainerPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteContainerPolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            DeleteContainerPolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            DeleteContainerPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteContainerPolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteContainerPolicyError {}
/// Errors returned by DeleteCorsPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteCorsPolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The CORS policy that you specified in the request does not exist.</p>
    CorsPolicyNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl DeleteCorsPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCorsPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(DeleteCorsPolicyError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(DeleteCorsPolicyError::ContainerNotFound(err.msg))
                }
                "CorsPolicyNotFoundException" => {
                    return RusotoError::Service(DeleteCorsPolicyError::CorsPolicyNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteCorsPolicyError::InternalServerError(
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
impl fmt::Display for DeleteCorsPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCorsPolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            DeleteCorsPolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            DeleteCorsPolicyError::CorsPolicyNotFound(ref cause) => write!(f, "{}", cause),
            DeleteCorsPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCorsPolicyError {}
/// Errors returned by DeleteLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteLifecyclePolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The policy that you specified in the request does not exist.</p>
    PolicyNotFound(String),
}

impl DeleteLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::ContainerInUse(
                        err.msg,
                    ))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::PolicyNotFound(
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
impl fmt::Display for DeleteLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLifecyclePolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            DeleteLifecyclePolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLifecyclePolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteLifecyclePolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLifecyclePolicyError {}
/// Errors returned by DescribeContainer
#[derive(Debug, PartialEq)]
pub enum DescribeContainerError {
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl DescribeContainerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeContainerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerNotFoundException" => {
                    return RusotoError::Service(DescribeContainerError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeContainerError::InternalServerError(
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
impl fmt::Display for DescribeContainerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeContainerError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeContainerError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeContainerError {}
/// Errors returned by GetContainerPolicy
#[derive(Debug, PartialEq)]
pub enum GetContainerPolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The policy that you specified in the request does not exist.</p>
    PolicyNotFound(String),
}

impl GetContainerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContainerPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(GetContainerPolicyError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(GetContainerPolicyError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetContainerPolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(GetContainerPolicyError::PolicyNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetContainerPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContainerPolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            GetContainerPolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            GetContainerPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetContainerPolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContainerPolicyError {}
/// Errors returned by GetCorsPolicy
#[derive(Debug, PartialEq)]
pub enum GetCorsPolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The CORS policy that you specified in the request does not exist.</p>
    CorsPolicyNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl GetCorsPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCorsPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(GetCorsPolicyError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(GetCorsPolicyError::ContainerNotFound(err.msg))
                }
                "CorsPolicyNotFoundException" => {
                    return RusotoError::Service(GetCorsPolicyError::CorsPolicyNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetCorsPolicyError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCorsPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCorsPolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            GetCorsPolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            GetCorsPolicyError::CorsPolicyNotFound(ref cause) => write!(f, "{}", cause),
            GetCorsPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCorsPolicyError {}
/// Errors returned by GetLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum GetLifecyclePolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>The policy that you specified in the request does not exist.</p>
    PolicyNotFound(String),
}

impl GetLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetLifecyclePolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::PolicyNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLifecyclePolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLifecyclePolicyError {}
/// Errors returned by ListContainers
#[derive(Debug, PartialEq)]
pub enum ListContainersError {
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl ListContainersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContainersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListContainersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListContainersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListContainersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListContainersError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(ListTagsForResourceError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutContainerPolicy
#[derive(Debug, PartialEq)]
pub enum PutContainerPolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl PutContainerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutContainerPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(PutContainerPolicyError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(PutContainerPolicyError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutContainerPolicyError::InternalServerError(
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
impl fmt::Display for PutContainerPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutContainerPolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            PutContainerPolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            PutContainerPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutContainerPolicyError {}
/// Errors returned by PutCorsPolicy
#[derive(Debug, PartialEq)]
pub enum PutCorsPolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl PutCorsPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutCorsPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(PutCorsPolicyError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(PutCorsPolicyError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutCorsPolicyError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutCorsPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutCorsPolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            PutCorsPolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            PutCorsPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutCorsPolicyError {}
/// Errors returned by PutLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum PutLifecyclePolicyError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl PutLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(PutLifecyclePolicyError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(PutLifecyclePolicyError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutLifecyclePolicyError::InternalServerError(
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
impl fmt::Display for PutLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLifecyclePolicyError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            PutLifecyclePolicyError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            PutLifecyclePolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLifecyclePolicyError {}
/// Errors returned by StartAccessLogging
#[derive(Debug, PartialEq)]
pub enum StartAccessLoggingError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl StartAccessLoggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartAccessLoggingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(StartAccessLoggingError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(StartAccessLoggingError::ContainerNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartAccessLoggingError::InternalServerError(
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
impl fmt::Display for StartAccessLoggingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartAccessLoggingError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            StartAccessLoggingError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            StartAccessLoggingError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartAccessLoggingError {}
/// Errors returned by StopAccessLogging
#[derive(Debug, PartialEq)]
pub enum StopAccessLoggingError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl StopAccessLoggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopAccessLoggingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(StopAccessLoggingError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(StopAccessLoggingError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StopAccessLoggingError::InternalServerError(
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
impl fmt::Display for StopAccessLoggingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopAccessLoggingError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            StopAccessLoggingError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            StopAccessLoggingError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopAccessLoggingError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(TagResourceError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            TagResourceError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The container that you specified in the request already exists or is being updated.</p>
    ContainerInUse(String),
    /// <p>The container that you specified in the request does not exist.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ContainerInUseException" => {
                    return RusotoError::Service(UntagResourceError::ContainerInUse(err.msg))
                }
                "ContainerNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::ContainerInUse(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the MediaStore API. MediaStore clients implement this trait.
#[async_trait]
pub trait MediaStore {
    /// <p>Creates a storage container to hold objects. A container is similar to a bucket in the Amazon S3 service.</p>
    async fn create_container(
        &self,
        input: CreateContainerInput,
    ) -> Result<CreateContainerOutput, RusotoError<CreateContainerError>>;

    /// <p>Deletes the specified container. Before you make a <code>DeleteContainer</code> request, delete any objects in the container or in any folders in the container. You can delete only empty containers. </p>
    async fn delete_container(
        &self,
        input: DeleteContainerInput,
    ) -> Result<DeleteContainerOutput, RusotoError<DeleteContainerError>>;

    /// <p>Deletes the access policy that is associated with the specified container.</p>
    async fn delete_container_policy(
        &self,
        input: DeleteContainerPolicyInput,
    ) -> Result<DeleteContainerPolicyOutput, RusotoError<DeleteContainerPolicyError>>;

    /// <p>Deletes the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:DeleteCorsPolicy</code> action. The container owner has this permission by default and can grant this permission to others.</p>
    async fn delete_cors_policy(
        &self,
        input: DeleteCorsPolicyInput,
    ) -> Result<DeleteCorsPolicyOutput, RusotoError<DeleteCorsPolicyError>>;

    /// <p>Removes an object lifecycle policy from a container. It takes up to 20 minutes for the change to take effect.</p>
    async fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyInput,
    ) -> Result<DeleteLifecyclePolicyOutput, RusotoError<DeleteLifecyclePolicyError>>;

    /// <p>Retrieves the properties of the requested container. This request is commonly used to retrieve the endpoint of a container. An endpoint is a value assigned by the service when a new container is created. A container's endpoint does not change after it has been assigned. The <code>DescribeContainer</code> request returns a single <code>Container</code> object based on <code>ContainerName</code>. To return all <code>Container</code> objects that are associated with a specified AWS account, use <a>ListContainers</a>.</p>
    async fn describe_container(
        &self,
        input: DescribeContainerInput,
    ) -> Result<DescribeContainerOutput, RusotoError<DescribeContainerError>>;

    /// <p>Retrieves the access policy for the specified container. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p>
    async fn get_container_policy(
        &self,
        input: GetContainerPolicyInput,
    ) -> Result<GetContainerPolicyOutput, RusotoError<GetContainerPolicyError>>;

    /// <p>Returns the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:GetCorsPolicy</code> action. By default, the container owner has this permission and can grant it to others.</p>
    async fn get_cors_policy(
        &self,
        input: GetCorsPolicyInput,
    ) -> Result<GetCorsPolicyOutput, RusotoError<GetCorsPolicyError>>;

    /// <p>Retrieves the object lifecycle policy that is assigned to a container.</p>
    async fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyInput,
    ) -> Result<GetLifecyclePolicyOutput, RusotoError<GetLifecyclePolicyError>>;

    /// <p>Lists the properties of all containers in AWS Elemental MediaStore. </p> <p>You can query to receive all the containers in one response. Or you can include the <code>MaxResults</code> parameter to receive a limited number of containers in each response. In this case, the response includes a token. To get the next set of containers, send the command again, this time with the <code>NextToken</code> parameter (with the returned token as its value). The next set of responses appears, with a token if there are still more containers to receive. </p> <p>See also <a>DescribeContainer</a>, which gets the properties of one container. </p>
    async fn list_containers(
        &self,
        input: ListContainersInput,
    ) -> Result<ListContainersOutput, RusotoError<ListContainersError>>;

    /// <p>Returns a list of the tags assigned to the specified container. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Creates an access policy for the specified container to restrict the users and clients that can access it. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p> <p>For this release of the REST API, you can create only one policy for a container. If you enter <code>PutContainerPolicy</code> twice, the second command modifies the existing policy. </p>
    async fn put_container_policy(
        &self,
        input: PutContainerPolicyInput,
    ) -> Result<PutContainerPolicyOutput, RusotoError<PutContainerPolicyError>>;

    /// <p>Sets the cross-origin resource sharing (CORS) configuration on a container so that the container can service cross-origin requests. For example, you might want to enable a request whose origin is http://www.example.com to access your AWS Elemental MediaStore container at my.example.container.com by using the browser's XMLHttpRequest capability.</p> <p>To enable CORS on a container, you attach a CORS policy to the container. In the CORS policy, you configure rules that identify origins and the HTTP methods that can be executed on your container. The policy can contain up to 398,000 characters. You can add up to 100 rules to a CORS policy. If more than one rule applies, the service uses the first applicable rule listed.</p> <p>To learn more about CORS, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/cors-policy.html">Cross-Origin Resource Sharing (CORS) in AWS Elemental MediaStore</a>.</p>
    async fn put_cors_policy(
        &self,
        input: PutCorsPolicyInput,
    ) -> Result<PutCorsPolicyOutput, RusotoError<PutCorsPolicyError>>;

    /// <p>Writes an object lifecycle policy to a container. If the container already has an object lifecycle policy, the service replaces the existing policy with the new policy. It takes up to 20 minutes for the change to take effect.</p> <p>For information about how to construct an object lifecycle policy, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/policies-object-lifecycle-components.html">Components of an Object Lifecycle Policy</a>.</p>
    async fn put_lifecycle_policy(
        &self,
        input: PutLifecyclePolicyInput,
    ) -> Result<PutLifecyclePolicyOutput, RusotoError<PutLifecyclePolicyError>>;

    /// <p>Starts access logging on the specified container. When you enable access logging on a container, MediaStore delivers access logs for objects stored in that container to Amazon CloudWatch Logs.</p>
    async fn start_access_logging(
        &self,
        input: StartAccessLoggingInput,
    ) -> Result<StartAccessLoggingOutput, RusotoError<StartAccessLoggingError>>;

    /// <p>Stops access logging on the specified container. When you stop access logging on a container, MediaStore stops sending access logs to Amazon CloudWatch Logs. These access logs are not saved and are not retrievable.</p>
    async fn stop_access_logging(
        &self,
        input: StopAccessLoggingInput,
    ) -> Result<StopAccessLoggingOutput, RusotoError<StopAccessLoggingError>>;

    /// <p>Adds tags to the specified AWS Elemental MediaStore container. Tags are key:value pairs that you can associate with AWS resources. For example, the tag key might be "customer" and the tag value might be "companyA." You can specify one or more tags to add to each container. You can add up to 50 tags to each container. For more information about tagging, including naming and usage conventions, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/tagging.html">Tagging Resources in MediaStore</a>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p>Removes tags from the specified container. You can specify one or more tags to remove. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;
}
/// A client for the MediaStore API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaStoreClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaStoreClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaStoreClient {
        MediaStoreClient { client, region }
    }
}

#[async_trait]
impl MediaStore for MediaStoreClient {
    /// <p>Creates a storage container to hold objects. A container is similar to a bucket in the Amazon S3 service.</p>
    async fn create_container(
        &self,
        input: CreateContainerInput,
    ) -> Result<CreateContainerOutput, RusotoError<CreateContainerError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.CreateContainer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateContainerOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateContainerError::from_response(response))
        }
    }

    /// <p>Deletes the specified container. Before you make a <code>DeleteContainer</code> request, delete any objects in the container or in any folders in the container. You can delete only empty containers. </p>
    async fn delete_container(
        &self,
        input: DeleteContainerInput,
    ) -> Result<DeleteContainerOutput, RusotoError<DeleteContainerError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DeleteContainer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteContainerOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteContainerError::from_response(response))
        }
    }

    /// <p>Deletes the access policy that is associated with the specified container.</p>
    async fn delete_container_policy(
        &self,
        input: DeleteContainerPolicyInput,
    ) -> Result<DeleteContainerPolicyOutput, RusotoError<DeleteContainerPolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DeleteContainerPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteContainerPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteContainerPolicyError::from_response(response))
        }
    }

    /// <p>Deletes the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:DeleteCorsPolicy</code> action. The container owner has this permission by default and can grant this permission to others.</p>
    async fn delete_cors_policy(
        &self,
        input: DeleteCorsPolicyInput,
    ) -> Result<DeleteCorsPolicyOutput, RusotoError<DeleteCorsPolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DeleteCorsPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteCorsPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCorsPolicyError::from_response(response))
        }
    }

    /// <p>Removes an object lifecycle policy from a container. It takes up to 20 minutes for the change to take effect.</p>
    async fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyInput,
    ) -> Result<DeleteLifecyclePolicyOutput, RusotoError<DeleteLifecyclePolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DeleteLifecyclePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLifecyclePolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Retrieves the properties of the requested container. This request is commonly used to retrieve the endpoint of a container. An endpoint is a value assigned by the service when a new container is created. A container's endpoint does not change after it has been assigned. The <code>DescribeContainer</code> request returns a single <code>Container</code> object based on <code>ContainerName</code>. To return all <code>Container</code> objects that are associated with a specified AWS account, use <a>ListContainers</a>.</p>
    async fn describe_container(
        &self,
        input: DescribeContainerInput,
    ) -> Result<DescribeContainerOutput, RusotoError<DescribeContainerError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.DescribeContainer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeContainerOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeContainerError::from_response(response))
        }
    }

    /// <p>Retrieves the access policy for the specified container. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p>
    async fn get_container_policy(
        &self,
        input: GetContainerPolicyInput,
    ) -> Result<GetContainerPolicyOutput, RusotoError<GetContainerPolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.GetContainerPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetContainerPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetContainerPolicyError::from_response(response))
        }
    }

    /// <p>Returns the cross-origin resource sharing (CORS) configuration information that is set for the container.</p> <p>To use this operation, you must have permission to perform the <code>MediaStore:GetCorsPolicy</code> action. By default, the container owner has this permission and can grant it to others.</p>
    async fn get_cors_policy(
        &self,
        input: GetCorsPolicyInput,
    ) -> Result<GetCorsPolicyOutput, RusotoError<GetCorsPolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.GetCorsPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetCorsPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCorsPolicyError::from_response(response))
        }
    }

    /// <p>Retrieves the object lifecycle policy that is assigned to a container.</p>
    async fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyInput,
    ) -> Result<GetLifecyclePolicyOutput, RusotoError<GetLifecyclePolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.GetLifecyclePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLifecyclePolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Lists the properties of all containers in AWS Elemental MediaStore. </p> <p>You can query to receive all the containers in one response. Or you can include the <code>MaxResults</code> parameter to receive a limited number of containers in each response. In this case, the response includes a token. To get the next set of containers, send the command again, this time with the <code>NextToken</code> parameter (with the returned token as its value). The next set of responses appears, with a token if there are still more containers to receive. </p> <p>See also <a>DescribeContainer</a>, which gets the properties of one container. </p>
    async fn list_containers(
        &self,
        input: ListContainersInput,
    ) -> Result<ListContainersOutput, RusotoError<ListContainersError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.ListContainers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListContainersOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListContainersError::from_response(response))
        }
    }

    /// <p>Returns a list of the tags assigned to the specified container. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Creates an access policy for the specified container to restrict the users and clients that can access it. For information about the data that is included in an access policy, see the <a href="https://aws.amazon.com/documentation/iam/">AWS Identity and Access Management User Guide</a>.</p> <p>For this release of the REST API, you can create only one policy for a container. If you enter <code>PutContainerPolicy</code> twice, the second command modifies the existing policy. </p>
    async fn put_container_policy(
        &self,
        input: PutContainerPolicyInput,
    ) -> Result<PutContainerPolicyOutput, RusotoError<PutContainerPolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.PutContainerPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutContainerPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutContainerPolicyError::from_response(response))
        }
    }

    /// <p>Sets the cross-origin resource sharing (CORS) configuration on a container so that the container can service cross-origin requests. For example, you might want to enable a request whose origin is http://www.example.com to access your AWS Elemental MediaStore container at my.example.container.com by using the browser's XMLHttpRequest capability.</p> <p>To enable CORS on a container, you attach a CORS policy to the container. In the CORS policy, you configure rules that identify origins and the HTTP methods that can be executed on your container. The policy can contain up to 398,000 characters. You can add up to 100 rules to a CORS policy. If more than one rule applies, the service uses the first applicable rule listed.</p> <p>To learn more about CORS, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/cors-policy.html">Cross-Origin Resource Sharing (CORS) in AWS Elemental MediaStore</a>.</p>
    async fn put_cors_policy(
        &self,
        input: PutCorsPolicyInput,
    ) -> Result<PutCorsPolicyOutput, RusotoError<PutCorsPolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.PutCorsPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutCorsPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutCorsPolicyError::from_response(response))
        }
    }

    /// <p>Writes an object lifecycle policy to a container. If the container already has an object lifecycle policy, the service replaces the existing policy with the new policy. It takes up to 20 minutes for the change to take effect.</p> <p>For information about how to construct an object lifecycle policy, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/policies-object-lifecycle-components.html">Components of an Object Lifecycle Policy</a>.</p>
    async fn put_lifecycle_policy(
        &self,
        input: PutLifecyclePolicyInput,
    ) -> Result<PutLifecyclePolicyOutput, RusotoError<PutLifecyclePolicyError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.PutLifecyclePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutLifecyclePolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Starts access logging on the specified container. When you enable access logging on a container, MediaStore delivers access logs for objects stored in that container to Amazon CloudWatch Logs.</p>
    async fn start_access_logging(
        &self,
        input: StartAccessLoggingInput,
    ) -> Result<StartAccessLoggingOutput, RusotoError<StartAccessLoggingError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.StartAccessLogging");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StartAccessLoggingOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartAccessLoggingError::from_response(response))
        }
    }

    /// <p>Stops access logging on the specified container. When you stop access logging on a container, MediaStore stops sending access logs to Amazon CloudWatch Logs. These access logs are not saved and are not retrievable.</p>
    async fn stop_access_logging(
        &self,
        input: StopAccessLoggingInput,
    ) -> Result<StopAccessLoggingOutput, RusotoError<StopAccessLoggingError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.StopAccessLogging");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StopAccessLoggingOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopAccessLoggingError::from_response(response))
        }
    }

    /// <p>Adds tags to the specified AWS Elemental MediaStore container. Tags are key:value pairs that you can associate with AWS resources. For example, the tag key might be "customer" and the tag value might be "companyA." You can specify one or more tags to add to each container. You can add up to 50 tags to each container. For more information about tagging, including naming and usage conventions, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/tagging.html">Tagging Resources in MediaStore</a>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from the specified container. You can specify one or more tags to remove. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "mediastore", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MediaStore_20170901.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
