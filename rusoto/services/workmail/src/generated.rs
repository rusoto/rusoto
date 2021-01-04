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

impl WorkmailClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "workmail", &self.region, request_uri);

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

/// <p>A rule that controls access to an Amazon WorkMail organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccessControlRule {
    /// <p>Access protocol actions to include in the rule. Valid values include <code>ActiveSync</code>, <code>AutoDiscover</code>, <code>EWS</code>, <code>IMAP</code>, <code>SMTP</code>, <code>WindowsOutlook</code>, and <code>WebMail</code>.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// <p>The date that the rule was created.</p>
    #[serde(rename = "DateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<f64>,
    /// <p>The date that the rule was modified.</p>
    #[serde(rename = "DateModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<f64>,
    /// <p>The rule description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The rule effect.</p>
    #[serde(rename = "Effect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// <p>IPv4 CIDR ranges to include in the rule.</p>
    #[serde(rename = "IpRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<String>>,
    /// <p>The rule name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Access protocol actions to exclude from the rule. Valid values include <code>ActiveSync</code>, <code>AutoDiscover</code>, <code>EWS</code>, <code>IMAP</code>, <code>SMTP</code>, <code>WindowsOutlook</code>, and <code>WebMail</code>.</p>
    #[serde(rename = "NotActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_actions: Option<Vec<String>>,
    /// <p>IPv4 CIDR ranges to exclude from the rule.</p>
    #[serde(rename = "NotIpRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ip_ranges: Option<Vec<String>>,
    /// <p>User IDs to exclude from the rule.</p>
    #[serde(rename = "NotUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_user_ids: Option<Vec<String>>,
    /// <p>User IDs to include in the rule.</p>
    #[serde(rename = "UserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateDelegateToResourceRequest {
    /// <p>The member (user or group) to associate to the resource.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The organization under which the resource exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The resource for which members (users or groups) are associated.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDelegateToResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateMemberToGroupRequest {
    /// <p>The group to which the member (user or group) is associated.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The member (user or group) to associate to the group.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateMemberToGroupResponse {}

