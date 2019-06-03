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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
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
}

impl AcceptInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AcceptInvitationError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AcceptInvitationError::InternalServerError(
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
}

impl ArchiveFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ArchiveFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ArchiveFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ArchiveFindingsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateSampleFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSampleFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSampleFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateSampleFindingsError::InternalServerError(
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
}

impl CreateThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateThreatIntelSetError::InternalServerError(
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
}

impl DeclineInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeclineInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeclineInvitationsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeclineInvitationsError::InternalServerError(
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
}

impl DeleteDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteInvitationsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteInvitationsError::InternalServerError(
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
}

impl DeleteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteThreatIntelSetError::InternalServerError(
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
}

impl DisassociateFromMasterAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateFromMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DisassociateFromMasterAccountError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DisassociateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociateMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DisassociateMembersError::InternalServerError(
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
}

impl GetDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetFindingsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetFindingsStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFindingsStatisticsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetFindingsStatisticsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetFindingsStatisticsError::InternalServerError(
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
}

impl GetIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetInvitationsCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInvitationsCountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetInvitationsCountError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetInvitationsCountError::InternalServerError(
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
}

impl GetMasterAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMasterAccountError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetMasterAccountError::InternalServerError(
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
}

impl GetMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetThreatIntelSetError::InternalServerError(
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
}

impl InviteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InviteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(InviteMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(InviteMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListDetectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDetectorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDetectorsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListDetectorsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListFiltersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFiltersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListFiltersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListFiltersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListFindingsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListIPSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIPSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListIPSetsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListIPSetsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListInvitationsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListInvitationsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListMembersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListThreatIntelSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListThreatIntelSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListThreatIntelSetsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListThreatIntelSetsError::InternalServerError(
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
}

impl StartMonitoringMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMonitoringMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartMonitoringMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StartMonitoringMembersError::InternalServerError(
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
}

impl StopMonitoringMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopMonitoringMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopMonitoringMembersError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StopMonitoringMembersError::InternalServerError(
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
}

impl UnarchiveFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnarchiveFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UnarchiveFindingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UnarchiveFindingsError::InternalServerError(
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
}

impl UpdateDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDetectorError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateDetectorError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdateFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFilterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFilterError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFilterError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdateFindingsFeedbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFindingsFeedbackError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFindingsFeedbackError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFindingsFeedbackError::InternalServerError(
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
}

impl UpdateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIPSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateIPSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateIPSetError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdateThreatIntelSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThreatIntelSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateThreatIntelSetError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateThreatIntelSetError::InternalServerError(
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
        }
    }
}
/// Trait representing the capabilities of the Amazon GuardDuty API. Amazon GuardDuty clients implement this trait.
pub trait GuardDuty {
    /// <p>Accepts the invitation to be monitored by a master GuardDuty account.</p>
    fn accept_invitation(&self, input: AcceptInvitationRequest)
        -> Request<AcceptInvitationRequest>;

    /// <p>Archives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn archive_findings(&self, input: ArchiveFindingsRequest) -> Request<ArchiveFindingsRequest>;

    /// <p>Creates a single Amazon GuardDuty detector. A detector is an object that represents the GuardDuty service. A detector must be created in order for GuardDuty to become operational.</p>
    fn create_detector(&self, input: CreateDetectorRequest) -> Request<CreateDetectorRequest>;

    /// <p>Creates a filter using the specified finding criteria.</p>
    fn create_filter(&self, input: CreateFilterRequest) -> Request<CreateFilterRequest>;

    /// <p>Creates a new IPSet - a list of trusted IP addresses that have been whitelisted for secure communication with AWS infrastructure and applications.</p>
    fn create_ip_set(&self, input: CreateIPSetRequest) -> Request<CreateIPSetRequest>;

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. The current AWS account can then invite these members to manage GuardDuty in their accounts.</p>
    fn create_members(&self, input: CreateMembersRequest) -> Request<CreateMembersRequest>;

    /// <p>Generates example findings of types specified by the list of finding types. If &#39;NULL&#39; is specified for findingTypes, the API generates example findings of all supported finding types.</p>
    fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> Request<CreateSampleFindingsRequest>;

    /// <p>Create a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets.</p>
    fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> Request<CreateThreatIntelSetRequest>;

