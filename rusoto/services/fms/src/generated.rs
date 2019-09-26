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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateAdminAccountRequest {
    /// <p>The AWS account ID to associate with AWS Firewall Manager as the AWS Firewall Manager administrator account. This can be an AWS Organizations master account or a member account. For more information about AWS Organizations and master accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts.html">Managing the AWS Accounts in Your Organization</a>.</p>
    #[serde(rename = "AdminAccount")]
    pub admin_account: String,
}

/// <p>Details of the resource that is not protected by the policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComplianceViolator {
    /// <p>The resource ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The resource type. This is in the format shown in <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. For example: <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> or <code>AWS::CloudFront::Distribution</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The reason that the resource is not protected by the policy.</p>
    #[serde(rename = "ViolationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNotificationChannelRequest {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePolicyRequest {
    /// <p>If <code>True</code>, the request will also perform a clean-up process that will:</p> <ul> <li> <p>Delete rule groups created by AWS Firewall Manager</p> </li> <li> <p>Remove web ACLs from in-scope resources</p> </li> <li> <p>Delete web ACLs that contain no rules or rule groups</p> </li> </ul> <p>After the cleanup, in-scope resources will no longer be protected by web ACLs in this policy. Protection of out-of-scope resources will remain unchanged. Scope is determined by tags and accounts associated with the policy. When creating the policy, if you specified that only resources in specific accounts or with specific tags be protected by the policy, those resources are in-scope. All others are out of scope. If you did not specify tags or accounts, all resources are in-scope. </p>
    #[serde(rename = "DeleteAllPolicyResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_all_policy_resources: Option<bool>,
    /// <p>The ID of the policy that you want to delete. <code>PolicyId</code> is returned by <code>PutPolicy</code> and by <code>ListPolicies</code>.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateAdminAccountRequest {}

/// <p>Describes the compliance status for the account. An account is considered non-compliant if it includes resources that are not protected by the specified policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EvaluationResult {
    /// <p>Describes an AWS account's compliance with the AWS Firewall Manager policy.</p>
    #[serde(rename = "ComplianceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    /// <p>Indicates that over 100 resources are non-compliant with the AWS Firewall Manager policy.</p>
    #[serde(rename = "EvaluationLimitExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_limit_exceeded: Option<bool>,
    /// <p>Number of resources that are non-compliant with the specified policy. A resource is considered non-compliant if it is not associated with the specified policy.</p>
    #[serde(rename = "ViolatorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violator_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAdminAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetComplianceDetailRequest {
    /// <p>The AWS account that owns the resources that you want to get the details for.</p>
    #[serde(rename = "MemberAccount")]
    pub member_account: String,
    /// <p>The ID of the policy that you want to get the details for. <code>PolicyId</code> is returned by <code>PutPolicy</code> and by <code>ListPolicies</code>.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetComplianceDetailResponse {
    /// <p>Information about the resources and the policy that you specified in the <code>GetComplianceDetail</code> request.</p>
    #[serde(rename = "PolicyComplianceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_compliance_detail: Option<PolicyComplianceDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetNotificationChannelRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPolicyRequest {
    /// <p>The ID of the AWS Firewall Manager policy that you want the details for.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetProtectionStatusRequest {
    /// <p>The end of the time period to query for the attacks. This is a <code>timestamp</code> type. The sample request above indicates a number type because the default used by AWS Firewall Manager is Unix time in seconds. However, any valid <code>timestamp</code> format is allowed.</p>
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
    /// <p>If you specify a value for <code>MaxResults</code> and you have more objects than the number that you specify for <code>MaxResults</code>, AWS Firewall Manager returns a <code>NextToken</code> value in the response that allows you to list another group of objects. For the second and subsequent <code>GetProtectionStatus</code> requests, specify the value of <code>NextToken</code> from the previous response to get information about another batch of objects.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the policy for which you want to get the attack information.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p>The start of the time period to query for the attacks. This is a <code>timestamp</code> type. The sample request above indicates a number type because the default used by AWS Firewall Manager is Unix time in seconds. However, any valid <code>timestamp</code> format is allowed.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetProtectionStatusResponse {
    /// <p>The ID of the AWS Firewall administrator account for this policy.</p>
    #[serde(rename = "AdminAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
    /// <p>Details about the attack, including the following:</p> <ul> <li> <p>Attack type</p> </li> <li> <p>Account ID</p> </li> <li> <p>ARN of the resource attacked</p> </li> <li> <p>Start time of the attack</p> </li> <li> <p>End time of the attack (ongoing attacks will not have an end time)</p> </li> </ul> <p>The details are in JSON format. An example is shown in the Examples section below.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>An AWS Firewall Manager policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Policy {
    /// <p>Specifies the AWS account IDs to exclude from the policy. The <code>IncludeMap</code> values are evaluated first, with all the appropriate account IDs added to the policy. Then the accounts listed in <code>ExcludeMap</code> are removed, resulting in the final list of accounts to add to the policy.</p> <p>The key to the map is <code>ACCOUNT</code>. For example, a valid <code>ExcludeMap</code> would be <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p>
    #[serde(rename = "ExcludeMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_map: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>If set to <code>True</code>, resources with the tags that are specified in the <code>ResourceTag</code> array are not protected by the policy. If set to <code>False</code>, and the <code>ResourceTag</code> array is not null, only resources with the specified tags are associated with the policy.</p>
    #[serde(rename = "ExcludeResourceTags")]
    pub exclude_resource_tags: bool,
    /// <p>Specifies the AWS account IDs to include in the policy. If <code>IncludeMap</code> is null, all accounts in the organization in AWS Organizations are included in the policy. If <code>IncludeMap</code> is not null, only values listed in <code>IncludeMap</code> are included in the policy.</p> <p>The key to the map is <code>ACCOUNT</code>. For example, a valid <code>IncludeMap</code> would be <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p>
    #[serde(rename = "IncludeMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_map: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ID of the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The friendly name of the AWS Firewall Manager policy.</p>
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
    /// <p>The type of resource to protect with the policy. This is in the format shown in <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. For example: <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> or <code>AWS::CloudFront::Distribution</code>.</p>
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

