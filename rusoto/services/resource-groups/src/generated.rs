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
#[allow(unused_imports)]
use rusoto_core::pagination::{all_pages, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// see [ResourceGroups::create_group]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGroupInput {
    /// <p><p>A configuration associates the resource group with an AWS service and specifies how the service can interact with the resources in the group. A configuration is an array of <a>GroupConfigurationItem</a> elements.</p> <note> <p>You can specify either a <code>Configuration</code> or a <code>ResourceQuery</code> in a group, but not both.</p> </note></p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Vec<GroupConfigurationItem>>,
    /// <p>The description of the resource group. Descriptions can consist of letters, numbers, hyphens, underscores, periods, and spaces.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the group, which is the identifier of the group in other operations. You can't change the name of a resource group after you create it. A resource group name can consist of letters, numbers, hyphens, periods, and underscores. The name cannot start with <code>AWS</code> or <code>aws</code>; these are reserved. A resource group name must be unique within each AWS Region in your AWS account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The resource query that determines which AWS resources are members of this group.</p> <note> <p>You can specify either a <code>ResourceQuery</code> or a <code>Configuration</code>, but not both.</p> </note></p>
    #[serde(rename = "ResourceQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_query: Option<ResourceQuery>,
    /// <p>The tags to add to the group. A tag is key-value pair string.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// see [ResourceGroups::create_group]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupOutput {
    /// <p>The description of the resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// <p><p>The service configuration associated with the resource group. AWS Resource Groups supports adding service configurations for the following resource group types:</p> <ul> <li> <p> <code>AWS::EC2::CapacityReservationPool</code> - Amazon EC2 capacity reservation pools. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/capacity-reservations-using.html#create-cr-group">Working with capacity reservation groups</a> in the <i>EC2 Users Guide</i>.</p> </li> </ul></p>
    #[serde(rename = "GroupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_configuration: Option<GroupConfiguration>,
    /// <p>The resource query associated with the group.</p>
    #[serde(rename = "ResourceQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_query: Option<ResourceQuery>,
    /// <p>The tags associated with the group.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// see [ResourceGroups::delete_group]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGroupInput {
    /// <p>The name or the ARN of the resource group to delete.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// see [ResourceGroups::delete_group]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGroupOutput {
    /// <p>A full description of the deleted resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

/// <p>A resource that failed to be added to or removed from a group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedResource {
    /// <p>The error code associated with the failure.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message text associated with the failure.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ARN of the resource that failed to be added or removed.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

/// see [ResourceGroups::get_group_configuration]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupConfigurationInput {
    /// <p>The name or the ARN of the resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// see [ResourceGroups::get_group_configuration]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupConfigurationOutput {
    /// <p>The configuration associated with the specified group.</p>
    #[serde(rename = "GroupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_configuration: Option<GroupConfiguration>,
}

/// see [ResourceGroups::get_group]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupInput {
    /// <p>The name or the ARN of the resource group to retrieve.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// see [ResourceGroups::get_group]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupOutput {
    /// <p>A full description of the resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

/// see [ResourceGroups::get_group_query]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupQueryInput {
    /// <p>The name or the ARN of the resource group to query.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// see [ResourceGroups::get_group_query]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupQueryOutput {
    /// <p>The resource query associated with the specified group.</p>
    #[serde(rename = "GroupQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_query: Option<GroupQuery>,
}

/// see [ResourceGroups::get_tags]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagsInput {
    /// <p>The ARN of the resource group whose tags you want to retrieve.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

