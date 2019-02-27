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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>AcceptInvitation request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcceptInvitationRequest {
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>This value is used to validate the master account to the member account.</p>
    #[serde(rename = "InvitationId")]
    pub invitation_id: String,
    /// <p>The account ID of the master GuardDuty account whose invitation you&#39;re accepting.</p>
    #[serde(rename = "MasterId")]
    pub master_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AcceptInvitationResponse {}

/// <p>The IAM access key details (IAM user information) of a user that engaged in the activity that prompted GuardDuty to generate a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AccessKeyDetails {
    /// <p>Access key ID of the user.</p>
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// <p>The principal ID of the user.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The name of the user.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// <p>The type of the user.</p>
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

/// <p>An object containing the member&#39;s accountId and email address.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AccountDetail {
    /// <p>Member account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>Member account&#39;s email address.</p>
    #[serde(rename = "Email")]
    pub email: String,
}

/// <p>Information about the activity described in a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Action {
    /// <p>GuardDuty Finding activity type.</p>
    #[serde(rename = "ActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>Information about the AWS<em>API</em>CALL action described in this finding.</p>
    #[serde(rename = "AwsApiCallAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_call_action: Option<AwsApiCallAction>,
    /// <p>Information about the DNS_REQUEST action described in this finding.</p>
    #[serde(rename = "DnsRequestAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_request_action: Option<DnsRequestAction>,
    /// <p>Information about the NETWORK_CONNECTION action described in this finding.</p>
    #[serde(rename = "NetworkConnectionAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connection_action: Option<NetworkConnectionAction>,
    /// <p>Information about the PORT_PROBE action described in this finding.</p>
    #[serde(rename = "PortProbeAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_action: Option<PortProbeAction>,
}

/// <p>ArchiveFindings request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ArchiveFindingsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to archive.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>IDs of the findings that you want to archive.</p>
    #[serde(rename = "FindingIds")]
    pub finding_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ArchiveFindingsResponse {}

/// <p>Information about the AWS<em>API</em>CALL action described in this finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AwsApiCallAction {
    /// <p>AWS API name.</p>
    #[serde(rename = "Api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    /// <p>AWS API caller type.</p>
    #[serde(rename = "CallerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_type: Option<String>,
    /// <p>Domain information for the AWS API call.</p>
    #[serde(rename = "DomainDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_details: Option<DomainDetails>,
    /// <p>Remote IP information of the connection.</p>
    #[serde(rename = "RemoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
    /// <p>AWS service name whose API was invoked.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>City information of the remote IP address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct City {
    /// <p>City name of the remote IP address.</p>
    #[serde(rename = "CityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
}

/// <p>Finding attribute (for example, accountId) for which conditions and values must be specified when querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// <p>Represents the equal condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Eq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    /// <p>Represents the greater than condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<i64>,
    /// <p>Represents the greater than equal condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<i64>,
    /// <p>Represents the less than condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Lt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<i64>,
    /// <p>Represents the less than equal condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<i64>,
    /// <p>Represents the not equal condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Neq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
}

/// <p>Country information of the remote IP address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Country {
    /// <p>Country code of the remote IP address.</p>
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// <p>Country name of the remote IP address.</p>
    #[serde(rename = "CountryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
}

/// <p>CreateDetector request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDetectorRequest {
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>A boolean value that specifies whether the detector is to be enabled.</p>
    #[serde(rename = "Enable")]
    pub enable: bool,
    /// <p>A enum value that specifies how frequently customer got Finding updates published.</p>
    #[serde(rename = "FindingPublishingFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDetectorResponse {
    /// <p>The unique ID of the created detector.</p>
    #[serde(rename = "DetectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
}

/// <p>CreateFilterRequest request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFilterRequest {
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The description of the filter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique ID of the detector that you want to update.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    #[serde(rename = "FindingCriteria")]
    pub finding_criteria: FindingCriteria,
    /// <p>The name of the filter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateFilterResponse {
    /// <p>The name of the successfully created filter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>CreateIPSet request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIPSetRequest {
    /// <p>A boolean value that indicates whether GuardDuty is to start using the uploaded IPSet.</p>
    #[serde(rename = "Activate")]
    pub activate: bool,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The unique ID of the detector that you want to update.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The format of the file that contains the IPSet.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>The URI of the file that contains the IPSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key)</p>
    #[serde(rename = "Location")]
    pub location: String,
    /// <p>The user friendly name to identify the IPSet. This name is displayed in all findings that are triggered by activity that involves IP addresses included in this IPSet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateIPSetResponse {
    #[serde(rename = "IpSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set_id: Option<String>,
}

/// <p>CreateMembers request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateMembersRequest {
    /// <p>A list of account ID and email address pairs of the accounts that you want to associate with the master GuardDuty account.</p>
    #[serde(rename = "AccountDetails")]
    pub account_details: Vec<AccountDetail>,
    /// <p>The unique ID of the detector of the GuardDuty account with which you want to associate member accounts.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

/// <p>CreateSampleFindings request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSampleFindingsRequest {
    /// <p>The ID of the detector to create sample findings for.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>Types of sample findings that you want to generate.</p>
    #[serde(rename = "FindingTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSampleFindingsResponse {}

/// <p>CreateThreatIntelSet request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateThreatIntelSetRequest {
    /// <p>A boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    #[serde(rename = "Activate")]
    pub activate: bool,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The unique ID of the detector that you want to update.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>The URI of the file that contains the ThreatIntelSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key).</p>
    #[serde(rename = "Location")]
    pub location: String,
    /// <p>A user-friendly ThreatIntelSet name that is displayed in all finding generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateThreatIntelSetResponse {
    #[serde(rename = "ThreatIntelSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_set_id: Option<String>,
}

/// <p>DeclineInvitations request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeclineInvitationsRequest {
    /// <p>A list of account IDs of the AWS accounts that sent invitations to the current member account that you want to decline invitations from.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeclineInvitationsResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDetectorRequest {
    /// <p>The unique ID that specifies the detector that you want to delete.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDetectorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFilterRequest {
    /// <p>The unique ID that specifies the detector where you want to delete a filter.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The name of the filter.</p>
    #[serde(rename = "FilterName")]
    pub filter_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteFilterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIPSetRequest {
    /// <p>The detectorID that specifies the GuardDuty service whose IPSet you want to delete.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID that specifies the IPSet that you want to delete.</p>
    #[serde(rename = "IpSetId")]
    pub ip_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteIPSetResponse {}

/// <p>DeleteInvitations request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInvitationsRequest {
    /// <p>A list of account IDs of the AWS accounts that sent invitations to the current member account that you want to delete invitations from.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteInvitationsResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

/// <p>DeleteMembers request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts that you want to delete.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whose members you want to delete.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteThreatIntelSetRequest {
    /// <p>The detectorID that specifies the GuardDuty service whose ThreatIntelSet you want to delete.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to delete.</p>
    #[serde(rename = "ThreatIntelSetId")]
    pub threat_intel_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteThreatIntelSetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateFromMasterAccountRequest {
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateFromMasterAccountResponse {}

/// <p>DisassociateMembers request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts that you want to disassociate from master.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whose members you want to disassociate from master.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

/// <p>Information about the DNS_REQUEST action described in this finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DnsRequestAction {
    /// <p>Domain information for the DNS request.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// <p>Domain information for the AWS API call.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainDetails {}

/// <p>Error response object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ErrorResponse {
    /// <p>The error message.</p>
    pub message: Option<String>,
    /// <p>The error type.</p>
    pub type_: Option<String>,
}

/// <p>Representation of a abnormal or suspicious activity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Finding {
    /// <p>AWS account ID where the activity occurred that prompted GuardDuty to generate a finding.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The ARN of a finding described by the action.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The confidence level of a finding.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    /// <p>The time stamp at which a finding was generated.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    /// <p>The description of a finding.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier that corresponds to a finding described by the action.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The AWS resource partition.</p>
    #[serde(rename = "Partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    /// <p>The AWS region where the activity occurred that prompted GuardDuty to generate a finding.</p>
    #[serde(rename = "Region")]
    pub region: String,
    /// <p>The AWS resource associated with the activity that prompted GuardDuty to generate a finding.</p>
    #[serde(rename = "Resource")]
    pub resource: Resource,
    /// <p>Findings&#39; schema version.</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// <p>Additional information assigned to the generated finding by GuardDuty.</p>
    #[serde(rename = "Service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    /// <p>The severity of a finding.</p>
    #[serde(rename = "Severity")]
    pub severity: f64,
    /// <p>The title of a finding.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The type of a finding described by the action.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The time stamp at which a finding was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
}

/// <p>Represents the criteria used for querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindingCriteria {
    /// <p>Represents a map of finding properties that match specified conditions and values when querying findings.</p>
    #[serde(rename = "Criterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion: Option<::std::collections::HashMap<String, Condition>>,
}

/// <p>Finding statistics object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FindingStatistics {
    /// <p>Represents a map of severity to count statistic for a set of findings</p>
    #[serde(rename = "CountBySeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_by_severity: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>Location information of the remote IP address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GeoLocation {
    /// <p>Latitude information of remote IP address.</p>
    #[serde(rename = "Lat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    /// <p>Longitude information of remote IP address.</p>
    #[serde(rename = "Lon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDetectorRequest {
    /// <p>The unique ID of the detector that you want to retrieve.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDetectorResponse {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "FindingPublishingFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFilterRequest {
    /// <p>The detector ID that specifies the GuardDuty service where you want to list the details of the specified filter.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The name of the filter whose details you want to get.</p>
    #[serde(rename = "FilterName")]
    pub filter_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFilterResponse {
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The description of the filter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    #[serde(rename = "FindingCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    /// <p>The name of the filter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// <p>GetFindings request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFindingsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to retrieve.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>IDs of the findings that you want to retrieve.</p>
    #[serde(rename = "FindingIds")]
    pub finding_ids: Vec<String>,
    /// <p>Represents the criteria used for sorting findings.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFindingsResponse {
    #[serde(rename = "Findings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<Finding>>,
}