/// <p>Describes the non-compliant resources in a member account for a specific AWS Firewall Manager policy. A maximum of 100 entries are displayed. If more than 100 resources are non-compliant, <code>EvaluationLimitExceeded</code> is set to <code>True</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PolicyComplianceDetail {
    /// <p>Indicates if over 100 resources are non-compliant with the AWS Firewall Manager policy.</p>
    #[serde(rename = "EvaluationLimitExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_limit_exceeded: Option<bool>,
    /// <p>A time stamp that indicates when the returned information should be considered out-of-date.</p>
    #[serde(rename = "ExpiredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<f64>,
    /// <p>Details about problems with dependent services, such as AWS WAF or AWS Config, that are causing a resource to be non-compliant. The details include the name of the dependent service and the error message received that indicates the problem with the service.</p>
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
    /// <p>An array of resources that are not protected by the policy.</p>
    #[serde(rename = "Violators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violators: Option<Vec<ComplianceViolator>>,
}

/// <p>Indicates whether the account is compliant with the specified policy. An account is considered non-compliant if it includes resources that are not protected by the policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PolicyComplianceStatus {
    /// <p>An array of <code>EvaluationResult</code> objects.</p>
    #[serde(rename = "EvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    /// <p>Details about problems with dependent services, such as AWS WAF or AWS Config, that are causing a resource to be non-compliant. The details include the name of the dependent service and the error message received that indicates the problem with the service.</p>
    #[serde(rename = "IssueInfoMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_info_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>Time stamp of the last update to the <code>EvaluationResult</code> objects.</p>
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
    /// <p>The friendly name of the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// <p>The AWS account that created the AWS Firewall Manager policy.</p>
    #[serde(rename = "PolicyOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_owner: Option<String>,
}

