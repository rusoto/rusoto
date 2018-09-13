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
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateAdminAccountRequest {
    /// <p>The AWS account ID to associate with AWS Firewall Manager as the AWS Firewall Manager administrator account. This can be an AWS Organizations master account or a member account. For more information about AWS Organizations and master accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts.html">Managing the AWS Accounts in Your Organization</a>.</p>
    #[serde(rename = "AdminAccount")]
    pub admin_account: String,
}

/// <p>Details of the resource that is not protected by the policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceViolator {
    /// <p>The resource ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The resource type. This is in the format shown in <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. Valid values are <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> or <code>AWS::CloudFront::Distribution</code>.</p>
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
    /// <p>The ID of the policy that you want to delete. <code>PolicyId</code> is returned by <code>PutPolicy</code> and by <code>ListPolicies</code>.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateAdminAccountRequest {}

/// <p>Describes the compliance status for the account. An account is considered non-compliant if it includes resources that are not protected by the specified policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetAdminAccountResponse {
    /// <p>The AWS account that is set as the AWS Firewall Manager administrator.</p>
    #[serde(rename = "AdminAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account: Option<String>,
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
pub struct GetComplianceDetailResponse {
    /// <p>Information about the resources and the policy that you specified in the <code>GetComplianceDetail</code> request.</p>
    #[serde(rename = "PolicyComplianceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_compliance_detail: Option<PolicyComplianceDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetNotificationChannelRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>If set to <code>True</code>, resources with the tags that are specified in the <code>ResourceTag</code> array are not protected by the policy. If set to <code>False</code>, and the <code>ResourceTag</code> array is not null, only resources with the specified tags are associated with the policy.</p>
    #[serde(rename = "ExcludeResourceTags")]
    pub exclude_resource_tags: bool,
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
    /// <p>The type of resource to protect with the policy, either an Application Load Balancer or a CloudFront distribution. This is in the format shown in <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. Valid values are <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> or <code>AWS::CloudFront::Distribution</code>.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>Details about the security service that is being used to protect the resources.</p>
    #[serde(rename = "SecurityServicePolicyData")]
    pub security_service_policy_data: SecurityServicePolicyData,
}