/// see [ResourceGroups::get_tags]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p><p>A resource group that contains AWS resources. You can assign resources to the group by associating either of the following elements with the group:</p> <ul> <li> <p> <a>ResourceQuery</a> - Use a resource query to specify a set of tag keys and values. All resources in the same AWS Region and AWS account that have those keys with the same values are included in the group. You can add a resource query when you create the group.</p> </li> <li> <p> <a>GroupConfiguration</a> - Use a service configuration to associate the group with an AWS service. The configuration specifies which resource types can be included in the group.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Group {
    /// <p>The description of the resource group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the resource group.</p>
    #[serde(rename = "GroupArn")]
    pub group_arn: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>A service configuration associated with a resource group. The configuration options are determined by the AWS service that defines the <code>Type</code>, and specifies which resources can be included in the group. You can add a service configuration when you create the group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupConfiguration {
    /// <p>The configuration currently associated with the group and in effect.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Vec<GroupConfigurationItem>>,
    /// <p>If present, the reason why a request to update the group configuration failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>If present, the new configuration that is in the process of being applied to the group.</p>
    #[serde(rename = "ProposedConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_configuration: Option<Vec<GroupConfigurationItem>>,
    /// <p>The current status of an attempt to update the group configuration.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An item in a group configuration. A group configuration can have one or more items.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GroupConfigurationItem {
    /// <p>A collection of parameters for this group configuration item.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<GroupConfigurationParameter>>,
    /// <p><p>Specifies the type of group configuration item. Each item must have a unique value for <code>type</code>.</p> <p>You can specify the following string values:</p> <ul> <li> <p> <code>AWS::EC2::CapacityReservationPool</code> </p> <p>For more information about EC2 capacity reservation groups, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/capacity-reservations-using.html#create-cr-group">Working with capacity reservation groups</a> in the <i>EC2 Users Guide</i>.</p> </li> <li> <p> <code>AWS::ResourceGroups::Generic</code> - Supports parameters that configure the behavior of resource groups of any type.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>A parameter for a group configuration item.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GroupConfigurationParameter {
    /// <p><p>The name of the group configuration parameter.</p> <p>You can specify the following string values:</p> <ul> <li> <p>For configuration item type <code>AWS::ResourceGroups::Generic</code>:</p> <ul> <li> <p> <code>allowed-resource-types</code> </p> <p>Specifies the types of resources that you can add to this group by using the <a>GroupResources</a> operation.</p> </li> </ul> </li> <li> <p>For configuration item type <code>AWS::EC2::CapacityReservationPool</code>:</p> <ul> <li> <p>None - This configuration item type doesn&#39;t support any parameters.</p> </li> </ul> <p>For more information about EC2 capacity reservation groups, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/capacity-reservations-using.html#create-cr-group">Working with capacity reservation groups</a> in the <i>EC2 Users Guide</i>.</p> </li> </ul></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The values of for this parameter.</p> <p>You can specify the following string value:</p> <ul> <li> <p>For item type <code>allowed-resource-types</code>: the only supported parameter value is <code>AWS::EC2::CapacityReservation</code>.</p> </li> </ul></p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>A filter collection that you can use to restrict the results from a <code>List</code> operation to only those you want to include.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GroupFilter {
    /// <p>The name of the filter. Filter names are case-sensitive.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>One or more filter values. Allowed filter values vary by group filter name, and are case-sensitive.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>The unique identifiers for a resource group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupIdentifier {
    /// <p>The ARN of the resource group.</p>
    #[serde(rename = "GroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>A mapping of a query attached to a resource group that determines the AWS resources that are members of the group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupQuery {
    /// <p>The name of the resource group that is associated with the specified resource query.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The resource query that determines which AWS resources are members of the associated resource group.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

/// see [ResourceGroups::group_resources]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GroupResourcesInput {
    /// <p>The name or the ARN of the resource group to add resources to.</p>
    #[serde(rename = "Group")]
    pub group: String,
    /// <p>The list of ARNs for resources to be added to the group. </p>
    #[serde(rename = "ResourceArns")]
    pub resource_arns: Vec<String>,
}

/// see [ResourceGroups::group_resources]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupResourcesOutput {
    /// <p>The ARNs of the resources that failed to be added to the group by this operation.</p>
    #[serde(rename = "Failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<FailedResource>>,
    /// <p>The ARNs of the resources that were successfully added to the group by this operation.</p>
    #[serde(rename = "Succeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<Vec<String>>,
}

