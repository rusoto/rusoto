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

impl FmsClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "fms", &self.region, request_uri);

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

/// <p>An individual AWS Firewall Manager application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct App {
    /// <p>The application's name.</p>
    #[serde(rename = "AppName")]
    pub app_name: String,
    /// <p>The application's port number, for example <code>80</code>.</p>
    #[serde(rename = "Port")]
    pub port: i64,
    /// <p>The IP protocol name or number. The name can be one of <code>tcp</code>, <code>udp</code>, or <code>icmp</code>. For information on possible numbers, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
}

/// <p>An AWS Firewall Manager applications list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AppsListData {
    /// <p>An array of applications in the AWS Firewall Manager applications list.</p>
    #[serde(rename = "AppsList")]
    pub apps_list: Vec<App>,
    /// <p>The time that the AWS Firewall Manager applications list was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The time that the AWS Firewall Manager applications list was last updated.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ID of the AWS Firewall Manager applications list.</p>
    #[serde(rename = "ListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// <p>The name of the AWS Firewall Manager applications list.</p>
    #[serde(rename = "ListName")]
    pub list_name: String,
    /// <p>A unique identifier for each update to the list. When you update the list, the update token must match the token of the current version of the application list. You can retrieve the update token by getting the list. </p>
    #[serde(rename = "ListUpdateToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_update_token: Option<String>,
    /// <p>A map of previous version numbers to their corresponding <code>App</code> object arrays.</p>
    #[serde(rename = "PreviousAppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_apps_list: Option<::std::collections::HashMap<String, Vec<App>>>,
}

/// <p>Details of the AWS Firewall Manager applications list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AppsListDataSummary {
    /// <p>An array of <code>App</code> objects in the AWS Firewall Manager applications list.</p>
    #[serde(rename = "AppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_list: Option<Vec<App>>,
    /// <p>The Amazon Resource Name (ARN) of the applications list.</p>
    #[serde(rename = "ListArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_arn: Option<String>,
    /// <p>The ID of the applications list.</p>
    #[serde(rename = "ListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// <p>The name of the applications list.</p>
    #[serde(rename = "ListName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateAdminAccountRequest {
    /// <p>The AWS account ID to associate with AWS Firewall Manager as the AWS Firewall Manager administrator account. This can be an AWS Organizations master account or a member account. For more information about AWS Organizations and master accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts.html">Managing the AWS Accounts in Your Organization</a>. </p>
    #[serde(rename = "AdminAccount")]
    pub admin_account: String,
}

/// <p>Violations for an EC2 instance resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AwsEc2InstanceViolation {
    /// <p>Violations for network interfaces associated with the EC2 instance.</p>
    #[serde(rename = "AwsEc2NetworkInterfaceViolations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_network_interface_violations: Option<Vec<AwsEc2NetworkInterfaceViolation>>,
    /// <p>The resource ID of the EC2 instance.</p>
    #[serde(rename = "ViolationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target: Option<String>,
}

/// <p>Violations for network interfaces associated with an EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AwsEc2NetworkInterfaceViolation {
    /// <p>List of security groups that violate the rules specified in the master security group of the AWS Firewall Manager policy.</p>
    #[serde(rename = "ViolatingSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violating_security_groups: Option<Vec<String>>,
    /// <p>The resource ID of the network interface.</p>
    #[serde(rename = "ViolationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target: Option<String>,
}

/// <p>Details of the rule violation in a security group when compared to the master security group of the AWS Firewall Manager policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AwsVPCSecurityGroupViolation {
    /// <p>List of rules specified in the security group of the AWS Firewall Manager policy that partially match the <code>ViolationTarget</code> rule.</p>
    #[serde(rename = "PartialMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_matches: Option<Vec<PartialMatch>>,
    /// <p>Remediation options for the rule specified in the <code>ViolationTarget</code>.</p>
    #[serde(rename = "PossibleSecurityGroupRemediationActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_security_group_remediation_actions: Option<Vec<SecurityGroupRemediationAction>>,
    /// <p>The security group rule that is being evaluated.</p>
    #[serde(rename = "ViolationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target: Option<String>,
    /// <p>A description of the security group that violates the policy.</p>
    #[serde(rename = "ViolationTargetDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target_description: Option<String>,
}