    /// <p>Declines invitations sent to the current member account by AWS account specified by their account IDs.</p>
    fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Request<DeclineInvitationsRequest>;

    /// <p>Deletes a Amazon GuardDuty detector specified by the detector ID.</p>
    fn delete_detector(&self, input: DeleteDetectorRequest) -> Request<DeleteDetectorRequest>;

    /// <p>Deletes the filter specified by the filter name.</p>
    fn delete_filter(&self, input: DeleteFilterRequest) -> Request<DeleteFilterRequest>;

    /// <p>Deletes the IPSet specified by the IPSet ID.</p>
    fn delete_ip_set(&self, input: DeleteIPSetRequest) -> Request<DeleteIPSetRequest>;

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Request<DeleteInvitationsRequest>;

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn delete_members(&self, input: DeleteMembersRequest) -> Request<DeleteMembersRequest>;

    /// <p>Deletes ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> Request<DeleteThreatIntelSetRequest>;

    /// <p>Disassociates the current GuardDuty member account from its master account.</p>
    fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> Request<DisassociateFromMasterAccountRequest>;

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Request<DisassociateMembersRequest>;

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    fn get_detector(&self, input: GetDetectorRequest) -> Request<GetDetectorRequest>;

    /// <p>Returns the details of the filter specified by the filter name.</p>
    fn get_filter(&self, input: GetFilterRequest) -> Request<GetFilterRequest>;

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    fn get_findings(&self, input: GetFindingsRequest) -> Request<GetFindingsRequest>;

    /// <p>Lists Amazon GuardDuty findings&#39; statistics for the specified detector ID.</p>
    fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> Request<GetFindingsStatisticsRequest>;

    /// <p>Retrieves the IPSet specified by the IPSet ID.</p>
    fn get_ip_set(&self, input: GetIPSetRequest) -> Request<GetIPSetRequest>;

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    fn get_invitations_count(&self) -> Request<GetInvitationsCountRequest>;

    /// <p>Provides the details for the GuardDuty master account to the current GuardDuty member account.</p>
    fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> Request<GetMasterAccountRequest>;

    /// <p>Retrieves GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn get_members(&self, input: GetMembersRequest) -> Request<GetMembersRequest>;

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> Request<GetThreatIntelSetRequest>;

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty and allow the current AWS account to view and manage these accounts&#39; GuardDuty findings on their behalf as the master account.</p>
    fn invite_members(&self, input: InviteMembersRequest) -> Request<InviteMembersRequest>;

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    fn list_detectors(&self, input: ListDetectorsRequest) -> Request<ListDetectorsRequest>;

    /// <p>Returns a paginated list of the current filters.</p>
    fn list_filters(&self, input: ListFiltersRequest) -> Request<ListFiltersRequest>;

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    fn list_findings(&self, input: ListFindingsRequest) -> Request<ListFindingsRequest>;

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID.</p>
    fn list_ip_sets(&self, input: ListIPSetsRequest) -> Request<ListIPSetsRequest>;

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    fn list_invitations(&self, input: ListInvitationsRequest) -> Request<ListInvitationsRequest>;

    /// <p>Lists details about all member accounts for the current GuardDuty master account.</p>
    fn list_members(&self, input: ListMembersRequest) -> Request<ListMembersRequest>;

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID.</p>
    fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> Request<ListThreatIntelSetsRequest>;

    /// <p>Re-enables GuardDuty to monitor findings of the member accounts specified by the account IDs. A master GuardDuty account can run this command after disabling GuardDuty from monitoring these members&#39; findings by running StopMonitoringMembers.</p>
    fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> Request<StartMonitoringMembersRequest>;

    /// <p>Disables GuardDuty from monitoring findings of the member accounts specified by the account IDs. After running this command, a master GuardDuty account can run StartMonitoringMembers to re-enable GuardDuty to monitor these members’ findings.</p>
    fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> Request<StopMonitoringMembersRequest>;

    /// <p>Unarchives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> Request<UnarchiveFindingsRequest>;

    /// <p>Updates an Amazon GuardDuty detector specified by the detectorId.</p>
    fn update_detector(&self, input: UpdateDetectorRequest) -> Request<UpdateDetectorRequest>;