/// see [ResourceGroups::list_group_resources]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGroupResourcesInput {
    /// <p>Filters, formatted as <a>ResourceFilter</a> objects, that you want to apply to a <code>ListGroupResources</code> operation. Filters the results to include only those of the specified resource types.</p> <ul> <li> <p> <code>resource-type</code> - Filter resources by their type. Specify up to five resource types in the format <code>AWS::ServiceCode::ResourceType</code>. For example, <code>AWS::EC2::Instance</code>, or <code>AWS::S3::Bucket</code>. </p> </li> </ul> <p>When you specify a <code>resource-type</code> filter for <code>ListGroupResources</code>, AWS Resource Groups validates your filter resource types against the types that are defined in the query associated with the group. For example, if a group contains only S3 buckets because its query specifies only that resource type, but your <code>resource-type</code> filter includes EC2 instances, AWS Resource Groups does not filter for EC2 instances. In this case, a <code>ListGroupResources</code> request returns a <code>BadRequestException</code> error with a message similar to the following:</p> <p> <code>The resource types specified as filters in the request are not valid.</code> </p> <p>The error includes a list of resource types that failed the validation because they are not part of the query associated with the group. This validation doesn't occur when the group query specifies <code>AWS::AllSupported</code>, because a group based on such a query can contain any of the allowed resource types for the query type (tag-based or AWS CloudFormation stack-based queries).</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ResourceFilter>>,
    /// <p>The name or the ARN of the resource group</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl PagedRequest for ListGroupResourcesInput {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.next_token = key;
        self
    }
}

/// see [ResourceGroups::list_group_resources]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupResourcesOutput {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
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

impl ListGroupResourcesOutput {
    fn pagination_page_opt(self) -> Option<Vec<ResourceIdentifier>> {
        Some(self.resource_identifiers.as_ref()?.clone())
    }
}

impl PagedOutput for ListGroupResourcesOutput {
    type Item = ResourceIdentifier;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.next_token.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ResourceIdentifier> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// see [ResourceGroups::list_groups]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGroupsInput {
    /// <p><p>Filters, formatted as <a>GroupFilter</a> objects, that you want to apply to a <code>ListGroups</code> operation.</p> <ul> <li> <p> <code>resource-type</code> - Filter the results to include only those of the specified resource types. Specify up to five resource types in the format <code>AWS::<i>ServiceCode</i>::<i>ResourceType</i> </code>. For example, <code>AWS::EC2::Instance</code>, or <code>AWS::S3::Bucket</code>.</p> </li> <li> <p> <code>configuration-type</code> - Filter the results to include only those groups that have the specified configuration types attached. The current supported values are:</p> <ul> <li> <p>AWS:EC2::CapacityReservationPool</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GroupFilter>>,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl PagedRequest for ListGroupsInput {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.next_token = key;
        self
    }
}

/// see [ResourceGroups::list_groups]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupsOutput {
    /// <p>A list of <a>GroupIdentifier</a> objects. Each identifier is an object that contains both the <code>Name</code> and the <code>GroupArn</code>.</p>
    #[serde(rename = "GroupIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifiers: Option<Vec<GroupIdentifier>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListGroupsOutput {}

