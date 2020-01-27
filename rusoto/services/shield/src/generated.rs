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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateDRTLogBucketRequest {
    /// <p>The Amazon S3 bucket that contains your AWS WAF logs.</p>
    #[serde(rename = "LogBucket")]
    pub log_bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDRTLogBucketResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateDRTRoleRequest {
    /// <p>The Amazon Resource Name (ARN) of the role the DRT will use to access your AWS account.</p> <p>Prior to making the <code>AssociateDRTRole</code> request, you must attach the <a href="https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy">AWSShieldDRTAccessPolicy</a> managed policy to this role. For more information see <a href=" https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html">Attaching and Detaching IAM Policies</a>.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDRTRoleResponse {}

/// <p>The details of a DDoS attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackDetail {
    /// <p>List of counters that describe the attack for the specified time period.</p>
    #[serde(rename = "AttackCounters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_counters: Option<Vec<SummarizedCounter>>,
    /// <p>The unique identifier (ID) of the attack.</p>
    #[serde(rename = "AttackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_id: Option<String>,
    /// <p>The array of <a>AttackProperty</a> objects.</p>
    #[serde(rename = "AttackProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_properties: Option<Vec<AttackProperty>>,
    /// <p>The time the attack ended, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>List of mitigation actions taken for the attack.</p>
    #[serde(rename = "Mitigations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigations: Option<Vec<Mitigation>>,
    /// <p>The ARN (Amazon Resource Name) of the resource that was attacked.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The time the attack started, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>If applicable, additional detail about the resource being attacked, for example, IP address or URL.</p>
    #[serde(rename = "SubResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_resources: Option<Vec<SubResourceSummary>>,
}

/// <p>Details of the described attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackProperty {
    /// <p>The type of distributed denial of service (DDoS) event that was observed. <code>NETWORK</code> indicates layer 3 and layer 4 events and <code>APPLICATION</code> indicates layer 7 events.</p>
    #[serde(rename = "AttackLayer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_layer: Option<String>,
    /// <p>Defines the DDoS attack property information that is provided. The <code>WORDPRESS_PINGBACK_REFLECTOR</code> and <code>WORDPRESS_PINGBACK_SOURCE</code> values are valid only for WordPress reflective pingback DDoS attacks.</p>
    #[serde(rename = "AttackPropertyIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_property_identifier: Option<String>,
    /// <p>The array of <a>Contributor</a> objects that includes the top five contributors to an attack. </p>
    #[serde(rename = "TopContributors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_contributors: Option<Vec<Contributor>>,
    /// <p>The total contributions made to this attack by all contributors, not just the five listed in the <code>TopContributors</code> list.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// <p>The unit of the <code>Value</code> of the contributions.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Summarizes all DDoS attacks for a specified time period.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackSummary {
    /// <p>The unique identifier (ID) of the attack.</p>
    #[serde(rename = "AttackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_id: Option<String>,
    /// <p>The list of attacks for a specified time period.</p>
    #[serde(rename = "AttackVectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_vectors: Option<Vec<AttackVectorDescription>>,
    /// <p>The end time of the attack, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The ARN (Amazon Resource Name) of the resource that was attacked.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The start time of the attack, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>Describes the attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackVectorDescription {
    /// <p><p>The attack type. Valid values:</p> <ul> <li> <p>UDP<em>TRAFFIC</p> </li> <li> <p>UDP</em>FRAGMENT</p> </li> <li> <p>GENERIC<em>UDP</em>REFLECTION</p> </li> <li> <p>DNS<em>REFLECTION</p> </li> <li> <p>NTP</em>REFLECTION</p> </li> <li> <p>CHARGEN<em>REFLECTION</p> </li> <li> <p>SSDP</em>REFLECTION</p> </li> <li> <p>PORT<em>MAPPER</p> </li> <li> <p>RIP</em>REFLECTION</p> </li> <li> <p>SNMP<em>REFLECTION</p> </li> <li> <p>MSSQL</em>REFLECTION</p> </li> <li> <p>NET<em>BIOS</em>REFLECTION</p> </li> <li> <p>SYN<em>FLOOD</p> </li> <li> <p>ACK</em>FLOOD</p> </li> <li> <p>REQUEST<em>FLOOD</p> </li> <li> <p>HTTP</em>REFLECTION</p> </li> <li> <p>UDS<em>REFLECTION</p> </li> <li> <p>MEMCACHED</em>REFLECTION</p> </li> </ul></p>
    #[serde(rename = "VectorType")]
    pub vector_type: String,
}

