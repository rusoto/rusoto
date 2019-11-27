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
use rusoto_core::{Client, HttpDispatchError, RusotoError, RusotoFuture};

use futures::{FutureExt, TryFutureExt};
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcceptInvitationRequest {
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>This value is used to validate the master account to the member account.</p>
    #[serde(rename = "InvitationId")]
    pub invitation_id: String,
    /// <p>The account ID of the master GuardDuty account whose invitation you're accepting.</p>
    #[serde(rename = "MasterId")]
    pub master_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AcceptInvitationResponse {}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AccountDetail {
    /// <p>Member account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>Member account's email address.</p>
    #[serde(rename = "Email")]
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Action {
    /// <p>GuardDuty Finding activity type.</p>
    #[serde(rename = "ActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>Information about the AWS_API_CALL action described in this finding.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct City {
    /// <p>City name of the remote IP address.</p>
    #[serde(rename = "CityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "Equals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<Vec<String>>,
    /// <p>Represents a greater than condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "GreaterThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<i64>,
    /// <p>Represents a greater than equal condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "GreaterThanOrEqual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal: Option<i64>,
    /// <p>Represents a less than condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "LessThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<i64>,
    /// <p>Represents a less than equal condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "LessThanOrEqual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal: Option<i64>,
    #[serde(rename = "NotEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<Vec<String>>,
}

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
    /// <p>The unique ID of the detector of the GuardDuty account for which you want to create a filter.</p>
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
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIPSetRequest {
    /// <p>A boolean value that indicates whether GuardDuty is to start using the uploaded IPSet.</p>
    #[serde(rename = "Activate")]
    pub activate: bool,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The unique ID of the detector of the GuardDuty account for which you want to create an IPSet.</p>
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
    /// <p>The ID of the IPSet resource.</p>
    #[serde(rename = "IpSetId")]
    pub ip_set_id: String,
}

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
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateThreatIntelSetRequest {
    /// <p>A boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    #[serde(rename = "Activate")]
    pub activate: bool,
    /// <p>The idempotency token for the create request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The unique ID of the detector of the GuardDuty account for which you want to create a threatIntelSet.</p>
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
    /// <p>The ID of the ThreatIntelSet resource.</p>
    #[serde(rename = "ThreatIntelSetId")]
    pub threat_intel_set_id: String,
}

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
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDetectorRequest {
    /// <p>The unique ID of the detector that you want to delete.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDetectorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFilterRequest {
    /// <p>The unique ID of the detector the filter is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The name of the filter you want to delete.</p>
    #[serde(rename = "FilterName")]
    pub filter_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteFilterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIPSetRequest {
    /// <p>The unique ID of the detector the ipSet is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the ipSet you want to delete.</p>
    #[serde(rename = "IpSetId")]
    pub ip_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteIPSetResponse {}

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
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

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
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteThreatIntelSetRequest {
    /// <p>The unique ID of the detector the threatIntelSet is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the threatIntelSet you want to delete.</p>
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
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DnsRequestAction {
    /// <p>Domain information for the DNS request.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainDetails {
    /// <p>Domain information for the AWS API call.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

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
    /// <p>Findings' schema version.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindingCriteria {
    /// <p>Represents a map of finding properties that match specified conditions and values when querying findings.</p>
    #[serde(rename = "Criterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion: Option<::std::collections::HashMap<String, Condition>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FindingStatistics {
    /// <p>Represents a map of severity to count statistic for a set of findings</p>
    #[serde(rename = "CountBySeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_by_severity: Option<::std::collections::HashMap<String, i64>>,
}

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
    /// <p>The unique ID of the detector that you want to get.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDetectorResponse {
    /// <p>Detector creation timestamp.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>Finding publishing frequency.</p>
    #[serde(rename = "FindingPublishingFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    /// <p>The GuardDuty service role.</p>
    #[serde(rename = "ServiceRole")]
    pub service_role: String,
    /// <p>The detector status.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>Detector last update timestamp.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFilterRequest {
    /// <p>The unique ID of the detector the filter is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The name of the filter you want to get.</p>
    #[serde(rename = "FilterName")]
    pub filter_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFilterResponse {
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The description of the filter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    /// <p>A list of findings.</p>
    #[serde(rename = "Findings")]
    pub findings: Vec<Finding>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFindingsStatisticsRequest {
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings' statistics you want to retrieve.</p>
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
    pub finding_statistics: FindingStatistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIPSetRequest {
    /// <p>The unique ID of the detector the ipSet is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the ipSet you want to get.</p>
    #[serde(rename = "IpSetId")]
    pub ip_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIPSetResponse {
    /// <p>The format of the file that contains the IPSet.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>The URI of the file that contains the IPSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key)</p>
    #[serde(rename = "Location")]
    pub location: String,
    /// <p>The user friendly name to identify the IPSet. This name is displayed in all findings that are triggered by activity that involves IP addresses included in this IPSet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The status of ipSet file uploaded.</p>
    #[serde(rename = "Status")]
    pub status: String,
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
    /// <p>Master account details.</p>
    #[serde(rename = "Master")]
    pub master: Master,
}

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
    /// <p>A list of members.</p>
    #[serde(rename = "Members")]
    pub members: Vec<Member>,
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetThreatIntelSetRequest {
    /// <p>The unique ID of the detector the threatIntelSet is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>The unique ID of the threatIntelSet you want to get.</p>
    #[serde(rename = "ThreatIntelSetId")]
    pub threat_intel_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetThreatIntelSetResponse {
    /// <p>The format of the threatIntelSet.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>The URI of the file that contains the ThreatIntelSet. For example (https://s3.us-west-2.amazonaws.com/my-bucket/my-object-key).</p>
    #[serde(rename = "Location")]
    pub location: String,
    /// <p>A user-friendly ThreatIntelSet name that is displayed in all finding generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The status of threatIntelSet file uploaded.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceDetails {
    /// <p>The availability zone of the EC2 instance.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The profile information of the EC2 instance.</p>
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
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDetectorsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDetectorsResponse {
    /// <p>A list of detector Ids.</p>
    #[serde(rename = "DetectorIds")]
    pub detector_ids: Vec<String>,
    /// <p>Pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFiltersRequest {
    /// <p>The unique ID of the detector the filter is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFiltersResponse {
    /// <p>A list of filter names</p>
    #[serde(rename = "FilterNames")]
    pub filter_names: Vec<String>,
    /// <p>Pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

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
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
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
    /// <p>The IDs of the findings you are listing.</p>
    #[serde(rename = "FindingIds")]
    pub finding_ids: Vec<String>,
    /// <p>Pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIPSetsRequest {
    /// <p>The unique ID of the detector the ipSet is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIPSetsResponse {
    /// <p>The IDs of the IPSet resources.</p>
    #[serde(rename = "IpSetIds")]
    pub ip_set_ids: Vec<String>,
    /// <p>Pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListInvitationsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListInvitationsResponse {
    /// <p>A list of invitation descriptions.</p>
    #[serde(rename = "Invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    /// <p>Pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListMembersRequest {
    /// <p>The unique ID of the detector the member is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies whether to only return associated members or to return all members (including members which haven't been invited yet or have been disassociated).</p>
    #[serde(rename = "OnlyAssociated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListMembersResponse {
    /// <p>A list of members.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>Pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListThreatIntelSetsRequest {
    /// <p>The unique ID of the detector the threatIntelSet is associated with.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 50. The maximum value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the list action. For subsequent calls to the action fill nextToken in the request with the value of NextToken from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListThreatIntelSetsResponse {
    /// <p>Pagination parameter to be used on the next list operation to retrieve more items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The IDs of the ThreatIntelSet resources.</p>
    #[serde(rename = "ThreatIntelSetIds")]
    pub threat_intel_set_ids: Vec<String>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Member {
    /// <p>Member account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>Member account's detector ID.</p>
    #[serde(rename = "DetectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>Member account's email address.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>Timestamp at which the invitation was sent</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    /// <p>Master account ID.</p>
    #[serde(rename = "MasterId")]
    pub master_id: String,
    /// <p>The status of the relationship between the member and the master.</p>
    #[serde(rename = "RelationshipStatus")]
    pub relationship_status: String,
    /// <p>Member last updated timestamp.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
}

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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Resource {
    /// <p>The IAM access key details (IAM user information) of a user that engaged in the activity that prompted GuardDuty to generate a finding.</p>
    #[serde(rename = "AccessKeyDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_details: Option<AccessKeyDetails>,
    /// <p>The information about the EC2 instance associated with the activity that prompted GuardDuty to generate a finding.</p>
    #[serde(rename = "InstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<InstanceDetails>,
    /// <p>The type of the AWS resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SecurityGroup {
    /// <p>EC2 instance's security group ID.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>EC2 instance's security group name.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartMonitoringMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts whose findings you want the master account to monitor.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account whom you want to re-enable to monitor members' findings.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartMonitoringMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopMonitoringMembersRequest {
    /// <p>A list of account IDs of the GuardDuty member accounts whose findings you want the master account to stop monitoring.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>The unique ID of the detector of the GuardDuty account that you want to stop from monitor members' findings.</p>
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopMonitoringMembersResponse {
    /// <p>A list of objects containing the unprocessed account and a result string explaining why it was unprocessed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    pub unprocessed_accounts: Vec<UnprocessedAccount>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnprocessedAccount {
    /// <p>AWS Account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>A reason why the account hasn't been processed.</p>
    #[serde(rename = "Result")]
    pub result: String,
}

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
    pub name: String,
}

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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
    /// <p>Bad request exception object.</p>
    BadRequest(String),
    /// <p>Internal server error exception object.</p>
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
#[async_trait]
pub trait GuardDuty {
    /// <p>Accepts the invitation to be monitored by a master GuardDuty account.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>>;

    /// <p>Archives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    async fn archive_findings(
        &self,
        input: ArchiveFindingsRequest,
    ) -> Result<ArchiveFindingsResponse, RusotoError<ArchiveFindingsError>>;

    /// <p>Creates a single Amazon GuardDuty detector. A detector is an object that represents the GuardDuty service. A detector must be created in order for GuardDuty to become operational.</p>
    async fn create_detector(
        &self,
        input: CreateDetectorRequest,
    ) -> Result<CreateDetectorResponse, RusotoError<CreateDetectorError>>;

    /// <p>Creates a filter using the specified finding criteria.</p>
    async fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> Result<CreateFilterResponse, RusotoError<CreateFilterError>>;

    /// <p>Creates a new IPSet - a list of trusted IP addresses that have been whitelisted for secure communication with AWS infrastructure and applications.</p>
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>>;

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. The current AWS account can then invite these members to manage GuardDuty in their accounts.</p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>>;

    /// <p>Generates example findings of types specified by the list of finding types. If 'NULL' is specified for findingTypes, the API generates example findings of all supported finding types.</p>
    async fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> Result<CreateSampleFindingsResponse, RusotoError<CreateSampleFindingsError>>;

    /// <p>Create a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets.</p>
    async fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> Result<CreateThreatIntelSetResponse, RusotoError<CreateThreatIntelSetError>>;

    /// <p>Declines invitations sent to the current member account by AWS account specified by their account IDs.</p>
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>>;

    /// <p>Deletes a Amazon GuardDuty detector specified by the detector ID.</p>
    async fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> Result<DeleteDetectorResponse, RusotoError<DeleteDetectorError>>;

    /// <p>Deletes the filter specified by the filter name.</p>
    async fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> Result<DeleteFilterResponse, RusotoError<DeleteFilterError>>;

    /// <p>Deletes the IPSet specified by the IPSet ID.</p>
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>>;

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>>;

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>>;

    /// <p>Deletes ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    async fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> Result<DeleteThreatIntelSetResponse, RusotoError<DeleteThreatIntelSetError>>;

    /// <p>Disassociates the current GuardDuty member account from its master account.</p>
    async fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    >;

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>>;

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    async fn get_detector(
        &self,
        input: GetDetectorRequest,
    ) -> Result<GetDetectorResponse, RusotoError<GetDetectorError>>;

    /// <p>Returns the details of the filter specified by the filter name.</p>
    async fn get_filter(
        &self,
        input: GetFilterRequest,
    ) -> Result<GetFilterResponse, RusotoError<GetFilterError>>;

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>>;

    /// <p>Lists Amazon GuardDuty findings' statistics for the specified detector ID.</p>
    async fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> Result<GetFindingsStatisticsResponse, RusotoError<GetFindingsStatisticsError>>;

    /// <p>Retrieves the IPSet specified by the IPSet ID.</p>
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>>;

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>>;

    /// <p>Provides the details for the GuardDuty master account to the current GuardDuty member account.</p>
    async fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>>;

    /// <p>Retrieves GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>>;

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    async fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> Result<GetThreatIntelSetResponse, RusotoError<GetThreatIntelSetError>>;

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty and allow the current AWS account to view and manage these accounts' GuardDuty findings on their behalf as the master account.</p>
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>>;

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>>;

    /// <p>Returns a paginated list of the current filters.</p>
    async fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> Result<ListFiltersResponse, RusotoError<ListFiltersError>>;

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    async fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> Result<ListFindingsResponse, RusotoError<ListFindingsError>>;

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID.</p>
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>>;

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>>;

    /// <p>Lists details about all member accounts for the current GuardDuty master account.</p>
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>>;

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID.</p>
    async fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> Result<ListThreatIntelSetsResponse, RusotoError<ListThreatIntelSetsError>>;

    /// <p>Re-enables GuardDuty to monitor findings of the member accounts specified by the account IDs. A master GuardDuty account can run this command after disabling GuardDuty from monitoring these members' findings by running StopMonitoringMembers.</p>
    async fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> Result<StartMonitoringMembersResponse, RusotoError<StartMonitoringMembersError>>;

    /// <p>Disables GuardDuty from monitoring findings of the member accounts specified by the account IDs. After running this command, a master GuardDuty account can run StartMonitoringMembers to re-enable GuardDuty to monitor these members’ findings.</p>
    async fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> Result<StopMonitoringMembersResponse, RusotoError<StopMonitoringMembersError>>;

    /// <p>Unarchives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    async fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> Result<UnarchiveFindingsResponse, RusotoError<UnarchiveFindingsError>>;

    /// <p>Updates an Amazon GuardDuty detector specified by the detectorId.</p>
    async fn update_detector(
        &self,
        input: UpdateDetectorRequest,
    ) -> Result<UpdateDetectorResponse, RusotoError<UpdateDetectorError>>;

    /// <p>Updates the filter specified by the filter name.</p>
    async fn update_filter(
        &self,
        input: UpdateFilterRequest,
    ) -> Result<UpdateFilterResponse, RusotoError<UpdateFilterError>>;

    /// <p>Marks specified Amazon GuardDuty findings as useful or not useful.</p>
    async fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> Result<UpdateFindingsFeedbackResponse, RusotoError<UpdateFindingsFeedbackError>>;

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>>;

    /// <p>Updates the ThreatIntelSet specified by ThreatIntelSet ID.</p>
    async fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> Result<UpdateThreatIntelSetResponse, RusotoError<UpdateThreatIntelSetError>>;
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
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        GuardDutyClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

#[async_trait]
impl GuardDuty for GuardDutyClient {
    /// <p>Accepts the invitation to be monitored by a master GuardDuty account.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(AcceptInvitationError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AcceptInvitationResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptInvitationError::from_response(response))
        }
    }

    /// <p>Archives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    async fn archive_findings(
        &self,
        input: ArchiveFindingsRequest,
    ) -> Result<ArchiveFindingsResponse, RusotoError<ArchiveFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/archive",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ArchiveFindingsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ArchiveFindingsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ArchiveFindingsError::from_response(response))
        }
    }

    /// <p>Creates a single Amazon GuardDuty detector. A detector is an object that represents the GuardDuty service. A detector must be created in order for GuardDuty to become operational.</p>
    async fn create_detector(
        &self,
        input: CreateDetectorRequest,
    ) -> Result<CreateDetectorResponse, RusotoError<CreateDetectorError>> {
        let request_uri = "/detector";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(CreateDetectorError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDetectorResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDetectorError::from_response(response))
        }
    }

    /// <p>Creates a filter using the specified finding criteria.</p>
    async fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> Result<CreateFilterResponse, RusotoError<CreateFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(CreateFilterError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFilterResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFilterError::from_response(response))
        }
    }

    /// <p>Creates a new IPSet - a list of trusted IP addresses that have been whitelisted for secure communication with AWS infrastructure and applications.</p>
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(CreateIPSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIPSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIPSetError::from_response(response))
        }
    }

    /// <p>Creates member accounts of the current AWS account by specifying a list of AWS account IDs. The current AWS account can then invite these members to manage GuardDuty in their accounts.</p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(CreateMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMembersError::from_response(response))
        }
    }

    /// <p>Generates example findings of types specified by the list of finding types. If 'NULL' is specified for findingTypes, the API generates example findings of all supported finding types.</p>
    async fn create_sample_findings(
        &self,
        input: CreateSampleFindingsRequest,
    ) -> Result<CreateSampleFindingsResponse, RusotoError<CreateSampleFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/create",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(CreateSampleFindingsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSampleFindingsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSampleFindingsError::from_response(response))
        }
    }

    /// <p>Create a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets.</p>
    async fn create_threat_intel_set(
        &self,
        input: CreateThreatIntelSetRequest,
    ) -> Result<CreateThreatIntelSetResponse, RusotoError<CreateThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(CreateThreatIntelSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateThreatIntelSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateThreatIntelSetError::from_response(response))
        }
    }

    /// <p>Declines invitations sent to the current member account by AWS account specified by their account IDs.</p>
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>> {
        let request_uri = "/invitation/decline";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DeclineInvitationsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeclineInvitationsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeclineInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes a Amazon GuardDuty detector specified by the detector ID.</p>
    async fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> Result<DeleteDetectorResponse, RusotoError<DeleteDetectorError>> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteDetectorError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDetectorResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDetectorError::from_response(response))
        }
    }

    /// <p>Deletes the filter specified by the filter name.</p>
    async fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> Result<DeleteFilterResponse, RusotoError<DeleteFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteFilterError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFilterResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFilterError::from_response(response))
        }
    }

    /// <p>Deletes the IPSet specified by the IPSet ID.</p>
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteIPSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteIPSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIPSetError::from_response(response))
        }
    }

    /// <p>Deletes invitations sent to the current member account by AWS accounts specified by their account IDs.</p>
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>> {
        let request_uri = "/invitation/delete";

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteInvitationsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInvitationsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/delete",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMembersError::from_response(response))
        }
    }

    /// <p>Deletes ThreatIntelSet specified by the ThreatIntelSet ID.</p>
    async fn delete_threat_intel_set(
        &self,
        input: DeleteThreatIntelSetRequest,
    ) -> Result<DeleteThreatIntelSetResponse, RusotoError<DeleteThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("DELETE", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteThreatIntelSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteThreatIntelSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteThreatIntelSetError::from_response(response))
        }
    }

    /// <p>Disassociates the current GuardDuty member account from its master account.</p>
    async fn disassociate_from_master_account(
        &self,
        input: DisassociateFromMasterAccountRequest,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    > {
        let request_uri = format!(
            "/detector/{detector_id}/master/disassociate",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DisassociateFromMasterAccountError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateFromMasterAccountResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateFromMasterAccountError::from_response(response))
        }
    }

    /// <p>Disassociates GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/disassociate",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(DisassociateMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateMembersError::from_response(response))
        }
    }

    /// <p>Retrieves an Amazon GuardDuty detector specified by the detectorId.</p>
    async fn get_detector(
        &self,
        input: GetDetectorRequest,
    ) -> Result<GetDetectorResponse, RusotoError<GetDetectorError>> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetDetectorError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDetectorResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDetectorError::from_response(response))
        }
    }

    /// <p>Returns the details of the filter specified by the filter name.</p>
    async fn get_filter(
        &self,
        input: GetFilterRequest,
    ) -> Result<GetFilterResponse, RusotoError<GetFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetFilterError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetFilterResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFilterError::from_response(response))
        }
    }

    /// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/get",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetFindingsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFindingsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFindingsError::from_response(response))
        }
    }

    /// <p>Lists Amazon GuardDuty findings' statistics for the specified detector ID.</p>
    async fn get_findings_statistics(
        &self,
        input: GetFindingsStatisticsRequest,
    ) -> Result<GetFindingsStatisticsResponse, RusotoError<GetFindingsStatisticsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/statistics",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetFindingsStatisticsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFindingsStatisticsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFindingsStatisticsError::from_response(response))
        }
    }

    /// <p>Retrieves the IPSet specified by the IPSet ID.</p>
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetIPSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetIPSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIPSetError::from_response(response))
        }
    }

    /// <p>Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation.</p>
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>> {
        let request_uri = "/invitation/count";

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetInvitationsCountError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInvitationsCountResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInvitationsCountError::from_response(response))
        }
    }

    /// <p>Provides the details for the GuardDuty master account to the current GuardDuty member account.</p>
    async fn get_master_account(
        &self,
        input: GetMasterAccountRequest,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>> {
        let request_uri = format!(
            "/detector/{detector_id}/master",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetMasterAccountError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMasterAccountResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMasterAccountError::from_response(response))
        }
    }

    /// <p>Retrieves GuardDuty member accounts (to the current GuardDuty master account) specified by the account IDs.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/get",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMembersError::from_response(response))
        }
    }

    /// <p>Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID.</p>
    async fn get_threat_intel_set(
        &self,
        input: GetThreatIntelSetRequest,
    ) -> Result<GetThreatIntelSetResponse, RusotoError<GetThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("GET", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(GetThreatIntelSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetThreatIntelSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetThreatIntelSetError::from_response(response))
        }
    }

    /// <p>Invites other AWS accounts (created as members of the current AWS account by CreateMembers) to enable GuardDuty and allow the current AWS account to view and manage these accounts' GuardDuty findings on their behalf as the master account.</p>
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/invite",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(InviteMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InviteMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InviteMembersError::from_response(response))
        }
    }

    /// <p>Lists detectorIds of all the existing Amazon GuardDuty detector resources.</p>
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>> {
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

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ListDetectorsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDetectorsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDetectorsError::from_response(response))
        }
    }

    /// <p>Returns a paginated list of the current filters.</p>
    async fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> Result<ListFiltersResponse, RusotoError<ListFiltersError>> {
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

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ListFiltersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFiltersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFiltersError::from_response(response))
        }
    }

    /// <p>Lists Amazon GuardDuty findings for the specified detector ID.</p>
    async fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> Result<ListFindingsResponse, RusotoError<ListFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ListFindingsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFindingsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFindingsError::from_response(response))
        }
    }

    /// <p>Lists the IPSets of the GuardDuty service specified by the detector ID.</p>
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>> {
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

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ListIPSetsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListIPSetsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIPSetsError::from_response(response))
        }
    }

    /// <p>Lists all GuardDuty membership invitations that were sent to the current AWS account.</p>
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>> {
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

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ListInvitationsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInvitationsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInvitationsError::from_response(response))
        }
    }

    /// <p>Lists details about all member accounts for the current GuardDuty master account.</p>
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>> {
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

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ListMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMembersError::from_response(response))
        }
    }

    /// <p>Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID.</p>
    async fn list_threat_intel_sets(
        &self,
        input: ListThreatIntelSetsRequest,
    ) -> Result<ListThreatIntelSetsResponse, RusotoError<ListThreatIntelSetsError>> {
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

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(ListThreatIntelSetsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListThreatIntelSetsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListThreatIntelSetsError::from_response(response))
        }
    }

    /// <p>Re-enables GuardDuty to monitor findings of the member accounts specified by the account IDs. A master GuardDuty account can run this command after disabling GuardDuty from monitoring these members' findings by running StopMonitoringMembers.</p>
    async fn start_monitoring_members(
        &self,
        input: StartMonitoringMembersRequest,
    ) -> Result<StartMonitoringMembersResponse, RusotoError<StartMonitoringMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/start",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(StartMonitoringMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartMonitoringMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartMonitoringMembersError::from_response(response))
        }
    }

    /// <p>Disables GuardDuty from monitoring findings of the member accounts specified by the account IDs. After running this command, a master GuardDuty account can run StartMonitoringMembers to re-enable GuardDuty to monitor these members’ findings.</p>
    async fn stop_monitoring_members(
        &self,
        input: StopMonitoringMembersRequest,
    ) -> Result<StopMonitoringMembersResponse, RusotoError<StopMonitoringMembersError>> {
        let request_uri = format!(
            "/detector/{detector_id}/member/stop",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(StopMonitoringMembersError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopMonitoringMembersResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopMonitoringMembersError::from_response(response))
        }
    }

    /// <p>Unarchives Amazon GuardDuty findings specified by the list of finding IDs.</p>
    async fn unarchive_findings(
        &self,
        input: UnarchiveFindingsRequest,
    ) -> Result<UnarchiveFindingsResponse, RusotoError<UnarchiveFindingsError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/unarchive",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(UnarchiveFindingsError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UnarchiveFindingsResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UnarchiveFindingsError::from_response(response))
        }
    }

    /// <p>Updates an Amazon GuardDuty detector specified by the detectorId.</p>
    async fn update_detector(
        &self,
        input: UpdateDetectorRequest,
    ) -> Result<UpdateDetectorResponse, RusotoError<UpdateDetectorError>> {
        let request_uri = format!("/detector/{detector_id}", detector_id = input.detector_id);

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateDetectorError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDetectorResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDetectorError::from_response(response))
        }
    }

    /// <p>Updates the filter specified by the filter name.</p>
    async fn update_filter(
        &self,
        input: UpdateFilterRequest,
    ) -> Result<UpdateFilterResponse, RusotoError<UpdateFilterError>> {
        let request_uri = format!(
            "/detector/{detector_id}/filter/{filter_name}",
            detector_id = input.detector_id,
            filter_name = input.filter_name
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateFilterError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFilterResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFilterError::from_response(response))
        }
    }

    /// <p>Marks specified Amazon GuardDuty findings as useful or not useful.</p>
    async fn update_findings_feedback(
        &self,
        input: UpdateFindingsFeedbackRequest,
    ) -> Result<UpdateFindingsFeedbackResponse, RusotoError<UpdateFindingsFeedbackError>> {
        let request_uri = format!(
            "/detector/{detector_id}/findings/feedback",
            detector_id = input.detector_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateFindingsFeedbackError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFindingsFeedbackResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFindingsFeedbackError::from_response(response))
        }
    }

    /// <p>Updates the IPSet specified by the IPSet ID.</p>
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/ipset/{ip_set_id}",
            detector_id = input.detector_id,
            ip_set_id = input.ip_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateIPSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateIPSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIPSetError::from_response(response))
        }
    }

    /// <p>Updates the ThreatIntelSet specified by ThreatIntelSet ID.</p>
    async fn update_threat_intel_set(
        &self,
        input: UpdateThreatIntelSetRequest,
    ) -> Result<UpdateThreatIntelSetResponse, RusotoError<UpdateThreatIntelSetError>> {
        let request_uri = format!(
            "/detector/{detector_id}/threatintelset/{threat_intel_set_id}",
            detector_id = input.detector_id,
            threat_intel_set_id = input.threat_intel_set_id
        );

        let mut request = SignedRequest::new("POST", "guardduty", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateThreatIntelSetError::SignAndDispatch)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateThreatIntelSetResponse, _>();

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateThreatIntelSetError::from_response(response))
        }
    }
}