/// <p>Details of the AWS Firewall Manager policy. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PolicySummary {
    /// <p>The Amazon Resource Name (ARN) of the specified policy.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The ID of the specified policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The friendly name of the specified policy.</p>
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// <p>Indicates if the policy should be automatically applied to new resources.</p>
    #[serde(rename = "RemediationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_enabled: Option<bool>,
    /// <p>The type of resource to protect with the policy. This is in the format shown in <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. For example: <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> or <code>AWS::CloudFront::Distribution</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The service that the policy is using to protect the resources. This specifies the type of policy that is created, either a WAF policy or Shield Advanced policy.</p>
    #[serde(rename = "SecurityServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_service_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutNotificationChannelRequest {
    /// <p>The Amazon Resource Name (ARN) of the IAM role that allows Amazon SNS to record AWS Firewall Manager activity. </p>
    #[serde(rename = "SnsRoleName")]
    pub sns_role_name: String,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic that collects notifications from AWS Firewall Manager.</p>
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutPolicyRequest {
    /// <p>The details of the AWS Firewall Manager policy to be created.</p>
    #[serde(rename = "Policy")]
    pub policy: Policy,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutPolicyResponse {
    /// <p>The details of the AWS Firewall Manager policy that was created.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
    /// <p>The Amazon Resource Name (ARN) of the policy that was created.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

/// <p>The resource tags that AWS Firewall Manager uses to determine if a particular resource should be included or excluded from protection by the AWS Firewall Manager policy. Tags enable you to categorize your AWS resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. Tags are combined with an "OR." That is, if you add more than one tag, if any of the tags matches, the resource is considered a match for the include or exclude. <a href="https://docs.aws.amazon.com/awsconsolehelpdocs/latest/gsg/tag-editor.html">Working with Tag Editor</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceTag {
    /// <p>The resource tag key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The resource tag value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Details about the security service that is being used to protect the resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityServicePolicyData {
    /// <p>Details about the service. This contains <code>WAF</code> data in JSON format, as shown in the following example:</p> <p> <code>ManagedServiceData": "{\"type\": \"WAF\", \"ruleGroups\": [{\"id\": \"12345678-1bcd-9012-efga-0987654321ab\", \"overrideAction\" : {\"type\": \"COUNT\"}}], \"defaultAction\": {\"type\": \"BLOCK\"}}</code> </p> <p>If this is a Shield Advanced policy, this string will be empty.</p>
    #[serde(rename = "ManagedServiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_service_data: Option<String>,
    /// <p>The service that the policy is using to protect the resources. This specifies the type of policy that is created, either a WAF policy or Shield Advanced policy.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// Errors returned by AssociateAdminAccount
#[derive(Debug, PartialEq)]
pub enum AssociateAdminAccountError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateAdminAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateAdminAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateAdminAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateAdminAccountError {
    fn description(&self) -> &str {
        match *self {
            AssociateAdminAccountError::InternalError(ref cause) => cause,
            AssociateAdminAccountError::InvalidInput(ref cause) => cause,
            AssociateAdminAccountError::InvalidOperation(ref cause) => cause,
            AssociateAdminAccountError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteNotificationChannel
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationChannelError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteNotificationChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNotificationChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteNotificationChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotificationChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteNotificationChannelError::InternalError(ref cause) => cause,
            DeleteNotificationChannelError::InvalidOperation(ref cause) => cause,
            DeleteNotificationChannelError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DeletePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeletePolicyError::InternalError(err.msg))
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DeletePolicyError::InvalidOperation(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePolicyError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeletePolicyError::InternalError(ref cause) => cause,
            DeletePolicyError::InvalidOperation(ref cause) => cause,
            DeletePolicyError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateAdminAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateAdminAccountError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateAdminAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateAdminAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateAdminAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateAdminAccountError {
    fn description(&self) -> &str {
        match *self {
            DisassociateAdminAccountError::InternalError(ref cause) => cause,
            DisassociateAdminAccountError::InvalidOperation(ref cause) => cause,
            DisassociateAdminAccountError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAdminAccount
#[derive(Debug, PartialEq)]
pub enum GetAdminAccountError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetAdminAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAdminAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAdminAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAdminAccountError {
    fn description(&self) -> &str {
        match *self {
            GetAdminAccountError::InternalError(ref cause) => cause,
            GetAdminAccountError::InvalidOperation(ref cause) => cause,
            GetAdminAccountError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceDetail
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetComplianceDetailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetComplianceDetailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetComplianceDetailError::InternalError(err.msg))
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetComplianceDetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceDetailError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceDetailError::InternalError(ref cause) => cause,
            GetComplianceDetailError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetNotificationChannel
#[derive(Debug, PartialEq)]
pub enum GetNotificationChannelError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetNotificationChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNotificationChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetNotificationChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetNotificationChannelError {
    fn description(&self) -> &str {
        match *self {
            GetNotificationChannelError::InternalError(ref cause) => cause,
            GetNotificationChannelError::InvalidOperation(ref cause) => cause,
            GetNotificationChannelError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The value of the <code>Type</code> parameter is invalid.</p>
    InvalidType(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetPolicyError::InternalError(ref cause) => cause,
            GetPolicyError::InvalidOperation(ref cause) => cause,
            GetPolicyError::InvalidType(ref cause) => cause,
            GetPolicyError::ResourceNotFound(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetProtectionStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetProtectionStatusError {
    fn description(&self) -> &str {
        match *self {
            GetProtectionStatusError::InternalError(ref cause) => cause,
            GetProtectionStatusError::InvalidInput(ref cause) => cause,
            GetProtectionStatusError::ResourceNotFound(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListComplianceStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListComplianceStatusError {
    fn description(&self) -> &str {
        match *self {
            ListComplianceStatusError::InternalError(ref cause) => cause,
            ListComplianceStatusError::ResourceNotFound(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListMemberAccountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMemberAccountsError {
    fn description(&self) -> &str {
        match *self {
            ListMemberAccountsError::InternalError(ref cause) => cause,
            ListMemberAccountsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicies
#[derive(Debug, PartialEq)]
pub enum ListPoliciesError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListPoliciesError::InternalError(ref cause) => cause,
            ListPoliciesError::InvalidOperation(ref cause) => cause,
            ListPoliciesError::LimitExceeded(ref cause) => cause,
            ListPoliciesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutNotificationChannel
#[derive(Debug, PartialEq)]
pub enum PutNotificationChannelError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
    InvalidOperation(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl PutNotificationChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutNotificationChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutNotificationChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutNotificationChannelError {
    fn description(&self) -> &str {
        match *self {
            PutNotificationChannelError::InternalError(ref cause) => cause,
            PutNotificationChannelError::InvalidOperation(ref cause) => cause,
            PutNotificationChannelError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutPolicy
#[derive(Debug, PartialEq)]
pub enum PutPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    InternalError(String),
    /// <p>The parameters of the request were invalid.</p>
    InvalidInput(String),
    /// <p>The operation failed because there was nothing to do. For example, you might have submitted an <code>AssociateAdminAccount</code> request, but the account ID that you submitted was already set as the AWS Firewall Manager administrator.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutPolicyError::InternalError(ref cause) => cause,
            PutPolicyError::InvalidInput(ref cause) => cause,
            PutPolicyError::InvalidOperation(ref cause) => cause,
            PutPolicyError::InvalidType(ref cause) => cause,
            PutPolicyError::LimitExceeded(ref cause) => cause,
            PutPolicyError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the FMS API. FMS clients implement this trait.
pub trait Fms {
    /// <p>Sets the AWS Firewall Manager administrator account. AWS Firewall Manager must be associated with the master account your AWS organization or associated with a member account that has the appropriate permissions. If the account ID that you submit is not an AWS Organizations master account, AWS Firewall Manager will set the appropriate permissions for the given member account.</p> <p>The account that you associate with AWS Firewall Manager is called the AWS Firewall Manager administrator account. </p>
    fn associate_admin_account(
        &self,
        input: AssociateAdminAccountRequest,
    ) -> RusotoFuture<(), AssociateAdminAccountError>;

    /// <p>Deletes an AWS Firewall Manager association with the IAM role and the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    fn delete_notification_channel(&self) -> RusotoFuture<(), DeleteNotificationChannelError>;

    /// <p>Permanently deletes an AWS Firewall Manager policy. </p>
    fn delete_policy(&self, input: DeletePolicyRequest) -> RusotoFuture<(), DeletePolicyError>;

    /// <p>Disassociates the account that has been set as the AWS Firewall Manager administrator account. To set a different account as the administrator account, you must submit an <code>AssociateAdminAccount</code> request .</p>
    fn disassociate_admin_account(&self) -> RusotoFuture<(), DisassociateAdminAccountError>;

    /// <p>Returns the AWS Organizations master account that is associated with AWS Firewall Manager as the AWS Firewall Manager administrator.</p>
    fn get_admin_account(&self) -> RusotoFuture<GetAdminAccountResponse, GetAdminAccountError>;

    /// <p>Returns detailed compliance information about the specified member account. Details include resources that are in and out of compliance with the specified policy. Resources are considered non-compliant if the specified policy has not been applied to them.</p>
    fn get_compliance_detail(
        &self,
        input: GetComplianceDetailRequest,
    ) -> RusotoFuture<GetComplianceDetailResponse, GetComplianceDetailError>;

    /// <p>Returns information about the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    fn get_notification_channel(
        &self,
    ) -> RusotoFuture<GetNotificationChannelResponse, GetNotificationChannelError>;

    /// <p>Returns information about the specified AWS Firewall Manager policy.</p>
    fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> RusotoFuture<GetPolicyResponse, GetPolicyError>;

    /// <p>If you created a Shield Advanced policy, returns policy-level attack summary information in the event of a potential DDoS attack.</p>
    fn get_protection_status(
        &self,
        input: GetProtectionStatusRequest,
    ) -> RusotoFuture<GetProtectionStatusResponse, GetProtectionStatusError>;

    /// <p>Returns an array of <code>PolicyComplianceStatus</code> objects in the response. Use <code>PolicyComplianceStatus</code> to get a summary of which member accounts are protected by the specified policy. </p>
    fn list_compliance_status(
        &self,
        input: ListComplianceStatusRequest,
    ) -> RusotoFuture<ListComplianceStatusResponse, ListComplianceStatusError>;

    /// <p>Returns a <code>MemberAccounts</code> object that lists the member accounts in the administrator's AWS organization.</p> <p>The <code>ListMemberAccounts</code> must be submitted by the account that is set as the AWS Firewall Manager administrator.</p>
    fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> RusotoFuture<ListMemberAccountsResponse, ListMemberAccountsError>;

    /// <p>Returns an array of <code>PolicySummary</code> objects in the response.</p>
    fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> RusotoFuture<ListPoliciesResponse, ListPoliciesError>;

    /// <p>Designates the IAM role and Amazon Simple Notification Service (SNS) topic that AWS Firewall Manager uses to record SNS logs.</p>
    fn put_notification_channel(
        &self,
        input: PutNotificationChannelRequest,
    ) -> RusotoFuture<(), PutNotificationChannelError>;

    /// <p>Creates an AWS Firewall Manager policy.</p> <p>Firewall Manager provides two types of policies: A Shield Advanced policy, which applies Shield Advanced protection to specified accounts and resources, or a WAF policy, which contains a rule group and defines which resources are to be protected by that rule group. A policy is specific to either WAF or Shield Advanced. If you want to enforce both WAF rules and Shield Advanced protection across accounts, you can create multiple policies. You can create one or more policies for WAF rules, and one or more policies for Shield Advanced.</p> <p>You must be subscribed to Shield Advanced to create a Shield Advanced policy. For more information on subscribing to Shield Advanced, see <a href="https://docs.aws.amazon.com/waf/latest/DDOSAPIReference/API_CreateSubscription.html">CreateSubscription</a>.</p>
    fn put_policy(
        &self,
        input: PutPolicyRequest,
    ) -> RusotoFuture<PutPolicyResponse, PutPolicyError>;
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
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> FmsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> FmsClient {
        FmsClient { client, region }
    }
}

impl Fms for FmsClient {
    /// <p>Sets the AWS Firewall Manager administrator account. AWS Firewall Manager must be associated with the master account your AWS organization or associated with a member account that has the appropriate permissions. If the account ID that you submit is not an AWS Organizations master account, AWS Firewall Manager will set the appropriate permissions for the given member account.</p> <p>The account that you associate with AWS Firewall Manager is called the AWS Firewall Manager administrator account. </p>
    fn associate_admin_account(
        &self,
        input: AssociateAdminAccountRequest,
    ) -> RusotoFuture<(), AssociateAdminAccountError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.AssociateAdminAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateAdminAccountError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an AWS Firewall Manager association with the IAM role and the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    fn delete_notification_channel(&self) -> RusotoFuture<(), DeleteNotificationChannelError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.DeleteNotificationChannel");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNotificationChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Permanently deletes an AWS Firewall Manager policy. </p>
    fn delete_policy(&self, input: DeletePolicyRequest) -> RusotoFuture<(), DeletePolicyError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.DeletePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disassociates the account that has been set as the AWS Firewall Manager administrator account. To set a different account as the administrator account, you must submit an <code>AssociateAdminAccount</code> request .</p>
    fn disassociate_admin_account(&self) -> RusotoFuture<(), DisassociateAdminAccountError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.DisassociateAdminAccount");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateAdminAccountError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the AWS Organizations master account that is associated with AWS Firewall Manager as the AWS Firewall Manager administrator.</p>
    fn get_admin_account(&self) -> RusotoFuture<GetAdminAccountResponse, GetAdminAccountError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.GetAdminAccount");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAdminAccountResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAdminAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns detailed compliance information about the specified member account. Details include resources that are in and out of compliance with the specified policy. Resources are considered non-compliant if the specified policy has not been applied to them.</p>
    fn get_compliance_detail(
        &self,
        input: GetComplianceDetailRequest,
    ) -> RusotoFuture<GetComplianceDetailResponse, GetComplianceDetailError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.GetComplianceDetail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetComplianceDetailResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetComplianceDetailError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    fn get_notification_channel(
        &self,
    ) -> RusotoFuture<GetNotificationChannelResponse, GetNotificationChannelError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.GetNotificationChannel");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetNotificationChannelResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetNotificationChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about the specified AWS Firewall Manager policy.</p>
    fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> RusotoFuture<GetPolicyResponse, GetPolicyError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.GetPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>If you created a Shield Advanced policy, returns policy-level attack summary information in the event of a potential DDoS attack.</p>
    fn get_protection_status(
        &self,
        input: GetProtectionStatusRequest,
    ) -> RusotoFuture<GetProtectionStatusResponse, GetProtectionStatusError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.GetProtectionStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetProtectionStatusResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetProtectionStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of <code>PolicyComplianceStatus</code> objects in the response. Use <code>PolicyComplianceStatus</code> to get a summary of which member accounts are protected by the specified policy. </p>
    fn list_compliance_status(
        &self,
        input: ListComplianceStatusRequest,
    ) -> RusotoFuture<ListComplianceStatusResponse, ListComplianceStatusError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.ListComplianceStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListComplianceStatusResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListComplianceStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a <code>MemberAccounts</code> object that lists the member accounts in the administrator's AWS organization.</p> <p>The <code>ListMemberAccounts</code> must be submitted by the account that is set as the AWS Firewall Manager administrator.</p>
    fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> RusotoFuture<ListMemberAccountsResponse, ListMemberAccountsError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.ListMemberAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListMemberAccountsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListMemberAccountsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <code>PolicySummary</code> objects in the response.</p>
    fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> RusotoFuture<ListPoliciesResponse, ListPoliciesError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.ListPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPoliciesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPoliciesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Designates the IAM role and Amazon Simple Notification Service (SNS) topic that AWS Firewall Manager uses to record SNS logs.</p>
    fn put_notification_channel(
        &self,
        input: PutNotificationChannelRequest,
    ) -> RusotoFuture<(), PutNotificationChannelError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.PutNotificationChannel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutNotificationChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates an AWS Firewall Manager policy.</p> <p>Firewall Manager provides two types of policies: A Shield Advanced policy, which applies Shield Advanced protection to specified accounts and resources, or a WAF policy, which contains a rule group and defines which resources are to be protected by that rule group. A policy is specific to either WAF or Shield Advanced. If you want to enforce both WAF rules and Shield Advanced protection across accounts, you can create multiple policies. You can create one or more policies for WAF rules, and one or more policies for Shield Advanced.</p> <p>You must be subscribed to Shield Advanced to create a Shield Advanced policy. For more information on subscribing to Shield Advanced, see <a href="https://docs.aws.amazon.com/waf/latest/DDOSAPIReference/API_CreateSubscription.html">CreateSubscription</a>.</p>
    fn put_policy(
        &self,
        input: PutPolicyRequest,
    ) -> RusotoFuture<PutPolicyResponse, PutPolicyError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.PutPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutPolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutPolicyError::from_response(response))),
                )
            }
        })
    }
}