/// <p>A contributor to the attack and their contribution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Contributor {
    /// <p>The name of the contributor. This is dependent on the <code>AttackPropertyIdentifier</code>. For example, if the <code>AttackPropertyIdentifier</code> is <code>SOURCE_COUNTRY</code>, the <code>Name</code> could be <code>United States</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The contribution of this contributor expressed in <a>Protection</a> units. For example <code>10,000</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProtectionRequest {
    /// <p>Friendly name for the <code>Protection</code> you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The ARN (Amazon Resource Name) of the resource to be protected.</p> <p>The ARN should be in one of the following formats:</p> <ul> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Elastic Load Balancer (Classic Load Balancer): <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/<i>load-balancer-name</i> </code> </p> </li> <li> <p>For an AWS CloudFront distribution: <code>arn:aws:cloudfront::<i>account-id</i>:distribution/<i>distribution-id</i> </code> </p> </li> <li> <p>For an AWS Global Accelerator accelerator: <code>arn:aws:globalaccelerator::<i>account-id</i>:accelerator/<i>accelerator-id</i> </code> </p> </li> <li> <p>For Amazon Route 53: <code>arn:aws:route53:::hostedzone/<i>hosted-zone-id</i> </code> </p> </li> <li> <p>For an Elastic IP address: <code>arn:aws:ec2:<i>region</i>:<i>account-id</i>:eip-allocation/<i>allocation-id</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProtectionResponse {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object that is created.</p>
    #[serde(rename = "ProtectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSubscriptionRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSubscriptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProtectionRequest {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object to be deleted.</p>
    #[serde(rename = "ProtectionId")]
    pub protection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProtectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSubscriptionRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSubscriptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAttackRequest {
    /// <p>The unique identifier (ID) for the attack that to be described.</p>
    #[serde(rename = "AttackId")]
    pub attack_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAttackResponse {
    /// <p>The attack that is described.</p>
    #[serde(rename = "Attack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<AttackDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDRTAccessRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDRTAccessResponse {
    /// <p>The list of Amazon S3 buckets accessed by the DRT.</p>
    #[serde(rename = "LogBucketList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_bucket_list: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the role the DRT used to access your AWS account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEmergencyContactSettingsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEmergencyContactSettingsResponse {
    /// <p>A list of email addresses that the DRT can use to contact you during a suspected attack.</p>
    #[serde(rename = "EmergencyContactList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProtectionRequest {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object that is described. When submitting the <code>DescribeProtection</code> request you must provide either the <code>ResourceArn</code> or the <code>ProtectionID</code>, but not both.</p>
    #[serde(rename = "ProtectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_id: Option<String>,
    /// <p>The ARN (Amazon Resource Name) of the AWS resource for the <a>Protection</a> object that is described. When submitting the <code>DescribeProtection</code> request you must provide either the <code>ResourceArn</code> or the <code>ProtectionID</code>, but not both.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProtectionResponse {
    /// <p>The <a>Protection</a> object that is described.</p>
    #[serde(rename = "Protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<Protection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSubscriptionRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSubscriptionResponse {
    /// <p>The AWS Shield Advanced subscription details for an account.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateDRTLogBucketRequest {
    /// <p>The Amazon S3 bucket that contains your AWS WAF logs.</p>
    #[serde(rename = "LogBucket")]
    pub log_bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDRTLogBucketResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateDRTRoleRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDRTRoleResponse {}

/// <p>Contact information that the DRT can use to contact you during a suspected attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyContact {
    /// <p>An email address that the DRT can use to contact you during a suspected attack.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSubscriptionStateRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSubscriptionStateResponse {
    /// <p>The status of the subscription.</p>
    #[serde(rename = "SubscriptionState")]
    pub subscription_state: String,
}

/// <p>Specifies how many protections of a given type you can create.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Limit {
    /// <p>The maximum number of protections that can be created for the specified <code>Type</code>.</p>
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    /// <p>The type of protection.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAttacksRequest {
    /// <p>The end of the time period for the attacks. This is a <code>timestamp</code> type. The sample request above indicates a <code>number</code> type because the default used by WAF is Unix time in seconds. However any valid <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp format</a> is allowed. </p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeRange>,
    /// <p>The maximum number of <a>AttackSummary</a> objects to be returned. If this is left blank, the first 20 results will be returned.</p> <p>This is a maximum value; it is possible that AWS WAF will return the results in smaller batches. That is, the number of <a>AttackSummary</a> objects returned could be less than <code>MaxResults</code>, even if there are still more <a>AttackSummary</a> objects yet to return. If there are more <a>AttackSummary</a> objects to return, AWS WAF will always also return a <code>NextToken</code>.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>ListAttacksRequest.NextMarker</code> value from a previous call to <code>ListAttacksRequest</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN (Amazon Resource Name) of the resource that was attacked. If this is left blank, all applicable resources for this account will be included.</p>
    #[serde(rename = "ResourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    /// <p>The start of the time period for the attacks. This is a <code>timestamp</code> type. The sample request above indicates a <code>number</code> type because the default used by WAF is Unix time in seconds. However any valid <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp format</a> is allowed. </p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeRange>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAttacksResponse {
    /// <p>The attack information for the specified time range.</p>
    #[serde(rename = "AttackSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_summaries: Option<Vec<AttackSummary>>,
    /// <p>The token returned by a previous call to indicate that there is more data available. If not null, more results are available. Pass this value for the <code>NextMarker</code> parameter in a subsequent call to <code>ListAttacks</code> to retrieve the next set of items.</p> <p>AWS WAF might return the list of <a>AttackSummary</a> objects in batches smaller than the number specified by MaxResults. If there are more <a>AttackSummary</a> objects to return, AWS WAF will always also return a <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProtectionsRequest {
    /// <p>The maximum number of <a>Protection</a> objects to be returned. If this is left blank the first 20 results will be returned.</p> <p>This is a maximum value; it is possible that AWS WAF will return the results in smaller batches. That is, the number of <a>Protection</a> objects returned could be less than <code>MaxResults</code>, even if there are still more <a>Protection</a> objects yet to return. If there are more <a>Protection</a> objects to return, AWS WAF will always also return a <code>NextToken</code>.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>ListProtectionsRequest.NextToken</code> value from a previous call to <code>ListProtections</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProtectionsResponse {
    /// <p>If you specify a value for <code>MaxResults</code> and you have more Protections than the value of MaxResults, AWS Shield Advanced returns a NextToken value in the response that allows you to list another group of Protections. For the second and subsequent ListProtections requests, specify the value of NextToken from the previous response to get information about another batch of Protections.</p> <p>AWS WAF might return the list of <a>Protection</a> objects in batches smaller than the number specified by MaxResults. If there are more <a>Protection</a> objects to return, AWS WAF will always also return a <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The array of enabled <a>Protection</a> objects.</p>
    #[serde(rename = "Protections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protections: Option<Vec<Protection>>,
}