/// <p>GetFindingsStatistics request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFindingsStatisticsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings&#39; statistics you want to retrieve.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>Represents the criteria used for querying findings.</p>
    #[serde(rename = "FindingCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    /// <p>Types of finding statistics to retrieve.</p>
    #[serde(rename = "FindingStatisticTypes")]
    pub finding_statistic_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFindingsStatisticsResponse {
    /// <p>Finding statistics object.</p>
    #[serde(rename = "FindingStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_statistics: Option<FindingStatistics>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIPSetRequest {
    /// <p>The detectorID that specifies the GuardDuty service whose IPSet you want to retrieve.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID that specifies the IPSet that you want to describe.</p>
    #[serde(rename = "IpSetId")]
    pub ip_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIPSetResponse {
    /// <p>The format of the file that contains the IPSet.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The URI of the file that contains the IPSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key)</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The user friendly name to identify the IPSet. This name is displayed in all findings that are triggered by activity that involves IP addresses included in this IPSet.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of ipSet file uploaded.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInvitationsCountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInvitationsCountResponse {
    /// <p>The number of received invitations.</p>
    #[serde(rename = "InvitationsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMasterAccountRequest {
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetMasterAccountResponse {
    #[serde(rename = "Master")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Master>,
}

/// <p>GetMembers request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts that you want to describe.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whose members you want to retrieve.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetMembersResponse {
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetThreatIntelSetRequest {
    /// <p>The detectorID that specifies the GuardDuty service whose ThreatIntelSet you want to describe.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to describe.</p>
    #[serde(rename = "ThreatIntelSetId")]
    pub threat_intel_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetThreatIntelSetResponse {
    /// <p>The format of the threatIntelSet.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The URI of the file that contains the ThreatIntelSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key).</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>A user-friendly ThreatIntelSet name that is displayed in all finding generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of threatIntelSet file uploaded.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The profile information of the EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IamInstanceProfile {
    /// <p>AWS EC2 instance profile ARN.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>AWS EC2 instance profile ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The information about the EC2 instance associated with the activity that prompted GuardDuty to generate a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceDetails {
    /// <p>The availability zone of the EC2 instance.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "IamInstanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<IamInstanceProfile>,
    /// <p>The image description of the EC2 instance.</p>
    #[serde(rename = "ImageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_description: Option<String>,
    /// <p>The image ID of the EC2 instance.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The ID of the EC2 instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The state of the EC2 instance.</p>
    #[serde(rename = "InstanceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
    /// <p>The type of the EC2 instance.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The launch time of the EC2 instance.</p>
    #[serde(rename = "LaunchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<String>,
    /// <p>The network interface information of the EC2 instance.</p>
    #[serde(rename = "NetworkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>The platform of the EC2 instance.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The product code of the EC2 instance.</p>
    #[serde(rename = "ProductCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<ProductCode>>,
    /// <p>The tags of the EC2 instance.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Invitation from an AWS account to become the current account&#39;s master.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Invitation {
    /// <p>Inviter account ID</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>This value is used to validate the inviter account to the member account.</p>
    #[serde(rename = "InvitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    /// <p>Timestamp at which the invitation was sent</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    /// <p>The status of the relationship between the inviter and invitee accounts.</p>
    #[serde(rename = "RelationshipStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

/// <p>InviteMembers request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InviteMembersRequest {
    /// <p>A list of account IDs of the accounts that you want to invite to GuardDuty as members.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account with which you want to invite members.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>A boolean value that specifies whether you want to disable email notification to the accounts that you’re inviting to GuardDuty as members.</p>
    #[serde(rename = "DisableEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_email_notification: Option<bool>,
    /// <p>The invitation message that you want to send to the accounts that you’re inviting to GuardDuty as members.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InviteMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDetectorsRequest {
    /// <p>You can use this parameter to indicate the maximum number of detectors that you want in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the ListDetectors action. For subsequent calls to the action fill nextToken in the request with the value of nextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDetectorsResponse {
    #[serde(rename = "DetectorIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFiltersRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service where you want to list filters.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>Indicates the maximum number of items that you want in the response. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Paginates results. Set the value of this parameter to NULL on your first call to the ListFilters operation.For subsequent calls to the operation, fill nextToken in the request with the value of nextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFiltersResponse {
    #[serde(rename = "FilterNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>ListFindings request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFindingsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to list.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>Represents the criteria used for querying findings.</p>
    #[serde(rename = "FindingCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the ListFindings action. For subsequent calls to the action fill nextToken in the request with the value of nextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Represents the criteria used for sorting findings.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFindingsResponse {
    #[serde(rename = "FindingIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIPSetsRequest {
    /// <p>The unique ID of the detector that you want to retrieve.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 7. The maximum value is 7.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the ListIPSet action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIPSetsResponse {
    #[serde(rename = "IpSetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListInvitationsRequest {
    /// <p>You can use this parameter to indicate the maximum number of invitations you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the ListInvitations action. Subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListInvitationsResponse {
    #[serde(rename = "Invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListMembersRequest {
    /// <p>The unique ID of the detector of the GuardDuty account whose members you want to list.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 1. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the ListMembers action. Subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies what member accounts the response is to include based on their relationship status with the master account. The default value is TRUE. If onlyAssociated is set to TRUE, the response will include member accounts whose relationship status with the master is set to Enabled, Disabled. If onlyAssociated is set to FALSE, the response will include all existing member accounts.</p>
    #[serde(rename = "OnlyAssociated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListMembersResponse {
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListThreatIntelSetsRequest {
    /// <p>The detectorID that specifies the GuardDuty service whose ThreatIntelSets you want to list.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 7. The maximum value is 7.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token to start retrieving threat intel sets from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListThreatIntelSetsResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ThreatIntelSetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_set_ids: Option<Vec<String>>,
}

/// <p>Local port information of the connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LocalPortDetails {
    /// <p>Port number of the local connection.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Port name of the local connection.</p>
    #[serde(rename = "PortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

/// <p>Contains details about the master account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Master {
    /// <p>Master account ID</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>This value is used to validate the master account to the member account.</p>
    #[serde(rename = "InvitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    /// <p>Timestamp at which the invitation was sent</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    /// <p>The status of the relationship between the master and member accounts.</p>
    #[serde(rename = "RelationshipStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

/// <p>Contains details about the member account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Member {
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "DetectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>Member account&#39;s email address.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>Timestamp at which the invitation was sent</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "MasterId")]
    pub master_id: String,
    /// <p>The status of the relationship between the member and the master.</p>
    #[serde(rename = "RelationshipStatus")]
    pub relationship_status: String,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
}

/// <p>Information about the NETWORK_CONNECTION action described in this finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NetworkConnectionAction {
    /// <p>Network connection blocked information.</p>
    #[serde(rename = "Blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// <p>Network connection direction.</p>
    #[serde(rename = "ConnectionDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_direction: Option<String>,
    /// <p>Local port information of the connection.</p>
    #[serde(rename = "LocalPortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<LocalPortDetails>,
    /// <p>Network connection protocol.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>Remote IP information of the connection.</p>
    #[serde(rename = "RemoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
    /// <p>Remote port information of the connection.</p>
    #[serde(rename = "RemotePortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_port_details: Option<RemotePortDetails>,
}

/// <p>The network interface information of the EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NetworkInterface {
    /// <p>A list of EC2 instance IPv6 address information.</p>
    #[serde(rename = "Ipv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_addresses: Option<Vec<String>>,
    /// <p>The ID of the network interface</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>Private DNS name of the EC2 instance.</p>
    #[serde(rename = "PrivateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>Private IP address of the EC2 instance.</p>
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>Other private IP address information of the EC2 instance.</p>
    #[serde(rename = "PrivateIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressDetails>>,
    /// <p>Public DNS name of the EC2 instance.</p>
    #[serde(rename = "PublicDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    /// <p>Public IP address of the EC2 instance.</p>
    #[serde(rename = "PublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// <p>Security groups associated with the EC2 instance.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroup>>,
    /// <p>The subnet ID of the EC2 instance.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The VPC ID of the EC2 instance.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>ISP Organization information of the remote IP address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Organization {
    /// <p>Autonomous system number of the internet provider of the remote IP address.</p>
    #[serde(rename = "Asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,
    /// <p>Organization that registered this ASN.</p>
    #[serde(rename = "AsnOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_org: Option<String>,
    /// <p>ISP information for the internet provider.</p>
    #[serde(rename = "Isp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    /// <p>Name of the internet provider.</p>
    #[serde(rename = "Org")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
}

/// <p>Information about the PORT_PROBE action described in this finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PortProbeAction {
    /// <p>Port probe blocked information.</p>
    #[serde(rename = "Blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// <p>A list of port probe details objects.</p>
    #[serde(rename = "PortProbeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_details: Option<Vec<PortProbeDetail>>,
}

/// <p>Details about the port probe finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PortProbeDetail {
    /// <p>Local port information of the connection.</p>
    #[serde(rename = "LocalPortDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<LocalPortDetails>,
    /// <p>Remote IP information of the connection.</p>
    #[serde(rename = "RemoteIpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<RemoteIpDetails>,
}

/// <p>Other private IP address information of the EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PrivateIpAddressDetails {
    /// <p>Private DNS name of the EC2 instance.</p>
    #[serde(rename = "PrivateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>Private IP address of the EC2 instance.</p>
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}

/// <p>The product code of the EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProductCode {
    /// <p>Product code information.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>Product code type.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

/// <p>Remote IP information of the connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoteIpDetails {
    /// <p>City information of the remote IP address.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<City>,
    /// <p>Country code of the remote IP address.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    /// <p>Location information of the remote IP address.</p>
    #[serde(rename = "GeoLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,
    /// <p>IPV4 remote address of the connection.</p>
    #[serde(rename = "IpAddressV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
    /// <p>ISP Organization information of the remote IP address.</p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

/// <p>Remote port information of the connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemotePortDetails {
    /// <p>Port number of the remote connection.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Port name of the remote connection.</p>
    #[serde(rename = "PortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

/// <p>The AWS resource associated with the activity that prompted GuardDuty to generate a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Resource {
    #[serde(rename = "AccessKeyDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_details: Option<AccessKeyDetails>,
    #[serde(rename = "InstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<InstanceDetails>,
    /// <p>The type of the AWS resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Security groups associated with the EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SecurityGroup {
    /// <p>EC2 instance&#39;s security group ID.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>EC2 instance&#39;s security group name.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>Additional information assigned to the generated finding by GuardDuty.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Service {
    /// <p>Information about the activity described in a finding.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// <p>Indicates whether this finding is archived.</p>
    #[serde(rename = "Archived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// <p>Total count of the occurrences of this finding type.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Detector ID for the GuardDuty service.</p>
    #[serde(rename = "DetectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>First seen timestamp of the activity that prompted GuardDuty to generate this finding.</p>
    #[serde(rename = "EventFirstSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_first_seen: Option<String>,
    /// <p>Last seen timestamp of the activity that prompted GuardDuty to generate this finding.</p>
    #[serde(rename = "EventLastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_last_seen: Option<String>,
    /// <p>Resource role information for this finding.</p>
    #[serde(rename = "ResourceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_role: Option<String>,
    /// <p>The name of the AWS service (GuardDuty) that generated a finding.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>Feedback left about the finding.</p>
    #[serde(rename = "UserFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_feedback: Option<String>,
}

/// <p>Represents the criteria used for sorting findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SortCriteria {
    /// <p>Represents the finding attribute (for example, accountId) by which to sort findings.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>Order by which the sorted findings are to be displayed.</p>
    #[serde(rename = "OrderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

