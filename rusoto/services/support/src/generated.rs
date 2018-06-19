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
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddAttachmentsToSetRequest {
    /// <p>The ID of the attachment set. If an <code>attachmentSetId</code> is not specified, a new attachment set is created, and the ID of the set is returned in the response. If an <code>attachmentSetId</code> is specified, the attachments are added to the specified set, if it exists.</p>
    #[serde(rename = "attachmentSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    /// <p>One or more attachments to add to the set. The limit is 3 attachments per set, and the size limit is 5 MB per attachment.</p>
    #[serde(rename = "attachments")]
    pub attachments: Vec<Attachment>,
}

/// <p>The ID and expiry time of the attachment set returned by the <a>AddAttachmentsToSet</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddAttachmentsToSetResponse {
    /// <p>The ID of the attachment set. If an <code>attachmentSetId</code> was not specified, a new attachment set is created, and the ID of the set is returned in the response. If an <code>attachmentSetId</code> was specified, the attachments are added to the specified set, if it exists.</p>
    #[serde(rename = "attachmentSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    /// <p>The time and date when the attachment set expires.</p>
    #[serde(rename = "expiryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
}

/// <p>To be written.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddCommunicationToCaseRequest {
    /// <p>The ID of a set of one or more attachments for the communication to add to the case. Create the set by calling <a>AddAttachmentsToSet</a> </p>
    #[serde(rename = "attachmentSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    /// <p>The email addresses in the CC line of an email to be added to the support case.</p>
    #[serde(rename = "ccEmailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_email_addresses: Option<Vec<String>>,
    /// <p>The body of an email communication to add to the support case.</p>
    #[serde(rename = "communicationBody")]
    pub communication_body: String,
}

/// <p>The result of the <a>AddCommunicationToCase</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddCommunicationToCaseResponse {
    /// <p>True if <a>AddCommunicationToCase</a> succeeds. Otherwise, returns an error.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}

/// <p>An attachment to a case communication. The attachment consists of the file name and the content of the file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    /// <p>The content of the attachment file.</p>
    #[serde(rename = "data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: Option<Vec<u8>>,
    /// <p>The name of the attachment file.</p>
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

/// <p>The file name and ID of an attachment to a case communication. You can use the ID to retrieve the attachment with the <a>DescribeAttachment</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachmentDetails {
    /// <p>The ID of the attachment.</p>
    #[serde(rename = "attachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>The file name of the attachment.</p>
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

/// <p><p>A JSON-formatted object that contains the metadata for a support case. It is contained the response from a <a>DescribeCases</a> request. <b>CaseDetails</b> contains the following fields:</p> <ul> <li> <p> <b>caseId.</b> The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i>.</p> </li> <li> <p> <b>categoryCode.</b> The category of problem for the AWS Support case. Corresponds to the CategoryCode values returned by a call to <a>DescribeServices</a>.</p> </li> <li> <p> <b>displayId.</b> The identifier for the case on pages in the AWS Support Center.</p> </li> <li> <p> <b>language.</b> The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English (&quot;en&quot;) and Japanese (&quot;ja&quot;). Language parameters must be passed explicitly for operations that take them.</p> </li> <li> <p> <b>recentCommunications.</b> One or more <a>Communication</a> objects. Fields of these objects are <code>attachments</code>, <code>body</code>, <code>caseId</code>, <code>submittedBy</code>, and <code>timeCreated</code>.</p> </li> <li> <p> <b>nextToken.</b> A resumption point for pagination.</p> </li> <li> <p> <b>serviceCode.</b> The identifier for the AWS service that corresponds to the service code defined in the call to <a>DescribeServices</a>.</p> </li> <li> <p> <b>severityCode. </b>The severity code assigned to the case. Contains one of the values returned by the call to <a>DescribeSeverityLevels</a>.</p> </li> <li> <p> <b>status.</b> The status of the case in the AWS Support Center.</p> </li> <li> <p> <b>subject.</b> The subject line of the case.</p> </li> <li> <p> <b>submittedBy.</b> The email address of the account that submitted the case.</p> </li> <li> <p> <b>timeCreated.</b> The time the case was created, in ISO-8601 format.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CaseDetails {
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    /// <p>The category of problem for the AWS Support case.</p>
    #[serde(rename = "categoryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_code: Option<String>,
    /// <p>The email addresses that receive copies of communication about the case.</p>
    #[serde(rename = "ccEmailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_email_addresses: Option<Vec<String>>,
    /// <p>The ID displayed for the case in the AWS Support Center. This is a numeric string.</p>
    #[serde(rename = "displayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>The five most recent communications between you and AWS Support Center, including the IDs of any attachments to the communications. Also includes a <code>nextToken</code> that you can use to retrieve earlier communications.</p>
    #[serde(rename = "recentCommunications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_communications: Option<RecentCaseCommunications>,
    /// <p>The code for the AWS service returned by the call to <a>DescribeServices</a>.</p>
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The code for the severity level returned by the call to <a>DescribeSeverityLevels</a>.</p>
    #[serde(rename = "severityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_code: Option<String>,
    /// <p>The status of the case.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subject line for the case in the AWS Support Center.</p>
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>The email address of the account that submitted the case.</p>
    #[serde(rename = "submittedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    /// <p>The time that the case was case created in the AWS Support Center.</p>
    #[serde(rename = "timeCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
}

/// <p>A JSON-formatted name/value pair that represents the category name and category code of the problem, selected from the <a>DescribeServices</a> response for each AWS service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Category {
    /// <p>The category code for the support case.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The category name for the support case.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A communication associated with an AWS Support case. The communication consists of the case ID, the message body, attachment information, the account email address, and the date and time of the communication.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Communication {
    /// <p>Information about the attachments to the case communication.</p>
    #[serde(rename = "attachmentSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set: Option<Vec<AttachmentDetails>>,
    /// <p>The text of the communication between the customer and AWS Support.</p>
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    /// <p>The email address of the account that submitted the AWS Support case.</p>
    #[serde(rename = "submittedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    /// <p>The time the communication was created.</p>
    #[serde(rename = "timeCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCaseRequest {
    /// <p>The ID of a set of one or more attachments for the case. Create the set by using <a>AddAttachmentsToSet</a>.</p>
    #[serde(rename = "attachmentSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    /// <p>The category of problem for the AWS Support case.</p>
    #[serde(rename = "categoryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_code: Option<String>,
    /// <p>A list of email addresses that AWS Support copies on case correspondence.</p>
    #[serde(rename = "ccEmailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_email_addresses: Option<Vec<String>>,
    /// <p>The communication body text when you create an AWS Support case by calling <a>CreateCase</a>.</p>
    #[serde(rename = "communicationBody")]
    pub communication_body: String,
    /// <p>The type of issue for the case. You can specify either "customer-service" or "technical." If you do not indicate a value, the default is "technical."</p>
    #[serde(rename = "issueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>The code for the AWS service returned by the call to <a>DescribeServices</a>.</p>
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p><p>The code for the severity level returned by the call to <a>DescribeSeverityLevels</a>.</p> <note> <p>The availability of severity levels depends on each customer&#39;s support subscription. In other words, your subscription may not necessarily require the urgent level of response time.</p> </note></p>
    #[serde(rename = "severityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_code: Option<String>,
    /// <p>The title of the AWS Support case.</p>
    #[serde(rename = "subject")]
    pub subject: String,
}

/// <p>The AWS Support case ID returned by a successful completion of the <a>CreateCase</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateCaseResponse {
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAttachmentRequest {
    /// <p>The ID of the attachment to return. Attachment IDs are returned by the <a>DescribeCommunications</a> operation.</p>
    #[serde(rename = "attachmentId")]
    pub attachment_id: String,
}