/// <p>The mitigation applied to a DDoS attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Mitigation {
    /// <p>The name of the mitigation taken for this attack.</p>
    #[serde(rename = "MitigationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_name: Option<String>,
}

/// <p>An object that represents a resource that is under DDoS protection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Protection {
    /// <p>The unique identifier (ID) of the protection.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the protection. For example, <code>My CloudFront distributions</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN (Amazon Resource Name) of the AWS resource that is protected.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

/// <p>The attack information for the specified SubResource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubResourceSummary {
    /// <p>The list of attack types and associated counters.</p>
    #[serde(rename = "AttackVectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_vectors: Option<Vec<SummarizedAttackVector>>,
    /// <p>The counters that describe the details of the attack.</p>
    #[serde(rename = "Counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<SummarizedCounter>>,
    /// <p>The unique identifier (ID) of the <code>SubResource</code>.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The <code>SubResource</code> type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about the AWS Shield Advanced subscription for an account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Subscription {
    /// <p>If <code>ENABLED</code>, the subscription will be automatically renewed at the end of the existing subscription period.</p> <p>When you initally create a subscription, <code>AutoRenew</code> is set to <code>ENABLED</code>. You can change this by submitting an <code>UpdateSubscription</code> request. If the <code>UpdateSubscription</code> request does not included a value for <code>AutoRenew</code>, the existing value for <code>AutoRenew</code> remains unchanged.</p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// <p>The date and time your subscription will end.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Specifies how many protections of a given type you can create.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Vec<Limit>>,
    /// <p>The start time of the subscription, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The length, in seconds, of the AWS Shield Advanced subscription for the account.</p>
    #[serde(rename = "TimeCommitmentInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_commitment_in_seconds: Option<i64>,
}

/// <p>A summary of information about the attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SummarizedAttackVector {
    /// <p>The list of counters that describe the details of the attack.</p>
    #[serde(rename = "VectorCounters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_counters: Option<Vec<SummarizedCounter>>,
    /// <p>The attack type, for example, SNMP reflection or SYN flood.</p>
    #[serde(rename = "VectorType")]
    pub vector_type: String,
}

/// <p>The counter that describes a DDoS attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SummarizedCounter {
    /// <p>The average value of the counter for a specified time period.</p>
    #[serde(rename = "Average")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    /// <p>The maximum value of the counter for a specified time period.</p>
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// <p>The number of counters for a specified time period.</p>
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// <p>The counter name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The total of counter values for a specified time period.</p>
    #[serde(rename = "Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    /// <p>The unit of the counters.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>The time range.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TimeRange {
    /// <p>The start time, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "FromInclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_inclusive: Option<f64>,
    /// <p>The end time, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "ToExclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_exclusive: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEmergencyContactSettingsRequest {
    /// <p>A list of email addresses that the DRT can use to contact you during a suspected attack.</p>
    #[serde(rename = "EmergencyContactList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEmergencyContactSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSubscriptionRequest {
    /// <p>When you initally create a subscription, <code>AutoRenew</code> is set to <code>ENABLED</code>. If <code>ENABLED</code>, the subscription will be automatically renewed at the end of the existing subscription period. You can change this by submitting an <code>UpdateSubscription</code> request. If the <code>UpdateSubscription</code> request does not included a value for <code>AutoRenew</code>, the existing value for <code>AutoRenew</code> remains unchanged.</p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSubscriptionResponse {}