/// <p>StartMonitoringMembers request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartMonitoringMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts whose findings you want the master account to monitor.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whom you want to re-enable to monitor members&#39; findings.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartMonitoringMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

/// <p>StopMonitoringMembers request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopMonitoringMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts whose findings you want the master account to stop monitoring.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account that you want to stop from monitor members&#39; findings.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopMonitoringMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

/// <p>A tag of the EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Tag {
    /// <p>EC2 instance tag key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>EC2 instance tag value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>UnarchiveFindings request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnarchiveFindingsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to unarchive.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>IDs of the findings that you want to unarchive.</p>
    #[serde(rename = "FindingIds")]
    pub finding_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnarchiveFindingsResponse {}

/// <p>An object containing the unprocessed account and a result string explaining why it was unprocessed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnprocessedAccount {
    /// <p>AWS Account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>A reason why the account hasn&#39;t been processed.</p>
    #[serde(rename = "Result")]
    pub result: String,
}

/// <p>UpdateDetector request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDetectorRequest {
    /// <p>The unique ID of the detector that you want to update.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>Updated boolean value for the detector that specifies whether the detector is enabled.</p>
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// <p>A enum value that specifies how frequently customer got Finding updates published.</p>
    #[serde(rename = "FindingPublishingFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDetectorResponse {}

/// <p>UpdateFilterRequest request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFilterRequest {
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The description of the filter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique ID of the detector that specifies the GuardDuty service where you want to update a filter.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The name of the filter.</p>
    #[serde(rename = "FilterName")]
    pub filter_name: String,
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    #[serde(rename = "FindingCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateFilterResponse {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>UpdateFindingsFeedback request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFindingsFeedbackRequest {
    /// <p>Additional feedback about the GuardDuty findings.</p>
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to mark as useful or not useful.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>Valid values: USEFUL | NOT_USEFUL</p>
    #[serde(rename = "Feedback")]
    pub feedback: String,
    /// <p>IDs of the findings that you want to mark as useful or not useful.</p>
    #[serde(rename = "FindingIds")]
    pub finding_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateFindingsFeedbackResponse {}

/// <p>UpdateIPSet request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIPSetRequest {
    /// <p>The updated boolean value that specifies whether the IPSet is active or not.</p>
    #[serde(rename = "Activate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    /// <p>The detectorID that specifies the GuardDuty service whose IPSet you want to update.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID that specifies the IPSet that you want to update.</p>
    #[serde(rename = "IpSetId")]
    pub ip_set_id: String,
    /// <p>The updated URI of the file that contains the IPSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key).</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The unique ID that specifies the IPSet that you want to update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateIPSetResponse {}

/// <p>UpdateThreatIntelSet request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateThreatIntelSetRequest {
    /// <p>The updated boolean value that specifies whether the ThreateIntelSet is active or not.</p>
    #[serde(rename = "Activate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    /// <p>The detectorID that specifies the GuardDuty service whose ThreatIntelSet you want to update.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The updated URI of the file that contains the ThreateIntelSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key)</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    #[serde(rename = "ThreatIntelSetId")]
    pub threat_intel_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateThreatIntelSetResponse {}

