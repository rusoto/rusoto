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
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bytes::Bytes>,
    /// <p>The name of the attachment file.</p>
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

/// <p>The file name and ID of an attachment to a case communication. You can use the ID to retrieve the attachment with the <a>DescribeAttachment</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p><p>A JSON-formatted object that contains the metadata for a support case. It is contained the response from a <a>DescribeCases</a> request. <b>CaseDetails</b> contains the following fields:</p> <ul> <li> <p> <b>caseId.</b> The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i>.</p> </li> <li> <p> <b>categoryCode.</b> The category of problem for the AWS Support case. Corresponds to the CategoryCode values returned by a call to <a>DescribeServices</a>.</p> </li> <li> <p> <b>displayId.</b> The identifier for the case on pages in the AWS Support Center.</p> </li> <li> <p> <b>language.</b> The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English (&quot;en&quot;) and Japanese (&quot;ja&quot;). Language parameters must be passed explicitly for operations that take them.</p> </li> <li> <p> <b>recentCommunications.</b> One or more <a>Communication</a> objects. Fields of these objects are <code>attachments</code>, <code>body</code>, <code>caseId</code>, <code>submittedBy</code>, and <code>timeCreated</code>.</p> </li> <li> <p> <b>nextToken.</b> A resumption point for pagination.</p> </li> <li> <p> <b>serviceCode.</b> The identifier for the AWS service that corresponds to the service code defined in the call to <a>DescribeServices</a>.</p> </li> <li> <p> <b>severityCode.</b> The severity code assigned to the case. Contains one of the values returned by the call to <a>DescribeSeverityLevels</a>. The possible values are: <code>low</code>, <code>normal</code>, <code>high</code>, <code>urgent</code>, and <code>critical</code>.</p> </li> <li> <p> <b>status.</b> The status of the case in the AWS Support Center. The possible values are: <code>resolved</code>, <code>pending-customer-action</code>, <code>opened</code>, <code>unassigned</code>, and <code>work-in-progress</code>.</p> </li> <li> <p> <b>subject.</b> The subject line of the case.</p> </li> <li> <p> <b>submittedBy.</b> The email address of the account that submitted the case.</p> </li> <li> <p> <b>timeCreated.</b> The time the case was created, in ISO-8601 format.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The code for the AWS service. You can get a list of codes and the corresponding service names by calling <a>DescribeServices</a>.</p>
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The code for the severity level returned by the call to <a>DescribeSeverityLevels</a>.</p>
    #[serde(rename = "severityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_code: Option<String>,
    /// <p>The status of the case. Valid values: <code>resolved</code> | <code>pending-customer-action</code> | <code>opened</code> | <code>unassigned</code> | <code>work-in-progress</code>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>A communication associated with an AWS Support case. The communication consists of the case ID, the message body, attachment information, the submitter of the communication, and the date and time of the communication.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The identity of the account that submitted, or responded to, the support case. Customer entries include the role or IAM user as well as the email address. For example, "AdminRole (Role) &lt;someone@example.com&gt;. Entries from the AWS Support team display "Amazon Web Services," and do not show an email address. </p>
    #[serde(rename = "submittedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    /// <p>The time the communication was created.</p>
    #[serde(rename = "timeCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p>The type of issue for the case. You can specify either &quot;customer-service&quot; or &quot;technical.&quot; If you do not indicate a value, the default is &quot;technical.&quot;</p> <note> <p>Service limit increases are not supported by the Support API; you must submit service limit increase requests in <a href="https://console.aws.amazon.com/support">Support Center</a>.</p> </note></p>
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
    /// <p><p>The code for the severity level returned by the call to <a>DescribeSeverityLevels</a>.</p> <note> <p>The availability of severity levels depends on the support plan for the account.</p> </note></p>
    #[serde(rename = "severityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_code: Option<String>,
    /// <p>The title of the AWS Support case.</p>
    #[serde(rename = "subject")]
    pub subject: String,
}

/// <p>The AWS Support case ID returned by a successful completion of the <a>CreateCase</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCaseResponse {
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAttachmentRequest {
    /// <p>The ID of the attachment to return. Attachment IDs are returned by the <a>DescribeCommunications</a> operation.</p>
    #[serde(rename = "attachmentId")]
    pub attachment_id: String,
}

/// <p>The content and file name of the attachment returned by the <a>DescribeAttachment</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAttachmentResponse {
    /// <p>The attachment content and file name.</p>
    #[serde(rename = "attachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServicesResponse {
    /// <p>A JSON-formatted list of AWS services.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSeverityLevelsRequest {
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// <p>The list of severity levels returned by the <a>DescribeSeverityLevels</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSeverityLevelsResponse {
    /// <p>The available severity levels for the support case. Available severity levels are defined by your service level agreement with AWS.</p>
    #[serde(rename = "severityLevels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_levels: Option<Vec<SeverityLevel>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrustedAdvisorCheckRefreshStatusesRequest {
    /// <p>The IDs of the Trusted Advisor checks to get the status of. <b>Note:</b> Specifying the check ID of a check that is automatically refreshed causes an <code>InvalidParameterValue</code> error.</p>
    #[serde(rename = "checkIds")]
    pub check_ids: Vec<String>,
}

