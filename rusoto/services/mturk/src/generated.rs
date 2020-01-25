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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptQualificationRequestRequest {
    /// <p> The value of the Qualification. You can omit this value if you are using the presence or absence of the Qualification as the basis for a HIT requirement. </p>
    #[serde(rename = "IntegerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<i64>,
    /// <p>The ID of the Qualification request, as returned by the <code>GetQualificationRequests</code> operation.</p>
    #[serde(rename = "QualificationRequestId")]
    pub qualification_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptQualificationRequestResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApproveAssignmentRequest {
    /// <p> The ID of the assignment. The assignment must correspond to a HIT created by the Requester. </p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
    /// <p> A flag indicating that an assignment should be approved even if it was previously rejected. Defaults to <code>False</code>. </p>
    #[serde(rename = "OverrideRejection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_rejection: Option<bool>,
    /// <p> A message for the Worker, which the Worker can see in the Status section of the web site. </p>
    #[serde(rename = "RequesterFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_feedback: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApproveAssignmentResponse {}

/// <p> The Assignment data structure represents a single assignment of a HIT to a Worker. The assignment tracks the Worker's efforts to complete the HIT, and contains the results for later retrieval. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Assignment {
    /// <p> The date and time the Worker accepted the assignment.</p>
    #[serde(rename = "AcceptTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_time: Option<f64>,
    /// <p> The Worker's answers submitted for the HIT contained in a QuestionFormAnswers document, if the Worker provides an answer. If the Worker does not provide any answers, Answer may contain a QuestionFormAnswers document, or Answer may be empty.</p>
    #[serde(rename = "Answer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// <p> If the Worker has submitted results and the Requester has approved the results, ApprovalTime is the date and time the Requester approved the results. This value is omitted from the assignment if the Requester has not yet approved the results.</p>
    #[serde(rename = "ApprovalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_time: Option<f64>,
    /// <p> A unique identifier for the assignment.</p>
    #[serde(rename = "AssignmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    /// <p> The status of the assignment.</p>
    #[serde(rename = "AssignmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    /// <p> If results have been submitted, AutoApprovalTime is the date and time the results of the assignment results are considered Approved automatically if they have not already been explicitly approved or rejected by the Requester. This value is derived from the auto-approval delay specified by the Requester in the HIT. This value is omitted from the assignment if the Worker has not yet submitted results.</p>
    #[serde(rename = "AutoApprovalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_approval_time: Option<f64>,
    /// <p> The date and time of the deadline for the assignment. This value is derived from the deadline specification for the HIT and the date and time the Worker accepted the HIT.</p>
    #[serde(rename = "Deadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<f64>,
    /// <p> The ID of the HIT.</p>
    #[serde(rename = "HITId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_id: Option<String>,
    /// <p> If the Worker has submitted results and the Requester has rejected the results, RejectionTime is the date and time the Requester rejected the results.</p>
    #[serde(rename = "RejectionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_time: Option<f64>,
    /// <p> The feedback string included with the call to the ApproveAssignment operation or the RejectAssignment operation, if the Requester approved or rejected the assignment and specified feedback.</p>
    #[serde(rename = "RequesterFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_feedback: Option<String>,
    /// <p> If the Worker has submitted results, SubmitTime is the date and time the assignment was submitted. This value is omitted from the assignment if the Worker has not yet submitted results.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    /// <p> The ID of the Worker who accepted the HIT.</p>
    #[serde(rename = "WorkerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateQualificationWithWorkerRequest {
    /// <p>The value of the Qualification to assign.</p>
    #[serde(rename = "IntegerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<i64>,
    /// <p>The ID of the Qualification type to use for the assigned Qualification.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
    /// <p> Specifies whether to send a notification email message to the Worker saying that the qualification was assigned to the Worker. Note: this is true by default. </p>
    #[serde(rename = "SendNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_notification: Option<bool>,
    /// <p> The ID of the Worker to whom the Qualification is being assigned. Worker IDs are included with submitted HIT assignments and Qualification requests. </p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateQualificationWithWorkerResponse {}

/// <p>An object representing a Bonus payment paid to a Worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BonusPayment {
    /// <p>The ID of the assignment associated with this bonus payment.</p>
    #[serde(rename = "AssignmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[serde(rename = "BonusAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_amount: Option<String>,
    /// <p>The date and time of when the bonus was granted.</p>
    #[serde(rename = "GrantTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_time: Option<f64>,
    /// <p>The Reason text given when the bonus was granted, if any.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the Worker to whom the bonus was paid.</p>
    #[serde(rename = "WorkerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAdditionalAssignmentsForHITRequest {
    /// <p>The ID of the HIT to extend.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
    /// <p>The number of additional assignments to request for this HIT.</p>
    #[serde(rename = "NumberOfAdditionalAssignments")]
    pub number_of_additional_assignments: i64,
    /// <p> A unique identifier for this request, which allows you to retry the call on error without extending the HIT multiple times. This is useful in cases such as network timeouts where it is unclear whether or not the call succeeded on the server. If the extend HIT already exists in the system from a previous call using the same <code>UniqueRequestToken</code>, subsequent calls will return an error with a message containing the request ID. </p>
    #[serde(rename = "UniqueRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_request_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAdditionalAssignmentsForHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHITRequest {
    /// <p> The amount of time, in seconds, that a Worker has to complete the HIT after accepting it. If a Worker does not complete the assignment within the specified duration, the assignment is considered abandoned. If the HIT is still active (that is, its lifetime has not elapsed), the assignment becomes available for other users to find and accept. </p>
    #[serde(rename = "AssignmentDurationInSeconds")]
    pub assignment_duration_in_seconds: i64,
    /// <p> The Assignment-level Review Policy applies to the assignments under the HIT. You can specify for Mechanical Turk to take various actions based on the policy. </p>
    #[serde(rename = "AssignmentReviewPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_review_policy: Option<ReviewPolicy>,
    /// <p> The number of seconds after an assignment for the HIT has been submitted, after which the assignment is considered Approved automatically unless the Requester explicitly rejects it. </p>
    #[serde(rename = "AutoApprovalDelayInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_approval_delay_in_seconds: Option<i64>,
    /// <p> A general description of the HIT. A description includes detailed information about the kind of task the HIT contains. On the Amazon Mechanical Turk web site, the HIT description appears in the expanded view of search results, and in the HIT and assignment screens. A good description gives the user enough information to evaluate the HIT before accepting it. </p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p> The HITLayoutId allows you to use a pre-existing HIT design with placeholder values and create an additional HIT by providing those values as HITLayoutParameters. </p> <p> Constraints: Either a Question parameter or a HITLayoutId parameter must be provided. </p>
    #[serde(rename = "HITLayoutId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_layout_id: Option<String>,
    /// <p> If the HITLayoutId is provided, any placeholder values must be filled in with values using the HITLayoutParameter structure. For more information, see HITLayout. </p>
    #[serde(rename = "HITLayoutParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_layout_parameters: Option<Vec<HITLayoutParameter>>,
    /// <p> The HIT-level Review Policy applies to the HIT. You can specify for Mechanical Turk to take various actions based on the policy. </p>
    #[serde(rename = "HITReviewPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_review_policy: Option<ReviewPolicy>,
    /// <p> One or more words or phrases that describe the HIT, separated by commas. These words are used in searches to find HITs. </p>
    #[serde(rename = "Keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// <p> An amount of time, in seconds, after which the HIT is no longer available for users to accept. After the lifetime of the HIT elapses, the HIT no longer appears in HIT searches, even if not all of the assignments for the HIT have been accepted. </p>
    #[serde(rename = "LifetimeInSeconds")]
    pub lifetime_in_seconds: i64,
    /// <p> The number of times the HIT can be accepted and completed before the HIT becomes unavailable. </p>
    #[serde(rename = "MaxAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_assignments: Option<i64>,
    /// <p> Conditions that a Worker's Qualifications must meet in order to accept the HIT. A HIT can have between zero and ten Qualification requirements. All requirements must be met in order for a Worker to accept the HIT. Additionally, other actions can be restricted using the <code>ActionsGuarded</code> field on each <code>QualificationRequirement</code> structure. </p>
    #[serde(rename = "QualificationRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_requirements: Option<Vec<QualificationRequirement>>,
    /// <p> The data the person completing the HIT uses to produce the results. </p> <p> Constraints: Must be a QuestionForm data structure, an ExternalQuestion data structure, or an HTMLQuestion data structure. The XML question data must not be larger than 64 kilobytes (65,535 bytes) in size, including whitespace. </p> <p>Either a Question parameter or a HITLayoutId parameter must be provided.</p>
    #[serde(rename = "Question")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    /// <p> An arbitrary data field. The RequesterAnnotation parameter lets your application attach arbitrary data to the HIT for tracking purposes. For example, this parameter could be an identifier internal to the Requester's application that corresponds with the HIT. </p> <p> The RequesterAnnotation parameter for a HIT is only visible to the Requester who created the HIT. It is not shown to the Worker, or any other Requester. </p> <p> The RequesterAnnotation parameter may be different for each HIT you submit. It does not affect how your HITs are grouped. </p>
    #[serde(rename = "RequesterAnnotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_annotation: Option<String>,
    /// <p> The amount of money the Requester will pay a Worker for successfully completing the HIT. </p>
    #[serde(rename = "Reward")]
    pub reward: String,
    /// <p> The title of the HIT. A title should be short and descriptive about the kind of task the HIT contains. On the Amazon Mechanical Turk web site, the HIT title appears in search results, and everywhere the HIT is mentioned. </p>
    #[serde(rename = "Title")]
    pub title: String,
    /// <p><p> A unique identifier for this request which allows you to retry the call on error without creating duplicate HITs. This is useful in cases such as network timeouts where it is unclear whether or not the call succeeded on the server. If the HIT already exists in the system from a previous call using the same UniqueRequestToken, subsequent calls will return a AWS.MechanicalTurk.HitAlreadyExists error with a message containing the HITId. </p> <note> <p> Note: It is your responsibility to ensure uniqueness of the token. The unique token expires after 24 hours. Subsequent calls using the same UniqueRequestToken made after the 24 hour limit could create duplicate HITs. </p> </note></p>
    #[serde(rename = "UniqueRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_request_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHITResponse {
    /// <p> Contains the newly created HIT data. For a description of the HIT data structure as it appears in responses, see the HIT Data Structure documentation. </p>
    #[serde(rename = "HIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<HIT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHITTypeRequest {
    /// <p> The amount of time, in seconds, that a Worker has to complete the HIT after accepting it. If a Worker does not complete the assignment within the specified duration, the assignment is considered abandoned. If the HIT is still active (that is, its lifetime has not elapsed), the assignment becomes available for other users to find and accept. </p>
    #[serde(rename = "AssignmentDurationInSeconds")]
    pub assignment_duration_in_seconds: i64,
    /// <p> The number of seconds after an assignment for the HIT has been submitted, after which the assignment is considered Approved automatically unless the Requester explicitly rejects it. </p>
    #[serde(rename = "AutoApprovalDelayInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_approval_delay_in_seconds: Option<i64>,
    /// <p> A general description of the HIT. A description includes detailed information about the kind of task the HIT contains. On the Amazon Mechanical Turk web site, the HIT description appears in the expanded view of search results, and in the HIT and assignment screens. A good description gives the user enough information to evaluate the HIT before accepting it. </p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p> One or more words or phrases that describe the HIT, separated by commas. These words are used in searches to find HITs. </p>
    #[serde(rename = "Keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// <p> Conditions that a Worker's Qualifications must meet in order to accept the HIT. A HIT can have between zero and ten Qualification requirements. All requirements must be met in order for a Worker to accept the HIT. Additionally, other actions can be restricted using the <code>ActionsGuarded</code> field on each <code>QualificationRequirement</code> structure. </p>
    #[serde(rename = "QualificationRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_requirements: Option<Vec<QualificationRequirement>>,
    /// <p> The amount of money the Requester will pay a Worker for successfully completing the HIT. </p>
    #[serde(rename = "Reward")]
    pub reward: String,
    /// <p> The title of the HIT. A title should be short and descriptive about the kind of task the HIT contains. On the Amazon Mechanical Turk web site, the HIT title appears in search results, and everywhere the HIT is mentioned. </p>
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHITTypeResponse {
    /// <p> The ID of the newly registered HIT type.</p>
    #[serde(rename = "HITTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_type_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHITWithHITTypeRequest {
    /// <p> The Assignment-level Review Policy applies to the assignments under the HIT. You can specify for Mechanical Turk to take various actions based on the policy. </p>
    #[serde(rename = "AssignmentReviewPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_review_policy: Option<ReviewPolicy>,
    /// <p> The HITLayoutId allows you to use a pre-existing HIT design with placeholder values and create an additional HIT by providing those values as HITLayoutParameters. </p> <p> Constraints: Either a Question parameter or a HITLayoutId parameter must be provided. </p>
    #[serde(rename = "HITLayoutId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_layout_id: Option<String>,
    /// <p> If the HITLayoutId is provided, any placeholder values must be filled in with values using the HITLayoutParameter structure. For more information, see HITLayout. </p>
    #[serde(rename = "HITLayoutParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_layout_parameters: Option<Vec<HITLayoutParameter>>,
    /// <p> The HIT-level Review Policy applies to the HIT. You can specify for Mechanical Turk to take various actions based on the policy. </p>
    #[serde(rename = "HITReviewPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_review_policy: Option<ReviewPolicy>,
    /// <p>The HIT type ID you want to create this HIT with.</p>
    #[serde(rename = "HITTypeId")]
    pub hit_type_id: String,
    /// <p> An amount of time, in seconds, after which the HIT is no longer available for users to accept. After the lifetime of the HIT elapses, the HIT no longer appears in HIT searches, even if not all of the assignments for the HIT have been accepted. </p>
    #[serde(rename = "LifetimeInSeconds")]
    pub lifetime_in_seconds: i64,
    /// <p> The number of times the HIT can be accepted and completed before the HIT becomes unavailable. </p>
    #[serde(rename = "MaxAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_assignments: Option<i64>,
    /// <p> The data the person completing the HIT uses to produce the results. </p> <p> Constraints: Must be a QuestionForm data structure, an ExternalQuestion data structure, or an HTMLQuestion data structure. The XML question data must not be larger than 64 kilobytes (65,535 bytes) in size, including whitespace. </p> <p>Either a Question parameter or a HITLayoutId parameter must be provided.</p>
    #[serde(rename = "Question")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    /// <p> An arbitrary data field. The RequesterAnnotation parameter lets your application attach arbitrary data to the HIT for tracking purposes. For example, this parameter could be an identifier internal to the Requester's application that corresponds with the HIT. </p> <p> The RequesterAnnotation parameter for a HIT is only visible to the Requester who created the HIT. It is not shown to the Worker, or any other Requester. </p> <p> The RequesterAnnotation parameter may be different for each HIT you submit. It does not affect how your HITs are grouped. </p>
    #[serde(rename = "RequesterAnnotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_annotation: Option<String>,
    /// <p><p> A unique identifier for this request which allows you to retry the call on error without creating duplicate HITs. This is useful in cases such as network timeouts where it is unclear whether or not the call succeeded on the server. If the HIT already exists in the system from a previous call using the same UniqueRequestToken, subsequent calls will return a AWS.MechanicalTurk.HitAlreadyExists error with a message containing the HITId. </p> <note> <p> Note: It is your responsibility to ensure uniqueness of the token. The unique token expires after 24 hours. Subsequent calls using the same UniqueRequestToken made after the 24 hour limit could create duplicate HITs. </p> </note></p>
    #[serde(rename = "UniqueRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_request_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHITWithHITTypeResponse {
    /// <p> Contains the newly created HIT data. For a description of the HIT data structure as it appears in responses, see the HIT Data Structure documentation. </p>
    #[serde(rename = "HIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<HIT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateQualificationTypeRequest {
    /// <p>The answers to the Qualification test specified in the Test parameter, in the form of an AnswerKey data structure.</p> <p>Constraints: Must not be longer than 65535 bytes.</p> <p>Constraints: None. If not specified, you must process Qualification requests manually.</p>
    #[serde(rename = "AnswerKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_key: Option<String>,
    /// <p>Specifies whether requests for the Qualification type are granted immediately, without prompting the Worker with a Qualification test.</p> <p>Constraints: If the Test parameter is specified, this parameter cannot be true.</p>
    #[serde(rename = "AutoGranted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_granted: Option<bool>,
    /// <p>The Qualification value to use for automatically granted Qualifications. This parameter is used only if the AutoGranted parameter is true.</p>
    #[serde(rename = "AutoGrantedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_granted_value: Option<i64>,
    /// <p>A long description for the Qualification type. On the Amazon Mechanical Turk website, the long description is displayed when a Worker examines a Qualification type.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>One or more words or phrases that describe the Qualification type, separated by commas. The keywords of a type make the type easier to find during a search.</p>
    #[serde(rename = "Keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// <p> The name you give to the Qualification type. The type name is used to represent the Qualification to Workers, and to find the type using a Qualification type search. It must be unique across all of your Qualification types.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The initial status of the Qualification type.</p> <p>Constraints: Valid values are: Active | Inactive</p>
    #[serde(rename = "QualificationTypeStatus")]
    pub qualification_type_status: String,
    /// <p>The number of seconds that a Worker must wait after requesting a Qualification of the Qualification type before the worker can retry the Qualification request.</p> <p>Constraints: None. If not specified, retries are disabled and Workers can request a Qualification of this type only once, even if the Worker has not been granted the Qualification. It is not possible to disable retries for a Qualification type after it has been created with retries enabled. If you want to disable retries, you must delete existing retry-enabled Qualification type and then create a new Qualification type with retries disabled.</p>
    #[serde(rename = "RetryDelayInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay_in_seconds: Option<i64>,
    /// <p> The questions for the Qualification test a Worker must answer correctly to obtain a Qualification of this type. If this parameter is specified, <code>TestDurationInSeconds</code> must also be specified. </p> <p>Constraints: Must not be longer than 65535 bytes. Must be a QuestionForm data structure. This parameter cannot be specified if AutoGranted is true.</p> <p>Constraints: None. If not specified, the Worker may request the Qualification without answering any questions.</p>
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    /// <p>The number of seconds the Worker has to complete the Qualification test, starting from the time the Worker requests the Qualification.</p>
    #[serde(rename = "TestDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_duration_in_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateQualificationTypeResponse {
    /// <p>The created Qualification type, returned as a QualificationType data structure.</p>
    #[serde(rename = "QualificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type: Option<QualificationType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWorkerBlockRequest {
    /// <p>A message explaining the reason for blocking the Worker. This parameter enables you to keep track of your Workers. The Worker does not see this message.</p>
    #[serde(rename = "Reason")]
    pub reason: String,
    /// <p>The ID of the Worker to block.</p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorkerBlockResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteHITRequest {
    /// <p>The ID of the HIT to be deleted.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteQualificationTypeRequest {
    /// <p>The ID of the QualificationType to dispose.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteQualificationTypeResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWorkerBlockRequest {
    /// <p>A message that explains the reason for unblocking the Worker. The Worker does not see this message.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the Worker to unblock.</p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWorkerBlockResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateQualificationFromWorkerRequest {
    /// <p>The ID of the Qualification type of the Qualification to be revoked.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
    /// <p>A text message that explains why the Qualification was revoked. The user who had the Qualification sees this message.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the Worker who possesses the Qualification to be revoked.</p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateQualificationFromWorkerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountBalanceRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccountBalanceResponse {
    #[serde(rename = "AvailableBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<String>,
    #[serde(rename = "OnHoldBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_balance: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAssignmentRequest {
    /// <p>The ID of the Assignment to be retrieved.</p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAssignmentResponse {
    /// <p> The assignment. The response includes one Assignment element. </p>
    #[serde(rename = "Assignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment: Option<Assignment>,
    /// <p> The HIT associated with this assignment. The response includes one HIT element.</p>
    #[serde(rename = "HIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<HIT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFileUploadURLRequest {
    /// <p>The ID of the assignment that contains the question with a FileUploadAnswer.</p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
    /// <p>The identifier of the question with a FileUploadAnswer, as specified in the QuestionForm of the HIT.</p>
    #[serde(rename = "QuestionIdentifier")]
    pub question_identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFileUploadURLResponse {
    /// <p> A temporary URL for the file that the Worker uploaded for the answer. </p>
    #[serde(rename = "FileUploadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_upload_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHITRequest {
    /// <p>The ID of the HIT to be retrieved.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetHITResponse {
    /// <p> Contains the requested HIT data.</p>
    #[serde(rename = "HIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<HIT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQualificationScoreRequest {
    /// <p>The ID of the QualificationType.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetQualificationScoreResponse {
    /// <p> The Qualification data structure of the Qualification assigned to a user, including the Qualification type and the value (score). </p>
    #[serde(rename = "Qualification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification: Option<Qualification>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQualificationTypeRequest {
    /// <p>The ID of the QualificationType.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetQualificationTypeResponse {
    /// <p> The returned Qualification Type</p>
    #[serde(rename = "QualificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type: Option<QualificationType>,
}

/// <p> The HIT data structure represents a single HIT, including all the information necessary for a Worker to accept and complete the HIT.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HIT {
    /// <p> The length of time, in seconds, that a Worker has to complete the HIT after accepting it.</p>
    #[serde(rename = "AssignmentDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_duration_in_seconds: Option<i64>,
    /// <p>The amount of time, in seconds, after the Worker submits an assignment for the HIT that the results are automatically approved by Amazon Mechanical Turk. This is the amount of time the Requester has to reject an assignment submitted by a Worker before the assignment is auto-approved and the Worker is paid. </p>
    #[serde(rename = "AutoApprovalDelayInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_approval_delay_in_seconds: Option<i64>,
    /// <p> The date and time the HIT was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p> A general description of the HIT.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date and time the HIT expires.</p>
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    /// <p> The ID of the HIT Group of this HIT.</p>
    #[serde(rename = "HITGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_group_id: Option<String>,
    /// <p> A unique identifier for the HIT.</p>
    #[serde(rename = "HITId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_id: Option<String>,
    /// <p> The ID of the HIT Layout of this HIT.</p>
    #[serde(rename = "HITLayoutId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_layout_id: Option<String>,
    /// <p> Indicates the review status of the HIT. Valid Values are NotReviewed | MarkedForReview | ReviewedAppropriate | ReviewedInappropriate.</p>
    #[serde(rename = "HITReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_review_status: Option<String>,
    /// <p>The status of the HIT and its assignments. Valid Values are Assignable | Unassignable | Reviewable | Reviewing | Disposed. </p>
    #[serde(rename = "HITStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_status: Option<String>,
    /// <p>The ID of the HIT type of this HIT</p>
    #[serde(rename = "HITTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_type_id: Option<String>,
    /// <p> One or more words or phrases that describe the HIT, separated by commas. Search terms similar to the keywords of a HIT are more likely to have the HIT in the search results.</p>
    #[serde(rename = "Keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// <p>The number of times the HIT can be accepted and completed before the HIT becomes unavailable. </p>
    #[serde(rename = "MaxAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_assignments: Option<i64>,
    /// <p> The number of assignments for this HIT that are available for Workers to accept.</p>
    #[serde(rename = "NumberOfAssignmentsAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_assignments_available: Option<i64>,
    /// <p> The number of assignments for this HIT that have been approved or rejected.</p>
    #[serde(rename = "NumberOfAssignmentsCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_assignments_completed: Option<i64>,
    /// <p> The number of assignments for this HIT that are being previewed or have been accepted by Workers, but have not yet been submitted, returned, or abandoned.</p>
    #[serde(rename = "NumberOfAssignmentsPending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_assignments_pending: Option<i64>,
    /// <p> Conditions that a Worker's Qualifications must meet in order to accept the HIT. A HIT can have between zero and ten Qualification requirements. All requirements must be met in order for a Worker to accept the HIT. Additionally, other actions can be restricted using the <code>ActionsGuarded</code> field on each <code>QualificationRequirement</code> structure. </p>
    #[serde(rename = "QualificationRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_requirements: Option<Vec<QualificationRequirement>>,
    /// <p> The data the Worker completing the HIT uses produce the results. This is either either a QuestionForm, HTMLQuestion or an ExternalQuestion data structure.</p>
    #[serde(rename = "Question")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    /// <p> An arbitrary data field the Requester who created the HIT can use. This field is visible only to the creator of the HIT.</p>
    #[serde(rename = "RequesterAnnotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_annotation: Option<String>,
    #[serde(rename = "Reward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<String>,
    /// <p> The title of the HIT.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p> The HITLayoutParameter data structure defines parameter values used with a HITLayout. A HITLayout is a reusable Amazon Mechanical Turk project template used to provide Human Intelligence Task (HIT) question data for CreateHIT. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HITLayoutParameter {
    /// <p> The name of the parameter in the HITLayout. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value substituted for the parameter referenced in the HITLayout. </p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAssignmentsForHITRequest {
    /// <p>The status of the assignments to return: Submitted | Approved | Rejected</p>
    #[serde(rename = "AssignmentStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_statuses: Option<Vec<String>>,
    /// <p>The ID of the HIT.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssignmentsForHITResponse {
    /// <p> The collection of Assignment data structures returned by this call.</p>
    #[serde(rename = "Assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<Assignment>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The number of assignments on the page in the filtered results list, equivalent to the number of assignments returned by this call.</p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBonusPaymentsRequest {
    /// <p>The ID of the assignment associated with the bonus payments to retrieve. If specified, only bonus payments for the given assignment are returned. Either the HITId parameter or the AssignmentId parameter must be specified</p>
    #[serde(rename = "AssignmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    /// <p>The ID of the HIT associated with the bonus payments to retrieve. If not specified, all bonus payments for all assignments for the given HIT are returned. Either the HITId parameter or the AssignmentId parameter must be specified</p>
    #[serde(rename = "HITId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBonusPaymentsResponse {
    /// <p>A successful request to the ListBonusPayments operation returns a list of BonusPayment objects. </p>
    #[serde(rename = "BonusPayments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_payments: Option<Vec<BonusPayment>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of bonus payments on this page in the filtered results list, equivalent to the number of bonus payments being returned by this call. </p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHITsForQualificationTypeRequest {
    /// <p> Limit the number of results returned. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination Token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The ID of the Qualification type to use when querying HITs. </p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHITsForQualificationTypeResponse {
    /// <p> The list of HIT elements returned by the query.</p>
    #[serde(rename = "HITs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hi_ts: Option<Vec<HIT>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The number of HITs on this page in the filtered results list, equivalent to the number of HITs being returned by this call. </p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHITsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHITsResponse {
    /// <p> The list of HIT elements returned by the query.</p>
    #[serde(rename = "HITs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hi_ts: Option<Vec<HIT>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of HITs on this page in the filtered results list, equivalent to the number of HITs being returned by this call.</p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQualificationRequestsRequest {
    /// <p> The maximum number of results to return in a single call. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the QualificationType.</p>
    #[serde(rename = "QualificationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListQualificationRequestsResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of Qualification requests on this page in the filtered results list, equivalent to the number of Qualification requests being returned by this call.</p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// <p>The Qualification request. The response includes one QualificationRequest element for each Qualification request returned by the query.</p>
    #[serde(rename = "QualificationRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_requests: Option<Vec<QualificationRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQualificationTypesRequest {
    /// <p> The maximum number of results to return in a single call. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Specifies that only Qualification types that the Requester created are returned. If false, the operation returns all Qualification types. </p>
    #[serde(rename = "MustBeOwnedByCaller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_be_owned_by_caller: Option<bool>,
    /// <p>Specifies that only Qualification types that a user can request through the Amazon Mechanical Turk web site, such as by taking a Qualification test, are returned as results of the search. Some Qualification types, such as those assigned automatically by the system, cannot be requested directly by users. If false, all Qualification types, including those managed by the system, are considered. Valid values are True | False. </p>
    #[serde(rename = "MustBeRequestable")]
    pub must_be_requestable: bool,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> A text query against all of the searchable attributes of Qualification types. </p>
    #[serde(rename = "Query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListQualificationTypesResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The number of Qualification types on this page in the filtered results list, equivalent to the number of types this operation returns. </p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// <p> The list of QualificationType elements returned by the query. </p>
    #[serde(rename = "QualificationTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_types: Option<Vec<QualificationType>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReviewPolicyResultsForHITRequest {
    /// <p>The unique identifier of the HIT to retrieve review results for.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
    /// <p>Limit the number of results returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The Policy Level(s) to retrieve review results for - HIT or Assignment. If omitted, the default behavior is to retrieve all data for both policy levels. For a list of all the described policies, see Review Policies. </p>
    #[serde(rename = "PolicyLevels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_levels: Option<Vec<String>>,
    /// <p> Specify if the operation should retrieve a list of the actions taken executing the Review Policies and their outcomes. </p>
    #[serde(rename = "RetrieveActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_actions: Option<bool>,
    /// <p> Specify if the operation should retrieve a list of the results computed by the Review Policies. </p>
    #[serde(rename = "RetrieveResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_results: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReviewPolicyResultsForHITResponse {
    /// <p> The name of the Assignment-level Review Policy. This contains only the PolicyName element. </p>
    #[serde(rename = "AssignmentReviewPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_review_policy: Option<ReviewPolicy>,
    /// <p> Contains both ReviewResult and ReviewAction elements for an Assignment. </p>
    #[serde(rename = "AssignmentReviewReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_review_report: Option<ReviewReport>,
    /// <p>The HITId of the HIT for which results have been returned.</p>
    #[serde(rename = "HITId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_id: Option<String>,
    /// <p>The name of the HIT-level Review Policy. This contains only the PolicyName element.</p>
    #[serde(rename = "HITReviewPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_review_policy: Option<ReviewPolicy>,
    /// <p>Contains both ReviewResult and ReviewAction elements for a particular HIT. </p>
    #[serde(rename = "HITReviewReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_review_report: Option<ReviewReport>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReviewableHITsRequest {
    /// <p> The ID of the HIT type of the HITs to consider for the query. If not specified, all HITs for the Reviewer are considered </p>
    #[serde(rename = "HITTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_type_id: Option<String>,
    /// <p> Limit the number of results returned. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination Token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> Can be either <code>Reviewable</code> or <code>Reviewing</code>. Reviewable is the default value. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReviewableHITsResponse {
    /// <p> The list of HIT elements returned by the query.</p>
    #[serde(rename = "HITs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hi_ts: Option<Vec<HIT>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The number of HITs on this page in the filtered results list, equivalent to the number of HITs being returned by this call. </p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorkerBlocksRequest {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorkerBlocksResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The number of assignments on the page in the filtered results list, equivalent to the number of assignments returned by this call.</p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// <p> The list of WorkerBlocks, containing the collection of Worker IDs and reasons for blocking.</p>
    #[serde(rename = "WorkerBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_blocks: Option<Vec<WorkerBlock>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorkersWithQualificationTypeRequest {
    /// <p> Limit the number of results returned. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination Token</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Qualification type of the Qualifications to return.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
    /// <p> The status of the Qualifications to return. Can be <code>Granted | Revoked</code>. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorkersWithQualificationTypeResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The number of Qualifications on this page in the filtered results list, equivalent to the number of Qualifications being returned by this call.</p>
    #[serde(rename = "NumResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// <p> The list of Qualification elements returned by this call. </p>
    #[serde(rename = "Qualifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<Vec<Qualification>>,
}

/// <p>The Locale data structure represents a geographical region or location.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locale {
    /// <p> The country of the locale. Must be a valid ISO 3166 country code. For example, the code US refers to the United States of America. </p>
    #[serde(rename = "Country")]
    pub country: String,
    /// <p>The state or subdivision of the locale. A valid ISO 3166-2 subdivision code. For example, the code WA refers to the state of Washington.</p>
    #[serde(rename = "Subdivision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision: Option<String>,
}

/// <p>The NotificationSpecification data structure describes a HIT event notification for a HIT type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotificationSpecification {
    /// <p><p> The target for notification messages. The Destination’s format is determined by the specified Transport: </p> <ul> <li> <p>When Transport is Email, the Destination is your email address.</p> </li> <li> <p>When Transport is SQS, the Destination is your queue URL.</p> </li> <li> <p>When Transport is SNS, the Destination is the ARN of your topic.</p> </li> </ul></p>
    #[serde(rename = "Destination")]
    pub destination: String,
    /// <p> The list of events that should cause notifications to be sent. Valid Values: AssignmentAccepted | AssignmentAbandoned | AssignmentReturned | AssignmentSubmitted | AssignmentRejected | AssignmentApproved | HITCreated | HITExtended | HITDisposed | HITReviewable | HITExpired | Ping. The Ping event is only valid for the SendTestEventNotification operation. </p>
    #[serde(rename = "EventTypes")]
    pub event_types: Vec<String>,
    /// <p> The method Amazon Mechanical Turk uses to send the notification. Valid Values: Email | SQS | SNS. </p>
    #[serde(rename = "Transport")]
    pub transport: String,
    /// <p>The version of the Notification API to use. Valid value is 2006-05-05.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

/// <p> When MTurk encounters an issue with notifying the Workers you specified, it returns back this object with failure details. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotifyWorkersFailureStatus {
    /// <p> Encoded value for the failure type. </p>
    #[serde(rename = "NotifyWorkersFailureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_workers_failure_code: Option<String>,
    /// <p> A message detailing the reason the Worker could not be notified. </p>
    #[serde(rename = "NotifyWorkersFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_workers_failure_message: Option<String>,
    /// <p> The ID of the Worker.</p>
    #[serde(rename = "WorkerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotifyWorkersRequest {
    /// <p>The text of the email message to send. Can include up to 4,096 characters</p>
    #[serde(rename = "MessageText")]
    pub message_text: String,
    /// <p>The subject line of the email message to send. Can include up to 200 characters.</p>
    #[serde(rename = "Subject")]
    pub subject: String,
    /// <p>A list of Worker IDs you wish to notify. You can notify upto 100 Workers at a time.</p>
    #[serde(rename = "WorkerIds")]
    pub worker_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotifyWorkersResponse {
    /// <p> When MTurk sends notifications to the list of Workers, it returns back any failures it encounters in this list of NotifyWorkersFailureStatus objects. </p>
    #[serde(rename = "NotifyWorkersFailureStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_workers_failure_statuses: Option<Vec<NotifyWorkersFailureStatus>>,
}

/// <p> This data structure is the data type for the AnswerKey parameter of the ScoreMyKnownAnswers/2011-09-01 Review Policy. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterMapEntry {
    /// <p> The QuestionID from the HIT that is used to identify which question requires Mechanical Turk to score as part of the ScoreMyKnownAnswers/2011-09-01 Review Policy. </p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p> The list of answers to the question specified in the MapEntry Key element. The Worker must match all values in order for the answer to be scored correctly. </p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p> Name of the parameter from the Review policy. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyParameter {
    /// <p> Name of the parameter from the list of Review Polices. </p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p> List of ParameterMapEntry objects. </p>
    #[serde(rename = "MapEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_entries: Option<Vec<ParameterMapEntry>>,
    /// <p> The list of values of the Parameter</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The Qualification data structure represents a Qualification assigned to a user, including the Qualification type and the value (score).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Qualification {
    /// <p> The date and time the Qualification was granted to the Worker. If the Worker's Qualification was revoked, and then re-granted based on a new Qualification request, GrantTime is the date and time of the last call to the AcceptQualificationRequest operation.</p>
    #[serde(rename = "GrantTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_time: Option<f64>,
    /// <p> The value (score) of the Qualification, if the Qualification has an integer value.</p>
    #[serde(rename = "IntegerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<i64>,
    #[serde(rename = "LocaleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_value: Option<Locale>,
    /// <p> The ID of the Qualification type for the Qualification.</p>
    #[serde(rename = "QualificationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type_id: Option<String>,
    /// <p> The status of the Qualification. Valid values are Granted | Revoked.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The ID of the Worker who possesses the Qualification. </p>
    #[serde(rename = "WorkerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}

/// <p> The QualificationRequest data structure represents a request a Worker has made for a Qualification. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QualificationRequest {
    /// <p> The Worker's answers for the Qualification type's test contained in a QuestionFormAnswers document, if the type has a test and the Worker has submitted answers. If the Worker does not provide any answers, Answer may be empty. </p>
    #[serde(rename = "Answer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// <p>The ID of the Qualification request, a unique identifier generated when the request was submitted. </p>
    #[serde(rename = "QualificationRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_request_id: Option<String>,
    /// <p> The ID of the Qualification type the Worker is requesting, as returned by the CreateQualificationType operation. </p>
    #[serde(rename = "QualificationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type_id: Option<String>,
    /// <p>The date and time the Qualification request had a status of Submitted. This is either the time the Worker submitted answers for a Qualification test, or the time the Worker requested the Qualification if the Qualification type does not have a test. </p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    /// <p> The contents of the Qualification test that was presented to the Worker, if the type has a test and the Worker has submitted answers. This value is identical to the QuestionForm associated with the Qualification type at the time the Worker requests the Qualification.</p>
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    /// <p> The ID of the Worker requesting the Qualification.</p>
    #[serde(rename = "WorkerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}

/// <p> The QualificationRequirement data structure describes a Qualification that a Worker must have before the Worker is allowed to accept a HIT. A requirement may optionally state that a Worker must have the Qualification in order to preview the HIT, or see the HIT in search results. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationRequirement {
    /// <p> Setting this attribute prevents Workers whose Qualifications do not meet this QualificationRequirement from taking the specified action. Valid arguments include "Accept" (Worker cannot accept the HIT, but can preview the HIT and see it in their search results), "PreviewAndAccept" (Worker cannot accept or preview the HIT, but can see the HIT in their search results), and "DiscoverPreviewAndAccept" (Worker cannot accept, preview, or see the HIT in their search results). It's possible for you to create a HIT with multiple QualificationRequirements (which can have different values for the ActionGuarded attribute). In this case, the Worker is only permitted to perform an action when they have met all QualificationRequirements guarding the action. The actions in the order of least restrictive to most restrictive are Discover, Preview and Accept. For example, if a Worker meets all QualificationRequirements that are set to DiscoverPreviewAndAccept, but do not meet all requirements that are set with PreviewAndAccept, then the Worker will be able to Discover, i.e. see the HIT in their search result, but will not be able to Preview or Accept the HIT. ActionsGuarded should not be used in combination with the <code>RequiredToPreview</code> field. </p>
    #[serde(rename = "ActionsGuarded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_guarded: Option<String>,
    /// <p>The kind of comparison to make against a Qualification's value. You can compare a Qualification's value to an IntegerValue to see if it is LessThan, LessThanOrEqualTo, GreaterThan, GreaterThanOrEqualTo, EqualTo, or NotEqualTo the IntegerValue. You can compare it to a LocaleValue to see if it is EqualTo, or NotEqualTo the LocaleValue. You can check to see if the value is In or NotIn a set of IntegerValue or LocaleValue values. Lastly, a Qualification requirement can also test if a Qualification Exists or DoesNotExist in the user's profile, regardless of its value. </p>
    #[serde(rename = "Comparator")]
    pub comparator: String,
    /// <p> The integer value to compare against the Qualification's value. IntegerValue must not be present if Comparator is Exists or DoesNotExist. IntegerValue can only be used if the Qualification type has an integer value; it cannot be used with the Worker_Locale QualificationType ID. When performing a set comparison by using the In or the NotIn comparator, you can use up to 15 IntegerValue elements in a QualificationRequirement data structure. </p>
    #[serde(rename = "IntegerValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_values: Option<Vec<i64>>,
    /// <p> The locale value to compare against the Qualification's value. The local value must be a valid ISO 3166 country code or supports ISO 3166-2 subdivisions. LocaleValue can only be used with a Worker_Locale QualificationType ID. LocaleValue can only be used with the EqualTo, NotEqualTo, In, and NotIn comparators. You must only use a single LocaleValue element when using the EqualTo or NotEqualTo comparators. When performing a set comparison by using the In or the NotIn comparator, you can use up to 30 LocaleValue elements in a QualificationRequirement data structure. </p>
    #[serde(rename = "LocaleValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_values: Option<Vec<Locale>>,
    /// <p> The ID of the Qualification type for the requirement.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
}