/// Errors returned by AcceptInvitation
#[derive(Debug, PartialEq)]
pub enum AcceptInvitationError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AcceptInvitationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> AcceptInvitationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return AcceptInvitationError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return AcceptInvitationError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return AcceptInvitationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AcceptInvitationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AcceptInvitationError {
    fn from(err: serde_json::error::Error) -> AcceptInvitationError {
        AcceptInvitationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptInvitationError {
    fn from(err: CredentialsError) -> AcceptInvitationError {
        AcceptInvitationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptInvitationError {
    fn from(err: HttpDispatchError) -> AcceptInvitationError {
        AcceptInvitationError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcceptInvitationError {
    fn from(err: io::Error) -> AcceptInvitationError {
        AcceptInvitationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcceptInvitationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptInvitationError {
    fn description(&self) -> &str {
        match *self {
            AcceptInvitationError::BadRequest(ref cause) => cause,
            AcceptInvitationError::InternalServerError(ref cause) => cause,
            AcceptInvitationError::Validation(ref cause) => cause,
            AcceptInvitationError::Credentials(ref err) => err.description(),
            AcceptInvitationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AcceptInvitationError::ParseError(ref cause) => cause,
            AcceptInvitationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ArchiveFindings
#[derive(Debug, PartialEq)]
pub enum ArchiveFindingsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ArchiveFindingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ArchiveFindingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ArchiveFindingsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ArchiveFindingsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return ArchiveFindingsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ArchiveFindingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ArchiveFindingsError {
    fn from(err: serde_json::error::Error) -> ArchiveFindingsError {
        ArchiveFindingsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ArchiveFindingsError {
    fn from(err: CredentialsError) -> ArchiveFindingsError {
        ArchiveFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ArchiveFindingsError {
    fn from(err: HttpDispatchError) -> ArchiveFindingsError {
        ArchiveFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ArchiveFindingsError {
    fn from(err: io::Error) -> ArchiveFindingsError {
        ArchiveFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ArchiveFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ArchiveFindingsError {
    fn description(&self) -> &str {
        match *self {
            ArchiveFindingsError::BadRequest(ref cause) => cause,
            ArchiveFindingsError::InternalServerError(ref cause) => cause,
            ArchiveFindingsError::Validation(ref cause) => cause,
            ArchiveFindingsError::Credentials(ref err) => err.description(),
            ArchiveFindingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ArchiveFindingsError::ParseError(ref cause) => cause,
            ArchiveFindingsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDetector
#[derive(Debug, PartialEq)]
pub enum CreateDetectorError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDetectorError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateDetectorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateDetectorError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateDetectorError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateDetectorError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateDetectorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDetectorError {
    fn from(err: serde_json::error::Error) -> CreateDetectorError {
        CreateDetectorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDetectorError {
    fn from(err: CredentialsError) -> CreateDetectorError {
        CreateDetectorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDetectorError {
    fn from(err: HttpDispatchError) -> CreateDetectorError {
        CreateDetectorError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDetectorError {
    fn from(err: io::Error) -> CreateDetectorError {
        CreateDetectorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDetectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDetectorError {
    fn description(&self) -> &str {
        match *self {
            CreateDetectorError::BadRequest(ref cause) => cause,
            CreateDetectorError::InternalServerError(ref cause) => cause,
            CreateDetectorError::Validation(ref cause) => cause,
            CreateDetectorError::Credentials(ref err) => err.description(),
            CreateDetectorError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDetectorError::ParseError(ref cause) => cause,
            CreateDetectorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateFilter
#[derive(Debug, PartialEq)]
pub enum CreateFilterError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateFilterError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateFilterError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateFilterError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateFilterError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateFilterError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateFilterError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateFilterError {
    fn from(err: serde_json::error::Error) -> CreateFilterError {
        CreateFilterError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFilterError {
    fn from(err: CredentialsError) -> CreateFilterError {
        CreateFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFilterError {
    fn from(err: HttpDispatchError) -> CreateFilterError {
        CreateFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFilterError {
    fn from(err: io::Error) -> CreateFilterError {
        CreateFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFilterError {
    fn description(&self) -> &str {
        match *self {
            CreateFilterError::BadRequest(ref cause) => cause,
            CreateFilterError::InternalServerError(ref cause) => cause,
            CreateFilterError::Validation(ref cause) => cause,
            CreateFilterError::Credentials(ref err) => err.description(),
            CreateFilterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFilterError::ParseError(ref cause) => cause,
            CreateFilterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateIPSet
#[derive(Debug, PartialEq)]
pub enum CreateIPSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateIPSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateIPSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateIPSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateIPSetError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateIPSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateIPSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateIPSetError {
    fn from(err: serde_json::error::Error) -> CreateIPSetError {
        CreateIPSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateIPSetError {
    fn from(err: CredentialsError) -> CreateIPSetError {
        CreateIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateIPSetError {
    fn from(err: HttpDispatchError) -> CreateIPSetError {
        CreateIPSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateIPSetError {
    fn from(err: io::Error) -> CreateIPSetError {
        CreateIPSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIPSetError {
    fn description(&self) -> &str {
        match *self {
            CreateIPSetError::BadRequest(ref cause) => cause,
            CreateIPSetError::InternalServerError(ref cause) => cause,
            CreateIPSetError::Validation(ref cause) => cause,
            CreateIPSetError::Credentials(ref err) => err.description(),
            CreateIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateIPSetError::ParseError(ref cause) => cause,
            CreateIPSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateMembers
#[derive(Debug, PartialEq)]
pub enum CreateMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateMembersError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateMembersError {
    fn from(err: serde_json::error::Error) -> CreateMembersError {
        CreateMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateMembersError {
    fn from(err: CredentialsError) -> CreateMembersError {
        CreateMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMembersError {
    fn from(err: HttpDispatchError) -> CreateMembersError {
        CreateMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMembersError {
    fn from(err: io::Error) -> CreateMembersError {
        CreateMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMembersError {
    fn description(&self) -> &str {
        match *self {
            CreateMembersError::BadRequest(ref cause) => cause,
            CreateMembersError::InternalServerError(ref cause) => cause,
            CreateMembersError::Validation(ref cause) => cause,
            CreateMembersError::Credentials(ref err) => err.description(),
            CreateMembersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateMembersError::ParseError(ref cause) => cause,
            CreateMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateSampleFindings
#[derive(Debug, PartialEq)]
pub enum CreateSampleFindingsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateSampleFindingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateSampleFindingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateSampleFindingsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateSampleFindingsError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateSampleFindingsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateSampleFindingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateSampleFindingsError {
    fn from(err: serde_json::error::Error) -> CreateSampleFindingsError {
        CreateSampleFindingsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSampleFindingsError {
    fn from(err: CredentialsError) -> CreateSampleFindingsError {
        CreateSampleFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSampleFindingsError {
    fn from(err: HttpDispatchError) -> CreateSampleFindingsError {
        CreateSampleFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSampleFindingsError {
    fn from(err: io::Error) -> CreateSampleFindingsError {
        CreateSampleFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSampleFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSampleFindingsError {
    fn description(&self) -> &str {
        match *self {
            CreateSampleFindingsError::BadRequest(ref cause) => cause,
            CreateSampleFindingsError::InternalServerError(ref cause) => cause,
            CreateSampleFindingsError::Validation(ref cause) => cause,
            CreateSampleFindingsError::Credentials(ref err) => err.description(),
            CreateSampleFindingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSampleFindingsError::ParseError(ref cause) => cause,
            CreateSampleFindingsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum CreateThreatIntelSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateThreatIntelSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateThreatIntelSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateThreatIntelSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateThreatIntelSetError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateThreatIntelSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateThreatIntelSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateThreatIntelSetError {
    fn from(err: serde_json::error::Error) -> CreateThreatIntelSetError {
        CreateThreatIntelSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateThreatIntelSetError {
    fn from(err: CredentialsError) -> CreateThreatIntelSetError {
        CreateThreatIntelSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateThreatIntelSetError {
    fn from(err: HttpDispatchError) -> CreateThreatIntelSetError {
        CreateThreatIntelSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateThreatIntelSetError {
    fn from(err: io::Error) -> CreateThreatIntelSetError {
        CreateThreatIntelSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateThreatIntelSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateThreatIntelSetError {
    fn description(&self) -> &str {
        match *self {
            CreateThreatIntelSetError::BadRequest(ref cause) => cause,
            CreateThreatIntelSetError::InternalServerError(ref cause) => cause,
            CreateThreatIntelSetError::Validation(ref cause) => cause,
            CreateThreatIntelSetError::Credentials(ref err) => err.description(),
            CreateThreatIntelSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateThreatIntelSetError::ParseError(ref cause) => cause,
            CreateThreatIntelSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeclineInvitations
#[derive(Debug, PartialEq)]
pub enum DeclineInvitationsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeclineInvitationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeclineInvitationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeclineInvitationsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeclineInvitationsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return DeclineInvitationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeclineInvitationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeclineInvitationsError {
    fn from(err: serde_json::error::Error) -> DeclineInvitationsError {
        DeclineInvitationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeclineInvitationsError {
    fn from(err: CredentialsError) -> DeclineInvitationsError {
        DeclineInvitationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeclineInvitationsError {
    fn from(err: HttpDispatchError) -> DeclineInvitationsError {
        DeclineInvitationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeclineInvitationsError {
    fn from(err: io::Error) -> DeclineInvitationsError {
        DeclineInvitationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeclineInvitationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeclineInvitationsError {
    fn description(&self) -> &str {
        match *self {
            DeclineInvitationsError::BadRequest(ref cause) => cause,
            DeclineInvitationsError::InternalServerError(ref cause) => cause,
            DeclineInvitationsError::Validation(ref cause) => cause,
            DeclineInvitationsError::Credentials(ref err) => err.description(),
            DeclineInvitationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeclineInvitationsError::ParseError(ref cause) => cause,
            DeclineInvitationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDetector
#[derive(Debug, PartialEq)]
pub enum DeleteDetectorError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDetectorError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDetectorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteDetectorError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteDetectorError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteDetectorError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDetectorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDetectorError {
    fn from(err: serde_json::error::Error) -> DeleteDetectorError {
        DeleteDetectorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDetectorError {
    fn from(err: CredentialsError) -> DeleteDetectorError {
        DeleteDetectorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDetectorError {
    fn from(err: HttpDispatchError) -> DeleteDetectorError {
        DeleteDetectorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDetectorError {
    fn from(err: io::Error) -> DeleteDetectorError {
        DeleteDetectorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDetectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDetectorError {
    fn description(&self) -> &str {
        match *self {
            DeleteDetectorError::BadRequest(ref cause) => cause,
            DeleteDetectorError::InternalServerError(ref cause) => cause,
            DeleteDetectorError::Validation(ref cause) => cause,
            DeleteDetectorError::Credentials(ref err) => err.description(),
            DeleteDetectorError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDetectorError::ParseError(ref cause) => cause,
            DeleteDetectorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteFilter
#[derive(Debug, PartialEq)]
pub enum DeleteFilterError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteFilterError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteFilterError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteFilterError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteFilterError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteFilterError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteFilterError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteFilterError {
    fn from(err: serde_json::error::Error) -> DeleteFilterError {
        DeleteFilterError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFilterError {
    fn from(err: CredentialsError) -> DeleteFilterError {
        DeleteFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFilterError {
    fn from(err: HttpDispatchError) -> DeleteFilterError {
        DeleteFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFilterError {
    fn from(err: io::Error) -> DeleteFilterError {
        DeleteFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFilterError {
    fn description(&self) -> &str {
        match *self {
            DeleteFilterError::BadRequest(ref cause) => cause,
            DeleteFilterError::InternalServerError(ref cause) => cause,
            DeleteFilterError::Validation(ref cause) => cause,
            DeleteFilterError::Credentials(ref err) => err.description(),
            DeleteFilterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFilterError::ParseError(ref cause) => cause,
            DeleteFilterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteIPSet
#[derive(Debug, PartialEq)]
pub enum DeleteIPSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteIPSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteIPSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteIPSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteIPSetError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteIPSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteIPSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteIPSetError {
    fn from(err: serde_json::error::Error) -> DeleteIPSetError {
        DeleteIPSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIPSetError {
    fn from(err: CredentialsError) -> DeleteIPSetError {
        DeleteIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIPSetError {
    fn from(err: HttpDispatchError) -> DeleteIPSetError {
        DeleteIPSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIPSetError {
    fn from(err: io::Error) -> DeleteIPSetError {
        DeleteIPSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIPSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteIPSetError::BadRequest(ref cause) => cause,
            DeleteIPSetError::InternalServerError(ref cause) => cause,
            DeleteIPSetError::Validation(ref cause) => cause,
            DeleteIPSetError::Credentials(ref err) => err.description(),
            DeleteIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteIPSetError::ParseError(ref cause) => cause,
            DeleteIPSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteInvitations
#[derive(Debug, PartialEq)]
pub enum DeleteInvitationsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteInvitationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteInvitationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteInvitationsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteInvitationsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteInvitationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteInvitationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteInvitationsError {
    fn from(err: serde_json::error::Error) -> DeleteInvitationsError {
        DeleteInvitationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteInvitationsError {
    fn from(err: CredentialsError) -> DeleteInvitationsError {
        DeleteInvitationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteInvitationsError {
    fn from(err: HttpDispatchError) -> DeleteInvitationsError {
        DeleteInvitationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteInvitationsError {
    fn from(err: io::Error) -> DeleteInvitationsError {
        DeleteInvitationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteInvitationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInvitationsError {
    fn description(&self) -> &str {
        match *self {
            DeleteInvitationsError::BadRequest(ref cause) => cause,
            DeleteInvitationsError::InternalServerError(ref cause) => cause,
            DeleteInvitationsError::Validation(ref cause) => cause,
            DeleteInvitationsError::Credentials(ref err) => err.description(),
            DeleteInvitationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteInvitationsError::ParseError(ref cause) => cause,
            DeleteInvitationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteMembers
#[derive(Debug, PartialEq)]
pub enum DeleteMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteMembersError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteMembersError {
    fn from(err: serde_json::error::Error) -> DeleteMembersError {
        DeleteMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMembersError {
    fn from(err: CredentialsError) -> DeleteMembersError {
        DeleteMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMembersError {
    fn from(err: HttpDispatchError) -> DeleteMembersError {
        DeleteMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMembersError {
    fn from(err: io::Error) -> DeleteMembersError {
        DeleteMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMembersError {
    fn description(&self) -> &str {
        match *self {
            DeleteMembersError::BadRequest(ref cause) => cause,
            DeleteMembersError::InternalServerError(ref cause) => cause,
            DeleteMembersError::Validation(ref cause) => cause,
            DeleteMembersError::Credentials(ref err) => err.description(),
            DeleteMembersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteMembersError::ParseError(ref cause) => cause,
            DeleteMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum DeleteThreatIntelSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteThreatIntelSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteThreatIntelSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteThreatIntelSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteThreatIntelSetError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteThreatIntelSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteThreatIntelSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteThreatIntelSetError {
    fn from(err: serde_json::error::Error) -> DeleteThreatIntelSetError {
        DeleteThreatIntelSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteThreatIntelSetError {
    fn from(err: CredentialsError) -> DeleteThreatIntelSetError {
        DeleteThreatIntelSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteThreatIntelSetError {
    fn from(err: HttpDispatchError) -> DeleteThreatIntelSetError {
        DeleteThreatIntelSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteThreatIntelSetError {
    fn from(err: io::Error) -> DeleteThreatIntelSetError {
        DeleteThreatIntelSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteThreatIntelSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThreatIntelSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteThreatIntelSetError::BadRequest(ref cause) => cause,
            DeleteThreatIntelSetError::InternalServerError(ref cause) => cause,
            DeleteThreatIntelSetError::Validation(ref cause) => cause,
            DeleteThreatIntelSetError::Credentials(ref err) => err.description(),
            DeleteThreatIntelSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteThreatIntelSetError::ParseError(ref cause) => cause,
            DeleteThreatIntelSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateFromMasterAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateFromMasterAccountError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateFromMasterAccountError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateFromMasterAccountError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DisassociateFromMasterAccountError::BadRequest(String::from(
                        error_message,
                    ));
                }
                "InternalServerErrorException" => {
                    return DisassociateFromMasterAccountError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateFromMasterAccountError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateFromMasterAccountError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateFromMasterAccountError {
    fn from(err: serde_json::error::Error) -> DisassociateFromMasterAccountError {
        DisassociateFromMasterAccountError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateFromMasterAccountError {
    fn from(err: CredentialsError) -> DisassociateFromMasterAccountError {
        DisassociateFromMasterAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateFromMasterAccountError {
    fn from(err: HttpDispatchError) -> DisassociateFromMasterAccountError {
        DisassociateFromMasterAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateFromMasterAccountError {
    fn from(err: io::Error) -> DisassociateFromMasterAccountError {
        DisassociateFromMasterAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateFromMasterAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateFromMasterAccountError {
    fn description(&self) -> &str {
        match *self {
            DisassociateFromMasterAccountError::BadRequest(ref cause) => cause,
            DisassociateFromMasterAccountError::InternalServerError(ref cause) => cause,
            DisassociateFromMasterAccountError::Validation(ref cause) => cause,
            DisassociateFromMasterAccountError::Credentials(ref err) => err.description(),
            DisassociateFromMasterAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateFromMasterAccountError::ParseError(ref cause) => cause,
            DisassociateFromMasterAccountError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateMembers
#[derive(Debug, PartialEq)]
pub enum DisassociateMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DisassociateMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DisassociateMembersError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateMembersError {
    fn from(err: serde_json::error::Error) -> DisassociateMembersError {
        DisassociateMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateMembersError {
    fn from(err: CredentialsError) -> DisassociateMembersError {
        DisassociateMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateMembersError {
    fn from(err: HttpDispatchError) -> DisassociateMembersError {
        DisassociateMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateMembersError {
    fn from(err: io::Error) -> DisassociateMembersError {
        DisassociateMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateMembersError {
    fn description(&self) -> &str {
        match *self {
            DisassociateMembersError::BadRequest(ref cause) => cause,
            DisassociateMembersError::InternalServerError(ref cause) => cause,
            DisassociateMembersError::Validation(ref cause) => cause,
            DisassociateMembersError::Credentials(ref err) => err.description(),
            DisassociateMembersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateMembersError::ParseError(ref cause) => cause,
            DisassociateMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDetector
#[derive(Debug, PartialEq)]
pub enum GetDetectorError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetDetectorError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetDetectorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetDetectorError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetDetectorError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return GetDetectorError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetDetectorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDetectorError {
    fn from(err: serde_json::error::Error) -> GetDetectorError {
        GetDetectorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDetectorError {
    fn from(err: CredentialsError) -> GetDetectorError {
        GetDetectorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDetectorError {
    fn from(err: HttpDispatchError) -> GetDetectorError {
        GetDetectorError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDetectorError {
    fn from(err: io::Error) -> GetDetectorError {
        GetDetectorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDetectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDetectorError {
    fn description(&self) -> &str {
        match *self {
            GetDetectorError::BadRequest(ref cause) => cause,
            GetDetectorError::InternalServerError(ref cause) => cause,
            GetDetectorError::Validation(ref cause) => cause,
            GetDetectorError::Credentials(ref err) => err.description(),
            GetDetectorError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDetectorError::ParseError(ref cause) => cause,
            GetDetectorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetFilter
#[derive(Debug, PartialEq)]
pub enum GetFilterError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFilterError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetFilterError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetFilterError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetFilterError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return GetFilterError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetFilterError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFilterError {
    fn from(err: serde_json::error::Error) -> GetFilterError {
        GetFilterError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFilterError {
    fn from(err: CredentialsError) -> GetFilterError {
        GetFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFilterError {
    fn from(err: HttpDispatchError) -> GetFilterError {
        GetFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFilterError {
    fn from(err: io::Error) -> GetFilterError {
        GetFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFilterError {
    fn description(&self) -> &str {
        match *self {
            GetFilterError::BadRequest(ref cause) => cause,
            GetFilterError::InternalServerError(ref cause) => cause,
            GetFilterError::Validation(ref cause) => cause,
            GetFilterError::Credentials(ref err) => err.description(),
            GetFilterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFilterError::ParseError(ref cause) => cause,
            GetFilterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetFindings
#[derive(Debug, PartialEq)]
pub enum GetFindingsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFindingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetFindingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetFindingsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetFindingsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return GetFindingsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetFindingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFindingsError {
    fn from(err: serde_json::error::Error) -> GetFindingsError {
        GetFindingsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFindingsError {
    fn from(err: CredentialsError) -> GetFindingsError {
        GetFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFindingsError {
    fn from(err: HttpDispatchError) -> GetFindingsError {
        GetFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFindingsError {
    fn from(err: io::Error) -> GetFindingsError {
        GetFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFindingsError {
    fn description(&self) -> &str {
        match *self {
            GetFindingsError::BadRequest(ref cause) => cause,
            GetFindingsError::InternalServerError(ref cause) => cause,
            GetFindingsError::Validation(ref cause) => cause,
            GetFindingsError::Credentials(ref err) => err.description(),
            GetFindingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFindingsError::ParseError(ref cause) => cause,
            GetFindingsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetFindingsStatistics
#[derive(Debug, PartialEq)]
pub enum GetFindingsStatisticsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFindingsStatisticsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetFindingsStatisticsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetFindingsStatisticsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetFindingsStatisticsError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetFindingsStatisticsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetFindingsStatisticsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFindingsStatisticsError {
    fn from(err: serde_json::error::Error) -> GetFindingsStatisticsError {
        GetFindingsStatisticsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFindingsStatisticsError {
    fn from(err: CredentialsError) -> GetFindingsStatisticsError {
        GetFindingsStatisticsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFindingsStatisticsError {
    fn from(err: HttpDispatchError) -> GetFindingsStatisticsError {
        GetFindingsStatisticsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFindingsStatisticsError {
    fn from(err: io::Error) -> GetFindingsStatisticsError {
        GetFindingsStatisticsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFindingsStatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFindingsStatisticsError {
    fn description(&self) -> &str {
        match *self {
            GetFindingsStatisticsError::BadRequest(ref cause) => cause,
            GetFindingsStatisticsError::InternalServerError(ref cause) => cause,
            GetFindingsStatisticsError::Validation(ref cause) => cause,
            GetFindingsStatisticsError::Credentials(ref err) => err.description(),
            GetFindingsStatisticsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetFindingsStatisticsError::ParseError(ref cause) => cause,
            GetFindingsStatisticsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetIPSet
#[derive(Debug, PartialEq)]
pub enum GetIPSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetIPSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetIPSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetIPSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetIPSetError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return GetIPSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetIPSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetIPSetError {
    fn from(err: serde_json::error::Error) -> GetIPSetError {
        GetIPSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIPSetError {
    fn from(err: CredentialsError) -> GetIPSetError {
        GetIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIPSetError {
    fn from(err: HttpDispatchError) -> GetIPSetError {
        GetIPSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIPSetError {
    fn from(err: io::Error) -> GetIPSetError {
        GetIPSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIPSetError {
    fn description(&self) -> &str {
        match *self {
            GetIPSetError::BadRequest(ref cause) => cause,
            GetIPSetError::InternalServerError(ref cause) => cause,
            GetIPSetError::Validation(ref cause) => cause,
            GetIPSetError::Credentials(ref err) => err.description(),
            GetIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetIPSetError::ParseError(ref cause) => cause,
            GetIPSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetInvitationsCount
#[derive(Debug, PartialEq)]
pub enum GetInvitationsCountError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetInvitationsCountError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetInvitationsCountError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetInvitationsCountError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetInvitationsCountError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetInvitationsCountError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetInvitationsCountError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetInvitationsCountError {
    fn from(err: serde_json::error::Error) -> GetInvitationsCountError {
        GetInvitationsCountError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInvitationsCountError {
    fn from(err: CredentialsError) -> GetInvitationsCountError {
        GetInvitationsCountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInvitationsCountError {
    fn from(err: HttpDispatchError) -> GetInvitationsCountError {
        GetInvitationsCountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInvitationsCountError {
    fn from(err: io::Error) -> GetInvitationsCountError {
        GetInvitationsCountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInvitationsCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInvitationsCountError {
    fn description(&self) -> &str {
        match *self {
            GetInvitationsCountError::BadRequest(ref cause) => cause,
            GetInvitationsCountError::InternalServerError(ref cause) => cause,
            GetInvitationsCountError::Validation(ref cause) => cause,
            GetInvitationsCountError::Credentials(ref err) => err.description(),
            GetInvitationsCountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInvitationsCountError::ParseError(ref cause) => cause,
            GetInvitationsCountError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetMasterAccount
#[derive(Debug, PartialEq)]
pub enum GetMasterAccountError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetMasterAccountError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetMasterAccountError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetMasterAccountError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetMasterAccountError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return GetMasterAccountError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetMasterAccountError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetMasterAccountError {
    fn from(err: serde_json::error::Error) -> GetMasterAccountError {
        GetMasterAccountError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMasterAccountError {
    fn from(err: CredentialsError) -> GetMasterAccountError {
        GetMasterAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMasterAccountError {
    fn from(err: HttpDispatchError) -> GetMasterAccountError {
        GetMasterAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMasterAccountError {
    fn from(err: io::Error) -> GetMasterAccountError {
        GetMasterAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMasterAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMasterAccountError {
    fn description(&self) -> &str {
        match *self {
            GetMasterAccountError::BadRequest(ref cause) => cause,
            GetMasterAccountError::InternalServerError(ref cause) => cause,
            GetMasterAccountError::Validation(ref cause) => cause,
            GetMasterAccountError::Credentials(ref err) => err.description(),
            GetMasterAccountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetMasterAccountError::ParseError(ref cause) => cause,
            GetMasterAccountError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetMembers
#[derive(Debug, PartialEq)]
pub enum GetMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetMembersError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return GetMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetMembersError {
    fn from(err: serde_json::error::Error) -> GetMembersError {
        GetMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMembersError {
    fn from(err: CredentialsError) -> GetMembersError {
        GetMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMembersError {
    fn from(err: HttpDispatchError) -> GetMembersError {
        GetMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMembersError {
    fn from(err: io::Error) -> GetMembersError {
        GetMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMembersError {
    fn description(&self) -> &str {
        match *self {
            GetMembersError::BadRequest(ref cause) => cause,
            GetMembersError::InternalServerError(ref cause) => cause,
            GetMembersError::Validation(ref cause) => cause,
            GetMembersError::Credentials(ref err) => err.description(),
            GetMembersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetMembersError::ParseError(ref cause) => cause,
            GetMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum GetThreatIntelSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetThreatIntelSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetThreatIntelSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetThreatIntelSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetThreatIntelSetError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return GetThreatIntelSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetThreatIntelSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetThreatIntelSetError {
    fn from(err: serde_json::error::Error) -> GetThreatIntelSetError {
        GetThreatIntelSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetThreatIntelSetError {
    fn from(err: CredentialsError) -> GetThreatIntelSetError {
        GetThreatIntelSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetThreatIntelSetError {
    fn from(err: HttpDispatchError) -> GetThreatIntelSetError {
        GetThreatIntelSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetThreatIntelSetError {
    fn from(err: io::Error) -> GetThreatIntelSetError {
        GetThreatIntelSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetThreatIntelSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetThreatIntelSetError {
    fn description(&self) -> &str {
        match *self {
            GetThreatIntelSetError::BadRequest(ref cause) => cause,
            GetThreatIntelSetError::InternalServerError(ref cause) => cause,
            GetThreatIntelSetError::Validation(ref cause) => cause,
            GetThreatIntelSetError::Credentials(ref err) => err.description(),
            GetThreatIntelSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetThreatIntelSetError::ParseError(ref cause) => cause,
            GetThreatIntelSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by InviteMembers
#[derive(Debug, PartialEq)]
pub enum InviteMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl InviteMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> InviteMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return InviteMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return InviteMembersError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return InviteMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return InviteMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for InviteMembersError {
    fn from(err: serde_json::error::Error) -> InviteMembersError {
        InviteMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for InviteMembersError {
    fn from(err: CredentialsError) -> InviteMembersError {
        InviteMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InviteMembersError {
    fn from(err: HttpDispatchError) -> InviteMembersError {
        InviteMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for InviteMembersError {
    fn from(err: io::Error) -> InviteMembersError {
        InviteMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InviteMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InviteMembersError {
    fn description(&self) -> &str {
        match *self {
            InviteMembersError::BadRequest(ref cause) => cause,
            InviteMembersError::InternalServerError(ref cause) => cause,
            InviteMembersError::Validation(ref cause) => cause,
            InviteMembersError::Credentials(ref err) => err.description(),
            InviteMembersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            InviteMembersError::ParseError(ref cause) => cause,
            InviteMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDetectors
#[derive(Debug, PartialEq)]
pub enum ListDetectorsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListDetectorsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListDetectorsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListDetectorsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListDetectorsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return ListDetectorsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDetectorsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDetectorsError {
    fn from(err: serde_json::error::Error) -> ListDetectorsError {
        ListDetectorsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDetectorsError {
    fn from(err: CredentialsError) -> ListDetectorsError {
        ListDetectorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDetectorsError {
    fn from(err: HttpDispatchError) -> ListDetectorsError {
        ListDetectorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDetectorsError {
    fn from(err: io::Error) -> ListDetectorsError {
        ListDetectorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDetectorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDetectorsError {
    fn description(&self) -> &str {
        match *self {
            ListDetectorsError::BadRequest(ref cause) => cause,
            ListDetectorsError::InternalServerError(ref cause) => cause,
            ListDetectorsError::Validation(ref cause) => cause,
            ListDetectorsError::Credentials(ref err) => err.description(),
            ListDetectorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDetectorsError::ParseError(ref cause) => cause,
            ListDetectorsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListFilters
#[derive(Debug, PartialEq)]
pub enum ListFiltersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListFiltersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListFiltersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListFiltersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListFiltersError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return ListFiltersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListFiltersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListFiltersError {
    fn from(err: serde_json::error::Error) -> ListFiltersError {
        ListFiltersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFiltersError {
    fn from(err: CredentialsError) -> ListFiltersError {
        ListFiltersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFiltersError {
    fn from(err: HttpDispatchError) -> ListFiltersError {
        ListFiltersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFiltersError {
    fn from(err: io::Error) -> ListFiltersError {
        ListFiltersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFiltersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFiltersError {
    fn description(&self) -> &str {
        match *self {
            ListFiltersError::BadRequest(ref cause) => cause,
            ListFiltersError::InternalServerError(ref cause) => cause,
            ListFiltersError::Validation(ref cause) => cause,
            ListFiltersError::Credentials(ref err) => err.description(),
            ListFiltersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFiltersError::ParseError(ref cause) => cause,
            ListFiltersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListFindings
#[derive(Debug, PartialEq)]
pub enum ListFindingsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListFindingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListFindingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListFindingsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListFindingsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return ListFindingsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListFindingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListFindingsError {
    fn from(err: serde_json::error::Error) -> ListFindingsError {
        ListFindingsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFindingsError {
    fn from(err: CredentialsError) -> ListFindingsError {
        ListFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFindingsError {
    fn from(err: HttpDispatchError) -> ListFindingsError {
        ListFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFindingsError {
    fn from(err: io::Error) -> ListFindingsError {
        ListFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFindingsError {
    fn description(&self) -> &str {
        match *self {
            ListFindingsError::BadRequest(ref cause) => cause,
            ListFindingsError::InternalServerError(ref cause) => cause,
            ListFindingsError::Validation(ref cause) => cause,
            ListFindingsError::Credentials(ref err) => err.description(),
            ListFindingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFindingsError::ParseError(ref cause) => cause,
            ListFindingsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListIPSets
#[derive(Debug, PartialEq)]
pub enum ListIPSetsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListIPSetsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListIPSetsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListIPSetsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListIPSetsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return ListIPSetsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListIPSetsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListIPSetsError {
    fn from(err: serde_json::error::Error) -> ListIPSetsError {
        ListIPSetsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIPSetsError {
    fn from(err: CredentialsError) -> ListIPSetsError {
        ListIPSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIPSetsError {
    fn from(err: HttpDispatchError) -> ListIPSetsError {
        ListIPSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIPSetsError {
    fn from(err: io::Error) -> ListIPSetsError {
        ListIPSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIPSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIPSetsError {
    fn description(&self) -> &str {
        match *self {
            ListIPSetsError::BadRequest(ref cause) => cause,
            ListIPSetsError::InternalServerError(ref cause) => cause,
            ListIPSetsError::Validation(ref cause) => cause,
            ListIPSetsError::Credentials(ref err) => err.description(),
            ListIPSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListIPSetsError::ParseError(ref cause) => cause,
            ListIPSetsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListInvitations
#[derive(Debug, PartialEq)]
pub enum ListInvitationsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListInvitationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListInvitationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListInvitationsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListInvitationsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return ListInvitationsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListInvitationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListInvitationsError {
    fn from(err: serde_json::error::Error) -> ListInvitationsError {
        ListInvitationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListInvitationsError {
    fn from(err: CredentialsError) -> ListInvitationsError {
        ListInvitationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInvitationsError {
    fn from(err: HttpDispatchError) -> ListInvitationsError {
        ListInvitationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInvitationsError {
    fn from(err: io::Error) -> ListInvitationsError {
        ListInvitationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInvitationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInvitationsError {
    fn description(&self) -> &str {
        match *self {
            ListInvitationsError::BadRequest(ref cause) => cause,
            ListInvitationsError::InternalServerError(ref cause) => cause,
            ListInvitationsError::Validation(ref cause) => cause,
            ListInvitationsError::Credentials(ref err) => err.description(),
            ListInvitationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListInvitationsError::ParseError(ref cause) => cause,
            ListInvitationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListMembers
#[derive(Debug, PartialEq)]
pub enum ListMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListMembersError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return ListMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListMembersError {
    fn from(err: serde_json::error::Error) -> ListMembersError {
        ListMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListMembersError {
    fn from(err: CredentialsError) -> ListMembersError {
        ListMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListMembersError {
    fn from(err: HttpDispatchError) -> ListMembersError {
        ListMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListMembersError {
    fn from(err: io::Error) -> ListMembersError {
        ListMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMembersError {
    fn description(&self) -> &str {
        match *self {
            ListMembersError::BadRequest(ref cause) => cause,
            ListMembersError::InternalServerError(ref cause) => cause,
            ListMembersError::Validation(ref cause) => cause,
            ListMembersError::Credentials(ref err) => err.description(),
            ListMembersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListMembersError::ParseError(ref cause) => cause,
            ListMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListThreatIntelSets
#[derive(Debug, PartialEq)]
pub enum ListThreatIntelSetsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListThreatIntelSetsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListThreatIntelSetsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListThreatIntelSetsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListThreatIntelSetsError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListThreatIntelSetsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListThreatIntelSetsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListThreatIntelSetsError {
    fn from(err: serde_json::error::Error) -> ListThreatIntelSetsError {
        ListThreatIntelSetsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThreatIntelSetsError {
    fn from(err: CredentialsError) -> ListThreatIntelSetsError {
        ListThreatIntelSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThreatIntelSetsError {
    fn from(err: HttpDispatchError) -> ListThreatIntelSetsError {
        ListThreatIntelSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThreatIntelSetsError {
    fn from(err: io::Error) -> ListThreatIntelSetsError {
        ListThreatIntelSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThreatIntelSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThreatIntelSetsError {
    fn description(&self) -> &str {
        match *self {
            ListThreatIntelSetsError::BadRequest(ref cause) => cause,
            ListThreatIntelSetsError::InternalServerError(ref cause) => cause,
            ListThreatIntelSetsError::Validation(ref cause) => cause,
            ListThreatIntelSetsError::Credentials(ref err) => err.description(),
            ListThreatIntelSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListThreatIntelSetsError::ParseError(ref cause) => cause,
            ListThreatIntelSetsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartMonitoringMembers
#[derive(Debug, PartialEq)]
pub enum StartMonitoringMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartMonitoringMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> StartMonitoringMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return StartMonitoringMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return StartMonitoringMembersError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartMonitoringMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StartMonitoringMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartMonitoringMembersError {
    fn from(err: serde_json::error::Error) -> StartMonitoringMembersError {
        StartMonitoringMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartMonitoringMembersError {
    fn from(err: CredentialsError) -> StartMonitoringMembersError {
        StartMonitoringMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartMonitoringMembersError {
    fn from(err: HttpDispatchError) -> StartMonitoringMembersError {
        StartMonitoringMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartMonitoringMembersError {
    fn from(err: io::Error) -> StartMonitoringMembersError {
        StartMonitoringMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartMonitoringMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartMonitoringMembersError {
    fn description(&self) -> &str {
        match *self {
            StartMonitoringMembersError::BadRequest(ref cause) => cause,
            StartMonitoringMembersError::InternalServerError(ref cause) => cause,
            StartMonitoringMembersError::Validation(ref cause) => cause,
            StartMonitoringMembersError::Credentials(ref err) => err.description(),
            StartMonitoringMembersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartMonitoringMembersError::ParseError(ref cause) => cause,
            StartMonitoringMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopMonitoringMembers
#[derive(Debug, PartialEq)]
pub enum StopMonitoringMembersError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StopMonitoringMembersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> StopMonitoringMembersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return StopMonitoringMembersError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return StopMonitoringMembersError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StopMonitoringMembersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StopMonitoringMembersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopMonitoringMembersError {
    fn from(err: serde_json::error::Error) -> StopMonitoringMembersError {
        StopMonitoringMembersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopMonitoringMembersError {
    fn from(err: CredentialsError) -> StopMonitoringMembersError {
        StopMonitoringMembersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopMonitoringMembersError {
    fn from(err: HttpDispatchError) -> StopMonitoringMembersError {
        StopMonitoringMembersError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopMonitoringMembersError {
    fn from(err: io::Error) -> StopMonitoringMembersError {
        StopMonitoringMembersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopMonitoringMembersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopMonitoringMembersError {
    fn description(&self) -> &str {
        match *self {
            StopMonitoringMembersError::BadRequest(ref cause) => cause,
            StopMonitoringMembersError::InternalServerError(ref cause) => cause,
            StopMonitoringMembersError::Validation(ref cause) => cause,
            StopMonitoringMembersError::Credentials(ref err) => err.description(),
            StopMonitoringMembersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopMonitoringMembersError::ParseError(ref cause) => cause,
            StopMonitoringMembersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UnarchiveFindings
#[derive(Debug, PartialEq)]
pub enum UnarchiveFindingsError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UnarchiveFindingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UnarchiveFindingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UnarchiveFindingsError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UnarchiveFindingsError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return UnarchiveFindingsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UnarchiveFindingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UnarchiveFindingsError {
    fn from(err: serde_json::error::Error) -> UnarchiveFindingsError {
        UnarchiveFindingsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UnarchiveFindingsError {
    fn from(err: CredentialsError) -> UnarchiveFindingsError {
        UnarchiveFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnarchiveFindingsError {
    fn from(err: HttpDispatchError) -> UnarchiveFindingsError {
        UnarchiveFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UnarchiveFindingsError {
    fn from(err: io::Error) -> UnarchiveFindingsError {
        UnarchiveFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UnarchiveFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnarchiveFindingsError {
    fn description(&self) -> &str {
        match *self {
            UnarchiveFindingsError::BadRequest(ref cause) => cause,
            UnarchiveFindingsError::InternalServerError(ref cause) => cause,
            UnarchiveFindingsError::Validation(ref cause) => cause,
            UnarchiveFindingsError::Credentials(ref err) => err.description(),
            UnarchiveFindingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UnarchiveFindingsError::ParseError(ref cause) => cause,
            UnarchiveFindingsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateDetector
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateDetectorError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDetectorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateDetectorError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateDetectorError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateDetectorError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateDetectorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDetectorError {
    fn from(err: serde_json::error::Error) -> UpdateDetectorError {
        UpdateDetectorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDetectorError {
    fn from(err: CredentialsError) -> UpdateDetectorError {
        UpdateDetectorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDetectorError {
    fn from(err: HttpDispatchError) -> UpdateDetectorError {
        UpdateDetectorError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDetectorError {
    fn from(err: io::Error) -> UpdateDetectorError {
        UpdateDetectorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDetectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDetectorError {
    fn description(&self) -> &str {
        match *self {
            UpdateDetectorError::BadRequest(ref cause) => cause,
            UpdateDetectorError::InternalServerError(ref cause) => cause,
            UpdateDetectorError::Validation(ref cause) => cause,
            UpdateDetectorError::Credentials(ref err) => err.description(),
            UpdateDetectorError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDetectorError::ParseError(ref cause) => cause,
            UpdateDetectorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateFilter
#[derive(Debug, PartialEq)]
pub enum UpdateFilterError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateFilterError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateFilterError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateFilterError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateFilterError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateFilterError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateFilterError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateFilterError {
    fn from(err: serde_json::error::Error) -> UpdateFilterError {
        UpdateFilterError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFilterError {
    fn from(err: CredentialsError) -> UpdateFilterError {
        UpdateFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFilterError {
    fn from(err: HttpDispatchError) -> UpdateFilterError {
        UpdateFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFilterError {
    fn from(err: io::Error) -> UpdateFilterError {
        UpdateFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFilterError {
    fn description(&self) -> &str {
        match *self {
            UpdateFilterError::BadRequest(ref cause) => cause,
            UpdateFilterError::InternalServerError(ref cause) => cause,
            UpdateFilterError::Validation(ref cause) => cause,
            UpdateFilterError::Credentials(ref err) => err.description(),
            UpdateFilterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateFilterError::ParseError(ref cause) => cause,
            UpdateFilterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateFindingsFeedback
#[derive(Debug, PartialEq)]
pub enum UpdateFindingsFeedbackError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateFindingsFeedbackError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateFindingsFeedbackError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateFindingsFeedbackError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateFindingsFeedbackError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return UpdateFindingsFeedbackError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateFindingsFeedbackError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateFindingsFeedbackError {
    fn from(err: serde_json::error::Error) -> UpdateFindingsFeedbackError {
        UpdateFindingsFeedbackError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFindingsFeedbackError {
    fn from(err: CredentialsError) -> UpdateFindingsFeedbackError {
        UpdateFindingsFeedbackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFindingsFeedbackError {
    fn from(err: HttpDispatchError) -> UpdateFindingsFeedbackError {
        UpdateFindingsFeedbackError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFindingsFeedbackError {
    fn from(err: io::Error) -> UpdateFindingsFeedbackError {
        UpdateFindingsFeedbackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFindingsFeedbackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFindingsFeedbackError {
    fn description(&self) -> &str {
        match *self {
            UpdateFindingsFeedbackError::BadRequest(ref cause) => cause,
            UpdateFindingsFeedbackError::InternalServerError(ref cause) => cause,
            UpdateFindingsFeedbackError::Validation(ref cause) => cause,
            UpdateFindingsFeedbackError::Credentials(ref err) => err.description(),
            UpdateFindingsFeedbackError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateFindingsFeedbackError::ParseError(ref cause) => cause,
            UpdateFindingsFeedbackError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateIPSet
#[derive(Debug, PartialEq)]
pub enum UpdateIPSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateIPSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateIPSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateIPSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateIPSetError::InternalServerError(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateIPSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateIPSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateIPSetError {
    fn from(err: serde_json::error::Error) -> UpdateIPSetError {
        UpdateIPSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateIPSetError {
    fn from(err: CredentialsError) -> UpdateIPSetError {
        UpdateIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateIPSetError {
    fn from(err: HttpDispatchError) -> UpdateIPSetError {
        UpdateIPSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateIPSetError {
    fn from(err: io::Error) -> UpdateIPSetError {
        UpdateIPSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIPSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateIPSetError::BadRequest(ref cause) => cause,
            UpdateIPSetError::InternalServerError(ref cause) => cause,
            UpdateIPSetError::Validation(ref cause) => cause,
            UpdateIPSetError::Credentials(ref err) => err.description(),
            UpdateIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateIPSetError::ParseError(ref cause) => cause,
            UpdateIPSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateThreatIntelSet
#[derive(Debug, PartialEq)]
pub enum UpdateThreatIntelSetError {
    /// <p>Error response object.</p>
    BadRequest(String),
    /// <p>Error response object.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateThreatIntelSetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateThreatIntelSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateThreatIntelSetError::BadRequest(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateThreatIntelSetError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return UpdateThreatIntelSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateThreatIntelSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateThreatIntelSetError {
    fn from(err: serde_json::error::Error) -> UpdateThreatIntelSetError {
        UpdateThreatIntelSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateThreatIntelSetError {
    fn from(err: CredentialsError) -> UpdateThreatIntelSetError {
        UpdateThreatIntelSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateThreatIntelSetError {
    fn from(err: HttpDispatchError) -> UpdateThreatIntelSetError {
        UpdateThreatIntelSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateThreatIntelSetError {
    fn from(err: io::Error) -> UpdateThreatIntelSetError {
        UpdateThreatIntelSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateThreatIntelSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateThreatIntelSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateThreatIntelSetError::BadRequest(ref cause) => cause,
            UpdateThreatIntelSetError::InternalServerError(ref cause) => cause,
            UpdateThreatIntelSetError::Validation(ref cause) => cause,
            UpdateThreatIntelSetError::Credentials(ref err) => err.description(),
            UpdateThreatIntelSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateThreatIntelSetError::ParseError(ref cause) => cause,
            UpdateThreatIntelSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon GuardDuty API. Amazon GuardDuty clients implement this trait.
pub trait GuardDuty {
    /// <p>Accepts the invitation to be monitored by a master GuardDuty account.</p>
    fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> RusotoFuture<AcceptInvitationResponse, AcceptInvitationError>;

    /// <p>Archives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn archive_findings(
        &self,
        input: ArchiveFindingsRequest,
    ) -> RusotoFuture<ArchiveFindingsResponse, ArchiveFindingsError>;

    /// <p>Creates a single Amazon GuardDuty detector. A detector is an object that represents the GuardDuty service. A detector must be created in order for GuardDuty to become operational.</p>
    fn create_detector(
        &self,
        input: CreateDetectorRequest,
    ) -> RusotoFuture<CreateDetectorResponse, CreateDetectorError>;

    /// <p>Creates a filter using the specified finding criteria.</p>
    fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> RusotoFuture<CreateFilterResponse, CreateFilterError>;

    /// <p>Creates a new IPSet - a list of trusted IP addresses that have been whitelisted for secure communication with AWS infrastructure and applications.</p>
    fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> RusotoFuture<CreateIPSetResponse, CreateIPSetError>;

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. The current AWS account can then invite these members to manage GuardDuty in their accounts.</p>
    fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> RusotoFuture<CreateMembersResponse, CreateMembersError>;

    /// <p>Generates example findings of types specified by the list of finding types. If &#39;NULL&#39; is specified for findingTypes, the API generates example findings of all supported finding types.</p>
    fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> RusotoFuture<CreateSampleFindingsResponse, CreateSampleFindingsError>;

    /// <p>Create a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets.</p>
    fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> RusotoFuture<CreateThreatIntelSetResponse, CreateThreatIntelSetError>;

    /// <p>Declines invitations sent to the current member account by AWS account specified by their account IDs.</p>
    fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> RusotoFuture<DeclineInvitationsResponse, DeclineInvitationsError>;

    /// <p>Deletes a Amazon GuardDuty detector specified by the detector ID.</p>
    fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> RusotoFuture<DeleteDetectorResponse, DeleteDetectorError>;

    /// <p>Deletes the filter specified by the filter name.</p>
    fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> RusotoFuture<DeleteFilterResponse, DeleteFilterError>;

    /// <p>Deletes the IPSet specified by the IPSet ID.</p>
    fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> RusotoFuture<DeleteIPSetResponse, DeleteIPSetError>;

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> RusotoFuture<DeleteInvitationsResponse, DeleteInvitationsError>;

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> RusotoFuture<DeleteMembersResponse, DeleteMembersError>;

    /// <p>Deletes ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> RusotoFuture<DeleteThreatIntelSetResponse, DeleteThreatIntelSetError>;

    /// <p>Disassociates the current GuardDuty member account from its master account.</p>
    fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> RusotoFuture<DisassociateFromMasterAccountResponse, DisassociateFromMasterAccountError>;

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> RusotoFuture<DisassociateMembersResponse, DisassociateMembersError>;

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    fn get_detector(
        &self,
        input: GetDetectorRequest,
    ) -> RusotoFuture<GetDetectorResponse, GetDetectorError>;

    /// <p>Returns the details of the filter specified by the filter name.</p>
    fn get_filter(
        &self,
        input: GetFilterRequest,
    ) -> RusotoFuture<GetFilterResponse, GetFilterError>;

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> RusotoFuture<GetFindingsResponse, GetFindingsError>;

    /// <p>Lists Amazon GuardDuty findings&#39; statistics for the specified detector ID.</p>
    fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> RusotoFuture<GetFindingsStatisticsResponse, GetFindingsStatisticsError>;

    /// <p>Retrieves the IPSet specified by the IPSet ID.</p>
    fn get_ip_set(&self, input: GetIPSetRequest) -> RusotoFuture<GetIPSetResponse, GetIPSetError>;

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    fn get_invitations_count(
        &self,
    ) -> RusotoFuture<GetInvitationsCountResponse, GetInvitationsCountError>;

    /// <p>Provides the details for the GuardDuty master account to the current GuardDuty member account.</p>
    fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> RusotoFuture<GetMasterAccountResponse, GetMasterAccountError>;

    /// <p>Retrieves GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> RusotoFuture<GetMembersResponse, GetMembersError>;

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> RusotoFuture<GetThreatIntelSetResponse, GetThreatIntelSetError>;

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty and allow the current AWS account to view and manage these accounts&#39; GuardDuty findings on their behalf as the master account.</p>
    fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> RusotoFuture<InviteMembersResponse, InviteMembersError>;

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> RusotoFuture<ListDetectorsResponse, ListDetectorsError>;

    /// <p>Returns a paginated list of the current filters.</p>
    fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> RusotoFuture<ListFiltersResponse, ListFiltersError>;

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> RusotoFuture<ListFindingsResponse, ListFindingsError>;

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID.</p>
    fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> RusotoFuture<ListIPSetsResponse, ListIPSetsError>;

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> RusotoFuture<ListInvitationsResponse, ListInvitationsError>;

    /// <p>Lists details about all member accounts for the current GuardDuty master account.</p>
    fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> RusotoFuture<ListMembersResponse, ListMembersError>;

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID.</p>
    fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> RusotoFuture<ListThreatIntelSetsResponse, ListThreatIntelSetsError>;

    /// <p>Re-enables GuardDuty to monitor findings of the member accounts specified by the account IDs. A master GuardDuty account can run this command after disabling GuardDuty from monitoring these members&#39; findings by running StopMonitoringMembers.</p>
    fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> RusotoFuture<StartMonitoringMembersResponse, StartMonitoringMembersError>;

    /// <p>Disables GuardDuty from monitoring findings of the member accounts specified by the account IDs. After running this command, a master GuardDuty account can run StartMonitoringMembers to re-enable GuardDuty to monitor these members’ findings.</p>
    fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> RusotoFuture<StopMonitoringMembersResponse, StopMonitoringMembersError>;

    /// <p>Unarchives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> RusotoFuture<UnarchiveFindingsResponse, UnarchiveFindingsError>;

    /// <p>Updates an Amazon GuardDuty detector specified by the detectorId.</p>
    fn update_detector(
        &self,
        input: UpdateDetectorRequest,
    ) -> RusotoFuture<UpdateDetectorResponse, UpdateDetectorError>;

    /// <p>Updates the filter specified by the filter name.</p>
    fn update_filter(
        &self,
        input: UpdateFilterRequest,
    ) -> RusotoFuture<UpdateFilterResponse, UpdateFilterError>;

    /// <p>Marks specified Amazon GuardDuty findings as useful or not useful.</p>
    fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> RusotoFuture<UpdateFindingsFeedbackResponse, UpdateFindingsFeedbackError>;

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> RusotoFuture<UpdateIPSetResponse, UpdateIPSetError>;

    /// <p>Updates the ThreatIntelSet specified by ThreatIntelSet ID.</p>
    fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> RusotoFuture<UpdateThreatIntelSetResponse, UpdateThreatIntelSetError>;
}
/// A client for the Amazon GuardDuty API.
#[derive(Clone)]
pub struct GuardDutyClient {
    client: Client,
    region: region::Region,
}

impl GuardDutyClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GuardDutyClient {
        GuardDutyClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GuardDutyClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        GuardDutyClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl GuardDuty for GuardDutyClient {
    /// <p>Accepts the invitation to be monitored by a master GuardDuty account.</p>
    fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> RusotoFuture<AcceptInvitationResponse, AcceptInvitationError> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AcceptInvitationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AcceptInvitationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Archives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn archive_findings(
        &self,
        input: ArchiveFindingsRequest,
    ) -> RusotoFuture<ArchiveFindingsResponse, ArchiveFindingsError> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/archive",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ArchiveFindingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ArchiveFindingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a single Amazon GuardDuty detector. A detector is an object that represents the GuardDuty service. A detector must be created in order for GuardDuty to become operational.</p>
    fn create_detector(
        &self,
        input: CreateDetectorRequest,
    ) -> RusotoFuture<CreateDetectorResponse, CreateDetectorError> {
        let request_uri = "/detector";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateDetectorResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDetectorError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a filter using the specified finding criteria.</p>
    fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> RusotoFuture<CreateFilterResponse, CreateFilterError> {
        let request_uri = format!(
            "/detector/{detector_id}/filter",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateFilterResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateFilterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new IPSet - a list of trusted IP addresses that have been whitelisted for secure communication with AWS infrastructure and applications.</p>
    fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> RusotoFuture<CreateIPSetResponse, CreateIPSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateIPSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. The current AWS account can then invite these members to manage GuardDuty in their accounts.</p>
    fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> RusotoFuture<CreateMembersResponse, CreateMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateMembersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Generates example findings of types specified by the list of finding types. If &#39;NULL&#39; is specified for findingTypes, the API generates example findings of all supported finding types.</p>
    fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> RusotoFuture<CreateSampleFindingsResponse, CreateSampleFindingsError> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/create",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateSampleFindingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateSampleFindingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Create a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets.</p>
    fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> RusotoFuture<CreateThreatIntelSetResponse, CreateThreatIntelSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateThreatIntelSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateThreatIntelSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Declines invitations sent to the current member account by AWS account specified by their account IDs.</p>
    fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> RusotoFuture<DeclineInvitationsResponse, DeclineInvitationsError> {
        let request_uri = "/invitation/decline";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeclineInvitationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeclineInvitationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a Amazon GuardDuty detector specified by the detector ID.</p>
    fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> RusotoFuture<DeleteDetectorResponse, DeleteDetectorError> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteDetectorResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDetectorError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the filter specified by the filter name.</p>
    fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> RusotoFuture<DeleteFilterResponse, DeleteFilterError> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteFilterResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFilterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the IPSet specified by the IPSet ID.</p>
    fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> RusotoFuture<DeleteIPSetResponse, DeleteIPSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteIPSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> RusotoFuture<DeleteInvitationsResponse, DeleteInvitationsError> {
        let request_uri = "/invitation/delete";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteInvitationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteInvitationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> RusotoFuture<DeleteMembersResponse, DeleteMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member/delete",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteMembersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> RusotoFuture<DeleteThreatIntelSetResponse, DeleteThreatIntelSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteThreatIntelSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteThreatIntelSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Disassociates the current GuardDuty member account from its master account.</p>
    fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> RusotoFuture<DisassociateFromMasterAccountResponse, DisassociateFromMasterAccountError>
    {
        let request_uri = format!(
            "/detector/{detector_id}/master/disassociate",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DisassociateFromMasterAccountResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateFromMasterAccountError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> RusotoFuture<DisassociateMembersResponse, DisassociateMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member/disassociate",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DisassociateMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisassociateMembersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    fn get_detector(
        &self,
        input: GetDetectorRequest,
    ) -> RusotoFuture<GetDetectorResponse, GetDetectorError> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetDetectorResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDetectorError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the details of the filter specified by the filter name.</p>
    fn get_filter(
        &self,
        input: GetFilterRequest,
    ) -> RusotoFuture<GetFilterResponse, GetFilterError> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFilterResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFilterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> RusotoFuture<GetFindingsResponse, GetFindingsError> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/get",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFindingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFindingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists Amazon GuardDuty findings&#39; statistics for the specified detector ID.</p>
    fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> RusotoFuture<GetFindingsStatisticsResponse, GetFindingsStatisticsError> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/statistics",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetFindingsStatisticsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetFindingsStatisticsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves the IPSet specified by the IPSet ID.</p>
    fn get_ip_set(&self, input: GetIPSetRequest) -> RusotoFuture<GetIPSetResponse, GetIPSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetIPSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    fn get_invitations_count(
        &self,
    ) -> RusotoFuture<GetInvitationsCountResponse, GetInvitationsCountError> {
        let request_uri = "/invitation/count";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetInvitationsCountResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetInvitationsCountError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Provides the details for the GuardDuty master account to the current GuardDuty member account.</p>
    fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> RusotoFuture<GetMasterAccountResponse, GetMasterAccountError> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetMasterAccountResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMasterAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> RusotoFuture<GetMembersResponse, GetMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member/get",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMembersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> RusotoFuture<GetThreatIntelSetResponse, GetThreatIntelSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetThreatIntelSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetThreatIntelSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty and allow the current AWS account to view and manage these accounts&#39; GuardDuty findings on their behalf as the master account.</p>
    fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> RusotoFuture<InviteMembersResponse, InviteMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member/invite",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<InviteMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InviteMembersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> RusotoFuture<ListDetectorsResponse, ListDetectorsError> {
        let request_uri = "/detector";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDetectorsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDetectorsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a paginated list of the current filters.</p>
    fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> RusotoFuture<ListFiltersResponse, ListFiltersError> {
        let request_uri = format!(
            "/detector/{detector_id}/filter",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListFiltersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFiltersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> RusotoFuture<ListFindingsResponse, ListFindingsError> {
        let request_uri = format!(
            "/detector/{detector_id}/findings",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListFindingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFindingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID.</p>
    fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> RusotoFuture<ListIPSetsResponse, ListIPSetsError> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListIPSetsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListIPSetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> RusotoFuture<ListInvitationsResponse, ListInvitationsError> {
        let request_uri = "/invitation";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListInvitationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListInvitationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists details about all member accounts for the current GuardDuty master account.</p>
    fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> RusotoFuture<ListMembersResponse, ListMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.only_associated {
            params.put("onlyAssociated", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListMembersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID.</p>
    fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> RusotoFuture<ListThreatIntelSetsResponse, ListThreatIntelSetsError> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListThreatIntelSetsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListThreatIntelSetsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Re-enables GuardDuty to monitor findings of the member accounts specified by the account IDs. A master GuardDuty account can run this command after disabling GuardDuty from monitoring these members&#39; findings by running StopMonitoringMembers.</p>
    fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> RusotoFuture<StartMonitoringMembersResponse, StartMonitoringMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member/start",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<StartMonitoringMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartMonitoringMembersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Disables GuardDuty from monitoring findings of the member accounts specified by the account IDs. After running this command, a master GuardDuty account can run StartMonitoringMembers to re-enable GuardDuty to monitor these members’ findings.</p>
    fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> RusotoFuture<StopMonitoringMembersResponse, StopMonitoringMembersError> {
        let request_uri = format!(
            "/detector/{detector_id}/member/stop",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<StopMonitoringMembersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopMonitoringMembersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Unarchives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> RusotoFuture<UnarchiveFindingsResponse, UnarchiveFindingsError> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/unarchive",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UnarchiveFindingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UnarchiveFindingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an Amazon GuardDuty detector specified by the detectorId.</p>
    fn update_detector(
        &self,
        input: UpdateDetectorRequest,
    ) -> RusotoFuture<UpdateDetectorResponse, UpdateDetectorError> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateDetectorResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDetectorError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the filter specified by the filter name.</p>
    fn update_filter(
        &self,
        input: UpdateFilterRequest,
    ) -> RusotoFuture<UpdateFilterResponse, UpdateFilterError> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateFilterResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateFilterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Marks specified Amazon GuardDuty findings as useful or not useful.</p>
    fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> RusotoFuture<UpdateFindingsFeedbackResponse, UpdateFindingsFeedbackError> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/feedback",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateFindingsFeedbackResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateFindingsFeedbackError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> RusotoFuture<UpdateIPSetResponse, UpdateIPSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateIPSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the ThreatIntelSet specified by ThreatIntelSet ID.</p>
    fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> RusotoFuture<UpdateThreatIntelSetResponse, UpdateThreatIntelSetError> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateThreatIntelSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateThreatIntelSetError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