impl PagedOutput for ListGroupsOutput {
    type Item = ListGroupsOutput;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.next_token.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ListGroupsOutput> {
        vec![self]
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p>A two-part error structure that can occur in <code>ListGroupResources</code> or <code>SearchResources</code> operations on CloudFormation stack-based queries. The error occurs if the CloudFormation stack on which the query is based either does not exist, or has a status that renders the stack inactive. A <code>QueryError</code> occurrence does not necessarily mean that AWS Resource Groups could not complete the operation, but the resulting group might have no member resources.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>The query that is used to define a resource group or a search for resources. A query specifies both a query type and a query string as a JSON object. See the examples section for example JSON strings.</p> <p>The examples that follow are shown as standard JSON strings. If you include such a string as a parameter to the AWS CLI or an SDK API, you might need to 'escape' the string into a single line. For example, see the <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-parameters-quoting-strings.html">Quoting strings</a> in the <i>AWS CLI User Guide</i>.</p> <p> <b>Example 1</b> </p> <p>The following generic example shows a resource query JSON string that includes only resources that meet the following criteria:</p> <ul> <li> <p>The resource type must be either <code>resource_type1</code> or <code>resource_type2</code>.</p> </li> <li> <p>The resource must have a tag <code>Key1</code> with a value of either <code>ValueA</code> or <code>ValueB</code>.</p> </li> <li> <p>The resource must have a tag <code>Key2</code> with a value of either <code>ValueC</code> or <code>ValueD</code>.</p> </li> </ul> <p> <code>{ "Type": "TAG_FILTERS_1_0", "Query": { "ResourceTypeFilters": [ "resource_type1", "resource_type2"], "TagFilters": [ { "Key": "Key1", "Values": ["ValueA","ValueB"] }, { "Key":"Key2", "Values":["ValueC","ValueD"] } ] } }</code> </p> <p>This has the equivalent "shortcut" syntax of the following:</p> <p> <code>{ "Type": "TAG_FILTERS_1_0", "Query": { "ResourceTypeFilters": [ "resource_type1", "resource_type2"], "TagFilters": [ { "Key1": ["ValueA","ValueB"] }, { "Key2": ["ValueC","ValueD"] } ] } }</code> </p> <p> <b>Example 2</b> </p> <p>The following example shows a resource query JSON string that includes only Amazon EC2 instances that are tagged <code>Stage</code> with a value of <code>Test</code>.</p> <p> <code>{ "Type": "TAG_FILTERS_1_0", "Query": "{ "ResourceTypeFilters": "AWS::EC2::Instance", "TagFilters": { "Stage": "Test" } } }</code> </p> <p> <b>Example 3</b> </p> <p>The following example shows a resource query JSON string that includes resource of any supported type as long as it is tagged <code>Stage</code> with a value of <code>Prod</code>.</p> <p> <code>{ "Type": "TAG_FILTERS_1_0", "Query": { "ResourceTypeFilters": "AWS::AllSupported", "TagFilters": { "Stage": "Prod" } } }</code> </p> <p> <b>Example 4</b> </p> <p>The following example shows a resource query JSON string that includes only Amazon EC2 instances and Amazon S3 buckets that are part of the specified AWS CloudFormation stack.</p> <p> <code>{ "Type": "CLOUDFORMATION_STACK_1_0", "Query": { "ResourceTypeFilters": [ "AWS::EC2::Instance", "AWS::S3::Bucket" ], "StackIdentifier": "arn:aws:cloudformation:us-west-2:123456789012:stack/AWStestuseraccount/fb0d5000-aba8-00e8-aa9e-50d5cEXAMPLE" } }</code> </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceQuery {
    /// <p>The query that defines a group or a search.</p>
    #[serde(rename = "Query")]
    pub query: String,
    /// <p><p>The type of the query. You can use the following values:</p> <ul> <li> <p> <i> <code>CLOUDFORMATION<em>STACK</em>1<em>0:</code> </i>Specifies that the <code>Query</code> contains an ARN for a CloudFormation stack.</p> </li> <li> <p> <i> <code>TAG</em>FILTERS<em>1</em>0:</code> </i>Specifies that the <code>Query</code> parameter contains a JSON string that represents a collection of simple tag filters for resource types and tags. The JSON string uses a syntax similar to the <code> <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_GetResources.html">GetResources</a> </code> operation, but uses only the <code> <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_GetResources.html#resourcegrouptagging-GetResources-request-ResourceTypeFilters"> ResourceTypeFilters</a> </code> and <code> <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_GetResources.html#resourcegrouptagging-GetResources-request-TagFiltersTagFilters">TagFilters</a> </code> fields. If you specify more than one tag key, only resources that match all tag keys, and at least one value of each specified tag key, are returned in your query. If you specify more than one value for a tag key, a resource matches the filter if it has a tag key value that matches <i>any</i> of the specified values.</p> <p>For example, consider the following sample query for resources that have two tags, <code>Stage</code> and <code>Version</code>, with two values each:</p> <p> <code>[{&quot;Stage&quot;:[&quot;Test&quot;,&quot;Deploy&quot;]},{&quot;Version&quot;:[&quot;1&quot;,&quot;2&quot;]}]</code> </p> <p>The results of this query could include the following.</p> <ul> <li> <p>An EC2 instance that has the following two tags: <code>{&quot;Stage&quot;:&quot;Deploy&quot;}</code>, and <code>{&quot;Version&quot;:&quot;2&quot;}</code> </p> </li> <li> <p>An S3 bucket that has the following two tags: <code>{&quot;Stage&quot;:&quot;Test&quot;}</code>, and <code>{&quot;Version&quot;:&quot;1&quot;}</code> </p> </li> </ul> <p>The query would not include the following items in the results, however. </p> <ul> <li> <p>An EC2 instance that has only the following tag: <code>{&quot;Stage&quot;:&quot;Deploy&quot;}</code>.</p> <p>The instance does not have <b>all</b> of the tag keys specified in the filter, so it is excluded from the results.</p> </li> <li> <p>An RDS database that has the following two tags: <code>{&quot;Stage&quot;:&quot;Archived&quot;}</code> and <code>{&quot;Version&quot;:&quot;4&quot;}</code> </p> <p>The database has all of the tag keys, but none of those keys has an associated value that matches at least one of the specified values in the filter.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// see [ResourceGroups::search_resources]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchResourcesInput {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The search query, using the same formats that are supported for resource group definition. For more information, see <a>CreateGroup</a>.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

impl PagedRequest for SearchResourcesInput {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.next_token = key;
        self
    }
}

/// see [ResourceGroups::search_resources]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchResourcesOutput {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
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

impl SearchResourcesOutput {
    fn pagination_page_opt(self) -> Option<Vec<ResourceIdentifier>> {
        Some(self.resource_identifiers.as_ref()?.clone())
    }
}

impl PagedOutput for SearchResourcesOutput {
    type Item = ResourceIdentifier;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.next_token.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ResourceIdentifier> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// see [ResourceGroups::tag]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagInput {
    /// <p>The ARN of the resource group to which to add tags.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags to add to the specified resource group. A tag is a string-to-string map of key-value pairs.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// see [ResourceGroups::tag]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagOutput {
    /// <p>The ARN of the tagged resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The tags that have been added to the specified resource group.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// see [ResourceGroups::ungroup_resources]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UngroupResourcesInput {
    /// <p>The name or the ARN of the resource group from which to remove the resources.</p>
    #[serde(rename = "Group")]
    pub group: String,
    /// <p>The ARNs of the resources to be removed from the group.</p>
    #[serde(rename = "ResourceArns")]
    pub resource_arns: Vec<String>,
}

/// see [ResourceGroups::ungroup_resources]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UngroupResourcesOutput {
    /// <p>The resources that failed to be removed from the group.</p>
    #[serde(rename = "Failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<FailedResource>>,
    /// <p>The ARNs of the resources that were successfully removed from the group.</p>
    #[serde(rename = "Succeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<Vec<String>>,
}