/// <p>The statuses of the Trusted Advisor checks returned by the <a>DescribeTrustedAdvisorCheckRefreshStatuses</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrustedAdvisorCheckRefreshStatusesResponse {
    /// <p>The refresh status of the specified Trusted Advisor checks.</p>
    #[serde(rename = "statuses")]
    pub statuses: Vec<TrustedAdvisorCheckRefreshStatus>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrustedAdvisorCheckResultResponse {
    /// <p>The detailed results of the Trusted Advisor check.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TrustedAdvisorCheckResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrustedAdvisorCheckSummariesRequest {
    /// <p>The IDs of the Trusted Advisor checks.</p>
    #[serde(rename = "checkIds")]
    pub check_ids: Vec<String>,
}

/// <p>The summaries of the Trusted Advisor checks returned by the <a>DescribeTrustedAdvisorCheckSummaries</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrustedAdvisorCheckSummariesResponse {
    /// <p>The summary information for the requested Trusted Advisor checks.</p>
    #[serde(rename = "summaries")]
    pub summaries: Vec<TrustedAdvisorCheckSummary>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrustedAdvisorChecksRequest {
    /// <p>The ISO 639-1 code for the language in which AWS provides support. AWS Support currently supports English ("en") and Japanese ("ja"). Language parameters must be passed explicitly for operations that take them.</p>
    #[serde(rename = "language")]
    pub language: String,
}

/// <p>Information about the Trusted Advisor checks returned by the <a>DescribeTrustedAdvisorChecks</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrustedAdvisorChecksResponse {
    /// <p>Information about all available Trusted Advisor checks.</p>
    #[serde(rename = "checks")]
    pub checks: Vec<TrustedAdvisorCheckDescription>,
}

/// <p>The five most recent communications associated with the case.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RefreshTrustedAdvisorCheckRequest {
    /// <p>The unique identifier for the Trusted Advisor check to refresh. <b>Note:</b> Specifying the check ID of a check that is automatically refreshed causes an <code>InvalidParameterValue</code> error.</p>
    #[serde(rename = "checkId")]
    pub check_id: String,
}

/// <p>The current refresh status of a Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RefreshTrustedAdvisorCheckResponse {
    /// <p>The current refresh status for a check, including the amount of time until the check is eligible for refresh.</p>
    #[serde(rename = "status")]
    pub status: TrustedAdvisorCheckRefreshStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResolveCaseRequest {
    /// <p>The AWS Support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    #[serde(rename = "caseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
}

/// <p>The status of the case returned by the <a>ResolveCase</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Information about an AWS service returned by the <a>DescribeServices</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>A code and name pair that represents the severity level of a support case. The available values depend on the support plan for the account. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/getting-started.html#choosing-severity">Choosing a Severity</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SeverityLevel {
    /// <p>The code for case severity level.</p> <p>Valid values: <code>low</code> | <code>normal</code> | <code>high</code> | <code>urgent</code> | <code>critical</code> </p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The name of the severity level that corresponds to the severity level code.</p> <note> <p>The values returned by the API differ from the values that are displayed in the AWS Support Center. For example, for the code "low", the API name is "Low", but the name in the Support Center is "General guidance". These are the Support Center code/name mappings:</p> <ul> <li> <p> <code>low</code>: General guidance</p> </li> <li> <p> <code>normal</code>: System impaired</p> </li> <li> <p> <code>high</code>: Production system impaired</p> </li> <li> <p> <code>urgent</code>: Production system down</p> </li> <li> <p> <code>critical</code>: Business-critical system down</p> </li> </ul> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/getting-started.html#choosing-severity">Choosing a Severity</a> </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The container for summary information that relates to the category of the Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrustedAdvisorCategorySpecificSummary {
    /// <p>The summary information about cost savings for a Trusted Advisor check that is in the Cost Optimizing category.</p>
    #[serde(rename = "costOptimizing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_optimizing: Option<TrustedAdvisorCostOptimizingSummary>,
}

/// <p>The description and metadata for a Trusted Advisor check.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrustedAdvisorCheckRefreshStatus {
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    #[serde(rename = "checkId")]
    pub check_id: String,
    /// <p>The amount of time, in milliseconds, until the Trusted Advisor check is eligible for refresh.</p>
    #[serde(rename = "millisUntilNextRefreshable")]
    pub millis_until_next_refreshable: i64,
    /// <p><p>The status of the Trusted Advisor check for which a refresh has been requested: </p> <ul> <li> <p> <code>none:</code> The check is not refreshed or the non-success status exceeds the timeout</p> </li> <li> <p> <code>enqueued:</code> The check refresh requests has entered the refresh queue</p> </li> <li> <p> <code>processing:</code> The check refresh request is picked up by the rule processing engine</p> </li> <li> <p> <code>success:</code> The check is successfully refreshed</p> </li> <li> <p> <code>abandoned:</code> The check refresh has failed</p> </li> </ul></p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>The results of a Trusted Advisor check returned by <a>DescribeTrustedAdvisorCheckResult</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrustedAdvisorResourceDetail {
    /// <p>Specifies whether the AWS resource was ignored by Trusted Advisor because it was marked as suppressed by the user.</p>
    #[serde(rename = "isSuppressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
    /// <p>Additional information about the identified resource. The exact metadata and its order can be obtained by inspecting the <a>TrustedAdvisorCheckDescription</a> object returned by the call to <a>DescribeTrustedAdvisorChecks</a>. <b>Metadata</b> contains all the data that is shown in the Excel download, even in those cases where the UI shows just summary data.</p>
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