/// Errors returned by AssociateDRTLogBucket
#[derive(Debug, PartialEq)]
pub enum AssociateDRTLogBucketError {
    /// <p>In order to grant the necessary access to the DDoS Response Team, the user submitting <code>AssociateDRTRole</code> must have the <code>iam:PassRole</code> permission. This error indicates the user did not have the appropriate permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p>
    AccessDeniedForDependency(String),
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the operation would exceed a limit.</p> <p> <code>Type</code> is the type of limit that would be exceeded.</p> <p> <code>Limit</code> is the threshold that would be exceeded.</p>
    LimitsExceeded(String),
    /// <p>The ARN of the role that you specifed does not exist.</p>
    NoAssociatedRole(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl AssociateDRTLogBucketError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateDRTLogBucketError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedForDependencyException" => {
                    return RusotoError::Service(
                        AssociateDRTLogBucketError::AccessDeniedForDependency(err.msg),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AssociateDRTLogBucketError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(AssociateDRTLogBucketError::InvalidOperation(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateDRTLogBucketError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitsExceededException" => {
                    return RusotoError::Service(AssociateDRTLogBucketError::LimitsExceeded(
                        err.msg,
                    ))
                }
                "NoAssociatedRoleException" => {
                    return RusotoError::Service(AssociateDRTLogBucketError::NoAssociatedRole(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(AssociateDRTLogBucketError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateDRTLogBucketError::ResourceNotFound(
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
impl fmt::Display for AssociateDRTLogBucketError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateDRTLogBucketError::AccessDeniedForDependency(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateDRTLogBucketError::InternalError(ref cause) => write!(f, "{}", cause),
            AssociateDRTLogBucketError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            AssociateDRTLogBucketError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateDRTLogBucketError::LimitsExceeded(ref cause) => write!(f, "{}", cause),
            AssociateDRTLogBucketError::NoAssociatedRole(ref cause) => write!(f, "{}", cause),
            AssociateDRTLogBucketError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            AssociateDRTLogBucketError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateDRTLogBucketError {}
/// Errors returned by AssociateDRTRole
#[derive(Debug, PartialEq)]
pub enum AssociateDRTRoleError {
    /// <p>In order to grant the necessary access to the DDoS Response Team, the user submitting <code>AssociateDRTRole</code> must have the <code>iam:PassRole</code> permission. This error indicates the user did not have the appropriate permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p>
    AccessDeniedForDependency(String),
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl AssociateDRTRoleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateDRTRoleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedForDependencyException" => {
                    return RusotoError::Service(AssociateDRTRoleError::AccessDeniedForDependency(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AssociateDRTRoleError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(AssociateDRTRoleError::InvalidOperation(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateDRTRoleError::InvalidParameter(err.msg))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(AssociateDRTRoleError::OptimisticLock(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateDRTRoleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateDRTRoleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateDRTRoleError::AccessDeniedForDependency(ref cause) => write!(f, "{}", cause),
            AssociateDRTRoleError::InternalError(ref cause) => write!(f, "{}", cause),
            AssociateDRTRoleError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            AssociateDRTRoleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateDRTRoleError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            AssociateDRTRoleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateDRTRoleError {}
/// Errors returned by CreateProtection
#[derive(Debug, PartialEq)]
pub enum CreateProtectionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the resource is invalid. You might not have access to the resource, or the resource might not exist.</p>
    InvalidResource(String),
    /// <p>Exception that indicates that the operation would exceed a limit.</p> <p> <code>Type</code> is the type of limit that would be exceeded.</p> <p> <code>Limit</code> is the threshold that would be exceeded.</p>
    LimitsExceeded(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl CreateProtectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProtectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateProtectionError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(CreateProtectionError::InvalidOperation(err.msg))
                }
                "InvalidResourceException" => {
                    return RusotoError::Service(CreateProtectionError::InvalidResource(err.msg))
                }
                "LimitsExceededException" => {
                    return RusotoError::Service(CreateProtectionError::LimitsExceeded(err.msg))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(CreateProtectionError::OptimisticLock(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateProtectionError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateProtectionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProtectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProtectionError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateProtectionError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            CreateProtectionError::InvalidResource(ref cause) => write!(f, "{}", cause),
            CreateProtectionError::LimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateProtectionError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            CreateProtectionError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateProtectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProtectionError {}
/// Errors returned by CreateSubscription
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreateSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateSubscriptionError::InternalError(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateSubscriptionError::ResourceAlreadyExists(
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
impl fmt::Display for CreateSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSubscriptionError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateSubscriptionError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSubscriptionError {}
/// Errors returned by DeleteProtection
#[derive(Debug, PartialEq)]
pub enum DeleteProtectionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DeleteProtectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProtectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteProtectionError::InternalError(err.msg))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(DeleteProtectionError::OptimisticLock(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProtectionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProtectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProtectionError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteProtectionError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            DeleteProtectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProtectionError {}
/// Errors returned by DeleteSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>You are trying to update a subscription that has not yet completed the 1-year commitment. You can change the <code>AutoRenew</code> parameter during the last 30 days of your subscription. This exception indicates that you are attempting to change <code>AutoRenew</code> prior to that period.</p>
    LockedSubscription(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DeleteSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteSubscriptionError::InternalError(err.msg))
                }
                "LockedSubscriptionException" => {
                    return RusotoError::Service(DeleteSubscriptionError::LockedSubscription(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSubscriptionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSubscriptionError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteSubscriptionError::LockedSubscription(ref cause) => write!(f, "{}", cause),
            DeleteSubscriptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSubscriptionError {}
/// Errors returned by DescribeAttack
#[derive(Debug, PartialEq)]
pub enum DescribeAttackError {
    /// <p>Exception that indicates the specified <code>AttackId</code> does not exist, or the requester does not have the appropriate permissions to access the <code>AttackId</code>.</p>
    AccessDenied(String),
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
}

impl DescribeAttackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAttackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAttackError::AccessDenied(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeAttackError::InternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAttackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAttackError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeAttackError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAttackError {}
/// Errors returned by DescribeDRTAccess
#[derive(Debug, PartialEq)]
pub enum DescribeDRTAccessError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeDRTAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDRTAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeDRTAccessError::InternalError(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDRTAccessError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDRTAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDRTAccessError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeDRTAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDRTAccessError {}
/// Errors returned by DescribeEmergencyContactSettings
#[derive(Debug, PartialEq)]
pub enum DescribeEmergencyContactSettingsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeEmergencyContactSettingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEmergencyContactSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(
                        DescribeEmergencyContactSettingsError::InternalError(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeEmergencyContactSettingsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEmergencyContactSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEmergencyContactSettingsError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEmergencyContactSettingsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEmergencyContactSettingsError {}
/// Errors returned by DescribeProtection
#[derive(Debug, PartialEq)]
pub enum DescribeProtectionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeProtectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProtectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeProtectionError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeProtectionError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProtectionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProtectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProtectionError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeProtectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeProtectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProtectionError {}
/// Errors returned by DescribeSubscription
#[derive(Debug, PartialEq)]
pub enum DescribeSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeSubscriptionError::InternalError(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSubscriptionError::ResourceNotFound(
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
impl fmt::Display for DescribeSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSubscriptionError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeSubscriptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSubscriptionError {}
/// Errors returned by DisassociateDRTLogBucket
#[derive(Debug, PartialEq)]
pub enum DisassociateDRTLogBucketError {
    /// <p>In order to grant the necessary access to the DDoS Response Team, the user submitting <code>AssociateDRTRole</code> must have the <code>iam:PassRole</code> permission. This error indicates the user did not have the appropriate permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p>
    AccessDeniedForDependency(String),
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>The ARN of the role that you specifed does not exist.</p>
    NoAssociatedRole(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DisassociateDRTLogBucketError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateDRTLogBucketError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedForDependencyException" => {
                    return RusotoError::Service(
                        DisassociateDRTLogBucketError::AccessDeniedForDependency(err.msg),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DisassociateDRTLogBucketError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DisassociateDRTLogBucketError::InvalidOperation(
                        err.msg,
                    ))
                }
                "NoAssociatedRoleException" => {
                    return RusotoError::Service(DisassociateDRTLogBucketError::NoAssociatedRole(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(DisassociateDRTLogBucketError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateDRTLogBucketError::ResourceNotFound(
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
impl fmt::Display for DisassociateDRTLogBucketError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateDRTLogBucketError::AccessDeniedForDependency(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateDRTLogBucketError::InternalError(ref cause) => write!(f, "{}", cause),
            DisassociateDRTLogBucketError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DisassociateDRTLogBucketError::NoAssociatedRole(ref cause) => write!(f, "{}", cause),
            DisassociateDRTLogBucketError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            DisassociateDRTLogBucketError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateDRTLogBucketError {}
/// Errors returned by DisassociateDRTRole
#[derive(Debug, PartialEq)]
pub enum DisassociateDRTRoleError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl DisassociateDRTRoleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateDRTRoleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DisassociateDRTRoleError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DisassociateDRTRoleError::InvalidOperation(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(DisassociateDRTRoleError::OptimisticLock(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateDRTRoleError::ResourceNotFound(
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
impl fmt::Display for DisassociateDRTRoleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateDRTRoleError::InternalError(ref cause) => write!(f, "{}", cause),
            DisassociateDRTRoleError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DisassociateDRTRoleError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            DisassociateDRTRoleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateDRTRoleError {}
/// Errors returned by GetSubscriptionState
#[derive(Debug, PartialEq)]
pub enum GetSubscriptionStateError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
}

impl GetSubscriptionStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSubscriptionStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetSubscriptionStateError::InternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSubscriptionStateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSubscriptionStateError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSubscriptionStateError {}
/// Errors returned by ListAttacks
#[derive(Debug, PartialEq)]
pub enum ListAttacksError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
}

impl ListAttacksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAttacksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListAttacksError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(ListAttacksError::InvalidOperation(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAttacksError::InvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAttacksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAttacksError::InternalError(ref cause) => write!(f, "{}", cause),
            ListAttacksError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            ListAttacksError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAttacksError {}
/// Errors returned by ListProtections
#[derive(Debug, PartialEq)]
pub enum ListProtectionsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the NextToken specified in the request is invalid. Submit the request using the NextToken value that was returned in the response.</p>
    InvalidPaginationToken(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl ListProtectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProtectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListProtectionsError::InternalError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListProtectionsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProtectionsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProtectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProtectionsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListProtectionsError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListProtectionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProtectionsError {}
/// Errors returned by UpdateEmergencyContactSettings
#[derive(Debug, PartialEq)]
pub enum UpdateEmergencyContactSettingsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl UpdateEmergencyContactSettingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateEmergencyContactSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(
                        UpdateEmergencyContactSettingsError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateEmergencyContactSettingsError::InvalidParameter(err.msg),
                    )
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(
                        UpdateEmergencyContactSettingsError::OptimisticLock(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateEmergencyContactSettingsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEmergencyContactSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEmergencyContactSettingsError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateEmergencyContactSettingsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateEmergencyContactSettingsError::OptimisticLock(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateEmergencyContactSettingsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateEmergencyContactSettingsError {}
/// Errors returned by UpdateSubscription
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
    /// <p>You are trying to update a subscription that has not yet completed the 1-year commitment. You can change the <code>AutoRenew</code> parameter during the last 30 days of your subscription. This exception indicates that you are attempting to change <code>AutoRenew</code> prior to that period.</p>
    LockedSubscription(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl UpdateSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateSubscriptionError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateSubscriptionError::InvalidParameter(err.msg))
                }
                "LockedSubscriptionException" => {
                    return RusotoError::Service(UpdateSubscriptionError::LockedSubscription(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(UpdateSubscriptionError::OptimisticLock(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateSubscriptionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSubscriptionError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateSubscriptionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateSubscriptionError::LockedSubscription(ref cause) => write!(f, "{}", cause),
            UpdateSubscriptionError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            UpdateSubscriptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSubscriptionError {}
/// Trait representing the capabilities of the AWS Shield API. AWS Shield clients implement this trait.
#[async_trait]
pub trait Shield {
    /// <p>Authorizes the DDoS Response team (DRT) to access the specified Amazon S3 bucket containing your AWS WAF logs. You can associate up to 10 Amazon S3 buckets with your subscription.</p> <p>To use the services of the DRT and make an <code>AssociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_log_bucket(
        &self,
        input: AssociateDRTLogBucketRequest,
    ) -> Result<AssociateDRTLogBucketResponse, RusotoError<AssociateDRTLogBucketError>>;

    /// <p>Authorizes the DDoS Response team (DRT), using the specified role, to access your AWS account to assist with DDoS attack mitigation during potential attacks. This enables the DRT to inspect your AWS WAF configuration and create or update AWS WAF rules and web ACLs.</p> <p>You can associate only one <code>RoleArn</code> with your subscription. If you submit an <code>AssociateDRTRole</code> request for an account that already has an associated role, the new <code>RoleArn</code> will replace the existing <code>RoleArn</code>. </p> <p>Prior to making the <code>AssociateDRTRole</code> request, you must attach the <a href="https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy">AWSShieldDRTAccessPolicy</a> managed policy to the role you will specify in the request. For more information see <a href=" https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html">Attaching and Detaching IAM Policies</a>. The role must also trust the service principal <code> drt.shield.amazonaws.com</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html">IAM JSON Policy Elements: Principal</a>.</p> <p>The DRT will have access only to your AWS WAF and Shield resources. By submitting this request, you authorize the DRT to inspect your AWS WAF and Shield configuration and create and update AWS WAF rules and web ACLs on your behalf. The DRT takes these actions only if explicitly authorized by you.</p> <p>You must have the <code>iam:PassRole</code> permission to make an <code>AssociateDRTRole</code> request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p> <p>To use the services of the DRT and make an <code>AssociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_role(
        &self,
        input: AssociateDRTRoleRequest,
    ) -> Result<AssociateDRTRoleResponse, RusotoError<AssociateDRTRoleError>>;

    /// <p>Enables AWS Shield Advanced for a specific AWS resource. The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, AWS Global Accelerator accelerator, Elastic IP Address, or an Amazon Route 53 hosted zone.</p> <p>You can add protection to only a single resource with each CreateProtection request. If you want to add protection to multiple resources at once, use the <a href="https://console.aws.amazon.com/waf/">AWS WAF console</a>. For more information see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/getting-started-ddos.html">Getting Started with AWS Shield Advanced</a> and <a href="https://docs.aws.amazon.com/waf/latest/developerguide/configure-new-protection.html">Add AWS Shield Advanced Protection to more AWS Resources</a>.</p>
    async fn create_protection(
        &self,
        input: CreateProtectionRequest,
    ) -> Result<CreateProtectionResponse, RusotoError<CreateProtectionError>>;

    /// <p>Activates AWS Shield Advanced for an account.</p> <p>As part of this request you can specify <code>EmergencySettings</code> that automaticaly grant the DDoS response team (DRT) needed permissions to assist you during a suspected DDoS attack. For more information see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/authorize-DRT.html">Authorize the DDoS Response Team to Create Rules and Web ACLs on Your Behalf</a>.</p> <p>To use the services of the DRT, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p> <p>When you initally create a subscription, your subscription is set to be automatically renewed at the end of the existing subscription period. You can change this by submitting an <code>UpdateSubscription</code> request. </p>
    async fn create_subscription(
        &self,
    ) -> Result<CreateSubscriptionResponse, RusotoError<CreateSubscriptionError>>;

    /// <p>Deletes an AWS Shield Advanced <a>Protection</a>.</p>
    async fn delete_protection(
        &self,
        input: DeleteProtectionRequest,
    ) -> Result<DeleteProtectionResponse, RusotoError<DeleteProtectionError>>;

    /// <p>Removes AWS Shield Advanced from an account. AWS Shield Advanced requires a 1-year subscription commitment. You cannot delete a subscription prior to the completion of that commitment. </p>
    async fn delete_subscription(
        &self,
    ) -> Result<DeleteSubscriptionResponse, RusotoError<DeleteSubscriptionError>>;

    /// <p>Describes the details of a DDoS attack. </p>
    async fn describe_attack(
        &self,
        input: DescribeAttackRequest,
    ) -> Result<DescribeAttackResponse, RusotoError<DescribeAttackError>>;

    /// <p>Returns the current role and list of Amazon S3 log buckets used by the DDoS Response team (DRT) to access your AWS account while assisting with attack mitigation.</p>
    async fn describe_drt_access(
        &self,
    ) -> Result<DescribeDRTAccessResponse, RusotoError<DescribeDRTAccessError>>;

    /// <p>Lists the email addresses that the DRT can use to contact you during a suspected attack.</p>
    async fn describe_emergency_contact_settings(
        &self,
    ) -> Result<
        DescribeEmergencyContactSettingsResponse,
        RusotoError<DescribeEmergencyContactSettingsError>,
    >;

    /// <p>Lists the details of a <a>Protection</a> object.</p>
    async fn describe_protection(
        &self,
        input: DescribeProtectionRequest,
    ) -> Result<DescribeProtectionResponse, RusotoError<DescribeProtectionError>>;

    /// <p>Provides details about the AWS Shield Advanced subscription for an account.</p>
    async fn describe_subscription(
        &self,
    ) -> Result<DescribeSubscriptionResponse, RusotoError<DescribeSubscriptionError>>;

    /// <p>Removes the DDoS Response team's (DRT) access to the specified Amazon S3 bucket containing your AWS WAF logs.</p> <p>To make a <code>DisassociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTLogBucket</code> request to remove this access.</p>
    async fn disassociate_drt_log_bucket(
        &self,
        input: DisassociateDRTLogBucketRequest,
    ) -> Result<DisassociateDRTLogBucketResponse, RusotoError<DisassociateDRTLogBucketError>>;

    /// <p>Removes the DDoS Response team's (DRT) access to your AWS account.</p> <p>To make a <code>DisassociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTRole</code> request to remove this access.</p>
    async fn disassociate_drt_role(
        &self,
    ) -> Result<DisassociateDRTRoleResponse, RusotoError<DisassociateDRTRoleError>>;

    /// <p>Returns the <code>SubscriptionState</code>, either <code>Active</code> or <code>Inactive</code>.</p>
    async fn get_subscription_state(
        &self,
    ) -> Result<GetSubscriptionStateResponse, RusotoError<GetSubscriptionStateError>>;

    /// <p>Returns all ongoing DDoS attacks or all DDoS attacks during a specified time period.</p>
    async fn list_attacks(
        &self,
        input: ListAttacksRequest,
    ) -> Result<ListAttacksResponse, RusotoError<ListAttacksError>>;

    /// <p>Lists all <a>Protection</a> objects for the account.</p>
    async fn list_protections(
        &self,
        input: ListProtectionsRequest,
    ) -> Result<ListProtectionsResponse, RusotoError<ListProtectionsError>>;

    /// <p>Updates the details of the list of email addresses that the DRT can use to contact you during a suspected attack.</p>
    async fn update_emergency_contact_settings(
        &self,
        input: UpdateEmergencyContactSettingsRequest,
    ) -> Result<
        UpdateEmergencyContactSettingsResponse,
        RusotoError<UpdateEmergencyContactSettingsError>,
    >;

    /// <p>Updates the details of an existing subscription. Only enter values for parameters you want to change. Empty parameters are not updated.</p>
    async fn update_subscription(
        &self,
        input: UpdateSubscriptionRequest,
    ) -> Result<UpdateSubscriptionResponse, RusotoError<UpdateSubscriptionError>>;
}
/// A client for the AWS Shield API.
#[derive(Clone)]
pub struct ShieldClient {
    client: Client,
    region: region::Region,
}

impl ShieldClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ShieldClient {
        ShieldClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ShieldClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ShieldClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ShieldClient {
        ShieldClient { client, region }
    }
}

#[async_trait]
impl Shield for ShieldClient {
    /// <p>Authorizes the DDoS Response team (DRT) to access the specified Amazon S3 bucket containing your AWS WAF logs. You can associate up to 10 Amazon S3 buckets with your subscription.</p> <p>To use the services of the DRT and make an <code>AssociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_log_bucket(
        &self,
        input: AssociateDRTLogBucketRequest,
    ) -> Result<AssociateDRTLogBucketResponse, RusotoError<AssociateDRTLogBucketError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.AssociateDRTLogBucket");
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
                .deserialize::<AssociateDRTLogBucketResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateDRTLogBucketError::from_response(response))
        }
    }

    /// <p>Authorizes the DDoS Response team (DRT), using the specified role, to access your AWS account to assist with DDoS attack mitigation during potential attacks. This enables the DRT to inspect your AWS WAF configuration and create or update AWS WAF rules and web ACLs.</p> <p>You can associate only one <code>RoleArn</code> with your subscription. If you submit an <code>AssociateDRTRole</code> request for an account that already has an associated role, the new <code>RoleArn</code> will replace the existing <code>RoleArn</code>. </p> <p>Prior to making the <code>AssociateDRTRole</code> request, you must attach the <a href="https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy">AWSShieldDRTAccessPolicy</a> managed policy to the role you will specify in the request. For more information see <a href=" https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html">Attaching and Detaching IAM Policies</a>. The role must also trust the service principal <code> drt.shield.amazonaws.com</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html">IAM JSON Policy Elements: Principal</a>.</p> <p>The DRT will have access only to your AWS WAF and Shield resources. By submitting this request, you authorize the DRT to inspect your AWS WAF and Shield configuration and create and update AWS WAF rules and web ACLs on your behalf. The DRT takes these actions only if explicitly authorized by you.</p> <p>You must have the <code>iam:PassRole</code> permission to make an <code>AssociateDRTRole</code> request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p> <p>To use the services of the DRT and make an <code>AssociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_role(
        &self,
        input: AssociateDRTRoleRequest,
    ) -> Result<AssociateDRTRoleResponse, RusotoError<AssociateDRTRoleError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.AssociateDRTRole");
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
                .deserialize::<AssociateDRTRoleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateDRTRoleError::from_response(response))
        }
    }

    /// <p>Enables AWS Shield Advanced for a specific AWS resource. The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, AWS Global Accelerator accelerator, Elastic IP Address, or an Amazon Route 53 hosted zone.</p> <p>You can add protection to only a single resource with each CreateProtection request. If you want to add protection to multiple resources at once, use the <a href="https://console.aws.amazon.com/waf/">AWS WAF console</a>. For more information see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/getting-started-ddos.html">Getting Started with AWS Shield Advanced</a> and <a href="https://docs.aws.amazon.com/waf/latest/developerguide/configure-new-protection.html">Add AWS Shield Advanced Protection to more AWS Resources</a>.</p>
    async fn create_protection(
        &self,
        input: CreateProtectionRequest,
    ) -> Result<CreateProtectionResponse, RusotoError<CreateProtectionError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.CreateProtection");
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
                .deserialize::<CreateProtectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProtectionError::from_response(response))
        }
    }

    /// <p>Activates AWS Shield Advanced for an account.</p> <p>As part of this request you can specify <code>EmergencySettings</code> that automaticaly grant the DDoS response team (DRT) needed permissions to assist you during a suspected DDoS attack. For more information see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/authorize-DRT.html">Authorize the DDoS Response Team to Create Rules and Web ACLs on Your Behalf</a>.</p> <p>To use the services of the DRT, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p> <p>When you initally create a subscription, your subscription is set to be automatically renewed at the end of the existing subscription period. You can change this by submitting an <code>UpdateSubscription</code> request. </p>
    async fn create_subscription(
        &self,
    ) -> Result<CreateSubscriptionResponse, RusotoError<CreateSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.CreateSubscription");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSubscriptionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSubscriptionError::from_response(response))
        }
    }

    /// <p>Deletes an AWS Shield Advanced <a>Protection</a>.</p>
    async fn delete_protection(
        &self,
        input: DeleteProtectionRequest,
    ) -> Result<DeleteProtectionResponse, RusotoError<DeleteProtectionError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DeleteProtection");
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
                .deserialize::<DeleteProtectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProtectionError::from_response(response))
        }
    }

    /// <p>Removes AWS Shield Advanced from an account. AWS Shield Advanced requires a 1-year subscription commitment. You cannot delete a subscription prior to the completion of that commitment. </p>
    async fn delete_subscription(
        &self,
    ) -> Result<DeleteSubscriptionResponse, RusotoError<DeleteSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DeleteSubscription");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSubscriptionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSubscriptionError::from_response(response))
        }
    }

    /// <p>Describes the details of a DDoS attack. </p>
    async fn describe_attack(
        &self,
        input: DescribeAttackRequest,
    ) -> Result<DescribeAttackResponse, RusotoError<DescribeAttackError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeAttack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeAttackResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAttackError::from_response(response))
        }
    }

    /// <p>Returns the current role and list of Amazon S3 log buckets used by the DDoS Response team (DRT) to access your AWS account while assisting with attack mitigation.</p>
    async fn describe_drt_access(
        &self,
    ) -> Result<DescribeDRTAccessResponse, RusotoError<DescribeDRTAccessError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeDRTAccess");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDRTAccessResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDRTAccessError::from_response(response))
        }
    }

    /// <p>Lists the email addresses that the DRT can use to contact you during a suspected attack.</p>
    async fn describe_emergency_contact_settings(
        &self,
    ) -> Result<
        DescribeEmergencyContactSettingsResponse,
        RusotoError<DescribeEmergencyContactSettingsError>,
    > {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.DescribeEmergencyContactSettings",
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
                .deserialize::<DescribeEmergencyContactSettingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEmergencyContactSettingsError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists the details of a <a>Protection</a> object.</p>
    async fn describe_protection(
        &self,
        input: DescribeProtectionRequest,
    ) -> Result<DescribeProtectionResponse, RusotoError<DescribeProtectionError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeProtection");
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
                .deserialize::<DescribeProtectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProtectionError::from_response(response))
        }
    }

    /// <p>Provides details about the AWS Shield Advanced subscription for an account.</p>
    async fn describe_subscription(
        &self,
    ) -> Result<DescribeSubscriptionResponse, RusotoError<DescribeSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeSubscription");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSubscriptionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSubscriptionError::from_response(response))
        }
    }

    /// <p>Removes the DDoS Response team's (DRT) access to the specified Amazon S3 bucket containing your AWS WAF logs.</p> <p>To make a <code>DisassociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTLogBucket</code> request to remove this access.</p>
    async fn disassociate_drt_log_bucket(
        &self,
        input: DisassociateDRTLogBucketRequest,
    ) -> Result<DisassociateDRTLogBucketResponse, RusotoError<DisassociateDRTLogBucketError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.DisassociateDRTLogBucket",
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
                .deserialize::<DisassociateDRTLogBucketResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateDRTLogBucketError::from_response(response))
        }
    }

    /// <p>Removes the DDoS Response team's (DRT) access to your AWS account.</p> <p>To make a <code>DisassociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTRole</code> request to remove this access.</p>
    async fn disassociate_drt_role(
        &self,
    ) -> Result<DisassociateDRTRoleResponse, RusotoError<DisassociateDRTRoleError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DisassociateDRTRole");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateDRTRoleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateDRTRoleError::from_response(response))
        }
    }

    /// <p>Returns the <code>SubscriptionState</code>, either <code>Active</code> or <code>Inactive</code>.</p>
    async fn get_subscription_state(
        &self,
    ) -> Result<GetSubscriptionStateResponse, RusotoError<GetSubscriptionStateError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.GetSubscriptionState");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSubscriptionStateResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetSubscriptionStateError::from_response(response))
        }
    }

    /// <p>Returns all ongoing DDoS attacks or all DDoS attacks during a specified time period.</p>
    async fn list_attacks(
        &self,
        input: ListAttacksRequest,
    ) -> Result<ListAttacksResponse, RusotoError<ListAttacksError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.ListAttacks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListAttacksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAttacksError::from_response(response))
        }
    }

    /// <p>Lists all <a>Protection</a> objects for the account.</p>
    async fn list_protections(
        &self,
        input: ListProtectionsRequest,
    ) -> Result<ListProtectionsResponse, RusotoError<ListProtectionsError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.ListProtections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListProtectionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListProtectionsError::from_response(response))
        }
    }

    /// <p>Updates the details of the list of email addresses that the DRT can use to contact you during a suspected attack.</p>
    async fn update_emergency_contact_settings(
        &self,
        input: UpdateEmergencyContactSettingsRequest,
    ) -> Result<
        UpdateEmergencyContactSettingsResponse,
        RusotoError<UpdateEmergencyContactSettingsError>,
    > {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.UpdateEmergencyContactSettings",
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
                .deserialize::<UpdateEmergencyContactSettingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEmergencyContactSettingsError::from_response(response))
        }
    }

    /// <p>Updates the details of an existing subscription. Only enter values for parameters you want to change. Empty parameters are not updated.</p>
    async fn update_subscription(
        &self,
        input: UpdateSubscriptionRequest,
    ) -> Result<UpdateSubscriptionResponse, RusotoError<UpdateSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.UpdateSubscription");
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
                .deserialize::<UpdateSubscriptionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSubscriptionError::from_response(response))
        }
    }
}