/// see [ResourceGroups::untag]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagInput {
    /// <p>The ARN of the resource group from which to remove tags. The command removed both the specified keys and any values associated with those keys.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "Keys")]
    pub keys: Vec<String>,
}

/// see [ResourceGroups::untag]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagOutput {
    /// <p>The ARN of the resource group from which tags have been removed.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The keys of the tags that were removed.</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// see [ResourceGroups::update_group]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGroupInput {
    /// <p>The new description that you want to update the resource group with. Descriptions can contain letters, numbers, hyphens, underscores, periods, and spaces.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name or the ARN of the resource group to modify.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// see [ResourceGroups::update_group]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupOutput {
    /// <p>The update description of the resource group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

/// see [ResourceGroups::update_group_query]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGroupQueryInput {
    /// <p>The name or the ARN of the resource group to query.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>The resource query to determine which AWS resources are members of this resource group.</p>
    #[serde(rename = "ResourceQuery")]
    pub resource_query: ResourceQuery,
}

/// see [ResourceGroups::update_group_query]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupQueryOutput {
    /// <p>The updated resource query associated with the resource group after the update.</p>
    #[serde(rename = "GroupQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_query: Option<GroupQuery>,
}

/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
/// Errors returned by GetGroupConfiguration
#[derive(Debug, PartialEq)]
pub enum GetGroupConfigurationError {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
    TooManyRequests(String),
}