/// <p>Details of the resource that is not protected by the policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComplianceViolator {
    /// <p>The resource ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The resource type. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. For example: <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code>, <code>AWS::CloudFront::Distribution</code>, or <code>AWS::NetworkFirewall::FirewallPolicy</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The reason that the resource is not protected by the policy.</p>
    #[serde(rename = "ViolationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_reason: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppsListRequest {
    /// <p>The ID of the applications list that you want to delete. You can retrieve this ID from <code>PutAppsList</code>, <code>ListAppsLists</code>, and <code>GetAppsList</code>.</p>
    #[serde(rename = "ListId")]
    pub list_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNotificationChannelRequest {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePolicyRequest {
    /// <p>If <code>True</code>, the request performs cleanup according to the policy type. </p> <p>For AWS WAF and Shield Advanced policies, the cleanup does the following:</p> <ul> <li> <p>Deletes rule groups created by AWS Firewall Manager</p> </li> <li> <p>Removes web ACLs from in-scope resources</p> </li> <li> <p>Deletes web ACLs that contain no rules or rule groups</p> </li> </ul> <p>For security group policies, the cleanup does the following for each security group in the policy:</p> <ul> <li> <p>Disassociates the security group from in-scope resources </p> </li> <li> <p>Deletes the security group if it was created through Firewall Manager and if it's no longer associated with any resources through another policy</p> </li> </ul> <p>After the cleanup, in-scope resources are no longer protected by web ACLs in this policy. Protection of out-of-scope resources remains unchanged. Scope is determined by tags that you create and accounts that you associate with the policy. When creating the policy, if you specify that only resources in specific accounts or with specific tags are in scope of the policy, those accounts and resources are handled by the policy. All others are out of scope. If you don't specify tags or accounts, all resources are in scope. </p>
    #[serde(rename = "DeleteAllPolicyResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_all_policy_resources: Option<bool>,
    /// <p>The ID of the policy that you want to delete. You can retrieve this ID from <code>PutPolicy</code> and <code>ListPolicies</code>.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProtocolsListRequest {
    /// <p>The ID of the protocols list that you want to delete. You can retrieve this ID from <code>PutProtocolsList</code>, <code>ListProtocolsLists</code>, and <code>GetProtocolsLost</code>.</p>
    #[serde(rename = "ListId")]
    pub list_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateAdminAccountRequest {}

/// <p>Describes the compliance status for the account. An account is considered noncompliant if it includes resources that are not protected by the specified policy or that don't comply with the policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EvaluationResult {
    /// <p>Describes an AWS account's compliance with the AWS Firewall Manager policy.</p>
    #[serde(rename = "ComplianceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    /// <p>Indicates that over 100 resources are noncompliant with the AWS Firewall Manager policy.</p>
    #[serde(rename = "EvaluationLimitExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_limit_exceeded: Option<bool>,
    /// <p>The number of resources that are noncompliant with the specified policy. For AWS WAF and Shield Advanced policies, a resource is considered noncompliant if it is not associated with the policy. For security group policies, a resource is considered noncompliant if it doesn't comply with the rules of the policy and remediation is disabled or not possible.</p>
    #[serde(rename = "ViolatorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violator_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAdminAccountRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAdminAccountResponse {
    /// <p>The AWS account that is set as the AWS Firewall Manager administrator.</p>
    #[serde(rename = "AdminAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account: Option<String>,
    /// <p>The status of the AWS account that you set as the AWS Firewall Manager administrator.</p>
    #[serde(rename = "RoleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppsListRequest {
    /// <p>Specifies whether the list to retrieve is a default list owned by AWS Firewall Manager.</p>
    #[serde(rename = "DefaultList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_list: Option<bool>,
    /// <p>The ID of the AWS Firewall Manager applications list that you want the details for.</p>
    #[serde(rename = "ListId")]
    pub list_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppsListResponse {
    /// <p>Information about the specified AWS Firewall Manager applications list.</p>
    #[serde(rename = "AppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_list: Option<AppsListData>,
    /// <p>The Amazon Resource Name (ARN) of the applications list.</p>
    #[serde(rename = "AppsListArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_list_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetComplianceDetailRequest {
    /// <p>The AWS account that owns the resources that you want to get the details for.</p>
    #[serde(rename = "MemberAccount")]
    pub member_account: String,
    /// <p>The ID of the policy that you want to get the details for. <code>PolicyId</code> is returned by <code>PutPolicy</code> and by <code>ListPolicies</code>.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetComplianceDetailResponse {
    /// <p>Information about the resources and the policy that you specified in the <code>GetComplianceDetail</code> request.</p>
    #[serde(rename = "PolicyComplianceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_compliance_detail: Option<PolicyComplianceDetail>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNotificationChannelRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNotificationChannelResponse {
    /// <p>The IAM role that is used by AWS Firewall Manager to record activity to SNS.</p>
    #[serde(rename = "SnsRoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_role_name: Option<String>,
    /// <p>The SNS topic that records AWS Firewall Manager activity. </p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPolicyRequest {
    /// <p>The ID of the AWS Firewall Manager policy that you want the details for.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPolicyResponse {
    /// <p>Information about the specified AWS Firewall Manager policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
    /// <p>The Amazon Resource Name (ARN) of the specified policy.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProtectionStatusRequest {
    /// <p>The end of the time period to query for the attacks. This is a <code>timestamp</code> type. The request syntax listing indicates a <code>number</code> type because the default used by AWS Firewall Manager is Unix time in seconds. However, any valid <code>timestamp</code> format is allowed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Specifies the number of objects that you want AWS Firewall Manager to return for this request. If you have more objects than the number that you specify for <code>MaxResults</code>, the response includes a <code>NextToken</code> value that you can use to get another batch of objects.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The AWS account that is in scope of the policy that you want to get the details for.</p>
    #[serde(rename = "MemberAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_id: Option<String>,
    /// <p>If you specify a value for <code>MaxResults</code> and you have more objects than the number that you specify for <code>MaxResults</code>, AWS Firewall Manager returns a <code>NextToken</code> value in the response, which you can use to retrieve another group of objects. For the second and subsequent <code>GetProtectionStatus</code> requests, specify the value of <code>NextToken</code> from the previous response to get information about another batch of objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the policy for which you want to get the attack information.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p>The start of the time period to query for the attacks. This is a <code>timestamp</code> type. The request syntax listing indicates a <code>number</code> type because the default used by AWS Firewall Manager is Unix time in seconds. However, any valid <code>timestamp</code> format is allowed.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProtectionStatusResponse {
    /// <p>The ID of the AWS Firewall administrator account for this policy.</p>
    #[serde(rename = "AdminAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
    /// <p>Details about the attack, including the following:</p> <ul> <li> <p>Attack type</p> </li> <li> <p>Account ID</p> </li> <li> <p>ARN of the resource attacked</p> </li> <li> <p>Start time of the attack</p> </li> <li> <p>End time of the attack (ongoing attacks will not have an end time)</p> </li> </ul> <p>The details are in JSON format. </p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p>If you have more objects than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more objects, submit another <code>GetProtectionStatus</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p> <p>AWS SDKs provide auto-pagination that identify <code>NextToken</code> in a response and make subsequent request calls automatically on your behalf. However, this feature is not supported by <code>GetProtectionStatus</code>. You must submit subsequent requests with <code>NextToken</code> using your own processes. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service type that is protected by the policy. Currently, this is always <code>SHIELD_ADVANCED</code>.</p>
    #[serde(rename = "ServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProtocolsListRequest {
    /// <p>Specifies whether the list to retrieve is a default list owned by AWS Firewall Manager.</p>
    #[serde(rename = "DefaultList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_list: Option<bool>,
    /// <p>The ID of the AWS Firewall Manager protocols list that you want the details for.</p>
    #[serde(rename = "ListId")]
    pub list_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProtocolsListResponse {
    /// <p>Information about the specified AWS Firewall Manager protocols list.</p>
    #[serde(rename = "ProtocolsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols_list: Option<ProtocolsListData>,
    /// <p>The Amazon Resource Name (ARN) of the specified protocols list.</p>
    #[serde(rename = "ProtocolsListArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols_list_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetViolationDetailsRequest {
    /// <p>The AWS account ID that you want the details for.</p>
    #[serde(rename = "MemberAccount")]
    pub member_account: String,
    /// <p>The ID of the AWS Firewall Manager policy that you want the details for. This currently only supports security group content audit policies.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p>The ID of the resource that has violations.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The resource type. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. Supported resource types are: <code>AWS::EC2::Instance</code>, <code>AWS::EC2::NetworkInterface</code>, <code>AWS::EC2::SecurityGroup</code>, <code>AWS::NetworkFirewall::FirewallPolicy</code>, and <code>AWS::EC2::Subnet</code>. </p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetViolationDetailsResponse {
    /// <p>Violation detail for a resource.</p>
    #[serde(rename = "ViolationDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_detail: Option<ViolationDetail>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAppsListsRequest {
    /// <p>Specifies whether the lists to retrieve are default lists owned by AWS Firewall Manager.</p>
    #[serde(rename = "DefaultLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lists: Option<bool>,
    /// <p>The maximum number of objects that you want AWS Firewall Manager to return for this request. If more objects are available, in the response, AWS Firewall Manager provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p> <p>If you don't specify this, AWS Firewall Manager returns all available objects.</p>
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    /// <p>If you specify a value for <code>MaxResults</code> in your list request, and you have more objects than the maximum, AWS Firewall Manager returns this token in the response. For all but the first request, you provide the token returned by the prior request in the request parameters, to retrieve the next batch of objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAppsListsResponse {
    /// <p>An array of <code>AppsListDataSummary</code> objects.</p>
    #[serde(rename = "AppsLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_lists: Option<Vec<AppsListDataSummary>>,
    /// <p>If you specify a value for <code>MaxResults</code> in your list request, and you have more objects than the maximum, AWS Firewall Manager returns this token in the response. You can use this token in subsequent requests to retrieve the next batch of objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListComplianceStatusRequest {
    /// <p>Specifies the number of <code>PolicyComplianceStatus</code> objects that you want AWS Firewall Manager to return for this request. If you have more <code>PolicyComplianceStatus</code> objects than the number that you specify for <code>MaxResults</code>, the response includes a <code>NextToken</code> value that you can use to get another batch of <code>PolicyComplianceStatus</code> objects.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you specify a value for <code>MaxResults</code> and you have more <code>PolicyComplianceStatus</code> objects than the number that you specify for <code>MaxResults</code>, AWS Firewall Manager returns a <code>NextToken</code> value in the response that allows you to list another group of <code>PolicyComplianceStatus</code> objects. For the second and subsequent <code>ListComplianceStatus</code> requests, specify the value of <code>NextToken</code> from the previous response to get information about another batch of <code>PolicyComplianceStatus</code> objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the AWS Firewall Manager policy that you want the details for.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListComplianceStatusResponse {
    /// <p>If you have more <code>PolicyComplianceStatus</code> objects than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more <code>PolicyComplianceStatus</code> objects, submit another <code>ListComplianceStatus</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>PolicyComplianceStatus</code> objects.</p>
    #[serde(rename = "PolicyComplianceStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_compliance_status_list: Option<Vec<PolicyComplianceStatus>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMemberAccountsRequest {
    /// <p>Specifies the number of member account IDs that you want AWS Firewall Manager to return for this request. If you have more IDs than the number that you specify for <code>MaxResults</code>, the response includes a <code>NextToken</code> value that you can use to get another batch of member account IDs.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you specify a value for <code>MaxResults</code> and you have more account IDs than the number that you specify for <code>MaxResults</code>, AWS Firewall Manager returns a <code>NextToken</code> value in the response that allows you to list another group of IDs. For the second and subsequent <code>ListMemberAccountsRequest</code> requests, specify the value of <code>NextToken</code> from the previous response to get information about another batch of member account IDs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMemberAccountsResponse {
    /// <p>An array of account IDs.</p>
    #[serde(rename = "MemberAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_accounts: Option<Vec<String>>,
    /// <p>If you have more member account IDs than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more IDs, submit another <code>ListMemberAccounts</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPoliciesRequest {
    /// <p>Specifies the number of <code>PolicySummary</code> objects that you want AWS Firewall Manager to return for this request. If you have more <code>PolicySummary</code> objects than the number that you specify for <code>MaxResults</code>, the response includes a <code>NextToken</code> value that you can use to get another batch of <code>PolicySummary</code> objects.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you specify a value for <code>MaxResults</code> and you have more <code>PolicySummary</code> objects than the number that you specify for <code>MaxResults</code>, AWS Firewall Manager returns a <code>NextToken</code> value in the response that allows you to list another group of <code>PolicySummary</code> objects. For the second and subsequent <code>ListPolicies</code> requests, specify the value of <code>NextToken</code> from the previous response to get information about another batch of <code>PolicySummary</code> objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPoliciesResponse {
    /// <p>If you have more <code>PolicySummary</code> objects than the number that you specified for <code>MaxResults</code> in the request, the response includes a <code>NextToken</code> value. To list more <code>PolicySummary</code> objects, submit another <code>ListPolicies</code> request, and specify the <code>NextToken</code> value from the response in the <code>NextToken</code> value in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>PolicySummary</code> objects.</p>
    #[serde(rename = "PolicyList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_list: Option<Vec<PolicySummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProtocolsListsRequest {
    /// <p>Specifies whether the lists to retrieve are default lists owned by AWS Firewall Manager.</p>
    #[serde(rename = "DefaultLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lists: Option<bool>,
    /// <p>The maximum number of objects that you want AWS Firewall Manager to return for this request. If more objects are available, in the response, AWS Firewall Manager provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p> <p>If you don't specify this, AWS Firewall Manager returns all available objects.</p>
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    /// <p>If you specify a value for <code>MaxResults</code> in your list request, and you have more objects than the maximum, AWS Firewall Manager returns this token in the response. For all but the first request, you provide the token returned by the prior request in the request parameters, to retrieve the next batch of objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProtocolsListsResponse {
    /// <p>If you specify a value for <code>MaxResults</code> in your list request, and you have more objects than the maximum, AWS Firewall Manager returns this token in the response. You can use this token in subsequent requests to retrieve the next batch of objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>ProtocolsListDataSummary</code> objects.</p>
    #[serde(rename = "ProtocolsLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols_lists: Option<Vec<ProtocolsListDataSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to return tags for. The AWS Firewall Manager resources that support tagging are policies, applications lists, and protocols lists. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags associated with the resource.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p>Violation details for AWS Network Firewall for a subnet that's not associated to the expected Firewall Manager managed route table.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkFirewallMissingExpectedRTViolation {
    /// <p>The Availability Zone of a violating subnet. </p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The resource ID of the current route table that's associated with the subnet, if one is available.</p>
    #[serde(rename = "CurrentRouteTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_route_table: Option<String>,
    /// <p>The resource ID of the route table that should be associated with the subnet.</p>
    #[serde(rename = "ExpectedRouteTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_route_table: Option<String>,
    /// <p>The resource ID of the VPC associated with a violating subnet.</p>
    #[serde(rename = "VPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,
    /// <p>The ID of the AWS Network Firewall or VPC resource that's in violation.</p>
    #[serde(rename = "ViolationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target: Option<String>,
}

/// <p>Violation details for AWS Network Firewall for a subnet that doesn't have a Firewall Manager managed firewall in its VPC. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkFirewallMissingFirewallViolation {
    /// <p>The Availability Zone of a violating subnet. </p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The reason the resource has this violation, if one is available. </p>
    #[serde(rename = "TargetViolationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_violation_reason: Option<String>,
    /// <p>The resource ID of the VPC associated with a violating subnet.</p>
    #[serde(rename = "VPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,
    /// <p>The ID of the AWS Network Firewall or VPC resource that's in violation.</p>
    #[serde(rename = "ViolationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target: Option<String>,
}

/// <p>Violation details for AWS Network Firewall for an Availability Zone that's missing the expected Firewall Manager managed subnet.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkFirewallMissingSubnetViolation {
    /// <p>The Availability Zone of a violating subnet. </p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The reason the resource has this violation, if one is available. </p>
    #[serde(rename = "TargetViolationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_violation_reason: Option<String>,
    /// <p>The resource ID of the VPC associated with a violating subnet.</p>
    #[serde(rename = "VPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,
    /// <p>The ID of the AWS Network Firewall or VPC resource that's in violation.</p>
    #[serde(rename = "ViolationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target: Option<String>,
}

/// <p>The definition of the AWS Network Firewall firewall policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkFirewallPolicyDescription {
    /// <p>The stateful rule groups that are used in the Network Firewall firewall policy. </p>
    #[serde(rename = "StatefulRuleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rule_groups: Option<Vec<StatefulRuleGroup>>,
    /// <p>Names of custom actions that are available for use in the stateless default actions settings.</p>
    #[serde(rename = "StatelessCustomActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_custom_actions: Option<Vec<String>>,
    /// <p>The actions to take on packets that don't match any of the stateless rule groups. </p>
    #[serde(rename = "StatelessDefaultActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_default_actions: Option<Vec<String>>,
    /// <p>The actions to take on packet fragments that don't match any of the stateless rule groups. </p>
    #[serde(rename = "StatelessFragmentDefaultActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_fragment_default_actions: Option<Vec<String>>,
    /// <p>The stateless rule groups that are used in the Network Firewall firewall policy. </p>
    #[serde(rename = "StatelessRuleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_rule_groups: Option<Vec<StatelessRuleGroup>>,
}

