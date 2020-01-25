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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGroupInput {
    /// <p>The name of the resource group to delete.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGroupOutput {
    /// <p>A full description of the deleted resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupInput {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupOutput {
    /// <p>A full description of the resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupQueryInput {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupQueryOutput {
    /// <p>The resource query associated with the specified group.</p>
    #[serde(rename = "GroupQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_query: Option<GroupQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagsInput {
    /// <p>The ARN of the resource group for which you want a list of tags. The resource must exist within the account you are using.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTagsOutput {
    /// <p>The ARN of the tagged resource group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The tags associated with the specified resource group.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A resource group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupQuery {
    /// <p>The name of a resource group that is associated with a specific resource query.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The resource query which determines which AWS resources are members of the associated resource group.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The type of the query. The valid values in this release are <code>TAG_FILTERS_1_0</code> and <code>CLOUDFORMATION_STACK_1_0</code>.</p> <p> <i> <code>TAG_FILTERS_1_0:</code> </i> A JSON syntax that lets you specify a collection of simple tag filters for resource types and tags, as supported by the AWS Tagging API <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_GetResources.html">GetResources</a> operation. If you specify more than one tag key, only resources that match all tag keys, and at least one value of each specified tag key, are returned in your query. If you specify more than one value for a tag key, a resource matches the filter if it has a tag key value that matches <i>any</i> of the specified values.</p> <p>For example, consider the following sample query for resources that have two tags, <code>Stage</code> and <code>Version</code>, with two values each. (<code>[{"Key":"Stage","Values":["Test","Deploy"]},{"Key":"Version","Values":["1","2"]}]</code>) The results of this query might include the following.</p> <ul> <li> <p>An EC2 instance that has the following two tags: <code>{"Key":"Stage","Value":"Deploy"}</code>, and <code>{"Key":"Version","Value":"2"}</code> </p> </li> <li> <p>An S3 bucket that has the following two tags: {"Key":"Stage","Value":"Test"}, and {"Key":"Version","Value":"1"}</p> </li> </ul> <p>The query would not return the following results, however. The following EC2 instance does not have all tag keys specified in the filter, so it is rejected. The RDS database has all of the tag keys, but no values that match at least one of the specified tag key values in the filter.</p> <ul> <li> <p>An EC2 instance that has only the following tag: <code>{"Key":"Stage","Value":"Deploy"}</code>.</p> </li> <li> <p>An RDS database that has the following two tags: <code>{"Key":"Stage","Value":"Archived"}</code>, and <code>{"Key":"Version","Value":"4"}</code> </p> </li> </ul> <p> <i> <code>CLOUDFORMATION_STACK_1_0:</code> </i> A JSON syntax that lets you specify a CloudFormation stack ARN.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagInput {
    /// <p>The ARN of the resource to which to add tags.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags to add to the specified resource. A tag is a string-to-string map of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagInput {
    /// <p>The ARN of the resource from which to remove tags.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "Keys")]
    pub keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupOutput {
    /// <p>The full description of the resource group after it has been updated.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGroupQueryInput {
    /// <p>The name of the resource group for which you want to edit the query.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The resource query that determines which AWS resources are members of the resource group.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateGroupError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateGroupError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateGroupError::MethodNotAllowed(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateGroupError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGroupError {}
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
}

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteGroupError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteGroupError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteGroupError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteGroupError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGroupError {}
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
}

impl GetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGroupError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetGroupError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetGroupError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetGroupError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetGroupError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            GetGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGroupError {}
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
}

impl GetGroupQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupQueryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGroupQueryError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetGroupQueryError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetGroupQueryError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetGroupQueryError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetGroupQueryError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGroupQueryError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGroupQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGroupQueryError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetGroupQueryError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetGroupQueryError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetGroupQueryError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetGroupQueryError::NotFound(ref cause) => write!(f, "{}", cause),
            GetGroupQueryError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGroupQueryError {}
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
}

impl GetTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetTagsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetTagsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetTagsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetTagsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTagsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetTagsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTagsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetTagsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetTagsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetTagsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetTagsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetTagsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTagsError {}
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
}

impl ListGroupResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListGroupResourcesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListGroupResourcesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListGroupResourcesError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(ListGroupResourcesError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListGroupResourcesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListGroupResourcesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListGroupResourcesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGroupResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGroupResourcesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListGroupResourcesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListGroupResourcesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListGroupResourcesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            ListGroupResourcesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListGroupResourcesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListGroupResourcesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroupResourcesError {}
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
}