impl GetGroupConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGroupConfigurationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetGroupConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetGroupConfigurationError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetGroupConfigurationError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetGroupConfigurationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGroupConfigurationError::TooManyRequests(
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
impl fmt::Display for GetGroupConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGroupConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetGroupConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetGroupConfigurationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetGroupConfigurationError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetGroupConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetGroupConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGroupConfigurationError {}
/// Errors returned by GetGroupQuery
#[derive(Debug, PartialEq)]
pub enum GetGroupQueryError {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
/// Errors returned by GroupResources
#[derive(Debug, PartialEq)]
pub enum GroupResourcesError {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
    TooManyRequests(String),
}

impl GroupResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GroupResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GroupResourcesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GroupResourcesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GroupResourcesError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GroupResourcesError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GroupResourcesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GroupResourcesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GroupResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GroupResourcesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GroupResourcesError::Forbidden(ref cause) => write!(f, "{}", cause),
            GroupResourcesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GroupResourcesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GroupResourcesError::NotFound(ref cause) => write!(f, "{}", cause),
            GroupResourcesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GroupResourcesError {}
/// Errors returned by ListGroupResources
#[derive(Debug, PartialEq)]
pub enum ListGroupResourcesError {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
    TooManyRequests(String),
    /// <p>The request was rejected because it doesn't have valid credentials for the target resource.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
    TooManyRequests(String),
    /// <p>The request was rejected because it doesn't have valid credentials for the target resource.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
/// Errors returned by UngroupResources
#[derive(Debug, PartialEq)]
pub enum UngroupResourcesError {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
    TooManyRequests(String),
}

impl UngroupResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UngroupResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UngroupResourcesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UngroupResourcesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UngroupResourcesError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UngroupResourcesError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UngroupResourcesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UngroupResourcesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UngroupResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UngroupResourcesError::BadRequest(ref cause) => write!(f, "{}", cause),
            UngroupResourcesError::Forbidden(ref cause) => write!(f, "{}", cause),
            UngroupResourcesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UngroupResourcesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UngroupResourcesError::NotFound(ref cause) => write!(f, "{}", cause),
            UngroupResourcesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UngroupResourcesError {}
/// Errors returned by Untag
#[derive(Debug, PartialEq)]
pub enum UntagError {
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
    /// <p>The request includes one or more parameters that violate validation rules.</p>
    BadRequest(String),
    /// <p>The caller isn't authorized to make the request. Check permissions.</p>
    Forbidden(String),
    /// <p>An internal error occurred while processing the request. Try again later.</p>
    InternalServerError(String),
    /// <p>The request uses an HTTP method that isn't allowed for the specified resource.</p>
    MethodNotAllowed(String),
    /// <p>One or more of the specified resources don't exist.</p>
    NotFound(String),
    /// <p>You've exceeded throttling limits by making too many requests in a period of time.</p>
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
        RusotoError::Unknown(res)
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
pub trait ResourceGroups: Clone + Sync + Send + 'static {
    /// <p>Creates a resource group with the specified name and description. You can optionally include a resource query, or a service configuration.</p>
    async fn create_group(
        &self,
        input: CreateGroupInput,
    ) -> Result<CreateGroupOutput, RusotoError<CreateGroupError>>;

    /// <p>Deletes the specified resource group. Deleting a resource group does not delete any resources that are members of the group; it only deletes the group structure.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupInput,
    ) -> Result<DeleteGroupOutput, RusotoError<DeleteGroupError>>;

    /// <p>Returns information about a specified resource group.</p>
    async fn get_group(
        &self,
        input: GetGroupInput,
    ) -> Result<GetGroupOutput, RusotoError<GetGroupError>>;

    /// <p><p>Returns the service configuration associated with the specified resource group. AWS Resource Groups supports configurations for the following resource group types:</p> <ul> <li> <p> <code>AWS::EC2::CapacityReservationPool</code> - Amazon EC2 capacity reservation pools. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/capacity-reservations-using.html#create-cr-group">Working with capacity reservation groups</a> in the <i>EC2 Users Guide</i>.</p> </li> </ul></p>
    async fn get_group_configuration(
        &self,
        input: GetGroupConfigurationInput,
    ) -> Result<GetGroupConfigurationOutput, RusotoError<GetGroupConfigurationError>>;

    /// <p>Retrieves the resource query associated with the specified resource group.</p>
    async fn get_group_query(
        &self,
        input: GetGroupQueryInput,
    ) -> Result<GetGroupQueryOutput, RusotoError<GetGroupQueryError>>;

    /// <p>Returns a list of tags that are associated with a resource group, specified by an ARN.</p>
    async fn get_tags(
        &self,
        input: GetTagsInput,
    ) -> Result<GetTagsOutput, RusotoError<GetTagsError>>;

    /// <p>Adds the specified resources to the specified group.</p>
    async fn group_resources(
        &self,
        input: GroupResourcesInput,
    ) -> Result<GroupResourcesOutput, RusotoError<GroupResourcesError>>;

    /// <p>Returns a list of ARNs of the resources that are members of a specified resource group.</p>
    async fn list_group_resources(
        &self,
        input: ListGroupResourcesInput,
    ) -> Result<ListGroupResourcesOutput, RusotoError<ListGroupResourcesError>>;

    /// Auto-paginating version of `list_group_resources`
    fn list_group_resources_pages(
        &self,
        input: ListGroupResourcesInput,
    ) -> RusotoStream<ResourceIdentifier, ListGroupResourcesError> {
        all_pages(self.clone(), input, move |client, state| {
            client.list_group_resources(state.clone())
        })
    }

    /// <p>Returns a list of existing resource groups in your account.</p>
    async fn list_groups(
        &self,
        input: ListGroupsInput,
    ) -> Result<ListGroupsOutput, RusotoError<ListGroupsError>>;

    /// Auto-paginating version of `list_groups`
    fn list_groups_pages(
        &self,
        input: ListGroupsInput,
    ) -> RusotoStream<ListGroupsOutput, ListGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.list_groups(state.clone())
        })
    }

    /// <p>Returns a list of AWS resource identifiers that matches the specified query. The query uses the same format as a resource query in a CreateGroup or UpdateGroupQuery operation.</p>
    async fn search_resources(
        &self,
        input: SearchResourcesInput,
    ) -> Result<SearchResourcesOutput, RusotoError<SearchResourcesError>>;

    /// Auto-paginating version of `search_resources`
    fn search_resources_pages(
        &self,
        input: SearchResourcesInput,
    ) -> RusotoStream<ResourceIdentifier, SearchResourcesError> {
        all_pages(self.clone(), input, move |client, state| {
            client.search_resources(state.clone())
        })
    }

    /// <p><p>Adds tags to a resource group with the specified ARN. Existing tags on a resource group are not changed if they are not specified in the request parameters.</p> <important> <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p> </important></p>
    async fn tag(&self, input: TagInput) -> Result<TagOutput, RusotoError<TagError>>;

    /// <p>Removes the specified resources from the specified group.</p>
    async fn ungroup_resources(
        &self,
        input: UngroupResourcesInput,
    ) -> Result<UngroupResourcesOutput, RusotoError<UngroupResourcesError>>;

    /// <p>Deletes tags from a specified resource group.</p>
    async fn untag(&self, input: UntagInput) -> Result<UntagOutput, RusotoError<UntagError>>;

    /// <p>Updates the description for an existing group. You cannot update the name of a resource group.</p>
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
    /// <p>Creates a resource group with the specified name and description. You can optionally include a resource query, or a service configuration.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGroupError::from_response(response))
        }
    }

    /// <p>Deletes the specified resource group. Deleting a resource group does not delete any resources that are members of the group; it only deletes the group structure.</p>
    #[allow(unused_mut)]
    async fn delete_group(
        &self,
        input: DeleteGroupInput,
    ) -> Result<DeleteGroupOutput, RusotoError<DeleteGroupError>> {
        let request_uri = "/delete-group";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGroupError::from_response(response))
        }
    }

    /// <p>Returns information about a specified resource group.</p>
    #[allow(unused_mut)]
    async fn get_group(
        &self,
        input: GetGroupInput,
    ) -> Result<GetGroupOutput, RusotoError<GetGroupError>> {
        let request_uri = "/get-group";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupError::from_response(response))
        }
    }

    /// <p><p>Returns the service configuration associated with the specified resource group. AWS Resource Groups supports configurations for the following resource group types:</p> <ul> <li> <p> <code>AWS::EC2::CapacityReservationPool</code> - Amazon EC2 capacity reservation pools. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/capacity-reservations-using.html#create-cr-group">Working with capacity reservation groups</a> in the <i>EC2 Users Guide</i>.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn get_group_configuration(
        &self,
        input: GetGroupConfigurationInput,
    ) -> Result<GetGroupConfigurationOutput, RusotoError<GetGroupConfigurationError>> {
        let request_uri = "/get-group-configuration";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetGroupConfigurationOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupConfigurationError::from_response(response))
        }
    }

    /// <p>Retrieves the resource query associated with the specified resource group.</p>
    #[allow(unused_mut)]
    async fn get_group_query(
        &self,
        input: GetGroupQueryInput,
    ) -> Result<GetGroupQueryOutput, RusotoError<GetGroupQueryError>> {
        let request_uri = "/get-group-query";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetGroupQueryOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupQueryError::from_response(response))
        }
    }

    /// <p>Returns a list of tags that are associated with a resource group, specified by an ARN.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetTagsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTagsError::from_response(response))
        }
    }

    /// <p>Adds the specified resources to the specified group.</p>
    #[allow(unused_mut)]
    async fn group_resources(
        &self,
        input: GroupResourcesInput,
    ) -> Result<GroupResourcesOutput, RusotoError<GroupResourcesError>> {
        let request_uri = "/group-resources";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GroupResourcesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GroupResourcesError::from_response(response))
        }
    }

    /// <p>Returns a list of ARNs of the resources that are members of a specified resource group.</p>
    #[allow(unused_mut)]
    async fn list_group_resources(
        &self,
        input: ListGroupResourcesInput,
    ) -> Result<ListGroupResourcesOutput, RusotoError<ListGroupResourcesError>> {
        let request_uri = "/list-group-resources";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListGroupResourcesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroupResourcesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing resource groups in your account.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListGroupsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroupsError::from_response(response))
        }
    }

    /// <p>Returns a list of AWS resource identifiers that matches the specified query. The query uses the same format as a resource query in a CreateGroup or UpdateGroupQuery operation.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SearchResourcesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SearchResourcesError::from_response(response))
        }
    }

    /// <p><p>Adds tags to a resource group with the specified ARN. Existing tags on a resource group are not changed if they are not specified in the request parameters.</p> <important> <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p> </important></p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<TagOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagError::from_response(response))
        }
    }

    /// <p>Removes the specified resources from the specified group.</p>
    #[allow(unused_mut)]
    async fn ungroup_resources(
        &self,
        input: UngroupResourcesInput,
    ) -> Result<UngroupResourcesOutput, RusotoError<UngroupResourcesError>> {
        let request_uri = "/ungroup-resources";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UngroupResourcesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UngroupResourcesError::from_response(response))
        }
    }

    /// <p>Deletes tags from a specified resource group.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<UntagOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagError::from_response(response))
        }
    }

    /// <p>Updates the description for an existing group. You cannot update the name of a resource group.</p>
    #[allow(unused_mut)]
    async fn update_group(
        &self,
        input: UpdateGroupInput,
    ) -> Result<UpdateGroupOutput, RusotoError<UpdateGroupError>> {
        let request_uri = "/update-group";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGroupOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupError::from_response(response))
        }
    }

    /// <p>Updates the resource query of a group.</p>
    #[allow(unused_mut)]
    async fn update_group_query(
        &self,
        input: UpdateGroupQueryInput,
    ) -> Result<UpdateGroupQueryOutput, RusotoError<UpdateGroupQueryError>> {
        let request_uri = "/update-group-query";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGroupQueryOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupQueryError::from_response(response))
        }
    }
}