/// <p>The content and file name of the attachment returned by the <a>DescribeAttachment</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAttachmentResponse {
    /// <p>The attachment content and file name.</p>
    #[serde(rename = "attachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCasesRequest {
    /// <p>The start date for a filtered date search on support case communications. Case communications are available for 12 months after creation.</p>
    #[serde(rename = "afterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<String>,
    /// <p>The end date for a filtered date search on support case communications. Case communications are available for 12 months after creation.</p>
    #[serde(rename = "beforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<String>,
    /// <p>A list of ID numbers of the support cases you want returned. The maximum number of cases is 100.</p>
    #[serde(rename = "caseIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id_list: Option<Vec<String>>,
    /// <p>The ID displayed for a case in the AWS Support Center user interface.</p>
    #[serde(rename = "displayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    /// <p>Specifies whether communications should be included in the <a>DescribeCases</a> results. The default is <i>true</i>.</p>
    #[serde(rename = "includeCommunications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_communications: Option<bool>,
    /// <p>Specifies whether resolved support cases should be included in the <a>DescribeCases</a> results. The default is <i>false</i>.</p>
    #[serde(rename = "includeResolvedCases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resolved_cases: Option<bool>,
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>The maximum number of results to return before paginating.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A resumption point for pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returns an array of <a>CaseDetails</a> objects and a <code>nextToken</code> that defines a point for pagination in the result set.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCasesResponse {
    /// <p>The details for the cases that match the request.</p>
    #[serde(rename = "cases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cases: Option<Vec<CaseDetails>>,
    /// <p>A resumption point for pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCommunicationsRequest {
    /// <p>The start date for a filtered date search on support case communications. Case communications are available for 12 months after creation.</p>
    #[serde(rename = "afterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<String>,
    /// <p>The end date for a filtered date search on support case communications. Case communications are available for 12 months after creation.</p>
    #[serde(rename = "beforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<String>,
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    pub case_id: String,
    /// <p>The maximum number of results to return before paginating.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A resumption point for pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The communications returned by the <a>DescribeCommunications</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCommunicationsResponse {
    /// <p>The communications for the case.</p>
    #[serde(rename = "communications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications: Option<Vec<Communication>>,
    /// <p>A resumption point for pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeServicesRequest {
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>A JSON-formatted list of service codes available for AWS services.</p>
    #[serde(rename = "serviceCodeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code_list: Option<Vec<String>>,
}

/// <p>The list of AWS services returned by the <a>DescribeServices</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeServicesResponse {
    /// <p>A JSON-formatted list of AWS services.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSeverityLevelsRequest {
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// <p>The list of severity levels returned by the <a>DescribeSeverityLevels</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSeverityLevelsResponse {
    /// <p>The available severity levels for the support case. Available severity levels are defined by your service level agreement with AWS.</p>
    #[serde(rename = "severityLevels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_levels: Option<Vec<SeverityLevel>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTrustedAdvisorCheckRefreshStatusesRequest {
    /// <p>The IDs of the Trusted Advisor checks to get the status of. <b>Note:</b> Specifying the check ID of a check that is automatically refreshed causes an <code>InvalidParameterValue</code> error.</p>
    #[serde(rename = "checkIds")]
    pub check_ids: Vec<String>,
}

/// <p>The statuses of the Trusted Advisor checks returned by the <a>DescribeTrustedAdvisorCheckRefreshStatuses</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTrustedAdvisorCheckRefreshStatusesResponse {
    /// <p>The refresh status of the specified Trusted Advisor checks.</p>
    #[serde(rename = "statuses")]
    pub statuses: Vec<TrustedAdvisorCheckRefreshStatus>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTrustedAdvisorCheckResultRequest {
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    #[serde(rename = "checkId")]
    pub check_id: String,
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// <p>The result of the Trusted Advisor check returned by the <a>DescribeTrustedAdvisorCheckResult</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTrustedAdvisorCheckResultResponse {
    /// <p>The detailed results of the Trusted Advisor check.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TrustedAdvisorCheckResult>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTrustedAdvisorCheckSummariesRequest {
    /// <p>The IDs of the Trusted Advisor checks.</p>
    #[serde(rename = "checkIds")]
    pub check_ids: Vec<String>,
}

/// <p>The summaries of the Trusted Advisor checks returned by the <a>DescribeTrustedAdvisorCheckSummaries</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTrustedAdvisorCheckSummariesResponse {
    /// <p>The summary information for the requested Trusted Advisor checks.</p>
    #[serde(rename = "summaries")]
    pub summaries: Vec<TrustedAdvisorCheckSummary>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTrustedAdvisorChecksRequest {
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    pub language: String,
}

/// <p>Information about the Trusted Advisor checks returned by the <a>DescribeTrustedAdvisorChecks</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTrustedAdvisorChecksResponse {
    /// <p>Information about all available Trusted Advisor checks.</p>
    #[serde(rename = "checks")]
    pub checks: Vec<TrustedAdvisorCheckDescription>,
}

/// <p>The five most recent communications associated with the case.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RecentCaseCommunications {
    /// <p>The five most recent communications associated with the case.</p>
    #[serde(rename = "communications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications: Option<Vec<Communication>>,
    /// <p>A resumption point for pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RefreshTrustedAdvisorCheckRequest {
    /// <p>The unique identifier for the Trusted Advisor check to refresh. <b>Note:</b> Specifying the check ID of a check that is automatically refreshed causes an <code>InvalidParameterValue</code> error.</p>
    #[serde(rename = "checkId")]
    pub check_id: String,
}

/// <p>The current refresh status of a Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RefreshTrustedAdvisorCheckResponse {
    /// <p>The current refresh status for a check, including the amount of time until the check is eligible for refresh.</p>
    #[serde(rename = "status")]
    pub status: TrustedAdvisorCheckRefreshStatus,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResolveCaseRequest {
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
}

/// <p>The status of the case returned by the <a>ResolveCase</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResolveCaseResponse {
    /// <p>The status of the case after the <a>ResolveCase</a> request was processed.</p>
    #[serde(rename = "finalCaseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_case_status: Option<String>,
    /// <p>The status of the case when the <a>ResolveCase</a> request was sent.</p>
    #[serde(rename = "initialCaseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_case_status: Option<String>,
}

/// <p>Information about an AWS service returned by the <a>DescribeServices</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Service {
    /// <p>A list of categories that describe the type of support issue a case describes. Categories consist of a category name and a category code. Category names and codes are passed to AWS Support when you call <a>CreateCase</a>.</p>
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    /// <p>The code for an AWS service returned by the <a>DescribeServices</a> response. The <code>name</code> element contains the corresponding friendly name.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The friendly name for an AWS service. The <code>code</code> element contains the corresponding code.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A code and name pair that represent a severity level that can be applied to a support case.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SeverityLevel {
    /// <p>One of four values: "low," "medium," "high," and "urgent". These values correspond to response times returned to the caller in <code>severityLevel.name</code>. </p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The name of the severity level that corresponds to the severity level code.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The container for summary information that relates to the category of the Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorCategorySpecificSummary {
    /// <p>The summary information about cost savings for a Trusted Advisor check that is in the Cost Optimizing category.</p>
    #[serde(rename = "costOptimizing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_optimizing: Option<TrustedAdvisorCostOptimizingSummary>,
}

/// <p>The description and metadata for a Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorCheckDescription {
    /// <p>The category of the Trusted Advisor check.</p>
    #[serde(rename = "category")]
    pub category: String,
    /// <p>The description of the Trusted Advisor check, which includes the alert criteria and recommended actions (contains HTML markup).</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The column headings for the data returned by the Trusted Advisor check. The order of the headings corresponds to the order of the data in the <b>Metadata</b> element of the <a>TrustedAdvisorResourceDetail</a> for the check. <b>Metadata</b> contains all the data that is shown in the Excel download, even in those cases where the UI shows just summary data. </p>
    #[serde(rename = "metadata")]
    pub metadata: Vec<String>,
    /// <p>The display name for the Trusted Advisor check.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>The refresh status of a Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorCheckRefreshStatus {
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    #[serde(rename = "checkId")]
    pub check_id: String,
    /// <p>The amount of time, in milliseconds, until the Trusted Advisor check is eligible for refresh.</p>
    #[serde(rename = "millisUntilNextRefreshable")]
    pub millis_until_next_refreshable: i64,
    /// <p>The status of the Trusted Advisor check for which a refresh has been requested: "none", "enqueued", "processing", "success", or "abandoned".</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>The results of a Trusted Advisor check returned by <a>DescribeTrustedAdvisorCheckResult</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorCheckResult {
    /// <p>Summary information that relates to the category of the check. Cost Optimizing is the only category that is currently supported.</p>
    #[serde(rename = "categorySpecificSummary")]
    pub category_specific_summary: TrustedAdvisorCategorySpecificSummary,
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    #[serde(rename = "checkId")]
    pub check_id: String,
    /// <p>The details about each resource listed in the check result.</p>
    #[serde(rename = "flaggedResources")]
    pub flagged_resources: Vec<TrustedAdvisorResourceDetail>,
    #[serde(rename = "resourcesSummary")]
    pub resources_summary: TrustedAdvisorResourcesSummary,
    /// <p>The alert status of the check: "ok" (green), "warning" (yellow), "error" (red), or "not_available".</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The time of the last refresh of the check.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

