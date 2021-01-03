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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl ShieldClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "shield", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateDRTLogBucketRequest {
    /// <p>The Amazon S3 bucket that contains your AWS WAF logs.</p>
    #[serde(rename = "LogBucket")]
    pub log_bucket: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDRTLogBucketResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateDRTRoleRequest {
    /// <p>The Amazon Resource Name (ARN) of the role the DRT will use to access your AWS account.</p> <p>Prior to making the <code>AssociateDRTRole</code> request, you must attach the <a href="https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy">AWSShieldDRTAccessPolicy</a> managed policy to this role. For more information see <a href=" https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html">Attaching and Detaching IAM Policies</a>.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDRTRoleResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateHealthCheckRequest {
    /// <p>The Amazon Resource Name (ARN) of the health check to associate with the protection.</p>
    #[serde(rename = "HealthCheckArn")]
    pub health_check_arn: String,
    /// <p>The unique identifier (ID) for the <a>Protection</a> object to add the health check association to. </p>
    #[serde(rename = "ProtectionId")]
    pub protection_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateHealthCheckResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateProactiveEngagementDetailsRequest {
    /// <p><p>A list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you for escalations to the DRT and to initiate proactive customer support. </p> <p>To enable proactive engagement, the contact list must include at least one phone number.</p> <note> <p>The contacts that you provide here replace any contacts that were already defined. If you already have contacts defined and want to use them, retrieve the list using <code>DescribeEmergencyContactSettings</code> and then provide it here. </p> </note></p>
    #[serde(rename = "EmergencyContactList")]
    pub emergency_contact_list: Vec<EmergencyContact>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateProactiveEngagementDetailsResponse {}

/// <p>The details of a DDoS attack.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The array of contributor objects that includes the top five contributors to an attack. </p>
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

/// <p>A single attack statistics data record. This is returned by <a>DescribeAttackStatistics</a> along with a time range indicating the time period that the attack statistics apply to. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackStatisticsDataItem {
    /// <p>The number of attacks detected during the time period. This is always present, but might be zero. </p>
    #[serde(rename = "AttackCount")]
    pub attack_count: i64,
    /// <p>Information about the volume of attacks during the time period. If the accompanying <code>AttackCount</code> is zero, this setting might be empty.</p>
    #[serde(rename = "AttackVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_volume: Option<AttackVolume>,
}

/// <p>Summarizes all DDoS attacks for a specified time period.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackVectorDescription {
    /// <p><p>The attack type. Valid values:</p> <ul> <li> <p>UDP<em>TRAFFIC</p> </li> <li> <p>UDP</em>FRAGMENT</p> </li> <li> <p>GENERIC<em>UDP</em>REFLECTION</p> </li> <li> <p>DNS<em>REFLECTION</p> </li> <li> <p>NTP</em>REFLECTION</p> </li> <li> <p>CHARGEN<em>REFLECTION</p> </li> <li> <p>SSDP</em>REFLECTION</p> </li> <li> <p>PORT<em>MAPPER</p> </li> <li> <p>RIP</em>REFLECTION</p> </li> <li> <p>SNMP<em>REFLECTION</p> </li> <li> <p>MSSQL</em>REFLECTION</p> </li> <li> <p>NET<em>BIOS</em>REFLECTION</p> </li> <li> <p>SYN<em>FLOOD</p> </li> <li> <p>ACK</em>FLOOD</p> </li> <li> <p>REQUEST<em>FLOOD</p> </li> <li> <p>HTTP</em>REFLECTION</p> </li> <li> <p>UDS<em>REFLECTION</p> </li> <li> <p>MEMCACHED</em>REFLECTION</p> </li> </ul></p>
    #[serde(rename = "VectorType")]
    pub vector_type: String,
}

/// <p>Information about the volume of attacks during the time period, included in an <a>AttackStatisticsDataItem</a>. If the accompanying <code>AttackCount</code> in the statistics object is zero, this setting might be empty.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackVolume {
    /// <p>A statistics object that uses bits per second as the unit. This is included for network level attacks. </p>
    #[serde(rename = "BitsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits_per_second: Option<AttackVolumeStatistics>,
    /// <p>A statistics object that uses packets per second as the unit. This is included for network level attacks. </p>
    #[serde(rename = "PacketsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_per_second: Option<AttackVolumeStatistics>,
    /// <p>A statistics object that uses requests per second as the unit. This is included for application level attacks, and is only available for accounts that are subscribed to Shield Advanced.</p>
    #[serde(rename = "RequestsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_second: Option<AttackVolumeStatistics>,
}

/// <p>Statistics objects for the various data types in <a>AttackVolume</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttackVolumeStatistics {
    /// <p>The maximum attack volume observed for the given unit.</p>
    #[serde(rename = "Max")]
    pub max: f64,
}

