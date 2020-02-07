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
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Information that shows whether a resource is compliant with the effective tag policy, including details on any noncompliant tag keys.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComplianceDetails {
    /// <p>Whether a resource is compliant with the effective tag policy.</p>
    #[serde(rename = "ComplianceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<bool>,
    /// <p>These are keys defined in the effective policy that are on the resource with either incorrect case treatment or noncompliant values. </p>
    #[serde(rename = "KeysWithNoncompliantValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys_with_noncompliant_values: Option<Vec<String>>,
    /// <p>These tag keys on the resource are noncompliant with the effective tag policy.</p>
    #[serde(rename = "NoncompliantKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncompliant_keys: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReportCreationInput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReportCreationOutput {
    /// <p>Details of the common errors that all operations return.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The path to the Amazon S3 bucket where the report was stored on creation.</p>
    #[serde(rename = "S3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<String>,
    /// <p><p>Reports the status of the operation.</p> <p>The operation status can be one of the following:</p> <ul> <li> <p> <code>RUNNING</code> - Report creation is in progress.</p> </li> <li> <p> <code>SUCCEEDED</code> - Report creation is complete. You can open the report from the Amazon S3 bucket that you specified when you ran <code>StartReportCreation</code>.</p> </li> <li> <p> <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not accessible. </p> </li> <li> <p> <code>NO REPORT</code> - No report was generated in the last 90 days.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Details of the common errors that all actions return.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetComplianceSummaryInput {
    /// <p>A list of attributes to group the counts of noncompliant resources by. If supplied, the counts are sorted by those attributes.</p>
    #[serde(rename = "GroupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,
    /// <p>A limit that restricts the number of results that are returned per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a <code>PaginationToken</code>, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of Regions to limit the output by. If you use this parameter, the count of returned noncompliant resources includes only resources in the specified Regions.</p>
    #[serde(rename = "RegionFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_filters: Option<Vec<String>>,
    /// <p>The constraints on the resources that you want returned. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p> <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). Consult the <i>AWS General Reference</i> for the following:</p> <ul> <li> <p>For a list of service name strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a>.</p> </li> <li> <p>For resource type strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example ARNs</a>.</p> </li> <li> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> </li> </ul> <p>You can specify multiple resource types by using an array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. </p>
    #[serde(rename = "ResourceTypeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    /// <p>A list of tag keys to limit the output by. If you use this parameter, the count of returned noncompliant resources includes only resources that have the specified tag keys.</p>
    #[serde(rename = "TagKeyFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_filters: Option<Vec<String>>,
    /// <p>The target identifiers (usually, specific account IDs) to limit the output by. If you use this parameter, the count of returned noncompliant resources includes only resources with the specified target IDs.</p>
    #[serde(rename = "TargetIdFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id_filters: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetComplianceSummaryOutput {
    /// <p>A string that indicates that the response contains more data than can be returned in a single response. To receive additional data, specify this string for the <code>PaginationToken</code> value in a subsequent request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A table that shows counts of noncompliant resources.</p>
    #[serde(rename = "SummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_list: Option<Vec<Summary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourcesInput {
    /// <p>Specifies whether to exclude resources that are compliant with the tag policy. Set this to <code>true</code> if you are interested in retrieving information on noncompliant resources only.</p> <p>You can use this parameter only if the <code>IncludeComplianceDetails</code> parameter is also set to <code>true</code>.</p>
    #[serde(rename = "ExcludeCompliantResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_compliant_resources: Option<bool>,
    /// <p>Specifies whether to include details regarding the compliance with the effective tag policy. Set this to <code>true</code> to determine whether resources are compliant with the tag policy and to get details.</p>
    #[serde(rename = "IncludeComplianceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_compliance_details: Option<bool>,
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a <code>PaginationToken</code>, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>The constraints on the resources that you want returned. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p> <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). Consult the <i>AWS General Reference</i> for the following:</p> <ul> <li> <p>For a list of service name strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a>.</p> </li> <li> <p>For resource type strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example ARNs</a>.</p> </li> <li> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> </li> </ul> <p>You can specify multiple resource types by using an array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. </p>
    #[serde(rename = "ResourceTypeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    /// <p>A limit that restricts the number of resources returned by GetResources in paginated output. You can set ResourcesPerPage to a minimum of 1 item and the maximum of 100 items. </p>
    #[serde(rename = "ResourcesPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_per_page: Option<i64>,
    /// <p><p>A list of TagFilters (keys and values). Each TagFilter specified must contain a key with values as optional. A request can include up to 50 keys, and each key can include up to 20 values. </p> <p>Note the following when deciding how to use TagFilters:</p> <ul> <li> <p>If you <i>do</i> specify a TagFilter, the response returns only those resources that are currently associated with the specified tag. </p> </li> <li> <p>If you <i>don&#39;t</i> specify a TagFilter, the response includes all resources that were ever associated with tags. Resources that currently don&#39;t have associated tags are shown with an empty tag set, like this: <code>&quot;Tags&quot;: []</code>.</p> </li> <li> <p>If you specify more than one filter in a single request, the response returns only those resources that satisfy all specified filters.</p> </li> <li> <p>If you specify a filter that contains more than one value for a key, the response returns resources that match any of the specified values for that key.</p> </li> <li> <p>If you don&#39;t specify any values for a key, the response returns resources that are tagged with that key irrespective of the value.</p> <p>For example, for filters: filter1 = {key1, {value1}}, filter2 = {key2, {value2,value3,value4}} , filter3 = {key3}:</p> <ul> <li> <p>GetResources( {filter1} ) returns resources tagged with key1=value1</p> </li> <li> <p>GetResources( {filter2} ) returns resources tagged with key2=value2 or key2=value3 or key2=value4</p> </li> <li> <p>GetResources( {filter3} ) returns resources tagged with any tag containing key3 as its tag key, irrespective of its value</p> </li> <li> <p>GetResources( {filter1,filter2,filter3} ) returns resources tagged with ( key1=value1) and ( key2=value2 or key2=value3 or key2=value4) and (key3, irrespective of the value)</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
    /// <p>AWS recommends using <code>ResourcesPerPage</code> instead of this parameter.</p> <p>A limit that restricts the number of tags (key and value pairs) returned by GetResources in paginated output. A resource with no tags is counted as having one tag (one key and value pair).</p> <p> <code>GetResources</code> does not split a resource and its associated tags across pages. If the specified <code>TagsPerPage</code> would cause such a break, a <code>PaginationToken</code> is returned in place of the affected resource and its tags. Use that token in another request to get the remaining data. For example, if you specify a <code>TagsPerPage</code> of <code>100</code> and the account has 22 resources with 10 tags each (meaning that each resource has 10 key and value pairs), the output will consist of three pages. The first page displays the first 10 resources, each with its 10 tags. The second page displays the next 10 resources, each with its 10 tags. The third page displays the remaining 2 resources, each with its 10 tags.</p> <p>You can set <code>TagsPerPage</code> to a minimum of 100 items and the maximum of 500 items.</p>
    #[serde(rename = "TagsPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_per_page: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagKeysInput {
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a <code>PaginationToken</code>, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagValuesInput {
    /// <p>The key for which you want to list all existing values in the specified Region for the AWS account.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a <code>PaginationToken</code>, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceTagMapping {
    /// <p>Information that shows whether a resource is compliant with the effective tag policy, including details on any noncompliant tag keys.</p>
    #[serde(rename = "ComplianceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_details: Option<ComplianceDetails>,
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The tags that have been applied to one or more AWS resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartReportCreationInput {
    /// <p>The name of the Amazon S3 bucket where the report will be stored; for example:</p> <p> <code>awsexamplebucket</code> </p> <p>For more information on S3 bucket requirements, including an example bucket policy, see the example S3 bucket policy on this page.</p>
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartReportCreationOutput {}

/// <p>A count of noncompliant resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Summary {
    /// <p>The timestamp that shows when this summary was generated in this Region. </p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// <p>The count of noncompliant resources.</p>
    #[serde(rename = "NonCompliantResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources: Option<i64>,
    /// <p>The AWS Region that the summary applies to.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The AWS resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The account identifier or the root identifier of the organization. If you don't know the root ID, you can call the AWS Organizations <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_ListRoots.html">ListRoots</a> API.</p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p>Whether the target is an account, an OU, or the organization root.</p>
    #[serde(rename = "TargetIdType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id_type: Option<String>,
}

/// <p>The metadata that you apply to AWS resources to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Tag {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A list of tags (keys and values) that are used to specify the associated resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagFilter {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourcesInput {
    /// <p>A list of ARNs. An ARN (Amazon Resource Name) uniquely identifies a resource. You can specify a minimum of 1 and a maximum of 20 ARNs (resources) to tag. An ARN can be set to a maximum of 1600 characters. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "ResourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>The tags that you want to add to the specified resources. A tag consists of a key and a value that you define.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourcesOutput {
    /// <p>Details of resources that could not be tagged. An error code, status code, and error message are returned for each failed item.</p>
    #[serde(rename = "FailedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourcesInput {
    /// <p>A list of ARNs. An ARN (Amazon Resource Name) uniquely identifies a resource. You can specify a minimum of 1 and a maximum of 20 ARNs (resources) to untag. An ARN can be set to a maximum of 1600 characters. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "ResourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>A list of the tag keys that you want to remove from the specified resources.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourcesOutput {
    /// <p>Details of resources that could not be untagged. An error code, status code, and error message are returned for each failed item. </p>
    #[serde(rename = "FailedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

/// Errors returned by DescribeReportCreation
#[derive(Debug, PartialEq)]
pub enum DescribeReportCreationError {
    /// <p><p>The request was denied because performing this operation violates a constraint. </p> <p>Some of the reasons in the following list might not apply to this specific operation.</p> <ul> <li> <p>You must meet the prerequisites for using tag policies. For information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html">Prerequisites and Permissions for Using Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You must enable the tag policies service principal (<code>tagpolicies.tag.amazonaws.com</code>) to integrate with AWS Organizations For information, see <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html">EnableAWSServiceAccess</a>.</p> </li> <li> <p>You must have a tag policy attached to the organization root, an OU, or an account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl DescribeReportCreationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReportCreationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConstraintViolationException" => {
                    return RusotoError::Service(DescribeReportCreationError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeReportCreationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeReportCreationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(DescribeReportCreationError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReportCreationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReportCreationError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            DescribeReportCreationError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeReportCreationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeReportCreationError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReportCreationError {}
/// Errors returned by GetComplianceSummary
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryError {
    /// <p><p>The request was denied because performing this operation violates a constraint. </p> <p>Some of the reasons in the following list might not apply to this specific operation.</p> <ul> <li> <p>You must meet the prerequisites for using tag policies. For information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html">Prerequisites and Permissions for Using Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You must enable the tag policies service principal (<code>tagpolicies.tag.amazonaws.com</code>) to integrate with AWS Organizations For information, see <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html">EnableAWSServiceAccess</a>.</p> </li> <li> <p>You must have a tag policy attached to the organization root, an OU, or an account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetComplianceSummaryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetComplianceSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConstraintViolationException" => {
                    return RusotoError::Service(GetComplianceSummaryError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetComplianceSummaryError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetComplianceSummaryError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetComplianceSummaryError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetComplianceSummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComplianceSummaryError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            GetComplianceSummaryError::InternalService(ref cause) => write!(f, "{}", cause),
            GetComplianceSummaryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetComplianceSummaryError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetComplianceSummaryError {}
/// Errors returned by GetResources
#[derive(Debug, PartialEq)]
pub enum GetResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResourcesError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetResourcesError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResourcesError::PaginationTokenExpired(ref cause) => write!(f, "{}", cause),
            GetResourcesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourcesError {}
/// Errors returned by GetTagKeys
#[derive(Debug, PartialEq)]
pub enum GetTagKeysError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetTagKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetTagKeysError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetTagKeysError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetTagKeysError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTagKeysError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTagKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTagKeysError::InternalService(ref cause) => write!(f, "{}", cause),
            GetTagKeysError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetTagKeysError::PaginationTokenExpired(ref cause) => write!(f, "{}", cause),
            GetTagKeysError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTagKeysError {}
/// Errors returned by GetTagValues
#[derive(Debug, PartialEq)]
pub enum GetTagValuesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetTagValuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagValuesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetTagValuesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetTagValuesError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetTagValuesError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTagValuesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTagValuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTagValuesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetTagValuesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetTagValuesError::PaginationTokenExpired(ref cause) => write!(f, "{}", cause),
            GetTagValuesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTagValuesError {}
/// Errors returned by StartReportCreation
#[derive(Debug, PartialEq)]
pub enum StartReportCreationError {
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The request was denied because performing this operation violates a constraint. </p> <p>Some of the reasons in the following list might not apply to this specific operation.</p> <ul> <li> <p>You must meet the prerequisites for using tag policies. For information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html">Prerequisites and Permissions for Using Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You must enable the tag policies service principal (<code>tagpolicies.tag.amazonaws.com</code>) to integrate with AWS Organizations For information, see <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html">EnableAWSServiceAccess</a>.</p> </li> <li> <p>You must have a tag policy attached to the organization root, an OU, or an account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl StartReportCreationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartReportCreationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StartReportCreationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(StartReportCreationError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartReportCreationError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartReportCreationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(StartReportCreationError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartReportCreationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartReportCreationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::InternalService(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartReportCreationError {}
/// Errors returned by TagResources
#[derive(Debug, PartialEq)]
pub enum TagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl TagResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourcesError::InvalidParameter(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(TagResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            TagResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourcesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourcesError {}
/// Errors returned by UntagResources
#[derive(Debug, PartialEq)]
pub enum UntagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl UntagResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourcesError::InvalidParameter(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(UntagResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            UntagResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourcesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourcesError {}
/// Trait representing the capabilities of the AWS Resource Groups Tagging API API. AWS Resource Groups Tagging API clients implement this trait.
#[async_trait]
pub trait ResourceGroupsTaggingApi {
    /// <p>Describes the status of the <code>StartReportCreation</code> operation. </p> <p>You can call this operation only from the organization's master account and from the us-east-1 Region.</p>
    async fn describe_report_creation(
        &self,
    ) -> Result<DescribeReportCreationOutput, RusotoError<DescribeReportCreationError>>;

    /// <p>Returns a table that shows counts of resources that are noncompliant with their tag policies.</p> <p>For more information on tag policies, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> <p>You can call this operation only from the organization's master account and from the us-east-1 Region.</p>
    async fn get_compliance_summary(
        &self,
        input: GetComplianceSummaryInput,
    ) -> Result<GetComplianceSummaryOutput, RusotoError<GetComplianceSummaryError>>;

    /// <p><p>Returns all the tagged or previously tagged resources that are located in the specified Region for the AWS account.</p> <p>Depending on what information you want returned, you can also specify the following:</p> <ul> <li> <p> <i>Filters</i> that specify what tags and resource types you want returned. The response includes all tags that are associated with the requested resources.</p> </li> <li> <p>Information about compliance with the account&#39;s effective tag policy. For more information on tag policies, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <note> <p>You can check the <code>PaginationToken</code> response parameter to determine if a query is complete. Queries occasionally return fewer results on a page than allowed. The <code>PaginationToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display. </p> </note></p>
    async fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> Result<GetResourcesOutput, RusotoError<GetResourcesError>>;

    /// <p>Returns all tag keys in the specified Region for the AWS account.</p>
    async fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> Result<GetTagKeysOutput, RusotoError<GetTagKeysError>>;

    /// <p>Returns all tag values for the specified key in the specified Region for the AWS account.</p>
    async fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> Result<GetTagValuesOutput, RusotoError<GetTagValuesError>>;

    /// <p>Generates a report that lists all tagged resources in accounts across your organization and tells whether each resource is compliant with the effective tag policy. Compliance data is refreshed daily. </p> <p>The generated report is saved to the following location:</p> <p> <code>s3://example-bucket/AwsTagPolicies/o-exampleorgid/YYYY-MM-ddTHH:mm:ssZ/report.csv</code> </p> <p>You can call this operation only from the organization's master account and from the us-east-1 Region.</p>
    async fn start_report_creation(
        &self,
        input: StartReportCreationInput,
    ) -> Result<StartReportCreationOutput, RusotoError<StartReportCreationError>>;

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of services that support tagging, see <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/Welcome.html">this list</a>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference.</i> </p> </li> <li> <p>You can only tag resources that are located in the specified Region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/Welcome.html">this list</a>.</p> </li> </ul></p>
    async fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> Result<TagResourcesOutput, RusotoError<TagResourcesError>>;

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/Welcome.html">this list</a>.</p> </li> <li> <p>You can only tag resources that are located in the specified Region for the AWS account.</p> </li> </ul></p>
    async fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> Result<UntagResourcesOutput, RusotoError<UntagResourcesError>>;
}
/// A client for the AWS Resource Groups Tagging API API.
#[derive(Clone)]
pub struct ResourceGroupsTaggingApiClient {
    client: Client,
    region: region::Region,
}

impl ResourceGroupsTaggingApiClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ResourceGroupsTaggingApiClient {
        ResourceGroupsTaggingApiClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ResourceGroupsTaggingApiClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ResourceGroupsTaggingApiClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(
        client: Client,
        region: region::Region,
    ) -> ResourceGroupsTaggingApiClient {
        ResourceGroupsTaggingApiClient { client, region }
    }
}

#[async_trait]
impl ResourceGroupsTaggingApi for ResourceGroupsTaggingApiClient {
    /// <p>Describes the status of the <code>StartReportCreation</code> operation. </p> <p>You can call this operation only from the organization's master account and from the us-east-1 Region.</p>
    async fn describe_report_creation(
        &self,
    ) -> Result<DescribeReportCreationOutput, RusotoError<DescribeReportCreationError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.DescribeReportCreation",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeReportCreationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeReportCreationError::from_response(response))
        }
    }

    /// <p>Returns a table that shows counts of resources that are noncompliant with their tag policies.</p> <p>For more information on tag policies, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> <p>You can call this operation only from the organization's master account and from the us-east-1 Region.</p>
    async fn get_compliance_summary(
        &self,
        input: GetComplianceSummaryInput,
    ) -> Result<GetComplianceSummaryOutput, RusotoError<GetComplianceSummaryError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetComplianceSummary",
        );
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
                .deserialize::<GetComplianceSummaryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetComplianceSummaryError::from_response(response))
        }
    }

    /// <p><p>Returns all the tagged or previously tagged resources that are located in the specified Region for the AWS account.</p> <p>Depending on what information you want returned, you can also specify the following:</p> <ul> <li> <p> <i>Filters</i> that specify what tags and resource types you want returned. The response includes all tags that are associated with the requested resources.</p> </li> <li> <p>Information about compliance with the account&#39;s effective tag policy. For more information on tag policies, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <note> <p>You can check the <code>PaginationToken</code> response parameter to determine if a query is complete. Queries occasionally return fewer results on a page than allowed. The <code>PaginationToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display. </p> </note></p>
    async fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> Result<GetResourcesOutput, RusotoError<GetResourcesError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetResourcesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourcesError::from_response(response))
        }
    }

    /// <p>Returns all tag keys in the specified Region for the AWS account.</p>
    async fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> Result<GetTagKeysOutput, RusotoError<GetTagKeysError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetTagKeysOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetTagKeysError::from_response(response))
        }
    }

    /// <p>Returns all tag values for the specified key in the specified Region for the AWS account.</p>
    async fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> Result<GetTagValuesOutput, RusotoError<GetTagValuesError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagValues",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetTagValuesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetTagValuesError::from_response(response))
        }
    }

    /// <p>Generates a report that lists all tagged resources in accounts across your organization and tells whether each resource is compliant with the effective tag policy. Compliance data is refreshed daily. </p> <p>The generated report is saved to the following location:</p> <p> <code>s3://example-bucket/AwsTagPolicies/o-exampleorgid/YYYY-MM-ddTHH:mm:ssZ/report.csv</code> </p> <p>You can call this operation only from the organization's master account and from the us-east-1 Region.</p>
    async fn start_report_creation(
        &self,
        input: StartReportCreationInput,
    ) -> Result<StartReportCreationOutput, RusotoError<StartReportCreationError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.StartReportCreation",
        );
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
                .deserialize::<StartReportCreationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartReportCreationError::from_response(response))
        }
    }

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of services that support tagging, see <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/Welcome.html">this list</a>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference.</i> </p> </li> <li> <p>You can only tag resources that are located in the specified Region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/Welcome.html">this list</a>.</p> </li> </ul></p>
    async fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> Result<TagResourcesOutput, RusotoError<TagResourcesError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.TagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourcesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourcesError::from_response(response))
        }
    }

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/Welcome.html">this list</a>.</p> </li> <li> <p>You can only tag resources that are located in the specified Region for the AWS account.</p> </li> </ul></p>
    async fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> Result<UntagResourcesOutput, RusotoError<UntagResourcesError>> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.UntagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourcesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourcesError::from_response(response))
        }
    }
}
