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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupInput {
    /// <p>The description of the resource group. Descriptions can have a maximum of 511 characters, including letters, numbers, hyphens, underscores, punctuation, and spaces.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the group, which is the identifier of the group in other operations. A resource group name cannot be updated after it is created. A resource group name can have a maximum of 128 characters, including letters, numbers, hyphens, dots, and underscores. The name cannot start with <code>AWS</code> or <code>aws</code>; these are reserved. A resource group name must be unique within your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The resource query that determines which AWS resources are members of this group.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
    /// <p>The tags to add to the group. A tag is a string-to-string map of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGroupOutput {
    /// <p>A full description of the resource group after it is created.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// <p>The resource query associated with the group.</p>
    #[serde(rename = "ResourceQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_query: Option<ResourceQuery>,
    /// <p>The tags associated with the group.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGroupInput {
    /// <p>The name of the resource group to delete.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGroupOutput {
    /// <p>A full description of the deleted resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupInput {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGroupOutput {
    /// <p>A full description of the resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupQueryInput {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGroupQueryOutput {
    /// <p>The resource query associated with the specified group.</p>
    #[serde(rename = "GroupQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_query: Option<GroupQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagsInput {
    /// <p>The ARN of the resource for which you want a list of tags. The resource must exist within the account you are using.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTagsOutput {
    /// <p>The ARN of the tagged resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The tags associated with the specified resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A resource group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Group {
    /// <p>The description of the resource group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of a resource group.</p>
    #[serde(rename = "GroupArn")]
    pub group_arn: String,
    /// <p>The name of a resource group.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>A filter name and value pair that is used to obtain more specific results from a list of groups.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GroupFilter {
    /// <p>The name of the filter. Filter names are case-sensitive.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>One or more filter values. Allowed filter values vary by group filter name, and are case-sensitive.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>The ARN and group name of a group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GroupIdentifier {
    /// <p>The ARN of a resource group.</p>
    #[serde(rename = "GroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The name of a resource group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>The underlying resource query of a resource group. Resources that match query results are part of the group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GroupQuery {
    /// <p>The name of a resource group that is associated with a specific resource query.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The resource query which determines which AWS resources are members of the associated resource group.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupResourcesInput {
    /// <p><p>Filters, formatted as ResourceFilter objects, that you want to apply to a ListGroupResources operation.</p> <ul> <li> <p> <code>resource-type</code> - Filter resources by their type. Specify up to five resource types in the format AWS::ServiceCode::ResourceType. For example, AWS::EC2::Instance, or AWS::S3::Bucket.</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ResourceFilter>>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The maximum number of group member ARNs that are returned in a single call by ListGroupResources, in paginated output. By default, this number is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The NextToken value that is returned in a paginated ListGroupResources request. To get the next page of results, run the call again, add the NextToken parameter, and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListGroupResourcesOutput {
    /// <p>The NextToken value to include in a subsequent <code>ListGroupResources</code> request, to get more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>QueryError</code> objects. Each error is an object that contains <code>ErrorCode</code> and <code>Message</code> structures. Possible values for <code>ErrorCode</code> are <code>CLOUDFORMATION_STACK_INACTIVE</code> and <code>CLOUDFORMATION_STACK_NOT_EXISTING</code>.</p>
    #[serde(rename = "QueryErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_errors: Option<Vec<QueryError>>,
    /// <p>The ARNs and resource types of resources that are members of the group that you specified.</p>
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<ResourceIdentifier>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupsInput {
    /// <p><p>Filters, formatted as GroupFilter objects, that you want to apply to a ListGroups operation.</p> <ul> <li> <p> <code>resource-type</code> - Filter groups by resource type. Specify up to five resource types in the format AWS::ServiceCode::ResourceType. For example, AWS::EC2::Instance, or AWS::S3::Bucket.</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GroupFilter>>,
    /// <p>The maximum number of resource group results that are returned by ListGroups in paginated output. By default, this number is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The NextToken value that is returned in a paginated <code>ListGroups</code> request. To get the next page of results, run the call again, add the NextToken parameter, and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListGroupsOutput {
    /// <p>A list of GroupIdentifier objects. Each identifier is an object that contains both the GroupName and the GroupArn.</p>
    #[serde(rename = "GroupIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifiers: Option<Vec<GroupIdentifier>>,
    /// <p>The NextToken value to include in a subsequent <code>ListGroups</code> request, to get more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A two-part error structure that can occur in <code>ListGroupResources</code> or <code>SearchResources</code> operations on CloudFormation stack-based queries. The error occurs if the CloudFormation stack on which the query is based either does not exist, or has a status that renders the stack inactive. A <code>QueryError</code> occurrence does not necessarily mean that AWS Resource Groups could not complete the operation, but the resulting group might have no member resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct QueryError {
    /// <p>Possible values are <code>CLOUDFORMATION_STACK_INACTIVE</code> and <code>CLOUDFORMATION_STACK_NOT_EXISTING</code>.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A message that explains the <code>ErrorCode</code> value. Messages might state that the specified CloudFormation stack does not exist (or no longer exists). For <code>CLOUDFORMATION_STACK_INACTIVE</code>, the message typically states that the CloudFormation stack has a status that is not (or no longer) active, such as <code>CREATE_FAILED</code>.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>A filter name and value pair that is used to obtain more specific results from a list of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResourceFilter {
    /// <p>The name of the filter. Filter names are case-sensitive.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>One or more filter values. Allowed filter values vary by resource filter name, and are case-sensitive.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>The ARN of a resource, and its resource type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceIdentifier {
    /// <p>The ARN of a resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The resource type of a resource, such as <code>AWS::EC2::Instance</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The query that is used to define a resource group or a search for resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceQuery {
    /// <p>The query that defines a group or a search.</p>
    #[serde(rename = "Query")]
    pub query: String,
    /// <p>The type of the query. The valid values in this release are <code>TAG_FILTERS_1_0</code> and <code>CLOUDFORMATION_STACK_1_0</code>.</p> <p> <i> <code>TAG_FILTERS_1_0:</code> </i> A JSON syntax that lets you specify a collection of simple tag filters for resource types and tags, as supported by the AWS Tagging API <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_GetResources.html">GetResources</a> operation. If you specify more than one tag key, only resources that match all tag keys, and at least one value of each specified tag key, are returned in your query. If you specify more than one value for a tag key, a resource matches the filter if it has a tag key value that matches <i>any</i> of the specified values.</p> <p>For example, consider the following sample query for resources that have two tags, <code>Stage</code> and <code>Version</code>, with two values each. (<code>[{"Key":"Stage","Values":["Test","Deploy"]},{"Key":"Version","Values":["1","2"]}]</code>) The results of this query might include the following.</p> <ul> <li> <p>An EC2 instance that has the following two tags: <code>{"Key":"Stage","Values":["Deploy"]}</code>, and <code>{"Key":"Version","Values":["2"]}</code> </p> </li> <li> <p>An S3 bucket that has the following two tags: {"Key":"Stage","Values":["Test","Deploy"]}, and {"Key":"Version","Values":["1"]}</p> </li> </ul> <p>The query would not return the following results, however. The following EC2 instance does not have all tag keys specified in the filter, so it is rejected. The RDS database has all of the tag keys, but no values that match at least one of the specified tag key values in the filter.</p> <ul> <li> <p>An EC2 instance that has only the following tag: <code>{"Key":"Stage","Values":["Deploy"]}</code>.</p> </li> <li> <p>An RDS database that has the following two tags: <code>{"Key":"Stage","Values":["Archived"]}</code>, and <code>{"Key":"Version","Values":["4"]}</code> </p> </li> </ul> <p> <i> <code>CLOUDFORMATION_STACK_1_0:</code> </i> A JSON syntax that lets you specify a CloudFormation stack ARN.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchResourcesInput {
    /// <p>The maximum number of group member ARNs returned by <code>SearchResources</code> in paginated output. By default, this number is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The NextToken value that is returned in a paginated <code>SearchResources</code> request. To get the next page of results, run the call again, add the NextToken parameter, and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The search query, using the same formats that are supported for resource group definition.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchResourcesOutput {
    /// <p>The NextToken value to include in a subsequent <code>SearchResources</code> request, to get more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>QueryError</code> objects. Each error is an object that contains <code>ErrorCode</code> and <code>Message</code> structures. Possible values for <code>ErrorCode</code> are <code>CLOUDFORMATION_STACK_INACTIVE</code> and <code>CLOUDFORMATION_STACK_NOT_EXISTING</code>.</p>
    #[serde(rename = "QueryErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_errors: Option<Vec<QueryError>>,
    /// <p>The ARNs and resource types of resources that are members of the group that you specified.</p>
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<ResourceIdentifier>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagInput {
    /// <p>The ARN of the resource to which to add tags.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags to add to the specified resource. A tag is a string-to-string map of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagOutput {
    /// <p>The ARN of the tagged resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The tags that have been added to the specified resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagInput {
    /// <p>The ARN of the resource from which to remove tags.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "Keys")]
    pub keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagOutput {
    /// <p>The ARN of the resource from which tags have been removed.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The keys of tags that have been removed.</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupInput {
    /// <p>The description of the resource group. Descriptions can have a maximum of 511 characters, including letters, numbers, hyphens, underscores, punctuation, and spaces.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the resource group for which you want to update its description.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGroupOutput {
    /// <p>The full description of the resource group after it has been updated.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupQueryInput {
    /// <p>The name of the resource group for which you want to edit the query.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The resource query that determines which AWS resources are members of the resource group.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGroupQueryOutput {
    /// <p>The resource query associated with the resource group after the update.</p>
    #[serde(rename = "GroupQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_query: Option<GroupQuery>,
}

/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl CreateGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateGroupError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return CreateGroupError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateGroupError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return CreateGroupError::MethodNotAllowed(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return CreateGroupError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateGroupError {
    fn from(err: serde_json::error::Error) -> CreateGroupError {
        CreateGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGroupError {
    fn from(err: CredentialsError) -> CreateGroupError {
        CreateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGroupError {
    fn from(err: HttpDispatchError) -> CreateGroupError {
        CreateGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGroupError {
    fn from(err: io::Error) -> CreateGroupError {
        CreateGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupError::BadRequest(ref cause) => cause,
            CreateGroupError::Forbidden(ref cause) => cause,
            CreateGroupError::InternalServerError(ref cause) => cause,
            CreateGroupError::MethodNotAllowed(ref cause) => cause,
            CreateGroupError::TooManyRequests(ref cause) => cause,
            CreateGroupError::Validation(ref cause) => cause,
            CreateGroupError::Credentials(ref err) => err.description(),
            CreateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateGroupError::ParseError(ref cause) => cause,
            CreateGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl DeleteGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteGroupError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return DeleteGroupError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteGroupError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return DeleteGroupError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteGroupError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteGroupError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteGroupError {
    fn from(err: serde_json::error::Error) -> DeleteGroupError {
        DeleteGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGroupError {
    fn from(err: CredentialsError) -> DeleteGroupError {
        DeleteGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGroupError {
    fn from(err: HttpDispatchError) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGroupError {
    fn from(err: io::Error) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteGroupError::BadRequest(ref cause) => cause,
            DeleteGroupError::Forbidden(ref cause) => cause,
            DeleteGroupError::InternalServerError(ref cause) => cause,
            DeleteGroupError::MethodNotAllowed(ref cause) => cause,
            DeleteGroupError::NotFound(ref cause) => cause,
            DeleteGroupError::TooManyRequests(ref cause) => cause,
            DeleteGroupError::Validation(ref cause) => cause,
            DeleteGroupError::Credentials(ref err) => err.description(),
            DeleteGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGroupError::ParseError(ref cause) => cause,
            DeleteGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetGroup
#[derive(Debug, PartialEq)]
pub enum GetGroupError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl GetGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetGroupError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return GetGroupError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetGroupError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return GetGroupError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => return GetGroupError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetGroupError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetGroupError {
    fn from(err: serde_json::error::Error) -> GetGroupError {
        GetGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupError {
    fn from(err: CredentialsError) -> GetGroupError {
        GetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupError {
    fn from(err: HttpDispatchError) -> GetGroupError {
        GetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupError {
    fn from(err: io::Error) -> GetGroupError {
        GetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupError {
    fn description(&self) -> &str {
        match *self {
            GetGroupError::BadRequest(ref cause) => cause,
            GetGroupError::Forbidden(ref cause) => cause,
            GetGroupError::InternalServerError(ref cause) => cause,
            GetGroupError::MethodNotAllowed(ref cause) => cause,
            GetGroupError::NotFound(ref cause) => cause,
            GetGroupError::TooManyRequests(ref cause) => cause,
            GetGroupError::Validation(ref cause) => cause,
            GetGroupError::Credentials(ref err) => err.description(),
            GetGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGroupError::ParseError(ref cause) => cause,
            GetGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetGroupQuery
#[derive(Debug, PartialEq)]
pub enum GetGroupQueryError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl GetGroupQueryError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetGroupQueryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetGroupQueryError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return GetGroupQueryError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetGroupQueryError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return GetGroupQueryError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => {
                    return GetGroupQueryError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return GetGroupQueryError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetGroupQueryError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetGroupQueryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetGroupQueryError {
    fn from(err: serde_json::error::Error) -> GetGroupQueryError {
        GetGroupQueryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupQueryError {
    fn from(err: CredentialsError) -> GetGroupQueryError {
        GetGroupQueryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupQueryError {
    fn from(err: HttpDispatchError) -> GetGroupQueryError {
        GetGroupQueryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupQueryError {
    fn from(err: io::Error) -> GetGroupQueryError {
        GetGroupQueryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupQueryError {
    fn description(&self) -> &str {
        match *self {
            GetGroupQueryError::BadRequest(ref cause) => cause,
            GetGroupQueryError::Forbidden(ref cause) => cause,
            GetGroupQueryError::InternalServerError(ref cause) => cause,
            GetGroupQueryError::MethodNotAllowed(ref cause) => cause,
            GetGroupQueryError::NotFound(ref cause) => cause,
            GetGroupQueryError::TooManyRequests(ref cause) => cause,
            GetGroupQueryError::Validation(ref cause) => cause,
            GetGroupQueryError::Credentials(ref err) => err.description(),
            GetGroupQueryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGroupQueryError::ParseError(ref cause) => cause,
            GetGroupQueryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetTags
#[derive(Debug, PartialEq)]
pub enum GetTagsError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl GetTagsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetTagsError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => return GetTagsError::Forbidden(String::from(error_message)),
                "InternalServerErrorException" => {
                    return GetTagsError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return GetTagsError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => return GetTagsError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetTagsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => return GetTagsError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetTagsError {
    fn from(err: serde_json::error::Error) -> GetTagsError {
        GetTagsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTagsError {
    fn from(err: CredentialsError) -> GetTagsError {
        GetTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTagsError {
    fn from(err: HttpDispatchError) -> GetTagsError {
        GetTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTagsError {
    fn from(err: io::Error) -> GetTagsError {
        GetTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagsError {
    fn description(&self) -> &str {
        match *self {
            GetTagsError::BadRequest(ref cause) => cause,
            GetTagsError::Forbidden(ref cause) => cause,
            GetTagsError::InternalServerError(ref cause) => cause,
            GetTagsError::MethodNotAllowed(ref cause) => cause,
            GetTagsError::NotFound(ref cause) => cause,
            GetTagsError::TooManyRequests(ref cause) => cause,
            GetTagsError::Validation(ref cause) => cause,
            GetTagsError::Credentials(ref err) => err.description(),
            GetTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTagsError::ParseError(ref cause) => cause,
            GetTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListGroupResources
#[derive(Debug, PartialEq)]
pub enum ListGroupResourcesError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
    TooManyRequests(String),
    /// <p>The request has not been applied because it lacks valid authentication credentials for the target resource.</p>
    Unauthorized(String),
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

impl ListGroupResourcesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListGroupResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListGroupResourcesError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return ListGroupResourcesError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListGroupResourcesError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return ListGroupResourcesError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => {
                    return ListGroupResourcesError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListGroupResourcesError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ListGroupResourcesError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ListGroupResourcesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListGroupResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListGroupResourcesError {
    fn from(err: serde_json::error::Error) -> ListGroupResourcesError {
        ListGroupResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupResourcesError {
    fn from(err: CredentialsError) -> ListGroupResourcesError {
        ListGroupResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupResourcesError {
    fn from(err: HttpDispatchError) -> ListGroupResourcesError {
        ListGroupResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGroupResourcesError {
    fn from(err: io::Error) -> ListGroupResourcesError {
        ListGroupResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGroupResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListGroupResourcesError::BadRequest(ref cause) => cause,
            ListGroupResourcesError::Forbidden(ref cause) => cause,
            ListGroupResourcesError::InternalServerError(ref cause) => cause,
            ListGroupResourcesError::MethodNotAllowed(ref cause) => cause,
            ListGroupResourcesError::NotFound(ref cause) => cause,
            ListGroupResourcesError::TooManyRequests(ref cause) => cause,
            ListGroupResourcesError::Unauthorized(ref cause) => cause,
            ListGroupResourcesError::Validation(ref cause) => cause,
            ListGroupResourcesError::Credentials(ref err) => err.description(),
            ListGroupResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListGroupResourcesError::ParseError(ref cause) => cause,
            ListGroupResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl ListGroupsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListGroupsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListGroupsError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return ListGroupsError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListGroupsError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return ListGroupsError::MethodNotAllowed(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListGroupsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListGroupsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListGroupsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListGroupsError {
    fn from(err: serde_json::error::Error) -> ListGroupsError {
        ListGroupsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupsError {
    fn from(err: CredentialsError) -> ListGroupsError {
        ListGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupsError {
    fn from(err: HttpDispatchError) -> ListGroupsError {
        ListGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGroupsError {
    fn from(err: io::Error) -> ListGroupsError {
        ListGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListGroupsError::BadRequest(ref cause) => cause,
            ListGroupsError::Forbidden(ref cause) => cause,
            ListGroupsError::InternalServerError(ref cause) => cause,
            ListGroupsError::MethodNotAllowed(ref cause) => cause,
            ListGroupsError::TooManyRequests(ref cause) => cause,
            ListGroupsError::Validation(ref cause) => cause,
            ListGroupsError::Credentials(ref err) => err.description(),
            ListGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGroupsError::ParseError(ref cause) => cause,
            ListGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SearchResources
#[derive(Debug, PartialEq)]
pub enum SearchResourcesError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>The caller has exceeded throttling limits.</p>
    TooManyRequests(String),
    /// <p>The request has not been applied because it lacks valid authentication credentials for the target resource.</p>
    Unauthorized(String),
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

impl SearchResourcesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SearchResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return SearchResourcesError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return SearchResourcesError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return SearchResourcesError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return SearchResourcesError::MethodNotAllowed(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return SearchResourcesError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return SearchResourcesError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return SearchResourcesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchResourcesError {
    fn from(err: serde_json::error::Error) -> SearchResourcesError {
        SearchResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchResourcesError {
    fn from(err: CredentialsError) -> SearchResourcesError {
        SearchResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchResourcesError {
    fn from(err: HttpDispatchError) -> SearchResourcesError {
        SearchResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchResourcesError {
    fn from(err: io::Error) -> SearchResourcesError {
        SearchResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchResourcesError {
    fn description(&self) -> &str {
        match *self {
            SearchResourcesError::BadRequest(ref cause) => cause,
            SearchResourcesError::Forbidden(ref cause) => cause,
            SearchResourcesError::InternalServerError(ref cause) => cause,
            SearchResourcesError::MethodNotAllowed(ref cause) => cause,
            SearchResourcesError::TooManyRequests(ref cause) => cause,
            SearchResourcesError::Unauthorized(ref cause) => cause,
            SearchResourcesError::Validation(ref cause) => cause,
            SearchResourcesError::Credentials(ref err) => err.description(),
            SearchResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchResourcesError::ParseError(ref cause) => cause,
            SearchResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by Tag
#[derive(Debug, PartialEq)]
pub enum TagError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl TagError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> TagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => return TagError::BadRequest(String::from(error_message)),
                "ForbiddenException" => return TagError::Forbidden(String::from(error_message)),
                "InternalServerErrorException" => {
                    return TagError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return TagError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => return TagError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return TagError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => return TagError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return TagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagError {
    fn from(err: serde_json::error::Error) -> TagError {
        TagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TagError {
    fn from(err: CredentialsError) -> TagError {
        TagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagError {
    fn from(err: HttpDispatchError) -> TagError {
        TagError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagError {
    fn from(err: io::Error) -> TagError {
        TagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagError {
    fn description(&self) -> &str {
        match *self {
            TagError::BadRequest(ref cause) => cause,
            TagError::Forbidden(ref cause) => cause,
            TagError::InternalServerError(ref cause) => cause,
            TagError::MethodNotAllowed(ref cause) => cause,
            TagError::NotFound(ref cause) => cause,
            TagError::TooManyRequests(ref cause) => cause,
            TagError::Validation(ref cause) => cause,
            TagError::Credentials(ref err) => err.description(),
            TagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagError::ParseError(ref cause) => cause,
            TagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by Untag
#[derive(Debug, PartialEq)]
pub enum UntagError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl UntagError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UntagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => return UntagError::BadRequest(String::from(error_message)),
                "ForbiddenException" => return UntagError::Forbidden(String::from(error_message)),
                "InternalServerErrorException" => {
                    return UntagError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return UntagError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => return UntagError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return UntagError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => return UntagError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return UntagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagError {
    fn from(err: serde_json::error::Error) -> UntagError {
        UntagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagError {
    fn from(err: CredentialsError) -> UntagError {
        UntagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagError {
    fn from(err: HttpDispatchError) -> UntagError {
        UntagError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagError {
    fn from(err: io::Error) -> UntagError {
        UntagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagError {
    fn description(&self) -> &str {
        match *self {
            UntagError::BadRequest(ref cause) => cause,
            UntagError::Forbidden(ref cause) => cause,
            UntagError::InternalServerError(ref cause) => cause,
            UntagError::MethodNotAllowed(ref cause) => cause,
            UntagError::NotFound(ref cause) => cause,
            UntagError::TooManyRequests(ref cause) => cause,
            UntagError::Validation(ref cause) => cause,
            UntagError::Credentials(ref err) => err.description(),
            UntagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagError::ParseError(ref cause) => cause,
            UntagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl UpdateGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateGroupError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return UpdateGroupError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateGroupError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return UpdateGroupError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateGroupError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateGroupError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateGroupError {
    fn from(err: serde_json::error::Error) -> UpdateGroupError {
        UpdateGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGroupError {
    fn from(err: CredentialsError) -> UpdateGroupError {
        UpdateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGroupError {
    fn from(err: HttpDispatchError) -> UpdateGroupError {
        UpdateGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGroupError {
    fn from(err: io::Error) -> UpdateGroupError {
        UpdateGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupError::BadRequest(ref cause) => cause,
            UpdateGroupError::Forbidden(ref cause) => cause,
            UpdateGroupError::InternalServerError(ref cause) => cause,
            UpdateGroupError::MethodNotAllowed(ref cause) => cause,
            UpdateGroupError::NotFound(ref cause) => cause,
            UpdateGroupError::TooManyRequests(ref cause) => cause,
            UpdateGroupError::Validation(ref cause) => cause,
            UpdateGroupError::Credentials(ref err) => err.description(),
            UpdateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateGroupError::ParseError(ref cause) => cause,
            UpdateGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateGroupQuery
#[derive(Debug, PartialEq)]
pub enum UpdateGroupQueryError {
    /// <p>The request does not comply with validation rules that are defined for the request parameters.</p>
    BadRequest(String),
    /// <p>The caller is not authorized to make the request.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method which is not allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more resources specified in the request do not exist.</p>
    NotFound(String),
    /// <p>The caller has exceeded throttling limits.</p>
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

impl UpdateGroupQueryError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateGroupQueryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateGroupQueryError::BadRequest(String::from(error_message));
                }
                "ForbiddenException" => {
                    return UpdateGroupQueryError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateGroupQueryError::InternalServerError(String::from(error_message));
                }
                "MethodNotAllowedException" => {
                    return UpdateGroupQueryError::MethodNotAllowed(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateGroupQueryError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateGroupQueryError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateGroupQueryError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateGroupQueryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateGroupQueryError {
    fn from(err: serde_json::error::Error) -> UpdateGroupQueryError {
        UpdateGroupQueryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGroupQueryError {
    fn from(err: CredentialsError) -> UpdateGroupQueryError {
        UpdateGroupQueryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGroupQueryError {
    fn from(err: HttpDispatchError) -> UpdateGroupQueryError {
        UpdateGroupQueryError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGroupQueryError {
    fn from(err: io::Error) -> UpdateGroupQueryError {
        UpdateGroupQueryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGroupQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupQueryError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupQueryError::BadRequest(ref cause) => cause,
            UpdateGroupQueryError::Forbidden(ref cause) => cause,
            UpdateGroupQueryError::InternalServerError(ref cause) => cause,
            UpdateGroupQueryError::MethodNotAllowed(ref cause) => cause,
            UpdateGroupQueryError::NotFound(ref cause) => cause,
            UpdateGroupQueryError::TooManyRequests(ref cause) => cause,
            UpdateGroupQueryError::Validation(ref cause) => cause,
            UpdateGroupQueryError::Credentials(ref err) => err.description(),
            UpdateGroupQueryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateGroupQueryError::ParseError(ref cause) => cause,
            UpdateGroupQueryError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Resource Groups API. Resource Groups clients implement this trait.
pub trait ResourceGroups {
    /// <p>Creates a group with a specified name, description, and resource query.</p>
    fn create_group(
        &self,
        input: CreateGroupInput,
    ) -> RusotoFuture<CreateGroupOutput, CreateGroupError>;

    /// <p>Deletes a specified resource group. Deleting a resource group does not delete resources that are members of the group; it only deletes the group structure.</p>
    fn delete_group(
        &self,
        input: DeleteGroupInput,
    ) -> RusotoFuture<DeleteGroupOutput, DeleteGroupError>;

    /// <p>Returns information about a specified resource group.</p>
    fn get_group(&self, input: GetGroupInput) -> RusotoFuture<GetGroupOutput, GetGroupError>;

    /// <p>Returns the resource query associated with the specified resource group.</p>
    fn get_group_query(
        &self,
        input: GetGroupQueryInput,
    ) -> RusotoFuture<GetGroupQueryOutput, GetGroupQueryError>;

    /// <p>Returns a list of tags that are associated with a resource, specified by an ARN.</p>
    fn get_tags(&self, input: GetTagsInput) -> RusotoFuture<GetTagsOutput, GetTagsError>;

    /// <p>Returns a list of ARNs of resources that are members of a specified resource group.</p>
    fn list_group_resources(
        &self,
        input: ListGroupResourcesInput,
    ) -> RusotoFuture<ListGroupResourcesOutput, ListGroupResourcesError>;

    /// <p>Returns a list of existing resource groups in your account.</p>
    fn list_groups(
        &self,
        input: ListGroupsInput,
    ) -> RusotoFuture<ListGroupsOutput, ListGroupsError>;

    /// <p>Returns a list of AWS resource identifiers that matches a specified query. The query uses the same format as a resource query in a CreateGroup or UpdateGroupQuery operation.</p>
    fn search_resources(
        &self,
        input: SearchResourcesInput,
    ) -> RusotoFuture<SearchResourcesOutput, SearchResourcesError>;

    /// <p>Adds specified tags to a resource with the specified ARN. Existing tags on a resource are not changed if they are not specified in the request parameters.</p>
    fn tag(&self, input: TagInput) -> RusotoFuture<TagOutput, TagError>;

    /// <p>Deletes specified tags from a specified resource.</p>
    fn untag(&self, input: UntagInput) -> RusotoFuture<UntagOutput, UntagError>;

    /// <p>Updates an existing group with a new or changed description. You cannot update the name of a resource group.</p>
    fn update_group(
        &self,
        input: UpdateGroupInput,
    ) -> RusotoFuture<UpdateGroupOutput, UpdateGroupError>;

    /// <p>Updates the resource query of a group.</p>
    fn update_group_query(
        &self,
        input: UpdateGroupQueryInput,
    ) -> RusotoFuture<UpdateGroupQueryOutput, UpdateGroupQueryError>;
}
/// A client for the Resource Groups API.
#[derive(Clone)]
pub struct ResourceGroupsClient {
    client: Client,
    region: region::Region,
}

impl ResourceGroupsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ResourceGroupsClient {
        ResourceGroupsClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ResourceGroupsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ResourceGroupsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ResourceGroups for ResourceGroupsClient {
    /// <p>Creates a group with a specified name, description, and resource query.</p>
    fn create_group(
        &self,
        input: CreateGroupInput,
    ) -> RusotoFuture<CreateGroupOutput, CreateGroupError> {
        let request_uri = "/groups";

        let mut request = SignedRequest::new("POST", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateGroupOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified resource group. Deleting a resource group does not delete resources that are members of the group; it only deletes the group structure.</p>
    fn delete_group(
        &self,
        input: DeleteGroupInput,
    ) -> RusotoFuture<DeleteGroupOutput, DeleteGroupError> {
        let request_uri = format!("/groups/{group_name}", group_name = input.group_name);

        let mut request =
            SignedRequest::new("DELETE", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteGroupOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specified resource group.</p>
    fn get_group(&self, input: GetGroupInput) -> RusotoFuture<GetGroupOutput, GetGroupError> {
        let request_uri = format!("/groups/{group_name}", group_name = input.group_name);

        let mut request = SignedRequest::new("GET", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetGroupOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the resource query associated with the specified resource group.</p>
    fn get_group_query(
        &self,
        input: GetGroupQueryInput,
    ) -> RusotoFuture<GetGroupQueryOutput, GetGroupQueryError> {
        let request_uri = format!("/groups/{group_name}/query", group_name = input.group_name);

        let mut request = SignedRequest::new("GET", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetGroupQueryOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGroupQueryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of tags that are associated with a resource, specified by an ARN.</p>
    fn get_tags(&self, input: GetTagsInput) -> RusotoFuture<GetTagsOutput, GetTagsError> {
        let request_uri = format!("/resources/{arn}/tags", arn = input.arn);

        let mut request = SignedRequest::new("GET", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetTagsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of ARNs of resources that are members of a specified resource group.</p>
    fn list_group_resources(
        &self,
        input: ListGroupResourcesInput,
    ) -> RusotoFuture<ListGroupResourcesOutput, ListGroupResourcesError> {
        let request_uri = format!(
            "/groups/{group_name}/resource-identifiers-list",
            group_name = input.group_name
        );

        let mut request = SignedRequest::new("POST", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListGroupResourcesOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGroupResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of existing resource groups in your account.</p>
    fn list_groups(
        &self,
        input: ListGroupsInput,
    ) -> RusotoFuture<ListGroupsOutput, ListGroupsError> {
        let request_uri = "/groups-list";

        let mut request = SignedRequest::new("POST", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListGroupsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of AWS resource identifiers that matches a specified query. The query uses the same format as a resource query in a CreateGroup or UpdateGroupQuery operation.</p>
    fn search_resources(
        &self,
        input: SearchResourcesInput,
    ) -> RusotoFuture<SearchResourcesOutput, SearchResourcesError> {
        let request_uri = "/resources/search";

        let mut request = SignedRequest::new("POST", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<SearchResourcesOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds specified tags to a resource with the specified ARN. Existing tags on a resource are not changed if they are not specified in the request parameters.</p>
    fn tag(&self, input: TagInput) -> RusotoFuture<TagOutput, TagError> {
        let request_uri = format!("/resources/{arn}/tags", arn = input.arn);

        let mut request = SignedRequest::new("PUT", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TagOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes specified tags from a specified resource.</p>
    fn untag(&self, input: UntagInput) -> RusotoFuture<UntagOutput, UntagError> {
        let request_uri = format!("/resources/{arn}/tags", arn = input.arn);

        let mut request =
            SignedRequest::new("PATCH", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UntagOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing group with a new or changed description. You cannot update the name of a resource group.</p>
    fn update_group(
        &self,
        input: UpdateGroupInput,
    ) -> RusotoFuture<UpdateGroupOutput, UpdateGroupError> {
        let request_uri = format!("/groups/{group_name}", group_name = input.group_name);

        let mut request = SignedRequest::new("PUT", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateGroupOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the resource query of a group.</p>
    fn update_group_query(
        &self,
        input: UpdateGroupQueryInput,
    ) -> RusotoFuture<UpdateGroupQueryOutput, UpdateGroupQueryError> {
        let request_uri = format!("/groups/{group_name}/query", group_name = input.group_name);

        let mut request = SignedRequest::new("PUT", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateGroupQueryOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGroupQueryError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
