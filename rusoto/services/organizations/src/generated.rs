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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcceptHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to accept.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AcceptHandshakeResponse {
    /// <p>A structure that contains details about the accepted handshake.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

/// <p>Contains information about an AWS account that is a member of an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Account {
    /// <p>The Amazon Resource Name (ARN) of the account.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The email address associated with the AWS account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for this parameter is a string of characters that represents a standard Internet email address.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The unique identifier (ID) of the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The method by which the account joined the organization.</p>
    #[serde(rename = "JoinedMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_method: Option<String>,
    /// <p>The date the account became a part of the organization.</p>
    #[serde(rename = "JoinedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_timestamp: Option<f64>,
    /// <p>The friendly name of the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the account in the organization.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachPolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want to attach to the target. You can get the ID for the policy by calling the <a>ListPolicies</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p><p>The unique identifier (ID) of the root, OU, or account that you want to attach the policy to. You can get the ID by calling the <a>ListRoots</a>, <a>ListOrganizationalUnitsForParent</a>, or <a>ListAccounts</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to cancel. You can get the ID from the <a>ListHandshakesForOrganization</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelHandshakeResponse {
    /// <p>A structure that contains details about the handshake that you canceled.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

/// <p>Contains a list of child entities, either OUs or accounts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Child {
    /// <p><p>The unique identifier (ID) of this child entity.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a child ID string requires one of the following:</p> <ul> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of this child entity.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAccountRequest {
    /// <p>The friendly name of the member account.</p>
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// <p>The email address of the owner to assign to the new member account. This email address must not already be associated with another AWS account. You must use a valid email address to complete account creation. You cannot access the root user of the account or remove an account that was created with an invalid email address.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>If set to <code>ALLOW</code>, the new account enables IAM users to access account billing information <i>if</i> they have the required permissions. If set to <code>DENY</code>, then only the root user of the new account can access account billing information. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>If you do not specify this parameter, the value defaults to ALLOW, and IAM users and roles with the required permissions can access billing information for the new account.</p>
    #[serde(rename = "IamUserAccessToBilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_access_to_billing: Option<String>,
    /// <p>(Optional)</p> <p>The name of an IAM role that Organizations automatically preconfigures in the new member account. This role trusts the master account, allowing users in the master account to assume the role, as permitted by the master account administrator. The role has administrator permissions in the new member account.</p> <p>If you do not specify this parameter, the role name defaults to <code>OrganizationAccountAccessRole</code>.</p> <p>For more information about how to use this role to access the member account, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_access.html#orgs_manage_accounts_create-cross-account-role">Accessing and Administering the Member Accounts in Your Organization</a> in the <i>AWS Organizations User Guide</i>, and steps 2 and 3 in <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/tutorial_cross-account-with-roles.html">Tutorial: Delegate Access Across AWS Accounts Using IAM Roles</a> in the <i>IAM User Guide</i>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of characters that can consist of uppercase letters, lowercase letters, digits with no spaces, and any of the following characters: =,.@-</p>
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAccountResponse {
    /// <p>A structure that contains details about the request to create an account. This response structure might not be fully populated when you first receive it because account creation is an asynchronous process. You can pass the returned CreateAccountStatus ID as a parameter to <code> <a>DescribeCreateAccountStatus</a> </code> to get status about the progress of the request at later times. </p>
    #[serde(rename = "CreateAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

/// <p>Contains the status about a <a>CreateAccount</a> request to create an AWS account in an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAccountStatus {
    /// <p>If the account was created successfully, the unique identifier (ID) of the new account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The account name given to the account when it was created.</p>
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// <p>The date and time that the account was created and the request completed.</p>
    #[serde(rename = "CompletedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    /// <p><p>If the request failed, a description of the reason for the failure.</p> <ul> <li> <p>ACCOUNT<em>LIMIT</em>EXCEEDED: The account could not be created because you have reached the limit on the number of accounts in your organization.</p> </li> <li> <p>EMAIL<em>ALREADY</em>EXISTS: The account could not be created because another AWS account with that email address already exists.</p> </li> <li> <p>INVALID<em>ADDRESS: The account could not be created because the address you provided is not valid.</p> </li> <li> <p>INVALID</em>EMAIL: The account could not be created because the email address you provided is not valid.</p> </li> <li> <p>INTERNAL_FAILURE: The account could not be created because of an internal failure. Try again later. If the problem persists, contact Customer Support.</p> </li> </ul></p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The unique identifier (ID) that references this request. You get this value from the response of the initial <a>CreateAccount</a> request to create the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an create account request ID string requires "car-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The date and time that the request was made for the account creation.</p>
    #[serde(rename = "RequestedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    /// <p>The status of the request.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateOrganizationRequest {
    /// <p><p>Specifies the feature set supported by the new organization. Each feature set supports different levels of functionality.</p> <ul> <li> <p> <i>CONSOLIDATED<em>BILLING</i>: All member accounts have their bills consolidated to and paid by the master account. For more information, see &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>getting-started<em>concepts.html#feature-set-cb-only&quot;&gt;Consolidated Billing</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <i>ALL</i>: In addition to all the features supported by the consolidated billing feature set, the master account can also apply any type of policy to any member account in the organization. For more information, see &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>getting-started_concepts.html#feature-set-all&quot;&gt;All features</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul></p>
    #[serde(rename = "FeatureSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateOrganizationResponse {
    /// <p>A structure that contains details about the newly created organization.</p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateOrganizationalUnitRequest {
    /// <p>The friendly name to assign to the new OU.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The unique identifier (ID) of the parent root or OU in which you want to create the new OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateOrganizationalUnitResponse {
    /// <p>A structure that contains details about the newly created OU.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePolicyRequest {
    /// <p>The policy content to add to the new policy. For example, if you create a <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">service control policy</a> (SCP), this string must be JSON text that specifies the permissions that admins in attached accounts can delegate to their users, groups, and roles. For more information about the SCP syntax, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>An optional description to assign to the policy.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The friendly name to assign to the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The type of policy to create.</p> <note> <p>In the current release, the only type of policy that you can create is a service control policy (SCP).</p> </note></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePolicyResponse {
    /// <p>A structure that contains details about the newly created policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeclineHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to decline. You can get the ID from the <a>ListHandshakesForAccount</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeclineHandshakeResponse {
    /// <p>A structure that contains details about the declined handshake. The state is updated to show the value <code>DECLINED</code>.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteOrganizationalUnitRequest {
    /// <p>The unique identifier (ID) of the organizational unit that you want to delete. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want to delete. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAccountRequest {
    /// <p>The unique identifier (ID) of the AWS account that you want information about. You can get the ID from the <a>ListAccounts</a> or <a>ListAccountsForParent</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAccountResponse {
    /// <p>A structure that contains information about the requested account.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCreateAccountStatusRequest {
    /// <p>Specifies the <code>operationId</code> that uniquely identifies the request. You can get the ID from the response to an earlier <a>CreateAccount</a> request, or from the <a>ListCreateAccountStatus</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an create account request ID string requires "car-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "CreateAccountRequestId")]
    pub create_account_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCreateAccountStatusResponse {
    /// <p>A structure that contains the current status of an account creation request.</p>
    #[serde(rename = "CreateAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want information about. You can get the ID from the original call to <a>InviteAccountToOrganization</a>, or from a call to <a>ListHandshakesForAccount</a> or <a>ListHandshakesForOrganization</a>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeHandshakeResponse {
    /// <p>A structure that contains information about the specified handshake.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeOrganizationResponse {
    /// <p>A structure that contains information about the organization.</p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeOrganizationalUnitRequest {
    /// <p>The unique identifier (ID) of the organizational unit that you want details about. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeOrganizationalUnitResponse {
    /// <p>A structure that contains details about the specified OU.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want details about. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePolicyResponse {
    /// <p>A structure that contains details about the specified policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachPolicyRequest {
    /// <p>The unique identifier (ID) of the policy you want to detach. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p><p>The unique identifier (ID) of the root, OU, or account from which you want to detach the policy. You can get the ID from the <a>ListRoots</a>, <a>ListOrganizationalUnitsForParent</a>, or <a>ListAccounts</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableAWSServiceAccessRequest {
    /// <p>The service principal name of the AWS service for which you want to disable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisablePolicyTypeRequest {
    /// <p>The policy type that you want to disable in this root.</p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The unique identifier (ID) of the root in which you want to disable a policy type. You can get the ID from the <a>ListRoots</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lower-case letters or digits.</p>
    #[serde(rename = "RootId")]
    pub root_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisablePolicyTypeResponse {
    /// <p>A structure that shows the root with the updated list of enabled policy types.</p>
    #[serde(rename = "Root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableAWSServiceAccessRequest {
    /// <p>The service principal name of the AWS service for which you want to enable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableAllFeaturesRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnableAllFeaturesResponse {
    /// <p>A structure that contains details about the handshake created to support this request to enable all features in the organization.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnablePolicyTypeRequest {
    /// <p>The policy type that you want to enable.</p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The unique identifier (ID) of the root in which you want to enable a policy type. You can get the ID from the <a>ListRoots</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lower-case letters or digits.</p>
    #[serde(rename = "RootId")]
    pub root_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnablePolicyTypeResponse {
    /// <p>A structure that shows the root with the updated list of enabled policy types.</p>
    #[serde(rename = "Root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

/// <p>A structure that contains details of a service principal that is enabled to integrate with AWS Organizations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnabledServicePrincipal {
    /// <p>The date that the service principal was enabled for integration with AWS Organizations.</p>
    #[serde(rename = "DateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_enabled: Option<f64>,
    /// <p>The name of the service principal. This is typically in the form of a URL, such as: <code> <i>servicename</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

/// <p>Contains information that must be exchanged to securely establish a relationship between two accounts (an <i>originator</i> and a <i>recipient</i>). For example, when a master account (the originator) invites another account (the recipient) to join its organization, the two accounts exchange information as a series of handshake requests and responses.</p> <p> <b>Note:</b> Handshakes that are CANCELED, ACCEPTED, or DECLINED show up in lists for only 30 days after entering that state After that they are deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Handshake {
    /// <p><p>The type of handshake, indicating what action occurs when the recipient accepts the handshake. The following handshake types are supported:</p> <ul> <li> <p> <b>INVITE</b>: This type of handshake represents a request to join an organization. It is always sent from the master account to only non-member accounts.</p> </li> <li> <p> <b>ENABLE<em>ALL</em>FEATURES</b>: This type of handshake represents a request to enable all features in an organization. It is always sent from the master account to only <i>invited</i> member accounts. Created accounts do not receive this because those accounts were created by the organization&#39;s master account and approval is inferred.</p> </li> <li> <p> <b>APPROVE<em>ALL</em>FEATURES</b>: This type of handshake is sent from the Organizations service when all member accounts have approved the <code>ENABLE<em>ALL</em>FEATURES</code> invitation. It is sent only to the master account and signals the master that it can finalize the process to enable all features.</p> </li> </ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of a handshake.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the handshake expires. If the recipient of the handshake request fails to respond before the specified date and time, the handshake becomes inactive and is no longer valid.</p>
    #[serde(rename = "ExpirationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<f64>,
    /// <p>The unique identifier (ID) of a handshake. The originating account creates the ID when it initiates the handshake.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Information about the two accounts that are participating in the handshake.</p>
    #[serde(rename = "Parties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<HandshakeParty>>,
    /// <p>The date and time that the handshake request was made.</p>
    #[serde(rename = "RequestedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    /// <p>Additional information that is needed to process the handshake.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
    /// <p><p>The current state of the handshake. Use the state to trace the flow of the handshake through the process from its creation to its acceptance. The meaning of each of the valid values is as follows:</p> <ul> <li> <p> <b>REQUESTED</b>: This handshake was sent to multiple recipients (applicable to only some handshake types) and not all recipients have responded yet. The request stays in this state until all recipients respond.</p> </li> <li> <p> <b>OPEN</b>: This handshake was sent to multiple recipients (applicable to only some policy types) and all recipients have responded, allowing the originator to complete the handshake action.</p> </li> <li> <p> <b>CANCELED</b>: This handshake is no longer active because it was canceled by the originating account.</p> </li> <li> <p> <b>ACCEPTED</b>: This handshake is complete because it has been accepted by the recipient.</p> </li> <li> <p> <b>DECLINED</b>: This handshake is no longer active because it was declined by the recipient account.</p> </li> <li> <p> <b>EXPIRED</b>: This handshake is no longer active because the originator did not receive a response of any kind from the recipient before the expiration time (15 days).</p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Specifies the criteria that are used to select the handshakes for the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HandshakeFilter {
    /// <p>Specifies the type of handshake action.</p> <p>If you specify <code>ActionType</code>, you cannot also specify <code>ParentHandshakeId</code>.</p>
    #[serde(rename = "ActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>Specifies the parent handshake. Only used for handshake types that are a child of another type.</p> <p>If you specify <code>ParentHandshakeId</code>, you cannot also specify <code>ActionType</code>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "ParentHandshakeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_handshake_id: Option<String>,
}

/// <p>Identifies a participant in a handshake.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandshakeParty {
    /// <p>The unique identifier (ID) for the party.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The type of party.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Contains additional data that is needed to process a handshake.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HandshakeResource {
    /// <p>When needed, contains an additional array of <code>HandshakeResource</code> objects.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
    /// <p><p>The type of information being passed, specifying how the value is to be interpreted by the other party:</p> <ul> <li> <p> <code>ACCOUNT</code> - Specifies an AWS account ID number.</p> </li> <li> <p> <code>ORGANIZATION</code> - Specifies an organization ID number.</p> </li> <li> <p> <code>EMAIL</code> - Specifies the email address that is associated with the account that receives the handshake. </p> </li> <li> <p> <code>OWNER<em>EMAIL</code> - Specifies the email address associated with the master account. Included as information about an organization. </p> </li> <li> <p> <code>OWNER</em>NAME</code> - Specifies the name associated with the master account. Included as information about an organization. </p> </li> <li> <p> <code>NOTES</code> - Additional text provided by the handshake initiator and intended for the recipient to read.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The information that is passed to the other party in the handshake. The format of the value string must match the requirements of the specified type.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InviteAccountToOrganizationRequest {
    /// <p>Additional information that you want to include in the generated email to the recipient account owner.</p>
    #[serde(rename = "Notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// <p>The identifier (ID) of the AWS account that you want to invite to join your organization. This is a JSON object that contains the following elements: </p> <p> <code>{ "Type": "ACCOUNT", "Id": "&lt;<i> <b>account id number</b> </i>&gt;" }</code> </p> <p>If you use the AWS CLI, you can submit this as a single string, similar to the following example:</p> <p> <code>--target Id=123456789012,Type=ACCOUNT</code> </p> <p>If you specify <code>"Type": "ACCOUNT"</code>, then you must provide the AWS account ID number as the <code>Id</code>. If you specify <code>"Type": "EMAIL"</code>, then you must specify the email address that is associated with the account.</p> <p> <code>--target Id=bill@example.com,Type=EMAIL</code> </p>
    #[serde(rename = "Target")]
    pub target: HandshakeParty,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InviteAccountToOrganizationResponse {
    /// <p>A structure that contains details about the handshake that is created to support this invitation request.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAWSServiceAccessForOrganizationRequest {
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAWSServiceAccessForOrganizationResponse {
    /// <p>A list of the service principals for the services that are enabled to integrate with your organization. Each principal is a structure that includes the name and the date that it was enabled for integration with AWS Organizations.</p>
    #[serde(rename = "EnabledServicePrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_service_principals: Option<Vec<EnabledServicePrincipal>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAccountsForParentRequest {
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier (ID) for the parent root or organization unit (OU) whose accounts you want to list.</p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAccountsForParentResponse {
    /// <p>A list of the accounts in the specified root or OU.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAccountsRequest {
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAccountsResponse {
    /// <p>A list of objects in the organization.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListChildrenRequest {
    /// <p>Filters the output to include only the specified child type.</p>
    #[serde(rename = "ChildType")]
    pub child_type: String,
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) for the parent root or OU whose children you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListChildrenResponse {
    /// <p>The list of children of the specified parent container.</p>
    #[serde(rename = "Children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Child>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCreateAccountStatusRequest {
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of one or more states that you want included in the response. If this parameter is not present, then all requests are included in the response.</p>
    #[serde(rename = "States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListCreateAccountStatusResponse {
    /// <p>A list of objects with details about the requests. Certain elements, such as the accountId number, are present in the output only after the account has been successfully created.</p>
    #[serde(rename = "CreateAccountStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_statuses: Option<Vec<CreateAccountStatus>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListHandshakesForAccountRequest {
    /// <p>Filters the handshakes that you want included in the response. The default is all types. Use the <code>ActionType</code> element to limit the output to only a specified type, such as <code>INVITE</code>, <code>ENABLE-FULL-CONTROL</code>, or <code>APPROVE-FULL-CONTROL</code>. Alternatively, for the <code>ENABLE-FULL-CONTROL</code> handshake that generates a separate child handshake for each member account, you can specify <code>ParentHandshakeId</code> to see only the handshakes that were generated by that parent request.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListHandshakesForAccountResponse {
    /// <p>A list of <a>Handshake</a> objects with details about each of the handshakes that is associated with the specified account.</p>
    #[serde(rename = "Handshakes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListHandshakesForOrganizationRequest {
    /// <p>A filter of the handshakes that you want included in the response. The default is all types. Use the <code>ActionType</code> element to limit the output to only a specified type, such as <code>INVITE</code>, <code>ENABLE-ALL-FEATURES</code>, or <code>APPROVE-ALL-FEATURES</code>. Alternatively, for the <code>ENABLE-ALL-FEATURES</code> handshake that generates a separate child handshake for each member account, you can specify the <code>ParentHandshakeId</code> to see only the handshakes that were generated by that parent request.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListHandshakesForOrganizationResponse {
    /// <p>A list of <a>Handshake</a> objects with details about each of the handshakes that are associated with an organization.</p>
    #[serde(rename = "Handshakes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOrganizationalUnitsForParentRequest {
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) of the root or OU whose child OUs you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListOrganizationalUnitsForParentResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the OUs in the specified root or parent OU.</p>
    #[serde(rename = "OrganizationalUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_units: Option<Vec<OrganizationalUnit>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListParentsRequest {
    /// <p><p>The unique identifier (ID) of the OU or account whose parent containers you want to list. Do not specify a root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a child ID string requires one of the following:</p> <ul> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ChildId")]
    pub child_id: String,
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListParentsResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parents for the specified child account or OU.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPoliciesForTargetRequest {
    /// <p>The type of policy that you want to include in the returned list.</p>
    #[serde(rename = "Filter")]
    pub filter: String,
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) of the root, organizational unit, or account whose policies you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPoliciesForTargetResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of policies that match the criteria in the request.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPoliciesRequest {
    /// <p>Specifies the type of policy that you want to include in the response.</p>
    #[serde(rename = "Filter")]
    pub filter: String,
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPoliciesResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of policies that match the filter criteria in the request. The output list does not include the policy contents. To see the content for a policy, see <a>DescribePolicy</a>.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRootsRequest {
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListRootsResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of roots that are defined in an organization.</p>
    #[serde(rename = "Roots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<Vec<Root>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTargetsForPolicyRequest {
    /// <p>(Optional) Use this to limit the number of results you want included in the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier (ID) of the policy for which you want to know its attachments.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTargetsForPolicyResponse {
    /// <p>If present, this value indicates that there is more output available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of structures, each of which contains details about one of the entities to which the specified policy is attached.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<PolicyTargetSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MoveAccountRequest {
    /// <p>The unique identifier (ID) of the account that you want to move.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p><p>The unique identifier (ID) of the root or organizational unit that you want to move the account to.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "DestinationParentId")]
    pub destination_parent_id: String,
    /// <p><p>The unique identifier (ID) of the root or organizational unit that you want to move the account from.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "SourceParentId")]
    pub source_parent_id: String,
}

/// <p>Contains details about an organization. An organization is a collection of accounts that are centrally managed together using consolidated billing, organized hierarchically with organizational units (OUs), and controlled with policies .</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Organization {
    /// <p>The Amazon Resource Name (ARN) of an organization.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>A list of policy types that are enabled for this organization. For example, if your organization has all features enabled, then service control policies (SCPs) are included in the list.</p> <note> <p>Even if a policy type is shown as available in the organization, you can separately enable and disable them at the root level by using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of a policy type in that root.</p> </note></p>
    #[serde(rename = "AvailablePolicyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_policy_types: Option<Vec<PolicyTypeSummary>>,
    /// <p>Specifies the functionality that currently is available to the organization. If set to "ALL", then all features are enabled and policies can be applied to accounts in the organization. If set to "CONSOLIDATED_BILLING", then only consolidated billing functionality is available. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "FeatureSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    /// <p>The unique identifier (ID) of an organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organization ID string requires "o-" followed by from 10 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the account that is designated as the master account for the organization.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "MasterAccountArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_arn: Option<String>,
    /// <p>The email address that is associated with the AWS account that is designated as the master account for the organization.</p>
    #[serde(rename = "MasterAccountEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_email: Option<String>,
    /// <p>The unique identifier (ID) of the master account of an organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "MasterAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_id: Option<String>,
}

/// <p>Contains details about an organizational unit (OU). An OU is a container of AWS accounts within a root of an organization. Policies that are attached to an OU apply to all accounts contained in that OU and in any child OUs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OrganizationalUnit {
    /// <p>The Amazon Resource Name (ARN) of this OU.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The unique identifier (ID) associated with this OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of this OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about either a root or an organizational unit (OU) that can contain OUs or accounts in an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Parent {
    /// <p><p>The unique identifier (ID) of the parent entity.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of the parent entity.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains rules to be applied to the affected accounts. Policies can be attached directly to accounts, or to roots and OUs to affect all accounts in those hierarchies.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Policy {
    /// <p>The text content of the policy.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>A structure that contains additional details about the policy.</p>
    #[serde(rename = "PolicySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_summary: Option<PolicySummary>,
}

/// <p>Contains information about a policy, but does not include the content. To see the content of a policy, see <a>DescribePolicy</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PolicySummary {
    /// <p>The Amazon Resource Name (ARN) of the policy.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A boolean value that indicates whether the specified policy is an AWS managed policy. If true, then you can attach the policy to roots, OUs, or accounts, but you cannot edit it.</p>
    #[serde(rename = "AwsManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_managed: Option<bool>,
    /// <p>The description of the policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier (ID) of the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of policy.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information about a root, OU, or account that a policy is attached to.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PolicyTargetSummary {
    /// <p>The Amazon Resource Name (ARN) of the policy target.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the policy target.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The unique identifier (ID) of the policy target.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p>Root: a string that begins with &quot;r-&quot; followed by from 4 to 32 lower-case letters or digits.</p> </li> <li> <p>Account: a string that consists of exactly 12 digits.</p> </li> <li> <p>Organizational unit (OU): a string that begins with &quot;ou-&quot; followed by from 4 to 32 lower-case letters or digits (the ID of the root that the OU is in) followed by a second &quot;-&quot; dash and from 8 to 32 additional lower-case letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p>The type of the policy target.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information about a policy type and its status in the associated root.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PolicyTypeSummary {
    /// <p>The status of the policy type as it relates to the associated root. To attach a policy of the specified type to a root or to an OU or account in that root, it must be available in the organization and enabled for that root.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the policy type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveAccountFromOrganizationRequest {
    /// <p>The unique identifier (ID) of the member account that you want to remove from the organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

/// <p>Contains details about a root. A root is a top-level parent node in the hierarchy of an organization that can contain organizational units (OUs) and accounts. Every root contains every AWS account in the organization. Each root enables the accounts to be organized in a different way and to have different policy types enabled for use in that root.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Root {
    /// <p>The Amazon Resource Name (ARN) of the root.</p> <p>For more information about ARNs in Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The unique identifier (ID) for the root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lower-case letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The types of policies that are currently enabled for the root and therefore can be attached to the root or to its OUs or accounts.</p> <note> <p>Even if a policy type is shown as available in the organization, you can separately enable and disable them at the root level by using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. Use <a>DescribeOrganization</a> to see the availability of the policy types in that organization.</p> </note></p>
    #[serde(rename = "PolicyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_types: Option<Vec<PolicyTypeSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateOrganizationalUnitRequest {
    /// <p>The new name that you want to assign to the OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier (ID) of the OU that you want to rename. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lower-case letters or digits (the ID of the root that contains the OU) followed by a second "-" dash and from 8 to 32 additional lower-case letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateOrganizationalUnitResponse {
    /// <p>A structure that contains the details about the specified OU, including its new name.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePolicyRequest {
    /// <p>If provided, the new content for the policy. The text must be correctly formatted JSON that complies with the syntax for the policy's type. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>If provided, the new description for the policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If provided, the new name for the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier (ID) of the policy that you want to update.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lower-case letters or digits.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdatePolicyResponse {
    /// <p>A structure that contains details about the updated policy, showing the requested changes.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

/// Errors returned by AcceptHandshake
#[derive(Debug, PartialEq)]
pub enum AcceptHandshakeError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The operation you attempted requires you to have the <code>iam:CreateServiceLinkedRole</code> so that Organizations can create the required service-linked role. You do not have that permission.</p>
    AccessDeniedForDependency(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>ALREADY<em>IN</em>AN<em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>ORGANIZATION</em>ALREADY<em>HAS</em>ALL<em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You cannot issue new invitations to join an organization while it is in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You cannot complete the operation with an account that does not have a payment instrument, such as a credit card, associated with it.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p>We can't find a handshake with the HandshakeId that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted, or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AcceptHandshakeError {
    pub fn from_body(body: &str) -> AcceptHandshakeError {
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
                    "AWSOrganizationsNotInUseException" => {
                        AcceptHandshakeError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        AcceptHandshakeError::AccessDenied(String::from(error_message))
                    }
                    "AccessDeniedForDependencyException" => {
                        AcceptHandshakeError::AccessDeniedForDependency(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        AcceptHandshakeError::ConcurrentModification(String::from(error_message))
                    }
                    "HandshakeAlreadyInStateException" => {
                        AcceptHandshakeError::HandshakeAlreadyInState(String::from(error_message))
                    }
                    "HandshakeConstraintViolationException" => {
                        AcceptHandshakeError::HandshakeConstraintViolation(String::from(
                            error_message,
                        ))
                    }
                    "HandshakeNotFoundException" => {
                        AcceptHandshakeError::HandshakeNotFound(String::from(error_message))
                    }
                    "InvalidHandshakeTransitionException" => {
                        AcceptHandshakeError::InvalidHandshakeTransition(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        AcceptHandshakeError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        AcceptHandshakeError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AcceptHandshakeError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        AcceptHandshakeError::Validation(error_message.to_string())
                    }
                    _ => AcceptHandshakeError::Unknown(String::from(body)),
                }
            }
            Err(_) => AcceptHandshakeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AcceptHandshakeError {
    fn from(err: serde_json::error::Error) -> AcceptHandshakeError {
        AcceptHandshakeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptHandshakeError {
    fn from(err: CredentialsError) -> AcceptHandshakeError {
        AcceptHandshakeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptHandshakeError {
    fn from(err: HttpDispatchError) -> AcceptHandshakeError {
        AcceptHandshakeError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcceptHandshakeError {
    fn from(err: io::Error) -> AcceptHandshakeError {
        AcceptHandshakeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcceptHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptHandshakeError {
    fn description(&self) -> &str {
        match *self {
            AcceptHandshakeError::AWSOrganizationsNotInUse(ref cause) => cause,
            AcceptHandshakeError::AccessDenied(ref cause) => cause,
            AcceptHandshakeError::AccessDeniedForDependency(ref cause) => cause,
            AcceptHandshakeError::ConcurrentModification(ref cause) => cause,
            AcceptHandshakeError::HandshakeAlreadyInState(ref cause) => cause,
            AcceptHandshakeError::HandshakeConstraintViolation(ref cause) => cause,
            AcceptHandshakeError::HandshakeNotFound(ref cause) => cause,
            AcceptHandshakeError::InvalidHandshakeTransition(ref cause) => cause,
            AcceptHandshakeError::InvalidInput(ref cause) => cause,
            AcceptHandshakeError::Service(ref cause) => cause,
            AcceptHandshakeError::TooManyRequests(ref cause) => cause,
            AcceptHandshakeError::Validation(ref cause) => cause,
            AcceptHandshakeError::Credentials(ref err) => err.description(),
            AcceptHandshakeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AcceptHandshakeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPolicyError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The selected policy is already attached to the specified target.</p>
    DuplicatePolicyAttachment(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the PolicyId that you specified.</p>
    PolicyNotFound(String),
    /// <p>The specified policy type is not currently enabled in this root. You cannot attach policies of the specified type to entities in a root until you enable that type in the root. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotEnabled(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, or account with the TargetId that you specified.</p>
    TargetNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachPolicyError {
    pub fn from_body(body: &str) -> AttachPolicyError {
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
                    "AWSOrganizationsNotInUseException" => {
                        AttachPolicyError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        AttachPolicyError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        AttachPolicyError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        AttachPolicyError::ConstraintViolation(String::from(error_message))
                    }
                    "DuplicatePolicyAttachmentException" => {
                        AttachPolicyError::DuplicatePolicyAttachment(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        AttachPolicyError::InvalidInput(String::from(error_message))
                    }
                    "PolicyNotFoundException" => {
                        AttachPolicyError::PolicyNotFound(String::from(error_message))
                    }
                    "PolicyTypeNotEnabledException" => {
                        AttachPolicyError::PolicyTypeNotEnabled(String::from(error_message))
                    }
                    "ServiceException" => AttachPolicyError::Service(String::from(error_message)),
                    "TargetNotFoundException" => {
                        AttachPolicyError::TargetNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AttachPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachPolicyError::Validation(error_message.to_string())
                    }
                    _ => AttachPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachPolicyError {
    fn from(err: serde_json::error::Error) -> AttachPolicyError {
        AttachPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachPolicyError {
    fn from(err: CredentialsError) -> AttachPolicyError {
        AttachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachPolicyError {
    fn from(err: HttpDispatchError) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachPolicyError {
    fn from(err: io::Error) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachPolicyError {
    fn description(&self) -> &str {
        match *self {
            AttachPolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            AttachPolicyError::AccessDenied(ref cause) => cause,
            AttachPolicyError::ConcurrentModification(ref cause) => cause,
            AttachPolicyError::ConstraintViolation(ref cause) => cause,
            AttachPolicyError::DuplicatePolicyAttachment(ref cause) => cause,
            AttachPolicyError::InvalidInput(ref cause) => cause,
            AttachPolicyError::PolicyNotFound(ref cause) => cause,
            AttachPolicyError::PolicyTypeNotEnabled(ref cause) => cause,
            AttachPolicyError::Service(ref cause) => cause,
            AttachPolicyError::TargetNotFound(ref cause) => cause,
            AttachPolicyError::TooManyRequests(ref cause) => cause,
            AttachPolicyError::Validation(ref cause) => cause,
            AttachPolicyError::Credentials(ref err) => err.description(),
            AttachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelHandshake
#[derive(Debug, PartialEq)]
pub enum CancelHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p>We can't find a handshake with the HandshakeId that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted, or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelHandshakeError {
    pub fn from_body(body: &str) -> CancelHandshakeError {
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
                    "AccessDeniedException" => {
                        CancelHandshakeError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        CancelHandshakeError::ConcurrentModification(String::from(error_message))
                    }
                    "HandshakeAlreadyInStateException" => {
                        CancelHandshakeError::HandshakeAlreadyInState(String::from(error_message))
                    }
                    "HandshakeNotFoundException" => {
                        CancelHandshakeError::HandshakeNotFound(String::from(error_message))
                    }
                    "InvalidHandshakeTransitionException" => {
                        CancelHandshakeError::InvalidHandshakeTransition(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        CancelHandshakeError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        CancelHandshakeError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CancelHandshakeError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelHandshakeError::Validation(error_message.to_string())
                    }
                    _ => CancelHandshakeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelHandshakeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelHandshakeError {
    fn from(err: serde_json::error::Error) -> CancelHandshakeError {
        CancelHandshakeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelHandshakeError {
    fn from(err: CredentialsError) -> CancelHandshakeError {
        CancelHandshakeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelHandshakeError {
    fn from(err: HttpDispatchError) -> CancelHandshakeError {
        CancelHandshakeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelHandshakeError {
    fn from(err: io::Error) -> CancelHandshakeError {
        CancelHandshakeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelHandshakeError {
    fn description(&self) -> &str {
        match *self {
            CancelHandshakeError::AccessDenied(ref cause) => cause,
            CancelHandshakeError::ConcurrentModification(ref cause) => cause,
            CancelHandshakeError::HandshakeAlreadyInState(ref cause) => cause,
            CancelHandshakeError::HandshakeNotFound(ref cause) => cause,
            CancelHandshakeError::InvalidHandshakeTransition(ref cause) => cause,
            CancelHandshakeError::InvalidInput(ref cause) => cause,
            CancelHandshakeError::Service(ref cause) => cause,
            CancelHandshakeError::TooManyRequests(ref cause) => cause,
            CancelHandshakeError::Validation(ref cause) => cause,
            CancelHandshakeError::Credentials(ref err) => err.description(),
            CancelHandshakeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelHandshakeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAccount
#[derive(Debug, PartialEq)]
pub enum CreateAccountError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>AWS Organizations could not finalize the creation of your organization. Try again later. If this persists, contact AWS customer support.</p>
    FinalizingOrganization(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAccountError {
    pub fn from_body(body: &str) -> CreateAccountError {
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
                    "AWSOrganizationsNotInUseException" => {
                        CreateAccountError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        CreateAccountError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        CreateAccountError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        CreateAccountError::ConstraintViolation(String::from(error_message))
                    }
                    "FinalizingOrganizationException" => {
                        CreateAccountError::FinalizingOrganization(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateAccountError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => CreateAccountError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreateAccountError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAccountError::Validation(error_message.to_string())
                    }
                    _ => CreateAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAccountError {
    fn from(err: serde_json::error::Error) -> CreateAccountError {
        CreateAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAccountError {
    fn from(err: CredentialsError) -> CreateAccountError {
        CreateAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAccountError {
    fn from(err: HttpDispatchError) -> CreateAccountError {
        CreateAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAccountError {
    fn from(err: io::Error) -> CreateAccountError {
        CreateAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAccountError {
    fn description(&self) -> &str {
        match *self {
            CreateAccountError::AWSOrganizationsNotInUse(ref cause) => cause,
            CreateAccountError::AccessDenied(ref cause) => cause,
            CreateAccountError::ConcurrentModification(ref cause) => cause,
            CreateAccountError::ConstraintViolation(ref cause) => cause,
            CreateAccountError::FinalizingOrganization(ref cause) => cause,
            CreateAccountError::InvalidInput(ref cause) => cause,
            CreateAccountError::Service(ref cause) => cause,
            CreateAccountError::TooManyRequests(ref cause) => cause,
            CreateAccountError::Validation(ref cause) => cause,
            CreateAccountError::Credentials(ref err) => err.description(),
            CreateAccountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOrganization
#[derive(Debug, PartialEq)]
pub enum CreateOrganizationError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The operation you attempted requires you to have the <code>iam:CreateServiceLinkedRole</code> so that Organizations can create the required service-linked role. You do not have that permission.</p>
    AccessDeniedForDependency(String),
    /// <p>This account is already a member of an organization. An account can belong to only one organization at a time.</p>
    AlreadyInOrganization(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateOrganizationError {
    pub fn from_body(body: &str) -> CreateOrganizationError {
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
                    "AccessDeniedException" => {
                        CreateOrganizationError::AccessDenied(String::from(error_message))
                    }
                    "AccessDeniedForDependencyException" => {
                        CreateOrganizationError::AccessDeniedForDependency(String::from(
                            error_message,
                        ))
                    }
                    "AlreadyInOrganizationException" => {
                        CreateOrganizationError::AlreadyInOrganization(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        CreateOrganizationError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        CreateOrganizationError::ConstraintViolation(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateOrganizationError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateOrganizationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateOrganizationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateOrganizationError::Validation(error_message.to_string())
                    }
                    _ => CreateOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateOrganizationError {
    fn from(err: serde_json::error::Error) -> CreateOrganizationError {
        CreateOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateOrganizationError {
    fn from(err: CredentialsError) -> CreateOrganizationError {
        CreateOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateOrganizationError {
    fn from(err: HttpDispatchError) -> CreateOrganizationError {
        CreateOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateOrganizationError {
    fn from(err: io::Error) -> CreateOrganizationError {
        CreateOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOrganizationError {
    fn description(&self) -> &str {
        match *self {
            CreateOrganizationError::AccessDenied(ref cause) => cause,
            CreateOrganizationError::AccessDeniedForDependency(ref cause) => cause,
            CreateOrganizationError::AlreadyInOrganization(ref cause) => cause,
            CreateOrganizationError::ConcurrentModification(ref cause) => cause,
            CreateOrganizationError::ConstraintViolation(ref cause) => cause,
            CreateOrganizationError::InvalidInput(ref cause) => cause,
            CreateOrganizationError::Service(ref cause) => cause,
            CreateOrganizationError::TooManyRequests(ref cause) => cause,
            CreateOrganizationError::Validation(ref cause) => cause,
            CreateOrganizationError::Credentials(ref err) => err.description(),
            CreateOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum CreateOrganizationalUnitError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>An organizational unit (OU) with the same name already exists.</p>
    DuplicateOrganizationalUnit(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or organizational unit (OU) with the ParentId that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateOrganizationalUnitError {
    pub fn from_body(body: &str) -> CreateOrganizationalUnitError {
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
                    "AWSOrganizationsNotInUseException" => {
                        CreateOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        CreateOrganizationalUnitError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        CreateOrganizationalUnitError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "ConstraintViolationException" => {
                        CreateOrganizationalUnitError::ConstraintViolation(String::from(
                            error_message,
                        ))
                    }
                    "DuplicateOrganizationalUnitException" => {
                        CreateOrganizationalUnitError::DuplicateOrganizationalUnit(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        CreateOrganizationalUnitError::InvalidInput(String::from(error_message))
                    }
                    "ParentNotFoundException" => {
                        CreateOrganizationalUnitError::ParentNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateOrganizationalUnitError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateOrganizationalUnitError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateOrganizationalUnitError::Validation(error_message.to_string())
                    }
                    _ => CreateOrganizationalUnitError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateOrganizationalUnitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateOrganizationalUnitError {
    fn from(err: serde_json::error::Error) -> CreateOrganizationalUnitError {
        CreateOrganizationalUnitError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateOrganizationalUnitError {
    fn from(err: CredentialsError) -> CreateOrganizationalUnitError {
        CreateOrganizationalUnitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateOrganizationalUnitError {
    fn from(err: HttpDispatchError) -> CreateOrganizationalUnitError {
        CreateOrganizationalUnitError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateOrganizationalUnitError {
    fn from(err: io::Error) -> CreateOrganizationalUnitError {
        CreateOrganizationalUnitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            CreateOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            CreateOrganizationalUnitError::AccessDenied(ref cause) => cause,
            CreateOrganizationalUnitError::ConcurrentModification(ref cause) => cause,
            CreateOrganizationalUnitError::ConstraintViolation(ref cause) => cause,
            CreateOrganizationalUnitError::DuplicateOrganizationalUnit(ref cause) => cause,
            CreateOrganizationalUnitError::InvalidInput(ref cause) => cause,
            CreateOrganizationalUnitError::ParentNotFound(ref cause) => cause,
            CreateOrganizationalUnitError::Service(ref cause) => cause,
            CreateOrganizationalUnitError::TooManyRequests(ref cause) => cause,
            CreateOrganizationalUnitError::Validation(ref cause) => cause,
            CreateOrganizationalUnitError::Credentials(ref err) => err.description(),
            CreateOrganizationalUnitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateOrganizationalUnitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePolicy
#[derive(Debug, PartialEq)]
pub enum CreatePolicyError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>A policy with the same name already exists.</p>
    DuplicatePolicy(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The provided policy document does not meet the requirements of the specified policy type. For example, the syntax might be incorrect. For details about service control policy syntax, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    MalformedPolicyDocument(String),
    /// <p>You can't use the specified policy type with the feature set currently enabled for this organization. For example, you can enable service control policies (SCPs) only after you enable all features in the organization. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html#enable_policies_on_root">Enabling and Disabling a Policy Type on a Root</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotAvailableForOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreatePolicyError {
    pub fn from_body(body: &str) -> CreatePolicyError {
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
                    "AWSOrganizationsNotInUseException" => {
                        CreatePolicyError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        CreatePolicyError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        CreatePolicyError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        CreatePolicyError::ConstraintViolation(String::from(error_message))
                    }
                    "DuplicatePolicyException" => {
                        CreatePolicyError::DuplicatePolicy(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreatePolicyError::InvalidInput(String::from(error_message))
                    }
                    "MalformedPolicyDocumentException" => {
                        CreatePolicyError::MalformedPolicyDocument(String::from(error_message))
                    }
                    "PolicyTypeNotAvailableForOrganizationException" => {
                        CreatePolicyError::PolicyTypeNotAvailableForOrganization(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => CreatePolicyError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreatePolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePolicyError::Validation(error_message.to_string())
                    }
                    _ => CreatePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePolicyError {
    fn from(err: serde_json::error::Error) -> CreatePolicyError {
        CreatePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePolicyError {
    fn from(err: CredentialsError) -> CreatePolicyError {
        CreatePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePolicyError {
    fn from(err: HttpDispatchError) -> CreatePolicyError {
        CreatePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePolicyError {
    fn from(err: io::Error) -> CreatePolicyError {
        CreatePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePolicyError {
    fn description(&self) -> &str {
        match *self {
            CreatePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            CreatePolicyError::AccessDenied(ref cause) => cause,
            CreatePolicyError::ConcurrentModification(ref cause) => cause,
            CreatePolicyError::ConstraintViolation(ref cause) => cause,
            CreatePolicyError::DuplicatePolicy(ref cause) => cause,
            CreatePolicyError::InvalidInput(ref cause) => cause,
            CreatePolicyError::MalformedPolicyDocument(ref cause) => cause,
            CreatePolicyError::PolicyTypeNotAvailableForOrganization(ref cause) => cause,
            CreatePolicyError::Service(ref cause) => cause,
            CreatePolicyError::TooManyRequests(ref cause) => cause,
            CreatePolicyError::Validation(ref cause) => cause,
            CreatePolicyError::Credentials(ref err) => err.description(),
            CreatePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeclineHandshake
#[derive(Debug, PartialEq)]
pub enum DeclineHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p>We can't find a handshake with the HandshakeId that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted, or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeclineHandshakeError {
    pub fn from_body(body: &str) -> DeclineHandshakeError {
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
                    "AccessDeniedException" => {
                        DeclineHandshakeError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DeclineHandshakeError::ConcurrentModification(String::from(error_message))
                    }
                    "HandshakeAlreadyInStateException" => {
                        DeclineHandshakeError::HandshakeAlreadyInState(String::from(error_message))
                    }
                    "HandshakeNotFoundException" => {
                        DeclineHandshakeError::HandshakeNotFound(String::from(error_message))
                    }
                    "InvalidHandshakeTransitionException" => {
                        DeclineHandshakeError::InvalidHandshakeTransition(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        DeclineHandshakeError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeclineHandshakeError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeclineHandshakeError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeclineHandshakeError::Validation(error_message.to_string())
                    }
                    _ => DeclineHandshakeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeclineHandshakeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeclineHandshakeError {
    fn from(err: serde_json::error::Error) -> DeclineHandshakeError {
        DeclineHandshakeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeclineHandshakeError {
    fn from(err: CredentialsError) -> DeclineHandshakeError {
        DeclineHandshakeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeclineHandshakeError {
    fn from(err: HttpDispatchError) -> DeclineHandshakeError {
        DeclineHandshakeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeclineHandshakeError {
    fn from(err: io::Error) -> DeclineHandshakeError {
        DeclineHandshakeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeclineHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeclineHandshakeError {
    fn description(&self) -> &str {
        match *self {
            DeclineHandshakeError::AccessDenied(ref cause) => cause,
            DeclineHandshakeError::ConcurrentModification(ref cause) => cause,
            DeclineHandshakeError::HandshakeAlreadyInState(ref cause) => cause,
            DeclineHandshakeError::HandshakeNotFound(ref cause) => cause,
            DeclineHandshakeError::InvalidHandshakeTransition(ref cause) => cause,
            DeclineHandshakeError::InvalidInput(ref cause) => cause,
            DeclineHandshakeError::Service(ref cause) => cause,
            DeclineHandshakeError::TooManyRequests(ref cause) => cause,
            DeclineHandshakeError::Validation(ref cause) => cause,
            DeclineHandshakeError::Credentials(ref err) => err.description(),
            DeclineHandshakeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeclineHandshakeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteOrganization
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The organization isn't empty. To delete an organization, you must first remove all accounts except the master account, delete all organizational units (OUs), and delete all policies.</p>
    OrganizationNotEmpty(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteOrganizationError {
    pub fn from_body(body: &str) -> DeleteOrganizationError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DeleteOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        DeleteOrganizationError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DeleteOrganizationError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteOrganizationError::InvalidInput(String::from(error_message))
                    }
                    "OrganizationNotEmptyException" => {
                        DeleteOrganizationError::OrganizationNotEmpty(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeleteOrganizationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteOrganizationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteOrganizationError::Validation(error_message.to_string())
                    }
                    _ => DeleteOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteOrganizationError {
    fn from(err: serde_json::error::Error) -> DeleteOrganizationError {
        DeleteOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteOrganizationError {
    fn from(err: CredentialsError) -> DeleteOrganizationError {
        DeleteOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteOrganizationError {
    fn from(err: HttpDispatchError) -> DeleteOrganizationError {
        DeleteOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteOrganizationError {
    fn from(err: io::Error) -> DeleteOrganizationError {
        DeleteOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteOrganizationError {
    fn description(&self) -> &str {
        match *self {
            DeleteOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            DeleteOrganizationError::AccessDenied(ref cause) => cause,
            DeleteOrganizationError::ConcurrentModification(ref cause) => cause,
            DeleteOrganizationError::InvalidInput(ref cause) => cause,
            DeleteOrganizationError::OrganizationNotEmpty(ref cause) => cause,
            DeleteOrganizationError::Service(ref cause) => cause,
            DeleteOrganizationError::TooManyRequests(ref cause) => cause,
            DeleteOrganizationError::Validation(ref cause) => cause,
            DeleteOrganizationError::Credentials(ref err) => err.description(),
            DeleteOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationalUnitError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The specified organizational unit (OU) is not empty. Move all accounts to another root or to other OUs, remove all child OUs, and then try the operation again.</p>
    OrganizationalUnitNotEmpty(String),
    /// <p>We can't find an organizational unit (OU) with the OrganizationalUnitId that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteOrganizationalUnitError {
    pub fn from_body(body: &str) -> DeleteOrganizationalUnitError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DeleteOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        DeleteOrganizationalUnitError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DeleteOrganizationalUnitError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        DeleteOrganizationalUnitError::InvalidInput(String::from(error_message))
                    }
                    "OrganizationalUnitNotEmptyException" => {
                        DeleteOrganizationalUnitError::OrganizationalUnitNotEmpty(String::from(
                            error_message,
                        ))
                    }
                    "OrganizationalUnitNotFoundException" => {
                        DeleteOrganizationalUnitError::OrganizationalUnitNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        DeleteOrganizationalUnitError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteOrganizationalUnitError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteOrganizationalUnitError::Validation(error_message.to_string())
                    }
                    _ => DeleteOrganizationalUnitError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteOrganizationalUnitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteOrganizationalUnitError {
    fn from(err: serde_json::error::Error) -> DeleteOrganizationalUnitError {
        DeleteOrganizationalUnitError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteOrganizationalUnitError {
    fn from(err: CredentialsError) -> DeleteOrganizationalUnitError {
        DeleteOrganizationalUnitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteOrganizationalUnitError {
    fn from(err: HttpDispatchError) -> DeleteOrganizationalUnitError {
        DeleteOrganizationalUnitError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteOrganizationalUnitError {
    fn from(err: io::Error) -> DeleteOrganizationalUnitError {
        DeleteOrganizationalUnitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            DeleteOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            DeleteOrganizationalUnitError::AccessDenied(ref cause) => cause,
            DeleteOrganizationalUnitError::ConcurrentModification(ref cause) => cause,
            DeleteOrganizationalUnitError::InvalidInput(ref cause) => cause,
            DeleteOrganizationalUnitError::OrganizationalUnitNotEmpty(ref cause) => cause,
            DeleteOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => cause,
            DeleteOrganizationalUnitError::Service(ref cause) => cause,
            DeleteOrganizationalUnitError::TooManyRequests(ref cause) => cause,
            DeleteOrganizationalUnitError::Validation(ref cause) => cause,
            DeleteOrganizationalUnitError::Credentials(ref err) => err.description(),
            DeleteOrganizationalUnitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteOrganizationalUnitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The policy is attached to one or more entities. You must detach it from all roots, organizational units (OUs), and accounts before performing this operation.</p>
    PolicyInUse(String),
    /// <p>We can't find a policy with the PolicyId that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
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
                    "AWSOrganizationsNotInUseException" => {
                        DeletePolicyError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        DeletePolicyError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DeletePolicyError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeletePolicyError::InvalidInput(String::from(error_message))
                    }
                    "PolicyInUseException" => {
                        DeletePolicyError::PolicyInUse(String::from(error_message))
                    }
                    "PolicyNotFoundException" => {
                        DeletePolicyError::PolicyNotFound(String::from(error_message))
                    }
                    "ServiceException" => DeletePolicyError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeletePolicyError::TooManyRequests(String::from(error_message))
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
            DeletePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            DeletePolicyError::AccessDenied(ref cause) => cause,
            DeletePolicyError::ConcurrentModification(ref cause) => cause,
            DeletePolicyError::InvalidInput(ref cause) => cause,
            DeletePolicyError::PolicyInUse(ref cause) => cause,
            DeletePolicyError::PolicyNotFound(ref cause) => cause,
            DeletePolicyError::Service(ref cause) => cause,
            DeletePolicyError::TooManyRequests(ref cause) => cause,
            DeletePolicyError::Validation(ref cause) => cause,
            DeletePolicyError::Credentials(ref err) => err.description(),
            DeletePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccount
#[derive(Debug, PartialEq)]
pub enum DescribeAccountError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the AccountId that you specified, or the account whose credentials you used to make this request is not a member of an organization.</p>
    AccountNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAccountError {
    pub fn from_body(body: &str) -> DescribeAccountError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DescribeAccountError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        DescribeAccountError::AccessDenied(String::from(error_message))
                    }
                    "AccountNotFoundException" => {
                        DescribeAccountError::AccountNotFound(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeAccountError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        DescribeAccountError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeAccountError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAccountError::Validation(error_message.to_string())
                    }
                    _ => DescribeAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAccountError {
    fn from(err: serde_json::error::Error) -> DescribeAccountError {
        DescribeAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAccountError {
    fn from(err: CredentialsError) -> DescribeAccountError {
        DescribeAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAccountError {
    fn from(err: HttpDispatchError) -> DescribeAccountError {
        DescribeAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAccountError {
    fn from(err: io::Error) -> DescribeAccountError {
        DescribeAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeAccountError::AccessDenied(ref cause) => cause,
            DescribeAccountError::AccountNotFound(ref cause) => cause,
            DescribeAccountError::InvalidInput(ref cause) => cause,
            DescribeAccountError::Service(ref cause) => cause,
            DescribeAccountError::TooManyRequests(ref cause) => cause,
            DescribeAccountError::Validation(ref cause) => cause,
            DescribeAccountError::Credentials(ref err) => err.description(),
            DescribeAccountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCreateAccountStatus
#[derive(Debug, PartialEq)]
pub enum DescribeCreateAccountStatusError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>We can't find an create account request with the CreateAccountRequestId that you specified.</p>
    CreateAccountStatusNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCreateAccountStatusError {
    pub fn from_body(body: &str) -> DescribeCreateAccountStatusError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DescribeCreateAccountStatusError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        DescribeCreateAccountStatusError::AccessDenied(String::from(error_message))
                    }
                    "CreateAccountStatusNotFoundException" => {
                        DescribeCreateAccountStatusError::CreateAccountStatusNotFound(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        DescribeCreateAccountStatusError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        DescribeCreateAccountStatusError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeCreateAccountStatusError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeCreateAccountStatusError::Validation(error_message.to_string())
                    }
                    _ => DescribeCreateAccountStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCreateAccountStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCreateAccountStatusError {
    fn from(err: serde_json::error::Error) -> DescribeCreateAccountStatusError {
        DescribeCreateAccountStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCreateAccountStatusError {
    fn from(err: CredentialsError) -> DescribeCreateAccountStatusError {
        DescribeCreateAccountStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCreateAccountStatusError {
    fn from(err: HttpDispatchError) -> DescribeCreateAccountStatusError {
        DescribeCreateAccountStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCreateAccountStatusError {
    fn from(err: io::Error) -> DescribeCreateAccountStatusError {
        DescribeCreateAccountStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCreateAccountStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCreateAccountStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeCreateAccountStatusError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeCreateAccountStatusError::AccessDenied(ref cause) => cause,
            DescribeCreateAccountStatusError::CreateAccountStatusNotFound(ref cause) => cause,
            DescribeCreateAccountStatusError::InvalidInput(ref cause) => cause,
            DescribeCreateAccountStatusError::Service(ref cause) => cause,
            DescribeCreateAccountStatusError::TooManyRequests(ref cause) => cause,
            DescribeCreateAccountStatusError::Validation(ref cause) => cause,
            DescribeCreateAccountStatusError::Credentials(ref err) => err.description(),
            DescribeCreateAccountStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCreateAccountStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeHandshake
#[derive(Debug, PartialEq)]
pub enum DescribeHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>We can't find a handshake with the HandshakeId that you specified.</p>
    HandshakeNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeHandshakeError {
    pub fn from_body(body: &str) -> DescribeHandshakeError {
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
                    "AccessDeniedException" => {
                        DescribeHandshakeError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DescribeHandshakeError::ConcurrentModification(String::from(error_message))
                    }
                    "HandshakeNotFoundException" => {
                        DescribeHandshakeError::HandshakeNotFound(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeHandshakeError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        DescribeHandshakeError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeHandshakeError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeHandshakeError::Validation(error_message.to_string())
                    }
                    _ => DescribeHandshakeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeHandshakeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeHandshakeError {
    fn from(err: serde_json::error::Error) -> DescribeHandshakeError {
        DescribeHandshakeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeHandshakeError {
    fn from(err: CredentialsError) -> DescribeHandshakeError {
        DescribeHandshakeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeHandshakeError {
    fn from(err: HttpDispatchError) -> DescribeHandshakeError {
        DescribeHandshakeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeHandshakeError {
    fn from(err: io::Error) -> DescribeHandshakeError {
        DescribeHandshakeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHandshakeError {
    fn description(&self) -> &str {
        match *self {
            DescribeHandshakeError::AccessDenied(ref cause) => cause,
            DescribeHandshakeError::ConcurrentModification(ref cause) => cause,
            DescribeHandshakeError::HandshakeNotFound(ref cause) => cause,
            DescribeHandshakeError::InvalidInput(ref cause) => cause,
            DescribeHandshakeError::Service(ref cause) => cause,
            DescribeHandshakeError::TooManyRequests(ref cause) => cause,
            DescribeHandshakeError::Validation(ref cause) => cause,
            DescribeHandshakeError::Credentials(ref err) => err.description(),
            DescribeHandshakeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeHandshakeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeOrganizationError {
    pub fn from_body(body: &str) -> DescribeOrganizationError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DescribeOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        DescribeOrganizationError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DescribeOrganizationError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        DescribeOrganizationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeOrganizationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeOrganizationError::Validation(error_message.to_string())
                    }
                    _ => DescribeOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeOrganizationError {
    fn from(err: serde_json::error::Error) -> DescribeOrganizationError {
        DescribeOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeOrganizationError {
    fn from(err: CredentialsError) -> DescribeOrganizationError {
        DescribeOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeOrganizationError {
    fn from(err: HttpDispatchError) -> DescribeOrganizationError {
        DescribeOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeOrganizationError {
    fn from(err: io::Error) -> DescribeOrganizationError {
        DescribeOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrganizationError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeOrganizationError::AccessDenied(ref cause) => cause,
            DescribeOrganizationError::ConcurrentModification(ref cause) => cause,
            DescribeOrganizationError::Service(ref cause) => cause,
            DescribeOrganizationError::TooManyRequests(ref cause) => cause,
            DescribeOrganizationError::Validation(ref cause) => cause,
            DescribeOrganizationError::Credentials(ref err) => err.description(),
            DescribeOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationalUnitError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find an organizational unit (OU) with the OrganizationalUnitId that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeOrganizationalUnitError {
    pub fn from_body(body: &str) -> DescribeOrganizationalUnitError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DescribeOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        DescribeOrganizationalUnitError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeOrganizationalUnitError::InvalidInput(String::from(error_message))
                    }
                    "OrganizationalUnitNotFoundException" => {
                        DescribeOrganizationalUnitError::OrganizationalUnitNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        DescribeOrganizationalUnitError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => DescribeOrganizationalUnitError::TooManyRequests(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeOrganizationalUnitError::Validation(error_message.to_string())
                    }
                    _ => DescribeOrganizationalUnitError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeOrganizationalUnitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeOrganizationalUnitError {
    fn from(err: serde_json::error::Error) -> DescribeOrganizationalUnitError {
        DescribeOrganizationalUnitError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeOrganizationalUnitError {
    fn from(err: CredentialsError) -> DescribeOrganizationalUnitError {
        DescribeOrganizationalUnitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeOrganizationalUnitError {
    fn from(err: HttpDispatchError) -> DescribeOrganizationalUnitError {
        DescribeOrganizationalUnitError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeOrganizationalUnitError {
    fn from(err: io::Error) -> DescribeOrganizationalUnitError {
        DescribeOrganizationalUnitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribeOrganizationalUnitError::AccessDenied(ref cause) => cause,
            DescribeOrganizationalUnitError::InvalidInput(ref cause) => cause,
            DescribeOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => cause,
            DescribeOrganizationalUnitError::Service(ref cause) => cause,
            DescribeOrganizationalUnitError::TooManyRequests(ref cause) => cause,
            DescribeOrganizationalUnitError::Validation(ref cause) => cause,
            DescribeOrganizationalUnitError::Credentials(ref err) => err.description(),
            DescribeOrganizationalUnitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeOrganizationalUnitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePolicy
#[derive(Debug, PartialEq)]
pub enum DescribePolicyError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the PolicyId that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribePolicyError {
    pub fn from_body(body: &str) -> DescribePolicyError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DescribePolicyError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        DescribePolicyError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribePolicyError::InvalidInput(String::from(error_message))
                    }
                    "PolicyNotFoundException" => {
                        DescribePolicyError::PolicyNotFound(String::from(error_message))
                    }
                    "ServiceException" => DescribePolicyError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DescribePolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribePolicyError::Validation(error_message.to_string())
                    }
                    _ => DescribePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePolicyError {
    fn from(err: serde_json::error::Error) -> DescribePolicyError {
        DescribePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePolicyError {
    fn from(err: CredentialsError) -> DescribePolicyError {
        DescribePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePolicyError {
    fn from(err: HttpDispatchError) -> DescribePolicyError {
        DescribePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePolicyError {
    fn from(err: io::Error) -> DescribePolicyError {
        DescribePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePolicyError {
    fn description(&self) -> &str {
        match *self {
            DescribePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            DescribePolicyError::AccessDenied(ref cause) => cause,
            DescribePolicyError::InvalidInput(ref cause) => cause,
            DescribePolicyError::PolicyNotFound(ref cause) => cause,
            DescribePolicyError::Service(ref cause) => cause,
            DescribePolicyError::TooManyRequests(ref cause) => cause,
            DescribePolicyError::Validation(ref cause) => cause,
            DescribePolicyError::Credentials(ref err) => err.description(),
            DescribePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPolicyError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The policy isn't attached to the specified target in the specified root.</p>
    PolicyNotAttached(String),
    /// <p>We can't find a policy with the PolicyId that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, or account with the TargetId that you specified.</p>
    TargetNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachPolicyError {
    pub fn from_body(body: &str) -> DetachPolicyError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DetachPolicyError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        DetachPolicyError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DetachPolicyError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        DetachPolicyError::ConstraintViolation(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DetachPolicyError::InvalidInput(String::from(error_message))
                    }
                    "PolicyNotAttachedException" => {
                        DetachPolicyError::PolicyNotAttached(String::from(error_message))
                    }
                    "PolicyNotFoundException" => {
                        DetachPolicyError::PolicyNotFound(String::from(error_message))
                    }
                    "ServiceException" => DetachPolicyError::Service(String::from(error_message)),
                    "TargetNotFoundException" => {
                        DetachPolicyError::TargetNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DetachPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachPolicyError::Validation(error_message.to_string())
                    }
                    _ => DetachPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachPolicyError {
    fn from(err: serde_json::error::Error) -> DetachPolicyError {
        DetachPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachPolicyError {
    fn from(err: CredentialsError) -> DetachPolicyError {
        DetachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachPolicyError {
    fn from(err: HttpDispatchError) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachPolicyError {
    fn from(err: io::Error) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachPolicyError {
    fn description(&self) -> &str {
        match *self {
            DetachPolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            DetachPolicyError::AccessDenied(ref cause) => cause,
            DetachPolicyError::ConcurrentModification(ref cause) => cause,
            DetachPolicyError::ConstraintViolation(ref cause) => cause,
            DetachPolicyError::InvalidInput(ref cause) => cause,
            DetachPolicyError::PolicyNotAttached(ref cause) => cause,
            DetachPolicyError::PolicyNotFound(ref cause) => cause,
            DetachPolicyError::Service(ref cause) => cause,
            DetachPolicyError::TargetNotFound(ref cause) => cause,
            DetachPolicyError::TooManyRequests(ref cause) => cause,
            DetachPolicyError::Validation(ref cause) => cause,
            DetachPolicyError::Credentials(ref err) => err.description(),
            DetachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableAWSServiceAccess
#[derive(Debug, PartialEq)]
pub enum DisableAWSServiceAccessError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableAWSServiceAccessError {
    pub fn from_body(body: &str) -> DisableAWSServiceAccessError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DisableAWSServiceAccessError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        DisableAWSServiceAccessError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DisableAWSServiceAccessError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "ConstraintViolationException" => {
                        DisableAWSServiceAccessError::ConstraintViolation(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        DisableAWSServiceAccessError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        DisableAWSServiceAccessError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DisableAWSServiceAccessError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableAWSServiceAccessError::Validation(error_message.to_string())
                    }
                    _ => DisableAWSServiceAccessError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableAWSServiceAccessError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableAWSServiceAccessError {
    fn from(err: serde_json::error::Error) -> DisableAWSServiceAccessError {
        DisableAWSServiceAccessError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableAWSServiceAccessError {
    fn from(err: CredentialsError) -> DisableAWSServiceAccessError {
        DisableAWSServiceAccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableAWSServiceAccessError {
    fn from(err: HttpDispatchError) -> DisableAWSServiceAccessError {
        DisableAWSServiceAccessError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableAWSServiceAccessError {
    fn from(err: io::Error) -> DisableAWSServiceAccessError {
        DisableAWSServiceAccessError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableAWSServiceAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableAWSServiceAccessError {
    fn description(&self) -> &str {
        match *self {
            DisableAWSServiceAccessError::AWSOrganizationsNotInUse(ref cause) => cause,
            DisableAWSServiceAccessError::AccessDenied(ref cause) => cause,
            DisableAWSServiceAccessError::ConcurrentModification(ref cause) => cause,
            DisableAWSServiceAccessError::ConstraintViolation(ref cause) => cause,
            DisableAWSServiceAccessError::InvalidInput(ref cause) => cause,
            DisableAWSServiceAccessError::Service(ref cause) => cause,
            DisableAWSServiceAccessError::TooManyRequests(ref cause) => cause,
            DisableAWSServiceAccessError::Validation(ref cause) => cause,
            DisableAWSServiceAccessError::Credentials(ref err) => err.description(),
            DisableAWSServiceAccessError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableAWSServiceAccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisablePolicyType
#[derive(Debug, PartialEq)]
pub enum DisablePolicyTypeError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The specified policy type is not currently enabled in this root. You cannot attach policies of the specified type to entities in a root until you enable that type in the root. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotEnabled(String),
    /// <p>We can't find a root with the RootId that you specified.</p>
    RootNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisablePolicyTypeError {
    pub fn from_body(body: &str) -> DisablePolicyTypeError {
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
                    "AWSOrganizationsNotInUseException" => {
                        DisablePolicyTypeError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        DisablePolicyTypeError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        DisablePolicyTypeError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        DisablePolicyTypeError::ConstraintViolation(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DisablePolicyTypeError::InvalidInput(String::from(error_message))
                    }
                    "PolicyTypeNotEnabledException" => {
                        DisablePolicyTypeError::PolicyTypeNotEnabled(String::from(error_message))
                    }
                    "RootNotFoundException" => {
                        DisablePolicyTypeError::RootNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        DisablePolicyTypeError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DisablePolicyTypeError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisablePolicyTypeError::Validation(error_message.to_string())
                    }
                    _ => DisablePolicyTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisablePolicyTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisablePolicyTypeError {
    fn from(err: serde_json::error::Error) -> DisablePolicyTypeError {
        DisablePolicyTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisablePolicyTypeError {
    fn from(err: CredentialsError) -> DisablePolicyTypeError {
        DisablePolicyTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisablePolicyTypeError {
    fn from(err: HttpDispatchError) -> DisablePolicyTypeError {
        DisablePolicyTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisablePolicyTypeError {
    fn from(err: io::Error) -> DisablePolicyTypeError {
        DisablePolicyTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisablePolicyTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisablePolicyTypeError {
    fn description(&self) -> &str {
        match *self {
            DisablePolicyTypeError::AWSOrganizationsNotInUse(ref cause) => cause,
            DisablePolicyTypeError::AccessDenied(ref cause) => cause,
            DisablePolicyTypeError::ConcurrentModification(ref cause) => cause,
            DisablePolicyTypeError::ConstraintViolation(ref cause) => cause,
            DisablePolicyTypeError::InvalidInput(ref cause) => cause,
            DisablePolicyTypeError::PolicyTypeNotEnabled(ref cause) => cause,
            DisablePolicyTypeError::RootNotFound(ref cause) => cause,
            DisablePolicyTypeError::Service(ref cause) => cause,
            DisablePolicyTypeError::TooManyRequests(ref cause) => cause,
            DisablePolicyTypeError::Validation(ref cause) => cause,
            DisablePolicyTypeError::Credentials(ref err) => err.description(),
            DisablePolicyTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisablePolicyTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableAWSServiceAccess
#[derive(Debug, PartialEq)]
pub enum EnableAWSServiceAccessError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableAWSServiceAccessError {
    pub fn from_body(body: &str) -> EnableAWSServiceAccessError {
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
                    "AWSOrganizationsNotInUseException" => {
                        EnableAWSServiceAccessError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        EnableAWSServiceAccessError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        EnableAWSServiceAccessError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "ConstraintViolationException" => {
                        EnableAWSServiceAccessError::ConstraintViolation(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        EnableAWSServiceAccessError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        EnableAWSServiceAccessError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        EnableAWSServiceAccessError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableAWSServiceAccessError::Validation(error_message.to_string())
                    }
                    _ => EnableAWSServiceAccessError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableAWSServiceAccessError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableAWSServiceAccessError {
    fn from(err: serde_json::error::Error) -> EnableAWSServiceAccessError {
        EnableAWSServiceAccessError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableAWSServiceAccessError {
    fn from(err: CredentialsError) -> EnableAWSServiceAccessError {
        EnableAWSServiceAccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableAWSServiceAccessError {
    fn from(err: HttpDispatchError) -> EnableAWSServiceAccessError {
        EnableAWSServiceAccessError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableAWSServiceAccessError {
    fn from(err: io::Error) -> EnableAWSServiceAccessError {
        EnableAWSServiceAccessError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableAWSServiceAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAWSServiceAccessError {
    fn description(&self) -> &str {
        match *self {
            EnableAWSServiceAccessError::AWSOrganizationsNotInUse(ref cause) => cause,
            EnableAWSServiceAccessError::AccessDenied(ref cause) => cause,
            EnableAWSServiceAccessError::ConcurrentModification(ref cause) => cause,
            EnableAWSServiceAccessError::ConstraintViolation(ref cause) => cause,
            EnableAWSServiceAccessError::InvalidInput(ref cause) => cause,
            EnableAWSServiceAccessError::Service(ref cause) => cause,
            EnableAWSServiceAccessError::TooManyRequests(ref cause) => cause,
            EnableAWSServiceAccessError::Validation(ref cause) => cause,
            EnableAWSServiceAccessError::Credentials(ref err) => err.description(),
            EnableAWSServiceAccessError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableAWSServiceAccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableAllFeatures
#[derive(Debug, PartialEq)]
pub enum EnableAllFeaturesError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>ALREADY<em>IN</em>AN<em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>ORGANIZATION</em>ALREADY<em>HAS</em>ALL<em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You cannot issue new invitations to join an organization while it is in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You cannot complete the operation with an account that does not have a payment instrument, such as a credit card, associated with it.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableAllFeaturesError {
    pub fn from_body(body: &str) -> EnableAllFeaturesError {
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
                    "AWSOrganizationsNotInUseException" => {
                        EnableAllFeaturesError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        EnableAllFeaturesError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        EnableAllFeaturesError::ConcurrentModification(String::from(error_message))
                    }
                    "HandshakeConstraintViolationException" => {
                        EnableAllFeaturesError::HandshakeConstraintViolation(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        EnableAllFeaturesError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        EnableAllFeaturesError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        EnableAllFeaturesError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableAllFeaturesError::Validation(error_message.to_string())
                    }
                    _ => EnableAllFeaturesError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableAllFeaturesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableAllFeaturesError {
    fn from(err: serde_json::error::Error) -> EnableAllFeaturesError {
        EnableAllFeaturesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableAllFeaturesError {
    fn from(err: CredentialsError) -> EnableAllFeaturesError {
        EnableAllFeaturesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableAllFeaturesError {
    fn from(err: HttpDispatchError) -> EnableAllFeaturesError {
        EnableAllFeaturesError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableAllFeaturesError {
    fn from(err: io::Error) -> EnableAllFeaturesError {
        EnableAllFeaturesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableAllFeaturesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAllFeaturesError {
    fn description(&self) -> &str {
        match *self {
            EnableAllFeaturesError::AWSOrganizationsNotInUse(ref cause) => cause,
            EnableAllFeaturesError::AccessDenied(ref cause) => cause,
            EnableAllFeaturesError::ConcurrentModification(ref cause) => cause,
            EnableAllFeaturesError::HandshakeConstraintViolation(ref cause) => cause,
            EnableAllFeaturesError::InvalidInput(ref cause) => cause,
            EnableAllFeaturesError::Service(ref cause) => cause,
            EnableAllFeaturesError::TooManyRequests(ref cause) => cause,
            EnableAllFeaturesError::Validation(ref cause) => cause,
            EnableAllFeaturesError::Credentials(ref err) => err.description(),
            EnableAllFeaturesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableAllFeaturesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnablePolicyType
#[derive(Debug, PartialEq)]
pub enum EnablePolicyTypeError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The specified policy type is already enabled in the specified root.</p>
    PolicyTypeAlreadyEnabled(String),
    /// <p>You can't use the specified policy type with the feature set currently enabled for this organization. For example, you can enable service control policies (SCPs) only after you enable all features in the organization. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html#enable_policies_on_root">Enabling and Disabling a Policy Type on a Root</a> in the <i>AWS Organizations User Guide</i>.</p>
    PolicyTypeNotAvailableForOrganization(String),
    /// <p>We can't find a root with the RootId that you specified.</p>
    RootNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnablePolicyTypeError {
    pub fn from_body(body: &str) -> EnablePolicyTypeError {
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
                    "AWSOrganizationsNotInUseException" => {
                        EnablePolicyTypeError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        EnablePolicyTypeError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        EnablePolicyTypeError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        EnablePolicyTypeError::ConstraintViolation(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        EnablePolicyTypeError::InvalidInput(String::from(error_message))
                    }
                    "PolicyTypeAlreadyEnabledException" => {
                        EnablePolicyTypeError::PolicyTypeAlreadyEnabled(String::from(error_message))
                    }
                    "PolicyTypeNotAvailableForOrganizationException" => {
                        EnablePolicyTypeError::PolicyTypeNotAvailableForOrganization(String::from(
                            error_message,
                        ))
                    }
                    "RootNotFoundException" => {
                        EnablePolicyTypeError::RootNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        EnablePolicyTypeError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        EnablePolicyTypeError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnablePolicyTypeError::Validation(error_message.to_string())
                    }
                    _ => EnablePolicyTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnablePolicyTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnablePolicyTypeError {
    fn from(err: serde_json::error::Error) -> EnablePolicyTypeError {
        EnablePolicyTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnablePolicyTypeError {
    fn from(err: CredentialsError) -> EnablePolicyTypeError {
        EnablePolicyTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnablePolicyTypeError {
    fn from(err: HttpDispatchError) -> EnablePolicyTypeError {
        EnablePolicyTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnablePolicyTypeError {
    fn from(err: io::Error) -> EnablePolicyTypeError {
        EnablePolicyTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnablePolicyTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnablePolicyTypeError {
    fn description(&self) -> &str {
        match *self {
            EnablePolicyTypeError::AWSOrganizationsNotInUse(ref cause) => cause,
            EnablePolicyTypeError::AccessDenied(ref cause) => cause,
            EnablePolicyTypeError::ConcurrentModification(ref cause) => cause,
            EnablePolicyTypeError::ConstraintViolation(ref cause) => cause,
            EnablePolicyTypeError::InvalidInput(ref cause) => cause,
            EnablePolicyTypeError::PolicyTypeAlreadyEnabled(ref cause) => cause,
            EnablePolicyTypeError::PolicyTypeNotAvailableForOrganization(ref cause) => cause,
            EnablePolicyTypeError::RootNotFound(ref cause) => cause,
            EnablePolicyTypeError::Service(ref cause) => cause,
            EnablePolicyTypeError::TooManyRequests(ref cause) => cause,
            EnablePolicyTypeError::Validation(ref cause) => cause,
            EnablePolicyTypeError::Credentials(ref err) => err.description(),
            EnablePolicyTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnablePolicyTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InviteAccountToOrganization
#[derive(Debug, PartialEq)]
pub enum InviteAccountToOrganizationError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>A handshake with the same action and target already exists. For example, if you invited an account to join your organization, the invited account might already have a pending invitation from this organization. If you intend to resend an invitation to an account, ensure that existing handshakes that might be considered duplicates are canceled or declined.</p>
    DuplicateHandshake(String),
    /// <p>AWS Organizations could not finalize the creation of your organization. Try again later. If this persists, contact AWS customer support.</p>
    FinalizingOrganization(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>ALREADY<em>IN</em>AN<em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>ORGANIZATION</em>ALREADY<em>HAS</em>ALL<em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You cannot issue new invitations to join an organization while it is in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You cannot complete the operation with an account that does not have a payment instrument, such as a credit card, associated with it.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InviteAccountToOrganizationError {
    pub fn from_body(body: &str) -> InviteAccountToOrganizationError {
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
                    "AWSOrganizationsNotInUseException" => {
                        InviteAccountToOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        InviteAccountToOrganizationError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        InviteAccountToOrganizationError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "DuplicateHandshakeException" => {
                        InviteAccountToOrganizationError::DuplicateHandshake(String::from(
                            error_message,
                        ))
                    }
                    "FinalizingOrganizationException" => {
                        InviteAccountToOrganizationError::FinalizingOrganization(String::from(
                            error_message,
                        ))
                    }
                    "HandshakeConstraintViolationException" => {
                        InviteAccountToOrganizationError::HandshakeConstraintViolation(
                            String::from(error_message),
                        )
                    }
                    "InvalidInputException" => {
                        InviteAccountToOrganizationError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        InviteAccountToOrganizationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        InviteAccountToOrganizationError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        InviteAccountToOrganizationError::Validation(error_message.to_string())
                    }
                    _ => InviteAccountToOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => InviteAccountToOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InviteAccountToOrganizationError {
    fn from(err: serde_json::error::Error) -> InviteAccountToOrganizationError {
        InviteAccountToOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InviteAccountToOrganizationError {
    fn from(err: CredentialsError) -> InviteAccountToOrganizationError {
        InviteAccountToOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InviteAccountToOrganizationError {
    fn from(err: HttpDispatchError) -> InviteAccountToOrganizationError {
        InviteAccountToOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for InviteAccountToOrganizationError {
    fn from(err: io::Error) -> InviteAccountToOrganizationError {
        InviteAccountToOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InviteAccountToOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InviteAccountToOrganizationError {
    fn description(&self) -> &str {
        match *self {
            InviteAccountToOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            InviteAccountToOrganizationError::AccessDenied(ref cause) => cause,
            InviteAccountToOrganizationError::ConcurrentModification(ref cause) => cause,
            InviteAccountToOrganizationError::DuplicateHandshake(ref cause) => cause,
            InviteAccountToOrganizationError::FinalizingOrganization(ref cause) => cause,
            InviteAccountToOrganizationError::HandshakeConstraintViolation(ref cause) => cause,
            InviteAccountToOrganizationError::InvalidInput(ref cause) => cause,
            InviteAccountToOrganizationError::Service(ref cause) => cause,
            InviteAccountToOrganizationError::TooManyRequests(ref cause) => cause,
            InviteAccountToOrganizationError::Validation(ref cause) => cause,
            InviteAccountToOrganizationError::Credentials(ref err) => err.description(),
            InviteAccountToOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InviteAccountToOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by LeaveOrganization
#[derive(Debug, PartialEq)]
pub enum LeaveOrganizationError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the AccountId that you specified, or the account whose credentials you used to make this request is not a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>You can't remove a master account from an organization. If you want the master account to become a member account in another organization, you must first delete the current organization of the master account.</p>
    MasterCannotLeaveOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl LeaveOrganizationError {
    pub fn from_body(body: &str) -> LeaveOrganizationError {
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
                    "AWSOrganizationsNotInUseException" => {
                        LeaveOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        LeaveOrganizationError::AccessDenied(String::from(error_message))
                    }
                    "AccountNotFoundException" => {
                        LeaveOrganizationError::AccountNotFound(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        LeaveOrganizationError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        LeaveOrganizationError::ConstraintViolation(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        LeaveOrganizationError::InvalidInput(String::from(error_message))
                    }
                    "MasterCannotLeaveOrganizationException" => {
                        LeaveOrganizationError::MasterCannotLeaveOrganization(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        LeaveOrganizationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        LeaveOrganizationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        LeaveOrganizationError::Validation(error_message.to_string())
                    }
                    _ => LeaveOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => LeaveOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for LeaveOrganizationError {
    fn from(err: serde_json::error::Error) -> LeaveOrganizationError {
        LeaveOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for LeaveOrganizationError {
    fn from(err: CredentialsError) -> LeaveOrganizationError {
        LeaveOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for LeaveOrganizationError {
    fn from(err: HttpDispatchError) -> LeaveOrganizationError {
        LeaveOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for LeaveOrganizationError {
    fn from(err: io::Error) -> LeaveOrganizationError {
        LeaveOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for LeaveOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LeaveOrganizationError {
    fn description(&self) -> &str {
        match *self {
            LeaveOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            LeaveOrganizationError::AccessDenied(ref cause) => cause,
            LeaveOrganizationError::AccountNotFound(ref cause) => cause,
            LeaveOrganizationError::ConcurrentModification(ref cause) => cause,
            LeaveOrganizationError::ConstraintViolation(ref cause) => cause,
            LeaveOrganizationError::InvalidInput(ref cause) => cause,
            LeaveOrganizationError::MasterCannotLeaveOrganization(ref cause) => cause,
            LeaveOrganizationError::Service(ref cause) => cause,
            LeaveOrganizationError::TooManyRequests(ref cause) => cause,
            LeaveOrganizationError::Validation(ref cause) => cause,
            LeaveOrganizationError::Credentials(ref err) => err.description(),
            LeaveOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            LeaveOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAWSServiceAccessForOrganization
#[derive(Debug, PartialEq)]
pub enum ListAWSServiceAccessForOrganizationError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAWSServiceAccessForOrganizationError {
    pub fn from_body(body: &str) -> ListAWSServiceAccessForOrganizationError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListAWSServiceAccessForOrganizationError::AWSOrganizationsNotInUse(
                            String::from(error_message),
                        )
                    }
                    "AccessDeniedException" => {
                        ListAWSServiceAccessForOrganizationError::AccessDenied(String::from(
                            error_message,
                        ))
                    }
                    "ConstraintViolationException" => {
                        ListAWSServiceAccessForOrganizationError::ConstraintViolation(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        ListAWSServiceAccessForOrganizationError::InvalidInput(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => ListAWSServiceAccessForOrganizationError::Service(
                        String::from(error_message),
                    ),
                    "TooManyRequestsException" => {
                        ListAWSServiceAccessForOrganizationError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => ListAWSServiceAccessForOrganizationError::Validation(
                        error_message.to_string(),
                    ),
                    _ => ListAWSServiceAccessForOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAWSServiceAccessForOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAWSServiceAccessForOrganizationError {
    fn from(err: serde_json::error::Error) -> ListAWSServiceAccessForOrganizationError {
        ListAWSServiceAccessForOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAWSServiceAccessForOrganizationError {
    fn from(err: CredentialsError) -> ListAWSServiceAccessForOrganizationError {
        ListAWSServiceAccessForOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAWSServiceAccessForOrganizationError {
    fn from(err: HttpDispatchError) -> ListAWSServiceAccessForOrganizationError {
        ListAWSServiceAccessForOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAWSServiceAccessForOrganizationError {
    fn from(err: io::Error) -> ListAWSServiceAccessForOrganizationError {
        ListAWSServiceAccessForOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAWSServiceAccessForOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAWSServiceAccessForOrganizationError {
    fn description(&self) -> &str {
        match *self {
            ListAWSServiceAccessForOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::AccessDenied(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::ConstraintViolation(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::InvalidInput(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::Service(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::TooManyRequests(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::Validation(ref cause) => cause,
            ListAWSServiceAccessForOrganizationError::Credentials(ref err) => err.description(),
            ListAWSServiceAccessForOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAWSServiceAccessForOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAccounts
#[derive(Debug, PartialEq)]
pub enum ListAccountsError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAccountsError {
    pub fn from_body(body: &str) -> ListAccountsError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListAccountsError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        ListAccountsError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListAccountsError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => ListAccountsError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListAccountsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAccountsError::Validation(error_message.to_string())
                    }
                    _ => ListAccountsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAccountsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAccountsError {
    fn from(err: serde_json::error::Error) -> ListAccountsError {
        ListAccountsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAccountsError {
    fn from(err: CredentialsError) -> ListAccountsError {
        ListAccountsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAccountsError {
    fn from(err: HttpDispatchError) -> ListAccountsError {
        ListAccountsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAccountsError {
    fn from(err: io::Error) -> ListAccountsError {
        ListAccountsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAccountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAccountsError {
    fn description(&self) -> &str {
        match *self {
            ListAccountsError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListAccountsError::AccessDenied(ref cause) => cause,
            ListAccountsError::InvalidInput(ref cause) => cause,
            ListAccountsError::Service(ref cause) => cause,
            ListAccountsError::TooManyRequests(ref cause) => cause,
            ListAccountsError::Validation(ref cause) => cause,
            ListAccountsError::Credentials(ref err) => err.description(),
            ListAccountsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAccountsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAccountsForParent
#[derive(Debug, PartialEq)]
pub enum ListAccountsForParentError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or organizational unit (OU) with the ParentId that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAccountsForParentError {
    pub fn from_body(body: &str) -> ListAccountsForParentError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListAccountsForParentError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        ListAccountsForParentError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListAccountsForParentError::InvalidInput(String::from(error_message))
                    }
                    "ParentNotFoundException" => {
                        ListAccountsForParentError::ParentNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListAccountsForParentError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListAccountsForParentError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAccountsForParentError::Validation(error_message.to_string())
                    }
                    _ => ListAccountsForParentError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAccountsForParentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAccountsForParentError {
    fn from(err: serde_json::error::Error) -> ListAccountsForParentError {
        ListAccountsForParentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAccountsForParentError {
    fn from(err: CredentialsError) -> ListAccountsForParentError {
        ListAccountsForParentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAccountsForParentError {
    fn from(err: HttpDispatchError) -> ListAccountsForParentError {
        ListAccountsForParentError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAccountsForParentError {
    fn from(err: io::Error) -> ListAccountsForParentError {
        ListAccountsForParentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAccountsForParentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAccountsForParentError {
    fn description(&self) -> &str {
        match *self {
            ListAccountsForParentError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListAccountsForParentError::AccessDenied(ref cause) => cause,
            ListAccountsForParentError::InvalidInput(ref cause) => cause,
            ListAccountsForParentError::ParentNotFound(ref cause) => cause,
            ListAccountsForParentError::Service(ref cause) => cause,
            ListAccountsForParentError::TooManyRequests(ref cause) => cause,
            ListAccountsForParentError::Validation(ref cause) => cause,
            ListAccountsForParentError::Credentials(ref err) => err.description(),
            ListAccountsForParentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAccountsForParentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListChildren
#[derive(Debug, PartialEq)]
pub enum ListChildrenError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or organizational unit (OU) with the ParentId that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListChildrenError {
    pub fn from_body(body: &str) -> ListChildrenError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListChildrenError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        ListChildrenError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListChildrenError::InvalidInput(String::from(error_message))
                    }
                    "ParentNotFoundException" => {
                        ListChildrenError::ParentNotFound(String::from(error_message))
                    }
                    "ServiceException" => ListChildrenError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListChildrenError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListChildrenError::Validation(error_message.to_string())
                    }
                    _ => ListChildrenError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListChildrenError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListChildrenError {
    fn from(err: serde_json::error::Error) -> ListChildrenError {
        ListChildrenError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListChildrenError {
    fn from(err: CredentialsError) -> ListChildrenError {
        ListChildrenError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListChildrenError {
    fn from(err: HttpDispatchError) -> ListChildrenError {
        ListChildrenError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListChildrenError {
    fn from(err: io::Error) -> ListChildrenError {
        ListChildrenError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListChildrenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListChildrenError {
    fn description(&self) -> &str {
        match *self {
            ListChildrenError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListChildrenError::AccessDenied(ref cause) => cause,
            ListChildrenError::InvalidInput(ref cause) => cause,
            ListChildrenError::ParentNotFound(ref cause) => cause,
            ListChildrenError::Service(ref cause) => cause,
            ListChildrenError::TooManyRequests(ref cause) => cause,
            ListChildrenError::Validation(ref cause) => cause,
            ListChildrenError::Credentials(ref err) => err.description(),
            ListChildrenError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListChildrenError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCreateAccountStatus
#[derive(Debug, PartialEq)]
pub enum ListCreateAccountStatusError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCreateAccountStatusError {
    pub fn from_body(body: &str) -> ListCreateAccountStatusError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListCreateAccountStatusError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        ListCreateAccountStatusError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListCreateAccountStatusError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListCreateAccountStatusError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListCreateAccountStatusError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCreateAccountStatusError::Validation(error_message.to_string())
                    }
                    _ => ListCreateAccountStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCreateAccountStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCreateAccountStatusError {
    fn from(err: serde_json::error::Error) -> ListCreateAccountStatusError {
        ListCreateAccountStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCreateAccountStatusError {
    fn from(err: CredentialsError) -> ListCreateAccountStatusError {
        ListCreateAccountStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCreateAccountStatusError {
    fn from(err: HttpDispatchError) -> ListCreateAccountStatusError {
        ListCreateAccountStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCreateAccountStatusError {
    fn from(err: io::Error) -> ListCreateAccountStatusError {
        ListCreateAccountStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCreateAccountStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCreateAccountStatusError {
    fn description(&self) -> &str {
        match *self {
            ListCreateAccountStatusError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListCreateAccountStatusError::AccessDenied(ref cause) => cause,
            ListCreateAccountStatusError::InvalidInput(ref cause) => cause,
            ListCreateAccountStatusError::Service(ref cause) => cause,
            ListCreateAccountStatusError::TooManyRequests(ref cause) => cause,
            ListCreateAccountStatusError::Validation(ref cause) => cause,
            ListCreateAccountStatusError::Credentials(ref err) => err.description(),
            ListCreateAccountStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCreateAccountStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHandshakesForAccount
#[derive(Debug, PartialEq)]
pub enum ListHandshakesForAccountError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListHandshakesForAccountError {
    pub fn from_body(body: &str) -> ListHandshakesForAccountError {
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
                    "AccessDeniedException" => {
                        ListHandshakesForAccountError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        ListHandshakesForAccountError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        ListHandshakesForAccountError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListHandshakesForAccountError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListHandshakesForAccountError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListHandshakesForAccountError::Validation(error_message.to_string())
                    }
                    _ => ListHandshakesForAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListHandshakesForAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListHandshakesForAccountError {
    fn from(err: serde_json::error::Error) -> ListHandshakesForAccountError {
        ListHandshakesForAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListHandshakesForAccountError {
    fn from(err: CredentialsError) -> ListHandshakesForAccountError {
        ListHandshakesForAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHandshakesForAccountError {
    fn from(err: HttpDispatchError) -> ListHandshakesForAccountError {
        ListHandshakesForAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHandshakesForAccountError {
    fn from(err: io::Error) -> ListHandshakesForAccountError {
        ListHandshakesForAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHandshakesForAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHandshakesForAccountError {
    fn description(&self) -> &str {
        match *self {
            ListHandshakesForAccountError::AccessDenied(ref cause) => cause,
            ListHandshakesForAccountError::ConcurrentModification(ref cause) => cause,
            ListHandshakesForAccountError::InvalidInput(ref cause) => cause,
            ListHandshakesForAccountError::Service(ref cause) => cause,
            ListHandshakesForAccountError::TooManyRequests(ref cause) => cause,
            ListHandshakesForAccountError::Validation(ref cause) => cause,
            ListHandshakesForAccountError::Credentials(ref err) => err.description(),
            ListHandshakesForAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListHandshakesForAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHandshakesForOrganization
#[derive(Debug, PartialEq)]
pub enum ListHandshakesForOrganizationError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListHandshakesForOrganizationError {
    pub fn from_body(body: &str) -> ListHandshakesForOrganizationError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListHandshakesForOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => ListHandshakesForOrganizationError::AccessDenied(
                        String::from(error_message),
                    ),
                    "ConcurrentModificationException" => {
                        ListHandshakesForOrganizationError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => ListHandshakesForOrganizationError::InvalidInput(
                        String::from(error_message),
                    ),
                    "ServiceException" => {
                        ListHandshakesForOrganizationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListHandshakesForOrganizationError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListHandshakesForOrganizationError::Validation(error_message.to_string())
                    }
                    _ => ListHandshakesForOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListHandshakesForOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListHandshakesForOrganizationError {
    fn from(err: serde_json::error::Error) -> ListHandshakesForOrganizationError {
        ListHandshakesForOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListHandshakesForOrganizationError {
    fn from(err: CredentialsError) -> ListHandshakesForOrganizationError {
        ListHandshakesForOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHandshakesForOrganizationError {
    fn from(err: HttpDispatchError) -> ListHandshakesForOrganizationError {
        ListHandshakesForOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHandshakesForOrganizationError {
    fn from(err: io::Error) -> ListHandshakesForOrganizationError {
        ListHandshakesForOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHandshakesForOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHandshakesForOrganizationError {
    fn description(&self) -> &str {
        match *self {
            ListHandshakesForOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListHandshakesForOrganizationError::AccessDenied(ref cause) => cause,
            ListHandshakesForOrganizationError::ConcurrentModification(ref cause) => cause,
            ListHandshakesForOrganizationError::InvalidInput(ref cause) => cause,
            ListHandshakesForOrganizationError::Service(ref cause) => cause,
            ListHandshakesForOrganizationError::TooManyRequests(ref cause) => cause,
            ListHandshakesForOrganizationError::Validation(ref cause) => cause,
            ListHandshakesForOrganizationError::Credentials(ref err) => err.description(),
            ListHandshakesForOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListHandshakesForOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOrganizationalUnitsForParent
#[derive(Debug, PartialEq)]
pub enum ListOrganizationalUnitsForParentError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or organizational unit (OU) with the ParentId that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOrganizationalUnitsForParentError {
    pub fn from_body(body: &str) -> ListOrganizationalUnitsForParentError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListOrganizationalUnitsForParentError::AWSOrganizationsNotInUse(
                            String::from(error_message),
                        )
                    }
                    "AccessDeniedException" => ListOrganizationalUnitsForParentError::AccessDenied(
                        String::from(error_message),
                    ),
                    "InvalidInputException" => ListOrganizationalUnitsForParentError::InvalidInput(
                        String::from(error_message),
                    ),
                    "ParentNotFoundException" => {
                        ListOrganizationalUnitsForParentError::ParentNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        ListOrganizationalUnitsForParentError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListOrganizationalUnitsForParentError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListOrganizationalUnitsForParentError::Validation(error_message.to_string())
                    }
                    _ => ListOrganizationalUnitsForParentError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOrganizationalUnitsForParentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOrganizationalUnitsForParentError {
    fn from(err: serde_json::error::Error) -> ListOrganizationalUnitsForParentError {
        ListOrganizationalUnitsForParentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOrganizationalUnitsForParentError {
    fn from(err: CredentialsError) -> ListOrganizationalUnitsForParentError {
        ListOrganizationalUnitsForParentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOrganizationalUnitsForParentError {
    fn from(err: HttpDispatchError) -> ListOrganizationalUnitsForParentError {
        ListOrganizationalUnitsForParentError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOrganizationalUnitsForParentError {
    fn from(err: io::Error) -> ListOrganizationalUnitsForParentError {
        ListOrganizationalUnitsForParentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOrganizationalUnitsForParentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOrganizationalUnitsForParentError {
    fn description(&self) -> &str {
        match *self {
            ListOrganizationalUnitsForParentError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListOrganizationalUnitsForParentError::AccessDenied(ref cause) => cause,
            ListOrganizationalUnitsForParentError::InvalidInput(ref cause) => cause,
            ListOrganizationalUnitsForParentError::ParentNotFound(ref cause) => cause,
            ListOrganizationalUnitsForParentError::Service(ref cause) => cause,
            ListOrganizationalUnitsForParentError::TooManyRequests(ref cause) => cause,
            ListOrganizationalUnitsForParentError::Validation(ref cause) => cause,
            ListOrganizationalUnitsForParentError::Credentials(ref err) => err.description(),
            ListOrganizationalUnitsForParentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOrganizationalUnitsForParentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListParents
#[derive(Debug, PartialEq)]
pub enum ListParentsError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>We can't find an organizational unit (OU) or AWS account with the ChildId that you specified.</p>
    ChildNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListParentsError {
    pub fn from_body(body: &str) -> ListParentsError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListParentsError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        ListParentsError::AccessDenied(String::from(error_message))
                    }
                    "ChildNotFoundException" => {
                        ListParentsError::ChildNotFound(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListParentsError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => ListParentsError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListParentsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListParentsError::Validation(error_message.to_string())
                    }
                    _ => ListParentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListParentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListParentsError {
    fn from(err: serde_json::error::Error) -> ListParentsError {
        ListParentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListParentsError {
    fn from(err: CredentialsError) -> ListParentsError {
        ListParentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListParentsError {
    fn from(err: HttpDispatchError) -> ListParentsError {
        ListParentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListParentsError {
    fn from(err: io::Error) -> ListParentsError {
        ListParentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListParentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListParentsError {
    fn description(&self) -> &str {
        match *self {
            ListParentsError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListParentsError::AccessDenied(ref cause) => cause,
            ListParentsError::ChildNotFound(ref cause) => cause,
            ListParentsError::InvalidInput(ref cause) => cause,
            ListParentsError::Service(ref cause) => cause,
            ListParentsError::TooManyRequests(ref cause) => cause,
            ListParentsError::Validation(ref cause) => cause,
            ListParentsError::Credentials(ref err) => err.description(),
            ListParentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListParentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicies
#[derive(Debug, PartialEq)]
pub enum ListPoliciesError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
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
                    "AWSOrganizationsNotInUseException" => {
                        ListPoliciesError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        ListPoliciesError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListPoliciesError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => ListPoliciesError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListPoliciesError::TooManyRequests(String::from(error_message))
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
            ListPoliciesError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListPoliciesError::AccessDenied(ref cause) => cause,
            ListPoliciesError::InvalidInput(ref cause) => cause,
            ListPoliciesError::Service(ref cause) => cause,
            ListPoliciesError::TooManyRequests(ref cause) => cause,
            ListPoliciesError::Validation(ref cause) => cause,
            ListPoliciesError::Credentials(ref err) => err.description(),
            ListPoliciesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPoliciesForTarget
#[derive(Debug, PartialEq)]
pub enum ListPoliciesForTargetError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, or account with the TargetId that you specified.</p>
    TargetNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPoliciesForTargetError {
    pub fn from_body(body: &str) -> ListPoliciesForTargetError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListPoliciesForTargetError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        ListPoliciesForTargetError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListPoliciesForTargetError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListPoliciesForTargetError::Service(String::from(error_message))
                    }
                    "TargetNotFoundException" => {
                        ListPoliciesForTargetError::TargetNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListPoliciesForTargetError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPoliciesForTargetError::Validation(error_message.to_string())
                    }
                    _ => ListPoliciesForTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPoliciesForTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPoliciesForTargetError {
    fn from(err: serde_json::error::Error) -> ListPoliciesForTargetError {
        ListPoliciesForTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPoliciesForTargetError {
    fn from(err: CredentialsError) -> ListPoliciesForTargetError {
        ListPoliciesForTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPoliciesForTargetError {
    fn from(err: HttpDispatchError) -> ListPoliciesForTargetError {
        ListPoliciesForTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPoliciesForTargetError {
    fn from(err: io::Error) -> ListPoliciesForTargetError {
        ListPoliciesForTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPoliciesForTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPoliciesForTargetError {
    fn description(&self) -> &str {
        match *self {
            ListPoliciesForTargetError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListPoliciesForTargetError::AccessDenied(ref cause) => cause,
            ListPoliciesForTargetError::InvalidInput(ref cause) => cause,
            ListPoliciesForTargetError::Service(ref cause) => cause,
            ListPoliciesForTargetError::TargetNotFound(ref cause) => cause,
            ListPoliciesForTargetError::TooManyRequests(ref cause) => cause,
            ListPoliciesForTargetError::Validation(ref cause) => cause,
            ListPoliciesForTargetError::Credentials(ref err) => err.description(),
            ListPoliciesForTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPoliciesForTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRoots
#[derive(Debug, PartialEq)]
pub enum ListRootsError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListRootsError {
    pub fn from_body(body: &str) -> ListRootsError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListRootsError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        ListRootsError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListRootsError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => ListRootsError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListRootsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => ListRootsError::Validation(error_message.to_string()),
                    _ => ListRootsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRootsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRootsError {
    fn from(err: serde_json::error::Error) -> ListRootsError {
        ListRootsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRootsError {
    fn from(err: CredentialsError) -> ListRootsError {
        ListRootsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRootsError {
    fn from(err: HttpDispatchError) -> ListRootsError {
        ListRootsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRootsError {
    fn from(err: io::Error) -> ListRootsError {
        ListRootsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRootsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRootsError {
    fn description(&self) -> &str {
        match *self {
            ListRootsError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListRootsError::AccessDenied(ref cause) => cause,
            ListRootsError::InvalidInput(ref cause) => cause,
            ListRootsError::Service(ref cause) => cause,
            ListRootsError::TooManyRequests(ref cause) => cause,
            ListRootsError::Validation(ref cause) => cause,
            ListRootsError::Credentials(ref err) => err.description(),
            ListRootsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRootsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTargetsForPolicy
#[derive(Debug, PartialEq)]
pub enum ListTargetsForPolicyError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the PolicyId that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTargetsForPolicyError {
    pub fn from_body(body: &str) -> ListTargetsForPolicyError {
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
                    "AWSOrganizationsNotInUseException" => {
                        ListTargetsForPolicyError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        ListTargetsForPolicyError::AccessDenied(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListTargetsForPolicyError::InvalidInput(String::from(error_message))
                    }
                    "PolicyNotFoundException" => {
                        ListTargetsForPolicyError::PolicyNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListTargetsForPolicyError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListTargetsForPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTargetsForPolicyError::Validation(error_message.to_string())
                    }
                    _ => ListTargetsForPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTargetsForPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTargetsForPolicyError {
    fn from(err: serde_json::error::Error) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTargetsForPolicyError {
    fn from(err: CredentialsError) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTargetsForPolicyError {
    fn from(err: HttpDispatchError) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTargetsForPolicyError {
    fn from(err: io::Error) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTargetsForPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTargetsForPolicyError {
    fn description(&self) -> &str {
        match *self {
            ListTargetsForPolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            ListTargetsForPolicyError::AccessDenied(ref cause) => cause,
            ListTargetsForPolicyError::InvalidInput(ref cause) => cause,
            ListTargetsForPolicyError::PolicyNotFound(ref cause) => cause,
            ListTargetsForPolicyError::Service(ref cause) => cause,
            ListTargetsForPolicyError::TooManyRequests(ref cause) => cause,
            ListTargetsForPolicyError::Validation(ref cause) => cause,
            ListTargetsForPolicyError::Credentials(ref err) => err.description(),
            ListTargetsForPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTargetsForPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by MoveAccount
#[derive(Debug, PartialEq)]
pub enum MoveAccountError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the AccountId that you specified, or the account whose credentials you used to make this request is not a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>We can't find the destination container (a root or OU) with the ParentId that you specified.</p>
    DestinationParentNotFound(String),
    /// <p>That account is already present in the specified destination.</p>
    DuplicateAccount(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a source root or OU with the ParentId that you specified.</p>
    SourceParentNotFound(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl MoveAccountError {
    pub fn from_body(body: &str) -> MoveAccountError {
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
                    "AWSOrganizationsNotInUseException" => {
                        MoveAccountError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        MoveAccountError::AccessDenied(String::from(error_message))
                    }
                    "AccountNotFoundException" => {
                        MoveAccountError::AccountNotFound(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        MoveAccountError::ConcurrentModification(String::from(error_message))
                    }
                    "DestinationParentNotFoundException" => {
                        MoveAccountError::DestinationParentNotFound(String::from(error_message))
                    }
                    "DuplicateAccountException" => {
                        MoveAccountError::DuplicateAccount(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        MoveAccountError::InvalidInput(String::from(error_message))
                    }
                    "ServiceException" => MoveAccountError::Service(String::from(error_message)),
                    "SourceParentNotFoundException" => {
                        MoveAccountError::SourceParentNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        MoveAccountError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        MoveAccountError::Validation(error_message.to_string())
                    }
                    _ => MoveAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => MoveAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for MoveAccountError {
    fn from(err: serde_json::error::Error) -> MoveAccountError {
        MoveAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for MoveAccountError {
    fn from(err: CredentialsError) -> MoveAccountError {
        MoveAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for MoveAccountError {
    fn from(err: HttpDispatchError) -> MoveAccountError {
        MoveAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for MoveAccountError {
    fn from(err: io::Error) -> MoveAccountError {
        MoveAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for MoveAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for MoveAccountError {
    fn description(&self) -> &str {
        match *self {
            MoveAccountError::AWSOrganizationsNotInUse(ref cause) => cause,
            MoveAccountError::AccessDenied(ref cause) => cause,
            MoveAccountError::AccountNotFound(ref cause) => cause,
            MoveAccountError::ConcurrentModification(ref cause) => cause,
            MoveAccountError::DestinationParentNotFound(ref cause) => cause,
            MoveAccountError::DuplicateAccount(ref cause) => cause,
            MoveAccountError::InvalidInput(ref cause) => cause,
            MoveAccountError::Service(ref cause) => cause,
            MoveAccountError::SourceParentNotFound(ref cause) => cause,
            MoveAccountError::TooManyRequests(ref cause) => cause,
            MoveAccountError::Validation(ref cause) => cause,
            MoveAccountError::Credentials(ref err) => err.description(),
            MoveAccountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            MoveAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveAccountFromOrganization
#[derive(Debug, PartialEq)]
pub enum RemoveAccountFromOrganizationError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the AccountId that you specified, or the account whose credentials you used to make this request is not a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>You can't remove a master account from an organization. If you want the master account to become a member account in another organization, you must first delete the current organization of the master account.</p>
    MasterCannotLeaveOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveAccountFromOrganizationError {
    pub fn from_body(body: &str) -> RemoveAccountFromOrganizationError {
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
                    "AWSOrganizationsNotInUseException" => {
                        RemoveAccountFromOrganizationError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => RemoveAccountFromOrganizationError::AccessDenied(
                        String::from(error_message),
                    ),
                    "AccountNotFoundException" => {
                        RemoveAccountFromOrganizationError::AccountNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ConcurrentModificationException" => {
                        RemoveAccountFromOrganizationError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "ConstraintViolationException" => {
                        RemoveAccountFromOrganizationError::ConstraintViolation(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => RemoveAccountFromOrganizationError::InvalidInput(
                        String::from(error_message),
                    ),
                    "MasterCannotLeaveOrganizationException" => {
                        RemoveAccountFromOrganizationError::MasterCannotLeaveOrganization(
                            String::from(error_message),
                        )
                    }
                    "ServiceException" => {
                        RemoveAccountFromOrganizationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        RemoveAccountFromOrganizationError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RemoveAccountFromOrganizationError::Validation(error_message.to_string())
                    }
                    _ => RemoveAccountFromOrganizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveAccountFromOrganizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveAccountFromOrganizationError {
    fn from(err: serde_json::error::Error) -> RemoveAccountFromOrganizationError {
        RemoveAccountFromOrganizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveAccountFromOrganizationError {
    fn from(err: CredentialsError) -> RemoveAccountFromOrganizationError {
        RemoveAccountFromOrganizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveAccountFromOrganizationError {
    fn from(err: HttpDispatchError) -> RemoveAccountFromOrganizationError {
        RemoveAccountFromOrganizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveAccountFromOrganizationError {
    fn from(err: io::Error) -> RemoveAccountFromOrganizationError {
        RemoveAccountFromOrganizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveAccountFromOrganizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAccountFromOrganizationError {
    fn description(&self) -> &str {
        match *self {
            RemoveAccountFromOrganizationError::AWSOrganizationsNotInUse(ref cause) => cause,
            RemoveAccountFromOrganizationError::AccessDenied(ref cause) => cause,
            RemoveAccountFromOrganizationError::AccountNotFound(ref cause) => cause,
            RemoveAccountFromOrganizationError::ConcurrentModification(ref cause) => cause,
            RemoveAccountFromOrganizationError::ConstraintViolation(ref cause) => cause,
            RemoveAccountFromOrganizationError::InvalidInput(ref cause) => cause,
            RemoveAccountFromOrganizationError::MasterCannotLeaveOrganization(ref cause) => cause,
            RemoveAccountFromOrganizationError::Service(ref cause) => cause,
            RemoveAccountFromOrganizationError::TooManyRequests(ref cause) => cause,
            RemoveAccountFromOrganizationError::Validation(ref cause) => cause,
            RemoveAccountFromOrganizationError::Credentials(ref err) => err.description(),
            RemoveAccountFromOrganizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveAccountFromOrganizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum UpdateOrganizationalUnitError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>An organizational unit (OU) with the same name already exists.</p>
    DuplicateOrganizationalUnit(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find an organizational unit (OU) with the OrganizationalUnitId that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateOrganizationalUnitError {
    pub fn from_body(body: &str) -> UpdateOrganizationalUnitError {
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
                    "AWSOrganizationsNotInUseException" => {
                        UpdateOrganizationalUnitError::AWSOrganizationsNotInUse(String::from(
                            error_message,
                        ))
                    }
                    "AccessDeniedException" => {
                        UpdateOrganizationalUnitError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        UpdateOrganizationalUnitError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "DuplicateOrganizationalUnitException" => {
                        UpdateOrganizationalUnitError::DuplicateOrganizationalUnit(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        UpdateOrganizationalUnitError::InvalidInput(String::from(error_message))
                    }
                    "OrganizationalUnitNotFoundException" => {
                        UpdateOrganizationalUnitError::OrganizationalUnitNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        UpdateOrganizationalUnitError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateOrganizationalUnitError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateOrganizationalUnitError::Validation(error_message.to_string())
                    }
                    _ => UpdateOrganizationalUnitError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateOrganizationalUnitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateOrganizationalUnitError {
    fn from(err: serde_json::error::Error) -> UpdateOrganizationalUnitError {
        UpdateOrganizationalUnitError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateOrganizationalUnitError {
    fn from(err: CredentialsError) -> UpdateOrganizationalUnitError {
        UpdateOrganizationalUnitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateOrganizationalUnitError {
    fn from(err: HttpDispatchError) -> UpdateOrganizationalUnitError {
        UpdateOrganizationalUnitError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateOrganizationalUnitError {
    fn from(err: io::Error) -> UpdateOrganizationalUnitError {
        UpdateOrganizationalUnitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateOrganizationalUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateOrganizationalUnitError {
    fn description(&self) -> &str {
        match *self {
            UpdateOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => cause,
            UpdateOrganizationalUnitError::AccessDenied(ref cause) => cause,
            UpdateOrganizationalUnitError::ConcurrentModification(ref cause) => cause,
            UpdateOrganizationalUnitError::DuplicateOrganizationalUnit(ref cause) => cause,
            UpdateOrganizationalUnitError::InvalidInput(ref cause) => cause,
            UpdateOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => cause,
            UpdateOrganizationalUnitError::Service(ref cause) => cause,
            UpdateOrganizationalUnitError::TooManyRequests(ref cause) => cause,
            UpdateOrganizationalUnitError::Validation(ref cause) => cause,
            UpdateOrganizationalUnitError::Credentials(ref err) => err.description(),
            UpdateOrganizationalUnitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateOrganizationalUnitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePolicy
#[derive(Debug, PartialEq)]
pub enum UpdatePolicyError {
    /// <p>Your account is not a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide</i>.</p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to removing the last SCP from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <p/> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact AWS Support to request an increase in your limit. </p> <p>Or, The number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations, or contact AWS Support to request an increase in the number of accounts.</p> <p> <b>Note</b>: deleted and closed accounts still count toward your limit.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes you can send in one day.</p> </li> <li> <p>OU<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of organizational units you can have in an organization.</p> </li> <li> <p>OU</em>DEPTH<em>LIMIT</em>EXCEEDED: You attempted to create an organizational unit tree that is too many levels deep.</p> </li> <li> <p>POLICY<em>NUMBER</em>LIMIT<em>EXCEEDED. You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>MAX</em>POLICY<em>TYPE</em>ATTACHMENT<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>WITHOUT</em>PHONE<em>VERIFICATION: You attempted to remove an account from the organization that does not yet have enough information to exist as a stand-alone account. This account requires you to first complete phone verification. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#leave-without-all-info&quot;&gt;To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s master account to the marketplace that corresponds to the master account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide contact a valid address and phone number for the master account. Then try the operation again.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>A policy with the same name already exists.</p>
    DuplicatePolicy(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and cannot be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified a value that is not valid for that parameter.</p> </li> <li> <p>INVALID</em>FULL<em>NAME</em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID<em>LIST</em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID<em>PARTY</em>TYPE<em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the NextToken parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>PATTERN</em>TARGET<em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>ROLE<em>NAME: You provided a role name that is not valid. A role name can’t begin with the reserved prefix &#39;AWSServiceRoleFor&#39;.</p> </li> <li> <p>INVALID</em>SYNTAX<em>ORGANIZATION</em>ARN: You specified an invalid ARN for the organization.</p> </li> <li> <p>INVALID<em>SYNTAX</em>POLICY<em>ID: You specified an invalid policy ID. </p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The provided policy document does not meet the requirements of the specified policy type. For example, the syntax might be incorrect. For details about service control policy syntax, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide</i>.</p>
    MalformedPolicyDocument(String),
    /// <p>We can't find a policy with the PolicyId that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You've sent too many requests in too short a period of time. The limit helps protect against denial-of-service attacks. Try again later.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdatePolicyError {
    pub fn from_body(body: &str) -> UpdatePolicyError {
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
                    "AWSOrganizationsNotInUseException" => {
                        UpdatePolicyError::AWSOrganizationsNotInUse(String::from(error_message))
                    }
                    "AccessDeniedException" => {
                        UpdatePolicyError::AccessDenied(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        UpdatePolicyError::ConcurrentModification(String::from(error_message))
                    }
                    "ConstraintViolationException" => {
                        UpdatePolicyError::ConstraintViolation(String::from(error_message))
                    }
                    "DuplicatePolicyException" => {
                        UpdatePolicyError::DuplicatePolicy(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdatePolicyError::InvalidInput(String::from(error_message))
                    }
                    "MalformedPolicyDocumentException" => {
                        UpdatePolicyError::MalformedPolicyDocument(String::from(error_message))
                    }
                    "PolicyNotFoundException" => {
                        UpdatePolicyError::PolicyNotFound(String::from(error_message))
                    }
                    "ServiceException" => UpdatePolicyError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UpdatePolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdatePolicyError::Validation(error_message.to_string())
                    }
                    _ => UpdatePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdatePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdatePolicyError {
    fn from(err: serde_json::error::Error) -> UpdatePolicyError {
        UpdatePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePolicyError {
    fn from(err: CredentialsError) -> UpdatePolicyError {
        UpdatePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePolicyError {
    fn from(err: HttpDispatchError) -> UpdatePolicyError {
        UpdatePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePolicyError {
    fn from(err: io::Error) -> UpdatePolicyError {
        UpdatePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePolicyError {
    fn description(&self) -> &str {
        match *self {
            UpdatePolicyError::AWSOrganizationsNotInUse(ref cause) => cause,
            UpdatePolicyError::AccessDenied(ref cause) => cause,
            UpdatePolicyError::ConcurrentModification(ref cause) => cause,
            UpdatePolicyError::ConstraintViolation(ref cause) => cause,
            UpdatePolicyError::DuplicatePolicy(ref cause) => cause,
            UpdatePolicyError::InvalidInput(ref cause) => cause,
            UpdatePolicyError::MalformedPolicyDocument(ref cause) => cause,
            UpdatePolicyError::PolicyNotFound(ref cause) => cause,
            UpdatePolicyError::Service(ref cause) => cause,
            UpdatePolicyError::TooManyRequests(ref cause) => cause,
            UpdatePolicyError::Validation(ref cause) => cause,
            UpdatePolicyError::Credentials(ref err) => err.description(),
            UpdatePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdatePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Organizations API. Organizations clients implement this trait.
pub trait Organizations {
    /// <p>Sends a response to the originator of a handshake agreeing to the action proposed by the handshake request. </p> <p>This operation can be called only by the following principals when they also have the relevant IAM permissions:</p> <ul> <li> <p> <b>Invitation to join</b> or <b>Approve all features request</b> handshakes: only a principal from the member account. </p> <p>The user who calls the API for an invitation to join must have the <code>organizations:AcceptHandshake</code> permission. If you enabled all features in the organization, then the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that Organizations can create the required service-linked role named <i>OrgsServiceLinkedRoleName</i>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <b>Enable all features final confirmation</b> handshake: only a principal from the master account.</p> <p>For more information about invitations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_invites.html">Inviting an AWS Account to Join Your Organization</a> in the <i>AWS Organizations User Guide</i>. For more information about requests to enable all features in the organization, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>After you accept a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn accept_handshake(
        &self,
        input: AcceptHandshakeRequest,
    ) -> RusotoFuture<AcceptHandshakeResponse, AcceptHandshakeError>;

    /// <p>Attaches a policy to a root, an organizational unit, or an individual account. How the policy affects accounts depends on the type of policy:</p> <ul> <li> <p> <b>Service control policy (SCP)</b> - An SCP specifies what permissions can be delegated to users in affected member accounts. The scope of influence for a policy depends on what you attach the policy to:</p> <ul> <li> <p>If you attach an SCP to a root, it affects all accounts in the organization.</p> </li> <li> <p>If you attach an SCP to an OU, it affects all accounts in that OU and in any child OUs.</p> </li> <li> <p>If you attach the policy directly to an account, then it affects only that account.</p> </li> </ul> <p>SCPs essentially are permission "filters". When you attach one SCP to a higher level root or OU, and you also attach a different SCP to a child OU or to an account, the child policy can further restrict only the permissions that pass through the parent filter and are available to the child. An SCP that is attached to a child cannot grant a permission that is not already granted by the parent. For example, imagine that the parent SCP allows permissions A, B, C, D, and E. The child SCP allows C, D, E, F, and G. The result is that the accounts affected by the child SCP are allowed to use only C, D, and E. They cannot use A or B because they were filtered out by the child OU. They also cannot use F and G because they were filtered out by the parent OU. They cannot be granted back by the child SCP; child SCPs can only filter the permissions they receive from the parent SCP.</p> <p>AWS Organizations attaches a default SCP named <code>"FullAWSAccess</code> to every root, OU, and account. This default SCP allows all services and actions, enabling any new child OU or account to inherit the permissions of the parent root or OU. If you detach the default policy, you must replace it with a policy that specifies the permissions that you want to allow in that OU or account.</p> <p>For more information about how Organizations policies permissions work, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">Using Service Control Policies</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>This operation can be called only from the organization's master account.</p>
    fn attach_policy(&self, input: AttachPolicyRequest) -> RusotoFuture<(), AttachPolicyError>;

    /// <p>Cancels a handshake. Canceling a handshake sets the handshake state to <code>CANCELED</code>. </p> <p>This operation can be called only from the account that originated the handshake. The recipient of the handshake can't cancel it, but can use <a>DeclineHandshake</a> instead. After a handshake is canceled, the recipient can no longer respond to that handshake.</p> <p>After you cancel a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn cancel_handshake(
        &self,
        input: CancelHandshakeRequest,
    ) -> RusotoFuture<CancelHandshakeResponse, CancelHandshakeError>;

    /// <p><p>Creates an AWS account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that AWS performs in the background. If you want to check the status of the request later, you need the <code>OperationId</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> <p>The user who calls the API for an invitation to join must have the <code>organizations:CreateAccount</code> permission. If you enabled all features in the organization, then the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that Organizations can create the required service-linked role named <i>OrgsServiceLinkedRoleName</i>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> <p>The user in the master account who calls this API must also have the <code>iam:CreateRole</code> permission because AWS Organizations preconfigures the new member account with a role (named <code>OrganizationAccountAccessRole</code> by default) that grants users in the master account administrator permissions in the new member account. Principals in the master account can assume the role. AWS Organizations clones the company name and address information for the new account from the organization&#39;s master account.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <p>For more information about creating accounts, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> <important> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account, such as a payment method and signing the End User Licence Agreement (EULA) is <i>not</i> automatically collected. If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable this, then only the account root user can access billing information. For information about how to disable this for an account, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important></p>
    fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> RusotoFuture<CreateAccountResponse, CreateAccountError>;

    /// <p>Creates an AWS organization. The account whose user is calling the CreateOrganization operation automatically becomes the <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/orgs_getting-started_concepts.html#account">master account</a> of the new organization.</p> <p>This operation must be called using credentials from the account that is to become the new organization's master account. The principal must also have the relevant IAM permissions.</p> <p>By default (or if you set the <code>FeatureSet</code> parameter to <code>ALL</code>), the new organization is created with all features enabled and service control policies automatically enabled in the root. If you instead choose to create the organization supporting only the consolidated billing features by setting the <code>FeatureSet</code> parameter to <code>CONSOLIDATED_BILLING"</code>, then no policy types are enabled by default and you cannot use organization policies.</p>
    fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> RusotoFuture<CreateOrganizationResponse, CreateOrganizationError>;

    /// <p>Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. The number of levels deep that you can nest OUs is dependent upon the policy types enabled for that root. For service control policies, the limit is five. </p> <p>For more information about OUs, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_ous.html">Managing Organizational Units</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_organizational_unit(
        &self,
        input: CreateOrganizationalUnitRequest,
    ) -> RusotoFuture<CreateOrganizationalUnitResponse, CreateOrganizationalUnitError>;

    /// <p>Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual AWS account.</p> <p>For more information about policies and their use, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html">Managing Organization Policies</a>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_policy(
        &self,
        input: CreatePolicyRequest,
    ) -> RusotoFuture<CreatePolicyResponse, CreatePolicyError>;

    /// <p>Declines a handshake request. This sets the handshake state to <code>DECLINED</code> and effectively deactivates the request.</p> <p>This operation can be called only from the account that received the handshake. The originator of the handshake can use <a>CancelHandshake</a> instead. The originator can't reactivate a declined request, but can re-initiate the process with a new handshake request.</p> <p>After you decline a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn decline_handshake(
        &self,
        input: DeclineHandshakeRequest,
    ) -> RusotoFuture<DeclineHandshakeResponse, DeclineHandshakeError>;

    /// <p>Deletes the organization. You can delete an organization only by using credentials from the master account. The organization must be empty of member accounts, OUs, and policies.</p>
    fn delete_organization(&self) -> RusotoFuture<(), DeleteOrganizationError>;

    /// <p>Deletes an organizational unit from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_organizational_unit(
        &self,
        input: DeleteOrganizationalUnitRequest,
    ) -> RusotoFuture<(), DeleteOrganizationalUnitError>;

    /// <p>Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all OUs, roots, and accounts.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_policy(&self, input: DeletePolicyRequest) -> RusotoFuture<(), DeletePolicyError>;

    /// <p>Retrieves Organizations-related information about the specified account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_account(
        &self,
        input: DescribeAccountRequest,
    ) -> RusotoFuture<DescribeAccountResponse, DescribeAccountError>;

    /// <p>Retrieves the current status of an asynchronous request to create an account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_create_account_status(
        &self,
        input: DescribeCreateAccountStatusRequest,
    ) -> RusotoFuture<DescribeCreateAccountStatusResponse, DescribeCreateAccountStatusError>;

    /// <p>Retrieves information about a previously requested handshake. The handshake ID comes from the response to the original <a>InviteAccountToOrganization</a> operation that generated the handshake.</p> <p>You can access handshakes that are ACCEPTED, DECLINED, or CANCELED for only 30 days after they change to that state. They are then deleted and no longer accessible.</p> <p>This operation can be called from any account in the organization.</p>
    fn describe_handshake(
        &self,
        input: DescribeHandshakeRequest,
    ) -> RusotoFuture<DescribeHandshakeResponse, DescribeHandshakeError>;

    /// <p><p>Retrieves information about the organization that the user&#39;s account belongs to.</p> <p>This operation can be called from any account in the organization.</p> <note> <p>Even if a policy type is shown as available in the organization, it can be disabled separately at the root level with <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of policy types for a specified root.</p> </note></p>
    fn describe_organization(
        &self,
    ) -> RusotoFuture<DescribeOrganizationResponse, DescribeOrganizationError>;

    /// <p>Retrieves information about an organizational unit (OU).</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_organizational_unit(
        &self,
        input: DescribeOrganizationalUnitRequest,
    ) -> RusotoFuture<DescribeOrganizationalUnitResponse, DescribeOrganizationalUnitError>;

    /// <p>Retrieves information about a policy.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_policy(
        &self,
        input: DescribePolicyRequest,
    ) -> RusotoFuture<DescribePolicyResponse, DescribePolicyError>;

    /// <p>Detaches a policy from a target root, organizational unit, or account. If the policy being detached is a service control policy (SCP), the changes to permissions for IAM users and roles in affected accounts are immediate.</p> <p> <b>Note:</b> Every root, OU, and account must have at least one SCP attached. If you want to replace the default <code>FullAWSAccess</code> policy with one that limits the permissions that can be delegated, then you must attach the replacement policy before you can remove the default one. This is the authorization strategy of <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_whitelist">whitelisting</a>. If you instead attach a second SCP and leave the <code>FullAWSAccess</code> SCP still attached, and specify <code>"Effect": "Deny"</code> in the second SCP to override the <code>"Effect": "Allow"</code> in the <code>FullAWSAccess</code> policy (or any other attached SCP), then you are using the authorization strategy of <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_blacklist">blacklisting</a>. </p> <p>This operation can be called only from the organization's master account.</p>
    fn detach_policy(&self, input: DetachPolicyRequest) -> RusotoFuture<(), DetachPolicyError>;

    /// <p>Disables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you disable integration, the specified service no longer can create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in <i>new</i> accounts in your organization. This means the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from AWS Organizations.</p> <p/> <important> <p>We recommend that you disable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the other service is aware that it can clean up any resources that are required only for the integration. How the service cleans up its resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>After you perform the <code>DisableAWSServiceAccess</code> operation, the specified service can no longer perform operations in your organization's accounts unless the operations are explicitly permitted by the IAM policies that are attached to your roles. </p> <p>For more information about integrating other services with AWS Organizations, including the list of services that work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn disable_aws_service_access(
        &self,
        input: DisableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), DisableAWSServiceAccessError>;

    /// <p><p>Disables an organizational control policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach policies of the specified type to that root or to any OU or account in that root. You can undo this by using the <a>EnablePolicyType</a> operation.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>If you disable a policy type for a root, it still shows as enabled for the organization if all features are enabled in that organization. Use <a>ListRoots</a> to see the status of policy types for a specified root. Use <a>DescribeOrganization</a> to see the status of policy types in the organization.</p> </note></p>
    fn disable_policy_type(
        &self,
        input: DisablePolicyTypeRequest,
    ) -> RusotoFuture<DisablePolicyTypeResponse, DisablePolicyTypeError>;

    /// <p>Enables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you enable integration, you allow the specified service to create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in all the accounts in your organization. This allows the service to perform operations on your behalf in your organization and its accounts.</p> <important> <p>We recommend that you enable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the service is aware that it can create the resources that are required for the integration. How the service creates those resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>For more information about enabling services to integrate with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account and only if the organization has <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">enabled all features</a>.</p>
    fn enable_aws_service_access(
        &self,
        input: EnableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), EnableAWSServiceAccessError>;

    /// <p>Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. Until you enable all features, you have access only to consolidated billing, and you can't use any of the advanced account administration features that AWS Organizations supports. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> <important> <p>This operation is required only for organizations that were created explicitly with only the consolidated billing features enabled, or that were migrated from a Consolidated Billing account family to Organizations. Calling this operation sends a handshake to every invited account in the organization. The feature set change can be finalized and the additional features enabled only after all administrators in the invited accounts approve the change by accepting the handshake.</p> </important> <p>After you enable all features, you can separately enable or disable individual policy types in a root using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. To see the status of policy types in a root, use <a>ListRoots</a>.</p> <p>After all invited member accounts accept the handshake, you finalize the feature set change by accepting the handshake that contains <code>"Action": "ENABLE_ALL_FEATURES"</code>. This completes the change.</p> <p>After you enable all features in your organization, the master account in the organization can apply policies on all member accounts. These policies can restrict what users and even administrators in those accounts can do. The master account can apply policies that prevent accounts from leaving the organization. Ensure that your account administrators are aware of this.</p> <p>This operation can be called only from the organization's master account. </p>
    fn enable_all_features(
        &self,
    ) -> RusotoFuture<EnableAllFeaturesResponse, EnableAllFeaturesError>;

    /// <p>Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any OU, or account in that root. You can undo this by using the <a>DisablePolicyType</a> operation.</p> <p>This operation can be called only from the organization's master account.</p> <p>You can enable a policy type in a root only if that policy type is available in the organization. Use <a>DescribeOrganization</a> to view the status of available policy types in the organization.</p> <p>To view the status of policy type in a root, use <a>ListRoots</a>.</p>
    fn enable_policy_type(
        &self,
        input: EnablePolicyTypeRequest,
    ) -> RusotoFuture<EnablePolicyTypeResponse, EnablePolicyTypeError>;

    /// <p><p>Sends an invitation to another account to join your organization as a member account. Organizations sends email on your behalf to the email address that is associated with the other account&#39;s owner. The invitation is implemented as a <a>Handshake</a> whose details are in the response.</p> <important> <p>You can invite AWS accounts only from the same seller as the master account. For example, if your organization&#39;s master account was created by Amazon Internet Services Pvt. Ltd (AISPL), an AWS seller in India, then you can only invite other AISPL accounts to your organization. You can&#39;t combine accounts from AISPL and AWS, or any other AWS seller. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/useconsolidatedbilliing-India.html">Consolidated Billing in India</a>.</p> </important> <p>This operation can be called only from the organization&#39;s master account.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important></p>
    fn invite_account_to_organization(
        &self,
        input: InviteAccountToOrganizationRequest,
    ) -> RusotoFuture<InviteAccountToOrganizationResponse, InviteAccountToOrganizationError>;

    /// <p><p>Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. To remove a member account as a user in the master account, use <a>RemoveAccountFromOrganization</a> instead.</p> <p>This operation can be called only from a member account in the organization.</p> <important> <ul> <li> <p>The master account in an organization with all features enabled can set service control policies (SCPs) that can restrict what administrators of member accounts can do, including preventing them from successfully calling <code>LeaveOrganization</code> and leaving the organization. </p> </li> <li> <p>You can leave an organization as a member account only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For each account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>You can leave an organization only after you enable IAM user access to billing in your account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </li> </ul> </important></p>
    fn leave_organization(&self) -> RusotoFuture<(), LeaveOrganizationError>;

    /// <p>Returns a list of the AWS services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operations on your organization and its accounts.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that currently work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn list_aws_service_access_for_organization(
        &self,
        input: ListAWSServiceAccessForOrganizationRequest,
    ) -> RusotoFuture<
        ListAWSServiceAccessForOrganizationResponse,
        ListAWSServiceAccessForOrganizationError,
    >;

    /// <p>Lists all the accounts in the organization. To request only the accounts in a specified root or OU, use the <a>ListAccountsForParent</a> operation instead.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> RusotoFuture<ListAccountsResponse, ListAccountsError>;

    /// <p>Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that are not in any OU. If you specify an OU, you get a list of all the accounts in only that OU, and not in any child OUs. To get a list of all accounts in the organization, use the <a>ListAccounts</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts_for_parent(
        &self,
        input: ListAccountsForParentRequest,
    ) -> RusotoFuture<ListAccountsForParentResponse, ListAccountsForParentError>;

    /// <p>Lists all of the OUs or accounts that are contained in the specified parent OU or root. This operation, along with <a>ListParents</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_children(
        &self,
        input: ListChildrenRequest,
    ) -> RusotoFuture<ListChildrenResponse, ListChildrenError>;

    /// <p>Lists the account creation requests that match the specified status that is currently being tracked for the organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_create_account_status(
        &self,
        input: ListCreateAccountStatusRequest,
    ) -> RusotoFuture<ListCreateAccountStatusResponse, ListCreateAccountStatusError>;

    /// <p>Lists the current handshakes that are associated with the account of the requesting user.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called from any account in the organization.</p>
    fn list_handshakes_for_account(
        &self,
        input: ListHandshakesForAccountRequest,
    ) -> RusotoFuture<ListHandshakesForAccountResponse, ListHandshakesForAccountError>;

    /// <p>Lists the handshakes that are associated with the organization that the requesting user is part of. The <code>ListHandshakesForOrganization</code> operation returns a list of handshake structures. Each structure contains details and status about a handshake.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_handshakes_for_organization(
        &self,
        input: ListHandshakesForOrganizationRequest,
    ) -> RusotoFuture<ListHandshakesForOrganizationResponse, ListHandshakesForOrganizationError>;

    /// <p>Lists the organizational units (OUs) in a parent organizational unit or root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_organizational_units_for_parent(
        &self,
        input: ListOrganizationalUnitsForParentRequest,
    ) -> RusotoFuture<ListOrganizationalUnitsForParentResponse, ListOrganizationalUnitsForParentError>;

    /// <p><p>Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with <a>ListChildren</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>In the current release, a child can have only a single parent. </p> </note></p>
    fn list_parents(
        &self,
        input: ListParentsRequest,
    ) -> RusotoFuture<ListParentsResponse, ListParentsError>;

    /// <p>Retrieves the list of all policies in an organization of a specified type.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> RusotoFuture<ListPoliciesResponse, ListPoliciesError>;

    /// <p>Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies_for_target(
        &self,
        input: ListPoliciesForTargetRequest,
    ) -> RusotoFuture<ListPoliciesForTargetResponse, ListPoliciesForTargetError>;

    /// <p><p>Lists the roots that are defined in the current organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>Policy types can be enabled and disabled in roots. This is distinct from whether they are available in the organization. When you enable all features, you make policy types available for use in that organization. Individual policy types can then be enabled and disabled in a root. To see the availability of a policy type in an organization, use <a>DescribeOrganization</a>.</p> </note></p>
    fn list_roots(
        &self,
        input: ListRootsRequest,
    ) -> RusotoFuture<ListRootsResponse, ListRootsError>;

    /// <p>Lists all the roots, OUs, and accounts to which the specified policy is attached.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_targets_for_policy(
        &self,
        input: ListTargetsForPolicyRequest,
    ) -> RusotoFuture<ListTargetsForPolicyResponse, ListTargetsForPolicyError>;

    /// <p>Moves an account from its current source parent root or OU to the specified destination parent root or OU.</p> <p>This operation can be called only from the organization's master account.</p>
    fn move_account(&self, input: MoveAccountRequest) -> RusotoFuture<(), MoveAccountError>;

    /// <p><p>Removes the specified account from the organization.</p> <p>The removed account becomes a stand-alone account that is not a member of any organization. It is no longer subject to any policies and is responsible for its own bill payments. The organization&#39;s master account is no longer charged for any expenses accrued by the member account after it is removed from the organization.</p> <p>This operation can be called only from the organization&#39;s master account. Member accounts can remove themselves with <a>LeaveOrganization</a> instead.</p> <important> <ul> <li> <p>You can remove an account from your organization only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For an account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. To remove an account that does not yet have this information, you must sign in as the member account and follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>You can remove a member account only after you enable IAM user access to billing in the member account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </li> </ul> </important></p>
    fn remove_account_from_organization(
        &self,
        input: RemoveAccountFromOrganizationRequest,
    ) -> RusotoFuture<(), RemoveAccountFromOrganizationError>;

    /// <p>Renames the specified organizational unit (OU). The ID and ARN do not change. The child OUs and accounts remain in place, and any attached policies of the OU remain attached. </p> <p>This operation can be called only from the organization's master account.</p>
    fn update_organizational_unit(
        &self,
        input: UpdateOrganizationalUnitRequest,
    ) -> RusotoFuture<UpdateOrganizationalUnitResponse, UpdateOrganizationalUnitError>;

    /// <p>Updates an existing policy with a new name, description, or content. If any parameter is not supplied, that value remains unchanged. Note that you cannot change a policy's type.</p> <p>This operation can be called only from the organization's master account.</p>
    fn update_policy(
        &self,
        input: UpdatePolicyRequest,
    ) -> RusotoFuture<UpdatePolicyResponse, UpdatePolicyError>;
}
/// A client for the Organizations API.
pub struct OrganizationsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl OrganizationsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> OrganizationsClient {
        OrganizationsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> OrganizationsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        OrganizationsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Organizations for OrganizationsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Sends a response to the originator of a handshake agreeing to the action proposed by the handshake request. </p> <p>This operation can be called only by the following principals when they also have the relevant IAM permissions:</p> <ul> <li> <p> <b>Invitation to join</b> or <b>Approve all features request</b> handshakes: only a principal from the member account. </p> <p>The user who calls the API for an invitation to join must have the <code>organizations:AcceptHandshake</code> permission. If you enabled all features in the organization, then the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that Organizations can create the required service-linked role named <i>OrgsServiceLinkedRoleName</i>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <b>Enable all features final confirmation</b> handshake: only a principal from the master account.</p> <p>For more information about invitations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_invites.html">Inviting an AWS Account to Join Your Organization</a> in the <i>AWS Organizations User Guide</i>. For more information about requests to enable all features in the organization, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>After you accept a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn accept_handshake(
        &self,
        input: AcceptHandshakeRequest,
    ) -> RusotoFuture<AcceptHandshakeResponse, AcceptHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.AcceptHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AcceptHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AcceptHandshakeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attaches a policy to a root, an organizational unit, or an individual account. How the policy affects accounts depends on the type of policy:</p> <ul> <li> <p> <b>Service control policy (SCP)</b> - An SCP specifies what permissions can be delegated to users in affected member accounts. The scope of influence for a policy depends on what you attach the policy to:</p> <ul> <li> <p>If you attach an SCP to a root, it affects all accounts in the organization.</p> </li> <li> <p>If you attach an SCP to an OU, it affects all accounts in that OU and in any child OUs.</p> </li> <li> <p>If you attach the policy directly to an account, then it affects only that account.</p> </li> </ul> <p>SCPs essentially are permission "filters". When you attach one SCP to a higher level root or OU, and you also attach a different SCP to a child OU or to an account, the child policy can further restrict only the permissions that pass through the parent filter and are available to the child. An SCP that is attached to a child cannot grant a permission that is not already granted by the parent. For example, imagine that the parent SCP allows permissions A, B, C, D, and E. The child SCP allows C, D, E, F, and G. The result is that the accounts affected by the child SCP are allowed to use only C, D, and E. They cannot use A or B because they were filtered out by the child OU. They also cannot use F and G because they were filtered out by the parent OU. They cannot be granted back by the child SCP; child SCPs can only filter the permissions they receive from the parent SCP.</p> <p>AWS Organizations attaches a default SCP named <code>"FullAWSAccess</code> to every root, OU, and account. This default SCP allows all services and actions, enabling any new child OU or account to inherit the permissions of the parent root or OU. If you detach the default policy, you must replace it with a policy that specifies the permissions that you want to allow in that OU or account.</p> <p>For more information about how Organizations policies permissions work, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">Using Service Control Policies</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> <p>This operation can be called only from the organization's master account.</p>
    fn attach_policy(&self, input: AttachPolicyRequest) -> RusotoFuture<(), AttachPolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.AttachPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Cancels a handshake. Canceling a handshake sets the handshake state to <code>CANCELED</code>. </p> <p>This operation can be called only from the account that originated the handshake. The recipient of the handshake can't cancel it, but can use <a>DeclineHandshake</a> instead. After a handshake is canceled, the recipient can no longer respond to that handshake.</p> <p>After you cancel a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn cancel_handshake(
        &self,
        input: CancelHandshakeRequest,
    ) -> RusotoFuture<CancelHandshakeResponse, CancelHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CancelHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelHandshakeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates an AWS account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that AWS performs in the background. If you want to check the status of the request later, you need the <code>OperationId</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> <p>The user who calls the API for an invitation to join must have the <code>organizations:CreateAccount</code> permission. If you enabled all features in the organization, then the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that Organizations can create the required service-linked role named <i>OrgsServiceLinkedRoleName</i>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> <p>The user in the master account who calls this API must also have the <code>iam:CreateRole</code> permission because AWS Organizations preconfigures the new member account with a role (named <code>OrganizationAccountAccessRole</code> by default) that grants users in the master account administrator permissions in the new member account. Principals in the master account can assume the role. AWS Organizations clones the company name and address information for the new account from the organization&#39;s master account.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <p>For more information about creating accounts, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> <important> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account, such as a payment method and signing the End User Licence Agreement (EULA) is <i>not</i> automatically collected. If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable this, then only the account root user can access billing information. For information about how to disable this for an account, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important></p>
    fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> RusotoFuture<CreateAccountResponse, CreateAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CreateAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAccountResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an AWS organization. The account whose user is calling the CreateOrganization operation automatically becomes the <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/orgs_getting-started_concepts.html#account">master account</a> of the new organization.</p> <p>This operation must be called using credentials from the account that is to become the new organization's master account. The principal must also have the relevant IAM permissions.</p> <p>By default (or if you set the <code>FeatureSet</code> parameter to <code>ALL</code>), the new organization is created with all features enabled and service control policies automatically enabled in the root. If you instead choose to create the organization supporting only the consolidated billing features by setting the <code>FeatureSet</code> parameter to <code>CONSOLIDATED_BILLING"</code>, then no policy types are enabled by default and you cannot use organization policies.</p>
    fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> RusotoFuture<CreateOrganizationResponse, CreateOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.CreateOrganization",
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

                    serde_json::from_str::<CreateOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. The number of levels deep that you can nest OUs is dependent upon the policy types enabled for that root. For service control policies, the limit is five. </p> <p>For more information about OUs, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_ous.html">Managing Organizational Units</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_organizational_unit(
        &self,
        input: CreateOrganizationalUnitRequest,
    ) -> RusotoFuture<CreateOrganizationalUnitResponse, CreateOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.CreateOrganizationalUnit",
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

                    serde_json::from_str::<CreateOrganizationalUnitResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateOrganizationalUnitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual AWS account.</p> <p>For more information about policies and their use, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html">Managing Organization Policies</a>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn create_policy(
        &self,
        input: CreatePolicyRequest,
    ) -> RusotoFuture<CreatePolicyResponse, CreatePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CreatePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreatePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Declines a handshake request. This sets the handshake state to <code>DECLINED</code> and effectively deactivates the request.</p> <p>This operation can be called only from the account that received the handshake. The originator of the handshake can use <a>CancelHandshake</a> instead. The originator can't reactivate a declined request, but can re-initiate the process with a new handshake request.</p> <p>After you decline a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that it is deleted.</p>
    fn decline_handshake(
        &self,
        input: DeclineHandshakeRequest,
    ) -> RusotoFuture<DeclineHandshakeResponse, DeclineHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DeclineHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeclineHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeclineHandshakeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the organization. You can delete an organization only by using credentials from the master account. The organization must be empty of member accounts, OUs, and policies.</p>
    fn delete_organization(&self) -> RusotoFuture<(), DeleteOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DeleteOrganization",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an organizational unit from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_organizational_unit(
        &self,
        input: DeleteOrganizationalUnitRequest,
    ) -> RusotoFuture<(), DeleteOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DeleteOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteOrganizationalUnitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all OUs, roots, and accounts.</p> <p>This operation can be called only from the organization's master account.</p>
    fn delete_policy(&self, input: DeletePolicyRequest) -> RusotoFuture<(), DeletePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DeletePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves Organizations-related information about the specified account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_account(
        &self,
        input: DescribeAccountRequest,
    ) -> RusotoFuture<DescribeAccountResponse, DescribeAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DescribeAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAccountResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the current status of an asynchronous request to create an account.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_create_account_status(
        &self,
        input: DescribeCreateAccountStatusRequest,
    ) -> RusotoFuture<DescribeCreateAccountStatusResponse, DescribeCreateAccountStatusError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeCreateAccountStatus",
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

                    serde_json::from_str::<DescribeCreateAccountStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCreateAccountStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves information about a previously requested handshake. The handshake ID comes from the response to the original <a>InviteAccountToOrganization</a> operation that generated the handshake.</p> <p>You can access handshakes that are ACCEPTED, DECLINED, or CANCELED for only 30 days after they change to that state. They are then deleted and no longer accessible.</p> <p>This operation can be called from any account in the organization.</p>
    fn describe_handshake(
        &self,
        input: DescribeHandshakeRequest,
    ) -> RusotoFuture<DescribeHandshakeResponse, DescribeHandshakeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeHandshake",
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

                    serde_json::from_str::<DescribeHandshakeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeHandshakeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves information about the organization that the user&#39;s account belongs to.</p> <p>This operation can be called from any account in the organization.</p> <note> <p>Even if a policy type is shown as available in the organization, it can be disabled separately at the root level with <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of policy types for a specified root.</p> </note></p>
    fn describe_organization(
        &self,
    ) -> RusotoFuture<DescribeOrganizationResponse, DescribeOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeOrganization",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves information about an organizational unit (OU).</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_organizational_unit(
        &self,
        input: DescribeOrganizationalUnitRequest,
    ) -> RusotoFuture<DescribeOrganizationalUnitResponse, DescribeOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeOrganizationalUnit",
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

                    serde_json::from_str::<DescribeOrganizationalUnitResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeOrganizationalUnitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves information about a policy.</p> <p>This operation can be called only from the organization's master account.</p>
    fn describe_policy(
        &self,
        input: DescribePolicyRequest,
    ) -> RusotoFuture<DescribePolicyResponse, DescribePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DescribePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Detaches a policy from a target root, organizational unit, or account. If the policy being detached is a service control policy (SCP), the changes to permissions for IAM users and roles in affected accounts are immediate.</p> <p> <b>Note:</b> Every root, OU, and account must have at least one SCP attached. If you want to replace the default <code>FullAWSAccess</code> policy with one that limits the permissions that can be delegated, then you must attach the replacement policy before you can remove the default one. This is the authorization strategy of <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_whitelist">whitelisting</a>. If you instead attach a second SCP and leave the <code>FullAWSAccess</code> SCP still attached, and specify <code>"Effect": "Deny"</code> in the second SCP to override the <code>"Effect": "Allow"</code> in the <code>FullAWSAccess</code> policy (or any other attached SCP), then you are using the authorization strategy of <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_about-scps.html#orgs_policies_blacklist">blacklisting</a>. </p> <p>This operation can be called only from the organization's master account.</p>
    fn detach_policy(&self, input: DetachPolicyRequest) -> RusotoFuture<(), DetachPolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DetachPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you disable integration, the specified service no longer can create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in <i>new</i> accounts in your organization. This means the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from AWS Organizations.</p> <p/> <important> <p>We recommend that you disable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the other service is aware that it can clean up any resources that are required only for the integration. How the service cleans up its resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>After you perform the <code>DisableAWSServiceAccess</code> operation, the specified service can no longer perform operations in your organization's accounts unless the operations are explicitly permitted by the IAM policies that are attached to your roles. </p> <p>For more information about integrating other services with AWS Organizations, including the list of services that work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn disable_aws_service_access(
        &self,
        input: DisableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), DisableAWSServiceAccessError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DisableAWSServiceAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableAWSServiceAccessError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Disables an organizational control policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach policies of the specified type to that root or to any OU or account in that root. You can undo this by using the <a>EnablePolicyType</a> operation.</p> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>If you disable a policy type for a root, it still shows as enabled for the organization if all features are enabled in that organization. Use <a>ListRoots</a> to see the status of policy types for a specified root. Use <a>DescribeOrganization</a> to see the status of policy types in the organization.</p> </note></p>
    fn disable_policy_type(
        &self,
        input: DisablePolicyTypeRequest,
    ) -> RusotoFuture<DisablePolicyTypeResponse, DisablePolicyTypeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DisablePolicyType",
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

                    serde_json::from_str::<DisablePolicyTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisablePolicyTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you enable integration, you allow the specified service to create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in all the accounts in your organization. This allows the service to perform operations on your behalf in your organization and its accounts.</p> <important> <p>We recommend that you enable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the service is aware that it can create the resources that are required for the integration. How the service creates those resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>For more information about enabling services to integrate with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account and only if the organization has <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">enabled all features</a>.</p>
    fn enable_aws_service_access(
        &self,
        input: EnableAWSServiceAccessRequest,
    ) -> RusotoFuture<(), EnableAWSServiceAccessError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.EnableAWSServiceAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableAWSServiceAccessError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. Until you enable all features, you have access only to consolidated billing, and you can't use any of the advanced account administration features that AWS Organizations supports. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p> <important> <p>This operation is required only for organizations that were created explicitly with only the consolidated billing features enabled, or that were migrated from a Consolidated Billing account family to Organizations. Calling this operation sends a handshake to every invited account in the organization. The feature set change can be finalized and the additional features enabled only after all administrators in the invited accounts approve the change by accepting the handshake.</p> </important> <p>After you enable all features, you can separately enable or disable individual policy types in a root using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. To see the status of policy types in a root, use <a>ListRoots</a>.</p> <p>After all invited member accounts accept the handshake, you finalize the feature set change by accepting the handshake that contains <code>"Action": "ENABLE_ALL_FEATURES"</code>. This completes the change.</p> <p>After you enable all features in your organization, the master account in the organization can apply policies on all member accounts. These policies can restrict what users and even administrators in those accounts can do. The master account can apply policies that prevent accounts from leaving the organization. Ensure that your account administrators are aware of this.</p> <p>This operation can be called only from the organization's master account. </p>
    fn enable_all_features(
        &self,
    ) -> RusotoFuture<EnableAllFeaturesResponse, EnableAllFeaturesError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.EnableAllFeatures",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EnableAllFeaturesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableAllFeaturesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any OU, or account in that root. You can undo this by using the <a>DisablePolicyType</a> operation.</p> <p>This operation can be called only from the organization's master account.</p> <p>You can enable a policy type in a root only if that policy type is available in the organization. Use <a>DescribeOrganization</a> to view the status of available policy types in the organization.</p> <p>To view the status of policy type in a root, use <a>ListRoots</a>.</p>
    fn enable_policy_type(
        &self,
        input: EnablePolicyTypeRequest,
    ) -> RusotoFuture<EnablePolicyTypeResponse, EnablePolicyTypeError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.EnablePolicyType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EnablePolicyTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnablePolicyTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Sends an invitation to another account to join your organization as a member account. Organizations sends email on your behalf to the email address that is associated with the other account&#39;s owner. The invitation is implemented as a <a>Handshake</a> whose details are in the response.</p> <important> <p>You can invite AWS accounts only from the same seller as the master account. For example, if your organization&#39;s master account was created by Amazon Internet Services Pvt. Ltd (AISPL), an AWS seller in India, then you can only invite other AISPL accounts to your organization. You can&#39;t combine accounts from AISPL and AWS, or any other AWS seller. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/useconsolidatedbilliing-India.html">Consolidated Billing in India</a>.</p> </important> <p>This operation can be called only from the organization&#39;s master account.</p> <important> <p>If you get an exception that indicates that you exceeded your account limits for the organization or that you can&quot;t add an account because your organization is still initializing, please contact <a href="https://console.aws.amazon.com/support/home#/"> AWS Customer Support</a>.</p> </important></p>
    fn invite_account_to_organization(
        &self,
        input: InviteAccountToOrganizationRequest,
    ) -> RusotoFuture<InviteAccountToOrganizationResponse, InviteAccountToOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.InviteAccountToOrganization",
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

                    serde_json::from_str::<InviteAccountToOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(InviteAccountToOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. To remove a member account as a user in the master account, use <a>RemoveAccountFromOrganization</a> instead.</p> <p>This operation can be called only from a member account in the organization.</p> <important> <ul> <li> <p>The master account in an organization with all features enabled can set service control policies (SCPs) that can restrict what administrators of member accounts can do, including preventing them from successfully calling <code>LeaveOrganization</code> and leaving the organization. </p> </li> <li> <p>You can leave an organization as a member account only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For each account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>You can leave an organization only after you enable IAM user access to billing in your account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </li> </ul> </important></p>
    fn leave_organization(&self) -> RusotoFuture<(), LeaveOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.LeaveOrganization",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(LeaveOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of the AWS services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operations on your organization and its accounts.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that currently work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's master account.</p>
    fn list_aws_service_access_for_organization(
        &self,
        input: ListAWSServiceAccessForOrganizationRequest,
    ) -> RusotoFuture<
        ListAWSServiceAccessForOrganizationResponse,
        ListAWSServiceAccessForOrganizationError,
    > {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListAWSServiceAccessForOrganization",
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

                    serde_json::from_str::<ListAWSServiceAccessForOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAWSServiceAccessForOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all the accounts in the organization. To request only the accounts in a specified root or OU, use the <a>ListAccountsForParent</a> operation instead.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> RusotoFuture<ListAccountsResponse, ListAccountsError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAccountsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAccountsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that are not in any OU. If you specify an OU, you get a list of all the accounts in only that OU, and not in any child OUs. To get a list of all accounts in the organization, use the <a>ListAccounts</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_accounts_for_parent(
        &self,
        input: ListAccountsForParentRequest,
    ) -> RusotoFuture<ListAccountsForParentResponse, ListAccountsForParentError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListAccountsForParent",
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

                    serde_json::from_str::<ListAccountsForParentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAccountsForParentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all of the OUs or accounts that are contained in the specified parent OU or root. This operation, along with <a>ListParents</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_children(
        &self,
        input: ListChildrenRequest,
    ) -> RusotoFuture<ListChildrenResponse, ListChildrenError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListChildren");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListChildrenResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListChildrenError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the account creation requests that match the specified status that is currently being tracked for the organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_create_account_status(
        &self,
        input: ListCreateAccountStatusRequest,
    ) -> RusotoFuture<ListCreateAccountStatusResponse, ListCreateAccountStatusError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListCreateAccountStatus",
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

                    serde_json::from_str::<ListCreateAccountStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListCreateAccountStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the current handshakes that are associated with the account of the requesting user.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called from any account in the organization.</p>
    fn list_handshakes_for_account(
        &self,
        input: ListHandshakesForAccountRequest,
    ) -> RusotoFuture<ListHandshakesForAccountResponse, ListHandshakesForAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListHandshakesForAccount",
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

                    serde_json::from_str::<ListHandshakesForAccountResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListHandshakesForAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the handshakes that are associated with the organization that the requesting user is part of. The <code>ListHandshakesForOrganization</code> operation returns a list of handshake structures. Each structure contains details and status about a handshake.</p> <p>Handshakes that are ACCEPTED, DECLINED, or CANCELED appear in the results of this API for only 30 days after changing to that state. After that they are deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_handshakes_for_organization(
        &self,
        input: ListHandshakesForOrganizationRequest,
    ) -> RusotoFuture<ListHandshakesForOrganizationResponse, ListHandshakesForOrganizationError>
    {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListHandshakesForOrganization",
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

                    serde_json::from_str::<ListHandshakesForOrganizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListHandshakesForOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the organizational units (OUs) in a parent organizational unit or root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_organizational_units_for_parent(
        &self,
        input: ListOrganizationalUnitsForParentRequest,
    ) -> RusotoFuture<ListOrganizationalUnitsForParentResponse, ListOrganizationalUnitsForParentError>
    {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListOrganizationalUnitsForParent",
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

                    serde_json::from_str::<ListOrganizationalUnitsForParentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListOrganizationalUnitsForParentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with <a>ListChildren</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>In the current release, a child can have only a single parent. </p> </note></p>
    fn list_parents(
        &self,
        input: ListParentsRequest,
    ) -> RusotoFuture<ListParentsResponse, ListParentsError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListParents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListParentsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListParentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the list of all policies in an organization of a specified type.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> RusotoFuture<ListPoliciesResponse, ListPoliciesError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPoliciesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_policies_for_target(
        &self,
        input: ListPoliciesForTargetRequest,
    ) -> RusotoFuture<ListPoliciesForTargetResponse, ListPoliciesForTargetError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListPoliciesForTarget",
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

                    serde_json::from_str::<ListPoliciesForTargetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPoliciesForTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Lists the roots that are defined in the current organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s master account.</p> <note> <p>Policy types can be enabled and disabled in roots. This is distinct from whether they are available in the organization. When you enable all features, you make policy types available for use in that organization. Individual policy types can then be enabled and disabled in a root. To see the availability of a policy type in an organization, use <a>DescribeOrganization</a>.</p> </note></p>
    fn list_roots(
        &self,
        input: ListRootsRequest,
    ) -> RusotoFuture<ListRootsResponse, ListRootsError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListRoots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRootsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListRootsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all the roots, OUs, and accounts to which the specified policy is attached.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's master account.</p>
    fn list_targets_for_policy(
        &self,
        input: ListTargetsForPolicyRequest,
    ) -> RusotoFuture<ListTargetsForPolicyResponse, ListTargetsForPolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListTargetsForPolicy",
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

                    serde_json::from_str::<ListTargetsForPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTargetsForPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Moves an account from its current source parent root or OU to the specified destination parent root or OU.</p> <p>This operation can be called only from the organization's master account.</p>
    fn move_account(&self, input: MoveAccountRequest) -> RusotoFuture<(), MoveAccountError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.MoveAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(MoveAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Removes the specified account from the organization.</p> <p>The removed account becomes a stand-alone account that is not a member of any organization. It is no longer subject to any policies and is responsible for its own bill payments. The organization&#39;s master account is no longer charged for any expenses accrued by the member account after it is removed from the organization.</p> <p>This operation can be called only from the organization&#39;s master account. Member accounts can remove themselves with <a>LeaveOrganization</a> instead.</p> <important> <ul> <li> <p>You can remove an account from your organization only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For an account that you want to make standalone, you must accept the End User License Agreement (EULA), choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account is not attached to an organization. To remove an account that does not yet have this information, you must sign in as the member account and follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>You can remove a member account only after you enable IAM user access to billing in the member account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> </li> </ul> </important></p>
    fn remove_account_from_organization(
        &self,
        input: RemoveAccountFromOrganizationRequest,
    ) -> RusotoFuture<(), RemoveAccountFromOrganizationError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.RemoveAccountFromOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveAccountFromOrganizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Renames the specified organizational unit (OU). The ID and ARN do not change. The child OUs and accounts remain in place, and any attached policies of the OU remain attached. </p> <p>This operation can be called only from the organization's master account.</p>
    fn update_organizational_unit(
        &self,
        input: UpdateOrganizationalUnitRequest,
    ) -> RusotoFuture<UpdateOrganizationalUnitResponse, UpdateOrganizationalUnitError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.UpdateOrganizationalUnit",
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

                    serde_json::from_str::<UpdateOrganizationalUnitResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateOrganizationalUnitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an existing policy with a new name, description, or content. If any parameter is not supplied, that value remains unchanged. Note that you cannot change a policy's type.</p> <p>This operation can be called only from the organization's master account.</p>
    fn update_policy(
        &self,
        input: UpdatePolicyRequest,
    ) -> RusotoFuture<UpdatePolicyResponse, UpdatePolicyError> {
        let mut request = SignedRequest::new("POST", "organizations", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.UpdatePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdatePolicyError::from_body(
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