/// <p> The QualificationType data structure represents a Qualification type, a description of a property of a Worker that must match the requirements of a HIT for the Worker to be able to accept the HIT. The type also describes how a Worker can obtain a Qualification of that type, such as through a Qualification test. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QualificationType {
    /// <p>The answers to the Qualification test specified in the Test parameter.</p>
    #[serde(rename = "AnswerKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_key: Option<String>,
    /// <p>Specifies that requests for the Qualification type are granted immediately, without prompting the Worker with a Qualification test. Valid values are True | False.</p>
    #[serde(rename = "AutoGranted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_granted: Option<bool>,
    /// <p> The Qualification integer value to use for automatically granted Qualifications, if AutoGranted is true. This is 1 by default. </p>
    #[serde(rename = "AutoGrantedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_granted_value: Option<i64>,
    /// <p> The date and time the Qualification type was created. </p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p> A long description for the Qualification type. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Specifies whether the Qualification type is one that a user can request through the Amazon Mechanical Turk web site, such as by taking a Qualification test. This value is False for Qualifications assigned automatically by the system. Valid values are True | False. </p>
    #[serde(rename = "IsRequestable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_requestable: Option<bool>,
    /// <p> One or more words or phrases that describe theQualification type, separated by commas. The Keywords make the type easier to find using a search. </p>
    #[serde(rename = "Keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// <p> The name of the Qualification type. The type name is used to identify the type, and to find the type using a Qualification type search. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> A unique identifier for the Qualification type. A Qualification type is given a Qualification type ID when you call the CreateQualificationType operation. </p>
    #[serde(rename = "QualificationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type_id: Option<String>,
    /// <p> The status of the Qualification type. A Qualification type's status determines if users can apply to receive a Qualification of this type, and if HITs can be created with requirements based on this type. Valid values are Active | Inactive. </p>
    #[serde(rename = "QualificationTypeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type_status: Option<String>,
    /// <p> The amount of time, in seconds, Workers must wait after taking the Qualification test before they can take it again. Workers can take a Qualification test multiple times if they were not granted the Qualification from a previous attempt, or if the test offers a gradient score and they want a better score. If not specified, retries are disabled and Workers can request a Qualification only once. </p>
    #[serde(rename = "RetryDelayInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay_in_seconds: Option<i64>,
    /// <p> The questions for a Qualification test associated with this Qualification type that a user can take to obtain a Qualification of this type. This parameter must be specified if AnswerKey is present. A Qualification type cannot have both a specified Test parameter and an AutoGranted value of true. </p>
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    /// <p> The amount of time, in seconds, given to a Worker to complete the Qualification test, beginning from the time the Worker requests the Qualification. </p>
    #[serde(rename = "TestDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_duration_in_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectAssignmentRequest {
    /// <p> The ID of the assignment. The assignment must correspond to a HIT created by the Requester. </p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
    /// <p> A message for the Worker, which the Worker can see in the Status section of the web site. </p>
    #[serde(rename = "RequesterFeedback")]
    pub requester_feedback: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectAssignmentResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectQualificationRequestRequest {
    /// <p> The ID of the Qualification request, as returned by the <code>ListQualificationRequests</code> operation. </p>
    #[serde(rename = "QualificationRequestId")]
    pub qualification_request_id: String,
    /// <p>A text message explaining why the request was rejected, to be shown to the Worker who made the request.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectQualificationRequestResponse {}

/// <p> Both the AssignmentReviewReport and the HITReviewReport elements contains the ReviewActionDetail data structure. This structure is returned multiple times for each action specified in the Review Policy. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReviewActionDetail {
    /// <p>The unique identifier for the action.</p>
    #[serde(rename = "ActionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    /// <p> The nature of the action itself. The Review Policy is responsible for examining the HIT and Assignments, emitting results, and deciding which other actions will be necessary. </p>
    #[serde(rename = "ActionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p> The date when the action was completed.</p>
    #[serde(rename = "CompleteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<f64>,
    /// <p> Present only when the Results have a FAILED Status.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p> A description of the outcome of the review.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p> The current disposition of the action: INTENDED, SUCCEEDED, FAILED, or CANCELLED. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The specific HITId or AssignmentID targeted by the action.</p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p> The type of object in TargetId.</p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

/// <p> HIT Review Policy data structures represent HIT review policies, which you specify when you create a HIT. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewPolicy {
    /// <p>Name of the parameter from the Review policy.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<PolicyParameter>>,
    /// <p> Name of a Review Policy: SimplePlurality/2011-09-01 or ScoreMyKnownAnswers/2011-09-01 </p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

/// <p> Contains both ReviewResult and ReviewAction elements for a particular HIT. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReviewReport {
    /// <p> A list of ReviewAction objects for each action specified in the Review Policy. </p>
    #[serde(rename = "ReviewActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_actions: Option<Vec<ReviewActionDetail>>,
    /// <p> A list of ReviewResults objects for each action specified in the Review Policy. </p>
    #[serde(rename = "ReviewResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_results: Option<Vec<ReviewResultDetail>>,
}