/// <p>A summary of a Trusted Advisor check result, including the alert status, last refresh, and number of resources examined.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorCheckSummary {
    /// <p>Summary information that relates to the category of the check. Cost Optimizing is the only category that is currently supported.</p>
    #[serde(rename = "categorySpecificSummary")]
    pub category_specific_summary: TrustedAdvisorCategorySpecificSummary,
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    #[serde(rename = "checkId")]
    pub check_id: String,
    /// <p>Specifies whether the Trusted Advisor check has flagged resources.</p>
    #[serde(rename = "hasFlaggedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_flagged_resources: Option<bool>,
    #[serde(rename = "resourcesSummary")]
    pub resources_summary: TrustedAdvisorResourcesSummary,
    /// <p>The alert status of the check: "ok" (green), "warning" (yellow), "error" (red), or "not_available".</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The time of the last refresh of the check.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

/// <p>The estimated cost savings that might be realized if the recommended actions are taken.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorCostOptimizingSummary {
    /// <p>The estimated monthly savings that might be realized if the recommended actions are taken.</p>
    #[serde(rename = "estimatedMonthlySavings")]
    pub estimated_monthly_savings: f64,
    /// <p>The estimated percentage of savings that might be realized if the recommended actions are taken.</p>
    #[serde(rename = "estimatedPercentMonthlySavings")]
    pub estimated_percent_monthly_savings: f64,
}

/// <p>Contains information about a resource identified by a Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorResourceDetail {
    /// <p>Specifies whether the AWS resource was ignored by Trusted Advisor because it was marked as suppressed by the user.</p>
    #[serde(rename = "isSuppressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
    /// <p>Additional information about the identified resource. The exact metadata and its order can be obtained by inspecting the <a>TrustedAdvisorCheckDescription</a> object returned by the call to <a>DescribeTrustedAdvisorChecks</a>. <b>Metadata</b> contains all the data that is shown in the Excel download, even in those cases where the UI shows just summary data. </p>
    #[serde(rename = "metadata")]
    pub metadata: Vec<String>,
    /// <p>The AWS region in which the identified resource is located.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The unique identifier for the identified resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>The status code for the resource identified in the Trusted Advisor check.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Details about AWS resources that were analyzed in a call to Trusted Advisor <a>DescribeTrustedAdvisorCheckSummaries</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrustedAdvisorResourcesSummary {
    /// <p>The number of AWS resources that were flagged (listed) by the Trusted Advisor check.</p>
    #[serde(rename = "resourcesFlagged")]
    pub resources_flagged: i64,
    /// <p>The number of AWS resources ignored by Trusted Advisor because information was unavailable.</p>
    #[serde(rename = "resourcesIgnored")]
    pub resources_ignored: i64,
    /// <p>The number of AWS resources that were analyzed by the Trusted Advisor check.</p>
    #[serde(rename = "resourcesProcessed")]
    pub resources_processed: i64,
    /// <p>The number of AWS resources ignored by Trusted Advisor because they were marked as suppressed by the user.</p>
    #[serde(rename = "resourcesSuppressed")]
    pub resources_suppressed: i64,
}