/// <p>Describes the non-compliant resources in a member account for a specific AWS Firewall Manager policy. A maximum of 100 entries are displayed. If more than 100 resources are non-compliant, <code>EvaluationLimitExceeded</code> is set to <code>True</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PolicyComplianceDetail {
    /// <p>Indicates if over 100 resources are non-compliant with the AWS Firewall Manager policy.</p>
    #[serde(rename = "EvaluationLimitExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_limit_exceeded: Option<bool>,
    /// <p>A time stamp that indicates when the returned information should be considered out-of-date.</p>
    #[serde(rename = "ExpiredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<f64>,
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
pub struct PolicyComplianceStatus {
    /// <p>An array of <code>EvaluationResult</code> objects.</p>
    #[serde(rename = "EvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
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
    /// <p>The type of resource to protect with the policy, either an Application Load Balancer or a CloudFront distribution. This is in the format shown in <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a>. Valid values are <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code> or <code>AWS::CloudFront::Distribution</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The service that the policy is using to protect the resources. This value is <code>WAF</code>.</p>
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
    /// <p>Details about the service. This contains <code>WAF</code> data in JSON format, as shown in the following example:</p> <p> <code>ManagedServiceData": "{\"type\": \"WAF\", \"ruleGroups\": [{\"id\": \"12345678-1bcd-9012-efga-0987654321ab\", \"overrideAction\" : {\"type\": \"COUNT\"}}], \"defaultAction\": {\"type\": \"BLOCK\"}}</code> </p>
    #[serde(rename = "ManagedServiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_service_data: Option<String>,
    /// <p>The service that the policy is using to protect the resources. This value is <code>WAF</code>.</p>
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateAdminAccountError {
    pub fn from_body(body: &str) -> AssociateAdminAccountError {
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
                    "InternalErrorException" => {
                        AssociateAdminAccountError::InternalError(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        AssociateAdminAccountError::InvalidInput(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        AssociateAdminAccountError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AssociateAdminAccountError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateAdminAccountError::Validation(error_message.to_string())
                    }
                    _ => AssociateAdminAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateAdminAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateAdminAccountError {
    fn from(err: serde_json::error::Error) -> AssociateAdminAccountError {
        AssociateAdminAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateAdminAccountError {
    fn from(err: CredentialsError) -> AssociateAdminAccountError {
        AssociateAdminAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateAdminAccountError {
    fn from(err: HttpDispatchError) -> AssociateAdminAccountError {
        AssociateAdminAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateAdminAccountError {
    fn from(err: io::Error) -> AssociateAdminAccountError {
        AssociateAdminAccountError::HttpDispatch(HttpDispatchError::from(err))
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
            AssociateAdminAccountError::Validation(ref cause) => cause,
            AssociateAdminAccountError::Credentials(ref err) => err.description(),
            AssociateAdminAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateAdminAccountError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteNotificationChannelError {
    pub fn from_body(body: &str) -> DeleteNotificationChannelError {
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
                    "InternalErrorException" => {
                        DeleteNotificationChannelError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        DeleteNotificationChannelError::InvalidOperation(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DeleteNotificationChannelError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteNotificationChannelError::Validation(error_message.to_string())
                    }
                    _ => DeleteNotificationChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteNotificationChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteNotificationChannelError {
    fn from(err: serde_json::error::Error) -> DeleteNotificationChannelError {
        DeleteNotificationChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNotificationChannelError {
    fn from(err: CredentialsError) -> DeleteNotificationChannelError {
        DeleteNotificationChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNotificationChannelError {
    fn from(err: HttpDispatchError) -> DeleteNotificationChannelError {
        DeleteNotificationChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNotificationChannelError {
    fn from(err: io::Error) -> DeleteNotificationChannelError {
        DeleteNotificationChannelError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteNotificationChannelError::Validation(ref cause) => cause,
            DeleteNotificationChannelError::Credentials(ref err) => err.description(),
            DeleteNotificationChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNotificationChannelError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePolicyError {
    pub fn from_body(body: &str) -> DeletePolicyError {
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
                    "InternalErrorException" => {
                        DeletePolicyError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        DeletePolicyError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeletePolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePolicyError::Validation(error_message.to_string())
                    }
                    _ => DeletePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePolicyError {
    fn from(err: serde_json::error::Error) -> DeletePolicyError {
        DeletePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePolicyError {
    fn from(err: CredentialsError) -> DeletePolicyError {
        DeletePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePolicyError {
    fn from(err: HttpDispatchError) -> DeletePolicyError {
        DeletePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePolicyError {
    fn from(err: io::Error) -> DeletePolicyError {
        DeletePolicyError::HttpDispatch(HttpDispatchError::from(err))
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
            DeletePolicyError::Validation(ref cause) => cause,
            DeletePolicyError::Credentials(ref err) => err.description(),
            DeletePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePolicyError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateAdminAccountError {
    pub fn from_body(body: &str) -> DisassociateAdminAccountError {
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
                    "InternalErrorException" => {
                        DisassociateAdminAccountError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        DisassociateAdminAccountError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DisassociateAdminAccountError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateAdminAccountError::Validation(error_message.to_string())
                    }
                    _ => DisassociateAdminAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateAdminAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateAdminAccountError {
    fn from(err: serde_json::error::Error) -> DisassociateAdminAccountError {
        DisassociateAdminAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateAdminAccountError {
    fn from(err: CredentialsError) -> DisassociateAdminAccountError {
        DisassociateAdminAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateAdminAccountError {
    fn from(err: HttpDispatchError) -> DisassociateAdminAccountError {
        DisassociateAdminAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateAdminAccountError {
    fn from(err: io::Error) -> DisassociateAdminAccountError {
        DisassociateAdminAccountError::HttpDispatch(HttpDispatchError::from(err))
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
            DisassociateAdminAccountError::Validation(ref cause) => cause,
            DisassociateAdminAccountError::Credentials(ref err) => err.description(),
            DisassociateAdminAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateAdminAccountError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAdminAccountError {
    pub fn from_body(body: &str) -> GetAdminAccountError {
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
                    "InternalErrorException" => {
                        GetAdminAccountError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        GetAdminAccountError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetAdminAccountError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAdminAccountError::Validation(error_message.to_string())
                    }
                    _ => GetAdminAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAdminAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAdminAccountError {
    fn from(err: serde_json::error::Error) -> GetAdminAccountError {
        GetAdminAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAdminAccountError {
    fn from(err: CredentialsError) -> GetAdminAccountError {
        GetAdminAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAdminAccountError {
    fn from(err: HttpDispatchError) -> GetAdminAccountError {
        GetAdminAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAdminAccountError {
    fn from(err: io::Error) -> GetAdminAccountError {
        GetAdminAccountError::HttpDispatch(HttpDispatchError::from(err))
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
            GetAdminAccountError::Validation(ref cause) => cause,
            GetAdminAccountError::Credentials(ref err) => err.description(),
            GetAdminAccountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAdminAccountError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetComplianceDetailError {
    pub fn from_body(body: &str) -> GetComplianceDetailError {
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
                    "InternalErrorException" => {
                        GetComplianceDetailError::InternalError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetComplianceDetailError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetComplianceDetailError::Validation(error_message.to_string())
                    }
                    _ => GetComplianceDetailError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetComplianceDetailError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetComplianceDetailError {
    fn from(err: serde_json::error::Error) -> GetComplianceDetailError {
        GetComplianceDetailError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetComplianceDetailError {
    fn from(err: CredentialsError) -> GetComplianceDetailError {
        GetComplianceDetailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetComplianceDetailError {
    fn from(err: HttpDispatchError) -> GetComplianceDetailError {
        GetComplianceDetailError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetComplianceDetailError {
    fn from(err: io::Error) -> GetComplianceDetailError {
        GetComplianceDetailError::HttpDispatch(HttpDispatchError::from(err))
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
            GetComplianceDetailError::Validation(ref cause) => cause,
            GetComplianceDetailError::Credentials(ref err) => err.description(),
            GetComplianceDetailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetComplianceDetailError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetNotificationChannelError {
    pub fn from_body(body: &str) -> GetNotificationChannelError {
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
                    "InternalErrorException" => {
                        GetNotificationChannelError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        GetNotificationChannelError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetNotificationChannelError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetNotificationChannelError::Validation(error_message.to_string())
                    }
                    _ => GetNotificationChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetNotificationChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetNotificationChannelError {
    fn from(err: serde_json::error::Error) -> GetNotificationChannelError {
        GetNotificationChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetNotificationChannelError {
    fn from(err: CredentialsError) -> GetNotificationChannelError {
        GetNotificationChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetNotificationChannelError {
    fn from(err: HttpDispatchError) -> GetNotificationChannelError {
        GetNotificationChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetNotificationChannelError {
    fn from(err: io::Error) -> GetNotificationChannelError {
        GetNotificationChannelError::HttpDispatch(HttpDispatchError::from(err))
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
            GetNotificationChannelError::Validation(ref cause) => cause,
            GetNotificationChannelError::Credentials(ref err) => err.description(),
            GetNotificationChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetNotificationChannelError::Unknown(ref cause) => cause,
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
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetPolicyError {
    pub fn from_body(body: &str) -> GetPolicyError {
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
                    "InternalErrorException" => {
                        GetPolicyError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        GetPolicyError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => GetPolicyError::Validation(error_message.to_string()),
                    _ => GetPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPolicyError {
    fn from(err: serde_json::error::Error) -> GetPolicyError {
        GetPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPolicyError {
    fn from(err: CredentialsError) -> GetPolicyError {
        GetPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPolicyError {
    fn from(err: HttpDispatchError) -> GetPolicyError {
        GetPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPolicyError {
    fn from(err: io::Error) -> GetPolicyError {
        GetPolicyError::HttpDispatch(HttpDispatchError::from(err))
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
            GetPolicyError::ResourceNotFound(ref cause) => cause,
            GetPolicyError::Validation(ref cause) => cause,
            GetPolicyError::Credentials(ref err) => err.description(),
            GetPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPolicyError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListComplianceStatusError {
    pub fn from_body(body: &str) -> ListComplianceStatusError {
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
                    "InternalErrorException" => {
                        ListComplianceStatusError::InternalError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListComplianceStatusError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListComplianceStatusError::Validation(error_message.to_string())
                    }
                    _ => ListComplianceStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListComplianceStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListComplianceStatusError {
    fn from(err: serde_json::error::Error) -> ListComplianceStatusError {
        ListComplianceStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListComplianceStatusError {
    fn from(err: CredentialsError) -> ListComplianceStatusError {
        ListComplianceStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListComplianceStatusError {
    fn from(err: HttpDispatchError) -> ListComplianceStatusError {
        ListComplianceStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListComplianceStatusError {
    fn from(err: io::Error) -> ListComplianceStatusError {
        ListComplianceStatusError::HttpDispatch(HttpDispatchError::from(err))
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
            ListComplianceStatusError::Validation(ref cause) => cause,
            ListComplianceStatusError::Credentials(ref err) => err.description(),
            ListComplianceStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListComplianceStatusError::Unknown(ref cause) => cause,
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
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>policy</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/fms-limits.html">Firewall Manager Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPoliciesError {
    pub fn from_body(body: &str) -> ListPoliciesError {
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
                    "InternalErrorException" => {
                        ListPoliciesError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        ListPoliciesError::InvalidOperation(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListPoliciesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPoliciesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPoliciesError {
    fn from(err: serde_json::error::Error) -> ListPoliciesError {
        ListPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPoliciesError {
    fn from(err: CredentialsError) -> ListPoliciesError {
        ListPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPoliciesError {
    fn from(err: HttpDispatchError) -> ListPoliciesError {
        ListPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPoliciesError {
    fn from(err: io::Error) -> ListPoliciesError {
        ListPoliciesError::HttpDispatch(HttpDispatchError::from(err))
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
            ListPoliciesError::Validation(ref cause) => cause,
            ListPoliciesError::Credentials(ref err) => err.description(),
            ListPoliciesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPoliciesError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutNotificationChannelError {
    pub fn from_body(body: &str) -> PutNotificationChannelError {
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
                    "InternalErrorException" => {
                        PutNotificationChannelError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        PutNotificationChannelError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutNotificationChannelError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutNotificationChannelError::Validation(error_message.to_string())
                    }
                    _ => PutNotificationChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutNotificationChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutNotificationChannelError {
    fn from(err: serde_json::error::Error) -> PutNotificationChannelError {
        PutNotificationChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutNotificationChannelError {
    fn from(err: CredentialsError) -> PutNotificationChannelError {
        PutNotificationChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutNotificationChannelError {
    fn from(err: HttpDispatchError) -> PutNotificationChannelError {
        PutNotificationChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutNotificationChannelError {
    fn from(err: io::Error) -> PutNotificationChannelError {
        PutNotificationChannelError::HttpDispatch(HttpDispatchError::from(err))
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
            PutNotificationChannelError::Validation(ref cause) => cause,
            PutNotificationChannelError::Credentials(ref err) => err.description(),
            PutNotificationChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutNotificationChannelError::Unknown(ref cause) => cause,
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
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutPolicyError {
    pub fn from_body(body: &str) -> PutPolicyError {
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
                    "InternalErrorException" => {
                        PutPolicyError::InternalError(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        PutPolicyError::InvalidInput(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        PutPolicyError::InvalidOperation(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => PutPolicyError::Validation(error_message.to_string()),
                    _ => PutPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutPolicyError {
    fn from(err: serde_json::error::Error) -> PutPolicyError {
        PutPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutPolicyError {
    fn from(err: CredentialsError) -> PutPolicyError {
        PutPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutPolicyError {
    fn from(err: HttpDispatchError) -> PutPolicyError {
        PutPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutPolicyError {
    fn from(err: io::Error) -> PutPolicyError {
        PutPolicyError::HttpDispatch(HttpDispatchError::from(err))
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
            PutPolicyError::ResourceNotFound(ref cause) => cause,
            PutPolicyError::Validation(ref cause) => cause,
            PutPolicyError::Credentials(ref err) => err.description(),
            PutPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the FMS API. FMS clients implement this trait.
pub trait Fms {
    /// <p>Sets the AWS Firewall Manager administrator account. AWS Firewall Manager must be associated with a master account in AWS Organizations or associated with a member account that has the appropriate permissions. If the account ID that you submit is not an AWS Organizations master account, AWS Firewall Manager will set the appropriate permissions for the given member account.</p> <p>The account that you associate with AWS Firewall Manager is called the AWS Firewall manager administrator account. </p>
    fn associate_admin_account(
        &self,
        input: AssociateAdminAccountRequest,
    ) -> RusotoFuture<(), AssociateAdminAccountError>;

    /// <p>Deletes an AWS Firewall Manager association with the IAM role and the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    fn delete_notification_channel(&self) -> RusotoFuture<(), DeleteNotificationChannelError>;

    /// <p>Permanently deletes an AWS Firewall Manager policy. </p>
    fn delete_policy(&self, input: DeletePolicyRequest) -> RusotoFuture<(), DeletePolicyError>;

    /// <p>Disassociates the account that has been set as the AWS Firewall Manager administrator account. You will need to submit an <code>AssociateAdminAccount</code> request to set a new account as the AWS Firewall administrator.</p>
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

    /// <p>Returns an array of <code>PolicyComplianceStatus</code> objects in the response. Use <code>PolicyComplianceStatus</code> to get a summary of which member accounts are protected by the specified policy. </p>
    fn list_compliance_status(
        &self,
        input: ListComplianceStatusRequest,
    ) -> RusotoFuture<ListComplianceStatusResponse, ListComplianceStatusError>;

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

    /// <p>Creates an AWS Firewall Manager policy.</p>
    fn put_policy(
        &self,
        input: PutPolicyRequest,
    ) -> RusotoFuture<PutPolicyResponse, PutPolicyError>;
}
/// A client for the FMS API.
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
            region: region,
        }
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
        FmsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Fms for FmsClient {
    /// <p>Sets the AWS Firewall Manager administrator account. AWS Firewall Manager must be associated with a master account in AWS Organizations or associated with a member account that has the appropriate permissions. If the account ID that you submit is not an AWS Organizations master account, AWS Firewall Manager will set the appropriate permissions for the given member account.</p> <p>The account that you associate with AWS Firewall Manager is called the AWS Firewall manager administrator account. </p>
    fn associate_admin_account(
        &self,
        input: AssociateAdminAccountRequest,
    ) -> RusotoFuture<(), AssociateAdminAccountError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.AssociateAdminAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateAdminAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes an AWS Firewall Manager association with the IAM role and the Amazon Simple Notification Service (SNS) topic that is used to record AWS Firewall Manager SNS logs.</p>
    fn delete_notification_channel(&self) -> RusotoFuture<(), DeleteNotificationChannelError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.DeleteNotificationChannel");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNotificationChannelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeletePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Disassociates the account that has been set as the AWS Firewall Manager administrator account. You will need to submit an <code>AssociateAdminAccount</code> request to set a new account as the AWS Firewall administrator.</p>
    fn disassociate_admin_account(&self) -> RusotoFuture<(), DisassociateAdminAccountError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.DisassociateAdminAccount");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateAdminAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the AWS Organizations master account that is associated with AWS Firewall Manager as the AWS Firewall Manager administrator.</p>
    fn get_admin_account(&self) -> RusotoFuture<GetAdminAccountResponse, GetAdminAccountError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.GetAdminAccount");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAdminAccountResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAdminAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetComplianceDetailResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceDetailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetNotificationChannelResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetNotificationChannelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListComplianceStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListComplianceStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPoliciesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutNotificationChannelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates an AWS Firewall Manager policy.</p>
    fn put_policy(
        &self,
        input: PutPolicyRequest,
    ) -> RusotoFuture<PutPolicyResponse, PutPolicyError> {
        let mut request = SignedRequest::new("POST", "fms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSFMS_20180101.PutPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