/// <p>A contributor to the attack and their contribution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProtectionGroupRequest {
    /// <p><p>Defines how AWS Shield combines resource data for the group in order to detect, mitigate, and report events.</p> <ul> <li> <p>Sum - Use the total traffic across the group. This is a good choice for most cases. Examples include Elastic IP addresses for EC2 instances that scale manually or automatically.</p> </li> <li> <p>Mean - Use the average of the traffic across the group. This is a good choice for resources that share traffic uniformly. Examples include accelerators and load balancers.</p> </li> <li> <p>Max - Use the highest traffic from each resource. This is useful for resources that don&#39;t share traffic and for resources that share that traffic in a non-uniform way. Examples include CloudFront distributions and origin resources for CloudFront distributions.</p> </li> </ul></p>
    #[serde(rename = "Aggregation")]
    pub aggregation: String,
    /// <p>The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set <code>Pattern</code> to <code>ARBITRARY</code> and you must not set it for any other <code>Pattern</code> setting. </p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    /// <p>The criteria to use to choose the protected resources for inclusion in the group. You can include all resources that have protections, provide a list of resource Amazon Resource Names (ARNs), or include all resources of a specified resource type. </p>
    #[serde(rename = "Pattern")]
    pub pattern: String,
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: String,
    /// <p>The resource type to include in the protection group. All protected resources of this type are included in the protection group. Newly protected resources of this type are automatically added to the group. You must set this when you set <code>Pattern</code> to <code>BY_RESOURCE_TYPE</code> and you must not set it for any other <code>Pattern</code> setting. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProtectionGroupResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProtectionRequest {
    /// <p>Friendly name for the <code>Protection</code> you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The ARN (Amazon Resource Name) of the resource to be protected.</p> <p>The ARN should be in one of the following formats:</p> <ul> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Elastic Load Balancer (Classic Load Balancer): <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/<i>load-balancer-name</i> </code> </p> </li> <li> <p>For an AWS CloudFront distribution: <code>arn:aws:cloudfront::<i>account-id</i>:distribution/<i>distribution-id</i> </code> </p> </li> <li> <p>For an AWS Global Accelerator accelerator: <code>arn:aws:globalaccelerator::<i>account-id</i>:accelerator/<i>accelerator-id</i> </code> </p> </li> <li> <p>For Amazon Route 53: <code>arn:aws:route53:::hostedzone/<i>hosted-zone-id</i> </code> </p> </li> <li> <p>For an Elastic IP address: <code>arn:aws:ec2:<i>region</i>:<i>account-id</i>:eip-allocation/<i>allocation-id</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProtectionResponse {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object that is created.</p>
    #[serde(rename = "ProtectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSubscriptionRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSubscriptionResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProtectionGroupRequest {
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProtectionGroupResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProtectionRequest {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object to be deleted.</p>
    #[serde(rename = "ProtectionId")]
    pub protection_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProtectionResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSubscriptionRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSubscriptionResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAttackRequest {
    /// <p>The unique identifier (ID) for the attack that to be described.</p>
    #[serde(rename = "AttackId")]
    pub attack_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAttackResponse {
    /// <p>The attack that is described.</p>
    #[serde(rename = "Attack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<AttackDetail>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAttackStatisticsRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAttackStatisticsResponse {
    /// <p>The data that describes the attacks detected during the time period.</p>
    #[serde(rename = "DataItems")]
    pub data_items: Vec<AttackStatisticsDataItem>,
    #[serde(rename = "TimeRange")]
    pub time_range: TimeRange,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDRTAccessRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEmergencyContactSettingsRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEmergencyContactSettingsResponse {
    /// <p>A list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you if you have proactive engagement enabled, for escalations to the DRT and to initiate proactive customer support.</p>
    #[serde(rename = "EmergencyContactList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProtectionGroupRequest {
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProtectionGroupResponse {
    /// <p>A grouping of protected resources that you and AWS Shield Advanced can monitor as a collective. This resource grouping improves the accuracy of detection and reduces false positives. </p>
    #[serde(rename = "ProtectionGroup")]
    pub protection_group: ProtectionGroup,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProtectionResponse {
    /// <p>The <a>Protection</a> object that is described.</p>
    #[serde(rename = "Protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<Protection>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSubscriptionRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSubscriptionResponse {
    /// <p>The AWS Shield Advanced subscription details for an account.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableProactiveEngagementRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableProactiveEngagementResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateDRTLogBucketRequest {
    /// <p>The Amazon S3 bucket that contains your AWS WAF logs.</p>
    #[serde(rename = "LogBucket")]
    pub log_bucket: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDRTLogBucketResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateDRTRoleRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDRTRoleResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateHealthCheckRequest {
    /// <p>The Amazon Resource Name (ARN) of the health check that is associated with the protection.</p>
    #[serde(rename = "HealthCheckArn")]
    pub health_check_arn: String,
    /// <p>The unique identifier (ID) for the <a>Protection</a> object to remove the health check association from. </p>
    #[serde(rename = "ProtectionId")]
    pub protection_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateHealthCheckResponse {}

/// <p>Contact information that the DRT can use to contact you if you have proactive engagement enabled, for escalations to the DRT and to initiate proactive customer support.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmergencyContact {
    /// <p>Additional notes regarding the contact. </p>
    #[serde(rename = "ContactNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_notes: Option<String>,
    /// <p>The email address for the contact.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// <p>The phone number for the contact.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableProactiveEngagementRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableProactiveEngagementResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSubscriptionStateRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSubscriptionStateResponse {
    /// <p>The status of the subscription.</p>
    #[serde(rename = "SubscriptionState")]
    pub subscription_state: String,
}

/// <p>Specifies how many protections of a given type you can create.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAttacksRequest {
    /// <p>The end of the time period for the attacks. This is a <code>timestamp</code> type. The sample request above indicates a <code>number</code> type because the default used by WAF is Unix time in seconds. However any valid <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp format</a> is allowed. </p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeRange>,
    /// <p>The maximum number of <a>AttackSummary</a> objects to return. If you leave this blank, Shield Advanced returns the first 20 results.</p> <p>This is a maximum value. Shield Advanced might return the results in smaller batches. That is, the number of objects returned could be less than <code>MaxResults</code>, even if there are still more objects yet to return. If there are more objects to return, Shield Advanced returns a value in <code>NextToken</code> that you can use in your next request, to get the next batch of objects.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAttacksResponse {
    /// <p>The attack information for the specified time range.</p>
    #[serde(rename = "AttackSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_summaries: Option<Vec<AttackSummary>>,
    /// <p>The token returned by a previous call to indicate that there is more data available. If not null, more results are available. Pass this value for the <code>NextMarker</code> parameter in a subsequent call to <code>ListAttacks</code> to retrieve the next set of items.</p> <p>Shield Advanced might return the list of <a>AttackSummary</a> objects in batches smaller than the number specified by MaxResults. If there are more attack summary objects to return, Shield Advanced will always also return a <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProtectionGroupsRequest {
    /// <p>The maximum number of <a>ProtectionGroup</a> objects to return. If you leave this blank, Shield Advanced returns the first 20 results.</p> <p>This is a maximum value. Shield Advanced might return the results in smaller batches. That is, the number of objects returned could be less than <code>MaxResults</code>, even if there are still more objects yet to return. If there are more objects to return, Shield Advanced returns a value in <code>NextToken</code> that you can use in your next request, to get the next batch of objects.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next token value from a previous call to <code>ListProtectionGroups</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProtectionGroupsResponse {
    /// <p>If you specify a value for <code>MaxResults</code> and you have more protection groups than the value of MaxResults, AWS Shield Advanced returns this token that you can use in your next request, to get the next batch of objects. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "ProtectionGroups")]
    pub protection_groups: Vec<ProtectionGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProtectionsRequest {
    /// <p>The maximum number of <a>Protection</a> objects to return. If you leave this blank, Shield Advanced returns the first 20 results.</p> <p>This is a maximum value. Shield Advanced might return the results in smaller batches. That is, the number of objects returned could be less than <code>MaxResults</code>, even if there are still more objects yet to return. If there are more objects to return, Shield Advanced returns a value in <code>NextToken</code> that you can use in your next request, to get the next batch of objects.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>ListProtectionsRequest.NextToken</code> value from a previous call to <code>ListProtections</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProtectionsResponse {
    /// <p>If you specify a value for <code>MaxResults</code> and you have more Protections than the value of MaxResults, AWS Shield Advanced returns a NextToken value in the response that allows you to list another group of Protections. For the second and subsequent ListProtections requests, specify the value of NextToken from the previous response to get information about another batch of Protections.</p> <p>Shield Advanced might return the list of <a>Protection</a> objects in batches smaller than the number specified by MaxResults. If there are more <a>Protection</a> objects to return, Shield Advanced will always also return a <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The array of enabled <a>Protection</a> objects.</p>
    #[serde(rename = "Protections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protections: Option<Vec<Protection>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourcesInProtectionGroupRequest {
    /// <p>The maximum number of resource ARN objects to return. If you leave this blank, Shield Advanced returns the first 20 results.</p> <p>This is a maximum value. Shield Advanced might return the results in smaller batches. That is, the number of objects returned could be less than <code>MaxResults</code>, even if there are still more objects yet to return. If there are more objects to return, Shield Advanced returns a value in <code>NextToken</code> that you can use in your next request, to get the next batch of objects.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next token value from a previous call to <code>ListResourcesInProtectionGroup</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourcesInProtectionGroupResponse {
    /// <p>If you specify a value for <code>MaxResults</code> and you have more resources in the protection group than the value of MaxResults, AWS Shield Advanced returns this token that you can use in your next request, to get the next batch of objects. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Names (ARNs) of the resources that are included in the protection group.</p>
    #[serde(rename = "ResourceArns")]
    pub resource_arns: Vec<String>,
}

/// <p>The mitigation applied to a DDoS attack.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Mitigation {
    /// <p>The name of the mitigation taken for this attack.</p>
    #[serde(rename = "MitigationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_name: Option<String>,
}

/// <p>An object that represents a resource that is under DDoS protection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Protection {
    /// <p>The unique identifier (ID) for the Route 53 health check that's associated with the protection. </p>
    #[serde(rename = "HealthCheckIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_ids: Option<Vec<String>>,
    /// <p>The unique identifier (ID) of the protection.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the protection. For example, <code>My CloudFront distributions</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN (Amazon Resource Name) of the AWS resource that is protected.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

/// <p>A grouping of protected resources that you and AWS Shield Advanced can monitor as a collective. This resource grouping improves the accuracy of detection and reduces false positives. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProtectionGroup {
    /// <p><p>Defines how AWS Shield combines resource data for the group in order to detect, mitigate, and report events.</p> <ul> <li> <p>Sum - Use the total traffic across the group. This is a good choice for most cases. Examples include Elastic IP addresses for EC2 instances that scale manually or automatically.</p> </li> <li> <p>Mean - Use the average of the traffic across the group. This is a good choice for resources that share traffic uniformly. Examples include accelerators and load balancers.</p> </li> <li> <p>Max - Use the highest traffic from each resource. This is useful for resources that don&#39;t share traffic and for resources that share that traffic in a non-uniform way. Examples include CloudFront distributions and origin resources for CloudFront distributions.</p> </li> </ul></p>
    #[serde(rename = "Aggregation")]
    pub aggregation: String,
    /// <p>The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set <code>Pattern</code> to <code>ARBITRARY</code> and you must not set it for any other <code>Pattern</code> setting. </p>
    #[serde(rename = "Members")]
    pub members: Vec<String>,
    /// <p>The criteria to use to choose the protected resources for inclusion in the group. You can include all resources that have protections, provide a list of resource Amazon Resource Names (ARNs), or include all resources of a specified resource type.</p>
    #[serde(rename = "Pattern")]
    pub pattern: String,
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: String,
    /// <p>The resource type to include in the protection group. All protected resources of this type are included in the protection group. You must set this when you set <code>Pattern</code> to <code>BY_RESOURCE_TYPE</code> and you must not set it for any other <code>Pattern</code> setting. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Limits settings on protection groups with arbitrary pattern type. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProtectionGroupArbitraryPatternLimits {
    /// <p>The maximum number of resources you can specify for a single arbitrary pattern in a protection group.</p>
    #[serde(rename = "MaxMembers")]
    pub max_members: i64,
}

/// <p>Limits settings on protection groups for your subscription. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProtectionGroupLimits {
    /// <p>The maximum number of protection groups that you can have at one time. </p>
    #[serde(rename = "MaxProtectionGroups")]
    pub max_protection_groups: i64,
    /// <p>Limits settings by pattern type in the protection groups for your subscription. </p>
    #[serde(rename = "PatternTypeLimits")]
    pub pattern_type_limits: ProtectionGroupPatternTypeLimits,
}

/// <p>Limits settings by pattern type in the protection groups for your subscription. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProtectionGroupPatternTypeLimits {
    /// <p>Limits settings on protection groups with arbitrary pattern type. </p>
    #[serde(rename = "ArbitraryPatternLimits")]
    pub arbitrary_pattern_limits: ProtectionGroupArbitraryPatternLimits,
}

/// <p>Limits settings on protections for your subscription. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProtectionLimits {
    /// <p>The maximum number of resource types that you can specify in a protection.</p>
    #[serde(rename = "ProtectedResourceTypeLimits")]
    pub protected_resource_type_limits: Vec<Limit>,
}

/// <p>The attack information for the specified SubResource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>If <code>ENABLED</code>, the DDoS Response Team (DRT) will use email and phone to notify contacts about escalations to the DRT and to initiate proactive customer support.</p> <p>If <code>PENDING</code>, you have requested proactive engagement and the request is pending. The status changes to <code>ENABLED</code> when your request is fully processed.</p> <p>If <code>DISABLED</code>, the DRT will not proactively notify contacts about escalations or to initiate proactive customer support. </p>
    #[serde(rename = "ProactiveEngagementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proactive_engagement_status: Option<String>,
    /// <p>The start time of the subscription, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Limits settings for your subscription. </p>
    #[serde(rename = "SubscriptionLimits")]
    pub subscription_limits: SubscriptionLimits,
    /// <p>The length, in seconds, of the AWS Shield Advanced subscription for the account.</p>
    #[serde(rename = "TimeCommitmentInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_commitment_in_seconds: Option<i64>,
}