/// <p>At least one delegate must be associated to the resource to disable automatic replies from the resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BookingOptions {
    /// <p>The resource's ability to automatically reply to requests. If disabled, delegates must be associated to the resource.</p>
    #[serde(rename = "AutoAcceptRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_requests: Option<bool>,
    /// <p>The resource's ability to automatically decline any conflicting requests.</p>
    #[serde(rename = "AutoDeclineConflictingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_decline_conflicting_requests: Option<bool>,
    /// <p>The resource's ability to automatically decline any recurring requests.</p>
    #[serde(rename = "AutoDeclineRecurringRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_decline_recurring_requests: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelMailboxExportJobRequest {
    /// <p>The idempotency token for the client request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The job ID.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelMailboxExportJobResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAliasRequest {
    /// <p>The alias to add to the member set.</p>
    #[serde(rename = "Alias")]
    pub alias: String,
    /// <p>The member (user or group) to which this alias is added.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The organization under which the member (user or group) exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAliasResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGroupRequest {
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The organization under which the group is to be created.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupResponse {
    /// <p>The identifier of the group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOrganizationRequest {
    /// <p>The organization alias.</p>
    #[serde(rename = "Alias")]
    pub alias: String,
    /// <p>The idempotency token associated with the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The AWS Directory Service directory ID.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The email domains to associate with the organization.</p>
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<Domain>>,
    /// <p>When <code>true</code>, allows organization interoperability between Amazon WorkMail and Microsoft Exchange. Can only be set to <code>true</code> if an AD Connector directory ID is included in the request.</p>
    #[serde(rename = "EnableInteroperability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_interoperability: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a customer managed master key from AWS KMS.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOrganizationResponse {
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateResourceRequest {
    /// <p>The name of the new resource.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier associated with the organization for which the resource is created.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The type of the new resource. The available types are <code>equipment</code> and <code>room</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResourceResponse {
    /// <p>The identifier of the new resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserRequest {
    /// <p>The display name for the new user.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// <p>The name for the new user. WorkMail directory user names have a maximum length of 64. All others have a maximum length of 20.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier of the organization for which the user is created.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The password for the new user.</p>
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The identifier for the new user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>The name of the attribute, which is one of the values defined in the UserAttribute enumeration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Delegate {
    /// <p>The identifier for the user or group associated as the resource's delegate.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The type of the delegate: user or group.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAccessControlRuleRequest {
    /// <p>The name of the access control rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The identifier for the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAccessControlRuleResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAliasRequest {
    /// <p>The aliases to be removed from the user's set of aliases. Duplicate entries in the list are collapsed into single entries (the list is transformed into a set).</p>
    #[serde(rename = "Alias")]
    pub alias: String,
    /// <p>The identifier for the member (user or group) from which to have the aliases removed.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the user exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAliasResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGroupRequest {
    /// <p>The identifier of the group to be deleted.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The organization that contains the group.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGroupResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMailboxPermissionsRequest {
    /// <p>The identifier of the member (user or group) that owns the mailbox.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier of the member (user or group) for which to delete granted permissions.</p>
    #[serde(rename = "GranteeId")]
    pub grantee_id: String,
    /// <p>The identifier of the organization under which the member (user or group) exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMailboxPermissionsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOrganizationRequest {
    /// <p>The idempotency token associated with the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>If true, deletes the AWS Directory Service directory associated with the organization.</p>
    #[serde(rename = "DeleteDirectory")]
    pub delete_directory: bool,
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteOrganizationResponse {
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The state of the organization.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResourceRequest {
    /// <p>The identifier associated with the organization from which the resource is deleted.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource to be deleted.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRetentionPolicyRequest {
    /// <p>The retention policy ID.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRetentionPolicyResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>The organization that contains the user to be deleted.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the user to be deleted.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterFromWorkMailRequest {
    /// <p>The identifier for the member (user or group) to be updated.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the Amazon WorkMail entity exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterFromWorkMailResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGroupRequest {
    /// <p>The identifier for the group to be described.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The identifier for the organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGroupResponse {
    /// <p>The date and time when a user was deregistered from WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the described group.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date and time when a user was registered to WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the described group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the described group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the user: enabled (registered to Amazon WorkMail) or disabled (deregistered or never registered to WorkMail).</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMailboxExportJobRequest {
    /// <p>The mailbox export job ID.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMailboxExportJobResponse {
    /// <p>The mailbox export job description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The mailbox export job end timestamp.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The identifier of the user or resource associated with the mailbox.</p>
    #[serde(rename = "EntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// <p>Error information for failed mailbox export jobs.</p>
    #[serde(rename = "ErrorInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<String>,
    /// <p>The estimated progress of the mailbox export job, in percentage points.</p>
    #[serde(rename = "EstimatedProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_progress: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the symmetric AWS Key Management Service (AWS KMS) key that encrypts the exported mailbox content.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that grants write permission to the Amazon Simple Storage Service (Amazon S3) bucket.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The name of the S3 bucket.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The path to the S3 bucket and file that the mailbox export job is exporting to.</p>
    #[serde(rename = "S3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    /// <p>The S3 bucket prefix.</p>
    #[serde(rename = "S3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
    /// <p>The mailbox export job start timestamp.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The state of the mailbox export job.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationRequest {
    /// <p>The identifier for the organization to be described.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationResponse {
    /// <p>The Amazon Resource Name (ARN) of the organization.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The alias for an organization.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The date at which the organization became usable in the WorkMail context, in UNIX epoch time format.</p>
    #[serde(rename = "CompletedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_date: Option<f64>,
    /// <p>The default mail domain associated with the organization.</p>
    #[serde(rename = "DefaultMailDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mail_domain: Option<String>,
    /// <p>The identifier for the directory associated with an Amazon WorkMail organization.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The type of directory associated with the WorkMail organization.</p>
    #[serde(rename = "DirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<String>,
    /// <p>(Optional) The error message indicating if unexpected behavior was encountered with regards to the organization.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of an organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The state of an organization.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeResourceRequest {
    /// <p>The identifier associated with the organization for which the resource is described.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource to be described.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeResourceResponse {
    /// <p>The booking options for the described resource.</p>
    #[serde(rename = "BookingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_options: Option<BookingOptions>,
    /// <p>The date and time when a resource was disabled from WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the described resource.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date and time when a resource was enabled for WorkMail, in UNIX epoch time format.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The name of the described resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier of the described resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The state of the resource: enabled (registered to Amazon WorkMail), disabled (deregistered or never registered to WorkMail), or deleted.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the described resource.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserRequest {
    /// <p>The identifier for the organization under which the user exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier for the user to be described.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>The date and time at which the user was disabled for Amazon WorkMail usage, in UNIX epoch time format.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The display name of the user.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email of the user.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date and time at which the user was enabled for Amazon WorkMail usage, in UNIX epoch time format.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The name for the user.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of a user: enabled (registered to Amazon WorkMail) or disabled (deregistered or never registered to WorkMail).</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The identifier for the described user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// <p>In certain cases, other entities are modeled as users. If interoperability is enabled, resources are imported into Amazon WorkMail as users. Because different WorkMail organizations rely on different directory types, administrators can distinguish between an unregistered user (account is disabled and has a user role) and the directory administrators. The values are USER, RESOURCE, and SYSTEM_USER.</p>
    #[serde(rename = "UserRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateDelegateFromResourceRequest {
    /// <p>The identifier for the member (user, group) to be removed from the resource's delegates.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the resource exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource from which delegates' set members are removed. </p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDelegateFromResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateMemberFromGroupRequest {
    /// <p>The identifier for the group from which members are removed.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The identifier for the member to be removed to the group.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The identifier for the organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateMemberFromGroupResponse {}

/// <p>The domain to associate with an Amazon WorkMail organization.</p> <p>When you configure a domain hosted in Amazon Route 53 (Route 53), all recommended DNS records are added to the organization when you create it. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/add_domain.html">Adding a domain</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Domain {
    /// <p>The fully qualified domain name.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The hosted zone ID for a domain hosted in Route 53. Required when configuring a domain hosted in Route 53.</p>
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
}

/// <p>The configuration applied to an organization's folders by its retention policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FolderConfiguration {
    /// <p>The action to take on the folder contents at the end of the folder configuration period.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The folder name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The period of time at which the folder configuration action is applied.</p>
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccessControlEffectRequest {
    /// <p>The access protocol action. Valid values include <code>ActiveSync</code>, <code>AutoDiscover</code>, <code>EWS</code>, <code>IMAP</code>, <code>SMTP</code>, <code>WindowsOutlook</code>, and <code>WebMail</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The IPv4 address.</p>
    #[serde(rename = "IpAddress")]
    pub ip_address: String,
    /// <p>The identifier for the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccessControlEffectResponse {
    /// <p>The rule effect.</p>
    #[serde(rename = "Effect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// <p>The rules that match the given parameters, resulting in an effect.</p>
    #[serde(rename = "MatchedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_rules: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDefaultRetentionPolicyRequest {
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDefaultRetentionPolicyResponse {
    /// <p>The retention policy description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The retention policy folder configurations.</p>
    #[serde(rename = "FolderConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_configurations: Option<Vec<FolderConfiguration>>,
    /// <p>The retention policy ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The retention policy name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMailboxDetailsRequest {
    /// <p>The identifier for the organization that contains the user whose mailbox details are being requested.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier for the user whose mailbox details are being requested.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMailboxDetailsResponse {
    /// <p>The maximum allowed mailbox size, in MB, for the specified user.</p>
    #[serde(rename = "MailboxQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_quota: Option<i64>,
    /// <p>The current mailbox size, in MB, for the specified user.</p>
    #[serde(rename = "MailboxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_size: Option<f64>,
}

/// <p>The representation of an Amazon WorkMail group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Group {
    /// <p>The date indicating when the group was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the group.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date indicating when the group was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the group, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAccessControlRulesRequest {
    /// <p>The identifier for the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAccessControlRulesResponse {
    /// <p>The access control rules.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AccessControlRule>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAliasesRequest {
    /// <p>The identifier for the entity for which to list the aliases.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the entity exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAliasesResponse {
    /// <p>The entity's paginated aliases.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGroupMembersRequest {
    /// <p>The identifier for the group to which the members (users or groups) are associated.</p>
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the group exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupMembersResponse {
    /// <p>The members associated to the group.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGroupsRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the groups exist.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroupsResponse {
    /// <p>The overview of groups for an organization.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMailboxExportJobsRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMailboxExportJobsResponse {
    /// <p>The mailbox export job details.</p>
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<MailboxExportJob>>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMailboxPermissionsRequest {
    /// <p>The identifier of the user, group, or resource for which to list mailbox permissions.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the organization under which the user, group, or resource exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMailboxPermissionsResponse {
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>One page of the user, group, or resource mailbox permissions.</p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOrganizationsRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOrganizationsResponse {
    /// <p>The token to use to retrieve the next page of results. The value is "null" when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The overview of owned organizations presented as a list of organization summaries.</p>
    #[serde(rename = "OrganizationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_summaries: Option<Vec<OrganizationSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourceDelegatesRequest {
    /// <p>The number of maximum results in a page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to paginate through the delegates associated with a resource.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization that contains the resource for which delegates are listed.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier for the resource whose delegates are listed.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceDelegatesResponse {
    /// <p>One page of the resource's delegates.</p>
    #[serde(rename = "Delegates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegates: Option<Vec<Delegate>>,
    /// <p>The token used to paginate through the delegates associated with a resource. While results are still available, it has an associated value. When the last page is reached, the token is empty. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourcesRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the resources exist.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourcesResponse {
    /// <p> The token used to paginate through all the organization's resources. While results are still available, it has an associated value. When the last page is reached, the token is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>One page of the organization's resource representation.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A list of tag key-value pairs.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUsersRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results. The first call does not contain any tokens.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the organization under which the users exist.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsersResponse {
    /// <p> The token to use to retrieve the next page of results. This value is `null` when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The overview of users for an organization.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

/// <p>The details of a mailbox export job, including the user or resource ID associated with the mailbox and the S3 bucket that the mailbox contents are exported to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MailboxExportJob {
    /// <p>The mailbox export job description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The mailbox export job end timestamp.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The identifier of the user or resource associated with the mailbox.</p>
    #[serde(rename = "EntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// <p>The estimated progress of the mailbox export job, in percentage points.</p>
    #[serde(rename = "EstimatedProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_progress: Option<i64>,
    /// <p>The identifier of the mailbox export job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name of the S3 bucket.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The path to the S3 bucket and file that the mailbox export job exports to.</p>
    #[serde(rename = "S3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    /// <p>The mailbox export job start timestamp.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The state of the mailbox export job.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>The representation of a user or group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Member {
    /// <p>The date indicating when the member was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The date indicating when the member was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the member.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the member.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the member, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A member can be a user or group.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The representation of an organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationSummary {
    /// <p>The alias associated with the organization.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The default email domain associated with the organization.</p>
    #[serde(rename = "DefaultMailDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mail_domain: Option<String>,
    /// <p>The error message associated with the organization. It is only present if unexpected behavior has occurred with regards to the organization. It provides insight or solutions regarding unexpected behavior.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier associated with the organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>The state associated with the organization.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Permission granted to a user, group, or resource to access a certain aspect of another user, group, or resource mailbox.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Permission {
    /// <p>The identifier of the user, group, or resource to which the permissions are granted.</p>
    #[serde(rename = "GranteeId")]
    pub grantee_id: String,
    /// <p>The type of user, group, or resource referred to in GranteeId.</p>
    #[serde(rename = "GranteeType")]
    pub grantee_type: String,
    /// <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full access to the mailbox, irrespective of other folder-level permissions set on the mailbox.</p>
    #[serde(rename = "PermissionValues")]
    pub permission_values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccessControlRuleRequest {
    /// <p>Access protocol actions to include in the rule. Valid values include <code>ActiveSync</code>, <code>AutoDiscover</code>, <code>EWS</code>, <code>IMAP</code>, <code>SMTP</code>, <code>WindowsOutlook</code>, and <code>WebMail</code>.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// <p>The rule description.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The rule effect.</p>
    #[serde(rename = "Effect")]
    pub effect: String,
    /// <p>IPv4 CIDR ranges to include in the rule.</p>
    #[serde(rename = "IpRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<String>>,
    /// <p>The rule name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Access protocol actions to exclude from the rule. Valid values include <code>ActiveSync</code>, <code>AutoDiscover</code>, <code>EWS</code>, <code>IMAP</code>, <code>SMTP</code>, <code>WindowsOutlook</code>, and <code>WebMail</code>.</p>
    #[serde(rename = "NotActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_actions: Option<Vec<String>>,
    /// <p>IPv4 CIDR ranges to exclude from the rule.</p>
    #[serde(rename = "NotIpRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ip_ranges: Option<Vec<String>>,
    /// <p>User IDs to exclude from the rule.</p>
    #[serde(rename = "NotUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_user_ids: Option<Vec<String>>,
    /// <p>The identifier of the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>User IDs to include in the rule.</p>
    #[serde(rename = "UserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccessControlRuleResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutMailboxPermissionsRequest {
    /// <p>The identifier of the user, group, or resource for which to update mailbox permissions.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier of the user, group, or resource to which to grant the permissions.</p>
    #[serde(rename = "GranteeId")]
    pub grantee_id: String,
    /// <p>The identifier of the organization under which the user, group, or resource exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full access to the mailbox, irrespective of other folder-level permissions set on the mailbox.</p>
    #[serde(rename = "PermissionValues")]
    pub permission_values: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutMailboxPermissionsResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRetentionPolicyRequest {
    /// <p>The retention policy description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The retention policy folder configurations.</p>
    #[serde(rename = "FolderConfigurations")]
    pub folder_configurations: Vec<FolderConfiguration>,
    /// <p>The retention policy ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The retention policy name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The organization ID.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRetentionPolicyResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterToWorkMailRequest {
    /// <p>The email for the user, group, or resource to be updated.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>The identifier for the user, group, or resource to be updated.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The identifier for the organization under which the user, group, or resource exists.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterToWorkMailResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetPasswordRequest {
    /// <p>The identifier of the organization that contains the user for which the password is reset.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The new password for the user.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The identifier of the user for whom the password is reset.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetPasswordResponse {}

/// <p>The representation of a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Resource {
    /// <p>The date indicating when the resource was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The email of the resource.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date indicating when the resource was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the resource.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the resource, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the resource: equipment or room.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMailboxExportJobRequest {
    /// <p>The idempotency token for the client request.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The mailbox export job description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the user or resource associated with the mailbox.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The Amazon Resource Name (ARN) of the symmetric AWS Key Management Service (AWS KMS) key that encrypts the exported mailbox content.</p>
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: String,
    /// <p>The identifier associated with the organization.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that grants write permission to the S3 bucket.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The name of the S3 bucket.</p>
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    /// <p>The S3 bucket prefix.</p>
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMailboxExportJobResponse {
    /// <p>The job ID.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Describes a tag applied to a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The tag key-value pairs.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMailboxQuotaRequest {
    /// <p>The updated mailbox quota, in MB, for the specified user.</p>
    #[serde(rename = "MailboxQuota")]
    pub mailbox_quota: i64,
    /// <p>The identifier for the organization that contains the user for whom to update the mailbox quota.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifer for the user for whom to update the mailbox quota.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMailboxQuotaResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePrimaryEmailAddressRequest {
    /// <p>The value of the email to be updated as primary.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>The user, group, or resource to update.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
    /// <p>The organization that contains the user, group, or resource to update.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePrimaryEmailAddressResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateResourceRequest {
    /// <p>The resource's booking options to be updated.</p>
    #[serde(rename = "BookingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_options: Option<BookingOptions>,
    /// <p>The name of the resource to be updated.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier associated with the organization for which the resource is updated.</p>
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    /// <p>The identifier of the resource to be updated.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResourceResponse {}

/// <p>The representation of an Amazon WorkMail user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct User {
    /// <p>The date indicating when the user was disabled from Amazon WorkMail use.</p>
    #[serde(rename = "DisabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<f64>,
    /// <p>The display name of the user.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The email of the user.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The date indicating when the user was enabled for Amazon WorkMail use.</p>
    #[serde(rename = "EnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_date: Option<f64>,
    /// <p>The identifier of the user.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the user.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the user, which can be ENABLED, DISABLED, or DELETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The role of the user.</p>
    #[serde(rename = "UserRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
}

/// Errors returned by AssociateDelegateToResource
#[derive(Debug, PartialEq)]
pub enum AssociateDelegateToResourceError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl AssociateDelegateToResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateDelegateToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(AssociateDelegateToResourceError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(AssociateDelegateToResourceError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateDelegateToResourceError::InvalidParameter(err.msg),
                    )
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        AssociateDelegateToResourceError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(
                        AssociateDelegateToResourceError::OrganizationState(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateDelegateToResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateDelegateToResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            AssociateDelegateToResourceError::EntityState(ref cause) => write!(f, "{}", cause),
            AssociateDelegateToResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateDelegateToResourceError::OrganizationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateDelegateToResourceError::OrganizationState(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateDelegateToResourceError {}
/// Errors returned by AssociateMemberToGroup
#[derive(Debug, PartialEq)]
pub enum AssociateMemberToGroupError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl AssociateMemberToGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateMemberToGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        AssociateMemberToGroupError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::OrganizationState(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(AssociateMemberToGroupError::UnsupportedOperation(
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
impl fmt::Display for AssociateMemberToGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateMemberToGroupError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateMemberToGroupError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            AssociateMemberToGroupError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            AssociateMemberToGroupError::EntityState(ref cause) => write!(f, "{}", cause),
            AssociateMemberToGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateMemberToGroupError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            AssociateMemberToGroupError::OrganizationState(ref cause) => write!(f, "{}", cause),
            AssociateMemberToGroupError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateMemberToGroupError {}
/// Errors returned by CancelMailboxExportJob
#[derive(Debug, PartialEq)]
pub enum CancelMailboxExportJobError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl CancelMailboxExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelMailboxExportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(CancelMailboxExportJobError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelMailboxExportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CancelMailboxExportJobError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CancelMailboxExportJobError::OrganizationState(
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
impl fmt::Display for CancelMailboxExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelMailboxExportJobError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            CancelMailboxExportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelMailboxExportJobError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            CancelMailboxExportJobError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelMailboxExportJobError {}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The request exceeds the limit of the resource.</p>
    LimitExceeded(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EmailAddressInUseException" => {
                    return RusotoError::Service(CreateAliasError::EmailAddressInUse(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateAliasError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(CreateAliasError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateAliasError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAliasError::LimitExceeded(err.msg))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(CreateAliasError::MailDomainNotFound(err.msg))
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(CreateAliasError::MailDomainState(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateAliasError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateAliasError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAliasError::EmailAddressInUse(ref cause) => write!(f, "{}", cause),
            CreateAliasError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            CreateAliasError::EntityState(ref cause) => write!(f, "{}", cause),
            CreateAliasError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAliasError::MailDomainNotFound(ref cause) => write!(f, "{}", cause),
            CreateAliasError::MailDomainState(ref cause) => write!(f, "{}", cause),
            CreateAliasError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            CreateAliasError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAliasError {}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The user, group, or resource name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>This user, group, or resource name is not allowed in Amazon WorkMail.</p>
    ReservedName(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        CreateGroupError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateGroupError::DirectoryUnavailable(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateGroupError::InvalidParameter(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(CreateGroupError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateGroupError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateGroupError::OrganizationState(err.msg))
                }
                "ReservedNameException" => {
                    return RusotoError::Service(CreateGroupError::ReservedName(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateGroupError::UnsupportedOperation(err.msg))
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
            CreateGroupError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateGroupError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            CreateGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateGroupError::NameAvailability(ref cause) => write!(f, "{}", cause),
            CreateGroupError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            CreateGroupError::OrganizationState(ref cause) => write!(f, "{}", cause),
            CreateGroupError::ReservedName(ref cause) => write!(f, "{}", cause),
            CreateGroupError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGroupError {}
/// Errors returned by CreateOrganization
#[derive(Debug, PartialEq)]
pub enum CreateOrganizationError {
    /// <p>The directory is already in use by another WorkMail organization in the same account and Region.</p>
    DirectoryInUse(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The request exceeds the limit of the resource.</p>
    LimitExceeded(String),
    /// <p>The user, group, or resource name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
}

impl CreateOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryInUseException" => {
                    return RusotoError::Service(CreateOrganizationError::DirectoryInUse(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateOrganizationError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateOrganizationError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateOrganizationError::LimitExceeded(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(CreateOrganizationError::NameAvailability(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOrganizationError::DirectoryInUse(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::NameAvailability(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateOrganizationError {}
/// Errors returned by CreateResource
#[derive(Debug, PartialEq)]
pub enum CreateResourceError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The user, group, or resource name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>This user, group, or resource name is not allowed in Amazon WorkMail.</p>
    ReservedName(String),
}

impl CreateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        CreateResourceError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateResourceError::DirectoryUnavailable(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateResourceError::InvalidParameter(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(CreateResourceError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateResourceError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateResourceError::OrganizationState(err.msg))
                }
                "ReservedNameException" => {
                    return RusotoError::Service(CreateResourceError::ReservedName(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateResourceError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateResourceError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            CreateResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateResourceError::NameAvailability(ref cause) => write!(f, "{}", cause),
            CreateResourceError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            CreateResourceError::OrganizationState(ref cause) => write!(f, "{}", cause),
            CreateResourceError::ReservedName(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateResourceError {}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The supplied password doesn't match the minimum security constraints, such as length or use of special characters.</p>
    InvalidPassword(String),
    /// <p>The user, group, or resource name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>This user, group, or resource name is not allowed in Amazon WorkMail.</p>
    ReservedName(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        CreateUserError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateUserError::DirectoryUnavailable(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(CreateUserError::InvalidPassword(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(CreateUserError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(CreateUserError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(CreateUserError::OrganizationState(err.msg))
                }
                "ReservedNameException" => {
                    return RusotoError::Service(CreateUserError::ReservedName(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateUserError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUserError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            CreateUserError::NameAvailability(ref cause) => write!(f, "{}", cause),
            CreateUserError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            CreateUserError::OrganizationState(ref cause) => write!(f, "{}", cause),
            CreateUserError::ReservedName(ref cause) => write!(f, "{}", cause),
            CreateUserError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by DeleteAccessControlRule
#[derive(Debug, PartialEq)]
pub enum DeleteAccessControlRuleError {
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DeleteAccessControlRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAccessControlRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DeleteAccessControlRuleError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteAccessControlRuleError::OrganizationState(
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
impl fmt::Display for DeleteAccessControlRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAccessControlRuleError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAccessControlRuleError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAccessControlRuleError {}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteAliasError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAliasError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteAliasError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAliasError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::EntityState(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAliasError {}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        DeleteGroupError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(DeleteGroupError::DirectoryUnavailable(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteGroupError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteGroupError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteGroupError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteGroupError::OrganizationState(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteGroupError::UnsupportedOperation(err.msg))
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
            DeleteGroupError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteGroupError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::EntityState(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::OrganizationState(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGroupError {}
/// Errors returned by DeleteMailboxPermissions
#[derive(Debug, PartialEq)]
pub enum DeleteMailboxPermissionsError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DeleteMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMailboxPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DeleteMailboxPermissionsError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteMailboxPermissionsError::OrganizationState(
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
impl fmt::Display for DeleteMailboxPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMailboxPermissionsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DeleteMailboxPermissionsError::EntityState(ref cause) => write!(f, "{}", cause),
            DeleteMailboxPermissionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteMailboxPermissionsError::OrganizationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteMailboxPermissionsError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMailboxPermissionsError {}
/// Errors returned by DeleteOrganization
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DeleteOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteOrganizationError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteOrganizationError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteOrganizationError::OrganizationState(
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
impl fmt::Display for DeleteOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOrganizationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOrganizationError {}
/// Errors returned by DeleteResource
#[derive(Debug, PartialEq)]
pub enum DeleteResourceError {
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DeleteResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityStateException" => {
                    return RusotoError::Service(DeleteResourceError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResourceError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteResourceError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteResourceError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResourceError::EntityState(ref cause) => write!(f, "{}", cause),
            DeleteResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteResourceError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeleteResourceError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResourceError {}
/// Errors returned by DeleteRetentionPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteRetentionPolicyError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DeleteRetentionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRetentionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteRetentionPolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteRetentionPolicyError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteRetentionPolicyError::OrganizationState(
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
impl fmt::Display for DeleteRetentionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRetentionPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRetentionPolicyError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeleteRetentionPolicyError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRetentionPolicyError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        DeleteUserError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(DeleteUserError::DirectoryUnavailable(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeleteUserError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeleteUserError::OrganizationState(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteUserError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteUserError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteUserError::EntityState(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserError::OrganizationState(ref cause) => write!(f, "{}", cause),
            DeleteUserError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DeregisterFromWorkMail
#[derive(Debug, PartialEq)]
pub enum DeregisterFromWorkMailError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DeregisterFromWorkMailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterFromWorkMailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DeregisterFromWorkMailError::OrganizationState(
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
impl fmt::Display for DeregisterFromWorkMailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterFromWorkMailError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DeregisterFromWorkMailError::EntityState(ref cause) => write!(f, "{}", cause),
            DeregisterFromWorkMailError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeregisterFromWorkMailError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DeregisterFromWorkMailError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterFromWorkMailError {}
/// Errors returned by DescribeGroup
#[derive(Debug, PartialEq)]
pub enum DescribeGroupError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DescribeGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeGroupError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeGroupError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeGroupError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DescribeGroupError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGroupError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DescribeGroupError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGroupError {}
/// Errors returned by DescribeMailboxExportJob
#[derive(Debug, PartialEq)]
pub enum DescribeMailboxExportJobError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DescribeMailboxExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMailboxExportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeMailboxExportJobError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeMailboxExportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DescribeMailboxExportJobError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DescribeMailboxExportJobError::OrganizationState(
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
impl fmt::Display for DescribeMailboxExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMailboxExportJobError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DescribeMailboxExportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeMailboxExportJobError::OrganizationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMailboxExportJobError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMailboxExportJobError {}
/// Errors returned by DescribeOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
}

impl DescribeOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeOrganizationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeOrganizationError::OrganizationNotFound(
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
impl fmt::Display for DescribeOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOrganizationError {}
/// Errors returned by DescribeResource
#[derive(Debug, PartialEq)]
pub enum DescribeResourceError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DescribeResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeResourceError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeResourceError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeResourceError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DescribeResourceError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DescribeResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeResourceError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DescribeResourceError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeResourceError {}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(DescribeUserError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserError {}
/// Errors returned by DisassociateDelegateFromResource
#[derive(Debug, PartialEq)]
pub enum DisassociateDelegateFromResourceError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl DisassociateDelegateFromResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateDelegateFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::EntityNotFound(err.msg),
                    )
                }
                "EntityStateException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::EntityState(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::InvalidParameter(err.msg),
                    )
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(
                        DisassociateDelegateFromResourceError::OrganizationState(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateDelegateFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateDelegateFromResourceError::EntityNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateDelegateFromResourceError::EntityState(ref cause) => write!(f, "{}", cause),
            DisassociateDelegateFromResourceError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateDelegateFromResourceError::OrganizationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateDelegateFromResourceError::OrganizationState(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateDelegateFromResourceError {}
/// Errors returned by DisassociateMemberFromGroup
#[derive(Debug, PartialEq)]
pub enum DisassociateMemberFromGroupError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl DisassociateMemberFromGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateMemberFromGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::DirectoryServiceAuthenticationFailed(
                            err.msg,
                        ),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(DisassociateMemberFromGroupError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(DisassociateMemberFromGroupError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::InvalidParameter(err.msg),
                    )
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::OrganizationState(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DisassociateMemberFromGroupError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateMemberFromGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateMemberFromGroupError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateMemberFromGroupError::DirectoryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateMemberFromGroupError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateMemberFromGroupError::EntityState(ref cause) => write!(f, "{}", cause),
            DisassociateMemberFromGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateMemberFromGroupError::OrganizationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateMemberFromGroupError::OrganizationState(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateMemberFromGroupError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateMemberFromGroupError {}
/// Errors returned by GetAccessControlEffect
#[derive(Debug, PartialEq)]
pub enum GetAccessControlEffectError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl GetAccessControlEffectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccessControlEffectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetAccessControlEffectError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAccessControlEffectError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(GetAccessControlEffectError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(GetAccessControlEffectError::OrganizationState(
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
impl fmt::Display for GetAccessControlEffectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccessControlEffectError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            GetAccessControlEffectError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetAccessControlEffectError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            GetAccessControlEffectError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccessControlEffectError {}
/// Errors returned by GetDefaultRetentionPolicy
#[derive(Debug, PartialEq)]
pub enum GetDefaultRetentionPolicyError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl GetDefaultRetentionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDefaultRetentionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDefaultRetentionPolicyError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDefaultRetentionPolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        GetDefaultRetentionPolicyError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(GetDefaultRetentionPolicyError::OrganizationState(
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
impl fmt::Display for GetDefaultRetentionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDefaultRetentionPolicyError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            GetDefaultRetentionPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDefaultRetentionPolicyError::OrganizationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDefaultRetentionPolicyError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDefaultRetentionPolicyError {}
/// Errors returned by GetMailboxDetails
#[derive(Debug, PartialEq)]
pub enum GetMailboxDetailsError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl GetMailboxDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMailboxDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetMailboxDetailsError::EntityNotFound(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(GetMailboxDetailsError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(GetMailboxDetailsError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMailboxDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMailboxDetailsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            GetMailboxDetailsError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            GetMailboxDetailsError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMailboxDetailsError {}
/// Errors returned by ListAccessControlRules
#[derive(Debug, PartialEq)]
pub enum ListAccessControlRulesError {
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListAccessControlRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccessControlRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListAccessControlRulesError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListAccessControlRulesError::OrganizationState(
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
impl fmt::Display for ListAccessControlRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccessControlRulesError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListAccessControlRulesError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccessControlRulesError {}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListAliasesError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ListAliasesError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAliasesError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListAliasesError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListAliasesError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAliasesError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            ListAliasesError::EntityState(ref cause) => write!(f, "{}", cause),
            ListAliasesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListAliasesError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListAliasesError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAliasesError {}
/// Errors returned by ListGroupMembers
#[derive(Debug, PartialEq)]
pub enum ListGroupMembersError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListGroupMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupMembersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListGroupMembersError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ListGroupMembersError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListGroupMembersError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListGroupMembersError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListGroupMembersError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGroupMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGroupMembersError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            ListGroupMembersError::EntityState(ref cause) => write!(f, "{}", cause),
            ListGroupMembersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListGroupMembersError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListGroupMembersError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroupMembersError {}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListGroupsError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListGroupsError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListGroupsError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListGroupsError::OrganizationState(err.msg))
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
            ListGroupsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            ListGroupsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListGroupsError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListGroupsError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroupsError {}
/// Errors returned by ListMailboxExportJobs
#[derive(Debug, PartialEq)]
pub enum ListMailboxExportJobsError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListMailboxExportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMailboxExportJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListMailboxExportJobsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListMailboxExportJobsError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListMailboxExportJobsError::OrganizationState(
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
impl fmt::Display for ListMailboxExportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMailboxExportJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListMailboxExportJobsError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListMailboxExportJobsError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMailboxExportJobsError {}
/// Errors returned by ListMailboxPermissions
#[derive(Debug, PartialEq)]
pub enum ListMailboxPermissionsError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMailboxPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListMailboxPermissionsError::OrganizationState(
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
impl fmt::Display for ListMailboxPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMailboxPermissionsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            ListMailboxPermissionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListMailboxPermissionsError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListMailboxPermissionsError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMailboxPermissionsError {}
/// Errors returned by ListOrganizations
#[derive(Debug, PartialEq)]
pub enum ListOrganizationsError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
}

impl ListOrganizationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOrganizationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListOrganizationsError::InvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOrganizationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOrganizationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOrganizationsError {}
/// Errors returned by ListResourceDelegates
#[derive(Debug, PartialEq)]
pub enum ListResourceDelegatesError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListResourceDelegatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceDelegatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ListResourceDelegatesError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ListResourceDelegatesError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResourceDelegatesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListResourceDelegatesError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListResourceDelegatesError::OrganizationState(
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
impl fmt::Display for ListResourceDelegatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourceDelegatesError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            ListResourceDelegatesError::EntityState(ref cause) => write!(f, "{}", cause),
            ListResourceDelegatesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResourceDelegatesError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListResourceDelegatesError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourceDelegatesError {}
/// Errors returned by ListResources
#[derive(Debug, PartialEq)]
pub enum ListResourcesError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResourcesError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListResourcesError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListResourcesError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResourcesError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListResourcesError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourcesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
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
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ListUsersError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ListUsersError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUsersError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ListUsersError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersError {}
/// Errors returned by PutAccessControlRule
#[derive(Debug, PartialEq)]
pub enum PutAccessControlRuleError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The request exceeds the limit of the resource.</p>
    LimitExceeded(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl PutAccessControlRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAccessControlRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(PutAccessControlRuleError::EntityNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAccessControlRuleError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutAccessControlRuleError::LimitExceeded(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(PutAccessControlRuleError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(PutAccessControlRuleError::OrganizationState(
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
impl fmt::Display for PutAccessControlRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccessControlRuleError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            PutAccessControlRuleError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutAccessControlRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutAccessControlRuleError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            PutAccessControlRuleError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAccessControlRuleError {}
/// Errors returned by PutMailboxPermissions
#[derive(Debug, PartialEq)]
pub enum PutMailboxPermissionsError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl PutMailboxPermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutMailboxPermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(PutMailboxPermissionsError::OrganizationState(
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
impl fmt::Display for PutMailboxPermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutMailboxPermissionsError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            PutMailboxPermissionsError::EntityState(ref cause) => write!(f, "{}", cause),
            PutMailboxPermissionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutMailboxPermissionsError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            PutMailboxPermissionsError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutMailboxPermissionsError {}
/// Errors returned by PutRetentionPolicy
#[derive(Debug, PartialEq)]
pub enum PutRetentionPolicyError {
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The request exceeds the limit of the resource.</p>
    LimitExceeded(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl PutRetentionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRetentionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutRetentionPolicyError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutRetentionPolicyError::LimitExceeded(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(PutRetentionPolicyError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(PutRetentionPolicyError::OrganizationState(
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
impl fmt::Display for PutRetentionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRetentionPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutRetentionPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutRetentionPolicyError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            PutRetentionPolicyError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRetentionPolicyError {}
/// Errors returned by RegisterToWorkMail
#[derive(Debug, PartialEq)]
pub enum RegisterToWorkMailError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The user, group, or resource that you're trying to register is already registered.</p>
    EntityAlreadyRegistered(String),
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl RegisterToWorkMailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterToWorkMailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        RegisterToWorkMailError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(RegisterToWorkMailError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EmailAddressInUseException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EmailAddressInUse(
                        err.msg,
                    ))
                }
                "EntityAlreadyRegisteredException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EntityAlreadyRegistered(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(RegisterToWorkMailError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterToWorkMailError::InvalidParameter(err.msg))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(RegisterToWorkMailError::MailDomainNotFound(
                        err.msg,
                    ))
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(RegisterToWorkMailError::MailDomainState(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(RegisterToWorkMailError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(RegisterToWorkMailError::OrganizationState(
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
impl fmt::Display for RegisterToWorkMailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterToWorkMailError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterToWorkMailError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::EmailAddressInUse(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::EntityAlreadyRegistered(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::EntityState(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::MailDomainNotFound(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::MailDomainState(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            RegisterToWorkMailError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterToWorkMailError {}
/// Errors returned by ResetPassword
#[derive(Debug, PartialEq)]
pub enum ResetPasswordError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The supplied password doesn't match the minimum security constraints, such as length or use of special characters.</p>
    InvalidPassword(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl ResetPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        ResetPasswordError::DirectoryServiceAuthenticationFailed(err.msg),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(ResetPasswordError::DirectoryUnavailable(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(ResetPasswordError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(ResetPasswordError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ResetPasswordError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ResetPasswordError::InvalidPassword(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(ResetPasswordError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(ResetPasswordError::OrganizationState(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ResetPasswordError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResetPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResetPasswordError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            ResetPasswordError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            ResetPasswordError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            ResetPasswordError::EntityState(ref cause) => write!(f, "{}", cause),
            ResetPasswordError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ResetPasswordError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            ResetPasswordError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            ResetPasswordError::OrganizationState(ref cause) => write!(f, "{}", cause),
            ResetPasswordError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResetPasswordError {}
/// Errors returned by StartMailboxExportJob
#[derive(Debug, PartialEq)]
pub enum StartMailboxExportJobError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>The request exceeds the limit of the resource.</p>
    LimitExceeded(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl StartMailboxExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMailboxExportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartMailboxExportJobError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartMailboxExportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartMailboxExportJobError::LimitExceeded(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(StartMailboxExportJobError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(StartMailboxExportJobError::OrganizationState(
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
impl fmt::Display for StartMailboxExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMailboxExportJobError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            StartMailboxExportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartMailboxExportJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartMailboxExportJobError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            StartMailboxExportJobError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMailboxExportJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The resource can have up to 50 user-applied tags.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "OrganizationStateException" => {
                    return RusotoError::Service(TagResourceError::OrganizationState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
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
            TagResourceError::OrganizationState(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The resource cannot be found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
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
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateMailboxQuota
#[derive(Debug, PartialEq)]
pub enum UpdateMailboxQuotaError {
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl UpdateMailboxQuotaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMailboxQuotaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateMailboxQuotaError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(UpdateMailboxQuotaError::EntityState(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateMailboxQuotaError::InvalidParameter(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(UpdateMailboxQuotaError::OrganizationNotFound(
                        err.msg,
                    ))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(UpdateMailboxQuotaError::OrganizationState(
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
impl fmt::Display for UpdateMailboxQuotaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMailboxQuotaError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            UpdateMailboxQuotaError::EntityState(ref cause) => write!(f, "{}", cause),
            UpdateMailboxQuotaError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateMailboxQuotaError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            UpdateMailboxQuotaError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMailboxQuotaError {}
/// Errors returned by UpdatePrimaryEmailAddress
#[derive(Debug, PartialEq)]
pub enum UpdatePrimaryEmailAddressError {
    /// <p>The directory service doesn't recognize the credentials supplied by WorkMail.</p>
    DirectoryServiceAuthenticationFailed(String),
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>One or more of the input parameters don't match the service's restrictions.</p>
    InvalidParameter(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
    /// <p>You can't perform a write operation against a read-only directory.</p>
    UnsupportedOperation(String),
}

impl UpdatePrimaryEmailAddressError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePrimaryEmailAddressError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryServiceAuthenticationFailedException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::DirectoryServiceAuthenticationFailed(
                            err.msg,
                        ),
                    )
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::DirectoryUnavailable(err.msg),
                    )
                }
                "EmailAddressInUseException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::EmailAddressInUse(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::EntityNotFound(
                        err.msg,
                    ))
                }
                "EntityStateException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::EntityState(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::MailDomainNotFound(err.msg),
                    )
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::MailDomainState(
                        err.msg,
                    ))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::OrganizationNotFound(err.msg),
                    )
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(UpdatePrimaryEmailAddressError::OrganizationState(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        UpdatePrimaryEmailAddressError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePrimaryEmailAddressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePrimaryEmailAddressError::DirectoryServiceAuthenticationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePrimaryEmailAddressError::DirectoryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePrimaryEmailAddressError::EmailAddressInUse(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryEmailAddressError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryEmailAddressError::EntityState(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryEmailAddressError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryEmailAddressError::MailDomainNotFound(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryEmailAddressError::MailDomainState(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryEmailAddressError::OrganizationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePrimaryEmailAddressError::OrganizationState(ref cause) => write!(f, "{}", cause),
            UpdatePrimaryEmailAddressError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdatePrimaryEmailAddressError {}
/// Errors returned by UpdateResource
#[derive(Debug, PartialEq)]
pub enum UpdateResourceError {
    /// <p>The directory is unavailable. It might be located in another Region or deleted.</p>
    DirectoryUnavailable(String),
    /// <p>The email address that you're trying to assign is already created for a different user, group, or resource.</p>
    EmailAddressInUse(String),
    /// <p>The identifier supplied for the user, group, or resource does not exist in your organization.</p>
    EntityNotFound(String),
    /// <p>You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user.</p>
    EntityState(String),
    /// <p>The configuration for a resource isn't valid. A resource must either be able to auto-respond to requests or have at least one delegate associated that can do so on its behalf.</p>
    InvalidConfiguration(String),
    /// <p>For an email or alias to be created in Amazon WorkMail, the included domain must be defined in the organization.</p>
    MailDomainNotFound(String),
    /// <p>After a domain has been added to the organization, it must be verified. The domain is not yet verified.</p>
    MailDomainState(String),
    /// <p>The user, group, or resource name isn't unique in Amazon WorkMail.</p>
    NameAvailability(String),
    /// <p>An operation received a valid organization identifier that either doesn't belong or exist in the system.</p>
    OrganizationNotFound(String),
    /// <p>The organization must have a valid state to perform certain operations on the organization or its members.</p>
    OrganizationState(String),
}

impl UpdateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(UpdateResourceError::DirectoryUnavailable(err.msg))
                }
                "EmailAddressInUseException" => {
                    return RusotoError::Service(UpdateResourceError::EmailAddressInUse(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::EntityNotFound(err.msg))
                }
                "EntityStateException" => {
                    return RusotoError::Service(UpdateResourceError::EntityState(err.msg))
                }
                "InvalidConfigurationException" => {
                    return RusotoError::Service(UpdateResourceError::InvalidConfiguration(err.msg))
                }
                "MailDomainNotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::MailDomainNotFound(err.msg))
                }
                "MailDomainStateException" => {
                    return RusotoError::Service(UpdateResourceError::MailDomainState(err.msg))
                }
                "NameAvailabilityException" => {
                    return RusotoError::Service(UpdateResourceError::NameAvailability(err.msg))
                }
                "OrganizationNotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::OrganizationNotFound(err.msg))
                }
                "OrganizationStateException" => {
                    return RusotoError::Service(UpdateResourceError::OrganizationState(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateResourceError::DirectoryUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::EmailAddressInUse(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::EntityNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::EntityState(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::InvalidConfiguration(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::MailDomainNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::MailDomainState(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::NameAvailability(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::OrganizationNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResourceError::OrganizationState(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateResourceError {}
/// Trait representing the capabilities of the Amazon WorkMail API. Amazon WorkMail clients implement this trait.
#[async_trait]
pub trait Workmail {
    /// <p>Adds a member (user or group) to the resource's set of delegates.</p>
    async fn associate_delegate_to_resource(
        &self,
        input: AssociateDelegateToResourceRequest,
    ) -> Result<AssociateDelegateToResourceResponse, RusotoError<AssociateDelegateToResourceError>>;

    /// <p>Adds a member (user or group) to the group's set.</p>
    async fn associate_member_to_group(
        &self,
        input: AssociateMemberToGroupRequest,
    ) -> Result<AssociateMemberToGroupResponse, RusotoError<AssociateMemberToGroupError>>;

    /// <p><p>Cancels a mailbox export job.</p> <note> <p>If the mailbox export job is near completion, it might not be possible to cancel it.</p> </note></p>
    async fn cancel_mailbox_export_job(
        &self,
        input: CancelMailboxExportJobRequest,
    ) -> Result<CancelMailboxExportJobResponse, RusotoError<CancelMailboxExportJobError>>;

    /// <p>Adds an alias to the set of a given member (user or group) of Amazon WorkMail.</p>
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<CreateAliasResponse, RusotoError<CreateAliasError>>;

    /// <p>Creates a group that can be used in Amazon WorkMail by calling the <a>RegisterToWorkMail</a> operation.</p>
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, RusotoError<CreateGroupError>>;

    /// <p>Creates a new Amazon WorkMail organization. Optionally, you can choose to associate an existing AWS Directory Service directory with your organization. If an AWS Directory Service directory ID is specified, the organization alias must match the directory alias. If you choose not to associate an existing directory with your organization, then we create a new Amazon WorkMail directory for you. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/add_new_organization.html">Adding an organization</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p> <p>You can associate multiple email domains with an organization, then set your default email domain from the Amazon WorkMail console. You can also associate a domain that is managed in an Amazon Route 53 public hosted zone. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/add_domain.html">Adding a domain</a> and <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/default_domain.html">Choosing the default domain</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p> <p>Optionally, you can use a customer managed master key from AWS Key Management Service (AWS KMS) to encrypt email for your organization. If you don't associate an AWS KMS key, Amazon WorkMail creates a default AWS managed master key for you.</p>
    async fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse, RusotoError<CreateOrganizationError>>;

    /// <p>Creates a new Amazon WorkMail resource. </p>
    async fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> Result<CreateResourceResponse, RusotoError<CreateResourceError>>;

    /// <p>Creates a user who can be used in Amazon WorkMail by calling the <a>RegisterToWorkMail</a> operation.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>>;

    /// <p>Deletes an access control rule for the specified WorkMail organization.</p>
    async fn delete_access_control_rule(
        &self,
        input: DeleteAccessControlRuleRequest,
    ) -> Result<DeleteAccessControlRuleResponse, RusotoError<DeleteAccessControlRuleError>>;

    /// <p>Remove one or more specified aliases from a set of aliases for a given user.</p>
    async fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> Result<DeleteAliasResponse, RusotoError<DeleteAliasError>>;

    /// <p>Deletes a group from Amazon WorkMail.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<DeleteGroupResponse, RusotoError<DeleteGroupError>>;

    /// <p>Deletes permissions granted to a member (user or group).</p>
    async fn delete_mailbox_permissions(
        &self,
        input: DeleteMailboxPermissionsRequest,
    ) -> Result<DeleteMailboxPermissionsResponse, RusotoError<DeleteMailboxPermissionsError>>;

    /// <p>Deletes an Amazon WorkMail organization and all underlying AWS resources managed by Amazon WorkMail as part of the organization. You can choose whether to delete the associated directory. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/remove_organization.html">Removing an organization</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p>
    async fn delete_organization(
        &self,
        input: DeleteOrganizationRequest,
    ) -> Result<DeleteOrganizationResponse, RusotoError<DeleteOrganizationError>>;

    /// <p>Deletes the specified resource. </p>
    async fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> Result<DeleteResourceResponse, RusotoError<DeleteResourceError>>;

    /// <p>Deletes the specified retention policy from the specified organization.</p>
    async fn delete_retention_policy(
        &self,
        input: DeleteRetentionPolicyRequest,
    ) -> Result<DeleteRetentionPolicyResponse, RusotoError<DeleteRetentionPolicyError>>;

    /// <p>Deletes a user from Amazon WorkMail and all subsequent systems. Before you can delete a user, the user state must be <code>DISABLED</code>. Use the <a>DescribeUser</a> action to confirm the user state.</p> <p>Deleting a user is permanent and cannot be undone. WorkMail archives user mailboxes for 30 days before they are permanently removed.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, RusotoError<DeleteUserError>>;

    /// <p>Mark a user, group, or resource as no longer used in Amazon WorkMail. This action disassociates the mailbox and schedules it for clean-up. WorkMail keeps mailboxes for 30 days before they are permanently removed. The functionality in the console is <i>Disable</i>.</p>
    async fn deregister_from_work_mail(
        &self,
        input: DeregisterFromWorkMailRequest,
    ) -> Result<DeregisterFromWorkMailResponse, RusotoError<DeregisterFromWorkMailError>>;

    /// <p>Returns the data available for the group.</p>
    async fn describe_group(
        &self,
        input: DescribeGroupRequest,
    ) -> Result<DescribeGroupResponse, RusotoError<DescribeGroupError>>;

    /// <p>Describes the current status of a mailbox export job.</p>
    async fn describe_mailbox_export_job(
        &self,
        input: DescribeMailboxExportJobRequest,
    ) -> Result<DescribeMailboxExportJobResponse, RusotoError<DescribeMailboxExportJobError>>;

    /// <p>Provides more information regarding a given organization based on its identifier.</p>
    async fn describe_organization(
        &self,
        input: DescribeOrganizationRequest,
    ) -> Result<DescribeOrganizationResponse, RusotoError<DescribeOrganizationError>>;

    /// <p>Returns the data available for the resource.</p>
    async fn describe_resource(
        &self,
        input: DescribeResourceRequest,
    ) -> Result<DescribeResourceResponse, RusotoError<DescribeResourceError>>;

    /// <p>Provides information regarding the user.</p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>>;

    /// <p>Removes a member from the resource's set of delegates.</p>
    async fn disassociate_delegate_from_resource(
        &self,
        input: DisassociateDelegateFromResourceRequest,
    ) -> Result<
        DisassociateDelegateFromResourceResponse,
        RusotoError<DisassociateDelegateFromResourceError>,
    >;

    /// <p>Removes a member from a group.</p>
    async fn disassociate_member_from_group(
        &self,
        input: DisassociateMemberFromGroupRequest,
    ) -> Result<DisassociateMemberFromGroupResponse, RusotoError<DisassociateMemberFromGroupError>>;

    /// <p>Gets the effects of an organization's access control rules as they apply to a specified IPv4 address, access protocol action, or user ID. </p>
    async fn get_access_control_effect(
        &self,
        input: GetAccessControlEffectRequest,
    ) -> Result<GetAccessControlEffectResponse, RusotoError<GetAccessControlEffectError>>;

    /// <p>Gets the default retention policy details for the specified organization.</p>
    async fn get_default_retention_policy(
        &self,
        input: GetDefaultRetentionPolicyRequest,
    ) -> Result<GetDefaultRetentionPolicyResponse, RusotoError<GetDefaultRetentionPolicyError>>;

    /// <p>Requests a user's mailbox details for a specified organization and user.</p>
    async fn get_mailbox_details(
        &self,
        input: GetMailboxDetailsRequest,
    ) -> Result<GetMailboxDetailsResponse, RusotoError<GetMailboxDetailsError>>;

    /// <p>Lists the access control rules for the specified organization.</p>
    async fn list_access_control_rules(
        &self,
        input: ListAccessControlRulesRequest,
    ) -> Result<ListAccessControlRulesResponse, RusotoError<ListAccessControlRulesError>>;

    /// <p>Creates a paginated call to list the aliases associated with a given entity.</p>
    async fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, RusotoError<ListAliasesError>>;

    /// <p>Returns an overview of the members of a group. Users and groups can be members of a group.</p>
    async fn list_group_members(
        &self,
        input: ListGroupMembersRequest,
    ) -> Result<ListGroupMembersResponse, RusotoError<ListGroupMembersError>>;

    /// <p>Returns summaries of the organization's groups.</p>
    async fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, RusotoError<ListGroupsError>>;

    /// <p>Lists the mailbox export jobs started for the specified organization within the last seven days.</p>
    async fn list_mailbox_export_jobs(
        &self,
        input: ListMailboxExportJobsRequest,
    ) -> Result<ListMailboxExportJobsResponse, RusotoError<ListMailboxExportJobsError>>;

    /// <p>Lists the mailbox permissions associated with a user, group, or resource mailbox.</p>
    async fn list_mailbox_permissions(
        &self,
        input: ListMailboxPermissionsRequest,
    ) -> Result<ListMailboxPermissionsResponse, RusotoError<ListMailboxPermissionsError>>;

    /// <p>Returns summaries of the customer's organizations.</p>
    async fn list_organizations(
        &self,
        input: ListOrganizationsRequest,
    ) -> Result<ListOrganizationsResponse, RusotoError<ListOrganizationsError>>;

    /// <p>Lists the delegates associated with a resource. Users and groups can be resource delegates and answer requests on behalf of the resource.</p>
    async fn list_resource_delegates(
        &self,
        input: ListResourceDelegatesRequest,
    ) -> Result<ListResourceDelegatesResponse, RusotoError<ListResourceDelegatesError>>;

    /// <p>Returns summaries of the organization's resources.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResponse, RusotoError<ListResourcesError>>;

    /// <p>Lists the tags applied to an Amazon WorkMail organization resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns summaries of the organization's users.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>>;

    /// <p>Adds a new access control rule for the specified organization. The rule allows or denies access to the organization for the specified IPv4 addresses, access protocol actions, and user IDs. Adding a new rule with the same name as an existing rule replaces the older rule.</p>
    async fn put_access_control_rule(
        &self,
        input: PutAccessControlRuleRequest,
    ) -> Result<PutAccessControlRuleResponse, RusotoError<PutAccessControlRuleError>>;

    /// <p>Sets permissions for a user, group, or resource. This replaces any pre-existing permissions.</p>
    async fn put_mailbox_permissions(
        &self,
        input: PutMailboxPermissionsRequest,
    ) -> Result<PutMailboxPermissionsResponse, RusotoError<PutMailboxPermissionsError>>;

    /// <p>Puts a retention policy to the specified organization.</p>
    async fn put_retention_policy(
        &self,
        input: PutRetentionPolicyRequest,
    ) -> Result<PutRetentionPolicyResponse, RusotoError<PutRetentionPolicyError>>;

    /// <p>Registers an existing and disabled user, group, or resource for Amazon WorkMail use by associating a mailbox and calendaring capabilities. It performs no change if the user, group, or resource is enabled and fails if the user, group, or resource is deleted. This operation results in the accumulation of costs. For more information, see <a href="https://aws.amazon.com/workmail/pricing">Pricing</a>. The equivalent console functionality for this operation is <i>Enable</i>. </p> <p>Users can either be created by calling the <a>CreateUser</a> API operation or they can be synchronized from your directory. For more information, see <a>DeregisterFromWorkMail</a>.</p>
    async fn register_to_work_mail(
        &self,
        input: RegisterToWorkMailRequest,
    ) -> Result<RegisterToWorkMailResponse, RusotoError<RegisterToWorkMailError>>;

    /// <p>Allows the administrator to reset the password for a user.</p>
    async fn reset_password(
        &self,
        input: ResetPasswordRequest,
    ) -> Result<ResetPasswordResponse, RusotoError<ResetPasswordError>>;

    /// <p>Starts a mailbox export job to export MIME-format email messages and calendar items from the specified mailbox to the specified Amazon Simple Storage Service (Amazon S3) bucket. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/mail-export.html">Exporting mailbox content</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p>
    async fn start_mailbox_export_job(
        &self,
        input: StartMailboxExportJobRequest,
    ) -> Result<StartMailboxExportJobResponse, RusotoError<StartMailboxExportJobError>>;

    /// <p>Applies the specified tags to the specified Amazon WorkMail organization resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Untags the specified tags from the specified Amazon WorkMail organization resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates a user's current mailbox quota for a specified organization and user.</p>
    async fn update_mailbox_quota(
        &self,
        input: UpdateMailboxQuotaRequest,
    ) -> Result<UpdateMailboxQuotaResponse, RusotoError<UpdateMailboxQuotaError>>;

    /// <p>Updates the primary email for a user, group, or resource. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email), and the email provided in the input is promoted as the primary.</p>
    async fn update_primary_email_address(
        &self,
        input: UpdatePrimaryEmailAddressRequest,
    ) -> Result<UpdatePrimaryEmailAddressResponse, RusotoError<UpdatePrimaryEmailAddressError>>;

    /// <p>Updates data for the resource. To have the latest information, it must be preceded by a <a>DescribeResource</a> call. The dataset in the request should be the one expected when performing another <code>DescribeResource</code> call.</p>
    async fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> Result<UpdateResourceResponse, RusotoError<UpdateResourceError>>;
}
/// A client for the Amazon WorkMail API.
#[derive(Clone)]
pub struct WorkmailClient {
    client: Client,
    region: region::Region,
}

impl WorkmailClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WorkmailClient {
        WorkmailClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorkmailClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        WorkmailClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> WorkmailClient {
        WorkmailClient { client, region }
    }
}

#[async_trait]
impl Workmail for WorkmailClient {
    /// <p>Adds a member (user or group) to the resource's set of delegates.</p>
    async fn associate_delegate_to_resource(
        &self,
        input: AssociateDelegateToResourceRequest,
    ) -> Result<AssociateDelegateToResourceResponse, RusotoError<AssociateDelegateToResourceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "WorkMailService.AssociateDelegateToResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateDelegateToResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateDelegateToResourceResponse, _>()
    }

    /// <p>Adds a member (user or group) to the group's set.</p>
    async fn associate_member_to_group(
        &self,
        input: AssociateMemberToGroupRequest,
    ) -> Result<AssociateMemberToGroupResponse, RusotoError<AssociateMemberToGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.AssociateMemberToGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateMemberToGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateMemberToGroupResponse, _>()
    }

    /// <p><p>Cancels a mailbox export job.</p> <note> <p>If the mailbox export job is near completion, it might not be possible to cancel it.</p> </note></p>
    async fn cancel_mailbox_export_job(
        &self,
        input: CancelMailboxExportJobRequest,
    ) -> Result<CancelMailboxExportJobResponse, RusotoError<CancelMailboxExportJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.CancelMailboxExportJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelMailboxExportJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CancelMailboxExportJobResponse, _>()
    }

    /// <p>Adds an alias to the set of a given member (user or group) of Amazon WorkMail.</p>
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<CreateAliasResponse, RusotoError<CreateAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAliasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAliasResponse, _>()
    }

    /// <p>Creates a group that can be used in Amazon WorkMail by calling the <a>RegisterToWorkMail</a> operation.</p>
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, RusotoError<CreateGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.CreateGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateGroupResponse, _>()
    }

    /// <p>Creates a new Amazon WorkMail organization. Optionally, you can choose to associate an existing AWS Directory Service directory with your organization. If an AWS Directory Service directory ID is specified, the organization alias must match the directory alias. If you choose not to associate an existing directory with your organization, then we create a new Amazon WorkMail directory for you. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/add_new_organization.html">Adding an organization</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p> <p>You can associate multiple email domains with an organization, then set your default email domain from the Amazon WorkMail console. You can also associate a domain that is managed in an Amazon Route 53 public hosted zone. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/add_domain.html">Adding a domain</a> and <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/default_domain.html">Choosing the default domain</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p> <p>Optionally, you can use a customer managed master key from AWS Key Management Service (AWS KMS) to encrypt email for your organization. If you don't associate an AWS KMS key, Amazon WorkMail creates a default AWS managed master key for you.</p>
    async fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse, RusotoError<CreateOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.CreateOrganization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateOrganizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateOrganizationResponse, _>()
    }

    /// <p>Creates a new Amazon WorkMail resource. </p>
    async fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> Result<CreateResourceResponse, RusotoError<CreateResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.CreateResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateResourceResponse, _>()
    }

    /// <p>Creates a user who can be used in Amazon WorkMail by calling the <a>RegisterToWorkMail</a> operation.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.CreateUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateUserResponse, _>()
    }

    /// <p>Deletes an access control rule for the specified WorkMail organization.</p>
    async fn delete_access_control_rule(
        &self,
        input: DeleteAccessControlRuleRequest,
    ) -> Result<DeleteAccessControlRuleResponse, RusotoError<DeleteAccessControlRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteAccessControlRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAccessControlRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAccessControlRuleResponse, _>()
    }

    /// <p>Remove one or more specified aliases from a set of aliases for a given user.</p>
    async fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> Result<DeleteAliasResponse, RusotoError<DeleteAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAliasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteAliasResponse, _>()
    }

    /// <p>Deletes a group from Amazon WorkMail.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<DeleteGroupResponse, RusotoError<DeleteGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteGroupResponse, _>()
    }

    /// <p>Deletes permissions granted to a member (user or group).</p>
    async fn delete_mailbox_permissions(
        &self,
        input: DeleteMailboxPermissionsRequest,
    ) -> Result<DeleteMailboxPermissionsResponse, RusotoError<DeleteMailboxPermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteMailboxPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteMailboxPermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteMailboxPermissionsResponse, _>()
    }

    /// <p>Deletes an Amazon WorkMail organization and all underlying AWS resources managed by Amazon WorkMail as part of the organization. You can choose whether to delete the associated directory. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/remove_organization.html">Removing an organization</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p>
    async fn delete_organization(
        &self,
        input: DeleteOrganizationRequest,
    ) -> Result<DeleteOrganizationResponse, RusotoError<DeleteOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteOrganization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteOrganizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteOrganizationResponse, _>()
    }

    /// <p>Deletes the specified resource. </p>
    async fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> Result<DeleteResourceResponse, RusotoError<DeleteResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteResourceResponse, _>()
    }

    /// <p>Deletes the specified retention policy from the specified organization.</p>
    async fn delete_retention_policy(
        &self,
        input: DeleteRetentionPolicyRequest,
    ) -> Result<DeleteRetentionPolicyResponse, RusotoError<DeleteRetentionPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteRetentionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRetentionPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteRetentionPolicyResponse, _>()
    }

    /// <p>Deletes a user from Amazon WorkMail and all subsequent systems. Before you can delete a user, the user state must be <code>DISABLED</code>. Use the <a>DescribeUser</a> action to confirm the user state.</p> <p>Deleting a user is permanent and cannot be undone. WorkMail archives user mailboxes for 30 days before they are permanently removed.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, RusotoError<DeleteUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeleteUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteUserResponse, _>()
    }

    /// <p>Mark a user, group, or resource as no longer used in Amazon WorkMail. This action disassociates the mailbox and schedules it for clean-up. WorkMail keeps mailboxes for 30 days before they are permanently removed. The functionality in the console is <i>Disable</i>.</p>
    async fn deregister_from_work_mail(
        &self,
        input: DeregisterFromWorkMailRequest,
    ) -> Result<DeregisterFromWorkMailResponse, RusotoError<DeregisterFromWorkMailError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DeregisterFromWorkMail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterFromWorkMailError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeregisterFromWorkMailResponse, _>()
    }

    /// <p>Returns the data available for the group.</p>
    async fn describe_group(
        &self,
        input: DescribeGroupRequest,
    ) -> Result<DescribeGroupResponse, RusotoError<DescribeGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DescribeGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeGroupResponse, _>()
    }

    /// <p>Describes the current status of a mailbox export job.</p>
    async fn describe_mailbox_export_job(
        &self,
        input: DescribeMailboxExportJobRequest,
    ) -> Result<DescribeMailboxExportJobResponse, RusotoError<DescribeMailboxExportJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DescribeMailboxExportJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeMailboxExportJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeMailboxExportJobResponse, _>()
    }

    /// <p>Provides more information regarding a given organization based on its identifier.</p>
    async fn describe_organization(
        &self,
        input: DescribeOrganizationRequest,
    ) -> Result<DescribeOrganizationResponse, RusotoError<DescribeOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DescribeOrganization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeOrganizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeOrganizationResponse, _>()
    }

    /// <p>Returns the data available for the resource.</p>
    async fn describe_resource(
        &self,
        input: DescribeResourceRequest,
    ) -> Result<DescribeResourceResponse, RusotoError<DescribeResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DescribeResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeResourceResponse, _>()
    }

    /// <p>Provides information regarding the user.</p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.DescribeUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeUserError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeUserResponse, _>()
    }

    /// <p>Removes a member from the resource's set of delegates.</p>
    async fn disassociate_delegate_from_resource(
        &self,
        input: DisassociateDelegateFromResourceRequest,
    ) -> Result<
        DisassociateDelegateFromResourceResponse,
        RusotoError<DisassociateDelegateFromResourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "WorkMailService.DisassociateDelegateFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DisassociateDelegateFromResourceError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateDelegateFromResourceResponse, _>()
    }

    /// <p>Removes a member from a group.</p>
    async fn disassociate_member_from_group(
        &self,
        input: DisassociateMemberFromGroupRequest,
    ) -> Result<DisassociateMemberFromGroupResponse, RusotoError<DisassociateMemberFromGroupError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "WorkMailService.DisassociateMemberFromGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateMemberFromGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateMemberFromGroupResponse, _>()
    }

    /// <p>Gets the effects of an organization's access control rules as they apply to a specified IPv4 address, access protocol action, or user ID. </p>
    async fn get_access_control_effect(
        &self,
        input: GetAccessControlEffectRequest,
    ) -> Result<GetAccessControlEffectResponse, RusotoError<GetAccessControlEffectError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.GetAccessControlEffect");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAccessControlEffectError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAccessControlEffectResponse, _>()
    }

    /// <p>Gets the default retention policy details for the specified organization.</p>
    async fn get_default_retention_policy(
        &self,
        input: GetDefaultRetentionPolicyRequest,
    ) -> Result<GetDefaultRetentionPolicyResponse, RusotoError<GetDefaultRetentionPolicyError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.GetDefaultRetentionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDefaultRetentionPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetDefaultRetentionPolicyResponse, _>()
    }

    /// <p>Requests a user's mailbox details for a specified organization and user.</p>
    async fn get_mailbox_details(
        &self,
        input: GetMailboxDetailsRequest,
    ) -> Result<GetMailboxDetailsResponse, RusotoError<GetMailboxDetailsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.GetMailboxDetails");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetMailboxDetailsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetMailboxDetailsResponse, _>()
    }

    /// <p>Lists the access control rules for the specified organization.</p>
    async fn list_access_control_rules(
        &self,
        input: ListAccessControlRulesRequest,
    ) -> Result<ListAccessControlRulesResponse, RusotoError<ListAccessControlRulesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListAccessControlRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAccessControlRulesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListAccessControlRulesResponse, _>()
    }

    /// <p>Creates a paginated call to list the aliases associated with a given entity.</p>
    async fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, RusotoError<ListAliasesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListAliases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAliasesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAliasesResponse, _>()
    }

    /// <p>Returns an overview of the members of a group. Users and groups can be members of a group.</p>
    async fn list_group_members(
        &self,
        input: ListGroupMembersRequest,
    ) -> Result<ListGroupMembersResponse, RusotoError<ListGroupMembersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListGroupMembers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListGroupMembersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGroupMembersResponse, _>()
    }

    /// <p>Returns summaries of the organization's groups.</p>
    async fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, RusotoError<ListGroupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGroupsResponse, _>()
    }

    /// <p>Lists the mailbox export jobs started for the specified organization within the last seven days.</p>
    async fn list_mailbox_export_jobs(
        &self,
        input: ListMailboxExportJobsRequest,
    ) -> Result<ListMailboxExportJobsResponse, RusotoError<ListMailboxExportJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListMailboxExportJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListMailboxExportJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListMailboxExportJobsResponse, _>()
    }

    /// <p>Lists the mailbox permissions associated with a user, group, or resource mailbox.</p>
    async fn list_mailbox_permissions(
        &self,
        input: ListMailboxPermissionsRequest,
    ) -> Result<ListMailboxPermissionsResponse, RusotoError<ListMailboxPermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListMailboxPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListMailboxPermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListMailboxPermissionsResponse, _>()
    }

    /// <p>Returns summaries of the customer's organizations.</p>
    async fn list_organizations(
        &self,
        input: ListOrganizationsRequest,
    ) -> Result<ListOrganizationsResponse, RusotoError<ListOrganizationsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListOrganizations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListOrganizationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListOrganizationsResponse, _>()
    }

    /// <p>Lists the delegates associated with a resource. Users and groups can be resource delegates and answer requests on behalf of the resource.</p>
    async fn list_resource_delegates(
        &self,
        input: ListResourceDelegatesRequest,
    ) -> Result<ListResourceDelegatesResponse, RusotoError<ListResourceDelegatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListResourceDelegates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListResourceDelegatesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListResourceDelegatesResponse, _>()
    }

    /// <p>Returns summaries of the organization's resources.</p>
    async fn list_resources(
        &self,
        input: ListResourcesRequest,
    ) -> Result<ListResourcesResponse, RusotoError<ListResourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListResources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListResourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListResourcesResponse, _>()
    }

    /// <p>Lists the tags applied to an Amazon WorkMail organization resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Returns summaries of the organization's users.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ListUsers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListUsersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListUsersResponse, _>()
    }

    /// <p>Adds a new access control rule for the specified organization. The rule allows or denies access to the organization for the specified IPv4 addresses, access protocol actions, and user IDs. Adding a new rule with the same name as an existing rule replaces the older rule.</p>
    async fn put_access_control_rule(
        &self,
        input: PutAccessControlRuleRequest,
    ) -> Result<PutAccessControlRuleResponse, RusotoError<PutAccessControlRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.PutAccessControlRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAccessControlRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutAccessControlRuleResponse, _>()
    }

    /// <p>Sets permissions for a user, group, or resource. This replaces any pre-existing permissions.</p>
    async fn put_mailbox_permissions(
        &self,
        input: PutMailboxPermissionsRequest,
    ) -> Result<PutMailboxPermissionsResponse, RusotoError<PutMailboxPermissionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.PutMailboxPermissions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutMailboxPermissionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutMailboxPermissionsResponse, _>()
    }

    /// <p>Puts a retention policy to the specified organization.</p>
    async fn put_retention_policy(
        &self,
        input: PutRetentionPolicyRequest,
    ) -> Result<PutRetentionPolicyResponse, RusotoError<PutRetentionPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.PutRetentionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutRetentionPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutRetentionPolicyResponse, _>()
    }

    /// <p>Registers an existing and disabled user, group, or resource for Amazon WorkMail use by associating a mailbox and calendaring capabilities. It performs no change if the user, group, or resource is enabled and fails if the user, group, or resource is deleted. This operation results in the accumulation of costs. For more information, see <a href="https://aws.amazon.com/workmail/pricing">Pricing</a>. The equivalent console functionality for this operation is <i>Enable</i>. </p> <p>Users can either be created by calling the <a>CreateUser</a> API operation or they can be synchronized from your directory. For more information, see <a>DeregisterFromWorkMail</a>.</p>
    async fn register_to_work_mail(
        &self,
        input: RegisterToWorkMailRequest,
    ) -> Result<RegisterToWorkMailResponse, RusotoError<RegisterToWorkMailError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.RegisterToWorkMail");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterToWorkMailError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RegisterToWorkMailResponse, _>()
    }

    /// <p>Allows the administrator to reset the password for a user.</p>
    async fn reset_password(
        &self,
        input: ResetPasswordRequest,
    ) -> Result<ResetPasswordResponse, RusotoError<ResetPasswordError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.ResetPassword");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ResetPasswordError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ResetPasswordResponse, _>()
    }

    /// <p>Starts a mailbox export job to export MIME-format email messages and calendar items from the specified mailbox to the specified Amazon Simple Storage Service (Amazon S3) bucket. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/mail-export.html">Exporting mailbox content</a> in the <i>Amazon WorkMail Administrator Guide</i>.</p>
    async fn start_mailbox_export_job(
        &self,
        input: StartMailboxExportJobRequest,
    ) -> Result<StartMailboxExportJobResponse, RusotoError<StartMailboxExportJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.StartMailboxExportJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartMailboxExportJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartMailboxExportJobResponse, _>()
    }

    /// <p>Applies the specified tags to the specified Amazon WorkMail organization resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Untags the specified tags from the specified Amazon WorkMail organization resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates a user's current mailbox quota for a specified organization and user.</p>
    async fn update_mailbox_quota(
        &self,
        input: UpdateMailboxQuotaRequest,
    ) -> Result<UpdateMailboxQuotaResponse, RusotoError<UpdateMailboxQuotaError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.UpdateMailboxQuota");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateMailboxQuotaError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateMailboxQuotaResponse, _>()
    }

    /// <p>Updates the primary email for a user, group, or resource. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email), and the email provided in the input is promoted as the primary.</p>
    async fn update_primary_email_address(
        &self,
        input: UpdatePrimaryEmailAddressRequest,
    ) -> Result<UpdatePrimaryEmailAddressResponse, RusotoError<UpdatePrimaryEmailAddressError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.UpdatePrimaryEmailAddress");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdatePrimaryEmailAddressError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdatePrimaryEmailAddressResponse, _>()
    }

    /// <p>Updates data for the resource. To have the latest information, it must be preceded by a <a>DescribeResource</a> call. The dataset in the request should be the one expected when performing another <code>DescribeResource</code> call.</p>
    async fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> Result<UpdateResourceResponse, RusotoError<UpdateResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "WorkMailService.UpdateResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateResourceResponse, _>()
    }
}