    /// <p>Updates the filter specified by the filter name.</p>
    fn update_filter(&self, input: UpdateFilterRequest) -> Request<UpdateFilterRequest>;

    /// <p>Marks specified Amazon GuardDuty findings as useful or not useful.</p>
    fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> Request<UpdateFindingsFeedbackRequest>;

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    fn update_ip_set(&self, input: UpdateIPSetRequest) -> Request<UpdateIPSetRequest>;

    /// <p>Updates the ThreatIntelSet specified by ThreatIntelSet ID.</p>
    fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> Request<UpdateThreatIntelSetRequest>;
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
            region,
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
            region,
        }
    }
}

impl GuardDuty for GuardDutyClient {
    /// <p>Accepts the invitation to be monitored by a master GuardDuty account.</p>
    fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Request<AcceptInvitationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Archives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn archive_findings(&self, input: ArchiveFindingsRequest) -> Request<ArchiveFindingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a single Amazon GuardDuty detector. A detector is an object that represents the GuardDuty service. A detector must be created in order for GuardDuty to become operational.</p>
    fn create_detector(&self, input: CreateDetectorRequest) -> Request<CreateDetectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a filter using the specified finding criteria.</p>
    fn create_filter(&self, input: CreateFilterRequest) -> Request<CreateFilterRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a new IPSet - a list of trusted IP addresses that have been whitelisted for secure communication with AWS infrastructure and applications.</p>
    fn create_ip_set(&self, input: CreateIPSetRequest) -> Request<CreateIPSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. The current AWS account can then invite these members to manage GuardDuty in their accounts.</p>
    fn create_members(&self, input: CreateMembersRequest) -> Request<CreateMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Generates example findings of types specified by the list of finding types. If &#39;NULL&#39; is specified for findingTypes, the API generates example findings of all supported finding types.</p>
    fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> Request<CreateSampleFindingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Create a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets.</p>
    fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> Request<CreateThreatIntelSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Declines invitations sent to the current member account by AWS account specified by their account IDs.</p>
    fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Request<DeclineInvitationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a Amazon GuardDuty detector specified by the detector ID.</p>
    fn delete_detector(&self, input: DeleteDetectorRequest) -> Request<DeleteDetectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the filter specified by the filter name.</p>
    fn delete_filter(&self, input: DeleteFilterRequest) -> Request<DeleteFilterRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the IPSet specified by the IPSet ID.</p>
    fn delete_ip_set(&self, input: DeleteIPSetRequest) -> Request<DeleteIPSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Request<DeleteInvitationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn delete_members(&self, input: DeleteMembersRequest) -> Request<DeleteMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> Request<DeleteThreatIntelSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates the current GuardDuty member account from its master account.</p>
    fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> Request<DisassociateFromMasterAccountRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Request<DisassociateMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    fn get_detector(&self, input: GetDetectorRequest) -> Request<GetDetectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the details of the filter specified by the filter name.</p>
    fn get_filter(&self, input: GetFilterRequest) -> Request<GetFilterRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    fn get_findings(&self, input: GetFindingsRequest) -> Request<GetFindingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists Amazon GuardDuty findings&#39; statistics for the specified detector ID.</p>
    fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> Request<GetFindingsStatisticsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the IPSet specified by the IPSet ID.</p>
    fn get_ip_set(&self, input: GetIPSetRequest) -> Request<GetIPSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    fn get_invitations_count(&self) -> Request<GetInvitationsCountRequest> {
        Request::new(
            GetInvitationsCountRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>Provides the details for the GuardDuty master account to the current GuardDuty member account.</p>
    fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> Request<GetMasterAccountRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    fn get_members(&self, input: GetMembersRequest) -> Request<GetMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> Request<GetThreatIntelSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty and allow the current AWS account to view and manage these accounts&#39; GuardDuty findings on their behalf as the master account.</p>
    fn invite_members(&self, input: InviteMembersRequest) -> Request<InviteMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    fn list_detectors(&self, input: ListDetectorsRequest) -> Request<ListDetectorsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns a paginated list of the current filters.</p>
    fn list_filters(&self, input: ListFiltersRequest) -> Request<ListFiltersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    fn list_findings(&self, input: ListFindingsRequest) -> Request<ListFindingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID.</p>
    fn list_ip_sets(&self, input: ListIPSetsRequest) -> Request<ListIPSetsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    fn list_invitations(&self, input: ListInvitationsRequest) -> Request<ListInvitationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists details about all member accounts for the current GuardDuty master account.</p>
    fn list_members(&self, input: ListMembersRequest) -> Request<ListMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID.</p>
    fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> Request<ListThreatIntelSetsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Re-enables GuardDuty to monitor findings of the member accounts specified by the account IDs. A master GuardDuty account can run this command after disabling GuardDuty from monitoring these members&#39; findings by running StopMonitoringMembers.</p>
    fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> Request<StartMonitoringMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disables GuardDuty from monitoring findings of the member accounts specified by the account IDs. After running this command, a master GuardDuty account can run StartMonitoringMembers to re-enable GuardDuty to monitor these members’ findings.</p>
    fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> Request<StopMonitoringMembersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Unarchives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> Request<UnarchiveFindingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates an Amazon GuardDuty detector specified by the detectorId.</p>
    fn update_detector(&self, input: UpdateDetectorRequest) -> Request<UpdateDetectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the filter specified by the filter name.</p>
    fn update_filter(&self, input: UpdateFilterRequest) -> Request<UpdateFilterRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Marks specified Amazon GuardDuty findings as useful or not useful.</p>
    fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> Request<UpdateFindingsFeedbackRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    fn update_ip_set(&self, input: UpdateIPSetRequest) -> Request<UpdateIPSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the ThreatIntelSet specified by ThreatIntelSet ID.</p>
    fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> Request<UpdateThreatIntelSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for AcceptInvitationRequest {
    type Output = AcceptInvitationResponse;
    type Error = AcceptInvitationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AcceptInvitationResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ArchiveFindingsRequest {
    type Output = ArchiveFindingsResponse;
    type Error = ArchiveFindingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/archive",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ArchiveFindingsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for CreateDetectorRequest {
    type Output = CreateDetectorResponse;
    type Error = CreateDetectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/detector";

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDetectorResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for CreateFilterRequest {
    type Output = CreateFilterResponse;
    type Error = CreateFilterError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/filter",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateFilterResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for CreateIPSetRequest {
    type Output = CreateIPSetResponse;
    type Error = CreateIPSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateIPSetResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for CreateMembersRequest {
    type Output = CreateMembersResponse;
    type Error = CreateMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for CreateSampleFindingsRequest {
    type Output = CreateSampleFindingsResponse;
    type Error = CreateSampleFindingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/create",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSampleFindingsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for CreateThreatIntelSetRequest {
    type Output = CreateThreatIntelSetResponse;
    type Error = CreateThreatIntelSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateThreatIntelSetResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeclineInvitationsRequest {
    type Output = DeclineInvitationsResponse;
    type Error = DeclineInvitationsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/invitation/decline";

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeclineInvitationsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeleteDetectorRequest {
    type Output = DeleteDetectorResponse;
    type Error = DeleteDetectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/detector/{detector_id}", detector_id = self.detector_id);

        let mut request = SignedRequest::new("DELETE", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDetectorResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeleteFilterRequest {
    type Output = DeleteFilterResponse;
    type Error = DeleteFilterError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = self.detector_id,
            filter_name = self.filter_name
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteFilterResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeleteIPSetRequest {
    type Output = DeleteIPSetResponse;
    type Error = DeleteIPSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = self.detector_id,
            ip_set_id = self.ip_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteIPSetResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeleteInvitationsRequest {
    type Output = DeleteInvitationsResponse;
    type Error = DeleteInvitationsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/invitation/delete";

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteInvitationsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeleteMembersRequest {
    type Output = DeleteMembersResponse;
    type Error = DeleteMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member/delete",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeleteThreatIntelSetRequest {
    type Output = DeleteThreatIntelSetResponse;
    type Error = DeleteThreatIntelSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = self.detector_id,
            threat_intel_set_id = self.threat_intel_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteThreatIntelSetResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DisassociateFromMasterAccountRequest {
    type Output = DisassociateFromMasterAccountResponse;
    type Error = DisassociateFromMasterAccountError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/master/disassociate",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateFromMasterAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateFromMasterAccountError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociateMembersRequest {
    type Output = DisassociateMembersResponse;
    type Error = DisassociateMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member/disassociate",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetDetectorRequest {
    type Output = GetDetectorResponse;
    type Error = GetDetectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/detector/{detector_id}", detector_id = self.detector_id);

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDetectorResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetFilterRequest {
    type Output = GetFilterResponse;
    type Error = GetFilterError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = self.detector_id,
            filter_name = self.filter_name
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFilterResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetFindingsRequest {
    type Output = GetFindingsResponse;
    type Error = GetFindingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/get",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFindingsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetFindingsStatisticsRequest {
    type Output = GetFindingsStatisticsResponse;
    type Error = GetFindingsStatisticsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/statistics",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFindingsStatisticsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetIPSetRequest {
    type Output = GetIPSetResponse;
    type Error = GetIPSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = self.detector_id,
            ip_set_id = self.ip_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetIPSetResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetInvitationsCountRequest {
    type Output = GetInvitationsCountResponse;
    type Error = GetInvitationsCountError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/invitation/count";

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInvitationsCountResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetMasterAccountRequest {
    type Output = GetMasterAccountResponse;
    type Error = GetMasterAccountError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMasterAccountResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetMembersRequest {
    type Output = GetMembersResponse;
    type Error = GetMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member/get",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetThreatIntelSetRequest {
    type Output = GetThreatIntelSetResponse;
    type Error = GetThreatIntelSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = self.detector_id,
            threat_intel_set_id = self.threat_intel_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetThreatIntelSetResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for InviteMembersRequest {
    type Output = InviteMembersResponse;
    type Error = InviteMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member/invite",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<InviteMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListDetectorsRequest {
    type Output = ListDetectorsResponse;
    type Error = ListDetectorsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/detector";

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDetectorsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListFiltersRequest {
    type Output = ListFiltersResponse;
    type Error = ListFiltersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/filter",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFiltersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListFindingsRequest {
    type Output = ListFindingsResponse;
    type Error = ListFindingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/findings",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFindingsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListIPSetsRequest {
    type Output = ListIPSetsResponse;
    type Error = ListIPSetsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListIPSetsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListInvitationsRequest {
    type Output = ListInvitationsResponse;
    type Error = ListInvitationsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/invitation";

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListInvitationsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListMembersRequest {
    type Output = ListMembersResponse;
    type Error = ListMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = self.only_associated {
            params.put("onlyAssociated", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListThreatIntelSetsRequest {
    type Output = ListThreatIntelSetsResponse;
    type Error = ListThreatIntelSetsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListThreatIntelSetsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for StartMonitoringMembersRequest {
    type Output = StartMonitoringMembersResponse;
    type Error = StartMonitoringMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member/start",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartMonitoringMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for StopMonitoringMembersRequest {
    type Output = StopMonitoringMembersResponse;
    type Error = StopMonitoringMembersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/member/stop",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopMonitoringMembersResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UnarchiveFindingsRequest {
    type Output = UnarchiveFindingsResponse;
    type Error = UnarchiveFindingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/unarchive",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UnarchiveFindingsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UpdateDetectorRequest {
    type Output = UpdateDetectorResponse;
    type Error = UpdateDetectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/detector/{detector_id}", detector_id = self.detector_id);

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDetectorResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UpdateFilterRequest {
    type Output = UpdateFilterResponse;
    type Error = UpdateFilterError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = self.detector_id,
            filter_name = self.filter_name
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateFilterResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UpdateFindingsFeedbackRequest {
    type Output = UpdateFindingsFeedbackResponse;
    type Error = UpdateFindingsFeedbackError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/feedback",
            detector_id = self.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateFindingsFeedbackResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UpdateIPSetRequest {
    type Output = UpdateIPSetResponse;
    type Error = UpdateIPSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = self.detector_id,
            ip_set_id = self.ip_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateIPSetResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UpdateThreatIntelSetRequest {
    type Output = UpdateThreatIntelSetResponse;
    type Error = UpdateThreatIntelSetError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = self.detector_id,
            threat_intel_set_id = self.threat_intel_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateThreatIntelSetResponse, _>()?;

                    Ok(result)
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