/// <p>Limits settings for your subscription. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubscriptionLimits {
    /// <p>Limits settings on protection groups for your subscription. </p>
    #[serde(rename = "ProtectionGroupLimits")]
    pub protection_group_limits: ProtectionGroupLimits,
    /// <p>Limits settings on protections for your subscription. </p>
    #[serde(rename = "ProtectionLimits")]
    pub protection_limits: ProtectionLimits,
}

/// <p>A summary of information about the attack.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>The time range. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEmergencyContactSettingsRequest {
    /// <p>A list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you if you have proactive engagement enabled, for escalations to the DRT and to initiate proactive customer support.</p> <p>If you have proactive engagement enabled, the contact list must include at least one phone number.</p>
    #[serde(rename = "EmergencyContactList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEmergencyContactSettingsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProtectionGroupRequest {
    /// <p><p>Defines how AWS Shield combines resource data for the group in order to detect, mitigate, and report events.</p> <ul> <li> <p>Sum - Use the total traffic across the group. This is a good choice for most cases. Examples include Elastic IP addresses for EC2 instances that scale manually or automatically.</p> </li> <li> <p>Mean - Use the average of the traffic across the group. This is a good choice for resources that share traffic uniformly. Examples include accelerators and load balancers.</p> </li> <li> <p>Max - Use the highest traffic from each resource. This is useful for resources that don&#39;t share traffic and for resources that share that traffic in a non-uniform way. Examples include CloudFront distributions and origin resources for CloudFront distributions.</p> </li> </ul></p>
    #[serde(rename = "Aggregation")]
    pub aggregation: String,
    /// <p>The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set <code>Pattern</code> to <code>ARBITRARY</code> and you must not set it for any other <code>Pattern</code> setting. </p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    /// <p>The criteria to use to choose the protected resources for inclusion in the group. You can include all resources that have protections, provide a list of resource Amazon Resource Names (ARNs), or include all resources of a specified resource type.</p>
    #[serde(rename = "Pattern")]
    pub pattern: String,
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    #[serde(rename = "ProtectionGroupId")]
    pub protection_group_id: String,
    /// <p>The resource type to include in the protection group. All protected resources of this type are included in the protection group. You must set this when you set <code>Pattern</code> to <code>BY_RESOURCE_TYPE</code> and you must not set it for any other <code>Pattern</code> setting. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProtectionGroupResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSubscriptionRequest {
    /// <p>When you initally create a subscription, <code>AutoRenew</code> is set to <code>ENABLED</code>. If <code>ENABLED</code>, the subscription will be automatically renewed at the end of the existing subscription period. You can change this by submitting an <code>UpdateSubscription</code> request. If the <code>UpdateSubscription</code> request does not included a value for <code>AutoRenew</code>, the existing value for <code>AutoRenew</code> remains unchanged.</p>
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSubscriptionResponse {}

/// <p>Provides information about a particular parameter passed inside a request that resulted in an exception.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidationExceptionField {
    /// <p>The message describing why the parameter failed validation.</p>
    pub message: String,
    /// <p>The name of the parameter that failed validation.</p>
    pub name: String,
}