/// Errors returned by AddAttachmentsToSet
#[derive(Debug, PartialEq)]
pub enum AddAttachmentsToSetError {
    /// <p>The limit for the number of attachment sets created in a short period of time has been exceeded.</p>
    AttachmentLimitExceeded(String),
    /// <p>The expiration time of the attachment set has passed. The set expires 1 hour after it is created.</p>
    AttachmentSetExpired(String),
    /// <p>An attachment set with the specified ID could not be found.</p>
    AttachmentSetIdNotFound(String),
    /// <p>A limit for the size of an attachment set has been exceeded. The limits are 3 attachments and 5 MB per attachment.</p>
    AttachmentSetSizeLimitExceeded(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddAttachmentsToSetError {
    pub fn from_body(body: &str) -> AddAttachmentsToSetError {
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
                    "AttachmentLimitExceeded" => AddAttachmentsToSetError::AttachmentLimitExceeded(
                        String::from(error_message),
                    ),
                    "AttachmentSetExpired" => {
                        AddAttachmentsToSetError::AttachmentSetExpired(String::from(error_message))
                    }
                    "AttachmentSetIdNotFound" => AddAttachmentsToSetError::AttachmentSetIdNotFound(
                        String::from(error_message),
                    ),
                    "AttachmentSetSizeLimitExceeded" => {
                        AddAttachmentsToSetError::AttachmentSetSizeLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        AddAttachmentsToSetError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddAttachmentsToSetError::Validation(error_message.to_string())
                    }
                    _ => AddAttachmentsToSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddAttachmentsToSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddAttachmentsToSetError {
    fn from(err: serde_json::error::Error) -> AddAttachmentsToSetError {
        AddAttachmentsToSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddAttachmentsToSetError {
    fn from(err: CredentialsError) -> AddAttachmentsToSetError {
        AddAttachmentsToSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddAttachmentsToSetError {
    fn from(err: HttpDispatchError) -> AddAttachmentsToSetError {
        AddAttachmentsToSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddAttachmentsToSetError {
    fn from(err: io::Error) -> AddAttachmentsToSetError {
        AddAttachmentsToSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddAttachmentsToSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddAttachmentsToSetError {
    fn description(&self) -> &str {
        match *self {
            AddAttachmentsToSetError::AttachmentLimitExceeded(ref cause) => cause,
            AddAttachmentsToSetError::AttachmentSetExpired(ref cause) => cause,
            AddAttachmentsToSetError::AttachmentSetIdNotFound(ref cause) => cause,
            AddAttachmentsToSetError::AttachmentSetSizeLimitExceeded(ref cause) => cause,
            AddAttachmentsToSetError::InternalServerError(ref cause) => cause,
            AddAttachmentsToSetError::Validation(ref cause) => cause,
            AddAttachmentsToSetError::Credentials(ref err) => err.description(),
            AddAttachmentsToSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddAttachmentsToSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddCommunicationToCase
#[derive(Debug, PartialEq)]
pub enum AddCommunicationToCaseError {
    /// <p>The expiration time of the attachment set has passed. The set expires 1 hour after it is created.</p>
    AttachmentSetExpired(String),
    /// <p>An attachment set with the specified ID could not be found.</p>
    AttachmentSetIdNotFound(String),
    /// <p>The requested <code>caseId</code> could not be located.</p>
    CaseIdNotFound(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddCommunicationToCaseError {
    pub fn from_body(body: &str) -> AddCommunicationToCaseError {
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
                    "AttachmentSetExpired" => AddCommunicationToCaseError::AttachmentSetExpired(
                        String::from(error_message),
                    ),
                    "AttachmentSetIdNotFound" => {
                        AddCommunicationToCaseError::AttachmentSetIdNotFound(String::from(
                            error_message,
                        ))
                    }
                    "CaseIdNotFound" => {
                        AddCommunicationToCaseError::CaseIdNotFound(String::from(error_message))
                    }
                    "InternalServerError" => AddCommunicationToCaseError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        AddCommunicationToCaseError::Validation(error_message.to_string())
                    }
                    _ => AddCommunicationToCaseError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddCommunicationToCaseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddCommunicationToCaseError {
    fn from(err: serde_json::error::Error) -> AddCommunicationToCaseError {
        AddCommunicationToCaseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddCommunicationToCaseError {
    fn from(err: CredentialsError) -> AddCommunicationToCaseError {
        AddCommunicationToCaseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddCommunicationToCaseError {
    fn from(err: HttpDispatchError) -> AddCommunicationToCaseError {
        AddCommunicationToCaseError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddCommunicationToCaseError {
    fn from(err: io::Error) -> AddCommunicationToCaseError {
        AddCommunicationToCaseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddCommunicationToCaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddCommunicationToCaseError {
    fn description(&self) -> &str {
        match *self {
            AddCommunicationToCaseError::AttachmentSetExpired(ref cause) => cause,
            AddCommunicationToCaseError::AttachmentSetIdNotFound(ref cause) => cause,
            AddCommunicationToCaseError::CaseIdNotFound(ref cause) => cause,
            AddCommunicationToCaseError::InternalServerError(ref cause) => cause,
            AddCommunicationToCaseError::Validation(ref cause) => cause,
            AddCommunicationToCaseError::Credentials(ref err) => err.description(),
            AddCommunicationToCaseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddCommunicationToCaseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCase
#[derive(Debug, PartialEq)]
pub enum CreateCaseError {
    /// <p>The expiration time of the attachment set has passed. The set expires 1 hour after it is created.</p>
    AttachmentSetExpired(String),
    /// <p>An attachment set with the specified ID could not be found.</p>
    AttachmentSetIdNotFound(String),
    /// <p>The case creation limit for the account has been exceeded.</p>
    CaseCreationLimitExceeded(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCaseError {
    pub fn from_body(body: &str) -> CreateCaseError {
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
                    "AttachmentSetExpired" => {
                        CreateCaseError::AttachmentSetExpired(String::from(error_message))
                    }
                    "AttachmentSetIdNotFound" => {
                        CreateCaseError::AttachmentSetIdNotFound(String::from(error_message))
                    }
                    "CaseCreationLimitExceeded" => {
                        CreateCaseError::CaseCreationLimitExceeded(String::from(error_message))
                    }
                    "InternalServerError" => {
                        CreateCaseError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => CreateCaseError::Validation(error_message.to_string()),
                    _ => CreateCaseError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCaseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCaseError {
    fn from(err: serde_json::error::Error) -> CreateCaseError {
        CreateCaseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCaseError {
    fn from(err: CredentialsError) -> CreateCaseError {
        CreateCaseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCaseError {
    fn from(err: HttpDispatchError) -> CreateCaseError {
        CreateCaseError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCaseError {
    fn from(err: io::Error) -> CreateCaseError {
        CreateCaseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCaseError {
    fn description(&self) -> &str {
        match *self {
            CreateCaseError::AttachmentSetExpired(ref cause) => cause,
            CreateCaseError::AttachmentSetIdNotFound(ref cause) => cause,
            CreateCaseError::CaseCreationLimitExceeded(ref cause) => cause,
            CreateCaseError::InternalServerError(ref cause) => cause,
            CreateCaseError::Validation(ref cause) => cause,
            CreateCaseError::Credentials(ref err) => err.description(),
            CreateCaseError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateCaseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAttachment
#[derive(Debug, PartialEq)]
pub enum DescribeAttachmentError {
    /// <p>An attachment with the specified ID could not be found.</p>
    AttachmentIdNotFound(String),
    /// <p>The limit for the number of <a>DescribeAttachment</a> requests in a short period of time has been exceeded.</p>
    DescribeAttachmentLimitExceeded(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAttachmentError {
    pub fn from_body(body: &str) -> DescribeAttachmentError {
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
                    "AttachmentIdNotFound" => {
                        DescribeAttachmentError::AttachmentIdNotFound(String::from(error_message))
                    }
                    "DescribeAttachmentLimitExceeded" => {
                        DescribeAttachmentError::DescribeAttachmentLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        DescribeAttachmentError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAttachmentError::Validation(error_message.to_string())
                    }
                    _ => DescribeAttachmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAttachmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAttachmentError {
    fn from(err: serde_json::error::Error) -> DescribeAttachmentError {
        DescribeAttachmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAttachmentError {
    fn from(err: CredentialsError) -> DescribeAttachmentError {
        DescribeAttachmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAttachmentError {
    fn from(err: HttpDispatchError) -> DescribeAttachmentError {
        DescribeAttachmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAttachmentError {
    fn from(err: io::Error) -> DescribeAttachmentError {
        DescribeAttachmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAttachmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAttachmentError {
    fn description(&self) -> &str {
        match *self {
            DescribeAttachmentError::AttachmentIdNotFound(ref cause) => cause,
            DescribeAttachmentError::DescribeAttachmentLimitExceeded(ref cause) => cause,
            DescribeAttachmentError::InternalServerError(ref cause) => cause,
            DescribeAttachmentError::Validation(ref cause) => cause,
            DescribeAttachmentError::Credentials(ref err) => err.description(),
            DescribeAttachmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAttachmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCases
#[derive(Debug, PartialEq)]
pub enum DescribeCasesError {
    /// <p>The requested <code>caseId</code> could not be located.</p>
    CaseIdNotFound(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCasesError {
    pub fn from_body(body: &str) -> DescribeCasesError {
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
                    "CaseIdNotFound" => {
                        DescribeCasesError::CaseIdNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DescribeCasesError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCasesError::Validation(error_message.to_string())
                    }
                    _ => DescribeCasesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCasesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCasesError {
    fn from(err: serde_json::error::Error) -> DescribeCasesError {
        DescribeCasesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCasesError {
    fn from(err: CredentialsError) -> DescribeCasesError {
        DescribeCasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCasesError {
    fn from(err: HttpDispatchError) -> DescribeCasesError {
        DescribeCasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCasesError {
    fn from(err: io::Error) -> DescribeCasesError {
        DescribeCasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCasesError {
    fn description(&self) -> &str {
        match *self {
            DescribeCasesError::CaseIdNotFound(ref cause) => cause,
            DescribeCasesError::InternalServerError(ref cause) => cause,
            DescribeCasesError::Validation(ref cause) => cause,
            DescribeCasesError::Credentials(ref err) => err.description(),
            DescribeCasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeCasesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCommunications
#[derive(Debug, PartialEq)]
pub enum DescribeCommunicationsError {
    /// <p>The requested <code>caseId</code> could not be located.</p>
    CaseIdNotFound(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCommunicationsError {
    pub fn from_body(body: &str) -> DescribeCommunicationsError {
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
                    "CaseIdNotFound" => {
                        DescribeCommunicationsError::CaseIdNotFound(String::from(error_message))
                    }
                    "InternalServerError" => DescribeCommunicationsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeCommunicationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeCommunicationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCommunicationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCommunicationsError {
    fn from(err: serde_json::error::Error) -> DescribeCommunicationsError {
        DescribeCommunicationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCommunicationsError {
    fn from(err: CredentialsError) -> DescribeCommunicationsError {
        DescribeCommunicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCommunicationsError {
    fn from(err: HttpDispatchError) -> DescribeCommunicationsError {
        DescribeCommunicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCommunicationsError {
    fn from(err: io::Error) -> DescribeCommunicationsError {
        DescribeCommunicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCommunicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCommunicationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeCommunicationsError::CaseIdNotFound(ref cause) => cause,
            DescribeCommunicationsError::InternalServerError(ref cause) => cause,
            DescribeCommunicationsError::Validation(ref cause) => cause,
            DescribeCommunicationsError::Credentials(ref err) => err.description(),
            DescribeCommunicationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCommunicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeServicesError {
    pub fn from_body(body: &str) -> DescribeServicesError {
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
                    "InternalServerError" => {
                        DescribeServicesError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeServicesError::Validation(error_message.to_string())
                    }
                    _ => DescribeServicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeServicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeServicesError {
    fn from(err: serde_json::error::Error) -> DescribeServicesError {
        DescribeServicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeServicesError {
    fn from(err: CredentialsError) -> DescribeServicesError {
        DescribeServicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeServicesError {
    fn from(err: HttpDispatchError) -> DescribeServicesError {
        DescribeServicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeServicesError {
    fn from(err: io::Error) -> DescribeServicesError {
        DescribeServicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeServicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServicesError {
    fn description(&self) -> &str {
        match *self {
            DescribeServicesError::InternalServerError(ref cause) => cause,
            DescribeServicesError::Validation(ref cause) => cause,
            DescribeServicesError::Credentials(ref err) => err.description(),
            DescribeServicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeServicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSeverityLevels
#[derive(Debug, PartialEq)]
pub enum DescribeSeverityLevelsError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSeverityLevelsError {
    pub fn from_body(body: &str) -> DescribeSeverityLevelsError {
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
                    "InternalServerError" => DescribeSeverityLevelsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeSeverityLevelsError::Validation(error_message.to_string())
                    }
                    _ => DescribeSeverityLevelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSeverityLevelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSeverityLevelsError {
    fn from(err: serde_json::error::Error) -> DescribeSeverityLevelsError {
        DescribeSeverityLevelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSeverityLevelsError {
    fn from(err: CredentialsError) -> DescribeSeverityLevelsError {
        DescribeSeverityLevelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSeverityLevelsError {
    fn from(err: HttpDispatchError) -> DescribeSeverityLevelsError {
        DescribeSeverityLevelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSeverityLevelsError {
    fn from(err: io::Error) -> DescribeSeverityLevelsError {
        DescribeSeverityLevelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSeverityLevelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSeverityLevelsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSeverityLevelsError::InternalServerError(ref cause) => cause,
            DescribeSeverityLevelsError::Validation(ref cause) => cause,
            DescribeSeverityLevelsError::Credentials(ref err) => err.description(),
            DescribeSeverityLevelsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSeverityLevelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTrustedAdvisorCheckRefreshStatuses
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorCheckRefreshStatusesError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTrustedAdvisorCheckRefreshStatusesError {
    pub fn from_body(body: &str) -> DescribeTrustedAdvisorCheckRefreshStatusesError {
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
                    "InternalServerError" => {
                        DescribeTrustedAdvisorCheckRefreshStatusesError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DescribeTrustedAdvisorCheckRefreshStatusesError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => {
                        DescribeTrustedAdvisorCheckRefreshStatusesError::Unknown(String::from(body))
                    }
                }
            }
            Err(_) => DescribeTrustedAdvisorCheckRefreshStatusesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTrustedAdvisorCheckRefreshStatusesError {
    fn from(err: serde_json::error::Error) -> DescribeTrustedAdvisorCheckRefreshStatusesError {
        DescribeTrustedAdvisorCheckRefreshStatusesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTrustedAdvisorCheckRefreshStatusesError {
    fn from(err: CredentialsError) -> DescribeTrustedAdvisorCheckRefreshStatusesError {
        DescribeTrustedAdvisorCheckRefreshStatusesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTrustedAdvisorCheckRefreshStatusesError {
    fn from(err: HttpDispatchError) -> DescribeTrustedAdvisorCheckRefreshStatusesError {
        DescribeTrustedAdvisorCheckRefreshStatusesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTrustedAdvisorCheckRefreshStatusesError {
    fn from(err: io::Error) -> DescribeTrustedAdvisorCheckRefreshStatusesError {
        DescribeTrustedAdvisorCheckRefreshStatusesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTrustedAdvisorCheckRefreshStatusesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTrustedAdvisorCheckRefreshStatusesError {
    fn description(&self) -> &str {
        match *self {
            DescribeTrustedAdvisorCheckRefreshStatusesError::InternalServerError(ref cause) => {
                cause
            }
            DescribeTrustedAdvisorCheckRefreshStatusesError::Validation(ref cause) => cause,
            DescribeTrustedAdvisorCheckRefreshStatusesError::Credentials(ref err) => {
                err.description()
            }
            DescribeTrustedAdvisorCheckRefreshStatusesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTrustedAdvisorCheckRefreshStatusesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTrustedAdvisorCheckResult
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorCheckResultError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTrustedAdvisorCheckResultError {
    pub fn from_body(body: &str) -> DescribeTrustedAdvisorCheckResultError {
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
                    "InternalServerError" => {
                        DescribeTrustedAdvisorCheckResultError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DescribeTrustedAdvisorCheckResultError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeTrustedAdvisorCheckResultError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTrustedAdvisorCheckResultError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTrustedAdvisorCheckResultError {
    fn from(err: serde_json::error::Error) -> DescribeTrustedAdvisorCheckResultError {
        DescribeTrustedAdvisorCheckResultError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTrustedAdvisorCheckResultError {
    fn from(err: CredentialsError) -> DescribeTrustedAdvisorCheckResultError {
        DescribeTrustedAdvisorCheckResultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTrustedAdvisorCheckResultError {
    fn from(err: HttpDispatchError) -> DescribeTrustedAdvisorCheckResultError {
        DescribeTrustedAdvisorCheckResultError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTrustedAdvisorCheckResultError {
    fn from(err: io::Error) -> DescribeTrustedAdvisorCheckResultError {
        DescribeTrustedAdvisorCheckResultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTrustedAdvisorCheckResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTrustedAdvisorCheckResultError {
    fn description(&self) -> &str {
        match *self {
            DescribeTrustedAdvisorCheckResultError::InternalServerError(ref cause) => cause,
            DescribeTrustedAdvisorCheckResultError::Validation(ref cause) => cause,
            DescribeTrustedAdvisorCheckResultError::Credentials(ref err) => err.description(),
            DescribeTrustedAdvisorCheckResultError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTrustedAdvisorCheckResultError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTrustedAdvisorCheckSummaries
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorCheckSummariesError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTrustedAdvisorCheckSummariesError {
    pub fn from_body(body: &str) -> DescribeTrustedAdvisorCheckSummariesError {
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
                    "InternalServerError" => {
                        DescribeTrustedAdvisorCheckSummariesError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => DescribeTrustedAdvisorCheckSummariesError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeTrustedAdvisorCheckSummariesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTrustedAdvisorCheckSummariesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTrustedAdvisorCheckSummariesError {
    fn from(err: serde_json::error::Error) -> DescribeTrustedAdvisorCheckSummariesError {
        DescribeTrustedAdvisorCheckSummariesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTrustedAdvisorCheckSummariesError {
    fn from(err: CredentialsError) -> DescribeTrustedAdvisorCheckSummariesError {
        DescribeTrustedAdvisorCheckSummariesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTrustedAdvisorCheckSummariesError {
    fn from(err: HttpDispatchError) -> DescribeTrustedAdvisorCheckSummariesError {
        DescribeTrustedAdvisorCheckSummariesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTrustedAdvisorCheckSummariesError {
    fn from(err: io::Error) -> DescribeTrustedAdvisorCheckSummariesError {
        DescribeTrustedAdvisorCheckSummariesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTrustedAdvisorCheckSummariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTrustedAdvisorCheckSummariesError {
    fn description(&self) -> &str {
        match *self {
            DescribeTrustedAdvisorCheckSummariesError::InternalServerError(ref cause) => cause,
            DescribeTrustedAdvisorCheckSummariesError::Validation(ref cause) => cause,
            DescribeTrustedAdvisorCheckSummariesError::Credentials(ref err) => err.description(),
            DescribeTrustedAdvisorCheckSummariesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTrustedAdvisorCheckSummariesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTrustedAdvisorChecks
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorChecksError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTrustedAdvisorChecksError {
    pub fn from_body(body: &str) -> DescribeTrustedAdvisorChecksError {
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
                    "InternalServerError" => {
                        DescribeTrustedAdvisorChecksError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeTrustedAdvisorChecksError::Validation(error_message.to_string())
                    }
                    _ => DescribeTrustedAdvisorChecksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTrustedAdvisorChecksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTrustedAdvisorChecksError {
    fn from(err: serde_json::error::Error) -> DescribeTrustedAdvisorChecksError {
        DescribeTrustedAdvisorChecksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTrustedAdvisorChecksError {
    fn from(err: CredentialsError) -> DescribeTrustedAdvisorChecksError {
        DescribeTrustedAdvisorChecksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTrustedAdvisorChecksError {
    fn from(err: HttpDispatchError) -> DescribeTrustedAdvisorChecksError {
        DescribeTrustedAdvisorChecksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTrustedAdvisorChecksError {
    fn from(err: io::Error) -> DescribeTrustedAdvisorChecksError {
        DescribeTrustedAdvisorChecksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTrustedAdvisorChecksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTrustedAdvisorChecksError {
    fn description(&self) -> &str {
        match *self {
            DescribeTrustedAdvisorChecksError::InternalServerError(ref cause) => cause,
            DescribeTrustedAdvisorChecksError::Validation(ref cause) => cause,
            DescribeTrustedAdvisorChecksError::Credentials(ref err) => err.description(),
            DescribeTrustedAdvisorChecksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTrustedAdvisorChecksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RefreshTrustedAdvisorCheck
#[derive(Debug, PartialEq)]
pub enum RefreshTrustedAdvisorCheckError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RefreshTrustedAdvisorCheckError {
    pub fn from_body(body: &str) -> RefreshTrustedAdvisorCheckError {
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
                    "InternalServerError" => RefreshTrustedAdvisorCheckError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        RefreshTrustedAdvisorCheckError::Validation(error_message.to_string())
                    }
                    _ => RefreshTrustedAdvisorCheckError::Unknown(String::from(body)),
                }
            }
            Err(_) => RefreshTrustedAdvisorCheckError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RefreshTrustedAdvisorCheckError {
    fn from(err: serde_json::error::Error) -> RefreshTrustedAdvisorCheckError {
        RefreshTrustedAdvisorCheckError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RefreshTrustedAdvisorCheckError {
    fn from(err: CredentialsError) -> RefreshTrustedAdvisorCheckError {
        RefreshTrustedAdvisorCheckError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RefreshTrustedAdvisorCheckError {
    fn from(err: HttpDispatchError) -> RefreshTrustedAdvisorCheckError {
        RefreshTrustedAdvisorCheckError::HttpDispatch(err)
    }
}
impl From<io::Error> for RefreshTrustedAdvisorCheckError {
    fn from(err: io::Error) -> RefreshTrustedAdvisorCheckError {
        RefreshTrustedAdvisorCheckError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RefreshTrustedAdvisorCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RefreshTrustedAdvisorCheckError {
    fn description(&self) -> &str {
        match *self {
            RefreshTrustedAdvisorCheckError::InternalServerError(ref cause) => cause,
            RefreshTrustedAdvisorCheckError::Validation(ref cause) => cause,
            RefreshTrustedAdvisorCheckError::Credentials(ref err) => err.description(),
            RefreshTrustedAdvisorCheckError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RefreshTrustedAdvisorCheckError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResolveCase
#[derive(Debug, PartialEq)]
pub enum ResolveCaseError {
    /// <p>The requested <code>caseId</code> could not be located.</p>
    CaseIdNotFound(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResolveCaseError {
    pub fn from_body(body: &str) -> ResolveCaseError {
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
                    "CaseIdNotFound" => {
                        ResolveCaseError::CaseIdNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        ResolveCaseError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ResolveCaseError::Validation(error_message.to_string())
                    }
                    _ => ResolveCaseError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResolveCaseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResolveCaseError {
    fn from(err: serde_json::error::Error) -> ResolveCaseError {
        ResolveCaseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResolveCaseError {
    fn from(err: CredentialsError) -> ResolveCaseError {
        ResolveCaseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResolveCaseError {
    fn from(err: HttpDispatchError) -> ResolveCaseError {
        ResolveCaseError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResolveCaseError {
    fn from(err: io::Error) -> ResolveCaseError {
        ResolveCaseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResolveCaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResolveCaseError {
    fn description(&self) -> &str {
        match *self {
            ResolveCaseError::CaseIdNotFound(ref cause) => cause,
            ResolveCaseError::InternalServerError(ref cause) => cause,
            ResolveCaseError::Validation(ref cause) => cause,
            ResolveCaseError::Credentials(ref err) => err.description(),
            ResolveCaseError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResolveCaseError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Support API. AWS Support clients implement this trait.
pub trait AWSSupport {
    /// <p>Adds one or more attachments to an attachment set. If an <code>attachmentSetId</code> is not specified, a new attachment set is created, and the ID of the set is returned in the response. If an <code>attachmentSetId</code> is specified, the attachments are added to the specified set, if it exists.</p> <p>An attachment set is a temporary container for attachments that are to be added to a case or case communication. The set is available for one hour after it is created; the <code>expiryTime</code> returned in the response indicates when the set expires. The maximum number of attachments in a set is 3, and the maximum size of any attachment in the set is 5 MB.</p>
    fn add_attachments_to_set(
        &self,
        input: AddAttachmentsToSetRequest,
    ) -> RusotoFuture<AddAttachmentsToSetResponse, AddAttachmentsToSetError>;

    /// <p>Adds additional customer communication to an AWS Support case. You use the <code>caseId</code> value to identify the case to add communication to. You can list a set of email addresses to copy on the communication using the <code>ccEmailAddresses</code> value. The <code>communicationBody</code> value contains the text of the communication.</p> <p>The response indicates the success or failure of the request.</p> <p>This operation implements a subset of the features of the AWS Support Center.</p>
    fn add_communication_to_case(
        &self,
        input: AddCommunicationToCaseRequest,
    ) -> RusotoFuture<AddCommunicationToCaseResponse, AddCommunicationToCaseError>;

    /// <p>Creates a new case in the AWS Support Center. This operation is modeled on the behavior of the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. Its parameters require you to specify the following information: </p> <ul> <li> <p> <b>issueType.</b> The type of issue for the case. You can specify either "customer-service" or "technical." If you do not indicate a value, the default is "technical." </p> </li> <li> <p> <b>serviceCode.</b> The code for an AWS service. You obtain the <code>serviceCode</code> by calling <a>DescribeServices</a>. </p> </li> <li> <p> <b>categoryCode.</b> The category for the service defined for the <code>serviceCode</code> value. You also obtain the category code for a service by calling <a>DescribeServices</a>. Each AWS service defines its own set of category codes. </p> </li> <li> <p> <b>severityCode.</b> A value that indicates the urgency of the case, which in turn determines the response time according to your service level agreement with AWS Support. You obtain the SeverityCode by calling <a>DescribeSeverityLevels</a>.</p> </li> <li> <p> <b>subject.</b> The <b>Subject</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>communicationBody.</b> The <b>Description</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>attachmentSetId.</b> The ID of a set of attachments that has been created by using <a>AddAttachmentsToSet</a>.</p> </li> <li> <p> <b>language.</b> The human language in which AWS Support handles the case. English and Japanese are currently supported.</p> </li> <li> <p> <b>ccEmailAddresses.</b> The AWS Support Center <b>CC</b> field on the <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. You can list email addresses to be copied on any correspondence about the case. The account that opens the case is already identified by passing the AWS Credentials in the HTTP POST method or in a method or function call from one of the programming languages supported by an <a href="http://aws.amazon.com/tools/">AWS SDK</a>. </p> </li> </ul> <note> <p>To add additional communication or attachments to an existing case, use <a>AddCommunicationToCase</a>.</p> </note> <p>A successful <a>CreateCase</a> request returns an AWS Support case number. Case numbers are used by the <a>DescribeCases</a> operation to retrieve existing AWS Support cases. </p>
    fn create_case(
        &self,
        input: CreateCaseRequest,
    ) -> RusotoFuture<CreateCaseResponse, CreateCaseError>;

    /// <p>Returns the attachment that has the specified ID. Attachment IDs are generated by the case management system when you add an attachment to a case or case communication. Attachment IDs are returned in the <a>AttachmentDetails</a> objects that are returned by the <a>DescribeCommunications</a> operation.</p>
    fn describe_attachment(
        &self,
        input: DescribeAttachmentRequest,
    ) -> RusotoFuture<DescribeAttachmentResponse, DescribeAttachmentError>;

    /// <p><p>Returns a list of cases that you specify by passing one or more case IDs. In addition, you can filter the cases by date by setting values for the <code>afterTime</code> and <code>beforeTime</code> request parameters. You can set values for the <code>includeResolvedCases</code> and <code>includeCommunications</code> request parameters to control how much information is returned. </p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>The response returns the following in JSON format:</p> <ul> <li> <p>One or more <a>CaseDetails</a> data types. </p> </li> <li> <p>One or more <code>nextToken</code> values, which specify where to paginate the returned records represented by the <code>CaseDetails</code> objects.</p> </li> </ul></p>
    fn describe_cases(
        &self,
        input: DescribeCasesRequest,
    ) -> RusotoFuture<DescribeCasesResponse, DescribeCasesError>;

    /// <p>Returns communications (and attachments) for one or more support cases. You can use the <code>afterTime</code> and <code>beforeTime</code> parameters to filter by date. You can use the <code>caseId</code> parameter to restrict the results to a particular case.</p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>You can use the <code>maxResults</code> and <code>nextToken</code> parameters to control the pagination of the result set. Set <code>maxResults</code> to the number of cases you want displayed on each page, and use <code>nextToken</code> to specify the resumption of pagination.</p>
    fn describe_communications(
        &self,
        input: DescribeCommunicationsRequest,
    ) -> RusotoFuture<DescribeCommunicationsResponse, DescribeCommunicationsError>;

    /// <p>Returns the current list of AWS services and a list of service categories that applies to each one. You then use service names and categories in your <a>CreateCase</a> requests. Each AWS service has its own set of categories.</p> <p>The service codes and category codes correspond to the values that are displayed in the <b>Service</b> and <b>Category</b> drop-down lists on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. The values in those fields, however, do not necessarily match the service codes and categories returned by the <code>DescribeServices</code> request. Always use the service codes and categories obtained programmatically. This practice ensures that you always have the most recent set of service and category codes.</p>
    fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> RusotoFuture<DescribeServicesResponse, DescribeServicesError>;

    /// <p>Returns the list of severity levels that you can assign to an AWS Support case. The severity level for a case is also a field in the <a>CaseDetails</a> data type included in any <a>CreateCase</a> request. </p>
    fn describe_severity_levels(
        &self,
        input: DescribeSeverityLevelsRequest,
    ) -> RusotoFuture<DescribeSeverityLevelsResponse, DescribeSeverityLevelsError>;

    /// <p><p>Returns the refresh status of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and their refresh statuses cannot be retrieved by using this operation. Use of the <code>DescribeTrustedAdvisorCheckRefreshStatuses</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note></p>
    fn describe_trusted_advisor_check_refresh_statuses(
        &self,
        input: DescribeTrustedAdvisorCheckRefreshStatusesRequest,
    ) -> RusotoFuture<
        DescribeTrustedAdvisorCheckRefreshStatusesResponse,
        DescribeTrustedAdvisorCheckRefreshStatusesError,
    >;

    /// <p><p>Returns the results of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains a <a>TrustedAdvisorCheckResult</a> object, which contains these three objects:</p> <ul> <li> <p> <a>TrustedAdvisorCategorySpecificSummary</a> </p> </li> <li> <p> <a>TrustedAdvisorResourceDetail</a> </p> </li> <li> <p> <a>TrustedAdvisorResourcesSummary</a> </p> </li> </ul> <p>In addition, the response contains these fields:</p> <ul> <li> <p> <b>status.</b> The alert status of the check: &quot;ok&quot; (green), &quot;warning&quot; (yellow), &quot;error&quot; (red), or &quot;not_available&quot;.</p> </li> <li> <p> <b>timestamp.</b> The time of the last refresh of the check.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    fn describe_trusted_advisor_check_result(
        &self,
        input: DescribeTrustedAdvisorCheckResultRequest,
    ) -> RusotoFuture<
        DescribeTrustedAdvisorCheckResultResponse,
        DescribeTrustedAdvisorCheckResultError,
    >;

    /// <p>Returns the summaries of the results of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains an array of <a>TrustedAdvisorCheckSummary</a> objects.</p>
    fn describe_trusted_advisor_check_summaries(
        &self,
        input: DescribeTrustedAdvisorCheckSummariesRequest,
    ) -> RusotoFuture<
        DescribeTrustedAdvisorCheckSummariesResponse,
        DescribeTrustedAdvisorCheckSummariesError,
    >;

    /// <p>Returns information about all available Trusted Advisor checks, including name, ID, category, description, and metadata. You must specify a language code; English ("en") and Japanese ("ja") are currently supported. The response contains a <a>TrustedAdvisorCheckDescription</a> for each check.</p>
    fn describe_trusted_advisor_checks(
        &self,
        input: DescribeTrustedAdvisorChecksRequest,
    ) -> RusotoFuture<DescribeTrustedAdvisorChecksResponse, DescribeTrustedAdvisorChecksError>;

    /// <p><p>Requests a refresh of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and they cannot be refreshed by using this operation. Use of the <code>RefreshTrustedAdvisorCheck</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note> <p>The response contains a <a>TrustedAdvisorCheckRefreshStatus</a> object, which contains these fields:</p> <ul> <li> <p> <b>status.</b> The refresh status of the check: &quot;none&quot;, &quot;enqueued&quot;, &quot;processing&quot;, &quot;success&quot;, or &quot;abandoned&quot;.</p> </li> <li> <p> <b>millisUntilNextRefreshable.</b> The amount of time, in milliseconds, until the check is eligible for refresh.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    fn refresh_trusted_advisor_check(
        &self,
        input: RefreshTrustedAdvisorCheckRequest,
    ) -> RusotoFuture<RefreshTrustedAdvisorCheckResponse, RefreshTrustedAdvisorCheckError>;

    /// <p>Takes a <code>caseId</code> and returns the initial state of the case along with the state of the case after the call to <a>ResolveCase</a> completed.</p>
    fn resolve_case(
        &self,
        input: ResolveCaseRequest,
    ) -> RusotoFuture<ResolveCaseResponse, ResolveCaseError>;
}
/// A client for the AWS Support API.
pub struct AWSSupportClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl AWSSupportClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> AWSSupportClient {
        AWSSupportClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> AWSSupportClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AWSSupportClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> AWSSupport for AWSSupportClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds one or more attachments to an attachment set. If an <code>attachmentSetId</code> is not specified, a new attachment set is created, and the ID of the set is returned in the response. If an <code>attachmentSetId</code> is specified, the attachments are added to the specified set, if it exists.</p> <p>An attachment set is a temporary container for attachments that are to be added to a case or case communication. The set is available for one hour after it is created; the <code>expiryTime</code> returned in the response indicates when the set expires. The maximum number of attachments in a set is 3, and the maximum size of any attachment in the set is 5 MB.</p>
    fn add_attachments_to_set(
        &self,
        input: AddAttachmentsToSetRequest,
    ) -> RusotoFuture<AddAttachmentsToSetResponse, AddAttachmentsToSetError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.AddAttachmentsToSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddAttachmentsToSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddAttachmentsToSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds additional customer communication to an AWS Support case. You use the <code>caseId</code> value to identify the case to add communication to. You can list a set of email addresses to copy on the communication using the <code>ccEmailAddresses</code> value. The <code>communicationBody</code> value contains the text of the communication.</p> <p>The response indicates the success or failure of the request.</p> <p>This operation implements a subset of the features of the AWS Support Center.</p>
    fn add_communication_to_case(
        &self,
        input: AddCommunicationToCaseRequest,
    ) -> RusotoFuture<AddCommunicationToCaseResponse, AddCommunicationToCaseError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.AddCommunicationToCase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddCommunicationToCaseResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddCommunicationToCaseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new case in the AWS Support Center. This operation is modeled on the behavior of the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. Its parameters require you to specify the following information: </p> <ul> <li> <p> <b>issueType.</b> The type of issue for the case. You can specify either "customer-service" or "technical." If you do not indicate a value, the default is "technical." </p> </li> <li> <p> <b>serviceCode.</b> The code for an AWS service. You obtain the <code>serviceCode</code> by calling <a>DescribeServices</a>. </p> </li> <li> <p> <b>categoryCode.</b> The category for the service defined for the <code>serviceCode</code> value. You also obtain the category code for a service by calling <a>DescribeServices</a>. Each AWS service defines its own set of category codes. </p> </li> <li> <p> <b>severityCode.</b> A value that indicates the urgency of the case, which in turn determines the response time according to your service level agreement with AWS Support. You obtain the SeverityCode by calling <a>DescribeSeverityLevels</a>.</p> </li> <li> <p> <b>subject.</b> The <b>Subject</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>communicationBody.</b> The <b>Description</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>attachmentSetId.</b> The ID of a set of attachments that has been created by using <a>AddAttachmentsToSet</a>.</p> </li> <li> <p> <b>language.</b> The human language in which AWS Support handles the case. English and Japanese are currently supported.</p> </li> <li> <p> <b>ccEmailAddresses.</b> The AWS Support Center <b>CC</b> field on the <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. You can list email addresses to be copied on any correspondence about the case. The account that opens the case is already identified by passing the AWS Credentials in the HTTP POST method or in a method or function call from one of the programming languages supported by an <a href="http://aws.amazon.com/tools/">AWS SDK</a>. </p> </li> </ul> <note> <p>To add additional communication or attachments to an existing case, use <a>AddCommunicationToCase</a>.</p> </note> <p>A successful <a>CreateCase</a> request returns an AWS Support case number. Case numbers are used by the <a>DescribeCases</a> operation to retrieve existing AWS Support cases. </p>
    fn create_case(
        &self,
        input: CreateCaseRequest,
    ) -> RusotoFuture<CreateCaseResponse, CreateCaseError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.CreateCase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateCaseResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateCaseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the attachment that has the specified ID. Attachment IDs are generated by the case management system when you add an attachment to a case or case communication. Attachment IDs are returned in the <a>AttachmentDetails</a> objects that are returned by the <a>DescribeCommunications</a> operation.</p>
    fn describe_attachment(
        &self,
        input: DescribeAttachmentRequest,
    ) -> RusotoFuture<DescribeAttachmentResponse, DescribeAttachmentError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeAttachment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAttachmentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAttachmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns a list of cases that you specify by passing one or more case IDs. In addition, you can filter the cases by date by setting values for the <code>afterTime</code> and <code>beforeTime</code> request parameters. You can set values for the <code>includeResolvedCases</code> and <code>includeCommunications</code> request parameters to control how much information is returned. </p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>The response returns the following in JSON format:</p> <ul> <li> <p>One or more <a>CaseDetails</a> data types. </p> </li> <li> <p>One or more <code>nextToken</code> values, which specify where to paginate the returned records represented by the <code>CaseDetails</code> objects.</p> </li> </ul></p>
    fn describe_cases(
        &self,
        input: DescribeCasesRequest,
    ) -> RusotoFuture<DescribeCasesResponse, DescribeCasesError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeCases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCasesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCasesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns communications (and attachments) for one or more support cases. You can use the <code>afterTime</code> and <code>beforeTime</code> parameters to filter by date. You can use the <code>caseId</code> parameter to restrict the results to a particular case.</p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>You can use the <code>maxResults</code> and <code>nextToken</code> parameters to control the pagination of the result set. Set <code>maxResults</code> to the number of cases you want displayed on each page, and use <code>nextToken</code> to specify the resumption of pagination.</p>
    fn describe_communications(
        &self,
        input: DescribeCommunicationsRequest,
    ) -> RusotoFuture<DescribeCommunicationsResponse, DescribeCommunicationsError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeCommunications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCommunicationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCommunicationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the current list of AWS services and a list of service categories that applies to each one. You then use service names and categories in your <a>CreateCase</a> requests. Each AWS service has its own set of categories.</p> <p>The service codes and category codes correspond to the values that are displayed in the <b>Service</b> and <b>Category</b> drop-down lists on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. The values in those fields, however, do not necessarily match the service codes and categories returned by the <code>DescribeServices</code> request. Always use the service codes and categories obtained programmatically. This practice ensures that you always have the most recent set of service and category codes.</p>
    fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> RusotoFuture<DescribeServicesResponse, DescribeServicesError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeServices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeServicesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeServicesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the list of severity levels that you can assign to an AWS Support case. The severity level for a case is also a field in the <a>CaseDetails</a> data type included in any <a>CreateCase</a> request. </p>
    fn describe_severity_levels(
        &self,
        input: DescribeSeverityLevelsRequest,
    ) -> RusotoFuture<DescribeSeverityLevelsResponse, DescribeSeverityLevelsError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeSeverityLevels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeSeverityLevelsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSeverityLevelsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the refresh status of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and their refresh statuses cannot be retrieved by using this operation. Use of the <code>DescribeTrustedAdvisorCheckRefreshStatuses</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note></p>
    fn describe_trusted_advisor_check_refresh_statuses(
        &self,
        input: DescribeTrustedAdvisorCheckRefreshStatusesRequest,
    ) -> RusotoFuture<
        DescribeTrustedAdvisorCheckRefreshStatusesResponse,
        DescribeTrustedAdvisorCheckRefreshStatusesError,
    > {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorCheckRefreshStatuses",
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

                    serde_json::from_str::<DescribeTrustedAdvisorCheckRefreshStatusesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTrustedAdvisorCheckRefreshStatusesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the results of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains a <a>TrustedAdvisorCheckResult</a> object, which contains these three objects:</p> <ul> <li> <p> <a>TrustedAdvisorCategorySpecificSummary</a> </p> </li> <li> <p> <a>TrustedAdvisorResourceDetail</a> </p> </li> <li> <p> <a>TrustedAdvisorResourcesSummary</a> </p> </li> </ul> <p>In addition, the response contains these fields:</p> <ul> <li> <p> <b>status.</b> The alert status of the check: &quot;ok&quot; (green), &quot;warning&quot; (yellow), &quot;error&quot; (red), or &quot;not_available&quot;.</p> </li> <li> <p> <b>timestamp.</b> The time of the last refresh of the check.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    fn describe_trusted_advisor_check_result(
        &self,
        input: DescribeTrustedAdvisorCheckResultRequest,
    ) -> RusotoFuture<
        DescribeTrustedAdvisorCheckResultResponse,
        DescribeTrustedAdvisorCheckResultError,
    > {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorCheckResult",
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

                    serde_json::from_str::<DescribeTrustedAdvisorCheckResultResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTrustedAdvisorCheckResultError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the summaries of the results of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains an array of <a>TrustedAdvisorCheckSummary</a> objects.</p>
    fn describe_trusted_advisor_check_summaries(
        &self,
        input: DescribeTrustedAdvisorCheckSummariesRequest,
    ) -> RusotoFuture<
        DescribeTrustedAdvisorCheckSummariesResponse,
        DescribeTrustedAdvisorCheckSummariesError,
    > {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorCheckSummaries",
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

                    serde_json::from_str::<DescribeTrustedAdvisorCheckSummariesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTrustedAdvisorCheckSummariesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all available Trusted Advisor checks, including name, ID, category, description, and metadata. You must specify a language code; English ("en") and Japanese ("ja") are currently supported. The response contains a <a>TrustedAdvisorCheckDescription</a> for each check.</p>
    fn describe_trusted_advisor_checks(
        &self,
        input: DescribeTrustedAdvisorChecksRequest,
    ) -> RusotoFuture<DescribeTrustedAdvisorChecksResponse, DescribeTrustedAdvisorChecksError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorChecks",
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

                    serde_json::from_str::<DescribeTrustedAdvisorChecksResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTrustedAdvisorChecksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Requests a refresh of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and they cannot be refreshed by using this operation. Use of the <code>RefreshTrustedAdvisorCheck</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note> <p>The response contains a <a>TrustedAdvisorCheckRefreshStatus</a> object, which contains these fields:</p> <ul> <li> <p> <b>status.</b> The refresh status of the check: &quot;none&quot;, &quot;enqueued&quot;, &quot;processing&quot;, &quot;success&quot;, or &quot;abandoned&quot;.</p> </li> <li> <p> <b>millisUntilNextRefreshable.</b> The amount of time, in milliseconds, until the check is eligible for refresh.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    fn refresh_trusted_advisor_check(
        &self,
        input: RefreshTrustedAdvisorCheckRequest,
    ) -> RusotoFuture<RefreshTrustedAdvisorCheckResponse, RefreshTrustedAdvisorCheckError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.RefreshTrustedAdvisorCheck",
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

                    serde_json::from_str::<RefreshTrustedAdvisorCheckResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RefreshTrustedAdvisorCheckError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Takes a <code>caseId</code> and returns the initial state of the case along with the state of the case after the call to <a>ResolveCase</a> completed.</p>
    fn resolve_case(
        &self,
        input: ResolveCaseRequest,
    ) -> RusotoFuture<ResolveCaseResponse, ResolveCaseError> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.ResolveCase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ResolveCaseResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResolveCaseError::from_body(
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