/// <p>Violation details for AWS Network Firewall for a firewall policy that has a different <a>NetworkFirewallPolicyDescription</a> than is required by the Firewall Manager policy. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkFirewallPolicyModifiedViolation {
    /// <p>The policy that's currently in use in the individual account. </p>
    #[serde(rename = "CurrentPolicyDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_policy_description: Option<NetworkFirewallPolicyDescription>,
    /// <p>The policy that should be in use in the individual account in order to be compliant. </p>
    #[serde(rename = "ExpectedPolicyDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_policy_description: Option<NetworkFirewallPolicyDescription>,
    /// <p>The ID of the AWS Network Firewall or VPC resource that's in violation.</p>
    #[serde(rename = "ViolationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_target: Option<String>,
}

/// <p>The reference rule that partially matches the <code>ViolationTarget</code> rule and violation reason.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PartialMatch {
    /// <p>The reference rule from the master security group of the AWS Firewall Manager policy.</p>
    #[serde(rename = "Reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// <p>The violation reason.</p>
    #[serde(rename = "TargetViolationReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_violation_reasons: Option<Vec<String>>,
}

/// <p>An AWS Firewall Manager policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Policy {
    /// <p><p>Specifies the AWS account IDs and AWS Organizations organizational units (OUs) to exclude from the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p> <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, AWS Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p> <p>You can specify account IDs, OUs, or a combination: </p> <ul> <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li> <li> <p>Specify OUs by setting the key to <code>ORG<em>UNIT</code>. For example, the following is a valid map: <code>{“ORG</em>UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li> <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li> </ul></p>
    #[serde(rename = "ExcludeMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_map: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>If set to <code>True</code>, resources with the tags that are specified in the <code>ResourceTag</code> array are not in scope of the policy. If set to <code>False</code>, and the <code>ResourceTag</code> array is not null, only resources with the specified tags are in scope of the policy.</p>
    #[serde(rename = "ExcludeResourceTags")]
    pub exclude_resource_tags: bool,
    /// <p><p>Specifies the AWS account IDs and AWS Organizations organizational units (OUs) to include in the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p> <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, AWS Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p> <p>You can specify account IDs, OUs, or a combination: </p> <ul> <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li> <li> <p>Specify OUs by setting the key to <code>ORG<em>UNIT</code>. For example, the following is a valid map: <code>{“ORG</em>UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li> <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li> </ul></p>
    #[serde(rename = "IncludeMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_map: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ID of the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The name of the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p>A unique identifier for each update to the policy. When issuing a <code>PutPolicy</code> request, the <code>PolicyUpdateToken</code> in the request must match the <code>PolicyUpdateToken</code> of the current policy version. To get the <code>PolicyUpdateToken</code> of the current policy version, use a <code>GetPolicy</code> request.</p>
    #[serde(rename = "PolicyUpdateToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_update_token: Option<String>,
    /// <p>Indicates if the policy should be automatically applied to new resources.</p>
    #[serde(rename = "RemediationEnabled")]
    pub remediation_enabled: bool,
    /// <p>An array of <code>ResourceTag</code> objects.</p>
    #[serde(rename = "ResourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
    /// <p>The type of resource protected by or in scope of the policy. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. For AWS WAF and Shield Advanced, examples include <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> and <code>AWS::CloudFront::Distribution</code>. For a security group common policy, valid values are <code>AWS::EC2::NetworkInterface</code> and <code>AWS::EC2::Instance</code>. For a security group content audit policy, valid values are <code>AWS::EC2::SecurityGroup</code>, <code>AWS::EC2::NetworkInterface</code>, and <code>AWS::EC2::Instance</code>. For a security group usage audit policy, the value is <code>AWS::EC2::SecurityGroup</code>. For an AWS Network Firewall policy, the value is <code>AWS::EC2::VPC</code>.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>An array of <code>ResourceType</code>.</p>
    #[serde(rename = "ResourceTypeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_list: Option<Vec<String>>,
    /// <p>Details about the security service that is being used to protect the resources.</p>
    #[serde(rename = "SecurityServicePolicyData")]
    pub security_service_policy_data: SecurityServicePolicyData,
}

/// <p>Describes the noncompliant resources in a member account for a specific AWS Firewall Manager policy. A maximum of 100 entries are displayed. If more than 100 resources are noncompliant, <code>EvaluationLimitExceeded</code> is set to <code>True</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PolicyComplianceDetail {
    /// <p>Indicates if over 100 resources are noncompliant with the AWS Firewall Manager policy.</p>
    #[serde(rename = "EvaluationLimitExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_limit_exceeded: Option<bool>,
    /// <p>A timestamp that indicates when the returned information should be considered out of date.</p>
    #[serde(rename = "ExpiredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<f64>,
    /// <p>Details about problems with dependent services, such as AWS WAF or AWS Config, that are causing a resource to be noncompliant. The details include the name of the dependent service and the error message received that indicates the problem with the service.</p>
    #[serde(rename = "IssueInfoMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_info_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>The AWS account ID.</p>
    #[serde(rename = "MemberAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account: Option<String>,
    /// <p>The ID of the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The AWS account that created the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_owner: Option<String>,
    /// <p>An array of resources that aren't protected by the AWS WAF or Shield Advanced policy or that aren't in compliance with the security group policy.</p>
    #[serde(rename = "Violators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violators: Option<Vec<ComplianceViolator>>,
}

