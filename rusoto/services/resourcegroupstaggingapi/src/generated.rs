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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Details of the common errors that all actions return.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailureInfo {
    /// <p>The code of the common error. Valid values include <code>InternalServiceException</code>, <code>InvalidParameterException</code>, and any valid error code returned by the AWS service that hosts the resource that you want to tag.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The message of the common error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The HTTP status code of the common error.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourcesInput {
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a <code>PaginationToken</code>, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p><p>The constraints on the resources that you want returned. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all tagged Amazon EC2 resources (which includes tagged EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p> <p>The string for each service name and resource type is the same as that embedded in a resource&#39;s Amazon Resource Name (ARN). Consult the <i>AWS General Reference</i> for the following:</p> <ul> <li> <p>For a list of service name strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a>.</p> </li> <li> <p>For resource type strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example ARNs</a>.</p> </li> <li> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> </li> </ul></p>
    #[serde(rename = "ResourceTypeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    /// <p>A limit that restricts the number of resources returned by GetResources in paginated output. You can set ResourcesPerPage to a minimum of 1 item and the maximum of 50 items. </p>
    #[serde(rename = "ResourcesPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_per_page: Option<i64>,
    /// <p>A list of tags (keys and values). A request can include up to 50 keys, and each key can include up to 20 values.</p> <p>If you specify multiple filters connected by an AND operator in a single request, the response returns only those resources that are associated with every specified filter.</p> <p>If you specify multiple filters connected by an OR operator in a single request, the response returns all resources that are associated with at least one or possibly more of the specified filters.</p>
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
    /// <p>A limit that restricts the number of tags (key and value pairs) returned by GetResources in paginated output. A resource with no tags is counted as having one tag (one key and value pair).</p> <p> <code>GetResources</code> does not split a resource and its associated tags across pages. If the specified <code>TagsPerPage</code> would cause such a break, a <code>PaginationToken</code> is returned in place of the affected resource and its tags. Use that token in another request to get the remaining data. For example, if you specify a <code>TagsPerPage</code> of <code>100</code> and the account has 22 resources with 10 tags each (meaning that each resource has 10 key and value pairs), the output will consist of 3 pages, with the first page displaying the first 10 resources, each with its 10 tags, the second page displaying the next 10 resources each with its 10 tags, and the third page displaying the remaining 2 resources, each with its 10 tags.</p> <p/> <p>You can set <code>TagsPerPage</code> to a minimum of 100 items and the maximum of 500 items.</p>
    #[serde(rename = "TagsPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_per_page: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetResourcesOutput {
    /// <p>A string that indicates that the response contains more data than can be returned in a single response. To receive additional data, specify this string for the <code>PaginationToken</code> value in a subsequent request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of resource ARNs and the tags (keys and values) associated with each.</p>
    #[serde(rename = "ResourceTagMappingList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_mapping_list: Option<Vec<ResourceTagMapping>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagKeysInput {
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a PaginationToken, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetTagKeysOutput {
    /// <p>A string that indicates that the response contains more data than can be returned in a single response. To receive additional data, specify this string for the <code>PaginationToken</code> value in a subsequent request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of all tag keys in the AWS account.</p>
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagValuesInput {
    /// <p>The key for which you want to list all existing values in the specified region for the AWS account.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a PaginationToken, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetTagValuesOutput {
    /// <p>A string that indicates that the response contains more data than can be returned in a single response. To receive additional data, specify this string for the <code>PaginationToken</code> value in a subsequent request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of all tag values for the specified key in the AWS account.</p>
    #[serde(rename = "TagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

/// <p>A list of resource ARNs and the tags (keys and values) that are associated with each.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceTagMapping {
    /// <p>An array of resource ARN(s).</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The tags that have been applied to one or more AWS resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The metadata that you apply to AWS resources to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-basics">Tag Basics</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A list of tags (keys and values) that are used to specify the associated resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagFilter {
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourcesInput {
    /// <p>A list of ARNs. An ARN (Amazon Resource Name) uniquely identifies a resource. You can specify a minimum of 1 and a maximum of 20 ARNs (resources) to tag. An ARN can be set to a maximum of 1600 characters. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "ResourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>The tags that you want to add to the specified resources. A tag consists of a key and a value that you define.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagResourcesOutput {
    /// <p>Details of resources that could not be tagged. An error code, status code, and error message are returned for each failed item.</p>
    #[serde(rename = "FailedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourcesInput {
    /// <p>A list of ARNs. An ARN (Amazon Resource Name) uniquely identifies a resource. You can specify a minimum of 1 and a maximum of 20 ARNs (resources) to untag. An ARN can be set to a maximum of 1600 characters. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "ResourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>A list of the tag keys that you want to remove from the specified resources.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagResourcesOutput {
    /// <p>Details of resources that could not be untagged. An error code, status code, and error message are returned for each failed item.</p>
    #[serde(rename = "FailedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

/// Errors returned by GetResources
#[derive(Debug, PartialEq)]
pub enum GetResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetResourcesError {
    pub fn from_body(body: &str) -> GetResourcesError {
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
                    "InternalServiceException" => {
                        GetResourcesError::InternalService(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetResourcesError::InvalidParameter(String::from(error_message))
                    }
                    "PaginationTokenExpiredException" => {
                        GetResourcesError::PaginationTokenExpired(String::from(error_message))
                    }
                    "ThrottledException" => {
                        GetResourcesError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetResourcesError::Validation(error_message.to_string())
                    }
                    _ => GetResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetResourcesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetResourcesError {
    fn from(err: serde_json::error::Error) -> GetResourcesError {
        GetResourcesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourcesError {
    fn from(err: CredentialsError) -> GetResourcesError {
        GetResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourcesError {
    fn from(err: HttpDispatchError) -> GetResourcesError {
        GetResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourcesError {
    fn from(err: io::Error) -> GetResourcesError {
        GetResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourcesError {
    fn description(&self) -> &str {
        match *self {
            GetResourcesError::InternalService(ref cause) => cause,
            GetResourcesError::InvalidParameter(ref cause) => cause,
            GetResourcesError::PaginationTokenExpired(ref cause) => cause,
            GetResourcesError::Throttled(ref cause) => cause,
            GetResourcesError::Validation(ref cause) => cause,
            GetResourcesError::Credentials(ref err) => err.description(),
            GetResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTagKeys
#[derive(Debug, PartialEq)]
pub enum GetTagKeysError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTagKeysError {
    pub fn from_body(body: &str) -> GetTagKeysError {
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
                    "InternalServiceException" => {
                        GetTagKeysError::InternalService(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetTagKeysError::InvalidParameter(String::from(error_message))
                    }
                    "PaginationTokenExpiredException" => {
                        GetTagKeysError::PaginationTokenExpired(String::from(error_message))
                    }
                    "ThrottledException" => GetTagKeysError::Throttled(String::from(error_message)),
                    "ValidationException" => GetTagKeysError::Validation(error_message.to_string()),
                    _ => GetTagKeysError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTagKeysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTagKeysError {
    fn from(err: serde_json::error::Error) -> GetTagKeysError {
        GetTagKeysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTagKeysError {
    fn from(err: CredentialsError) -> GetTagKeysError {
        GetTagKeysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTagKeysError {
    fn from(err: HttpDispatchError) -> GetTagKeysError {
        GetTagKeysError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTagKeysError {
    fn from(err: io::Error) -> GetTagKeysError {
        GetTagKeysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTagKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagKeysError {
    fn description(&self) -> &str {
        match *self {
            GetTagKeysError::InternalService(ref cause) => cause,
            GetTagKeysError::InvalidParameter(ref cause) => cause,
            GetTagKeysError::PaginationTokenExpired(ref cause) => cause,
            GetTagKeysError::Throttled(ref cause) => cause,
            GetTagKeysError::Validation(ref cause) => cause,
            GetTagKeysError::Credentials(ref err) => err.description(),
            GetTagKeysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTagKeysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTagValues
#[derive(Debug, PartialEq)]
pub enum GetTagValuesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTagValuesError {
    pub fn from_body(body: &str) -> GetTagValuesError {
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
                    "InternalServiceException" => {
                        GetTagValuesError::InternalService(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetTagValuesError::InvalidParameter(String::from(error_message))
                    }
                    "PaginationTokenExpiredException" => {
                        GetTagValuesError::PaginationTokenExpired(String::from(error_message))
                    }
                    "ThrottledException" => {
                        GetTagValuesError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTagValuesError::Validation(error_message.to_string())
                    }
                    _ => GetTagValuesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTagValuesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTagValuesError {
    fn from(err: serde_json::error::Error) -> GetTagValuesError {
        GetTagValuesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTagValuesError {
    fn from(err: CredentialsError) -> GetTagValuesError {
        GetTagValuesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTagValuesError {
    fn from(err: HttpDispatchError) -> GetTagValuesError {
        GetTagValuesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTagValuesError {
    fn from(err: io::Error) -> GetTagValuesError {
        GetTagValuesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTagValuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagValuesError {
    fn description(&self) -> &str {
        match *self {
            GetTagValuesError::InternalService(ref cause) => cause,
            GetTagValuesError::InvalidParameter(ref cause) => cause,
            GetTagValuesError::PaginationTokenExpired(ref cause) => cause,
            GetTagValuesError::Throttled(ref cause) => cause,
            GetTagValuesError::Validation(ref cause) => cause,
            GetTagValuesError::Credentials(ref err) => err.description(),
            GetTagValuesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTagValuesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResources
#[derive(Debug, PartialEq)]
pub enum TagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagResourcesError {
    pub fn from_body(body: &str) -> TagResourcesError {
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
                    "InternalServiceException" => {
                        TagResourcesError::InternalService(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        TagResourcesError::InvalidParameter(String::from(error_message))
                    }
                    "ThrottledException" => {
                        TagResourcesError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourcesError::Validation(error_message.to_string())
                    }
                    _ => TagResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourcesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourcesError {
    fn from(err: serde_json::error::Error) -> TagResourcesError {
        TagResourcesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourcesError {
    fn from(err: CredentialsError) -> TagResourcesError {
        TagResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourcesError {
    fn from(err: HttpDispatchError) -> TagResourcesError {
        TagResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourcesError {
    fn from(err: io::Error) -> TagResourcesError {
        TagResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourcesError {
    fn description(&self) -> &str {
        match *self {
            TagResourcesError::InternalService(ref cause) => cause,
            TagResourcesError::InvalidParameter(ref cause) => cause,
            TagResourcesError::Throttled(ref cause) => cause,
            TagResourcesError::Validation(ref cause) => cause,
            TagResourcesError::Credentials(ref err) => err.description(),
            TagResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResources
#[derive(Debug, PartialEq)]
pub enum UntagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagResourcesError {
    pub fn from_body(body: &str) -> UntagResourcesError {
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
                    "InternalServiceException" => {
                        UntagResourcesError::InternalService(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UntagResourcesError::InvalidParameter(String::from(error_message))
                    }
                    "ThrottledException" => {
                        UntagResourcesError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourcesError::Validation(error_message.to_string())
                    }
                    _ => UntagResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourcesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourcesError {
    fn from(err: serde_json::error::Error) -> UntagResourcesError {
        UntagResourcesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourcesError {
    fn from(err: CredentialsError) -> UntagResourcesError {
        UntagResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourcesError {
    fn from(err: HttpDispatchError) -> UntagResourcesError {
        UntagResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourcesError {
    fn from(err: io::Error) -> UntagResourcesError {
        UntagResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourcesError {
    fn description(&self) -> &str {
        match *self {
            UntagResourcesError::InternalService(ref cause) => cause,
            UntagResourcesError::InvalidParameter(ref cause) => cause,
            UntagResourcesError::Throttled(ref cause) => cause,
            UntagResourcesError::Validation(ref cause) => cause,
            UntagResourcesError::Credentials(ref err) => err.description(),
            UntagResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Resource Groups Tagging API API. AWS Resource Groups Tagging API clients implement this trait.
pub trait ResourceGroupsTaggingApi {
    /// <p>Returns all the tagged resources that are associated with the specified tags (keys and values) located in the specified region for the AWS account. The tags and the resource types that you specify in the request are known as <i>filters</i>. The response includes all tags that are associated with the requested resources. If no filter is provided, this action returns a paginated resource list with the associated tags.</p>
    fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> RusotoFuture<GetResourcesOutput, GetResourcesError>;

    /// <p>Returns all tag keys in the specified region for the AWS account.</p>
    fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> RusotoFuture<GetTagKeysOutput, GetTagKeysError>;

    /// <p>Returns all tag values for the specified key in the specified region for the AWS account.</p>
    fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> RusotoFuture<GetTagValuesOutput, GetTagValuesError>;

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of resources that support tagging, see <a href="http://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/supported-resources.html">Supported Resources</a> in the <i>AWS Resource Groups and Tag Editor User Guide</i>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions">Tag Restrictions</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see <a href="http://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups and Tag Editor User Guide</i>.</p> </li> </ul></p>
    fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> RusotoFuture<TagResourcesOutput, TagResourcesError>;

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see <a href="http://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups and Tag Editor User Guide</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> </ul></p>
    fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> RusotoFuture<UntagResourcesOutput, UntagResourcesError>;
}
/// A client for the AWS Resource Groups Tagging API API.
pub struct ResourceGroupsTaggingApiClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ResourceGroupsTaggingApiClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ResourceGroupsTaggingApiClient {
        ResourceGroupsTaggingApiClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ResourceGroupsTaggingApiClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ResourceGroupsTaggingApiClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> ResourceGroupsTaggingApi for ResourceGroupsTaggingApiClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Returns all the tagged resources that are associated with the specified tags (keys and values) located in the specified region for the AWS account. The tags and the resource types that you specify in the request are known as <i>filters</i>. The response includes all tags that are associated with the requested resources. If no filter is provided, this action returns a paginated resource list with the associated tags.</p>
    fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> RusotoFuture<GetResourcesOutput, GetResourcesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetResourcesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns all tag keys in the specified region for the AWS account.</p>
    fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> RusotoFuture<GetTagKeysOutput, GetTagKeysError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetTagKeysOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTagKeysError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns all tag values for the specified key in the specified region for the AWS account.</p>
    fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> RusotoFuture<GetTagValuesOutput, GetTagValuesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagValues",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetTagValuesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTagValuesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of resources that support tagging, see <a href="http://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/supported-resources.html">Supported Resources</a> in the <i>AWS Resource Groups and Tag Editor User Guide</i>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions">Tag Restrictions</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see <a href="http://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups and Tag Editor User Guide</i>.</p> </li> </ul></p>
    fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> RusotoFuture<TagResourcesOutput, TagResourcesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.TagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourcesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see <a href="http://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups and Tag Editor User Guide</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> </ul></p>
    fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> RusotoFuture<UntagResourcesOutput, UntagResourcesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.UntagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourcesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