/// <p> This data structure is returned multiple times for each result specified in the Review Policy. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReviewResultDetail {
    /// <p> A unique identifier of the Review action result. </p>
    #[serde(rename = "ActionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    /// <p> Key identifies the particular piece of reviewed information. </p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p> Specifies the QuestionId the result is describing. Depending on whether the TargetType is a HIT or Assignment this results could specify multiple values. If TargetType is HIT and QuestionId is absent, then the result describes results of the HIT, including the HIT agreement score. If ObjectType is Assignment and QuestionId is absent, then the result describes the Worker's performance on the HIT. </p>
    #[serde(rename = "QuestionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    /// <p>The HITID or AssignmentId about which this result was taken. Note that HIT-level Review Policies will often emit results about both the HIT itself and its Assignments, while Assignment-level review policies generally only emit results about the Assignment itself. </p>
    #[serde(rename = "SubjectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    /// <p> The type of the object from the SubjectId field.</p>
    #[serde(rename = "SubjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    /// <p> The values of Key provided by the review policies you have selected. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendBonusRequest {
    /// <p>The ID of the assignment for which this bonus is paid.</p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
    /// <p> The Bonus amount is a US Dollar amount specified using a string (for example, "5" represents $5.00 USD and "101.42" represents $101.42 USD). Do not include currency symbols or currency codes. </p>
    #[serde(rename = "BonusAmount")]
    pub bonus_amount: String,
    /// <p>A message that explains the reason for the bonus payment. The Worker receiving the bonus can see this message.</p>
    #[serde(rename = "Reason")]
    pub reason: String,
    /// <p>A unique identifier for this request, which allows you to retry the call on error without granting multiple bonuses. This is useful in cases such as network timeouts where it is unclear whether or not the call succeeded on the server. If the bonus already exists in the system from a previous call using the same UniqueRequestToken, subsequent calls will return an error with a message containing the request ID.</p>
    #[serde(rename = "UniqueRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_request_token: Option<String>,
    /// <p>The ID of the Worker being paid the bonus.</p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendBonusResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendTestEventNotificationRequest {
    /// <p> The notification specification to test. This value is identical to the value you would provide to the UpdateNotificationSettings operation when you establish the notification specification for a HIT type. </p>
    #[serde(rename = "Notification")]
    pub notification: NotificationSpecification,
    /// <p> The event to simulate to test the notification specification. This event is included in the test message even if the notification specification does not include the event type. The notification specification does not filter out the test event. </p>
    #[serde(rename = "TestEventType")]
    pub test_event_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendTestEventNotificationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateExpirationForHITRequest {
    /// <p> The date and time at which you want the HIT to expire </p>
    #[serde(rename = "ExpireAt")]
    pub expire_at: f64,
    /// <p> The HIT to update. </p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateExpirationForHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateHITReviewStatusRequest {
    /// <p> The ID of the HIT to update. </p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
    /// <p><p> Specifies how to update the HIT status. Default is <code>False</code>. </p> <ul> <li> <p> Setting this to false will only transition a HIT from <code>Reviewable</code> to <code>Reviewing</code> </p> </li> <li> <p> Setting this to true will only transition a HIT from <code>Reviewing</code> to <code>Reviewable</code> </p> </li> </ul></p>
    #[serde(rename = "Revert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateHITReviewStatusResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateHITTypeOfHITRequest {
    /// <p>The HIT to update.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
    /// <p>The ID of the new HIT type.</p>
    #[serde(rename = "HITTypeId")]
    pub hit_type_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateHITTypeOfHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNotificationSettingsRequest {
    /// <p> Specifies whether notifications are sent for HITs of this HIT type, according to the notification specification. You must specify either the Notification parameter or the Active parameter for the call to UpdateNotificationSettings to succeed. </p>
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// <p> The ID of the HIT type whose notification specification is being updated. </p>
    #[serde(rename = "HITTypeId")]
    pub hit_type_id: String,
    /// <p> The notification specification for the HIT type. </p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<NotificationSpecification>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNotificationSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateQualificationTypeRequest {
    /// <p>The answers to the Qualification test specified in the Test parameter, in the form of an AnswerKey data structure.</p>
    #[serde(rename = "AnswerKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_key: Option<String>,
    /// <p>Specifies whether requests for the Qualification type are granted immediately, without prompting the Worker with a Qualification test.</p> <p>Constraints: If the Test parameter is specified, this parameter cannot be true.</p>
    #[serde(rename = "AutoGranted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_granted: Option<bool>,
    /// <p>The Qualification value to use for automatically granted Qualifications. This parameter is used only if the AutoGranted parameter is true.</p>
    #[serde(rename = "AutoGrantedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_granted_value: Option<i64>,
    /// <p>The new description of the Qualification type.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the Qualification type to update.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
    /// <p>The new status of the Qualification type - Active | Inactive</p>
    #[serde(rename = "QualificationTypeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type_status: Option<String>,
    /// <p>The amount of time, in seconds, that Workers must wait after requesting a Qualification of the specified Qualification type before they can retry the Qualification request. It is not possible to disable retries for a Qualification type after it has been created with retries enabled. If you want to disable retries, you must dispose of the existing retry-enabled Qualification type using DisposeQualificationType and then create a new Qualification type with retries disabled using CreateQualificationType.</p>
    #[serde(rename = "RetryDelayInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay_in_seconds: Option<i64>,
    /// <p>The questions for the Qualification test a Worker must answer correctly to obtain a Qualification of this type. If this parameter is specified, <code>TestDurationInSeconds</code> must also be specified.</p> <p>Constraints: Must not be longer than 65535 bytes. Must be a QuestionForm data structure. This parameter cannot be specified if AutoGranted is true.</p> <p>Constraints: None. If not specified, the Worker may request the Qualification without answering any questions.</p>
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    /// <p>The number of seconds the Worker has to complete the Qualification test, starting from the time the Worker requests the Qualification.</p>
    #[serde(rename = "TestDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_duration_in_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateQualificationTypeResponse {
    /// <p> Contains a QualificationType data structure.</p>
    #[serde(rename = "QualificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type: Option<QualificationType>,
}

/// <p> The WorkerBlock data structure represents a Worker who has been blocked. It has two elements: the WorkerId and the Reason for the block. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkerBlock {
    /// <p> A message explaining the reason the Worker was blocked. </p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p> The ID of the Worker who accepted the HIT.</p>
    #[serde(rename = "WorkerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}

/// Errors returned by AcceptQualificationRequest
#[derive(Debug, PartialEq)]
pub enum AcceptQualificationRequestError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl AcceptQualificationRequestError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AcceptQualificationRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(AcceptQualificationRequestError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(AcceptQualificationRequestError::ServiceFault(
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
impl fmt::Display for AcceptQualificationRequestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptQualificationRequestError::RequestError(ref cause) => write!(f, "{}", cause),
            AcceptQualificationRequestError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptQualificationRequestError {}
/// Errors returned by ApproveAssignment
#[derive(Debug, PartialEq)]
pub enum ApproveAssignmentError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ApproveAssignmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ApproveAssignmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ApproveAssignmentError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ApproveAssignmentError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ApproveAssignmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApproveAssignmentError::RequestError(ref cause) => write!(f, "{}", cause),
            ApproveAssignmentError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ApproveAssignmentError {}
/// Errors returned by AssociateQualificationWithWorker
#[derive(Debug, PartialEq)]
pub enum AssociateQualificationWithWorkerError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl AssociateQualificationWithWorkerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateQualificationWithWorkerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(
                        AssociateQualificationWithWorkerError::RequestError(err.msg),
                    )
                }
                "ServiceFault" => {
                    return RusotoError::Service(
                        AssociateQualificationWithWorkerError::ServiceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateQualificationWithWorkerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateQualificationWithWorkerError::RequestError(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateQualificationWithWorkerError::ServiceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateQualificationWithWorkerError {}
/// Errors returned by CreateAdditionalAssignmentsForHIT
#[derive(Debug, PartialEq)]
pub enum CreateAdditionalAssignmentsForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl CreateAdditionalAssignmentsForHITError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateAdditionalAssignmentsForHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(
                        CreateAdditionalAssignmentsForHITError::RequestError(err.msg),
                    )
                }
                "ServiceFault" => {
                    return RusotoError::Service(
                        CreateAdditionalAssignmentsForHITError::ServiceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAdditionalAssignmentsForHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAdditionalAssignmentsForHITError::RequestError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateAdditionalAssignmentsForHITError::ServiceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateAdditionalAssignmentsForHITError {}
/// Errors returned by CreateHIT
#[derive(Debug, PartialEq)]
pub enum CreateHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl CreateHITError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(CreateHITError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(CreateHITError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHITError::RequestError(ref cause) => write!(f, "{}", cause),
            CreateHITError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHITError {}
/// Errors returned by CreateHITType
#[derive(Debug, PartialEq)]
pub enum CreateHITTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl CreateHITTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHITTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(CreateHITTypeError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(CreateHITTypeError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateHITTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHITTypeError::RequestError(ref cause) => write!(f, "{}", cause),
            CreateHITTypeError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHITTypeError {}
/// Errors returned by CreateHITWithHITType
#[derive(Debug, PartialEq)]
pub enum CreateHITWithHITTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl CreateHITWithHITTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHITWithHITTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(CreateHITWithHITTypeError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(CreateHITWithHITTypeError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateHITWithHITTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHITWithHITTypeError::RequestError(ref cause) => write!(f, "{}", cause),
            CreateHITWithHITTypeError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHITWithHITTypeError {}
/// Errors returned by CreateQualificationType
#[derive(Debug, PartialEq)]
pub enum CreateQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl CreateQualificationTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateQualificationTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(CreateQualificationTypeError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(CreateQualificationTypeError::ServiceFault(
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
impl fmt::Display for CreateQualificationTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateQualificationTypeError::RequestError(ref cause) => write!(f, "{}", cause),
            CreateQualificationTypeError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateQualificationTypeError {}
/// Errors returned by CreateWorkerBlock
#[derive(Debug, PartialEq)]
pub enum CreateWorkerBlockError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl CreateWorkerBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorkerBlockError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(CreateWorkerBlockError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(CreateWorkerBlockError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateWorkerBlockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWorkerBlockError::RequestError(ref cause) => write!(f, "{}", cause),
            CreateWorkerBlockError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWorkerBlockError {}
/// Errors returned by DeleteHIT
#[derive(Debug, PartialEq)]
pub enum DeleteHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl DeleteHITError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(DeleteHITError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(DeleteHITError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteHITError::RequestError(ref cause) => write!(f, "{}", cause),
            DeleteHITError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteHITError {}
/// Errors returned by DeleteQualificationType
#[derive(Debug, PartialEq)]
pub enum DeleteQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl DeleteQualificationTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteQualificationTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(DeleteQualificationTypeError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(DeleteQualificationTypeError::ServiceFault(
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
impl fmt::Display for DeleteQualificationTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteQualificationTypeError::RequestError(ref cause) => write!(f, "{}", cause),
            DeleteQualificationTypeError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteQualificationTypeError {}
/// Errors returned by DeleteWorkerBlock
#[derive(Debug, PartialEq)]
pub enum DeleteWorkerBlockError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl DeleteWorkerBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWorkerBlockError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(DeleteWorkerBlockError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(DeleteWorkerBlockError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteWorkerBlockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWorkerBlockError::RequestError(ref cause) => write!(f, "{}", cause),
            DeleteWorkerBlockError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWorkerBlockError {}
/// Errors returned by DisassociateQualificationFromWorker
#[derive(Debug, PartialEq)]
pub enum DisassociateQualificationFromWorkerError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl DisassociateQualificationFromWorkerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateQualificationFromWorkerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(
                        DisassociateQualificationFromWorkerError::RequestError(err.msg),
                    )
                }
                "ServiceFault" => {
                    return RusotoError::Service(
                        DisassociateQualificationFromWorkerError::ServiceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateQualificationFromWorkerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateQualificationFromWorkerError::RequestError(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateQualificationFromWorkerError::ServiceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateQualificationFromWorkerError {}
/// Errors returned by GetAccountBalance
#[derive(Debug, PartialEq)]
pub enum GetAccountBalanceError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl GetAccountBalanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountBalanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(GetAccountBalanceError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(GetAccountBalanceError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAccountBalanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccountBalanceError::RequestError(ref cause) => write!(f, "{}", cause),
            GetAccountBalanceError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountBalanceError {}
/// Errors returned by GetAssignment
#[derive(Debug, PartialEq)]
pub enum GetAssignmentError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl GetAssignmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAssignmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(GetAssignmentError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(GetAssignmentError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAssignmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAssignmentError::RequestError(ref cause) => write!(f, "{}", cause),
            GetAssignmentError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAssignmentError {}
/// Errors returned by GetFileUploadURL
#[derive(Debug, PartialEq)]
pub enum GetFileUploadURLError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl GetFileUploadURLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFileUploadURLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(GetFileUploadURLError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(GetFileUploadURLError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFileUploadURLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFileUploadURLError::RequestError(ref cause) => write!(f, "{}", cause),
            GetFileUploadURLError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFileUploadURLError {}
/// Errors returned by GetHIT
#[derive(Debug, PartialEq)]
pub enum GetHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl GetHITError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => return RusotoError::Service(GetHITError::RequestError(err.msg)),
                "ServiceFault" => return RusotoError::Service(GetHITError::ServiceFault(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHITError::RequestError(ref cause) => write!(f, "{}", cause),
            GetHITError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHITError {}
/// Errors returned by GetQualificationScore
#[derive(Debug, PartialEq)]
pub enum GetQualificationScoreError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl GetQualificationScoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQualificationScoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(GetQualificationScoreError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(GetQualificationScoreError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetQualificationScoreError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQualificationScoreError::RequestError(ref cause) => write!(f, "{}", cause),
            GetQualificationScoreError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetQualificationScoreError {}
/// Errors returned by GetQualificationType
#[derive(Debug, PartialEq)]
pub enum GetQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl GetQualificationTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQualificationTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(GetQualificationTypeError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(GetQualificationTypeError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetQualificationTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQualificationTypeError::RequestError(ref cause) => write!(f, "{}", cause),
            GetQualificationTypeError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetQualificationTypeError {}
/// Errors returned by ListAssignmentsForHIT
#[derive(Debug, PartialEq)]
pub enum ListAssignmentsForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListAssignmentsForHITError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssignmentsForHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListAssignmentsForHITError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListAssignmentsForHITError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssignmentsForHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAssignmentsForHITError::RequestError(ref cause) => write!(f, "{}", cause),
            ListAssignmentsForHITError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAssignmentsForHITError {}
/// Errors returned by ListBonusPayments
#[derive(Debug, PartialEq)]
pub enum ListBonusPaymentsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListBonusPaymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBonusPaymentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListBonusPaymentsError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListBonusPaymentsError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListBonusPaymentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBonusPaymentsError::RequestError(ref cause) => write!(f, "{}", cause),
            ListBonusPaymentsError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBonusPaymentsError {}
/// Errors returned by ListHITs
#[derive(Debug, PartialEq)]
pub enum ListHITsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListHITsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHITsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListHITsError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListHITsError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListHITsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHITsError::RequestError(ref cause) => write!(f, "{}", cause),
            ListHITsError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHITsError {}
/// Errors returned by ListHITsForQualificationType
#[derive(Debug, PartialEq)]
pub enum ListHITsForQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListHITsForQualificationTypeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListHITsForQualificationTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListHITsForQualificationTypeError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListHITsForQualificationTypeError::ServiceFault(
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
impl fmt::Display for ListHITsForQualificationTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHITsForQualificationTypeError::RequestError(ref cause) => write!(f, "{}", cause),
            ListHITsForQualificationTypeError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHITsForQualificationTypeError {}
/// Errors returned by ListQualificationRequests
#[derive(Debug, PartialEq)]
pub enum ListQualificationRequestsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListQualificationRequestsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQualificationRequestsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListQualificationRequestsError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListQualificationRequestsError::ServiceFault(
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
impl fmt::Display for ListQualificationRequestsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQualificationRequestsError::RequestError(ref cause) => write!(f, "{}", cause),
            ListQualificationRequestsError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQualificationRequestsError {}
/// Errors returned by ListQualificationTypes
#[derive(Debug, PartialEq)]
pub enum ListQualificationTypesError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListQualificationTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQualificationTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListQualificationTypesError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListQualificationTypesError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListQualificationTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQualificationTypesError::RequestError(ref cause) => write!(f, "{}", cause),
            ListQualificationTypesError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQualificationTypesError {}
/// Errors returned by ListReviewPolicyResultsForHIT
#[derive(Debug, PartialEq)]
pub enum ListReviewPolicyResultsForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListReviewPolicyResultsForHITError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListReviewPolicyResultsForHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListReviewPolicyResultsForHITError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListReviewPolicyResultsForHITError::ServiceFault(
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
impl fmt::Display for ListReviewPolicyResultsForHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReviewPolicyResultsForHITError::RequestError(ref cause) => write!(f, "{}", cause),
            ListReviewPolicyResultsForHITError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReviewPolicyResultsForHITError {}
/// Errors returned by ListReviewableHITs
#[derive(Debug, PartialEq)]
pub enum ListReviewableHITsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListReviewableHITsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReviewableHITsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListReviewableHITsError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListReviewableHITsError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListReviewableHITsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReviewableHITsError::RequestError(ref cause) => write!(f, "{}", cause),
            ListReviewableHITsError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReviewableHITsError {}
/// Errors returned by ListWorkerBlocks
#[derive(Debug, PartialEq)]
pub enum ListWorkerBlocksError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListWorkerBlocksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorkerBlocksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(ListWorkerBlocksError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(ListWorkerBlocksError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWorkerBlocksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorkerBlocksError::RequestError(ref cause) => write!(f, "{}", cause),
            ListWorkerBlocksError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWorkerBlocksError {}
/// Errors returned by ListWorkersWithQualificationType
#[derive(Debug, PartialEq)]
pub enum ListWorkersWithQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl ListWorkersWithQualificationTypeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListWorkersWithQualificationTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(
                        ListWorkersWithQualificationTypeError::RequestError(err.msg),
                    )
                }
                "ServiceFault" => {
                    return RusotoError::Service(
                        ListWorkersWithQualificationTypeError::ServiceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWorkersWithQualificationTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorkersWithQualificationTypeError::RequestError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListWorkersWithQualificationTypeError::ServiceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListWorkersWithQualificationTypeError {}
/// Errors returned by NotifyWorkers
#[derive(Debug, PartialEq)]
pub enum NotifyWorkersError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl NotifyWorkersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<NotifyWorkersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(NotifyWorkersError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(NotifyWorkersError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for NotifyWorkersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotifyWorkersError::RequestError(ref cause) => write!(f, "{}", cause),
            NotifyWorkersError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for NotifyWorkersError {}
/// Errors returned by RejectAssignment
#[derive(Debug, PartialEq)]
pub enum RejectAssignmentError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl RejectAssignmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectAssignmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(RejectAssignmentError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(RejectAssignmentError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RejectAssignmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectAssignmentError::RequestError(ref cause) => write!(f, "{}", cause),
            RejectAssignmentError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectAssignmentError {}
/// Errors returned by RejectQualificationRequest
#[derive(Debug, PartialEq)]
pub enum RejectQualificationRequestError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl RejectQualificationRequestError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RejectQualificationRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(RejectQualificationRequestError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(RejectQualificationRequestError::ServiceFault(
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
impl fmt::Display for RejectQualificationRequestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectQualificationRequestError::RequestError(ref cause) => write!(f, "{}", cause),
            RejectQualificationRequestError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectQualificationRequestError {}
/// Errors returned by SendBonus
#[derive(Debug, PartialEq)]
pub enum SendBonusError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl SendBonusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendBonusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(SendBonusError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(SendBonusError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendBonusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendBonusError::RequestError(ref cause) => write!(f, "{}", cause),
            SendBonusError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendBonusError {}
/// Errors returned by SendTestEventNotification
#[derive(Debug, PartialEq)]
pub enum SendTestEventNotificationError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl SendTestEventNotificationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendTestEventNotificationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(SendTestEventNotificationError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(SendTestEventNotificationError::ServiceFault(
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
impl fmt::Display for SendTestEventNotificationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendTestEventNotificationError::RequestError(ref cause) => write!(f, "{}", cause),
            SendTestEventNotificationError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendTestEventNotificationError {}
/// Errors returned by UpdateExpirationForHIT
#[derive(Debug, PartialEq)]
pub enum UpdateExpirationForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl UpdateExpirationForHITError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateExpirationForHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(UpdateExpirationForHITError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(UpdateExpirationForHITError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateExpirationForHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateExpirationForHITError::RequestError(ref cause) => write!(f, "{}", cause),
            UpdateExpirationForHITError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateExpirationForHITError {}
/// Errors returned by UpdateHITReviewStatus
#[derive(Debug, PartialEq)]
pub enum UpdateHITReviewStatusError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl UpdateHITReviewStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateHITReviewStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(UpdateHITReviewStatusError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(UpdateHITReviewStatusError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateHITReviewStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateHITReviewStatusError::RequestError(ref cause) => write!(f, "{}", cause),
            UpdateHITReviewStatusError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateHITReviewStatusError {}
/// Errors returned by UpdateHITTypeOfHIT
#[derive(Debug, PartialEq)]
pub enum UpdateHITTypeOfHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl UpdateHITTypeOfHITError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateHITTypeOfHITError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(UpdateHITTypeOfHITError::RequestError(err.msg))
                }
                "ServiceFault" => {
                    return RusotoError::Service(UpdateHITTypeOfHITError::ServiceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateHITTypeOfHITError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateHITTypeOfHITError::RequestError(ref cause) => write!(f, "{}", cause),
            UpdateHITTypeOfHITError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateHITTypeOfHITError {}
/// Errors returned by UpdateNotificationSettings
#[derive(Debug, PartialEq)]
pub enum UpdateNotificationSettingsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl UpdateNotificationSettingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateNotificationSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(UpdateNotificationSettingsError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(UpdateNotificationSettingsError::ServiceFault(
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
impl fmt::Display for UpdateNotificationSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNotificationSettingsError::RequestError(ref cause) => write!(f, "{}", cause),
            UpdateNotificationSettingsError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNotificationSettingsError {}
/// Errors returned by UpdateQualificationType
#[derive(Debug, PartialEq)]
pub enum UpdateQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
}

impl UpdateQualificationTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateQualificationTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "RequestError" => {
                    return RusotoError::Service(UpdateQualificationTypeError::RequestError(
                        err.msg,
                    ))
                }
                "ServiceFault" => {
                    return RusotoError::Service(UpdateQualificationTypeError::ServiceFault(
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
impl fmt::Display for UpdateQualificationTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateQualificationTypeError::RequestError(ref cause) => write!(f, "{}", cause),
            UpdateQualificationTypeError::ServiceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateQualificationTypeError {}
/// Trait representing the capabilities of the Amazon MTurk API. Amazon MTurk clients implement this trait.
#[async_trait]
pub trait MechanicalTurk {
    /// <p> The <code>AcceptQualificationRequest</code> operation approves a Worker's request for a Qualification. </p> <p> Only the owner of the Qualification type can grant a Qualification request for that type. </p> <p> A successful request for the <code>AcceptQualificationRequest</code> operation returns with no errors and an empty body. </p>
    async fn accept_qualification_request(
        &self,
        input: AcceptQualificationRequestRequest,
    ) -> Result<AcceptQualificationRequestResponse, RusotoError<AcceptQualificationRequestError>>;

    /// <p> The <code>ApproveAssignment</code> operation approves the results of a completed assignment. </p> <p> Approving an assignment initiates two payments from the Requester's Amazon.com account </p> <ul> <li> <p> The Worker who submitted the results is paid the reward specified in the HIT. </p> </li> <li> <p> Amazon Mechanical Turk fees are debited. </p> </li> </ul> <p> If the Requester's account does not have adequate funds for these payments, the call to ApproveAssignment returns an exception, and the approval is not processed. You can include an optional feedback message with the approval, which the Worker can see in the Status section of the web site. </p> <p> You can also call this operation for assignments that were previous rejected and approve them by explicitly overriding the previous rejection. This only works on rejected assignments that were submitted within the previous 30 days and only if the assignment's related HIT has not been deleted. </p>
    async fn approve_assignment(
        &self,
        input: ApproveAssignmentRequest,
    ) -> Result<ApproveAssignmentResponse, RusotoError<ApproveAssignmentError>>;

    /// <p><p> The <code>AssociateQualificationWithWorker</code> operation gives a Worker a Qualification. <code>AssociateQualificationWithWorker</code> does not require that the Worker submit a Qualification request. It gives the Qualification directly to the Worker. </p> <p> You can only assign a Qualification of a Qualification type that you created (using the <code>CreateQualificationType</code> operation). </p> <note> <p> Note: <code>AssociateQualificationWithWorker</code> does not affect any pending Qualification requests for the Qualification by the Worker. If you assign a Qualification to a Worker, then later grant a Qualification request made by the Worker, the granting of the request may modify the Qualification score. To resolve a pending Qualification request without affecting the Qualification the Worker already has, reject the request with the <code>RejectQualificationRequest</code> operation. </p> </note></p>
    async fn associate_qualification_with_worker(
        &self,
        input: AssociateQualificationWithWorkerRequest,
    ) -> Result<
        AssociateQualificationWithWorkerResponse,
        RusotoError<AssociateQualificationWithWorkerError>,
    >;

    /// <p><p> The <code>CreateAdditionalAssignmentsForHIT</code> operation increases the maximum number of assignments of an existing HIT. </p> <p> To extend the maximum number of assignments, specify the number of additional assignments.</p> <note> <ul> <li> <p>HITs created with fewer than 10 assignments cannot be extended to have 10 or more assignments. Attempting to add assignments in a way that brings the total number of assignments for a HIT from fewer than 10 assignments to 10 or more assignments will result in an <code>AWS.MechanicalTurk.InvalidMaximumAssignmentsIncrease</code> exception.</p> </li> <li> <p>HITs that were created before July 22, 2015 cannot be extended. Attempting to extend HITs that were created before July 22, 2015 will result in an <code>AWS.MechanicalTurk.HITTooOldForExtension</code> exception. </p> </li> </ul> </note></p>
    async fn create_additional_assignments_for_hit(
        &self,
        input: CreateAdditionalAssignmentsForHITRequest,
    ) -> Result<
        CreateAdditionalAssignmentsForHITResponse,
        RusotoError<CreateAdditionalAssignmentsForHITError>,
    >;

    /// <p><p>The <code>CreateHIT</code> operation creates a new Human Intelligence Task (HIT). The new HIT is made available for Workers to find and accept on the Amazon Mechanical Turk website. </p> <p> This operation allows you to specify a new HIT by passing in values for the properties of the HIT, such as its title, reward amount and number of assignments. When you pass these values to <code>CreateHIT</code>, a new HIT is created for you, with a new <code>HITTypeID</code>. The HITTypeID can be used to create additional HITs in the future without needing to specify common parameters such as the title, description and reward amount each time.</p> <p> An alternative way to create HITs is to first generate a HITTypeID using the <code>CreateHITType</code> operation and then call the <code>CreateHITWithHITType</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHIT also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>.</p> </note></p>
    async fn create_hit(
        &self,
        input: CreateHITRequest,
    ) -> Result<CreateHITResponse, RusotoError<CreateHITError>>;

    /// <p> The <code>CreateHITType</code> operation creates a new HIT type. This operation allows you to define a standard set of HIT properties to use when creating HITs. If you register a HIT type with values that match an existing HIT type, the HIT type ID of the existing type will be returned. </p>
    async fn create_hit_type(
        &self,
        input: CreateHITTypeRequest,
    ) -> Result<CreateHITTypeResponse, RusotoError<CreateHITTypeError>>;

    /// <p><p> The <code>CreateHITWithHITType</code> operation creates a new Human Intelligence Task (HIT) using an existing HITTypeID generated by the <code>CreateHITType</code> operation. </p> <p> This is an alternative way to create HITs from the <code>CreateHIT</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHITWithHITType also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>. </p> </note></p>
    async fn create_hit_with_hit_type(
        &self,
        input: CreateHITWithHITTypeRequest,
    ) -> Result<CreateHITWithHITTypeResponse, RusotoError<CreateHITWithHITTypeError>>;

    /// <p> The <code>CreateQualificationType</code> operation creates a new Qualification type, which is represented by a <code>QualificationType</code> data structure. </p>
    async fn create_qualification_type(
        &self,
        input: CreateQualificationTypeRequest,
    ) -> Result<CreateQualificationTypeResponse, RusotoError<CreateQualificationTypeError>>;

    /// <p>The <code>CreateWorkerBlock</code> operation allows you to prevent a Worker from working on your HITs. For example, you can block a Worker who is producing poor quality work. You can block up to 100,000 Workers.</p>
    async fn create_worker_block(
        &self,
        input: CreateWorkerBlockRequest,
    ) -> Result<CreateWorkerBlockResponse, RusotoError<CreateWorkerBlockError>>;

    /// <p><p> The <code>DeleteHIT</code> operation is used to delete HIT that is no longer needed. Only the Requester who created the HIT can delete it. </p> <p> You can only dispose of HITs that are in the <code>Reviewable</code> state, with all of their submitted assignments already either approved or rejected. If you call the DeleteHIT operation on a HIT that is not in the <code>Reviewable</code> state (for example, that has not expired, or still has active assignments), or on a HIT that is Reviewable but without all of its submitted assignments already approved or rejected, the service will return an error. </p> <note> <ul> <li> <p> HITs are automatically disposed of after 120 days. </p> </li> <li> <p> After you dispose of a HIT, you can no longer approve the HIT&#39;s rejected assignments. </p> </li> <li> <p> Disposed HITs are not returned in results for the ListHITs operation. </p> </li> <li> <p> Disposing HITs can improve the performance of operations such as ListReviewableHITs and ListHITs. </p> </li> </ul> </note></p>
    async fn delete_hit(
        &self,
        input: DeleteHITRequest,
    ) -> Result<DeleteHITResponse, RusotoError<DeleteHITError>>;

    /// <p><p> The <code>DeleteQualificationType</code> deletes a Qualification type and deletes any HIT types that are associated with the Qualification type. </p> <p>This operation does not revoke Qualifications already assigned to Workers because the Qualifications might be needed for active HITs. If there are any pending requests for the Qualification type, Amazon Mechanical Turk rejects those requests. After you delete a Qualification type, you can no longer use it to create HITs or HIT types.</p> <note> <p>DeleteQualificationType must wait for all the HITs that use the deleted Qualification type to be deleted before completing. It may take up to 48 hours before DeleteQualificationType completes and the unique name of the Qualification type is available for reuse with CreateQualificationType.</p> </note></p>
    async fn delete_qualification_type(
        &self,
        input: DeleteQualificationTypeRequest,
    ) -> Result<DeleteQualificationTypeResponse, RusotoError<DeleteQualificationTypeError>>;

    /// <p>The <code>DeleteWorkerBlock</code> operation allows you to reinstate a blocked Worker to work on your HITs. This operation reverses the effects of the CreateWorkerBlock operation. You need the Worker ID to use this operation. If the Worker ID is missing or invalid, this operation fails and returns the message “WorkerId is invalid.” If the specified Worker is not blocked, this operation returns successfully.</p>
    async fn delete_worker_block(
        &self,
        input: DeleteWorkerBlockRequest,
    ) -> Result<DeleteWorkerBlockResponse, RusotoError<DeleteWorkerBlockError>>;

    /// <p> The <code>DisassociateQualificationFromWorker</code> revokes a previously granted Qualification from a user. </p> <p> You can provide a text message explaining why the Qualification was revoked. The user who had the Qualification can see this message. </p>
    async fn disassociate_qualification_from_worker(
        &self,
        input: DisassociateQualificationFromWorkerRequest,
    ) -> Result<
        DisassociateQualificationFromWorkerResponse,
        RusotoError<DisassociateQualificationFromWorkerError>,
    >;

    /// <p>The <code>GetAccountBalance</code> operation retrieves the amount of money in your Amazon Mechanical Turk account.</p>
    async fn get_account_balance(
        &self,
    ) -> Result<GetAccountBalanceResponse, RusotoError<GetAccountBalanceError>>;

    /// <p> The <code>GetAssignment</code> operation retrieves the details of the specified Assignment. </p>
    async fn get_assignment(
        &self,
        input: GetAssignmentRequest,
    ) -> Result<GetAssignmentResponse, RusotoError<GetAssignmentError>>;

    /// <p> The <code>GetFileUploadURL</code> operation generates and returns a temporary URL. You use the temporary URL to retrieve a file uploaded by a Worker as an answer to a FileUploadAnswer question for a HIT. The temporary URL is generated the instant the GetFileUploadURL operation is called, and is valid for 60 seconds. You can get a temporary file upload URL any time until the HIT is disposed. After the HIT is disposed, any uploaded files are deleted, and cannot be retrieved. Pending Deprecation on December 12, 2017. The Answer Specification structure will no longer support the <code>FileUploadAnswer</code> element to be used for the QuestionForm data structure. Instead, we recommend that Requesters who want to create HITs asking Workers to upload files to use Amazon S3. </p>
    async fn get_file_upload_url(
        &self,
        input: GetFileUploadURLRequest,
    ) -> Result<GetFileUploadURLResponse, RusotoError<GetFileUploadURLError>>;

    /// <p> The <code>GetHIT</code> operation retrieves the details of the specified HIT. </p>
    async fn get_hit(
        &self,
        input: GetHITRequest,
    ) -> Result<GetHITResponse, RusotoError<GetHITError>>;

    /// <p> The <code>GetQualificationScore</code> operation returns the value of a Worker's Qualification for a given Qualification type. </p> <p> To get a Worker's Qualification, you must know the Worker's ID. The Worker's ID is included in the assignment data returned by the <code>ListAssignmentsForHIT</code> operation. </p> <p>Only the owner of a Qualification type can query the value of a Worker's Qualification of that type.</p>
    async fn get_qualification_score(
        &self,
        input: GetQualificationScoreRequest,
    ) -> Result<GetQualificationScoreResponse, RusotoError<GetQualificationScoreError>>;

    /// <p> The <code>GetQualificationType</code>operation retrieves information about a Qualification type using its ID. </p>
    async fn get_qualification_type(
        &self,
        input: GetQualificationTypeRequest,
    ) -> Result<GetQualificationTypeResponse, RusotoError<GetQualificationTypeError>>;

    /// <p> The <code>ListAssignmentsForHIT</code> operation retrieves completed assignments for a HIT. You can use this operation to retrieve the results for a HIT. </p> <p> You can get assignments for a HIT at any time, even if the HIT is not yet Reviewable. If a HIT requested multiple assignments, and has received some results but has not yet become Reviewable, you can still retrieve the partial results with this operation. </p> <p> Use the AssignmentStatus parameter to control which set of assignments for a HIT are returned. The ListAssignmentsForHIT operation can return submitted assignments awaiting approval, or it can return assignments that have already been approved or rejected. You can set AssignmentStatus=Approved,Rejected to get assignments that have already been approved and rejected together in one result set. </p> <p> Only the Requester who created the HIT can retrieve the assignments for that HIT. </p> <p> Results are sorted and divided into numbered pages and the operation returns a single page of results. You can use the parameters of the operation to control sorting and pagination. </p>
    async fn list_assignments_for_hit(
        &self,
        input: ListAssignmentsForHITRequest,
    ) -> Result<ListAssignmentsForHITResponse, RusotoError<ListAssignmentsForHITError>>;

    /// <p> The <code>ListBonusPayments</code> operation retrieves the amounts of bonuses you have paid to Workers for a given HIT or assignment. </p>
    async fn list_bonus_payments(
        &self,
        input: ListBonusPaymentsRequest,
    ) -> Result<ListBonusPaymentsResponse, RusotoError<ListBonusPaymentsError>>;

    /// <p> The <code>ListHITs</code> operation returns all of a Requester's HITs. The operation returns HITs of any status, except for HITs that have been deleted of with the DeleteHIT operation or that have been auto-deleted. </p>
    async fn list_hi_ts(
        &self,
        input: ListHITsRequest,
    ) -> Result<ListHITsResponse, RusotoError<ListHITsError>>;

    /// <p> The <code>ListHITsForQualificationType</code> operation returns the HITs that use the given Qualification type for a Qualification requirement. The operation returns HITs of any status, except for HITs that have been deleted with the <code>DeleteHIT</code> operation or that have been auto-deleted. </p>
    async fn list_hi_ts_for_qualification_type(
        &self,
        input: ListHITsForQualificationTypeRequest,
    ) -> Result<ListHITsForQualificationTypeResponse, RusotoError<ListHITsForQualificationTypeError>>;

    /// <p> The <code>ListQualificationRequests</code> operation retrieves requests for Qualifications of a particular Qualification type. The owner of the Qualification type calls this operation to poll for pending requests, and accepts them using the AcceptQualification operation. </p>
    async fn list_qualification_requests(
        &self,
        input: ListQualificationRequestsRequest,
    ) -> Result<ListQualificationRequestsResponse, RusotoError<ListQualificationRequestsError>>;

    /// <p> The <code>ListQualificationTypes</code> operation returns a list of Qualification types, filtered by an optional search term. </p>
    async fn list_qualification_types(
        &self,
        input: ListQualificationTypesRequest,
    ) -> Result<ListQualificationTypesResponse, RusotoError<ListQualificationTypesError>>;

    /// <p> The <code>ListReviewPolicyResultsForHIT</code> operation retrieves the computed results and the actions taken in the course of executing your Review Policies for a given HIT. For information about how to specify Review Policies when you call CreateHIT, see Review Policies. The ListReviewPolicyResultsForHIT operation can return results for both Assignment-level and HIT-level review results. </p>
    async fn list_review_policy_results_for_hit(
        &self,
        input: ListReviewPolicyResultsForHITRequest,
    ) -> Result<
        ListReviewPolicyResultsForHITResponse,
        RusotoError<ListReviewPolicyResultsForHITError>,
    >;

    /// <p> The <code>ListReviewableHITs</code> operation retrieves the HITs with Status equal to Reviewable or Status equal to Reviewing that belong to the Requester calling the operation. </p>
    async fn list_reviewable_hi_ts(
        &self,
        input: ListReviewableHITsRequest,
    ) -> Result<ListReviewableHITsResponse, RusotoError<ListReviewableHITsError>>;

    /// <p>The <code>ListWorkersBlocks</code> operation retrieves a list of Workers who are blocked from working on your HITs.</p>
    async fn list_worker_blocks(
        &self,
        input: ListWorkerBlocksRequest,
    ) -> Result<ListWorkerBlocksResponse, RusotoError<ListWorkerBlocksError>>;

    /// <p> The <code>ListWorkersWithQualificationType</code> operation returns all of the Workers that have been associated with a given Qualification type. </p>
    async fn list_workers_with_qualification_type(
        &self,
        input: ListWorkersWithQualificationTypeRequest,
    ) -> Result<
        ListWorkersWithQualificationTypeResponse,
        RusotoError<ListWorkersWithQualificationTypeError>,
    >;

    /// <p> The <code>NotifyWorkers</code> operation sends an email to one or more Workers that you specify with the Worker ID. You can specify up to 100 Worker IDs to send the same message with a single call to the NotifyWorkers operation. The NotifyWorkers operation will send a notification email to a Worker only if you have previously approved or rejected work from the Worker. </p>
    async fn notify_workers(
        &self,
        input: NotifyWorkersRequest,
    ) -> Result<NotifyWorkersResponse, RusotoError<NotifyWorkersError>>;

    /// <p> The <code>RejectAssignment</code> operation rejects the results of a completed assignment. </p> <p> You can include an optional feedback message with the rejection, which the Worker can see in the Status section of the web site. When you include a feedback message with the rejection, it helps the Worker understand why the assignment was rejected, and can improve the quality of the results the Worker submits in the future. </p> <p> Only the Requester who created the HIT can reject an assignment for the HIT. </p>
    async fn reject_assignment(
        &self,
        input: RejectAssignmentRequest,
    ) -> Result<RejectAssignmentResponse, RusotoError<RejectAssignmentError>>;

    /// <p> The <code>RejectQualificationRequest</code> operation rejects a user's request for a Qualification. </p> <p> You can provide a text message explaining why the request was rejected. The Worker who made the request can see this message.</p>
    async fn reject_qualification_request(
        &self,
        input: RejectQualificationRequestRequest,
    ) -> Result<RejectQualificationRequestResponse, RusotoError<RejectQualificationRequestError>>;

    /// <p> The <code>SendBonus</code> operation issues a payment of money from your account to a Worker. This payment happens separately from the reward you pay to the Worker when you approve the Worker's assignment. The SendBonus operation requires the Worker's ID and the assignment ID as parameters to initiate payment of the bonus. You must include a message that explains the reason for the bonus payment, as the Worker may not be expecting the payment. Amazon Mechanical Turk collects a fee for bonus payments, similar to the HIT listing fee. This operation fails if your account does not have enough funds to pay for both the bonus and the fees. </p>
    async fn send_bonus(
        &self,
        input: SendBonusRequest,
    ) -> Result<SendBonusResponse, RusotoError<SendBonusError>>;

    /// <p> The <code>SendTestEventNotification</code> operation causes Amazon Mechanical Turk to send a notification message as if a HIT event occurred, according to the provided notification specification. This allows you to test notifications without setting up notifications for a real HIT type and trying to trigger them using the website. When you call this operation, the service attempts to send the test notification immediately. </p>
    async fn send_test_event_notification(
        &self,
        input: SendTestEventNotificationRequest,
    ) -> Result<SendTestEventNotificationResponse, RusotoError<SendTestEventNotificationError>>;

    /// <p> The <code>UpdateExpirationForHIT</code> operation allows you update the expiration time of a HIT. If you update it to a time in the past, the HIT will be immediately expired. </p>
    async fn update_expiration_for_hit(
        &self,
        input: UpdateExpirationForHITRequest,
    ) -> Result<UpdateExpirationForHITResponse, RusotoError<UpdateExpirationForHITError>>;

    /// <p> The <code>UpdateHITReviewStatus</code> operation updates the status of a HIT. If the status is Reviewable, this operation can update the status to Reviewing, or it can revert a Reviewing HIT back to the Reviewable status. </p>
    async fn update_hit_review_status(
        &self,
        input: UpdateHITReviewStatusRequest,
    ) -> Result<UpdateHITReviewStatusResponse, RusotoError<UpdateHITReviewStatusError>>;

    /// <p> The <code>UpdateHITTypeOfHIT</code> operation allows you to change the HITType properties of a HIT. This operation disassociates the HIT from its old HITType properties and associates it with the new HITType properties. The HIT takes on the properties of the new HITType in place of the old ones. </p>
    async fn update_hit_type_of_hit(
        &self,
        input: UpdateHITTypeOfHITRequest,
    ) -> Result<UpdateHITTypeOfHITResponse, RusotoError<UpdateHITTypeOfHITError>>;

    /// <p> The <code>UpdateNotificationSettings</code> operation creates, updates, disables or re-enables notifications for a HIT type. If you call the UpdateNotificationSettings operation for a HIT type that already has a notification specification, the operation replaces the old specification with a new one. You can call the UpdateNotificationSettings operation to enable or disable notifications for the HIT type, without having to modify the notification specification itself by providing updates to the Active status without specifying a new notification specification. To change the Active status of a HIT type's notifications, the HIT type must already have a notification specification, or one must be provided in the same call to <code>UpdateNotificationSettings</code>. </p>
    async fn update_notification_settings(
        &self,
        input: UpdateNotificationSettingsRequest,
    ) -> Result<UpdateNotificationSettingsResponse, RusotoError<UpdateNotificationSettingsError>>;

    /// <p> The <code>UpdateQualificationType</code> operation modifies the attributes of an existing Qualification type, which is represented by a QualificationType data structure. Only the owner of a Qualification type can modify its attributes. </p> <p> Most attributes of a Qualification type can be changed after the type has been created. However, the Name and Keywords fields cannot be modified. The RetryDelayInSeconds parameter can be modified or added to change the delay or to enable retries, but RetryDelayInSeconds cannot be used to disable retries. </p> <p> You can use this operation to update the test for a Qualification type. The test is updated based on the values specified for the Test, TestDurationInSeconds and AnswerKey parameters. All three parameters specify the updated test. If you are updating the test for a type, you must specify the Test and TestDurationInSeconds parameters. The AnswerKey parameter is optional; omitting it specifies that the updated test does not have an answer key. </p> <p> If you omit the Test parameter, the test for the Qualification type is unchanged. There is no way to remove a test from a Qualification type that has one. If the type already has a test, you cannot update it to be AutoGranted. If the Qualification type does not have a test and one is provided by an update, the type will henceforth have a test. </p> <p> If you want to update the test duration or answer key for an existing test without changing the questions, you must specify a Test parameter with the original questions, along with the updated values. </p> <p> If you provide an updated Test but no AnswerKey, the new test will not have an answer key. Requests for such Qualifications must be granted manually. </p> <p> You can also update the AutoGranted and AutoGrantedValue attributes of the Qualification type.</p>
    async fn update_qualification_type(
        &self,
        input: UpdateQualificationTypeRequest,
    ) -> Result<UpdateQualificationTypeResponse, RusotoError<UpdateQualificationTypeError>>;
}
/// A client for the Amazon MTurk API.
#[derive(Clone)]
pub struct MechanicalTurkClient {
    client: Client,
    region: region::Region,
}

impl MechanicalTurkClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MechanicalTurkClient {
        MechanicalTurkClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MechanicalTurkClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MechanicalTurkClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MechanicalTurkClient {
        MechanicalTurkClient { client, region }
    }
}

#[async_trait]
impl MechanicalTurk for MechanicalTurkClient {
    /// <p> The <code>AcceptQualificationRequest</code> operation approves a Worker's request for a Qualification. </p> <p> Only the owner of the Qualification type can grant a Qualification request for that type. </p> <p> A successful request for the <code>AcceptQualificationRequest</code> operation returns with no errors and an empty body. </p>
    async fn accept_qualification_request(
        &self,
        input: AcceptQualificationRequestRequest,
    ) -> Result<AcceptQualificationRequestResponse, RusotoError<AcceptQualificationRequestError>>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.AcceptQualificationRequest",
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
                .deserialize::<AcceptQualificationRequestResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptQualificationRequestError::from_response(response))
        }
    }

    /// <p> The <code>ApproveAssignment</code> operation approves the results of a completed assignment. </p> <p> Approving an assignment initiates two payments from the Requester's Amazon.com account </p> <ul> <li> <p> The Worker who submitted the results is paid the reward specified in the HIT. </p> </li> <li> <p> Amazon Mechanical Turk fees are debited. </p> </li> </ul> <p> If the Requester's account does not have adequate funds for these payments, the call to ApproveAssignment returns an exception, and the approval is not processed. You can include an optional feedback message with the approval, which the Worker can see in the Status section of the web site. </p> <p> You can also call this operation for assignments that were previous rejected and approve them by explicitly overriding the previous rejection. This only works on rejected assignments that were submitted within the previous 30 days and only if the assignment's related HIT has not been deleted. </p>
    async fn approve_assignment(
        &self,
        input: ApproveAssignmentRequest,
    ) -> Result<ApproveAssignmentResponse, RusotoError<ApproveAssignmentError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ApproveAssignment",
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
                .deserialize::<ApproveAssignmentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ApproveAssignmentError::from_response(response))
        }
    }

    /// <p><p> The <code>AssociateQualificationWithWorker</code> operation gives a Worker a Qualification. <code>AssociateQualificationWithWorker</code> does not require that the Worker submit a Qualification request. It gives the Qualification directly to the Worker. </p> <p> You can only assign a Qualification of a Qualification type that you created (using the <code>CreateQualificationType</code> operation). </p> <note> <p> Note: <code>AssociateQualificationWithWorker</code> does not affect any pending Qualification requests for the Qualification by the Worker. If you assign a Qualification to a Worker, then later grant a Qualification request made by the Worker, the granting of the request may modify the Qualification score. To resolve a pending Qualification request without affecting the Qualification the Worker already has, reject the request with the <code>RejectQualificationRequest</code> operation. </p> </note></p>
    async fn associate_qualification_with_worker(
        &self,
        input: AssociateQualificationWithWorkerRequest,
    ) -> Result<
        AssociateQualificationWithWorkerResponse,
        RusotoError<AssociateQualificationWithWorkerError>,
    > {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.AssociateQualificationWithWorker",
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
                .deserialize::<AssociateQualificationWithWorkerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateQualificationWithWorkerError::from_response(
                response,
            ))
        }
    }

    /// <p><p> The <code>CreateAdditionalAssignmentsForHIT</code> operation increases the maximum number of assignments of an existing HIT. </p> <p> To extend the maximum number of assignments, specify the number of additional assignments.</p> <note> <ul> <li> <p>HITs created with fewer than 10 assignments cannot be extended to have 10 or more assignments. Attempting to add assignments in a way that brings the total number of assignments for a HIT from fewer than 10 assignments to 10 or more assignments will result in an <code>AWS.MechanicalTurk.InvalidMaximumAssignmentsIncrease</code> exception.</p> </li> <li> <p>HITs that were created before July 22, 2015 cannot be extended. Attempting to extend HITs that were created before July 22, 2015 will result in an <code>AWS.MechanicalTurk.HITTooOldForExtension</code> exception. </p> </li> </ul> </note></p>
    async fn create_additional_assignments_for_hit(
        &self,
        input: CreateAdditionalAssignmentsForHITRequest,
    ) -> Result<
        CreateAdditionalAssignmentsForHITResponse,
        RusotoError<CreateAdditionalAssignmentsForHITError>,
    > {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateAdditionalAssignmentsForHIT",
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
                .deserialize::<CreateAdditionalAssignmentsForHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAdditionalAssignmentsForHITError::from_response(
                response,
            ))
        }
    }

    /// <p><p>The <code>CreateHIT</code> operation creates a new Human Intelligence Task (HIT). The new HIT is made available for Workers to find and accept on the Amazon Mechanical Turk website. </p> <p> This operation allows you to specify a new HIT by passing in values for the properties of the HIT, such as its title, reward amount and number of assignments. When you pass these values to <code>CreateHIT</code>, a new HIT is created for you, with a new <code>HITTypeID</code>. The HITTypeID can be used to create additional HITs in the future without needing to specify common parameters such as the title, description and reward amount each time.</p> <p> An alternative way to create HITs is to first generate a HITTypeID using the <code>CreateHITType</code> operation and then call the <code>CreateHITWithHITType</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHIT also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>.</p> </note></p>
    async fn create_hit(
        &self,
        input: CreateHITRequest,
    ) -> Result<CreateHITResponse, RusotoError<CreateHITError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.CreateHIT");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHITError::from_response(response))
        }
    }

    /// <p> The <code>CreateHITType</code> operation creates a new HIT type. This operation allows you to define a standard set of HIT properties to use when creating HITs. If you register a HIT type with values that match an existing HIT type, the HIT type ID of the existing type will be returned. </p>
    async fn create_hit_type(
        &self,
        input: CreateHITTypeRequest,
    ) -> Result<CreateHITTypeResponse, RusotoError<CreateHITTypeError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateHITType",
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
            proto::json::ResponsePayload::new(&response).deserialize::<CreateHITTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHITTypeError::from_response(response))
        }
    }

    /// <p><p> The <code>CreateHITWithHITType</code> operation creates a new Human Intelligence Task (HIT) using an existing HITTypeID generated by the <code>CreateHITType</code> operation. </p> <p> This is an alternative way to create HITs from the <code>CreateHIT</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHITWithHITType also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>. </p> </note></p>
    async fn create_hit_with_hit_type(
        &self,
        input: CreateHITWithHITTypeRequest,
    ) -> Result<CreateHITWithHITTypeResponse, RusotoError<CreateHITWithHITTypeError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateHITWithHITType",
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
                .deserialize::<CreateHITWithHITTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHITWithHITTypeError::from_response(response))
        }
    }

    /// <p> The <code>CreateQualificationType</code> operation creates a new Qualification type, which is represented by a <code>QualificationType</code> data structure. </p>
    async fn create_qualification_type(
        &self,
        input: CreateQualificationTypeRequest,
    ) -> Result<CreateQualificationTypeResponse, RusotoError<CreateQualificationTypeError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateQualificationType",
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
                .deserialize::<CreateQualificationTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateQualificationTypeError::from_response(response))
        }
    }

    /// <p>The <code>CreateWorkerBlock</code> operation allows you to prevent a Worker from working on your HITs. For example, you can block a Worker who is producing poor quality work. You can block up to 100,000 Workers.</p>
    async fn create_worker_block(
        &self,
        input: CreateWorkerBlockRequest,
    ) -> Result<CreateWorkerBlockResponse, RusotoError<CreateWorkerBlockError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateWorkerBlock",
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
                .deserialize::<CreateWorkerBlockResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWorkerBlockError::from_response(response))
        }
    }

    /// <p><p> The <code>DeleteHIT</code> operation is used to delete HIT that is no longer needed. Only the Requester who created the HIT can delete it. </p> <p> You can only dispose of HITs that are in the <code>Reviewable</code> state, with all of their submitted assignments already either approved or rejected. If you call the DeleteHIT operation on a HIT that is not in the <code>Reviewable</code> state (for example, that has not expired, or still has active assignments), or on a HIT that is Reviewable but without all of its submitted assignments already approved or rejected, the service will return an error. </p> <note> <ul> <li> <p> HITs are automatically disposed of after 120 days. </p> </li> <li> <p> After you dispose of a HIT, you can no longer approve the HIT&#39;s rejected assignments. </p> </li> <li> <p> Disposed HITs are not returned in results for the ListHITs operation. </p> </li> <li> <p> Disposing HITs can improve the performance of operations such as ListReviewableHITs and ListHITs. </p> </li> </ul> </note></p>
    async fn delete_hit(
        &self,
        input: DeleteHITRequest,
    ) -> Result<DeleteHITResponse, RusotoError<DeleteHITError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.DeleteHIT");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteHITError::from_response(response))
        }
    }

    /// <p><p> The <code>DeleteQualificationType</code> deletes a Qualification type and deletes any HIT types that are associated with the Qualification type. </p> <p>This operation does not revoke Qualifications already assigned to Workers because the Qualifications might be needed for active HITs. If there are any pending requests for the Qualification type, Amazon Mechanical Turk rejects those requests. After you delete a Qualification type, you can no longer use it to create HITs or HIT types.</p> <note> <p>DeleteQualificationType must wait for all the HITs that use the deleted Qualification type to be deleted before completing. It may take up to 48 hours before DeleteQualificationType completes and the unique name of the Qualification type is available for reuse with CreateQualificationType.</p> </note></p>
    async fn delete_qualification_type(
        &self,
        input: DeleteQualificationTypeRequest,
    ) -> Result<DeleteQualificationTypeResponse, RusotoError<DeleteQualificationTypeError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.DeleteQualificationType",
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
                .deserialize::<DeleteQualificationTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteQualificationTypeError::from_response(response))
        }
    }

    /// <p>The <code>DeleteWorkerBlock</code> operation allows you to reinstate a blocked Worker to work on your HITs. This operation reverses the effects of the CreateWorkerBlock operation. You need the Worker ID to use this operation. If the Worker ID is missing or invalid, this operation fails and returns the message “WorkerId is invalid.” If the specified Worker is not blocked, this operation returns successfully.</p>
    async fn delete_worker_block(
        &self,
        input: DeleteWorkerBlockRequest,
    ) -> Result<DeleteWorkerBlockResponse, RusotoError<DeleteWorkerBlockError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.DeleteWorkerBlock",
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
                .deserialize::<DeleteWorkerBlockResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWorkerBlockError::from_response(response))
        }
    }

    /// <p> The <code>DisassociateQualificationFromWorker</code> revokes a previously granted Qualification from a user. </p> <p> You can provide a text message explaining why the Qualification was revoked. The user who had the Qualification can see this message. </p>
    async fn disassociate_qualification_from_worker(
        &self,
        input: DisassociateQualificationFromWorkerRequest,
    ) -> Result<
        DisassociateQualificationFromWorkerResponse,
        RusotoError<DisassociateQualificationFromWorkerError>,
    > {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.DisassociateQualificationFromWorker",
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
                .deserialize::<DisassociateQualificationFromWorkerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateQualificationFromWorkerError::from_response(
                response,
            ))
        }
    }

    /// <p>The <code>GetAccountBalance</code> operation retrieves the amount of money in your Amazon Mechanical Turk account.</p>
    async fn get_account_balance(
        &self,
    ) -> Result<GetAccountBalanceResponse, RusotoError<GetAccountBalanceError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetAccountBalance",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAccountBalanceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAccountBalanceError::from_response(response))
        }
    }

    /// <p> The <code>GetAssignment</code> operation retrieves the details of the specified Assignment. </p>
    async fn get_assignment(
        &self,
        input: GetAssignmentRequest,
    ) -> Result<GetAssignmentResponse, RusotoError<GetAssignmentError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetAssignment",
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
            proto::json::ResponsePayload::new(&response).deserialize::<GetAssignmentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAssignmentError::from_response(response))
        }
    }

    /// <p> The <code>GetFileUploadURL</code> operation generates and returns a temporary URL. You use the temporary URL to retrieve a file uploaded by a Worker as an answer to a FileUploadAnswer question for a HIT. The temporary URL is generated the instant the GetFileUploadURL operation is called, and is valid for 60 seconds. You can get a temporary file upload URL any time until the HIT is disposed. After the HIT is disposed, any uploaded files are deleted, and cannot be retrieved. Pending Deprecation on December 12, 2017. The Answer Specification structure will no longer support the <code>FileUploadAnswer</code> element to be used for the QuestionForm data structure. Instead, we recommend that Requesters who want to create HITs asking Workers to upload files to use Amazon S3. </p>
    async fn get_file_upload_url(
        &self,
        input: GetFileUploadURLRequest,
    ) -> Result<GetFileUploadURLResponse, RusotoError<GetFileUploadURLError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetFileUploadURL",
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
                .deserialize::<GetFileUploadURLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetFileUploadURLError::from_response(response))
        }
    }

    /// <p> The <code>GetHIT</code> operation retrieves the details of the specified HIT. </p>
    async fn get_hit(
        &self,
        input: GetHITRequest,
    ) -> Result<GetHITResponse, RusotoError<GetHITError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.GetHIT");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetHITError::from_response(response))
        }
    }

    /// <p> The <code>GetQualificationScore</code> operation returns the value of a Worker's Qualification for a given Qualification type. </p> <p> To get a Worker's Qualification, you must know the Worker's ID. The Worker's ID is included in the assignment data returned by the <code>ListAssignmentsForHIT</code> operation. </p> <p>Only the owner of a Qualification type can query the value of a Worker's Qualification of that type.</p>
    async fn get_qualification_score(
        &self,
        input: GetQualificationScoreRequest,
    ) -> Result<GetQualificationScoreResponse, RusotoError<GetQualificationScoreError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetQualificationScore",
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
                .deserialize::<GetQualificationScoreResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetQualificationScoreError::from_response(response))
        }
    }

    /// <p> The <code>GetQualificationType</code>operation retrieves information about a Qualification type using its ID. </p>
    async fn get_qualification_type(
        &self,
        input: GetQualificationTypeRequest,
    ) -> Result<GetQualificationTypeResponse, RusotoError<GetQualificationTypeError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetQualificationType",
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
                .deserialize::<GetQualificationTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetQualificationTypeError::from_response(response))
        }
    }

    /// <p> The <code>ListAssignmentsForHIT</code> operation retrieves completed assignments for a HIT. You can use this operation to retrieve the results for a HIT. </p> <p> You can get assignments for a HIT at any time, even if the HIT is not yet Reviewable. If a HIT requested multiple assignments, and has received some results but has not yet become Reviewable, you can still retrieve the partial results with this operation. </p> <p> Use the AssignmentStatus parameter to control which set of assignments for a HIT are returned. The ListAssignmentsForHIT operation can return submitted assignments awaiting approval, or it can return assignments that have already been approved or rejected. You can set AssignmentStatus=Approved,Rejected to get assignments that have already been approved and rejected together in one result set. </p> <p> Only the Requester who created the HIT can retrieve the assignments for that HIT. </p> <p> Results are sorted and divided into numbered pages and the operation returns a single page of results. You can use the parameters of the operation to control sorting and pagination. </p>
    async fn list_assignments_for_hit(
        &self,
        input: ListAssignmentsForHITRequest,
    ) -> Result<ListAssignmentsForHITResponse, RusotoError<ListAssignmentsForHITError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListAssignmentsForHIT",
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
                .deserialize::<ListAssignmentsForHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAssignmentsForHITError::from_response(response))
        }
    }

    /// <p> The <code>ListBonusPayments</code> operation retrieves the amounts of bonuses you have paid to Workers for a given HIT or assignment. </p>
    async fn list_bonus_payments(
        &self,
        input: ListBonusPaymentsRequest,
    ) -> Result<ListBonusPaymentsResponse, RusotoError<ListBonusPaymentsError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListBonusPayments",
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
                .deserialize::<ListBonusPaymentsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListBonusPaymentsError::from_response(response))
        }
    }

    /// <p> The <code>ListHITs</code> operation returns all of a Requester's HITs. The operation returns HITs of any status, except for HITs that have been deleted of with the DeleteHIT operation or that have been auto-deleted. </p>
    async fn list_hi_ts(
        &self,
        input: ListHITsRequest,
    ) -> Result<ListHITsResponse, RusotoError<ListHITsError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.ListHITs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListHITsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListHITsError::from_response(response))
        }
    }

    /// <p> The <code>ListHITsForQualificationType</code> operation returns the HITs that use the given Qualification type for a Qualification requirement. The operation returns HITs of any status, except for HITs that have been deleted with the <code>DeleteHIT</code> operation or that have been auto-deleted. </p>
    async fn list_hi_ts_for_qualification_type(
        &self,
        input: ListHITsForQualificationTypeRequest,
    ) -> Result<ListHITsForQualificationTypeResponse, RusotoError<ListHITsForQualificationTypeError>>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListHITsForQualificationType",
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
                .deserialize::<ListHITsForQualificationTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListHITsForQualificationTypeError::from_response(response))
        }
    }

    /// <p> The <code>ListQualificationRequests</code> operation retrieves requests for Qualifications of a particular Qualification type. The owner of the Qualification type calls this operation to poll for pending requests, and accepts them using the AcceptQualification operation. </p>
    async fn list_qualification_requests(
        &self,
        input: ListQualificationRequestsRequest,
    ) -> Result<ListQualificationRequestsResponse, RusotoError<ListQualificationRequestsError>>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListQualificationRequests",
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
                .deserialize::<ListQualificationRequestsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListQualificationRequestsError::from_response(response))
        }
    }

    /// <p> The <code>ListQualificationTypes</code> operation returns a list of Qualification types, filtered by an optional search term. </p>
    async fn list_qualification_types(
        &self,
        input: ListQualificationTypesRequest,
    ) -> Result<ListQualificationTypesResponse, RusotoError<ListQualificationTypesError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListQualificationTypes",
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
                .deserialize::<ListQualificationTypesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListQualificationTypesError::from_response(response))
        }
    }

    /// <p> The <code>ListReviewPolicyResultsForHIT</code> operation retrieves the computed results and the actions taken in the course of executing your Review Policies for a given HIT. For information about how to specify Review Policies when you call CreateHIT, see Review Policies. The ListReviewPolicyResultsForHIT operation can return results for both Assignment-level and HIT-level review results. </p>
    async fn list_review_policy_results_for_hit(
        &self,
        input: ListReviewPolicyResultsForHITRequest,
    ) -> Result<
        ListReviewPolicyResultsForHITResponse,
        RusotoError<ListReviewPolicyResultsForHITError>,
    > {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListReviewPolicyResultsForHIT",
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
                .deserialize::<ListReviewPolicyResultsForHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListReviewPolicyResultsForHITError::from_response(response))
        }
    }

    /// <p> The <code>ListReviewableHITs</code> operation retrieves the HITs with Status equal to Reviewable or Status equal to Reviewing that belong to the Requester calling the operation. </p>
    async fn list_reviewable_hi_ts(
        &self,
        input: ListReviewableHITsRequest,
    ) -> Result<ListReviewableHITsResponse, RusotoError<ListReviewableHITsError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListReviewableHITs",
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
                .deserialize::<ListReviewableHITsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListReviewableHITsError::from_response(response))
        }
    }

    /// <p>The <code>ListWorkersBlocks</code> operation retrieves a list of Workers who are blocked from working on your HITs.</p>
    async fn list_worker_blocks(
        &self,
        input: ListWorkerBlocksRequest,
    ) -> Result<ListWorkerBlocksResponse, RusotoError<ListWorkerBlocksError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListWorkerBlocks",
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
                .deserialize::<ListWorkerBlocksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorkerBlocksError::from_response(response))
        }
    }

    /// <p> The <code>ListWorkersWithQualificationType</code> operation returns all of the Workers that have been associated with a given Qualification type. </p>
    async fn list_workers_with_qualification_type(
        &self,
        input: ListWorkersWithQualificationTypeRequest,
    ) -> Result<
        ListWorkersWithQualificationTypeResponse,
        RusotoError<ListWorkersWithQualificationTypeError>,
    > {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListWorkersWithQualificationType",
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
                .deserialize::<ListWorkersWithQualificationTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorkersWithQualificationTypeError::from_response(
                response,
            ))
        }
    }

    /// <p> The <code>NotifyWorkers</code> operation sends an email to one or more Workers that you specify with the Worker ID. You can specify up to 100 Worker IDs to send the same message with a single call to the NotifyWorkers operation. The NotifyWorkers operation will send a notification email to a Worker only if you have previously approved or rejected work from the Worker. </p>
    async fn notify_workers(
        &self,
        input: NotifyWorkersRequest,
    ) -> Result<NotifyWorkersResponse, RusotoError<NotifyWorkersError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.NotifyWorkers",
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
            proto::json::ResponsePayload::new(&response).deserialize::<NotifyWorkersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(NotifyWorkersError::from_response(response))
        }
    }

    /// <p> The <code>RejectAssignment</code> operation rejects the results of a completed assignment. </p> <p> You can include an optional feedback message with the rejection, which the Worker can see in the Status section of the web site. When you include a feedback message with the rejection, it helps the Worker understand why the assignment was rejected, and can improve the quality of the results the Worker submits in the future. </p> <p> Only the Requester who created the HIT can reject an assignment for the HIT. </p>
    async fn reject_assignment(
        &self,
        input: RejectAssignmentRequest,
    ) -> Result<RejectAssignmentResponse, RusotoError<RejectAssignmentError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.RejectAssignment",
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
                .deserialize::<RejectAssignmentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RejectAssignmentError::from_response(response))
        }
    }

    /// <p> The <code>RejectQualificationRequest</code> operation rejects a user's request for a Qualification. </p> <p> You can provide a text message explaining why the request was rejected. The Worker who made the request can see this message.</p>
    async fn reject_qualification_request(
        &self,
        input: RejectQualificationRequestRequest,
    ) -> Result<RejectQualificationRequestResponse, RusotoError<RejectQualificationRequestError>>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.RejectQualificationRequest",
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
                .deserialize::<RejectQualificationRequestResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RejectQualificationRequestError::from_response(response))
        }
    }

    /// <p> The <code>SendBonus</code> operation issues a payment of money from your account to a Worker. This payment happens separately from the reward you pay to the Worker when you approve the Worker's assignment. The SendBonus operation requires the Worker's ID and the assignment ID as parameters to initiate payment of the bonus. You must include a message that explains the reason for the bonus payment, as the Worker may not be expecting the payment. Amazon Mechanical Turk collects a fee for bonus payments, similar to the HIT listing fee. This operation fails if your account does not have enough funds to pay for both the bonus and the fees. </p>
    async fn send_bonus(
        &self,
        input: SendBonusRequest,
    ) -> Result<SendBonusResponse, RusotoError<SendBonusError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.SendBonus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SendBonusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendBonusError::from_response(response))
        }
    }

    /// <p> The <code>SendTestEventNotification</code> operation causes Amazon Mechanical Turk to send a notification message as if a HIT event occurred, according to the provided notification specification. This allows you to test notifications without setting up notifications for a real HIT type and trying to trigger them using the website. When you call this operation, the service attempts to send the test notification immediately. </p>
    async fn send_test_event_notification(
        &self,
        input: SendTestEventNotificationRequest,
    ) -> Result<SendTestEventNotificationResponse, RusotoError<SendTestEventNotificationError>>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.SendTestEventNotification",
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
                .deserialize::<SendTestEventNotificationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendTestEventNotificationError::from_response(response))
        }
    }

    /// <p> The <code>UpdateExpirationForHIT</code> operation allows you update the expiration time of a HIT. If you update it to a time in the past, the HIT will be immediately expired. </p>
    async fn update_expiration_for_hit(
        &self,
        input: UpdateExpirationForHITRequest,
    ) -> Result<UpdateExpirationForHITResponse, RusotoError<UpdateExpirationForHITError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateExpirationForHIT",
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
                .deserialize::<UpdateExpirationForHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateExpirationForHITError::from_response(response))
        }
    }

    /// <p> The <code>UpdateHITReviewStatus</code> operation updates the status of a HIT. If the status is Reviewable, this operation can update the status to Reviewing, or it can revert a Reviewing HIT back to the Reviewable status. </p>
    async fn update_hit_review_status(
        &self,
        input: UpdateHITReviewStatusRequest,
    ) -> Result<UpdateHITReviewStatusResponse, RusotoError<UpdateHITReviewStatusError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateHITReviewStatus",
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
                .deserialize::<UpdateHITReviewStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateHITReviewStatusError::from_response(response))
        }
    }

    /// <p> The <code>UpdateHITTypeOfHIT</code> operation allows you to change the HITType properties of a HIT. This operation disassociates the HIT from its old HITType properties and associates it with the new HITType properties. The HIT takes on the properties of the new HITType in place of the old ones. </p>
    async fn update_hit_type_of_hit(
        &self,
        input: UpdateHITTypeOfHITRequest,
    ) -> Result<UpdateHITTypeOfHITResponse, RusotoError<UpdateHITTypeOfHITError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateHITTypeOfHIT",
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
                .deserialize::<UpdateHITTypeOfHITResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateHITTypeOfHITError::from_response(response))
        }
    }

    /// <p> The <code>UpdateNotificationSettings</code> operation creates, updates, disables or re-enables notifications for a HIT type. If you call the UpdateNotificationSettings operation for a HIT type that already has a notification specification, the operation replaces the old specification with a new one. You can call the UpdateNotificationSettings operation to enable or disable notifications for the HIT type, without having to modify the notification specification itself by providing updates to the Active status without specifying a new notification specification. To change the Active status of a HIT type's notifications, the HIT type must already have a notification specification, or one must be provided in the same call to <code>UpdateNotificationSettings</code>. </p>
    async fn update_notification_settings(
        &self,
        input: UpdateNotificationSettingsRequest,
    ) -> Result<UpdateNotificationSettingsResponse, RusotoError<UpdateNotificationSettingsError>>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateNotificationSettings",
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
                .deserialize::<UpdateNotificationSettingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNotificationSettingsError::from_response(response))
        }
    }

    /// <p> The <code>UpdateQualificationType</code> operation modifies the attributes of an existing Qualification type, which is represented by a QualificationType data structure. Only the owner of a Qualification type can modify its attributes. </p> <p> Most attributes of a Qualification type can be changed after the type has been created. However, the Name and Keywords fields cannot be modified. The RetryDelayInSeconds parameter can be modified or added to change the delay or to enable retries, but RetryDelayInSeconds cannot be used to disable retries. </p> <p> You can use this operation to update the test for a Qualification type. The test is updated based on the values specified for the Test, TestDurationInSeconds and AnswerKey parameters. All three parameters specify the updated test. If you are updating the test for a type, you must specify the Test and TestDurationInSeconds parameters. The AnswerKey parameter is optional; omitting it specifies that the updated test does not have an answer key. </p> <p> If you omit the Test parameter, the test for the Qualification type is unchanged. There is no way to remove a test from a Qualification type that has one. If the type already has a test, you cannot update it to be AutoGranted. If the Qualification type does not have a test and one is provided by an update, the type will henceforth have a test. </p> <p> If you want to update the test duration or answer key for an existing test without changing the questions, you must specify a Test parameter with the original questions, along with the updated values. </p> <p> If you provide an updated Test but no AnswerKey, the new test will not have an answer key. Requests for such Qualifications must be granted manually. </p> <p> You can also update the AutoGranted and AutoGrantedValue attributes of the Qualification type.</p>
    async fn update_qualification_type(
        &self,
        input: UpdateQualificationTypeRequest,
    ) -> Result<UpdateQualificationTypeResponse, RusotoError<UpdateQualificationTypeError>> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateQualificationType",
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
                .deserialize::<UpdateQualificationTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateQualificationTypeError::from_response(response))
        }
    }
}