/// <p>Indicates whether the account is compliant with the specified policy. An account is considered noncompliant if it includes resources that are not protected by the policy, for AWS WAF and Shield Advanced policies, or that are noncompliant with the policy, for security group policies.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PolicyComplianceStatus {
    /// <p>An array of <code>EvaluationResult</code> objects.</p>
    #[serde(rename = "EvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    /// <p>Details about problems with dependent services, such as AWS WAF or AWS Config, that are causing a resource to be noncompliant. The details include the name of the dependent service and the error message received that indicates the problem with the service.</p>
    #[serde(rename = "IssueInfoMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_info_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>Timestamp of the last update to the <code>EvaluationResult</code> objects.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The member account ID.</p>
    #[serde(rename = "MemberAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account: Option<String>,
    /// <p>The ID of the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The name of the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// <p>The AWS account that created the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_owner: Option<String>,
}

/// <p>Details of the AWS Firewall Manager policy. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PolicySummary {
    /// <p>The Amazon Resource Name (ARN) of the specified policy.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The ID of the specified policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The name of the specified policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// <p>Indicates if the policy should be automatically applied to new resources.</p>
    #[serde(rename = "RemediationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_enabled: Option<bool>,
    /// <p>The type of resource protected by or in scope of the policy. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. For AWS WAF and Shield Advanced, examples include <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> and <code>AWS::CloudFront::Distribution</code>. For a security group common policy, valid values are <code>AWS::EC2::NetworkInterface</code> and <code>AWS::EC2::Instance</code>. For a security group content audit policy, valid values are <code>AWS::EC2::SecurityGroup</code>, <code>AWS::EC2::NetworkInterface</code>, and <code>AWS::EC2::Instance</code>. For a security group usage audit policy, the value is <code>AWS::EC2::SecurityGroup</code>. For an AWS Network Firewall policy, the value is <code>AWS::EC2::VPC</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The service that the policy is using to protect the resources. This specifies the type of policy that is created, either an AWS WAF policy, a Shield Advanced policy, or a security group policy.</p>
    #[serde(rename = "SecurityServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_service_type: Option<String>,
}

/// <p>An AWS Firewall Manager protocols list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProtocolsListData {
    /// <p>The time that the AWS Firewall Manager protocols list was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The time that the AWS Firewall Manager protocols list was last updated.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ID of the AWS Firewall Manager protocols list.</p>
    #[serde(rename = "ListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// <p>The name of the AWS Firewall Manager protocols list.</p>
    #[serde(rename = "ListName")]
    pub list_name: String,
    /// <p>A unique identifier for each update to the list. When you update the list, the update token must match the token of the current version of the application list. You can retrieve the update token by getting the list. </p>
    #[serde(rename = "ListUpdateToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_update_token: Option<String>,
    /// <p>A map of previous version numbers to their corresponding protocol arrays.</p>
    #[serde(rename = "PreviousProtocolsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_protocols_list: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>An array of protocols in the AWS Firewall Manager protocols list.</p>
    #[serde(rename = "ProtocolsList")]
    pub protocols_list: Vec<String>,
}

/// <p>Details of the AWS Firewall Manager protocols list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProtocolsListDataSummary {
    /// <p>The Amazon Resource Name (ARN) of the specified protocols list.</p>
    #[serde(rename = "ListArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_arn: Option<String>,
    /// <p>The ID of the specified protocols list.</p>
    #[serde(rename = "ListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// <p>The name of the specified protocols list.</p>
    #[serde(rename = "ListName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// <p>An array of protocols in the AWS Firewall Manager protocols list.</p>
    #[serde(rename = "ProtocolsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols_list: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAppsListRequest {
    /// <p>The details of the AWS Firewall Manager applications list to be created.</p>
    #[serde(rename = "AppsList")]
    pub apps_list: AppsListData,
    /// <p>The tags associated with the resource.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAppsListResponse {
    /// <p>The details of the AWS Firewall Manager applications list.</p>
    #[serde(rename = "AppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_list: Option<AppsListData>,
    /// <p>The Amazon Resource Name (ARN) of the applications list.</p>
    #[serde(rename = "AppsListArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_list_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutNotificationChannelRequest {
    /// <p>The Amazon Resource Name (ARN) of the IAM role that allows Amazon SNS to record AWS Firewall Manager activity. </p>
    #[serde(rename = "SnsRoleName")]
    pub sns_role_name: String,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic that collects notifications from AWS Firewall Manager.</p>
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPolicyRequest {
    /// <p>The details of the AWS Firewall Manager policy to be created.</p>
    #[serde(rename = "Policy")]
    pub policy: Policy,
    /// <p>The tags to add to the AWS resource.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPolicyResponse {
    /// <p>The details of the AWS Firewall Manager policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutProtocolsListRequest {
    /// <p>The details of the AWS Firewall Manager protocols list to be created.</p>
    #[serde(rename = "ProtocolsList")]
    pub protocols_list: ProtocolsListData,
    /// <p>The tags associated with the resource.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutProtocolsListResponse {
    /// <p>The details of the AWS Firewall Manager protocols list.</p>
    #[serde(rename = "ProtocolsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols_list: Option<ProtocolsListData>,
    /// <p>The Amazon Resource Name (ARN) of the protocols list.</p>
    #[serde(rename = "ProtocolsListArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols_list_arn: Option<String>,
}

/// <p>The resource tags that AWS Firewall Manager uses to determine if a particular resource should be included or excluded from the AWS Firewall Manager policy. Tags enable you to categorize your AWS resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value. Firewall Manager combines the tags with "AND" so that, if you add more than one tag to a policy scope, a resource must have all the specified tags to be included or excluded. For more information, see <a href="https://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/tag-editor.html">Working with Tag Editor</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceTag {
    /// <p>The resource tag key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The resource tag value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Violation detail based on resource type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceViolation {
    /// <p>Violation details for an EC2 instance.</p>
    #[serde(rename = "AwsEc2InstanceViolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_instance_violation: Option<AwsEc2InstanceViolation>,
    /// <p>Violation details for network interface.</p>
    #[serde(rename = "AwsEc2NetworkInterfaceViolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_network_interface_violation: Option<AwsEc2NetworkInterfaceViolation>,
    /// <p>Violation details for security groups.</p>
    #[serde(rename = "AwsVPCSecurityGroupViolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_vpc_security_group_violation: Option<AwsVPCSecurityGroupViolation>,
    /// <p>Violation detail for an Network Firewall policy that indicates that a subnet is not associated with the expected Firewall Manager managed route table. </p>
    #[serde(rename = "NetworkFirewallMissingExpectedRTViolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_firewall_missing_expected_rt_violation:
        Option<NetworkFirewallMissingExpectedRTViolation>,
    /// <p>Violation detail for an Network Firewall policy that indicates that a subnet has no Firewall Manager managed firewall in its VPC. </p>
    #[serde(rename = "NetworkFirewallMissingFirewallViolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_firewall_missing_firewall_violation:
        Option<NetworkFirewallMissingFirewallViolation>,
    /// <p>Violation detail for an Network Firewall policy that indicates that an Availability Zone is missing the expected Firewall Manager managed subnet.</p>
    #[serde(rename = "NetworkFirewallMissingSubnetViolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_firewall_missing_subnet_violation: Option<NetworkFirewallMissingSubnetViolation>,
    /// <p>Violation detail for an Network Firewall policy that indicates that a firewall policy in an individual account has been modified in a way that makes it noncompliant. For example, the individual account owner might have deleted a rule group, changed the priority of a stateless rule group, or changed a policy default action.</p>
    #[serde(rename = "NetworkFirewallPolicyModifiedViolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_firewall_policy_modified_violation: Option<NetworkFirewallPolicyModifiedViolation>,
}

/// <p>Remediation option for the rule specified in the <code>ViolationTarget</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityGroupRemediationAction {
    /// <p>Brief description of the action that will be performed.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates if the current action is the default action.</p>
    #[serde(rename = "IsDefaultAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_action: Option<bool>,
    /// <p>The remediation action that will be performed.</p>
    #[serde(rename = "RemediationActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_action_type: Option<String>,
    /// <p>The final state of the rule specified in the <code>ViolationTarget</code> after it is remediated.</p>
    #[serde(rename = "RemediationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_result: Option<SecurityGroupRuleDescription>,
}

/// <p>Describes a set of permissions for a security group rule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityGroupRuleDescription {
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type number. A value of <code>-1</code> indicates all ICMP/ICMPv6 types.</p>
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p>The IPv4 ranges for the security group rule.</p>
    #[serde(rename = "IPV4Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_range: Option<String>,
    /// <p>The IPv6 ranges for the security group rule.</p>
    #[serde(rename = "IPV6Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_range: Option<String>,
    /// <p>The ID of the prefix list for the security group rule.</p>
    #[serde(rename = "PrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_id: Option<String>,
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code. A value of <code>-1</code> indicates all ICMP/ICMPv6 codes.</p>
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