/// <p>Details about AWS resources that were analyzed in a call to Trusted Advisor <a>DescribeTrustedAdvisorCheckSummaries</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl AddAttachmentsToSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddAttachmentsToSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AttachmentLimitExceeded" => {
                    return RusotoError::Service(AddAttachmentsToSetError::AttachmentLimitExceeded(
                        err.msg,
                    ))
                }
                "AttachmentSetExpired" => {
                    return RusotoError::Service(AddAttachmentsToSetError::AttachmentSetExpired(
                        err.msg,
                    ))
                }
                "AttachmentSetIdNotFound" => {
                    return RusotoError::Service(AddAttachmentsToSetError::AttachmentSetIdNotFound(
                        err.msg,
                    ))
                }
                "AttachmentSetSizeLimitExceeded" => {
                    return RusotoError::Service(
                        AddAttachmentsToSetError::AttachmentSetSizeLimitExceeded(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(AddAttachmentsToSetError::InternalServerError(
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
impl fmt::Display for AddAttachmentsToSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddAttachmentsToSetError::AttachmentLimitExceeded(ref cause) => write!(f, "{}", cause),
            AddAttachmentsToSetError::AttachmentSetExpired(ref cause) => write!(f, "{}", cause),
            AddAttachmentsToSetError::AttachmentSetIdNotFound(ref cause) => write!(f, "{}", cause),
            AddAttachmentsToSetError::AttachmentSetSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AddAttachmentsToSetError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddAttachmentsToSetError {}
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
}

impl AddCommunicationToCaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddCommunicationToCaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AttachmentSetExpired" => {
                    return RusotoError::Service(AddCommunicationToCaseError::AttachmentSetExpired(
                        err.msg,
                    ))
                }
                "AttachmentSetIdNotFound" => {
                    return RusotoError::Service(
                        AddCommunicationToCaseError::AttachmentSetIdNotFound(err.msg),
                    )
                }
                "CaseIdNotFound" => {
                    return RusotoError::Service(AddCommunicationToCaseError::CaseIdNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(AddCommunicationToCaseError::InternalServerError(
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
impl fmt::Display for AddCommunicationToCaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddCommunicationToCaseError::AttachmentSetExpired(ref cause) => write!(f, "{}", cause),
            AddCommunicationToCaseError::AttachmentSetIdNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AddCommunicationToCaseError::CaseIdNotFound(ref cause) => write!(f, "{}", cause),
            AddCommunicationToCaseError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddCommunicationToCaseError {}
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
}

impl CreateCaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AttachmentSetExpired" => {
                    return RusotoError::Service(CreateCaseError::AttachmentSetExpired(err.msg))
                }
                "AttachmentSetIdNotFound" => {
                    return RusotoError::Service(CreateCaseError::AttachmentSetIdNotFound(err.msg))
                }
                "CaseCreationLimitExceeded" => {
                    return RusotoError::Service(CreateCaseError::CaseCreationLimitExceeded(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateCaseError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCaseError::AttachmentSetExpired(ref cause) => write!(f, "{}", cause),
            CreateCaseError::AttachmentSetIdNotFound(ref cause) => write!(f, "{}", cause),
            CreateCaseError::CaseCreationLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateCaseError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCaseError {}
/// Errors returned by DescribeAttachment
#[derive(Debug, PartialEq)]
pub enum DescribeAttachmentError {
    /// <p>An attachment with the specified ID could not be found.</p>
    AttachmentIdNotFound(String),
    /// <p>The limit for the number of <a>DescribeAttachment</a> requests in a short period of time has been exceeded.</p>
    DescribeAttachmentLimitExceeded(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeAttachmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAttachmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AttachmentIdNotFound" => {
                    return RusotoError::Service(DescribeAttachmentError::AttachmentIdNotFound(
                        err.msg,
                    ))
                }
                "DescribeAttachmentLimitExceeded" => {
                    return RusotoError::Service(
                        DescribeAttachmentError::DescribeAttachmentLimitExceeded(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeAttachmentError::InternalServerError(
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
impl fmt::Display for DescribeAttachmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAttachmentError::AttachmentIdNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAttachmentError::DescribeAttachmentLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAttachmentError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAttachmentError {}
/// Errors returned by DescribeCases
#[derive(Debug, PartialEq)]
pub enum DescribeCasesError {
    /// <p>The requested <code>caseId</code> could not be located.</p>
    CaseIdNotFound(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeCasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CaseIdNotFound" => {
                    return RusotoError::Service(DescribeCasesError::CaseIdNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeCasesError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCasesError::CaseIdNotFound(ref cause) => write!(f, "{}", cause),
            DescribeCasesError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCasesError {}
/// Errors returned by DescribeCommunications
#[derive(Debug, PartialEq)]
pub enum DescribeCommunicationsError {
    /// <p>The requested <code>caseId</code> could not be located.</p>
    CaseIdNotFound(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeCommunicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCommunicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CaseIdNotFound" => {
                    return RusotoError::Service(DescribeCommunicationsError::CaseIdNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeCommunicationsError::InternalServerError(
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
impl fmt::Display for DescribeCommunicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCommunicationsError::CaseIdNotFound(ref cause) => write!(f, "{}", cause),
            DescribeCommunicationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCommunicationsError {}
/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeServicesError::InternalServerError(
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
impl fmt::Display for DescribeServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServicesError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServicesError {}
/// Errors returned by DescribeSeverityLevels
#[derive(Debug, PartialEq)]
pub enum DescribeSeverityLevelsError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeSeverityLevelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSeverityLevelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeSeverityLevelsError::InternalServerError(
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
impl fmt::Display for DescribeSeverityLevelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSeverityLevelsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSeverityLevelsError {}
/// Errors returned by DescribeTrustedAdvisorCheckRefreshStatuses
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorCheckRefreshStatusesError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeTrustedAdvisorCheckRefreshStatusesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTrustedAdvisorCheckRefreshStatusesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeTrustedAdvisorCheckRefreshStatusesError::InternalServerError(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTrustedAdvisorCheckRefreshStatusesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrustedAdvisorCheckRefreshStatusesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTrustedAdvisorCheckRefreshStatusesError {}
/// Errors returned by DescribeTrustedAdvisorCheckResult
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorCheckResultError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeTrustedAdvisorCheckResultError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTrustedAdvisorCheckResultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeTrustedAdvisorCheckResultError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTrustedAdvisorCheckResultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrustedAdvisorCheckResultError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTrustedAdvisorCheckResultError {}
/// Errors returned by DescribeTrustedAdvisorCheckSummaries
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorCheckSummariesError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeTrustedAdvisorCheckSummariesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTrustedAdvisorCheckSummariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeTrustedAdvisorCheckSummariesError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTrustedAdvisorCheckSummariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrustedAdvisorCheckSummariesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTrustedAdvisorCheckSummariesError {}
/// Errors returned by DescribeTrustedAdvisorChecks
#[derive(Debug, PartialEq)]
pub enum DescribeTrustedAdvisorChecksError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl DescribeTrustedAdvisorChecksError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTrustedAdvisorChecksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeTrustedAdvisorChecksError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTrustedAdvisorChecksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrustedAdvisorChecksError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTrustedAdvisorChecksError {}
/// Errors returned by RefreshTrustedAdvisorCheck
#[derive(Debug, PartialEq)]
pub enum RefreshTrustedAdvisorCheckError {
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl RefreshTrustedAdvisorCheckError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RefreshTrustedAdvisorCheckError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        RefreshTrustedAdvisorCheckError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RefreshTrustedAdvisorCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RefreshTrustedAdvisorCheckError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RefreshTrustedAdvisorCheckError {}
/// Errors returned by ResolveCase
#[derive(Debug, PartialEq)]
pub enum ResolveCaseError {
    /// <p>The requested <code>caseId</code> could not be located.</p>
    CaseIdNotFound(String),
    /// <p>An internal server error occurred.</p>
    InternalServerError(String),
}

impl ResolveCaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResolveCaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CaseIdNotFound" => {
                    return RusotoError::Service(ResolveCaseError::CaseIdNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ResolveCaseError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResolveCaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolveCaseError::CaseIdNotFound(ref cause) => write!(f, "{}", cause),
            ResolveCaseError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResolveCaseError {}
/// Trait representing the capabilities of the AWS Support API. AWS Support clients implement this trait.
#[async_trait]
pub trait AWSSupport {
    /// <p>Adds one or more attachments to an attachment set. If an <code>attachmentSetId</code> is not specified, a new attachment set is created, and the ID of the set is returned in the response. If an <code>attachmentSetId</code> is specified, the attachments are added to the specified set, if it exists.</p> <p>An attachment set is a temporary container for attachments that are to be added to a case or case communication. The set is available for one hour after it is created; the <code>expiryTime</code> returned in the response indicates when the set expires. The maximum number of attachments in a set is 3, and the maximum size of any attachment in the set is 5 MB.</p>
    async fn add_attachments_to_set(
        &self,
        input: AddAttachmentsToSetRequest,
    ) -> Result<AddAttachmentsToSetResponse, RusotoError<AddAttachmentsToSetError>>;

    /// <p>Adds additional customer communication to an AWS Support case. You use the <code>caseId</code> value to identify the case to add communication to. You can list a set of email addresses to copy on the communication using the <code>ccEmailAddresses</code> value. The <code>communicationBody</code> value contains the text of the communication.</p> <p>The response indicates the success or failure of the request.</p> <p>This operation implements a subset of the features of the AWS Support Center.</p>
    async fn add_communication_to_case(
        &self,
        input: AddCommunicationToCaseRequest,
    ) -> Result<AddCommunicationToCaseResponse, RusotoError<AddCommunicationToCaseError>>;

    /// <p>Creates a new case in the AWS Support Center. This operation is modeled on the behavior of the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. Its parameters require you to specify the following information:</p> <ul> <li> <p> <b>issueType.</b> The type of issue for the case. You can specify either "customer-service" or "technical." If you do not indicate a value, the default is "technical."</p> <note> <p>Service limit increases are not supported by the Support API; you must submit service limit increase requests in <a href="https://console.aws.amazon.com/support">Support Center</a>.</p> <p>The <code>caseId</code> is not the <code>displayId</code> that appears in <a href="https://console.aws.amazon.com/support">Support Center</a>. You can use the <a>DescribeCases</a> API to get the <code>displayId</code>.</p> </note> </li> <li> <p> <b>serviceCode.</b> The code for an AWS service. You can get the possible <code>serviceCode</code> values by calling <a>DescribeServices</a>.</p> </li> <li> <p> <b>categoryCode.</b> The category for the service defined for the <code>serviceCode</code> value. You also get the category code for a service by calling <a>DescribeServices</a>. Each AWS service defines its own set of category codes.</p> </li> <li> <p> <b>severityCode.</b> A value that indicates the urgency of the case, which in turn determines the response time according to your service level agreement with AWS Support. You can get the possible <code>severityCode</code> values by calling <a>DescribeSeverityLevels</a>. For more information about the meaning of the codes, see <a>SeverityLevel</a> and <a href="https://docs.aws.amazon.com/awssupport/latest/user/getting-started.html#choosing-severity">Choosing a Severity</a>.</p> </li> <li> <p> <b>subject.</b> The <b>Subject</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>communicationBody.</b> The <b>Description</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>attachmentSetId.</b> The ID of a set of attachments that has been created by using <a>AddAttachmentsToSet</a>.</p> </li> <li> <p> <b>language.</b> The human language in which AWS Support handles the case. English and Japanese are currently supported.</p> </li> <li> <p> <b>ccEmailAddresses.</b> The AWS Support Center <b>CC</b> field on the <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. You can list email addresses to be copied on any correspondence about the case. The account that opens the case is already identified by passing the AWS Credentials in the HTTP POST method or in a method or function call from one of the programming languages supported by an <a href="http://aws.amazon.com/tools/">AWS SDK</a>. </p> </li> </ul> <note> <p>To add additional communication or attachments to an existing case, use <a>AddCommunicationToCase</a>.</p> </note> <p>A successful <a>CreateCase</a> request returns an AWS Support case number. Case numbers are used by the <a>DescribeCases</a> operation to retrieve existing AWS Support cases.</p>
    async fn create_case(
        &self,
        input: CreateCaseRequest,
    ) -> Result<CreateCaseResponse, RusotoError<CreateCaseError>>;

    /// <p>Returns the attachment that has the specified ID. Attachment IDs are generated by the case management system when you add an attachment to a case or case communication. Attachment IDs are returned in the <a>AttachmentDetails</a> objects that are returned by the <a>DescribeCommunications</a> operation.</p>
    async fn describe_attachment(
        &self,
        input: DescribeAttachmentRequest,
    ) -> Result<DescribeAttachmentResponse, RusotoError<DescribeAttachmentError>>;

    /// <p><p>Returns a list of cases that you specify by passing one or more case IDs. In addition, you can filter the cases by date by setting values for the <code>afterTime</code> and <code>beforeTime</code> request parameters. You can set values for the <code>includeResolvedCases</code> and <code>includeCommunications</code> request parameters to control how much information is returned.</p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>The response returns the following in JSON format:</p> <ul> <li> <p>One or more <a>CaseDetails</a> data types.</p> </li> <li> <p>One or more <code>nextToken</code> values, which specify where to paginate the returned records represented by the <code>CaseDetails</code> objects.</p> </li> </ul></p>
    async fn describe_cases(
        &self,
        input: DescribeCasesRequest,
    ) -> Result<DescribeCasesResponse, RusotoError<DescribeCasesError>>;

    /// <p>Returns communications (and attachments) for one or more support cases. You can use the <code>afterTime</code> and <code>beforeTime</code> parameters to filter by date. You can use the <code>caseId</code> parameter to restrict the results to a particular case.</p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>You can use the <code>maxResults</code> and <code>nextToken</code> parameters to control the pagination of the result set. Set <code>maxResults</code> to the number of cases you want displayed on each page, and use <code>nextToken</code> to specify the resumption of pagination.</p>
    async fn describe_communications(
        &self,
        input: DescribeCommunicationsRequest,
    ) -> Result<DescribeCommunicationsResponse, RusotoError<DescribeCommunicationsError>>;

    /// <p>Returns the current list of AWS services and a list of service categories that applies to each one. You then use service names and categories in your <a>CreateCase</a> requests. Each AWS service has its own set of categories.</p> <p>The service codes and category codes correspond to the values that are displayed in the <b>Service</b> and <b>Category</b> drop-down lists on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. The values in those fields, however, do not necessarily match the service codes and categories returned by the <code>DescribeServices</code> request. Always use the service codes and categories obtained programmatically. This practice ensures that you always have the most recent set of service and category codes.</p>
    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>>;

    /// <p>Returns the list of severity levels that you can assign to an AWS Support case. The severity level for a case is also a field in the <a>CaseDetails</a> data type included in any <a>CreateCase</a> request.</p>
    async fn describe_severity_levels(
        &self,
        input: DescribeSeverityLevelsRequest,
    ) -> Result<DescribeSeverityLevelsResponse, RusotoError<DescribeSeverityLevelsError>>;

    /// <p><p>Returns the refresh status of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and their refresh statuses cannot be retrieved by using this operation. Use of the <code>DescribeTrustedAdvisorCheckRefreshStatuses</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note></p>
    async fn describe_trusted_advisor_check_refresh_statuses(
        &self,
        input: DescribeTrustedAdvisorCheckRefreshStatusesRequest,
    ) -> Result<
        DescribeTrustedAdvisorCheckRefreshStatusesResponse,
        RusotoError<DescribeTrustedAdvisorCheckRefreshStatusesError>,
    >;

    /// <p><p>Returns the results of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains a <a>TrustedAdvisorCheckResult</a> object, which contains these three objects:</p> <ul> <li> <p> <a>TrustedAdvisorCategorySpecificSummary</a> </p> </li> <li> <p> <a>TrustedAdvisorResourceDetail</a> </p> </li> <li> <p> <a>TrustedAdvisorResourcesSummary</a> </p> </li> </ul> <p>In addition, the response contains these fields:</p> <ul> <li> <p> <b>status.</b> The alert status of the check: &quot;ok&quot; (green), &quot;warning&quot; (yellow), &quot;error&quot; (red), or &quot;not_available&quot;.</p> </li> <li> <p> <b>timestamp.</b> The time of the last refresh of the check.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    async fn describe_trusted_advisor_check_result(
        &self,
        input: DescribeTrustedAdvisorCheckResultRequest,
    ) -> Result<
        DescribeTrustedAdvisorCheckResultResponse,
        RusotoError<DescribeTrustedAdvisorCheckResultError>,
    >;

    /// <p>Returns the summaries of the results of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains an array of <a>TrustedAdvisorCheckSummary</a> objects.</p>
    async fn describe_trusted_advisor_check_summaries(
        &self,
        input: DescribeTrustedAdvisorCheckSummariesRequest,
    ) -> Result<
        DescribeTrustedAdvisorCheckSummariesResponse,
        RusotoError<DescribeTrustedAdvisorCheckSummariesError>,
    >;

    /// <p>Returns information about all available Trusted Advisor checks, including name, ID, category, description, and metadata. You must specify a language code; English ("en") and Japanese ("ja") are currently supported. The response contains a <a>TrustedAdvisorCheckDescription</a> for each check. The region must be set to us-east-1.</p>
    async fn describe_trusted_advisor_checks(
        &self,
        input: DescribeTrustedAdvisorChecksRequest,
    ) -> Result<DescribeTrustedAdvisorChecksResponse, RusotoError<DescribeTrustedAdvisorChecksError>>;

    /// <p><p>Requests a refresh of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and they cannot be refreshed by using this operation. Use of the <code>RefreshTrustedAdvisorCheck</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note> <p>The response contains a <a>TrustedAdvisorCheckRefreshStatus</a> object, which contains these fields:</p> <ul> <li> <p> <b>status.</b> The refresh status of the check: </p> <ul> <li> <p> <code>none:</code> The check is not refreshed or the non-success status exceeds the timeout</p> </li> <li> <p> <code>enqueued:</code> The check refresh requests has entered the refresh queue</p> </li> <li> <p> <code>processing:</code> The check refresh request is picked up by the rule processing engine</p> </li> <li> <p> <code>success:</code> The check is successfully refreshed</p> </li> <li> <p> <code>abandoned:</code> The check refresh has failed</p> </li> </ul> </li> <li> <p> <b>millisUntilNextRefreshable.</b> The amount of time, in milliseconds, until the check is eligible for refresh.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    async fn refresh_trusted_advisor_check(
        &self,
        input: RefreshTrustedAdvisorCheckRequest,
    ) -> Result<RefreshTrustedAdvisorCheckResponse, RusotoError<RefreshTrustedAdvisorCheckError>>;

    /// <p>Takes a <code>caseId</code> and returns the initial state of the case along with the state of the case after the call to <a>ResolveCase</a> completed.</p>
    async fn resolve_case(
        &self,
        input: ResolveCaseRequest,
    ) -> Result<ResolveCaseResponse, RusotoError<ResolveCaseError>>;
}
/// A client for the AWS Support API.
#[derive(Clone)]
pub struct AWSSupportClient {
    client: Client,
    region: region::Region,
}

impl AWSSupportClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AWSSupportClient {
        AWSSupportClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AWSSupportClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AWSSupportClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AWSSupportClient {
        AWSSupportClient { client, region }
    }
}

#[async_trait]
impl AWSSupport for AWSSupportClient {
    /// <p>Adds one or more attachments to an attachment set. If an <code>attachmentSetId</code> is not specified, a new attachment set is created, and the ID of the set is returned in the response. If an <code>attachmentSetId</code> is specified, the attachments are added to the specified set, if it exists.</p> <p>An attachment set is a temporary container for attachments that are to be added to a case or case communication. The set is available for one hour after it is created; the <code>expiryTime</code> returned in the response indicates when the set expires. The maximum number of attachments in a set is 3, and the maximum size of any attachment in the set is 5 MB.</p>
    async fn add_attachments_to_set(
        &self,
        input: AddAttachmentsToSetRequest,
    ) -> Result<AddAttachmentsToSetResponse, RusotoError<AddAttachmentsToSetError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.AddAttachmentsToSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AddAttachmentsToSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddAttachmentsToSetError::from_response(response))
        }
    }

    /// <p>Adds additional customer communication to an AWS Support case. You use the <code>caseId</code> value to identify the case to add communication to. You can list a set of email addresses to copy on the communication using the <code>ccEmailAddresses</code> value. The <code>communicationBody</code> value contains the text of the communication.</p> <p>The response indicates the success or failure of the request.</p> <p>This operation implements a subset of the features of the AWS Support Center.</p>
    async fn add_communication_to_case(
        &self,
        input: AddCommunicationToCaseRequest,
    ) -> Result<AddCommunicationToCaseResponse, RusotoError<AddCommunicationToCaseError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.AddCommunicationToCase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AddCommunicationToCaseResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddCommunicationToCaseError::from_response(response))
        }
    }

    /// <p>Creates a new case in the AWS Support Center. This operation is modeled on the behavior of the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. Its parameters require you to specify the following information:</p> <ul> <li> <p> <b>issueType.</b> The type of issue for the case. You can specify either "customer-service" or "technical." If you do not indicate a value, the default is "technical."</p> <note> <p>Service limit increases are not supported by the Support API; you must submit service limit increase requests in <a href="https://console.aws.amazon.com/support">Support Center</a>.</p> <p>The <code>caseId</code> is not the <code>displayId</code> that appears in <a href="https://console.aws.amazon.com/support">Support Center</a>. You can use the <a>DescribeCases</a> API to get the <code>displayId</code>.</p> </note> </li> <li> <p> <b>serviceCode.</b> The code for an AWS service. You can get the possible <code>serviceCode</code> values by calling <a>DescribeServices</a>.</p> </li> <li> <p> <b>categoryCode.</b> The category for the service defined for the <code>serviceCode</code> value. You also get the category code for a service by calling <a>DescribeServices</a>. Each AWS service defines its own set of category codes.</p> </li> <li> <p> <b>severityCode.</b> A value that indicates the urgency of the case, which in turn determines the response time according to your service level agreement with AWS Support. You can get the possible <code>severityCode</code> values by calling <a>DescribeSeverityLevels</a>. For more information about the meaning of the codes, see <a>SeverityLevel</a> and <a href="https://docs.aws.amazon.com/awssupport/latest/user/getting-started.html#choosing-severity">Choosing a Severity</a>.</p> </li> <li> <p> <b>subject.</b> The <b>Subject</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>communicationBody.</b> The <b>Description</b> field on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> </li> <li> <p> <b>attachmentSetId.</b> The ID of a set of attachments that has been created by using <a>AddAttachmentsToSet</a>.</p> </li> <li> <p> <b>language.</b> The human language in which AWS Support handles the case. English and Japanese are currently supported.</p> </li> <li> <p> <b>ccEmailAddresses.</b> The AWS Support Center <b>CC</b> field on the <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. You can list email addresses to be copied on any correspondence about the case. The account that opens the case is already identified by passing the AWS Credentials in the HTTP POST method or in a method or function call from one of the programming languages supported by an <a href="http://aws.amazon.com/tools/">AWS SDK</a>. </p> </li> </ul> <note> <p>To add additional communication or attachments to an existing case, use <a>AddCommunicationToCase</a>.</p> </note> <p>A successful <a>CreateCase</a> request returns an AWS Support case number. Case numbers are used by the <a>DescribeCases</a> operation to retrieve existing AWS Support cases.</p>
    async fn create_case(
        &self,
        input: CreateCaseRequest,
    ) -> Result<CreateCaseResponse, RusotoError<CreateCaseError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.CreateCase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateCaseResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCaseError::from_response(response))
        }
    }

    /// <p>Returns the attachment that has the specified ID. Attachment IDs are generated by the case management system when you add an attachment to a case or case communication. Attachment IDs are returned in the <a>AttachmentDetails</a> objects that are returned by the <a>DescribeCommunications</a> operation.</p>
    async fn describe_attachment(
        &self,
        input: DescribeAttachmentRequest,
    ) -> Result<DescribeAttachmentResponse, RusotoError<DescribeAttachmentError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeAttachment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAttachmentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAttachmentError::from_response(response))
        }
    }

    /// <p><p>Returns a list of cases that you specify by passing one or more case IDs. In addition, you can filter the cases by date by setting values for the <code>afterTime</code> and <code>beforeTime</code> request parameters. You can set values for the <code>includeResolvedCases</code> and <code>includeCommunications</code> request parameters to control how much information is returned.</p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>The response returns the following in JSON format:</p> <ul> <li> <p>One or more <a>CaseDetails</a> data types.</p> </li> <li> <p>One or more <code>nextToken</code> values, which specify where to paginate the returned records represented by the <code>CaseDetails</code> objects.</p> </li> </ul></p>
    async fn describe_cases(
        &self,
        input: DescribeCasesRequest,
    ) -> Result<DescribeCasesResponse, RusotoError<DescribeCasesError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeCases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeCasesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCasesError::from_response(response))
        }
    }

    /// <p>Returns communications (and attachments) for one or more support cases. You can use the <code>afterTime</code> and <code>beforeTime</code> parameters to filter by date. You can use the <code>caseId</code> parameter to restrict the results to a particular case.</p> <p>Case data is available for 12 months after creation. If a case was created more than 12 months ago, a request for data might cause an error.</p> <p>You can use the <code>maxResults</code> and <code>nextToken</code> parameters to control the pagination of the result set. Set <code>maxResults</code> to the number of cases you want displayed on each page, and use <code>nextToken</code> to specify the resumption of pagination.</p>
    async fn describe_communications(
        &self,
        input: DescribeCommunicationsRequest,
    ) -> Result<DescribeCommunicationsResponse, RusotoError<DescribeCommunicationsError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeCommunications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeCommunicationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCommunicationsError::from_response(response))
        }
    }

    /// <p>Returns the current list of AWS services and a list of service categories that applies to each one. You then use service names and categories in your <a>CreateCase</a> requests. Each AWS service has its own set of categories.</p> <p>The service codes and category codes correspond to the values that are displayed in the <b>Service</b> and <b>Category</b> drop-down lists on the AWS Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page. The values in those fields, however, do not necessarily match the service codes and categories returned by the <code>DescribeServices</code> request. Always use the service codes and categories obtained programmatically. This practice ensures that you always have the most recent set of service and category codes.</p>
    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeServices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeServicesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeServicesError::from_response(response))
        }
    }

    /// <p>Returns the list of severity levels that you can assign to an AWS Support case. The severity level for a case is also a field in the <a>CaseDetails</a> data type included in any <a>CreateCase</a> request.</p>
    async fn describe_severity_levels(
        &self,
        input: DescribeSeverityLevelsRequest,
    ) -> Result<DescribeSeverityLevelsResponse, RusotoError<DescribeSeverityLevelsError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.DescribeSeverityLevels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSeverityLevelsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSeverityLevelsError::from_response(response))
        }
    }

    /// <p><p>Returns the refresh status of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and their refresh statuses cannot be retrieved by using this operation. Use of the <code>DescribeTrustedAdvisorCheckRefreshStatuses</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note></p>
    async fn describe_trusted_advisor_check_refresh_statuses(
        &self,
        input: DescribeTrustedAdvisorCheckRefreshStatusesRequest,
    ) -> Result<
        DescribeTrustedAdvisorCheckRefreshStatusesResponse,
        RusotoError<DescribeTrustedAdvisorCheckRefreshStatusesError>,
    > {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorCheckRefreshStatuses",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTrustedAdvisorCheckRefreshStatusesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrustedAdvisorCheckRefreshStatusesError::from_response(response))
        }
    }

    /// <p><p>Returns the results of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains a <a>TrustedAdvisorCheckResult</a> object, which contains these three objects:</p> <ul> <li> <p> <a>TrustedAdvisorCategorySpecificSummary</a> </p> </li> <li> <p> <a>TrustedAdvisorResourceDetail</a> </p> </li> <li> <p> <a>TrustedAdvisorResourcesSummary</a> </p> </li> </ul> <p>In addition, the response contains these fields:</p> <ul> <li> <p> <b>status.</b> The alert status of the check: &quot;ok&quot; (green), &quot;warning&quot; (yellow), &quot;error&quot; (red), or &quot;not_available&quot;.</p> </li> <li> <p> <b>timestamp.</b> The time of the last refresh of the check.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    async fn describe_trusted_advisor_check_result(
        &self,
        input: DescribeTrustedAdvisorCheckResultRequest,
    ) -> Result<
        DescribeTrustedAdvisorCheckResultResponse,
        RusotoError<DescribeTrustedAdvisorCheckResultError>,
    > {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorCheckResult",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTrustedAdvisorCheckResultResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrustedAdvisorCheckResultError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the summaries of the results of the Trusted Advisor checks that have the specified check IDs. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <p>The response contains an array of <a>TrustedAdvisorCheckSummary</a> objects.</p>
    async fn describe_trusted_advisor_check_summaries(
        &self,
        input: DescribeTrustedAdvisorCheckSummariesRequest,
    ) -> Result<
        DescribeTrustedAdvisorCheckSummariesResponse,
        RusotoError<DescribeTrustedAdvisorCheckSummariesError>,
    > {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorCheckSummaries",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTrustedAdvisorCheckSummariesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrustedAdvisorCheckSummariesError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns information about all available Trusted Advisor checks, including name, ID, category, description, and metadata. You must specify a language code; English ("en") and Japanese ("ja") are currently supported. The response contains a <a>TrustedAdvisorCheckDescription</a> for each check. The region must be set to us-east-1.</p>
    async fn describe_trusted_advisor_checks(
        &self,
        input: DescribeTrustedAdvisorChecksRequest,
    ) -> Result<DescribeTrustedAdvisorChecksResponse, RusotoError<DescribeTrustedAdvisorChecksError>>
    {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.DescribeTrustedAdvisorChecks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTrustedAdvisorChecksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrustedAdvisorChecksError::from_response(response))
        }
    }

    /// <p><p>Requests a refresh of the Trusted Advisor check that has the specified check ID. Check IDs can be obtained by calling <a>DescribeTrustedAdvisorChecks</a>.</p> <note> <p>Some checks are refreshed automatically, and they cannot be refreshed by using this operation. Use of the <code>RefreshTrustedAdvisorCheck</code> operation for these checks causes an <code>InvalidParameterValue</code> error.</p> </note> <p>The response contains a <a>TrustedAdvisorCheckRefreshStatus</a> object, which contains these fields:</p> <ul> <li> <p> <b>status.</b> The refresh status of the check: </p> <ul> <li> <p> <code>none:</code> The check is not refreshed or the non-success status exceeds the timeout</p> </li> <li> <p> <code>enqueued:</code> The check refresh requests has entered the refresh queue</p> </li> <li> <p> <code>processing:</code> The check refresh request is picked up by the rule processing engine</p> </li> <li> <p> <code>success:</code> The check is successfully refreshed</p> </li> <li> <p> <code>abandoned:</code> The check refresh has failed</p> </li> </ul> </li> <li> <p> <b>millisUntilNextRefreshable.</b> The amount of time, in milliseconds, until the check is eligible for refresh.</p> </li> <li> <p> <b>checkId.</b> The unique identifier for the check.</p> </li> </ul></p>
    async fn refresh_trusted_advisor_check(
        &self,
        input: RefreshTrustedAdvisorCheckRequest,
    ) -> Result<RefreshTrustedAdvisorCheckResponse, RusotoError<RefreshTrustedAdvisorCheckError>>
    {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSSupport_20130415.RefreshTrustedAdvisorCheck",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<RefreshTrustedAdvisorCheckResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RefreshTrustedAdvisorCheckError::from_response(response))
        }
    }

    /// <p>Takes a <code>caseId</code> and returns the initial state of the case along with the state of the case after the call to <a>ResolveCase</a> completed.</p>
    async fn resolve_case(
        &self,
        input: ResolveCaseRequest,
    ) -> Result<ResolveCaseResponse, RusotoError<ResolveCaseError>> {
        let mut request = SignedRequest::new("POST", "support", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSSupport_20130415.ResolveCase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ResolveCaseResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ResolveCaseError::from_response(response))
        }
    }
}