impl ListGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListGroupsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListGroupsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListGroupsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(ListGroupsError::MethodNotAllowed(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListGroupsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGroupsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListGroupsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListGroupsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListGroupsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            ListGroupsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroupsError {}
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
}

impl SearchResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SearchResourcesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(SearchResourcesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(SearchResourcesError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(SearchResourcesError::MethodNotAllowed(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SearchResourcesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(SearchResourcesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchResourcesError::BadRequest(ref cause) => write!(f, "{}", cause),
            SearchResourcesError::Forbidden(ref cause) => write!(f, "{}", cause),
            SearchResourcesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SearchResourcesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            SearchResourcesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            SearchResourcesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchResourcesError {}
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
}

impl TagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagError::BadRequest(err.msg))
                }
                "ForbiddenException" => return RusotoError::Service(TagError::Forbidden(err.msg)),
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(TagError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(TagError::NotFound(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagError::Forbidden(ref cause) => write!(f, "{}", cause),
            TagError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            TagError::NotFound(ref cause) => write!(f, "{}", cause),
            TagError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagError {}
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
}

impl UntagError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UntagError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UntagError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(UntagError::NotFound(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagError::Forbidden(ref cause) => write!(f, "{}", cause),
            UntagError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UntagError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagError {}
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
}

impl UpdateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGroupError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateGroupError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateGroupError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGroupError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGroupError {}
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
}

impl UpdateGroupQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGroupQueryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGroupQueryError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateGroupQueryError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateGroupQueryError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateGroupQueryError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGroupQueryError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateGroupQueryError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGroupQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGroupQueryError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateGroupQueryError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateGroupQueryError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateGroupQueryError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateGroupQueryError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGroupQueryError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGroupQueryError {}
/// Trait representing the capabilities of the Resource Groups API. Resource Groups clients implement this trait.
#[async_trait]
pub trait ResourceGroups {
    /// <p>Creates a group with a specified name, description, and resource query.</p>
    async fn create_group(
        &self,
        input: CreateGroupInput,
    ) -> Result<CreateGroupOutput, RusotoError<CreateGroupError>>;

    /// <p>Deletes a specified resource group. Deleting a resource group does not delete resources that are members of the group; it only deletes the group structure.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupInput,
    ) -> Result<DeleteGroupOutput, RusotoError<DeleteGroupError>>;

    /// <p>Returns information about a specified resource group.</p>
    async fn get_group(
        &self,
        input: GetGroupInput,
    ) -> Result<GetGroupOutput, RusotoError<GetGroupError>>;

    /// <p>Returns the resource query associated with the specified resource group.</p>
    async fn get_group_query(
        &self,
        input: GetGroupQueryInput,
    ) -> Result<GetGroupQueryOutput, RusotoError<GetGroupQueryError>>;

    /// <p>Returns a list of tags that are associated with a resource group, specified by an ARN.</p>
    async fn get_tags(
        &self,
        input: GetTagsInput,
    ) -> Result<GetTagsOutput, RusotoError<GetTagsError>>;

    /// <p>Returns a list of ARNs of resources that are members of a specified resource group.</p>
    async fn list_group_resources(
        &self,
        input: ListGroupResourcesInput,
    ) -> Result<ListGroupResourcesOutput, RusotoError<ListGroupResourcesError>>;

    /// <p>Returns a list of existing resource groups in your account.</p>
    async fn list_groups(
        &self,
        input: ListGroupsInput,
    ) -> Result<ListGroupsOutput, RusotoError<ListGroupsError>>;

    /// <p>Returns a list of AWS resource identifiers that matches a specified query. The query uses the same format as a resource query in a CreateGroup or UpdateGroupQuery operation.</p>
    async fn search_resources(
        &self,
        input: SearchResourcesInput,
    ) -> Result<SearchResourcesOutput, RusotoError<SearchResourcesError>>;

    /// <p>Adds tags to a resource group with the specified ARN. Existing tags on a resource group are not changed if they are not specified in the request parameters.</p>
    async fn tag(&self, input: TagInput) -> Result<TagOutput, RusotoError<TagError>>;

    /// <p>Deletes specified tags from a specified resource.</p>
    async fn untag(&self, input: UntagInput) -> Result<UntagOutput, RusotoError<UntagError>>;

    /// <p>Updates an existing group with a new or changed description. You cannot update the name of a resource group.</p>
    async fn update_group(
        &self,
        input: UpdateGroupInput,
    ) -> Result<UpdateGroupOutput, RusotoError<UpdateGroupError>>;

    /// <p>Updates the resource query of a group.</p>
    async fn update_group_query(
        &self,
        input: UpdateGroupQueryInput,
    ) -> Result<UpdateGroupQueryOutput, RusotoError<UpdateGroupQueryError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ResourceGroupsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ResourceGroupsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ResourceGroupsClient {
        ResourceGroupsClient { client, region }
    }
}

#[async_trait]
impl ResourceGroups for ResourceGroupsClient {
    /// <p>Creates a group with a specified name, description, and resource query.</p>
    async fn create_group(
        &self,
        input: CreateGroupInput,
    ) -> Result<CreateGroupOutput, RusotoError<CreateGroupError>> {
        let request_uri = "/groups";

        let mut request = SignedRequest::new("POST", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGroupError::from_response(response))
        }
    }

    /// <p>Deletes a specified resource group. Deleting a resource group does not delete resources that are members of the group; it only deletes the group structure.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupInput,
    ) -> Result<DeleteGroupOutput, RusotoError<DeleteGroupError>> {
        let request_uri = format!("/groups/{group_name}", group_name = input.group_name);

        let mut request =
            SignedRequest::new("DELETE", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGroupError::from_response(response))
        }
    }

    /// <p>Returns information about a specified resource group.</p>
    async fn get_group(
        &self,
        input: GetGroupInput,
    ) -> Result<GetGroupOutput, RusotoError<GetGroupError>> {
        let request_uri = format!("/groups/{group_name}", group_name = input.group_name);

        let mut request = SignedRequest::new("GET", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupError::from_response(response))
        }
    }

    /// <p>Returns the resource query associated with the specified resource group.</p>
    async fn get_group_query(
        &self,
        input: GetGroupQueryInput,
    ) -> Result<GetGroupQueryOutput, RusotoError<GetGroupQueryError>> {
        let request_uri = format!("/groups/{group_name}/query", group_name = input.group_name);

        let mut request = SignedRequest::new("GET", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetGroupQueryOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupQueryError::from_response(response))
        }
    }

    /// <p>Returns a list of tags that are associated with a resource group, specified by an ARN.</p>
    async fn get_tags(
        &self,
        input: GetTagsInput,
    ) -> Result<GetTagsOutput, RusotoError<GetTagsError>> {
        let request_uri = format!("/resources/{arn}/tags", arn = input.arn);

        let mut request = SignedRequest::new("GET", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetTagsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTagsError::from_response(response))
        }
    }

    /// <p>Returns a list of ARNs of resources that are members of a specified resource group.</p>
    async fn list_group_resources(
        &self,
        input: ListGroupResourcesInput,
    ) -> Result<ListGroupResourcesOutput, RusotoError<ListGroupResourcesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListGroupResourcesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroupResourcesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing resource groups in your account.</p>
    async fn list_groups(
        &self,
        input: ListGroupsInput,
    ) -> Result<ListGroupsOutput, RusotoError<ListGroupsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListGroupsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroupsError::from_response(response))
        }
    }

    /// <p>Returns a list of AWS resource identifiers that matches a specified query. The query uses the same format as a resource query in a CreateGroup or UpdateGroupQuery operation.</p>
    async fn search_resources(
        &self,
        input: SearchResourcesInput,
    ) -> Result<SearchResourcesOutput, RusotoError<SearchResourcesError>> {
        let request_uri = "/resources/search";

        let mut request = SignedRequest::new("POST", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SearchResourcesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SearchResourcesError::from_response(response))
        }
    }

    /// <p>Adds tags to a resource group with the specified ARN. Existing tags on a resource group are not changed if they are not specified in the request parameters.</p>
    async fn tag(&self, input: TagInput) -> Result<TagOutput, RusotoError<TagError>> {
        let request_uri = format!("/resources/{arn}/tags", arn = input.arn);

        let mut request = SignedRequest::new("PUT", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<TagOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from a specified resource.</p>
    async fn untag(&self, input: UntagInput) -> Result<UntagOutput, RusotoError<UntagError>> {
        let request_uri = format!("/resources/{arn}/tags", arn = input.arn);

        let mut request =
            SignedRequest::new("PATCH", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<UntagOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagError::from_response(response))
        }
    }

    /// <p>Updates an existing group with a new or changed description. You cannot update the name of a resource group.</p>
    async fn update_group(
        &self,
        input: UpdateGroupInput,
    ) -> Result<UpdateGroupOutput, RusotoError<UpdateGroupError>> {
        let request_uri = format!("/groups/{group_name}", group_name = input.group_name);

        let mut request = SignedRequest::new("PUT", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupError::from_response(response))
        }
    }

    /// <p>Updates the resource query of a group.</p>
    async fn update_group_query(
        &self,
        input: UpdateGroupQueryInput,
    ) -> Result<UpdateGroupQueryOutput, RusotoError<UpdateGroupQueryError>> {
        let request_uri = format!("/groups/{group_name}/query", group_name = input.group_name);

        let mut request = SignedRequest::new("PUT", "resource-groups", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGroupQueryOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupQueryError::from_response(response))
        }
    }
}