/// <p>Details about the security service that is being used to protect the resources.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SecurityServicePolicyData {
    /// <p><p>Details about the service that are specific to the service type, in JSON format. For service type <code>SHIELD<em>ADVANCED</code>, this is an empty string.</p> <ul> <li> <p>Example: <code>NETWORK</em>FIREWALL</code> </p> <p> <code>&quot;{&quot;type&quot;:&quot;NETWORK<em>FIREWALL&quot;,&quot;networkFirewallStatelessRuleGroupReferences&quot;:[{&quot;resourceARN&quot;:&quot;arn:aws:network-firewall:us-west-1:1234567891011:stateless-rulegroup/rulegroup2&quot;,&quot;priority&quot;:10}],&quot;networkFirewallStatelessDefaultActions&quot;:[&quot;aws:pass&quot;,&quot;custom1&quot;],&quot;networkFirewallStatelessFragmentDefaultActions&quot;:[&quot;custom2&quot;,&quot;aws:pass&quot;],&quot;networkFirewallStatelessCustomActions&quot;:[{&quot;actionName&quot;:&quot;custom1&quot;,&quot;actionDefinition&quot;:{&quot;publishMetricAction&quot;:{&quot;dimensions&quot;:[{&quot;value&quot;:&quot;dimension1&quot;}]}}},{&quot;actionName&quot;:&quot;custom2&quot;,&quot;actionDefinition&quot;:{&quot;publishMetricAction&quot;:{&quot;dimensions&quot;:[{&quot;value&quot;:&quot;dimension2&quot;}]}}}],&quot;networkFirewallStatefulRuleGroupReferences&quot;:[{&quot;resourceARN&quot;:&quot;arn:aws:network-firewall:us-west-1:1234567891011:stateful-rulegroup/rulegroup1&quot;}],&quot;networkFirewallOrchestrationConfig&quot;:{&quot;singleFirewallEndpointPerVPC&quot;:true,&quot;allowedIPV4CidrList&quot;:[&quot;10.24.34.0/28&quot;]} }&quot;</code> </p> </li> <li> <p>Example: <code>WAFV2</code> </p> <p> <code>&quot;{&quot;type&quot;:&quot;WAFV2&quot;,&quot;preProcessRuleGroups&quot;:[{&quot;ruleGroupArn&quot;:null,&quot;overrideAction&quot;:{&quot;type&quot;:&quot;NONE&quot;},&quot;managedRuleGroupIdentifier&quot;:{&quot;version&quot;:null,&quot;vendorName&quot;:&quot;AWS&quot;,&quot;managedRuleGroupName&quot;:&quot;AWSManagedRulesAmazonIpReputationList&quot;},&quot;ruleGroupType&quot;:&quot;ManagedRuleGroup&quot;,&quot;excludeRules&quot;:[]}],&quot;postProcessRuleGroups&quot;:[],&quot;defaultAction&quot;:{&quot;type&quot;:&quot;ALLOW&quot;},&quot;overrideCustomerWebACLAssociation&quot;:false,&quot;loggingConfiguration&quot;:{&quot;logDestinationConfigs&quot;:[&quot;arn:aws:firehose:us-west-2:12345678912:deliverystream/aws-waf-logs-fms-admin-destination&quot;],&quot;redactedFields&quot;:[{&quot;redactedFieldType&quot;:&quot;SingleHeader&quot;,&quot;redactedFieldValue&quot;:&quot;Cookies&quot;},{&quot;redactedFieldType&quot;:&quot;Method&quot;}]}}&quot;</code> </p> <p>In the <code>loggingConfiguration</code>, you can specify one <code>logDestinationConfigs</code>, you can optionally provide up to 20 <code>redactedFields</code>, and the <code>RedactedFieldType</code> must be one of <code>URI</code>, <code>QUERY</em>STRING</code>, <code>HEADER</code>, or <code>METHOD</code>.</p> </li> <li> <p>Example: <code>WAF Classic</code> </p> <p> <code>&quot;{&quot;type&quot;: &quot;WAF&quot;, &quot;ruleGroups&quot;: [{&quot;id&quot;:&quot;12345678-1bcd-9012-efga-0987654321ab&quot;, &quot;overrideAction&quot; : {&quot;type&quot;: &quot;COUNT&quot;}}], &quot;defaultAction&quot;: {&quot;type&quot;: &quot;BLOCK&quot;}}&quot;</code> </p> </li> <li> <p>Example: <code>SECURITY<em>GROUPS</em>COMMON</code> </p> <p> <code>&quot;{&quot;type&quot;:&quot;SECURITY<em>GROUPS</em>COMMON&quot;,&quot;revertManualSecurityGroupChanges&quot;:false,&quot;exclusiveResourceSecurityGroupManagement&quot;:false, &quot;applyToAllEC2InstanceENIs&quot;:false,&quot;securityGroups&quot;:[{&quot;id&quot;:&quot; sg-000e55995d61a06bd&quot;}]}&quot;</code> </p> </li> <li> <p>Example: <code>SECURITY<em>GROUPS</em>CONTENT<em>AUDIT</code> </p> <p> <code>&quot;{&quot;type&quot;:&quot;SECURITY</em>GROUPS<em>CONTENT</em>AUDIT&quot;,&quot;securityGroups&quot;:[{&quot;id&quot;:&quot;sg-000e55995d61a06bd&quot;}],&quot;securityGroupAction&quot;:{&quot;type&quot;:&quot;ALLOW&quot;}}&quot;</code> </p> <p>The security group action for content audit can be <code>ALLOW</code> or <code>DENY</code>. For <code>ALLOW</code>, all in-scope security group rules must be within the allowed range of the policy&#39;s security group rules. For <code>DENY</code>, all in-scope security group rules must not contain a value or a range that matches a rule value or range in the policy security group.</p> </li> <li> <p>Example: <code>SECURITY<em>GROUPS</em>USAGE<em>AUDIT</code> </p> <p> <code>&quot;{&quot;type&quot;:&quot;SECURITY</em>GROUPS<em>USAGE</em>AUDIT&quot;,&quot;deleteUnusedSecurityGroups&quot;:true,&quot;coalesceRedundantSecurityGroups&quot;:true}&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "ManagedServiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_service_data: Option<String>,
    /// <p>The service that the policy is using to protect the resources. This specifies the type of policy that is created, either an AWS WAF policy, a Shield Advanced policy, or a security group policy. For security group policies, Firewall Manager supports one security group for each common policy and for each content audit policy. This is an adjustable limit that you can increase by contacting AWS Support.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>AWS Network Firewall stateful rule group, used in a <a>NetworkFirewallPolicyDescription</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StatefulRuleGroup {
    /// <p>The resource ID of the rule group.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The name of the rule group.</p>
    #[serde(rename = "RuleGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
}

/// <p>AWS Network Firewall stateless rule group, used in a <a>NetworkFirewallPolicyDescription</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StatelessRuleGroup {
    /// <p>The priority of the rule group. AWS Network Firewall evaluates the stateless rule groups in a firewall policy starting from the lowest priority setting. </p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The resource ID of the rule group.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The name of the rule group.</p>
    #[serde(rename = "RuleGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
}

/// <p>A collection of key:value pairs associated with an AWS resource. The key:value pair can be anything you define. Typically, the tag key represents a category (such as "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 tags to each AWS resource. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>Part of the key:value pair that defines a tag. You can use a tag key to describe a category of information, such as "customer." Tag keys are case-sensitive.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Part of the key:value pair that defines a tag. You can use a tag value to describe a specific value within a category, such as "companyA" or "companyB." Tag values are case-sensitive. </p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to return tags for. The AWS Firewall Manager resources that support tagging are policies, applications lists, and protocols lists. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to return tags for. The AWS Firewall Manager resources that support tagging are policies, applications lists, and protocols lists. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to remove from the resource. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>Violations for a resource based on the specified AWS Firewall Manager policy and AWS account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ViolationDetail {
    /// <p>The AWS account that the violation details were requested for.</p>
    #[serde(rename = "MemberAccount")]
    pub member_account: String,
    /// <p>The ID of the AWS Firewall Manager policy that the violation details were requested for.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p>Brief description for the requested resource.</p>
    #[serde(rename = "ResourceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_description: Option<String>,
    /// <p>The resource ID that the violation details were requested for.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The <code>ResourceTag</code> objects associated with the resource.</p>
    #[serde(rename = "ResourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<Tag>>,
    /// <p>The resource type that the violation details were requested for.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>List of violations for the requested resource.</p>
    #[serde(rename = "ResourceViolations")]
    pub resource_violations: Vec<ResourceViolation>,
}