/// Errors returned by AssociateDRTLogBucket
#[derive(Debug, PartialEq)]
pub enum AssociateDRTLogBucketError {
    /// <p>In order to grant the necessary access to the DDoS Response Team (DRT), the user submitting the request must have the <code>iam:PassRole</code> permission. This error indicates the user did not have the appropriate permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p>
    AccessDeniedForDependency(String),
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the operation would exceed a limit.</p> <p> <code>Type</code> is the type of limit that would be exceeded.</p> <p> <code>Limit</code> is the threshold that would be exceeded.</p>
    LimitsExceeded(String),
    /// <p>The ARN of the role that you specifed does not exist.</p>
    NoAssociatedRole(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
    /// <p>In order to grant the necessary access to the DDoS Response Team (DRT), the user submitting the request must have the <code>iam:PassRole</code> permission. This error indicates the user did not have the appropriate permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p>
    AccessDeniedForDependency(String),
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by AssociateHealthCheck
#[derive(Debug, PartialEq)]
pub enum AssociateHealthCheckError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the operation would exceed a limit.</p> <p> <code>Type</code> is the type of limit that would be exceeded.</p> <p> <code>Limit</code> is the threshold that would be exceeded.</p>
    LimitsExceeded(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl AssociateHealthCheckError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateHealthCheckError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AssociateHealthCheckError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateHealthCheckError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitsExceededException" => {
                    return RusotoError::Service(AssociateHealthCheckError::LimitsExceeded(err.msg))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(AssociateHealthCheckError::OptimisticLock(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateHealthCheckError::ResourceNotFound(
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
impl fmt::Display for AssociateHealthCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateHealthCheckError::InternalError(ref cause) => write!(f, "{}", cause),
            AssociateHealthCheckError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateHealthCheckError::LimitsExceeded(ref cause) => write!(f, "{}", cause),
            AssociateHealthCheckError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            AssociateHealthCheckError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateHealthCheckError {}
/// Errors returned by AssociateProactiveEngagementDetails
#[derive(Debug, PartialEq)]
pub enum AssociateProactiveEngagementDetailsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl AssociateProactiveEngagementDetailsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateProactiveEngagementDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(
                        AssociateProactiveEngagementDetailsError::InternalError(err.msg),
                    )
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(
                        AssociateProactiveEngagementDetailsError::InvalidOperation(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateProactiveEngagementDetailsError::InvalidParameter(err.msg),
                    )
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(
                        AssociateProactiveEngagementDetailsError::OptimisticLock(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateProactiveEngagementDetailsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateProactiveEngagementDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateProactiveEngagementDetailsError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateProactiveEngagementDetailsError::InvalidOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateProactiveEngagementDetailsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateProactiveEngagementDetailsError::OptimisticLock(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateProactiveEngagementDetailsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateProactiveEngagementDetailsError {}
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
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource already exists. If available, this exception includes details in additional properties. </p>
    ResourceAlreadyExists(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by CreateProtectionGroup
#[derive(Debug, PartialEq)]
pub enum CreateProtectionGroupError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the operation would exceed a limit.</p> <p> <code>Type</code> is the type of limit that would be exceeded.</p> <p> <code>Limit</code> is the threshold that would be exceeded.</p>
    LimitsExceeded(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource already exists. If available, this exception includes details in additional properties. </p>
    ResourceAlreadyExists(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl CreateProtectionGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProtectionGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateProtectionGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateProtectionGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitsExceededException" => {
                    return RusotoError::Service(CreateProtectionGroupError::LimitsExceeded(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(CreateProtectionGroupError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateProtectionGroupError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateProtectionGroupError::ResourceNotFound(
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
impl fmt::Display for CreateProtectionGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProtectionGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateProtectionGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateProtectionGroupError::LimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateProtectionGroupError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            CreateProtectionGroupError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateProtectionGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProtectionGroupError {}
/// Errors returned by CreateSubscription
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource already exists. If available, this exception includes details in additional properties. </p>
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
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by DeleteProtectionGroup
#[derive(Debug, PartialEq)]
pub enum DeleteProtectionGroupError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl DeleteProtectionGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProtectionGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteProtectionGroupError::InternalError(err.msg))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(DeleteProtectionGroupError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProtectionGroupError::ResourceNotFound(
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
impl fmt::Display for DeleteProtectionGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProtectionGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteProtectionGroupError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            DeleteProtectionGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProtectionGroupError {}
/// Errors returned by DeleteSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>You are trying to update a subscription that has not yet completed the 1-year commitment. You can change the <code>AutoRenew</code> parameter during the last 30 days of your subscription. This exception indicates that you are attempting to change <code>AutoRenew</code> prior to that period.</p>
    LockedSubscription(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by DescribeAttackStatistics
#[derive(Debug, PartialEq)]
pub enum DescribeAttackStatisticsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
}

impl DescribeAttackStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAttackStatisticsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeAttackStatisticsError::InternalError(
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
impl fmt::Display for DescribeAttackStatisticsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAttackStatisticsError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAttackStatisticsError {}
/// Errors returned by DescribeDRTAccess
#[derive(Debug, PartialEq)]
pub enum DescribeDRTAccessError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by DescribeProtectionGroup
#[derive(Debug, PartialEq)]
pub enum DescribeProtectionGroupError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl DescribeProtectionGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProtectionGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeProtectionGroupError::InternalError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProtectionGroupError::ResourceNotFound(
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
impl fmt::Display for DescribeProtectionGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProtectionGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeProtectionGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProtectionGroupError {}
/// Errors returned by DescribeSubscription
#[derive(Debug, PartialEq)]
pub enum DescribeSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by DisableProactiveEngagement
#[derive(Debug, PartialEq)]
pub enum DisableProactiveEngagementError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl DisableProactiveEngagementError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableProactiveEngagementError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DisableProactiveEngagementError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DisableProactiveEngagementError::InvalidOperation(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisableProactiveEngagementError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(DisableProactiveEngagementError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableProactiveEngagementError::ResourceNotFound(
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
impl fmt::Display for DisableProactiveEngagementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableProactiveEngagementError::InternalError(ref cause) => write!(f, "{}", cause),
            DisableProactiveEngagementError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DisableProactiveEngagementError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisableProactiveEngagementError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            DisableProactiveEngagementError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableProactiveEngagementError {}
/// Errors returned by DisassociateDRTLogBucket
#[derive(Debug, PartialEq)]
pub enum DisassociateDRTLogBucketError {
    /// <p>In order to grant the necessary access to the DDoS Response Team (DRT), the user submitting the request must have the <code>iam:PassRole</code> permission. This error indicates the user did not have the appropriate permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p>
    AccessDeniedForDependency(String),
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>The ARN of the role that you specifed does not exist.</p>
    NoAssociatedRole(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by DisassociateHealthCheck
#[derive(Debug, PartialEq)]
pub enum DisassociateHealthCheckError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl DisassociateHealthCheckError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateHealthCheckError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DisassociateHealthCheckError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateHealthCheckError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(DisassociateHealthCheckError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateHealthCheckError::ResourceNotFound(
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
impl fmt::Display for DisassociateHealthCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateHealthCheckError::InternalError(ref cause) => write!(f, "{}", cause),
            DisassociateHealthCheckError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateHealthCheckError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            DisassociateHealthCheckError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateHealthCheckError {}
/// Errors returned by EnableProactiveEngagement
#[derive(Debug, PartialEq)]
pub enum EnableProactiveEngagementError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl EnableProactiveEngagementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableProactiveEngagementError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(EnableProactiveEngagementError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(EnableProactiveEngagementError::InvalidOperation(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(EnableProactiveEngagementError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(EnableProactiveEngagementError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(EnableProactiveEngagementError::ResourceNotFound(
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
impl fmt::Display for EnableProactiveEngagementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableProactiveEngagementError::InternalError(ref cause) => write!(f, "{}", cause),
            EnableProactiveEngagementError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            EnableProactiveEngagementError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            EnableProactiveEngagementError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            EnableProactiveEngagementError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableProactiveEngagementError {}
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
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by ListProtectionGroups
#[derive(Debug, PartialEq)]
pub enum ListProtectionGroupsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the NextToken specified in the request is invalid. Submit the request using the NextToken value that was returned in the response.</p>
    InvalidPaginationToken(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl ListProtectionGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProtectionGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListProtectionGroupsError::InternalError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListProtectionGroupsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProtectionGroupsError::ResourceNotFound(
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
impl fmt::Display for ListProtectionGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProtectionGroupsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListProtectionGroupsError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListProtectionGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProtectionGroupsError {}
/// Errors returned by ListProtections
#[derive(Debug, PartialEq)]
pub enum ListProtectionsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the NextToken specified in the request is invalid. Submit the request using the NextToken value that was returned in the response.</p>
    InvalidPaginationToken(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by ListResourcesInProtectionGroup
#[derive(Debug, PartialEq)]
pub enum ListResourcesInProtectionGroupError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the NextToken specified in the request is invalid. Submit the request using the NextToken value that was returned in the response.</p>
    InvalidPaginationToken(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl ListResourcesInProtectionGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListResourcesInProtectionGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(
                        ListResourcesInProtectionGroupError::InternalError(err.msg),
                    )
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListResourcesInProtectionGroupError::InvalidPaginationToken(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListResourcesInProtectionGroupError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourcesInProtectionGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourcesInProtectionGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            ListResourcesInProtectionGroupError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListResourcesInProtectionGroupError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListResourcesInProtectionGroupError {}
/// Errors returned by UpdateEmergencyContactSettings
#[derive(Debug, PartialEq)]
pub enum UpdateEmergencyContactSettingsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
/// Errors returned by UpdateProtectionGroup
#[derive(Debug, PartialEq)]
pub enum UpdateProtectionGroupError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
    ResourceNotFound(String),
}

impl UpdateProtectionGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProtectionGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateProtectionGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateProtectionGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OptimisticLockException" => {
                    return RusotoError::Service(UpdateProtectionGroupError::OptimisticLock(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProtectionGroupError::ResourceNotFound(
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
impl fmt::Display for UpdateProtectionGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProtectionGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateProtectionGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateProtectionGroupError::OptimisticLock(ref cause) => write!(f, "{}", cause),
            UpdateProtectionGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProtectionGroupError {}
/// Errors returned by UpdateSubscription
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. If available, this exception includes details in additional properties. </p>
    InvalidParameter(String),
    /// <p>You are trying to update a subscription that has not yet completed the 1-year commitment. You can change the <code>AutoRenew</code> parameter during the last 30 days of your subscription. This exception indicates that you are attempting to change <code>AutoRenew</code> prior to that period.</p>
    LockedSubscription(String),
    /// <p>Exception that indicates that the resource state has been modified by another client. Retrieve the resource and then retry your request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist. If available, this exception includes details in additional properties. </p>
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
    /// <p>Authorizes the DDoS Response Team (DRT) to access the specified Amazon S3 bucket containing your AWS WAF logs. You can associate up to 10 Amazon S3 buckets with your subscription.</p> <p>To use the services of the DRT and make an <code>AssociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_log_bucket(
        &self,
        input: AssociateDRTLogBucketRequest,
    ) -> Result<AssociateDRTLogBucketResponse, RusotoError<AssociateDRTLogBucketError>>;

    /// <p>Authorizes the DDoS Response Team (DRT), using the specified role, to access your AWS account to assist with DDoS attack mitigation during potential attacks. This enables the DRT to inspect your AWS WAF configuration and create or update AWS WAF rules and web ACLs.</p> <p>You can associate only one <code>RoleArn</code> with your subscription. If you submit an <code>AssociateDRTRole</code> request for an account that already has an associated role, the new <code>RoleArn</code> will replace the existing <code>RoleArn</code>. </p> <p>Prior to making the <code>AssociateDRTRole</code> request, you must attach the <a href="https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy">AWSShieldDRTAccessPolicy</a> managed policy to the role you will specify in the request. For more information see <a href=" https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html">Attaching and Detaching IAM Policies</a>. The role must also trust the service principal <code> drt.shield.amazonaws.com</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html">IAM JSON Policy Elements: Principal</a>.</p> <p>The DRT will have access only to your AWS WAF and Shield resources. By submitting this request, you authorize the DRT to inspect your AWS WAF and Shield configuration and create and update AWS WAF rules and web ACLs on your behalf. The DRT takes these actions only if explicitly authorized by you.</p> <p>You must have the <code>iam:PassRole</code> permission to make an <code>AssociateDRTRole</code> request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p> <p>To use the services of the DRT and make an <code>AssociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_role(
        &self,
        input: AssociateDRTRoleRequest,
    ) -> Result<AssociateDRTRoleResponse, RusotoError<AssociateDRTRoleError>>;

    /// <p>Adds health-based detection to the Shield Advanced protection for a resource. Shield Advanced health-based detection uses the health of your AWS resource to improve responsiveness and accuracy in attack detection and mitigation. </p> <p>You define the health check in Route 53 and then associate it with your Shield Advanced protection. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/ddos-overview.html#ddos-advanced-health-check-option">Shield Advanced Health-Based Detection</a> in the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF and AWS Shield Developer Guide</a>. </p>
    async fn associate_health_check(
        &self,
        input: AssociateHealthCheckRequest,
    ) -> Result<AssociateHealthCheckResponse, RusotoError<AssociateHealthCheckError>>;

    /// <p><p>Initializes proactive engagement and sets the list of contacts for the DDoS Response Team (DRT) to use. You must provide at least one phone number in the emergency contact list. </p> <p>After you have initialized proactive engagement using this call, to disable or enable proactive engagement, use the calls <code>DisableProactiveEngagement</code> and <code>EnableProactiveEngagement</code>. </p> <note> <p>This call defines the list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you for escalations to the DRT and to initiate proactive customer support.</p> <p>The contacts that you provide in the request replace any contacts that were already defined. If you already have contacts defined and want to use them, retrieve the list using <code>DescribeEmergencyContactSettings</code> and then provide it to this call. </p> </note></p>
    async fn associate_proactive_engagement_details(
        &self,
        input: AssociateProactiveEngagementDetailsRequest,
    ) -> Result<
        AssociateProactiveEngagementDetailsResponse,
        RusotoError<AssociateProactiveEngagementDetailsError>,
    >;

    /// <p>Enables AWS Shield Advanced for a specific AWS resource. The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, AWS Global Accelerator accelerator, Elastic IP Address, or an Amazon Route 53 hosted zone.</p> <p>You can add protection to only a single resource with each CreateProtection request. If you want to add protection to multiple resources at once, use the <a href="https://console.aws.amazon.com/waf/">AWS WAF console</a>. For more information see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/getting-started-ddos.html">Getting Started with AWS Shield Advanced</a> and <a href="https://docs.aws.amazon.com/waf/latest/developerguide/configure-new-protection.html">Add AWS Shield Advanced Protection to more AWS Resources</a>.</p>
    async fn create_protection(
        &self,
        input: CreateProtectionRequest,
    ) -> Result<CreateProtectionResponse, RusotoError<CreateProtectionError>>;

    /// <p>Creates a grouping of protected resources so they can be handled as a collective. This resource grouping improves the accuracy of detection and reduces false positives. </p>
    async fn create_protection_group(
        &self,
        input: CreateProtectionGroupRequest,
    ) -> Result<CreateProtectionGroupResponse, RusotoError<CreateProtectionGroupError>>;

    /// <p>Activates AWS Shield Advanced for an account.</p> <p>When you initally create a subscription, your subscription is set to be automatically renewed at the end of the existing subscription period. You can change this by submitting an <code>UpdateSubscription</code> request. </p>
    async fn create_subscription(
        &self,
    ) -> Result<CreateSubscriptionResponse, RusotoError<CreateSubscriptionError>>;

    /// <p>Deletes an AWS Shield Advanced <a>Protection</a>.</p>
    async fn delete_protection(
        &self,
        input: DeleteProtectionRequest,
    ) -> Result<DeleteProtectionResponse, RusotoError<DeleteProtectionError>>;

    /// <p>Removes the specified protection group.</p>
    async fn delete_protection_group(
        &self,
        input: DeleteProtectionGroupRequest,
    ) -> Result<DeleteProtectionGroupResponse, RusotoError<DeleteProtectionGroupError>>;

    /// <p>Removes AWS Shield Advanced from an account. AWS Shield Advanced requires a 1-year subscription commitment. You cannot delete a subscription prior to the completion of that commitment. </p>
    async fn delete_subscription(
        &self,
    ) -> Result<DeleteSubscriptionResponse, RusotoError<DeleteSubscriptionError>>;

    /// <p>Describes the details of a DDoS attack. </p>
    async fn describe_attack(
        &self,
        input: DescribeAttackRequest,
    ) -> Result<DescribeAttackResponse, RusotoError<DescribeAttackError>>;

    /// <p>Provides information about the number and type of attacks AWS Shield has detected in the last year for all resources that belong to your account, regardless of whether you've defined Shield protections for them. This operation is available to Shield customers as well as to Shield Advanced customers.</p> <p>The operation returns data for the time range of midnight UTC, one year ago, to midnight UTC, today. For example, if the current time is <code>2020-10-26 15:39:32 PDT</code>, equal to <code>2020-10-26 22:39:32 UTC</code>, then the time range for the attack data returned is from <code>2019-10-26 00:00:00 UTC</code> to <code>2020-10-26 00:00:00 UTC</code>. </p> <p>The time range indicates the period covered by the attack statistics data items.</p>
    async fn describe_attack_statistics(
        &self,
    ) -> Result<DescribeAttackStatisticsResponse, RusotoError<DescribeAttackStatisticsError>>;

    /// <p>Returns the current role and list of Amazon S3 log buckets used by the DDoS Response Team (DRT) to access your AWS account while assisting with attack mitigation.</p>
    async fn describe_drt_access(
        &self,
    ) -> Result<DescribeDRTAccessResponse, RusotoError<DescribeDRTAccessError>>;

    /// <p>A list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you if you have proactive engagement enabled, for escalations to the DRT and to initiate proactive customer support.</p>
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

    /// <p>Returns the specification for the specified protection group.</p>
    async fn describe_protection_group(
        &self,
        input: DescribeProtectionGroupRequest,
    ) -> Result<DescribeProtectionGroupResponse, RusotoError<DescribeProtectionGroupError>>;

    /// <p>Provides details about the AWS Shield Advanced subscription for an account.</p>
    async fn describe_subscription(
        &self,
    ) -> Result<DescribeSubscriptionResponse, RusotoError<DescribeSubscriptionError>>;

    /// <p>Removes authorization from the DDoS Response Team (DRT) to notify contacts about escalations to the DRT and to initiate proactive customer support.</p>
    async fn disable_proactive_engagement(
        &self,
    ) -> Result<DisableProactiveEngagementResponse, RusotoError<DisableProactiveEngagementError>>;

    /// <p>Removes the DDoS Response Team's (DRT) access to the specified Amazon S3 bucket containing your AWS WAF logs.</p> <p>To make a <code>DisassociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTLogBucket</code> request to remove this access.</p>
    async fn disassociate_drt_log_bucket(
        &self,
        input: DisassociateDRTLogBucketRequest,
    ) -> Result<DisassociateDRTLogBucketResponse, RusotoError<DisassociateDRTLogBucketError>>;

    /// <p>Removes the DDoS Response Team's (DRT) access to your AWS account.</p> <p>To make a <code>DisassociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTRole</code> request to remove this access.</p>
    async fn disassociate_drt_role(
        &self,
    ) -> Result<DisassociateDRTRoleResponse, RusotoError<DisassociateDRTRoleError>>;

    /// <p>Removes health-based detection from the Shield Advanced protection for a resource. Shield Advanced health-based detection uses the health of your AWS resource to improve responsiveness and accuracy in attack detection and mitigation. </p> <p>You define the health check in Route 53 and then associate or disassociate it with your Shield Advanced protection. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/ddos-overview.html#ddos-advanced-health-check-option">Shield Advanced Health-Based Detection</a> in the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF and AWS Shield Developer Guide</a>. </p>
    async fn disassociate_health_check(
        &self,
        input: DisassociateHealthCheckRequest,
    ) -> Result<DisassociateHealthCheckResponse, RusotoError<DisassociateHealthCheckError>>;

    /// <p>Authorizes the DDoS Response Team (DRT) to use email and phone to notify contacts about escalations to the DRT and to initiate proactive customer support.</p>
    async fn enable_proactive_engagement(
        &self,
    ) -> Result<EnableProactiveEngagementResponse, RusotoError<EnableProactiveEngagementError>>;

    /// <p>Returns the <code>SubscriptionState</code>, either <code>Active</code> or <code>Inactive</code>.</p>
    async fn get_subscription_state(
        &self,
    ) -> Result<GetSubscriptionStateResponse, RusotoError<GetSubscriptionStateError>>;

    /// <p>Returns all ongoing DDoS attacks or all DDoS attacks during a specified time period.</p>
    async fn list_attacks(
        &self,
        input: ListAttacksRequest,
    ) -> Result<ListAttacksResponse, RusotoError<ListAttacksError>>;

    /// <p>Retrieves the <a>ProtectionGroup</a> objects for the account.</p>
    async fn list_protection_groups(
        &self,
        input: ListProtectionGroupsRequest,
    ) -> Result<ListProtectionGroupsResponse, RusotoError<ListProtectionGroupsError>>;

    /// <p>Lists all <a>Protection</a> objects for the account.</p>
    async fn list_protections(
        &self,
        input: ListProtectionsRequest,
    ) -> Result<ListProtectionsResponse, RusotoError<ListProtectionsError>>;

    /// <p>Retrieves the resources that are included in the protection group. </p>
    async fn list_resources_in_protection_group(
        &self,
        input: ListResourcesInProtectionGroupRequest,
    ) -> Result<
        ListResourcesInProtectionGroupResponse,
        RusotoError<ListResourcesInProtectionGroupError>,
    >;

    /// <p>Updates the details of the list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you if you have proactive engagement enabled, for escalations to the DRT and to initiate proactive customer support.</p>
    async fn update_emergency_contact_settings(
        &self,
        input: UpdateEmergencyContactSettingsRequest,
    ) -> Result<
        UpdateEmergencyContactSettingsResponse,
        RusotoError<UpdateEmergencyContactSettingsError>,
    >;

    /// <p>Updates an existing protection group. A protection group is a grouping of protected resources so they can be handled as a collective. This resource grouping improves the accuracy of detection and reduces false positives. </p>
    async fn update_protection_group(
        &self,
        input: UpdateProtectionGroupRequest,
    ) -> Result<UpdateProtectionGroupResponse, RusotoError<UpdateProtectionGroupError>>;

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
    /// <p>Authorizes the DDoS Response Team (DRT) to access the specified Amazon S3 bucket containing your AWS WAF logs. You can associate up to 10 Amazon S3 buckets with your subscription.</p> <p>To use the services of the DRT and make an <code>AssociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_log_bucket(
        &self,
        input: AssociateDRTLogBucketRequest,
    ) -> Result<AssociateDRTLogBucketResponse, RusotoError<AssociateDRTLogBucketError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.AssociateDRTLogBucket");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateDRTLogBucketError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateDRTLogBucketResponse, _>()
    }

    /// <p>Authorizes the DDoS Response Team (DRT), using the specified role, to access your AWS account to assist with DDoS attack mitigation during potential attacks. This enables the DRT to inspect your AWS WAF configuration and create or update AWS WAF rules and web ACLs.</p> <p>You can associate only one <code>RoleArn</code> with your subscription. If you submit an <code>AssociateDRTRole</code> request for an account that already has an associated role, the new <code>RoleArn</code> will replace the existing <code>RoleArn</code>. </p> <p>Prior to making the <code>AssociateDRTRole</code> request, you must attach the <a href="https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy">AWSShieldDRTAccessPolicy</a> managed policy to the role you will specify in the request. For more information see <a href=" https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html">Attaching and Detaching IAM Policies</a>. The role must also trust the service principal <code> drt.shield.amazonaws.com</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html">IAM JSON Policy Elements: Principal</a>.</p> <p>The DRT will have access only to your AWS WAF and Shield resources. By submitting this request, you authorize the DRT to inspect your AWS WAF and Shield configuration and create and update AWS WAF rules and web ACLs on your behalf. The DRT takes these actions only if explicitly authorized by you.</p> <p>You must have the <code>iam:PassRole</code> permission to make an <code>AssociateDRTRole</code> request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html">Granting a User Permissions to Pass a Role to an AWS Service</a>. </p> <p>To use the services of the DRT and make an <code>AssociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>.</p>
    async fn associate_drt_role(
        &self,
        input: AssociateDRTRoleRequest,
    ) -> Result<AssociateDRTRoleResponse, RusotoError<AssociateDRTRoleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.AssociateDRTRole");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateDRTRoleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AssociateDRTRoleResponse, _>()
    }

    /// <p>Adds health-based detection to the Shield Advanced protection for a resource. Shield Advanced health-based detection uses the health of your AWS resource to improve responsiveness and accuracy in attack detection and mitigation. </p> <p>You define the health check in Route 53 and then associate it with your Shield Advanced protection. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/ddos-overview.html#ddos-advanced-health-check-option">Shield Advanced Health-Based Detection</a> in the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF and AWS Shield Developer Guide</a>. </p>
    async fn associate_health_check(
        &self,
        input: AssociateHealthCheckRequest,
    ) -> Result<AssociateHealthCheckResponse, RusotoError<AssociateHealthCheckError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.AssociateHealthCheck");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateHealthCheckError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateHealthCheckResponse, _>()
    }

    /// <p><p>Initializes proactive engagement and sets the list of contacts for the DDoS Response Team (DRT) to use. You must provide at least one phone number in the emergency contact list. </p> <p>After you have initialized proactive engagement using this call, to disable or enable proactive engagement, use the calls <code>DisableProactiveEngagement</code> and <code>EnableProactiveEngagement</code>. </p> <note> <p>This call defines the list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you for escalations to the DRT and to initiate proactive customer support.</p> <p>The contacts that you provide in the request replace any contacts that were already defined. If you already have contacts defined and want to use them, retrieve the list using <code>DescribeEmergencyContactSettings</code> and then provide it to this call. </p> </note></p>
    async fn associate_proactive_engagement_details(
        &self,
        input: AssociateProactiveEngagementDetailsRequest,
    ) -> Result<
        AssociateProactiveEngagementDetailsResponse,
        RusotoError<AssociateProactiveEngagementDetailsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.AssociateProactiveEngagementDetails",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                AssociateProactiveEngagementDetailsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateProactiveEngagementDetailsResponse, _>()
    }

    /// <p>Enables AWS Shield Advanced for a specific AWS resource. The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, AWS Global Accelerator accelerator, Elastic IP Address, or an Amazon Route 53 hosted zone.</p> <p>You can add protection to only a single resource with each CreateProtection request. If you want to add protection to multiple resources at once, use the <a href="https://console.aws.amazon.com/waf/">AWS WAF console</a>. For more information see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/getting-started-ddos.html">Getting Started with AWS Shield Advanced</a> and <a href="https://docs.aws.amazon.com/waf/latest/developerguide/configure-new-protection.html">Add AWS Shield Advanced Protection to more AWS Resources</a>.</p>
    async fn create_protection(
        &self,
        input: CreateProtectionRequest,
    ) -> Result<CreateProtectionResponse, RusotoError<CreateProtectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.CreateProtection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateProtectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateProtectionResponse, _>()
    }

    /// <p>Creates a grouping of protected resources so they can be handled as a collective. This resource grouping improves the accuracy of detection and reduces false positives. </p>
    async fn create_protection_group(
        &self,
        input: CreateProtectionGroupRequest,
    ) -> Result<CreateProtectionGroupResponse, RusotoError<CreateProtectionGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.CreateProtectionGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateProtectionGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateProtectionGroupResponse, _>()
    }

    /// <p>Activates AWS Shield Advanced for an account.</p> <p>When you initally create a subscription, your subscription is set to be automatically renewed at the end of the existing subscription period. You can change this by submitting an <code>UpdateSubscription</code> request. </p>
    async fn create_subscription(
        &self,
    ) -> Result<CreateSubscriptionResponse, RusotoError<CreateSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.CreateSubscription");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, CreateSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateSubscriptionResponse, _>()
    }

    /// <p>Deletes an AWS Shield Advanced <a>Protection</a>.</p>
    async fn delete_protection(
        &self,
        input: DeleteProtectionRequest,
    ) -> Result<DeleteProtectionResponse, RusotoError<DeleteProtectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DeleteProtection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteProtectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteProtectionResponse, _>()
    }

    /// <p>Removes the specified protection group.</p>
    async fn delete_protection_group(
        &self,
        input: DeleteProtectionGroupRequest,
    ) -> Result<DeleteProtectionGroupResponse, RusotoError<DeleteProtectionGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DeleteProtectionGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteProtectionGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteProtectionGroupResponse, _>()
    }

    /// <p>Removes AWS Shield Advanced from an account. AWS Shield Advanced requires a 1-year subscription commitment. You cannot delete a subscription prior to the completion of that commitment. </p>
    async fn delete_subscription(
        &self,
    ) -> Result<DeleteSubscriptionResponse, RusotoError<DeleteSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DeleteSubscription");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DeleteSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteSubscriptionResponse, _>()
    }

    /// <p>Describes the details of a DDoS attack. </p>
    async fn describe_attack(
        &self,
        input: DescribeAttackRequest,
    ) -> Result<DescribeAttackResponse, RusotoError<DescribeAttackError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeAttack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAttackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeAttackResponse, _>()
    }

    /// <p>Provides information about the number and type of attacks AWS Shield has detected in the last year for all resources that belong to your account, regardless of whether you've defined Shield protections for them. This operation is available to Shield customers as well as to Shield Advanced customers.</p> <p>The operation returns data for the time range of midnight UTC, one year ago, to midnight UTC, today. For example, if the current time is <code>2020-10-26 15:39:32 PDT</code>, equal to <code>2020-10-26 22:39:32 UTC</code>, then the time range for the attack data returned is from <code>2019-10-26 00:00:00 UTC</code> to <code>2020-10-26 00:00:00 UTC</code>. </p> <p>The time range indicates the period covered by the attack statistics data items.</p>
    async fn describe_attack_statistics(
        &self,
    ) -> Result<DescribeAttackStatisticsResponse, RusotoError<DescribeAttackStatisticsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.DescribeAttackStatistics",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeAttackStatisticsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeAttackStatisticsResponse, _>()
    }

    /// <p>Returns the current role and list of Amazon S3 log buckets used by the DDoS Response Team (DRT) to access your AWS account while assisting with attack mitigation.</p>
    async fn describe_drt_access(
        &self,
    ) -> Result<DescribeDRTAccessResponse, RusotoError<DescribeDRTAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeDRTAccess");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeDRTAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeDRTAccessResponse, _>()
    }

    /// <p>A list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you if you have proactive engagement enabled, for escalations to the DRT and to initiate proactive customer support.</p>
    async fn describe_emergency_contact_settings(
        &self,
    ) -> Result<
        DescribeEmergencyContactSettingsResponse,
        RusotoError<DescribeEmergencyContactSettingsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.DescribeEmergencyContactSettings",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeEmergencyContactSettingsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEmergencyContactSettingsResponse, _>()
    }

    /// <p>Lists the details of a <a>Protection</a> object.</p>
    async fn describe_protection(
        &self,
        input: DescribeProtectionRequest,
    ) -> Result<DescribeProtectionResponse, RusotoError<DescribeProtectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeProtection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProtectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeProtectionResponse, _>()
    }

    /// <p>Returns the specification for the specified protection group.</p>
    async fn describe_protection_group(
        &self,
        input: DescribeProtectionGroupRequest,
    ) -> Result<DescribeProtectionGroupResponse, RusotoError<DescribeProtectionGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeProtectionGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProtectionGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeProtectionGroupResponse, _>()
    }

    /// <p>Provides details about the AWS Shield Advanced subscription for an account.</p>
    async fn describe_subscription(
        &self,
    ) -> Result<DescribeSubscriptionResponse, RusotoError<DescribeSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeSubscription");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeSubscriptionResponse, _>()
    }

    /// <p>Removes authorization from the DDoS Response Team (DRT) to notify contacts about escalations to the DRT and to initiate proactive customer support.</p>
    async fn disable_proactive_engagement(
        &self,
    ) -> Result<DisableProactiveEngagementResponse, RusotoError<DisableProactiveEngagementError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.DisableProactiveEngagement",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DisableProactiveEngagementError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisableProactiveEngagementResponse, _>()
    }

    /// <p>Removes the DDoS Response Team's (DRT) access to the specified Amazon S3 bucket containing your AWS WAF logs.</p> <p>To make a <code>DisassociateDRTLogBucket</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTLogBucket</code> request to remove this access.</p>
    async fn disassociate_drt_log_bucket(
        &self,
        input: DisassociateDRTLogBucketRequest,
    ) -> Result<DisassociateDRTLogBucketResponse, RusotoError<DisassociateDRTLogBucketError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.DisassociateDRTLogBucket",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateDRTLogBucketError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateDRTLogBucketResponse, _>()
    }

    /// <p>Removes the DDoS Response Team's (DRT) access to your AWS account.</p> <p>To make a <code>DisassociateDRTRole</code> request, you must be subscribed to the <a href="https://aws.amazon.com/premiumsupport/business-support/">Business Support plan</a> or the <a href="https://aws.amazon.com/premiumsupport/enterprise-support/">Enterprise Support plan</a>. However, if you are not subscribed to one of these support plans, but had been previously and had granted the DRT access to your account, you can submit a <code>DisassociateDRTRole</code> request to remove this access.</p>
    async fn disassociate_drt_role(
        &self,
    ) -> Result<DisassociateDRTRoleResponse, RusotoError<DisassociateDRTRoleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DisassociateDRTRole");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DisassociateDRTRoleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisassociateDRTRoleResponse, _>()
    }

    /// <p>Removes health-based detection from the Shield Advanced protection for a resource. Shield Advanced health-based detection uses the health of your AWS resource to improve responsiveness and accuracy in attack detection and mitigation. </p> <p>You define the health check in Route 53 and then associate or disassociate it with your Shield Advanced protection. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/ddos-overview.html#ddos-advanced-health-check-option">Shield Advanced Health-Based Detection</a> in the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF and AWS Shield Developer Guide</a>. </p>
    async fn disassociate_health_check(
        &self,
        input: DisassociateHealthCheckRequest,
    ) -> Result<DisassociateHealthCheckResponse, RusotoError<DisassociateHealthCheckError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.DisassociateHealthCheck");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateHealthCheckError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateHealthCheckResponse, _>()
    }

    /// <p>Authorizes the DDoS Response Team (DRT) to use email and phone to notify contacts about escalations to the DRT and to initiate proactive customer support.</p>
    async fn enable_proactive_engagement(
        &self,
    ) -> Result<EnableProactiveEngagementResponse, RusotoError<EnableProactiveEngagementError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.EnableProactiveEngagement",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, EnableProactiveEngagementError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<EnableProactiveEngagementResponse, _>()
    }

    /// <p>Returns the <code>SubscriptionState</code>, either <code>Active</code> or <code>Inactive</code>.</p>
    async fn get_subscription_state(
        &self,
    ) -> Result<GetSubscriptionStateResponse, RusotoError<GetSubscriptionStateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.GetSubscriptionState");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetSubscriptionStateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSubscriptionStateResponse, _>()
    }

    /// <p>Returns all ongoing DDoS attacks or all DDoS attacks during a specified time period.</p>
    async fn list_attacks(
        &self,
        input: ListAttacksRequest,
    ) -> Result<ListAttacksResponse, RusotoError<ListAttacksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.ListAttacks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAttacksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAttacksResponse, _>()
    }

    /// <p>Retrieves the <a>ProtectionGroup</a> objects for the account.</p>
    async fn list_protection_groups(
        &self,
        input: ListProtectionGroupsRequest,
    ) -> Result<ListProtectionGroupsResponse, RusotoError<ListProtectionGroupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.ListProtectionGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListProtectionGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListProtectionGroupsResponse, _>()
    }

    /// <p>Lists all <a>Protection</a> objects for the account.</p>
    async fn list_protections(
        &self,
        input: ListProtectionsRequest,
    ) -> Result<ListProtectionsResponse, RusotoError<ListProtectionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.ListProtections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListProtectionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListProtectionsResponse, _>()
    }

    /// <p>Retrieves the resources that are included in the protection group. </p>
    async fn list_resources_in_protection_group(
        &self,
        input: ListResourcesInProtectionGroupRequest,
    ) -> Result<
        ListResourcesInProtectionGroupResponse,
        RusotoError<ListResourcesInProtectionGroupError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.ListResourcesInProtectionGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListResourcesInProtectionGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResourcesInProtectionGroupResponse, _>()
    }

    /// <p>Updates the details of the list of email addresses and phone numbers that the DDoS Response Team (DRT) can use to contact you if you have proactive engagement enabled, for escalations to the DRT and to initiate proactive customer support.</p>
    async fn update_emergency_contact_settings(
        &self,
        input: UpdateEmergencyContactSettingsRequest,
    ) -> Result<
        UpdateEmergencyContactSettingsResponse,
        RusotoError<UpdateEmergencyContactSettingsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShield_20160616.UpdateEmergencyContactSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateEmergencyContactSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateEmergencyContactSettingsResponse, _>()
    }

    /// <p>Updates an existing protection group. A protection group is a grouping of protected resources so they can be handled as a collective. This resource grouping improves the accuracy of detection and reduces false positives. </p>
    async fn update_protection_group(
        &self,
        input: UpdateProtectionGroupRequest,
    ) -> Result<UpdateProtectionGroupResponse, RusotoError<UpdateProtectionGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.UpdateProtectionGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateProtectionGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateProtectionGroupResponse, _>()
    }

    /// <p>Updates the details of an existing subscription. Only enter values for parameters you want to change. Empty parameters are not updated.</p>
    async fn update_subscription(
        &self,
        input: UpdateSubscriptionRequest,
    ) -> Result<UpdateSubscriptionResponse, RusotoError<UpdateSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSShield_20160616.UpdateSubscription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateSubscriptionResponse, _>()
    }
}