/// Errors returned by AssociateAdminAccount
#[derive(Debug, PartialEq)]
pub enum AssociateAdminAccountError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateAdminAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateAdminAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AssociateAdminAccountError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AssociateAdminAccountError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(AssociateAdminAccountError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateAdminAccountError::ResourceNotFound(
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
impl fmt::Display for AssociateAdminAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateAdminAccountError::InternalError(ref cause) => write!(f, "{}", cause),
            AssociateAdminAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AssociateAdminAccountError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            AssociateAdminAccountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateAdminAccountError {}
/// Errors returned by DeleteAppsList
#[derive(Debug, PartialEq)]
pub enum DeleteAppsListError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteAppsListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppsListError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteAppsListError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DeleteAppsListError::InvalidOperation(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAppsListError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppsListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppsListError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteAppsListError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DeleteAppsListError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAppsListError {}
/// Errors returned by DeleteNotificationChannel
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationChannelError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteNotificationChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNotificationChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteNotificationChannelError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DeleteNotificationChannelError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteNotificationChannelError::ResourceNotFound(
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
impl fmt::Display for DeleteNotificationChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNotificationChannelError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteNotificationChannelError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DeleteNotificationChannelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNotificationChannelError {}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeletePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeletePolicyError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeletePolicyError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DeletePolicyError::InvalidOperation(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeletePolicyError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePolicyError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePolicyError::InternalError(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePolicyError {}
/// Errors returned by DeleteProtocolsList
#[derive(Debug, PartialEq)]
pub enum DeleteProtocolsListError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteProtocolsListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProtocolsListError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteProtocolsListError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DeleteProtocolsListError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProtocolsListError::ResourceNotFound(
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
impl fmt::Display for DeleteProtocolsListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProtocolsListError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteProtocolsListError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DeleteProtocolsListError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProtocolsListError {}
/// Errors returned by DisassociateAdminAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateAdminAccountError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateAdminAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateAdminAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DisassociateAdminAccountError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DisassociateAdminAccountError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateAdminAccountError::ResourceNotFound(
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
impl fmt::Display for DisassociateAdminAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateAdminAccountError::InternalError(ref cause) => write!(f, "{}", cause),
            DisassociateAdminAccountError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            DisassociateAdminAccountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateAdminAccountError {}
/// Errors returned by GetAdminAccount
#[derive(Debug, PartialEq)]
pub enum GetAdminAccountError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetAdminAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAdminAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetAdminAccountError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(GetAdminAccountError::InvalidOperation(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAdminAccountError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAdminAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAdminAccountError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAdminAccountError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            GetAdminAccountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAdminAccountError {}
/// Errors returned by GetAppsList
#[derive(Debug, PartialEq)]
pub enum GetAppsListError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetAppsListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppsListError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetAppsListError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(GetAppsListError::InvalidOperation(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAppsListError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppsListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppsListError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAppsListError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            GetAppsListError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAppsListError {}
/// Errors returned by GetComplianceDetail
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetComplianceDetailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetComplianceDetailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetComplianceDetailError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetComplianceDetailError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(GetComplianceDetailError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetComplianceDetailError::ResourceNotFound(
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
impl fmt::Display for GetComplianceDetailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComplianceDetailError::InternalError(ref cause) => write!(f, "{}", cause),
            GetComplianceDetailError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetComplianceDetailError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            GetComplianceDetailError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetComplianceDetailError {}
/// Errors returned by GetNotificationChannel
#[derive(Debug, PartialEq)]
pub enum GetNotificationChannelError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetNotificationChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNotificationChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetNotificationChannelError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(GetNotificationChannelError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetNotificationChannelError::ResourceNotFound(
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
impl fmt::Display for GetNotificationChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNotificationChannelError::InternalError(ref cause) => write!(f, "{}", cause),
            GetNotificationChannelError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            GetNotificationChannelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNotificationChannelError {}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The value of the <code>Type</code> parameter is invalid.</p>
    InvalidType(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetPolicyError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(GetPolicyError::InvalidOperation(err.msg))
                }
                "InvalidTypeException" => {
                    return RusotoError::Service(GetPolicyError::InvalidType(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPolicyError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPolicyError::InternalError(ref cause) => write!(f, "{}", cause),
            GetPolicyError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            GetPolicyError::InvalidType(ref cause) => write!(f, "{}", cause),
            GetPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPolicyError {}
/// Errors returned by GetProtectionStatus
#[derive(Debug, PartialEq)]
pub enum GetProtectionStatusError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetProtectionStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProtectionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetProtectionStatusError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetProtectionStatusError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetProtectionStatusError::ResourceNotFound(
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
impl fmt::Display for GetProtectionStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProtectionStatusError::InternalError(ref cause) => write!(f, "{}", cause),
            GetProtectionStatusError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetProtectionStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProtectionStatusError {}
/// Errors returned by GetProtocolsList
#[derive(Debug, PartialEq)]
pub enum GetProtocolsListError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetProtocolsListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProtocolsListError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetProtocolsListError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(GetProtocolsListError::InvalidOperation(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetProtocolsListError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProtocolsListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProtocolsListError::InternalError(ref cause) => write!(f, "{}", cause),
            GetProtocolsListError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            GetProtocolsListError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProtocolsListError {}
/// Errors returned by GetViolationDetails
#[derive(Debug, PartialEq)]
pub enum GetViolationDetailsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetViolationDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetViolationDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetViolationDetailsError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetViolationDetailsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetViolationDetailsError::ResourceNotFound(
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
impl fmt::Display for GetViolationDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetViolationDetailsError::InternalError(ref cause) => write!(f, "{}", cause),
            GetViolationDetailsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetViolationDetailsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetViolationDetailsError {}
/// Errors returned by ListAppsLists
#[derive(Debug, PartialEq)]
pub enum ListAppsListsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListAppsListsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAppsListsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListAppsListsError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(ListAppsListsError::InvalidOperation(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListAppsListsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAppsListsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAppsListsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAppsListsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListAppsListsError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            ListAppsListsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListAppsListsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAppsListsError {}
/// Errors returned by ListComplianceStatus
#[derive(Debug, PartialEq)]
pub enum ListComplianceStatusError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListComplianceStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListComplianceStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListComplianceStatusError::InternalError(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListComplianceStatusError::ResourceNotFound(
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
impl fmt::Display for ListComplianceStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListComplianceStatusError::InternalError(ref cause) => write!(f, "{}", cause),
            ListComplianceStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListComplianceStatusError {}
/// Errors returned by ListMemberAccounts
#[derive(Debug, PartialEq)]
pub enum ListMemberAccountsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListMemberAccountsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMemberAccountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListMemberAccountsError::InternalError(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListMemberAccountsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMemberAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMemberAccountsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListMemberAccountsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMemberAccountsError {}
/// Errors returned by ListPolicies
#[derive(Debug, PartialEq)]
pub enum ListPoliciesError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListPoliciesError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(ListPoliciesError::InvalidOperation(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListPoliciesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPoliciesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPoliciesError::InternalError(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPoliciesError {}
/// Errors returned by ListProtocolsLists
#[derive(Debug, PartialEq)]
pub enum ListProtocolsListsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListProtocolsListsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProtocolsListsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListProtocolsListsError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(ListProtocolsListsError::InvalidOperation(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProtocolsListsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProtocolsListsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProtocolsListsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListProtocolsListsError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            ListProtocolsListsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProtocolsListsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
            ListTagsForResourceError::InternalError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutAppsList
#[derive(Debug, PartialEq)]
pub enum PutAppsListError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl PutAppsListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAppsListError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(PutAppsListError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutAppsListError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(PutAppsListError::InvalidOperation(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutAppsListError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutAppsListError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAppsListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAppsListError::InternalError(ref cause) => write!(f, "{}", cause),
            PutAppsListError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PutAppsListError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            PutAppsListError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutAppsListError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAppsListError {}
/// Errors returned by PutNotificationChannel
#[derive(Debug, PartialEq)]
pub enum PutNotificationChannelError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl PutNotificationChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutNotificationChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(PutNotificationChannelError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(PutNotificationChannelError::InvalidOperation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutNotificationChannelError::ResourceNotFound(
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
impl fmt::Display for PutNotificationChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutNotificationChannelError::InternalError(ref cause) => write!(f, "{}", cause),
            PutNotificationChannelError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            PutNotificationChannelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutNotificationChannelError {}
/// Errors returned by PutPolicy
#[derive(Debug, PartialEq)]
pub enum PutPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The value of the <code>Type</code> parameter is invalid.</p>
    InvalidType(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl PutPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(PutPolicyError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutPolicyError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(PutPolicyError::InvalidOperation(err.msg))
                }
                "InvalidTypeException" => {
                    return RusotoError::Service(PutPolicyError::InvalidType(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutPolicyError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutPolicyError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPolicyError::InternalError(ref cause) => write!(f, "{}", cause),
            PutPolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PutPolicyError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            PutPolicyError::InvalidType(ref cause) => write!(f, "{}", cause),
            PutPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPolicyError {}
/// Errors returned by PutProtocolsList
#[derive(Debug, PartialEq)]
pub enum PutProtocolsListError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl PutProtocolsListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutProtocolsListError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(PutProtocolsListError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutProtocolsListError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(PutProtocolsListError::InvalidOperation(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutProtocolsListError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutProtocolsListError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutProtocolsListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutProtocolsListError::InternalError(ref cause) => write!(f, "{}", cause),
            PutProtocolsListError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PutProtocolsListError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            PutProtocolsListError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutProtocolsListError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutProtocolsListError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(TagResourceError::InvalidOperation(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::InternalError(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do or the operation wasn't possible. For example, you might have submitted an <code>AssociateAdminAccount</code> request for an account ID that was already set as the AWS Firewall Manager administrator. Or you might have tried to access a Region that's disabled by default, and that you need to enable for the Firewall Manager administrator account and for AWS Organizations before you can access it.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(UntagResourceError::InvalidOperation(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::InternalError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the FMS API. FMS clients implement this trait.
#[async_trait]
pub trait Fms {
    /// <p>Sets the AWS Firewall Manager administrator account. AWS Firewall Manager must be associated with the master account of your AWS organization or associated with a member account that has the appropriate permissions. If the account ID that you submit is not an AWS Organizations master account, AWS Firewall Manager will set the appropriate permissions for the given member account.</p> <p>The account that you associate with AWS Firewall Manager is called the AWS Firewall Manager administrator account. </p>
    async fn associate_admin_account(
        &self,
        input: AssociateAdminAccountRequest,
    ) -> Result<(), RusotoError<AssociateAdminAccountError>>;

    /// <p>Permanently deletes an AWS Firewall Manager applications list.</p>
    async fn delete_apps_list(
        &self,
        input: DeleteAppsListRequest,
    ) -> Result<(), RusotoError<DeleteAppsListError>>;

    /// <p>Deletes an AWS Firewall Manager association with the IAM role and the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    async fn delete_notification_channel(
        &self,
    ) -> Result<(), RusotoError<DeleteNotificationChannelError>>;

    /// <p>Permanently deletes an AWS Firewall Manager policy. </p>
    async fn delete_policy(
        &self,
        input: DeletePolicyRequest,
    ) -> Result<(), RusotoError<DeletePolicyError>>;

    /// <p>Permanently deletes an AWS Firewall Manager protocols list.</p>
    async fn delete_protocols_list(
        &self,
        input: DeleteProtocolsListRequest,
    ) -> Result<(), RusotoError<DeleteProtocolsListError>>;

    /// <p>Disassociates the account that has been set as the AWS Firewall Manager administrator account. To set a different account as the administrator account, you must submit an <code>AssociateAdminAccount</code> request.</p>
    async fn disassociate_admin_account(
        &self,
    ) -> Result<(), RusotoError<DisassociateAdminAccountError>>;

    /// <p>Returns the AWS Organizations master account that is associated with AWS Firewall Manager as the AWS Firewall Manager administrator.</p>
    async fn get_admin_account(
        &self,
    ) -> Result<GetAdminAccountResponse, RusotoError<GetAdminAccountError>>;

    /// <p>Returns information about the specified AWS Firewall Manager applications list.</p>
    async fn get_apps_list(
        &self,
        input: GetAppsListRequest,
    ) -> Result<GetAppsListResponse, RusotoError<GetAppsListError>>;

    /// <p>Returns detailed compliance information about the specified member account. Details include resources that are in and out of compliance with the specified policy. Resources are considered noncompliant for AWS WAF and Shield Advanced policies if the specified policy has not been applied to them. Resources are considered noncompliant for security group policies if they are in scope of the policy, they violate one or more of the policy rules, and remediation is disabled or not possible. Resources are considered noncompliant for Network Firewall policies if a firewall is missing in the VPC, if the firewall endpoint isn't set up in an expected Availability Zone and subnet, if a subnet created by the Firewall Manager doesn't have the expected route table, and for modifications to a firewall policy that violate the Firewall Manager policy's rules. </p>
    async fn get_compliance_detail(
        &self,
        input: GetComplianceDetailRequest,
    ) -> Result<GetComplianceDetailResponse, RusotoError<GetComplianceDetailError>>;

    /// <p>Information about the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    async fn get_notification_channel(
        &self,
    ) -> Result<GetNotificationChannelResponse, RusotoError<GetNotificationChannelError>>;

    /// <p>Returns information about the specified AWS Firewall Manager policy.</p>
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>>;

    /// <p>If you created a Shield Advanced policy, returns policy-level attack summary information in the event of a potential DDoS attack. Other policy types are currently unsupported.</p>
    async fn get_protection_status(
        &self,
        input: GetProtectionStatusRequest,
    ) -> Result<GetProtectionStatusResponse, RusotoError<GetProtectionStatusError>>;

    /// <p>Returns information about the specified AWS Firewall Manager protocols list.</p>
    async fn get_protocols_list(
        &self,
        input: GetProtocolsListRequest,
    ) -> Result<GetProtocolsListResponse, RusotoError<GetProtocolsListError>>;

    /// <p>Retrieves violations for a resource based on the specified AWS Firewall Manager policy and AWS account.</p>
    async fn get_violation_details(
        &self,
        input: GetViolationDetailsRequest,
    ) -> Result<GetViolationDetailsResponse, RusotoError<GetViolationDetailsError>>;

    /// <p>Returns an array of <code>AppsListDataSummary</code> objects.</p>
    async fn list_apps_lists(
        &self,
        input: ListAppsListsRequest,
    ) -> Result<ListAppsListsResponse, RusotoError<ListAppsListsError>>;

    /// <p>Returns an array of <code>PolicyComplianceStatus</code> objects. Use <code>PolicyComplianceStatus</code> to get a summary of which member accounts are protected by the specified policy. </p>
    async fn list_compliance_status(
        &self,
        input: ListComplianceStatusRequest,
    ) -> Result<ListComplianceStatusResponse, RusotoError<ListComplianceStatusError>>;

    /// <p>Returns a <code>MemberAccounts</code> object that lists the member accounts in the administrator's AWS organization.</p> <p>The <code>ListMemberAccounts</code> must be submitted by the account that is set as the AWS Firewall Manager administrator.</p>
    async fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> Result<ListMemberAccountsResponse, RusotoError<ListMemberAccountsError>>;

    /// <p>Returns an array of <code>PolicySummary</code> objects.</p>
    async fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, RusotoError<ListPoliciesError>>;

    /// <p>Returns an array of <code>ProtocolsListDataSummary</code> objects.</p>
    async fn list_protocols_lists(
        &self,
        input: ListProtocolsListsRequest,
    ) -> Result<ListProtocolsListsResponse, RusotoError<ListProtocolsListsError>>;

    /// <p>Retrieves the list of tags for the specified AWS resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Creates an AWS Firewall Manager applications list.</p>
    async fn put_apps_list(
        &self,
        input: PutAppsListRequest,
    ) -> Result<PutAppsListResponse, RusotoError<PutAppsListError>>;

    /// <p>Designates the IAM role and Amazon Simple Notification Service (SNS) topic that AWS Firewall Manager uses to record SNS logs.</p> <p>To perform this action outside of the console, you must configure the SNS topic to allow the Firewall Manager role <code>AWSServiceRoleForFMS</code> to publish SNS logs. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-api-permissions-ref.html">Firewall Manager required permissions for API actions</a> in the <i>AWS Firewall Manager Developer Guide</i>.</p>
    async fn put_notification_channel(
        &self,
        input: PutNotificationChannelRequest,
    ) -> Result<(), RusotoError<PutNotificationChannelError>>;

    /// <p>Creates an AWS Firewall Manager policy.</p> <p>Firewall Manager provides the following types of policies: </p> <ul> <li> <p>An AWS WAF policy (type WAFV2), which defines rule groups to run first in the corresponding AWS WAF web ACL and rule groups to run last in the web ACL.</p> </li> <li> <p>An AWS WAF Classic policy (type WAF), which defines a rule group. </p> </li> <li> <p>A Shield Advanced policy, which applies Shield Advanced protection to specified accounts and resources.</p> </li> <li> <p>A security group policy, which manages VPC security groups across your AWS organization. </p> </li> <li> <p>An AWS Network Firewall policy, which provides firewall rules to filter network traffic in specified Amazon VPCs.</p> </li> </ul> <p>Each policy is specific to one of the types. If you want to enforce more than one policy type across accounts, create multiple policies. You can create multiple policies for each type.</p> <p>You must be subscribed to Shield Advanced to create a Shield Advanced policy. For more information about subscribing to Shield Advanced, see <a href="https://docs.aws.amazon.com/waf/latest/DDOSAPIReference/API_CreateSubscription.html">CreateSubscription</a>.</p>
    async fn put_policy(
        &self,
        input: PutPolicyRequest,
    ) -> Result<PutPolicyResponse, RusotoError<PutPolicyError>>;

    /// <p>Creates an AWS Firewall Manager protocols list.</p>
    async fn put_protocols_list(
        &self,
        input: PutProtocolsListRequest,
    ) -> Result<PutProtocolsListResponse, RusotoError<PutProtocolsListError>>;

    /// <p>Adds one or more tags to an AWS resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from an AWS resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the FMS API.
#[derive(Clone)]
pub struct FmsClient {
    client: Client,
    region: region::Region,
}

impl FmsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> FmsClient {
        FmsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> FmsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        FmsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> FmsClient {
        FmsClient { client, region }
    }
}

#[async_trait]
impl Fms for FmsClient {
    /// <p>Sets the AWS Firewall Manager administrator account. AWS Firewall Manager must be associated with the master account of your AWS organization or associated with a member account that has the appropriate permissions. If the account ID that you submit is not an AWS Organizations master account, AWS Firewall Manager will set the appropriate permissions for the given member account.</p> <p>The account that you associate with AWS Firewall Manager is called the AWS Firewall Manager administrator account. </p>
    async fn associate_admin_account(
        &self,
        input: AssociateAdminAccountRequest,
    ) -> Result<(), RusotoError<AssociateAdminAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.AssociateAdminAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateAdminAccountError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Permanently deletes an AWS Firewall Manager applications list.</p>
    async fn delete_apps_list(
        &self,
        input: DeleteAppsListRequest,
    ) -> Result<(), RusotoError<DeleteAppsListError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.DeleteAppsList");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAppsListError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes an AWS Firewall Manager association with the IAM role and the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    async fn delete_notification_channel(
        &self,
    ) -> Result<(), RusotoError<DeleteNotificationChannelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.DeleteNotificationChannel");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DeleteNotificationChannelError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Permanently deletes an AWS Firewall Manager policy. </p>
    async fn delete_policy(
        &self,
        input: DeletePolicyRequest,
    ) -> Result<(), RusotoError<DeletePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.DeletePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeletePolicyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Permanently deletes an AWS Firewall Manager protocols list.</p>
    async fn delete_protocols_list(
        &self,
        input: DeleteProtocolsListRequest,
    ) -> Result<(), RusotoError<DeleteProtocolsListError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.DeleteProtocolsList");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteProtocolsListError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Disassociates the account that has been set as the AWS Firewall Manager administrator account. To set a different account as the administrator account, you must submit an <code>AssociateAdminAccount</code> request.</p>
    async fn disassociate_admin_account(
        &self,
    ) -> Result<(), RusotoError<DisassociateAdminAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.DisassociateAdminAccount");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DisassociateAdminAccountError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Returns the AWS Organizations master account that is associated with AWS Firewall Manager as the AWS Firewall Manager administrator.</p>
    async fn get_admin_account(
        &self,
    ) -> Result<GetAdminAccountResponse, RusotoError<GetAdminAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetAdminAccount");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetAdminAccountError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetAdminAccountResponse, _>()
    }

    /// <p>Returns information about the specified AWS Firewall Manager applications list.</p>
    async fn get_apps_list(
        &self,
        input: GetAppsListRequest,
    ) -> Result<GetAppsListResponse, RusotoError<GetAppsListError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetAppsList");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAppsListError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetAppsListResponse, _>()
    }

    /// <p>Returns detailed compliance information about the specified member account. Details include resources that are in and out of compliance with the specified policy. Resources are considered noncompliant for AWS WAF and Shield Advanced policies if the specified policy has not been applied to them. Resources are considered noncompliant for security group policies if they are in scope of the policy, they violate one or more of the policy rules, and remediation is disabled or not possible. Resources are considered noncompliant for Network Firewall policies if a firewall is missing in the VPC, if the firewall endpoint isn't set up in an expected Availability Zone and subnet, if a subnet created by the Firewall Manager doesn't have the expected route table, and for modifications to a firewall policy that violate the Firewall Manager policy's rules. </p>
    async fn get_compliance_detail(
        &self,
        input: GetComplianceDetailRequest,
    ) -> Result<GetComplianceDetailResponse, RusotoError<GetComplianceDetailError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetComplianceDetail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetComplianceDetailError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetComplianceDetailResponse, _>()
    }

    /// <p>Information about the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    async fn get_notification_channel(
        &self,
    ) -> Result<GetNotificationChannelResponse, RusotoError<GetNotificationChannelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetNotificationChannel");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetNotificationChannelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetNotificationChannelResponse, _>()
    }

    /// <p>Returns information about the specified AWS Firewall Manager policy.</p>
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetPolicyResponse, _>()
    }

    /// <p>If you created a Shield Advanced policy, returns policy-level attack summary information in the event of a potential DDoS attack. Other policy types are currently unsupported.</p>
    async fn get_protection_status(
        &self,
        input: GetProtectionStatusRequest,
    ) -> Result<GetProtectionStatusResponse, RusotoError<GetProtectionStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetProtectionStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetProtectionStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetProtectionStatusResponse, _>()
    }

    /// <p>Returns information about the specified AWS Firewall Manager protocols list.</p>
    async fn get_protocols_list(
        &self,
        input: GetProtocolsListRequest,
    ) -> Result<GetProtocolsListResponse, RusotoError<GetProtocolsListError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetProtocolsList");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetProtocolsListError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetProtocolsListResponse, _>()
    }

    /// <p>Retrieves violations for a resource based on the specified AWS Firewall Manager policy and AWS account.</p>
    async fn get_violation_details(
        &self,
        input: GetViolationDetailsRequest,
    ) -> Result<GetViolationDetailsResponse, RusotoError<GetViolationDetailsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.GetViolationDetails");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetViolationDetailsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetViolationDetailsResponse, _>()
    }

    /// <p>Returns an array of <code>AppsListDataSummary</code> objects.</p>
    async fn list_apps_lists(
        &self,
        input: ListAppsListsRequest,
    ) -> Result<ListAppsListsResponse, RusotoError<ListAppsListsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.ListAppsLists");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAppsListsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAppsListsResponse, _>()
    }

    /// <p>Returns an array of <code>PolicyComplianceStatus</code> objects. Use <code>PolicyComplianceStatus</code> to get a summary of which member accounts are protected by the specified policy. </p>
    async fn list_compliance_status(
        &self,
        input: ListComplianceStatusRequest,
    ) -> Result<ListComplianceStatusResponse, RusotoError<ListComplianceStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.ListComplianceStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListComplianceStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListComplianceStatusResponse, _>()
    }

    /// <p>Returns a <code>MemberAccounts</code> object that lists the member accounts in the administrator's AWS organization.</p> <p>The <code>ListMemberAccounts</code> must be submitted by the account that is set as the AWS Firewall Manager administrator.</p>
    async fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> Result<ListMemberAccountsResponse, RusotoError<ListMemberAccountsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.ListMemberAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListMemberAccountsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListMemberAccountsResponse, _>()
    }

    /// <p>Returns an array of <code>PolicySummary</code> objects.</p>
    async fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, RusotoError<ListPoliciesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.ListPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPoliciesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListPoliciesResponse, _>()
    }

    /// <p>Returns an array of <code>ProtocolsListDataSummary</code> objects.</p>
    async fn list_protocols_lists(
        &self,
        input: ListProtocolsListsRequest,
    ) -> Result<ListProtocolsListsResponse, RusotoError<ListProtocolsListsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.ListProtocolsLists");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListProtocolsListsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListProtocolsListsResponse, _>()
    }

    /// <p>Retrieves the list of tags for the specified AWS resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Creates an AWS Firewall Manager applications list.</p>
    async fn put_apps_list(
        &self,
        input: PutAppsListRequest,
    ) -> Result<PutAppsListResponse, RusotoError<PutAppsListError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.PutAppsList");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAppsListError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutAppsListResponse, _>()
    }

    /// <p>Designates the IAM role and Amazon Simple Notification Service (SNS) topic that AWS Firewall Manager uses to record SNS logs.</p> <p>To perform this action outside of the console, you must configure the SNS topic to allow the Firewall Manager role <code>AWSServiceRoleForFMS</code> to publish SNS logs. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-api-permissions-ref.html">Firewall Manager required permissions for API actions</a> in the <i>AWS Firewall Manager Developer Guide</i>.</p>
    async fn put_notification_channel(
        &self,
        input: PutNotificationChannelRequest,
    ) -> Result<(), RusotoError<PutNotificationChannelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.PutNotificationChannel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutNotificationChannelError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Creates an AWS Firewall Manager policy.</p> <p>Firewall Manager provides the following types of policies: </p> <ul> <li> <p>An AWS WAF policy (type WAFV2), which defines rule groups to run first in the corresponding AWS WAF web ACL and rule groups to run last in the web ACL.</p> </li> <li> <p>An AWS WAF Classic policy (type WAF), which defines a rule group. </p> </li> <li> <p>A Shield Advanced policy, which applies Shield Advanced protection to specified accounts and resources.</p> </li> <li> <p>A security group policy, which manages VPC security groups across your AWS organization. </p> </li> <li> <p>An AWS Network Firewall policy, which provides firewall rules to filter network traffic in specified Amazon VPCs.</p> </li> </ul> <p>Each policy is specific to one of the types. If you want to enforce more than one policy type across accounts, create multiple policies. You can create multiple policies for each type.</p> <p>You must be subscribed to Shield Advanced to create a Shield Advanced policy. For more information about subscribing to Shield Advanced, see <a href="https://docs.aws.amazon.com/waf/latest/DDOSAPIReference/API_CreateSubscription.html">CreateSubscription</a>.</p>
    async fn put_policy(
        &self,
        input: PutPolicyRequest,
    ) -> Result<PutPolicyResponse, RusotoError<PutPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.PutPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutPolicyResponse, _>()
    }

    /// <p>Creates an AWS Firewall Manager protocols list.</p>
    async fn put_protocols_list(
        &self,
        input: PutProtocolsListRequest,
    ) -> Result<PutProtocolsListResponse, RusotoError<PutProtocolsListError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.PutProtocolsList");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutProtocolsListError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutProtocolsListResponse, _>()
    }

    /// <p>Adds one or more tags to an AWS resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes one or more tags from an AWS resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSFMS_20180101.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }
}
