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
pub struct AcceptQualificationRequestResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct ApproveAssignmentResponse {}

/// <p> The Assignment data structure represents a single assignment of a HIT to a Worker. The assignment tracks the Worker's efforts to complete the HIT, and contains the results for later retrieval. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct AssociateQualificationWithWorkerResponse {}

/// <p>An object representing a Bonus payment paid to a Worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct CreateAdditionalAssignmentsForHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateHITResponse {
    /// <p> Contains the newly created HIT data. For a description of the HIT data structure as it appears in responses, see the HIT Data Structure documentation. </p>
    #[serde(rename = "HIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<HIT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateHITTypeResponse {
    /// <p> The ID of the newly registered HIT type.</p>
    #[serde(rename = "HITTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_type_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateHITWithHITTypeResponse {
    /// <p> Contains the newly created HIT data. For a description of the HIT data structure as it appears in responses, see the HIT Data Structure documentation. </p>
    #[serde(rename = "HIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<HIT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateQualificationTypeResponse {
    /// <p>The created Qualification type, returned as a QualificationType data structure.</p>
    #[serde(rename = "QualificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type: Option<QualificationType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWorkerBlockRequest {
    /// <p>A message explaining the reason for blocking the Worker. This parameter enables you to keep track of your Workers. The Worker does not see this message.</p>
    #[serde(rename = "Reason")]
    pub reason: String,
    /// <p>The ID of the Worker to block.</p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateWorkerBlockResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteHITRequest {
    /// <p>The ID of the HIT to be deleted.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteQualificationTypeRequest {
    /// <p>The ID of the QualificationType to dispose.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteQualificationTypeResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DeleteWorkerBlockResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DisassociateQualificationFromWorkerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountBalanceRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAccountBalanceResponse {
    #[serde(rename = "AvailableBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<String>,
    #[serde(rename = "OnHoldBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_balance: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAssignmentRequest {
    /// <p>The ID of the Assignment to be retrieved.</p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetFileUploadURLRequest {
    /// <p>The ID of the assignment that contains the question with a FileUploadAnswer.</p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
    /// <p>The identifier of the question with a FileUploadAnswer, as specified in the QuestionForm of the HIT.</p>
    #[serde(rename = "QuestionIdentifier")]
    pub question_identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetFileUploadURLResponse {
    /// <p> A temporary URL for the file that the Worker uploaded for the answer. </p>
    #[serde(rename = "FileUploadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_upload_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetHITRequest {
    /// <p>The ID of the HIT to be retrieved.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetHITResponse {
    /// <p> Contains the requested HIT data.</p>
    #[serde(rename = "HIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit: Option<HIT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetQualificationScoreRequest {
    /// <p>The ID of the QualificationType.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    #[serde(rename = "WorkerId")]
    pub worker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetQualificationScoreResponse {
    /// <p> The Qualification data structure of the Qualification assigned to a user, including the Qualification type and the value (score). </p>
    #[serde(rename = "Qualification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification: Option<Qualification>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetQualificationTypeRequest {
    /// <p>The ID of the QualificationType.</p>
    #[serde(rename = "QualificationTypeId")]
    pub qualification_type_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetQualificationTypeResponse {
    /// <p> The returned Qualification Type</p>
    #[serde(rename = "QualificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type: Option<QualificationType>,
}

/// <p> The HIT data structure represents a single HIT, including all the information necessary for a Worker to accept and complete the HIT.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct HITLayoutParameter {
    /// <p> The name of the parameter in the HITLayout. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value substituted for the parameter referenced in the HITLayout. </p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct RejectAssignmentRequest {
    /// <p> The ID of the assignment. The assignment must correspond to a HIT created by the Requester. </p>
    #[serde(rename = "AssignmentId")]
    pub assignment_id: String,
    /// <p> A message for the Worker, which the Worker can see in the Status section of the web site. </p>
    #[serde(rename = "RequesterFeedback")]
    pub requester_feedback: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RejectAssignmentResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct RejectQualificationRequestResponse {}

/// <p> Both the AssignmentReviewReport and the HITReviewReport elements contains the ReviewActionDetail data structure. This structure is returned multiple times for each action specified in the Review Policy. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct SendBonusResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendTestEventNotificationRequest {
    /// <p> The notification specification to test. This value is identical to the value you would provide to the UpdateNotificationSettings operation when you establish the notification specification for a HIT type. </p>
    #[serde(rename = "Notification")]
    pub notification: NotificationSpecification,
    /// <p> The event to simulate to test the notification specification. This event is included in the test message even if the notification specification does not include the event type. The notification specification does not filter out the test event. </p>
    #[serde(rename = "TestEventType")]
    pub test_event_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SendTestEventNotificationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateExpirationForHITRequest {
    /// <p> The date and time at which you want the HIT to expire </p>
    #[serde(rename = "ExpireAt")]
    pub expire_at: f64,
    /// <p> The HIT to update. </p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateExpirationForHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateHITReviewStatusResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateHITTypeOfHITRequest {
    /// <p>The HIT to update.</p>
    #[serde(rename = "HITId")]
    pub hit_id: String,
    /// <p>The ID of the new HIT type.</p>
    #[serde(rename = "HITTypeId")]
    pub hit_type_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateHITTypeOfHITResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateNotificationSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateQualificationTypeResponse {
    /// <p> Contains a QualificationType data structure.</p>
    #[serde(rename = "QualificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_type: Option<QualificationType>,
}

/// <p> The WorkerBlock data structure represents a Worker who has been blocked. It has two elements: the WorkerId and the Reason for the block. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AcceptQualificationRequestError {
    pub fn from_body(body: &str) -> AcceptQualificationRequestError {
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
                    "RequestError" => {
                        AcceptQualificationRequestError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        AcceptQualificationRequestError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        AcceptQualificationRequestError::Validation(error_message.to_string())
                    }
                    _ => AcceptQualificationRequestError::Unknown(String::from(body)),
                }
            }
            Err(_) => AcceptQualificationRequestError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AcceptQualificationRequestError {
    fn from(err: serde_json::error::Error) -> AcceptQualificationRequestError {
        AcceptQualificationRequestError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptQualificationRequestError {
    fn from(err: CredentialsError) -> AcceptQualificationRequestError {
        AcceptQualificationRequestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptQualificationRequestError {
    fn from(err: HttpDispatchError) -> AcceptQualificationRequestError {
        AcceptQualificationRequestError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcceptQualificationRequestError {
    fn from(err: io::Error) -> AcceptQualificationRequestError {
        AcceptQualificationRequestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcceptQualificationRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptQualificationRequestError {
    fn description(&self) -> &str {
        match *self {
            AcceptQualificationRequestError::RequestError(ref cause) => cause,
            AcceptQualificationRequestError::ServiceFault(ref cause) => cause,
            AcceptQualificationRequestError::Validation(ref cause) => cause,
            AcceptQualificationRequestError::Credentials(ref err) => err.description(),
            AcceptQualificationRequestError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AcceptQualificationRequestError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ApproveAssignment
#[derive(Debug, PartialEq)]
pub enum ApproveAssignmentError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ApproveAssignmentError {
    pub fn from_body(body: &str) -> ApproveAssignmentError {
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
                    "RequestError" => {
                        ApproveAssignmentError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ApproveAssignmentError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ApproveAssignmentError::Validation(error_message.to_string())
                    }
                    _ => ApproveAssignmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => ApproveAssignmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ApproveAssignmentError {
    fn from(err: serde_json::error::Error) -> ApproveAssignmentError {
        ApproveAssignmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ApproveAssignmentError {
    fn from(err: CredentialsError) -> ApproveAssignmentError {
        ApproveAssignmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ApproveAssignmentError {
    fn from(err: HttpDispatchError) -> ApproveAssignmentError {
        ApproveAssignmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for ApproveAssignmentError {
    fn from(err: io::Error) -> ApproveAssignmentError {
        ApproveAssignmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ApproveAssignmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApproveAssignmentError {
    fn description(&self) -> &str {
        match *self {
            ApproveAssignmentError::RequestError(ref cause) => cause,
            ApproveAssignmentError::ServiceFault(ref cause) => cause,
            ApproveAssignmentError::Validation(ref cause) => cause,
            ApproveAssignmentError::Credentials(ref err) => err.description(),
            ApproveAssignmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ApproveAssignmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateQualificationWithWorker
#[derive(Debug, PartialEq)]
pub enum AssociateQualificationWithWorkerError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateQualificationWithWorkerError {
    pub fn from_body(body: &str) -> AssociateQualificationWithWorkerError {
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
                    "RequestError" => AssociateQualificationWithWorkerError::RequestError(
                        String::from(error_message),
                    ),
                    "ServiceFault" => AssociateQualificationWithWorkerError::ServiceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        AssociateQualificationWithWorkerError::Validation(error_message.to_string())
                    }
                    _ => AssociateQualificationWithWorkerError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateQualificationWithWorkerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateQualificationWithWorkerError {
    fn from(err: serde_json::error::Error) -> AssociateQualificationWithWorkerError {
        AssociateQualificationWithWorkerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateQualificationWithWorkerError {
    fn from(err: CredentialsError) -> AssociateQualificationWithWorkerError {
        AssociateQualificationWithWorkerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateQualificationWithWorkerError {
    fn from(err: HttpDispatchError) -> AssociateQualificationWithWorkerError {
        AssociateQualificationWithWorkerError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateQualificationWithWorkerError {
    fn from(err: io::Error) -> AssociateQualificationWithWorkerError {
        AssociateQualificationWithWorkerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateQualificationWithWorkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateQualificationWithWorkerError {
    fn description(&self) -> &str {
        match *self {
            AssociateQualificationWithWorkerError::RequestError(ref cause) => cause,
            AssociateQualificationWithWorkerError::ServiceFault(ref cause) => cause,
            AssociateQualificationWithWorkerError::Validation(ref cause) => cause,
            AssociateQualificationWithWorkerError::Credentials(ref err) => err.description(),
            AssociateQualificationWithWorkerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateQualificationWithWorkerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAdditionalAssignmentsForHIT
#[derive(Debug, PartialEq)]
pub enum CreateAdditionalAssignmentsForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAdditionalAssignmentsForHITError {
    pub fn from_body(body: &str) -> CreateAdditionalAssignmentsForHITError {
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
                    "RequestError" => CreateAdditionalAssignmentsForHITError::RequestError(
                        String::from(error_message),
                    ),
                    "ServiceFault" => CreateAdditionalAssignmentsForHITError::ServiceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => CreateAdditionalAssignmentsForHITError::Validation(
                        error_message.to_string(),
                    ),
                    _ => CreateAdditionalAssignmentsForHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAdditionalAssignmentsForHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAdditionalAssignmentsForHITError {
    fn from(err: serde_json::error::Error) -> CreateAdditionalAssignmentsForHITError {
        CreateAdditionalAssignmentsForHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAdditionalAssignmentsForHITError {
    fn from(err: CredentialsError) -> CreateAdditionalAssignmentsForHITError {
        CreateAdditionalAssignmentsForHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAdditionalAssignmentsForHITError {
    fn from(err: HttpDispatchError) -> CreateAdditionalAssignmentsForHITError {
        CreateAdditionalAssignmentsForHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAdditionalAssignmentsForHITError {
    fn from(err: io::Error) -> CreateAdditionalAssignmentsForHITError {
        CreateAdditionalAssignmentsForHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAdditionalAssignmentsForHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAdditionalAssignmentsForHITError {
    fn description(&self) -> &str {
        match *self {
            CreateAdditionalAssignmentsForHITError::RequestError(ref cause) => cause,
            CreateAdditionalAssignmentsForHITError::ServiceFault(ref cause) => cause,
            CreateAdditionalAssignmentsForHITError::Validation(ref cause) => cause,
            CreateAdditionalAssignmentsForHITError::Credentials(ref err) => err.description(),
            CreateAdditionalAssignmentsForHITError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAdditionalAssignmentsForHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHIT
#[derive(Debug, PartialEq)]
pub enum CreateHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHITError {
    pub fn from_body(body: &str) -> CreateHITError {
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
                    "RequestError" => CreateHITError::RequestError(String::from(error_message)),
                    "ServiceFault" => CreateHITError::ServiceFault(String::from(error_message)),
                    "ValidationException" => CreateHITError::Validation(error_message.to_string()),
                    _ => CreateHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateHITError {
    fn from(err: serde_json::error::Error) -> CreateHITError {
        CreateHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateHITError {
    fn from(err: CredentialsError) -> CreateHITError {
        CreateHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHITError {
    fn from(err: HttpDispatchError) -> CreateHITError {
        CreateHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHITError {
    fn from(err: io::Error) -> CreateHITError {
        CreateHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHITError {
    fn description(&self) -> &str {
        match *self {
            CreateHITError::RequestError(ref cause) => cause,
            CreateHITError::ServiceFault(ref cause) => cause,
            CreateHITError::Validation(ref cause) => cause,
            CreateHITError::Credentials(ref err) => err.description(),
            CreateHITError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHITType
#[derive(Debug, PartialEq)]
pub enum CreateHITTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHITTypeError {
    pub fn from_body(body: &str) -> CreateHITTypeError {
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
                    "RequestError" => CreateHITTypeError::RequestError(String::from(error_message)),
                    "ServiceFault" => CreateHITTypeError::ServiceFault(String::from(error_message)),
                    "ValidationException" => {
                        CreateHITTypeError::Validation(error_message.to_string())
                    }
                    _ => CreateHITTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateHITTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateHITTypeError {
    fn from(err: serde_json::error::Error) -> CreateHITTypeError {
        CreateHITTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateHITTypeError {
    fn from(err: CredentialsError) -> CreateHITTypeError {
        CreateHITTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHITTypeError {
    fn from(err: HttpDispatchError) -> CreateHITTypeError {
        CreateHITTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHITTypeError {
    fn from(err: io::Error) -> CreateHITTypeError {
        CreateHITTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHITTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHITTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateHITTypeError::RequestError(ref cause) => cause,
            CreateHITTypeError::ServiceFault(ref cause) => cause,
            CreateHITTypeError::Validation(ref cause) => cause,
            CreateHITTypeError::Credentials(ref err) => err.description(),
            CreateHITTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateHITTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHITWithHITType
#[derive(Debug, PartialEq)]
pub enum CreateHITWithHITTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHITWithHITTypeError {
    pub fn from_body(body: &str) -> CreateHITWithHITTypeError {
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
                    "RequestError" => {
                        CreateHITWithHITTypeError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        CreateHITWithHITTypeError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateHITWithHITTypeError::Validation(error_message.to_string())
                    }
                    _ => CreateHITWithHITTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateHITWithHITTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateHITWithHITTypeError {
    fn from(err: serde_json::error::Error) -> CreateHITWithHITTypeError {
        CreateHITWithHITTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateHITWithHITTypeError {
    fn from(err: CredentialsError) -> CreateHITWithHITTypeError {
        CreateHITWithHITTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHITWithHITTypeError {
    fn from(err: HttpDispatchError) -> CreateHITWithHITTypeError {
        CreateHITWithHITTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHITWithHITTypeError {
    fn from(err: io::Error) -> CreateHITWithHITTypeError {
        CreateHITWithHITTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHITWithHITTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHITWithHITTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateHITWithHITTypeError::RequestError(ref cause) => cause,
            CreateHITWithHITTypeError::ServiceFault(ref cause) => cause,
            CreateHITWithHITTypeError::Validation(ref cause) => cause,
            CreateHITWithHITTypeError::Credentials(ref err) => err.description(),
            CreateHITWithHITTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateHITWithHITTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateQualificationType
#[derive(Debug, PartialEq)]
pub enum CreateQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateQualificationTypeError {
    pub fn from_body(body: &str) -> CreateQualificationTypeError {
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
                    "RequestError" => {
                        CreateQualificationTypeError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        CreateQualificationTypeError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateQualificationTypeError::Validation(error_message.to_string())
                    }
                    _ => CreateQualificationTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateQualificationTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateQualificationTypeError {
    fn from(err: serde_json::error::Error) -> CreateQualificationTypeError {
        CreateQualificationTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateQualificationTypeError {
    fn from(err: CredentialsError) -> CreateQualificationTypeError {
        CreateQualificationTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateQualificationTypeError {
    fn from(err: HttpDispatchError) -> CreateQualificationTypeError {
        CreateQualificationTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateQualificationTypeError {
    fn from(err: io::Error) -> CreateQualificationTypeError {
        CreateQualificationTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateQualificationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateQualificationTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateQualificationTypeError::RequestError(ref cause) => cause,
            CreateQualificationTypeError::ServiceFault(ref cause) => cause,
            CreateQualificationTypeError::Validation(ref cause) => cause,
            CreateQualificationTypeError::Credentials(ref err) => err.description(),
            CreateQualificationTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateQualificationTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWorkerBlock
#[derive(Debug, PartialEq)]
pub enum CreateWorkerBlockError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateWorkerBlockError {
    pub fn from_body(body: &str) -> CreateWorkerBlockError {
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
                    "RequestError" => {
                        CreateWorkerBlockError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        CreateWorkerBlockError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateWorkerBlockError::Validation(error_message.to_string())
                    }
                    _ => CreateWorkerBlockError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateWorkerBlockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateWorkerBlockError {
    fn from(err: serde_json::error::Error) -> CreateWorkerBlockError {
        CreateWorkerBlockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateWorkerBlockError {
    fn from(err: CredentialsError) -> CreateWorkerBlockError {
        CreateWorkerBlockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateWorkerBlockError {
    fn from(err: HttpDispatchError) -> CreateWorkerBlockError {
        CreateWorkerBlockError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateWorkerBlockError {
    fn from(err: io::Error) -> CreateWorkerBlockError {
        CreateWorkerBlockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateWorkerBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWorkerBlockError {
    fn description(&self) -> &str {
        match *self {
            CreateWorkerBlockError::RequestError(ref cause) => cause,
            CreateWorkerBlockError::ServiceFault(ref cause) => cause,
            CreateWorkerBlockError::Validation(ref cause) => cause,
            CreateWorkerBlockError::Credentials(ref err) => err.description(),
            CreateWorkerBlockError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateWorkerBlockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHIT
#[derive(Debug, PartialEq)]
pub enum DeleteHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteHITError {
    pub fn from_body(body: &str) -> DeleteHITError {
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
                    "RequestError" => DeleteHITError::RequestError(String::from(error_message)),
                    "ServiceFault" => DeleteHITError::ServiceFault(String::from(error_message)),
                    "ValidationException" => DeleteHITError::Validation(error_message.to_string()),
                    _ => DeleteHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteHITError {
    fn from(err: serde_json::error::Error) -> DeleteHITError {
        DeleteHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteHITError {
    fn from(err: CredentialsError) -> DeleteHITError {
        DeleteHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteHITError {
    fn from(err: HttpDispatchError) -> DeleteHITError {
        DeleteHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteHITError {
    fn from(err: io::Error) -> DeleteHITError {
        DeleteHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHITError {
    fn description(&self) -> &str {
        match *self {
            DeleteHITError::RequestError(ref cause) => cause,
            DeleteHITError::ServiceFault(ref cause) => cause,
            DeleteHITError::Validation(ref cause) => cause,
            DeleteHITError::Credentials(ref err) => err.description(),
            DeleteHITError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteQualificationType
#[derive(Debug, PartialEq)]
pub enum DeleteQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteQualificationTypeError {
    pub fn from_body(body: &str) -> DeleteQualificationTypeError {
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
                    "RequestError" => {
                        DeleteQualificationTypeError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        DeleteQualificationTypeError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteQualificationTypeError::Validation(error_message.to_string())
                    }
                    _ => DeleteQualificationTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteQualificationTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteQualificationTypeError {
    fn from(err: serde_json::error::Error) -> DeleteQualificationTypeError {
        DeleteQualificationTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteQualificationTypeError {
    fn from(err: CredentialsError) -> DeleteQualificationTypeError {
        DeleteQualificationTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteQualificationTypeError {
    fn from(err: HttpDispatchError) -> DeleteQualificationTypeError {
        DeleteQualificationTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteQualificationTypeError {
    fn from(err: io::Error) -> DeleteQualificationTypeError {
        DeleteQualificationTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteQualificationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteQualificationTypeError {
    fn description(&self) -> &str {
        match *self {
            DeleteQualificationTypeError::RequestError(ref cause) => cause,
            DeleteQualificationTypeError::ServiceFault(ref cause) => cause,
            DeleteQualificationTypeError::Validation(ref cause) => cause,
            DeleteQualificationTypeError::Credentials(ref err) => err.description(),
            DeleteQualificationTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteQualificationTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWorkerBlock
#[derive(Debug, PartialEq)]
pub enum DeleteWorkerBlockError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteWorkerBlockError {
    pub fn from_body(body: &str) -> DeleteWorkerBlockError {
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
                    "RequestError" => {
                        DeleteWorkerBlockError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        DeleteWorkerBlockError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteWorkerBlockError::Validation(error_message.to_string())
                    }
                    _ => DeleteWorkerBlockError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteWorkerBlockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteWorkerBlockError {
    fn from(err: serde_json::error::Error) -> DeleteWorkerBlockError {
        DeleteWorkerBlockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteWorkerBlockError {
    fn from(err: CredentialsError) -> DeleteWorkerBlockError {
        DeleteWorkerBlockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteWorkerBlockError {
    fn from(err: HttpDispatchError) -> DeleteWorkerBlockError {
        DeleteWorkerBlockError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteWorkerBlockError {
    fn from(err: io::Error) -> DeleteWorkerBlockError {
        DeleteWorkerBlockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteWorkerBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWorkerBlockError {
    fn description(&self) -> &str {
        match *self {
            DeleteWorkerBlockError::RequestError(ref cause) => cause,
            DeleteWorkerBlockError::ServiceFault(ref cause) => cause,
            DeleteWorkerBlockError::Validation(ref cause) => cause,
            DeleteWorkerBlockError::Credentials(ref err) => err.description(),
            DeleteWorkerBlockError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteWorkerBlockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateQualificationFromWorker
#[derive(Debug, PartialEq)]
pub enum DisassociateQualificationFromWorkerError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateQualificationFromWorkerError {
    pub fn from_body(body: &str) -> DisassociateQualificationFromWorkerError {
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
                    "RequestError" => DisassociateQualificationFromWorkerError::RequestError(
                        String::from(error_message),
                    ),
                    "ServiceFault" => DisassociateQualificationFromWorkerError::ServiceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => DisassociateQualificationFromWorkerError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DisassociateQualificationFromWorkerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateQualificationFromWorkerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateQualificationFromWorkerError {
    fn from(err: serde_json::error::Error) -> DisassociateQualificationFromWorkerError {
        DisassociateQualificationFromWorkerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateQualificationFromWorkerError {
    fn from(err: CredentialsError) -> DisassociateQualificationFromWorkerError {
        DisassociateQualificationFromWorkerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateQualificationFromWorkerError {
    fn from(err: HttpDispatchError) -> DisassociateQualificationFromWorkerError {
        DisassociateQualificationFromWorkerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateQualificationFromWorkerError {
    fn from(err: io::Error) -> DisassociateQualificationFromWorkerError {
        DisassociateQualificationFromWorkerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateQualificationFromWorkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateQualificationFromWorkerError {
    fn description(&self) -> &str {
        match *self {
            DisassociateQualificationFromWorkerError::RequestError(ref cause) => cause,
            DisassociateQualificationFromWorkerError::ServiceFault(ref cause) => cause,
            DisassociateQualificationFromWorkerError::Validation(ref cause) => cause,
            DisassociateQualificationFromWorkerError::Credentials(ref err) => err.description(),
            DisassociateQualificationFromWorkerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateQualificationFromWorkerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccountBalance
#[derive(Debug, PartialEq)]
pub enum GetAccountBalanceError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAccountBalanceError {
    pub fn from_body(body: &str) -> GetAccountBalanceError {
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
                    "RequestError" => {
                        GetAccountBalanceError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        GetAccountBalanceError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAccountBalanceError::Validation(error_message.to_string())
                    }
                    _ => GetAccountBalanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAccountBalanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAccountBalanceError {
    fn from(err: serde_json::error::Error) -> GetAccountBalanceError {
        GetAccountBalanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAccountBalanceError {
    fn from(err: CredentialsError) -> GetAccountBalanceError {
        GetAccountBalanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAccountBalanceError {
    fn from(err: HttpDispatchError) -> GetAccountBalanceError {
        GetAccountBalanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAccountBalanceError {
    fn from(err: io::Error) -> GetAccountBalanceError {
        GetAccountBalanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAccountBalanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountBalanceError {
    fn description(&self) -> &str {
        match *self {
            GetAccountBalanceError::RequestError(ref cause) => cause,
            GetAccountBalanceError::ServiceFault(ref cause) => cause,
            GetAccountBalanceError::Validation(ref cause) => cause,
            GetAccountBalanceError::Credentials(ref err) => err.description(),
            GetAccountBalanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAccountBalanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAssignment
#[derive(Debug, PartialEq)]
pub enum GetAssignmentError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAssignmentError {
    pub fn from_body(body: &str) -> GetAssignmentError {
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
                    "RequestError" => GetAssignmentError::RequestError(String::from(error_message)),
                    "ServiceFault" => GetAssignmentError::ServiceFault(String::from(error_message)),
                    "ValidationException" => {
                        GetAssignmentError::Validation(error_message.to_string())
                    }
                    _ => GetAssignmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAssignmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAssignmentError {
    fn from(err: serde_json::error::Error) -> GetAssignmentError {
        GetAssignmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAssignmentError {
    fn from(err: CredentialsError) -> GetAssignmentError {
        GetAssignmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAssignmentError {
    fn from(err: HttpDispatchError) -> GetAssignmentError {
        GetAssignmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAssignmentError {
    fn from(err: io::Error) -> GetAssignmentError {
        GetAssignmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAssignmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAssignmentError {
    fn description(&self) -> &str {
        match *self {
            GetAssignmentError::RequestError(ref cause) => cause,
            GetAssignmentError::ServiceFault(ref cause) => cause,
            GetAssignmentError::Validation(ref cause) => cause,
            GetAssignmentError::Credentials(ref err) => err.description(),
            GetAssignmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAssignmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFileUploadURL
#[derive(Debug, PartialEq)]
pub enum GetFileUploadURLError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetFileUploadURLError {
    pub fn from_body(body: &str) -> GetFileUploadURLError {
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
                    "RequestError" => {
                        GetFileUploadURLError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        GetFileUploadURLError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetFileUploadURLError::Validation(error_message.to_string())
                    }
                    _ => GetFileUploadURLError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFileUploadURLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFileUploadURLError {
    fn from(err: serde_json::error::Error) -> GetFileUploadURLError {
        GetFileUploadURLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFileUploadURLError {
    fn from(err: CredentialsError) -> GetFileUploadURLError {
        GetFileUploadURLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFileUploadURLError {
    fn from(err: HttpDispatchError) -> GetFileUploadURLError {
        GetFileUploadURLError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFileUploadURLError {
    fn from(err: io::Error) -> GetFileUploadURLError {
        GetFileUploadURLError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFileUploadURLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFileUploadURLError {
    fn description(&self) -> &str {
        match *self {
            GetFileUploadURLError::RequestError(ref cause) => cause,
            GetFileUploadURLError::ServiceFault(ref cause) => cause,
            GetFileUploadURLError::Validation(ref cause) => cause,
            GetFileUploadURLError::Credentials(ref err) => err.description(),
            GetFileUploadURLError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFileUploadURLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHIT
#[derive(Debug, PartialEq)]
pub enum GetHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHITError {
    pub fn from_body(body: &str) -> GetHITError {
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
                    "RequestError" => GetHITError::RequestError(String::from(error_message)),
                    "ServiceFault" => GetHITError::ServiceFault(String::from(error_message)),
                    "ValidationException" => GetHITError::Validation(error_message.to_string()),
                    _ => GetHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetHITError {
    fn from(err: serde_json::error::Error) -> GetHITError {
        GetHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetHITError {
    fn from(err: CredentialsError) -> GetHITError {
        GetHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHITError {
    fn from(err: HttpDispatchError) -> GetHITError {
        GetHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHITError {
    fn from(err: io::Error) -> GetHITError {
        GetHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHITError {
    fn description(&self) -> &str {
        match *self {
            GetHITError::RequestError(ref cause) => cause,
            GetHITError::ServiceFault(ref cause) => cause,
            GetHITError::Validation(ref cause) => cause,
            GetHITError::Credentials(ref err) => err.description(),
            GetHITError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetQualificationScore
#[derive(Debug, PartialEq)]
pub enum GetQualificationScoreError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetQualificationScoreError {
    pub fn from_body(body: &str) -> GetQualificationScoreError {
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
                    "RequestError" => {
                        GetQualificationScoreError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        GetQualificationScoreError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetQualificationScoreError::Validation(error_message.to_string())
                    }
                    _ => GetQualificationScoreError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetQualificationScoreError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetQualificationScoreError {
    fn from(err: serde_json::error::Error) -> GetQualificationScoreError {
        GetQualificationScoreError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetQualificationScoreError {
    fn from(err: CredentialsError) -> GetQualificationScoreError {
        GetQualificationScoreError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQualificationScoreError {
    fn from(err: HttpDispatchError) -> GetQualificationScoreError {
        GetQualificationScoreError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQualificationScoreError {
    fn from(err: io::Error) -> GetQualificationScoreError {
        GetQualificationScoreError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetQualificationScoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQualificationScoreError {
    fn description(&self) -> &str {
        match *self {
            GetQualificationScoreError::RequestError(ref cause) => cause,
            GetQualificationScoreError::ServiceFault(ref cause) => cause,
            GetQualificationScoreError::Validation(ref cause) => cause,
            GetQualificationScoreError::Credentials(ref err) => err.description(),
            GetQualificationScoreError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetQualificationScoreError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetQualificationType
#[derive(Debug, PartialEq)]
pub enum GetQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetQualificationTypeError {
    pub fn from_body(body: &str) -> GetQualificationTypeError {
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
                    "RequestError" => {
                        GetQualificationTypeError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        GetQualificationTypeError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetQualificationTypeError::Validation(error_message.to_string())
                    }
                    _ => GetQualificationTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetQualificationTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetQualificationTypeError {
    fn from(err: serde_json::error::Error) -> GetQualificationTypeError {
        GetQualificationTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetQualificationTypeError {
    fn from(err: CredentialsError) -> GetQualificationTypeError {
        GetQualificationTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQualificationTypeError {
    fn from(err: HttpDispatchError) -> GetQualificationTypeError {
        GetQualificationTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQualificationTypeError {
    fn from(err: io::Error) -> GetQualificationTypeError {
        GetQualificationTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetQualificationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQualificationTypeError {
    fn description(&self) -> &str {
        match *self {
            GetQualificationTypeError::RequestError(ref cause) => cause,
            GetQualificationTypeError::ServiceFault(ref cause) => cause,
            GetQualificationTypeError::Validation(ref cause) => cause,
            GetQualificationTypeError::Credentials(ref err) => err.description(),
            GetQualificationTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetQualificationTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssignmentsForHIT
#[derive(Debug, PartialEq)]
pub enum ListAssignmentsForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssignmentsForHITError {
    pub fn from_body(body: &str) -> ListAssignmentsForHITError {
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
                    "RequestError" => {
                        ListAssignmentsForHITError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ListAssignmentsForHITError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAssignmentsForHITError::Validation(error_message.to_string())
                    }
                    _ => ListAssignmentsForHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssignmentsForHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssignmentsForHITError {
    fn from(err: serde_json::error::Error) -> ListAssignmentsForHITError {
        ListAssignmentsForHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssignmentsForHITError {
    fn from(err: CredentialsError) -> ListAssignmentsForHITError {
        ListAssignmentsForHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssignmentsForHITError {
    fn from(err: HttpDispatchError) -> ListAssignmentsForHITError {
        ListAssignmentsForHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssignmentsForHITError {
    fn from(err: io::Error) -> ListAssignmentsForHITError {
        ListAssignmentsForHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssignmentsForHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssignmentsForHITError {
    fn description(&self) -> &str {
        match *self {
            ListAssignmentsForHITError::RequestError(ref cause) => cause,
            ListAssignmentsForHITError::ServiceFault(ref cause) => cause,
            ListAssignmentsForHITError::Validation(ref cause) => cause,
            ListAssignmentsForHITError::Credentials(ref err) => err.description(),
            ListAssignmentsForHITError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssignmentsForHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBonusPayments
#[derive(Debug, PartialEq)]
pub enum ListBonusPaymentsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBonusPaymentsError {
    pub fn from_body(body: &str) -> ListBonusPaymentsError {
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
                    "RequestError" => {
                        ListBonusPaymentsError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ListBonusPaymentsError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListBonusPaymentsError::Validation(error_message.to_string())
                    }
                    _ => ListBonusPaymentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBonusPaymentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBonusPaymentsError {
    fn from(err: serde_json::error::Error) -> ListBonusPaymentsError {
        ListBonusPaymentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBonusPaymentsError {
    fn from(err: CredentialsError) -> ListBonusPaymentsError {
        ListBonusPaymentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBonusPaymentsError {
    fn from(err: HttpDispatchError) -> ListBonusPaymentsError {
        ListBonusPaymentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBonusPaymentsError {
    fn from(err: io::Error) -> ListBonusPaymentsError {
        ListBonusPaymentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBonusPaymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBonusPaymentsError {
    fn description(&self) -> &str {
        match *self {
            ListBonusPaymentsError::RequestError(ref cause) => cause,
            ListBonusPaymentsError::ServiceFault(ref cause) => cause,
            ListBonusPaymentsError::Validation(ref cause) => cause,
            ListBonusPaymentsError::Credentials(ref err) => err.description(),
            ListBonusPaymentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBonusPaymentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHITs
#[derive(Debug, PartialEq)]
pub enum ListHITsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListHITsError {
    pub fn from_body(body: &str) -> ListHITsError {
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
                    "RequestError" => ListHITsError::RequestError(String::from(error_message)),
                    "ServiceFault" => ListHITsError::ServiceFault(String::from(error_message)),
                    "ValidationException" => ListHITsError::Validation(error_message.to_string()),
                    _ => ListHITsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListHITsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListHITsError {
    fn from(err: serde_json::error::Error) -> ListHITsError {
        ListHITsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListHITsError {
    fn from(err: CredentialsError) -> ListHITsError {
        ListHITsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHITsError {
    fn from(err: HttpDispatchError) -> ListHITsError {
        ListHITsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHITsError {
    fn from(err: io::Error) -> ListHITsError {
        ListHITsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHITsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHITsError {
    fn description(&self) -> &str {
        match *self {
            ListHITsError::RequestError(ref cause) => cause,
            ListHITsError::ServiceFault(ref cause) => cause,
            ListHITsError::Validation(ref cause) => cause,
            ListHITsError::Credentials(ref err) => err.description(),
            ListHITsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListHITsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHITsForQualificationType
#[derive(Debug, PartialEq)]
pub enum ListHITsForQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListHITsForQualificationTypeError {
    pub fn from_body(body: &str) -> ListHITsForQualificationTypeError {
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
                    "RequestError" => {
                        ListHITsForQualificationTypeError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ListHITsForQualificationTypeError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListHITsForQualificationTypeError::Validation(error_message.to_string())
                    }
                    _ => ListHITsForQualificationTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListHITsForQualificationTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListHITsForQualificationTypeError {
    fn from(err: serde_json::error::Error) -> ListHITsForQualificationTypeError {
        ListHITsForQualificationTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListHITsForQualificationTypeError {
    fn from(err: CredentialsError) -> ListHITsForQualificationTypeError {
        ListHITsForQualificationTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHITsForQualificationTypeError {
    fn from(err: HttpDispatchError) -> ListHITsForQualificationTypeError {
        ListHITsForQualificationTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHITsForQualificationTypeError {
    fn from(err: io::Error) -> ListHITsForQualificationTypeError {
        ListHITsForQualificationTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHITsForQualificationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHITsForQualificationTypeError {
    fn description(&self) -> &str {
        match *self {
            ListHITsForQualificationTypeError::RequestError(ref cause) => cause,
            ListHITsForQualificationTypeError::ServiceFault(ref cause) => cause,
            ListHITsForQualificationTypeError::Validation(ref cause) => cause,
            ListHITsForQualificationTypeError::Credentials(ref err) => err.description(),
            ListHITsForQualificationTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListHITsForQualificationTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListQualificationRequests
#[derive(Debug, PartialEq)]
pub enum ListQualificationRequestsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListQualificationRequestsError {
    pub fn from_body(body: &str) -> ListQualificationRequestsError {
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
                    "RequestError" => {
                        ListQualificationRequestsError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ListQualificationRequestsError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListQualificationRequestsError::Validation(error_message.to_string())
                    }
                    _ => ListQualificationRequestsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListQualificationRequestsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListQualificationRequestsError {
    fn from(err: serde_json::error::Error) -> ListQualificationRequestsError {
        ListQualificationRequestsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListQualificationRequestsError {
    fn from(err: CredentialsError) -> ListQualificationRequestsError {
        ListQualificationRequestsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListQualificationRequestsError {
    fn from(err: HttpDispatchError) -> ListQualificationRequestsError {
        ListQualificationRequestsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListQualificationRequestsError {
    fn from(err: io::Error) -> ListQualificationRequestsError {
        ListQualificationRequestsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListQualificationRequestsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListQualificationRequestsError {
    fn description(&self) -> &str {
        match *self {
            ListQualificationRequestsError::RequestError(ref cause) => cause,
            ListQualificationRequestsError::ServiceFault(ref cause) => cause,
            ListQualificationRequestsError::Validation(ref cause) => cause,
            ListQualificationRequestsError::Credentials(ref err) => err.description(),
            ListQualificationRequestsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListQualificationRequestsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListQualificationTypes
#[derive(Debug, PartialEq)]
pub enum ListQualificationTypesError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListQualificationTypesError {
    pub fn from_body(body: &str) -> ListQualificationTypesError {
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
                    "RequestError" => {
                        ListQualificationTypesError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ListQualificationTypesError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListQualificationTypesError::Validation(error_message.to_string())
                    }
                    _ => ListQualificationTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListQualificationTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListQualificationTypesError {
    fn from(err: serde_json::error::Error) -> ListQualificationTypesError {
        ListQualificationTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListQualificationTypesError {
    fn from(err: CredentialsError) -> ListQualificationTypesError {
        ListQualificationTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListQualificationTypesError {
    fn from(err: HttpDispatchError) -> ListQualificationTypesError {
        ListQualificationTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListQualificationTypesError {
    fn from(err: io::Error) -> ListQualificationTypesError {
        ListQualificationTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListQualificationTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListQualificationTypesError {
    fn description(&self) -> &str {
        match *self {
            ListQualificationTypesError::RequestError(ref cause) => cause,
            ListQualificationTypesError::ServiceFault(ref cause) => cause,
            ListQualificationTypesError::Validation(ref cause) => cause,
            ListQualificationTypesError::Credentials(ref err) => err.description(),
            ListQualificationTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListQualificationTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListReviewPolicyResultsForHIT
#[derive(Debug, PartialEq)]
pub enum ListReviewPolicyResultsForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListReviewPolicyResultsForHITError {
    pub fn from_body(body: &str) -> ListReviewPolicyResultsForHITError {
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
                    "RequestError" => ListReviewPolicyResultsForHITError::RequestError(
                        String::from(error_message),
                    ),
                    "ServiceFault" => ListReviewPolicyResultsForHITError::ServiceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        ListReviewPolicyResultsForHITError::Validation(error_message.to_string())
                    }
                    _ => ListReviewPolicyResultsForHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListReviewPolicyResultsForHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListReviewPolicyResultsForHITError {
    fn from(err: serde_json::error::Error) -> ListReviewPolicyResultsForHITError {
        ListReviewPolicyResultsForHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListReviewPolicyResultsForHITError {
    fn from(err: CredentialsError) -> ListReviewPolicyResultsForHITError {
        ListReviewPolicyResultsForHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListReviewPolicyResultsForHITError {
    fn from(err: HttpDispatchError) -> ListReviewPolicyResultsForHITError {
        ListReviewPolicyResultsForHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListReviewPolicyResultsForHITError {
    fn from(err: io::Error) -> ListReviewPolicyResultsForHITError {
        ListReviewPolicyResultsForHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListReviewPolicyResultsForHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListReviewPolicyResultsForHITError {
    fn description(&self) -> &str {
        match *self {
            ListReviewPolicyResultsForHITError::RequestError(ref cause) => cause,
            ListReviewPolicyResultsForHITError::ServiceFault(ref cause) => cause,
            ListReviewPolicyResultsForHITError::Validation(ref cause) => cause,
            ListReviewPolicyResultsForHITError::Credentials(ref err) => err.description(),
            ListReviewPolicyResultsForHITError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListReviewPolicyResultsForHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListReviewableHITs
#[derive(Debug, PartialEq)]
pub enum ListReviewableHITsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListReviewableHITsError {
    pub fn from_body(body: &str) -> ListReviewableHITsError {
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
                    "RequestError" => {
                        ListReviewableHITsError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ListReviewableHITsError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListReviewableHITsError::Validation(error_message.to_string())
                    }
                    _ => ListReviewableHITsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListReviewableHITsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListReviewableHITsError {
    fn from(err: serde_json::error::Error) -> ListReviewableHITsError {
        ListReviewableHITsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListReviewableHITsError {
    fn from(err: CredentialsError) -> ListReviewableHITsError {
        ListReviewableHITsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListReviewableHITsError {
    fn from(err: HttpDispatchError) -> ListReviewableHITsError {
        ListReviewableHITsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListReviewableHITsError {
    fn from(err: io::Error) -> ListReviewableHITsError {
        ListReviewableHITsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListReviewableHITsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListReviewableHITsError {
    fn description(&self) -> &str {
        match *self {
            ListReviewableHITsError::RequestError(ref cause) => cause,
            ListReviewableHITsError::ServiceFault(ref cause) => cause,
            ListReviewableHITsError::Validation(ref cause) => cause,
            ListReviewableHITsError::Credentials(ref err) => err.description(),
            ListReviewableHITsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListReviewableHITsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWorkerBlocks
#[derive(Debug, PartialEq)]
pub enum ListWorkerBlocksError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListWorkerBlocksError {
    pub fn from_body(body: &str) -> ListWorkerBlocksError {
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
                    "RequestError" => {
                        ListWorkerBlocksError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        ListWorkerBlocksError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListWorkerBlocksError::Validation(error_message.to_string())
                    }
                    _ => ListWorkerBlocksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListWorkerBlocksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListWorkerBlocksError {
    fn from(err: serde_json::error::Error) -> ListWorkerBlocksError {
        ListWorkerBlocksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListWorkerBlocksError {
    fn from(err: CredentialsError) -> ListWorkerBlocksError {
        ListWorkerBlocksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListWorkerBlocksError {
    fn from(err: HttpDispatchError) -> ListWorkerBlocksError {
        ListWorkerBlocksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListWorkerBlocksError {
    fn from(err: io::Error) -> ListWorkerBlocksError {
        ListWorkerBlocksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListWorkerBlocksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWorkerBlocksError {
    fn description(&self) -> &str {
        match *self {
            ListWorkerBlocksError::RequestError(ref cause) => cause,
            ListWorkerBlocksError::ServiceFault(ref cause) => cause,
            ListWorkerBlocksError::Validation(ref cause) => cause,
            ListWorkerBlocksError::Credentials(ref err) => err.description(),
            ListWorkerBlocksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListWorkerBlocksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWorkersWithQualificationType
#[derive(Debug, PartialEq)]
pub enum ListWorkersWithQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListWorkersWithQualificationTypeError {
    pub fn from_body(body: &str) -> ListWorkersWithQualificationTypeError {
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
                    "RequestError" => ListWorkersWithQualificationTypeError::RequestError(
                        String::from(error_message),
                    ),
                    "ServiceFault" => ListWorkersWithQualificationTypeError::ServiceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        ListWorkersWithQualificationTypeError::Validation(error_message.to_string())
                    }
                    _ => ListWorkersWithQualificationTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListWorkersWithQualificationTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListWorkersWithQualificationTypeError {
    fn from(err: serde_json::error::Error) -> ListWorkersWithQualificationTypeError {
        ListWorkersWithQualificationTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListWorkersWithQualificationTypeError {
    fn from(err: CredentialsError) -> ListWorkersWithQualificationTypeError {
        ListWorkersWithQualificationTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListWorkersWithQualificationTypeError {
    fn from(err: HttpDispatchError) -> ListWorkersWithQualificationTypeError {
        ListWorkersWithQualificationTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListWorkersWithQualificationTypeError {
    fn from(err: io::Error) -> ListWorkersWithQualificationTypeError {
        ListWorkersWithQualificationTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListWorkersWithQualificationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWorkersWithQualificationTypeError {
    fn description(&self) -> &str {
        match *self {
            ListWorkersWithQualificationTypeError::RequestError(ref cause) => cause,
            ListWorkersWithQualificationTypeError::ServiceFault(ref cause) => cause,
            ListWorkersWithQualificationTypeError::Validation(ref cause) => cause,
            ListWorkersWithQualificationTypeError::Credentials(ref err) => err.description(),
            ListWorkersWithQualificationTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListWorkersWithQualificationTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by NotifyWorkers
#[derive(Debug, PartialEq)]
pub enum NotifyWorkersError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl NotifyWorkersError {
    pub fn from_body(body: &str) -> NotifyWorkersError {
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
                    "RequestError" => NotifyWorkersError::RequestError(String::from(error_message)),
                    "ServiceFault" => NotifyWorkersError::ServiceFault(String::from(error_message)),
                    "ValidationException" => {
                        NotifyWorkersError::Validation(error_message.to_string())
                    }
                    _ => NotifyWorkersError::Unknown(String::from(body)),
                }
            }
            Err(_) => NotifyWorkersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for NotifyWorkersError {
    fn from(err: serde_json::error::Error) -> NotifyWorkersError {
        NotifyWorkersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for NotifyWorkersError {
    fn from(err: CredentialsError) -> NotifyWorkersError {
        NotifyWorkersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for NotifyWorkersError {
    fn from(err: HttpDispatchError) -> NotifyWorkersError {
        NotifyWorkersError::HttpDispatch(err)
    }
}
impl From<io::Error> for NotifyWorkersError {
    fn from(err: io::Error) -> NotifyWorkersError {
        NotifyWorkersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for NotifyWorkersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for NotifyWorkersError {
    fn description(&self) -> &str {
        match *self {
            NotifyWorkersError::RequestError(ref cause) => cause,
            NotifyWorkersError::ServiceFault(ref cause) => cause,
            NotifyWorkersError::Validation(ref cause) => cause,
            NotifyWorkersError::Credentials(ref err) => err.description(),
            NotifyWorkersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            NotifyWorkersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RejectAssignment
#[derive(Debug, PartialEq)]
pub enum RejectAssignmentError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RejectAssignmentError {
    pub fn from_body(body: &str) -> RejectAssignmentError {
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
                    "RequestError" => {
                        RejectAssignmentError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        RejectAssignmentError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        RejectAssignmentError::Validation(error_message.to_string())
                    }
                    _ => RejectAssignmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => RejectAssignmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RejectAssignmentError {
    fn from(err: serde_json::error::Error) -> RejectAssignmentError {
        RejectAssignmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RejectAssignmentError {
    fn from(err: CredentialsError) -> RejectAssignmentError {
        RejectAssignmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RejectAssignmentError {
    fn from(err: HttpDispatchError) -> RejectAssignmentError {
        RejectAssignmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for RejectAssignmentError {
    fn from(err: io::Error) -> RejectAssignmentError {
        RejectAssignmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RejectAssignmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectAssignmentError {
    fn description(&self) -> &str {
        match *self {
            RejectAssignmentError::RequestError(ref cause) => cause,
            RejectAssignmentError::ServiceFault(ref cause) => cause,
            RejectAssignmentError::Validation(ref cause) => cause,
            RejectAssignmentError::Credentials(ref err) => err.description(),
            RejectAssignmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RejectAssignmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RejectQualificationRequest
#[derive(Debug, PartialEq)]
pub enum RejectQualificationRequestError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RejectQualificationRequestError {
    pub fn from_body(body: &str) -> RejectQualificationRequestError {
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
                    "RequestError" => {
                        RejectQualificationRequestError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        RejectQualificationRequestError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        RejectQualificationRequestError::Validation(error_message.to_string())
                    }
                    _ => RejectQualificationRequestError::Unknown(String::from(body)),
                }
            }
            Err(_) => RejectQualificationRequestError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RejectQualificationRequestError {
    fn from(err: serde_json::error::Error) -> RejectQualificationRequestError {
        RejectQualificationRequestError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RejectQualificationRequestError {
    fn from(err: CredentialsError) -> RejectQualificationRequestError {
        RejectQualificationRequestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RejectQualificationRequestError {
    fn from(err: HttpDispatchError) -> RejectQualificationRequestError {
        RejectQualificationRequestError::HttpDispatch(err)
    }
}
impl From<io::Error> for RejectQualificationRequestError {
    fn from(err: io::Error) -> RejectQualificationRequestError {
        RejectQualificationRequestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RejectQualificationRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectQualificationRequestError {
    fn description(&self) -> &str {
        match *self {
            RejectQualificationRequestError::RequestError(ref cause) => cause,
            RejectQualificationRequestError::ServiceFault(ref cause) => cause,
            RejectQualificationRequestError::Validation(ref cause) => cause,
            RejectQualificationRequestError::Credentials(ref err) => err.description(),
            RejectQualificationRequestError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RejectQualificationRequestError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendBonus
#[derive(Debug, PartialEq)]
pub enum SendBonusError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendBonusError {
    pub fn from_body(body: &str) -> SendBonusError {
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
                    "RequestError" => SendBonusError::RequestError(String::from(error_message)),
                    "ServiceFault" => SendBonusError::ServiceFault(String::from(error_message)),
                    "ValidationException" => SendBonusError::Validation(error_message.to_string()),
                    _ => SendBonusError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendBonusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendBonusError {
    fn from(err: serde_json::error::Error) -> SendBonusError {
        SendBonusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendBonusError {
    fn from(err: CredentialsError) -> SendBonusError {
        SendBonusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendBonusError {
    fn from(err: HttpDispatchError) -> SendBonusError {
        SendBonusError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendBonusError {
    fn from(err: io::Error) -> SendBonusError {
        SendBonusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendBonusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendBonusError {
    fn description(&self) -> &str {
        match *self {
            SendBonusError::RequestError(ref cause) => cause,
            SendBonusError::ServiceFault(ref cause) => cause,
            SendBonusError::Validation(ref cause) => cause,
            SendBonusError::Credentials(ref err) => err.description(),
            SendBonusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendBonusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendTestEventNotification
#[derive(Debug, PartialEq)]
pub enum SendTestEventNotificationError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendTestEventNotificationError {
    pub fn from_body(body: &str) -> SendTestEventNotificationError {
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
                    "RequestError" => {
                        SendTestEventNotificationError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        SendTestEventNotificationError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        SendTestEventNotificationError::Validation(error_message.to_string())
                    }
                    _ => SendTestEventNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendTestEventNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendTestEventNotificationError {
    fn from(err: serde_json::error::Error) -> SendTestEventNotificationError {
        SendTestEventNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendTestEventNotificationError {
    fn from(err: CredentialsError) -> SendTestEventNotificationError {
        SendTestEventNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendTestEventNotificationError {
    fn from(err: HttpDispatchError) -> SendTestEventNotificationError {
        SendTestEventNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendTestEventNotificationError {
    fn from(err: io::Error) -> SendTestEventNotificationError {
        SendTestEventNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendTestEventNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendTestEventNotificationError {
    fn description(&self) -> &str {
        match *self {
            SendTestEventNotificationError::RequestError(ref cause) => cause,
            SendTestEventNotificationError::ServiceFault(ref cause) => cause,
            SendTestEventNotificationError::Validation(ref cause) => cause,
            SendTestEventNotificationError::Credentials(ref err) => err.description(),
            SendTestEventNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SendTestEventNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateExpirationForHIT
#[derive(Debug, PartialEq)]
pub enum UpdateExpirationForHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateExpirationForHITError {
    pub fn from_body(body: &str) -> UpdateExpirationForHITError {
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
                    "RequestError" => {
                        UpdateExpirationForHITError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        UpdateExpirationForHITError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateExpirationForHITError::Validation(error_message.to_string())
                    }
                    _ => UpdateExpirationForHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateExpirationForHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateExpirationForHITError {
    fn from(err: serde_json::error::Error) -> UpdateExpirationForHITError {
        UpdateExpirationForHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateExpirationForHITError {
    fn from(err: CredentialsError) -> UpdateExpirationForHITError {
        UpdateExpirationForHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateExpirationForHITError {
    fn from(err: HttpDispatchError) -> UpdateExpirationForHITError {
        UpdateExpirationForHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateExpirationForHITError {
    fn from(err: io::Error) -> UpdateExpirationForHITError {
        UpdateExpirationForHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateExpirationForHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateExpirationForHITError {
    fn description(&self) -> &str {
        match *self {
            UpdateExpirationForHITError::RequestError(ref cause) => cause,
            UpdateExpirationForHITError::ServiceFault(ref cause) => cause,
            UpdateExpirationForHITError::Validation(ref cause) => cause,
            UpdateExpirationForHITError::Credentials(ref err) => err.description(),
            UpdateExpirationForHITError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateExpirationForHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateHITReviewStatus
#[derive(Debug, PartialEq)]
pub enum UpdateHITReviewStatusError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateHITReviewStatusError {
    pub fn from_body(body: &str) -> UpdateHITReviewStatusError {
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
                    "RequestError" => {
                        UpdateHITReviewStatusError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        UpdateHITReviewStatusError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateHITReviewStatusError::Validation(error_message.to_string())
                    }
                    _ => UpdateHITReviewStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateHITReviewStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateHITReviewStatusError {
    fn from(err: serde_json::error::Error) -> UpdateHITReviewStatusError {
        UpdateHITReviewStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateHITReviewStatusError {
    fn from(err: CredentialsError) -> UpdateHITReviewStatusError {
        UpdateHITReviewStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateHITReviewStatusError {
    fn from(err: HttpDispatchError) -> UpdateHITReviewStatusError {
        UpdateHITReviewStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateHITReviewStatusError {
    fn from(err: io::Error) -> UpdateHITReviewStatusError {
        UpdateHITReviewStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateHITReviewStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateHITReviewStatusError {
    fn description(&self) -> &str {
        match *self {
            UpdateHITReviewStatusError::RequestError(ref cause) => cause,
            UpdateHITReviewStatusError::ServiceFault(ref cause) => cause,
            UpdateHITReviewStatusError::Validation(ref cause) => cause,
            UpdateHITReviewStatusError::Credentials(ref err) => err.description(),
            UpdateHITReviewStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateHITReviewStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateHITTypeOfHIT
#[derive(Debug, PartialEq)]
pub enum UpdateHITTypeOfHITError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateHITTypeOfHITError {
    pub fn from_body(body: &str) -> UpdateHITTypeOfHITError {
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
                    "RequestError" => {
                        UpdateHITTypeOfHITError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        UpdateHITTypeOfHITError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateHITTypeOfHITError::Validation(error_message.to_string())
                    }
                    _ => UpdateHITTypeOfHITError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateHITTypeOfHITError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateHITTypeOfHITError {
    fn from(err: serde_json::error::Error) -> UpdateHITTypeOfHITError {
        UpdateHITTypeOfHITError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateHITTypeOfHITError {
    fn from(err: CredentialsError) -> UpdateHITTypeOfHITError {
        UpdateHITTypeOfHITError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateHITTypeOfHITError {
    fn from(err: HttpDispatchError) -> UpdateHITTypeOfHITError {
        UpdateHITTypeOfHITError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateHITTypeOfHITError {
    fn from(err: io::Error) -> UpdateHITTypeOfHITError {
        UpdateHITTypeOfHITError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateHITTypeOfHITError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateHITTypeOfHITError {
    fn description(&self) -> &str {
        match *self {
            UpdateHITTypeOfHITError::RequestError(ref cause) => cause,
            UpdateHITTypeOfHITError::ServiceFault(ref cause) => cause,
            UpdateHITTypeOfHITError::Validation(ref cause) => cause,
            UpdateHITTypeOfHITError::Credentials(ref err) => err.description(),
            UpdateHITTypeOfHITError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateHITTypeOfHITError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateNotificationSettings
#[derive(Debug, PartialEq)]
pub enum UpdateNotificationSettingsError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateNotificationSettingsError {
    pub fn from_body(body: &str) -> UpdateNotificationSettingsError {
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
                    "RequestError" => {
                        UpdateNotificationSettingsError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        UpdateNotificationSettingsError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateNotificationSettingsError::Validation(error_message.to_string())
                    }
                    _ => UpdateNotificationSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateNotificationSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateNotificationSettingsError {
    fn from(err: serde_json::error::Error) -> UpdateNotificationSettingsError {
        UpdateNotificationSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateNotificationSettingsError {
    fn from(err: CredentialsError) -> UpdateNotificationSettingsError {
        UpdateNotificationSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateNotificationSettingsError {
    fn from(err: HttpDispatchError) -> UpdateNotificationSettingsError {
        UpdateNotificationSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateNotificationSettingsError {
    fn from(err: io::Error) -> UpdateNotificationSettingsError {
        UpdateNotificationSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateNotificationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNotificationSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateNotificationSettingsError::RequestError(ref cause) => cause,
            UpdateNotificationSettingsError::ServiceFault(ref cause) => cause,
            UpdateNotificationSettingsError::Validation(ref cause) => cause,
            UpdateNotificationSettingsError::Credentials(ref err) => err.description(),
            UpdateNotificationSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateNotificationSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateQualificationType
#[derive(Debug, PartialEq)]
pub enum UpdateQualificationTypeError {
    /// <p>Your request is invalid.</p>
    RequestError(String),
    /// <p>Amazon Mechanical Turk is temporarily unable to process your request. Try your call again.</p>
    ServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateQualificationTypeError {
    pub fn from_body(body: &str) -> UpdateQualificationTypeError {
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
                    "RequestError" => {
                        UpdateQualificationTypeError::RequestError(String::from(error_message))
                    }
                    "ServiceFault" => {
                        UpdateQualificationTypeError::ServiceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateQualificationTypeError::Validation(error_message.to_string())
                    }
                    _ => UpdateQualificationTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateQualificationTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateQualificationTypeError {
    fn from(err: serde_json::error::Error) -> UpdateQualificationTypeError {
        UpdateQualificationTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateQualificationTypeError {
    fn from(err: CredentialsError) -> UpdateQualificationTypeError {
        UpdateQualificationTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateQualificationTypeError {
    fn from(err: HttpDispatchError) -> UpdateQualificationTypeError {
        UpdateQualificationTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateQualificationTypeError {
    fn from(err: io::Error) -> UpdateQualificationTypeError {
        UpdateQualificationTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateQualificationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateQualificationTypeError {
    fn description(&self) -> &str {
        match *self {
            UpdateQualificationTypeError::RequestError(ref cause) => cause,
            UpdateQualificationTypeError::ServiceFault(ref cause) => cause,
            UpdateQualificationTypeError::Validation(ref cause) => cause,
            UpdateQualificationTypeError::Credentials(ref err) => err.description(),
            UpdateQualificationTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateQualificationTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon MTurk API. Amazon MTurk clients implement this trait.
pub trait MechanicalTurk {
    /// <p> The <code>AcceptQualificationRequest</code> operation approves a Worker's request for a Qualification. </p> <p> Only the owner of the Qualification type can grant a Qualification request for that type. </p> <p> A successful request for the <code>AcceptQualificationRequest</code> operation returns with no errors and an empty body. </p>
    fn accept_qualification_request(
        &self,
        input: AcceptQualificationRequestRequest,
    ) -> RusotoFuture<AcceptQualificationRequestResponse, AcceptQualificationRequestError>;

    /// <p> The <code>ApproveAssignment</code> operation approves the results of a completed assignment. </p> <p> Approving an assignment initiates two payments from the Requester's Amazon.com account </p> <ul> <li> <p> The Worker who submitted the results is paid the reward specified in the HIT. </p> </li> <li> <p> Amazon Mechanical Turk fees are debited. </p> </li> </ul> <p> If the Requester's account does not have adequate funds for these payments, the call to ApproveAssignment returns an exception, and the approval is not processed. You can include an optional feedback message with the approval, which the Worker can see in the Status section of the web site. </p> <p> You can also call this operation for assignments that were previous rejected and approve them by explicitly overriding the previous rejection. This only works on rejected assignments that were submitted within the previous 30 days and only if the assignment's related HIT has not been deleted. </p>
    fn approve_assignment(
        &self,
        input: ApproveAssignmentRequest,
    ) -> RusotoFuture<ApproveAssignmentResponse, ApproveAssignmentError>;

    /// <p><p> The <code>AssociateQualificationWithWorker</code> operation gives a Worker a Qualification. <code>AssociateQualificationWithWorker</code> does not require that the Worker submit a Qualification request. It gives the Qualification directly to the Worker. </p> <p> You can only assign a Qualification of a Qualification type that you created (using the <code>CreateQualificationType</code> operation). </p> <note> <p> Note: <code>AssociateQualificationWithWorker</code> does not affect any pending Qualification requests for the Qualification by the Worker. If you assign a Qualification to a Worker, then later grant a Qualification request made by the Worker, the granting of the request may modify the Qualification score. To resolve a pending Qualification request without affecting the Qualification the Worker already has, reject the request with the <code>RejectQualificationRequest</code> operation. </p> </note></p>
    fn associate_qualification_with_worker(
        &self,
        input: AssociateQualificationWithWorkerRequest,
    ) -> RusotoFuture<AssociateQualificationWithWorkerResponse, AssociateQualificationWithWorkerError>;

    /// <p><p> The <code>CreateAdditionalAssignmentsForHIT</code> operation increases the maximum number of assignments of an existing HIT. </p> <p> To extend the maximum number of assignments, specify the number of additional assignments.</p> <note> <ul> <li> <p>HITs created with fewer than 10 assignments cannot be extended to have 10 or more assignments. Attempting to add assignments in a way that brings the total number of assignments for a HIT from fewer than 10 assignments to 10 or more assignments will result in an <code>AWS.MechanicalTurk.InvalidMaximumAssignmentsIncrease</code> exception.</p> </li> <li> <p>HITs that were created before July 22, 2015 cannot be extended. Attempting to extend HITs that were created before July 22, 2015 will result in an <code>AWS.MechanicalTurk.HITTooOldForExtension</code> exception. </p> </li> </ul> </note></p>
    fn create_additional_assignments_for_hit(
        &self,
        input: CreateAdditionalAssignmentsForHITRequest,
    ) -> RusotoFuture<
        CreateAdditionalAssignmentsForHITResponse,
        CreateAdditionalAssignmentsForHITError,
    >;

    /// <p><p>The <code>CreateHIT</code> operation creates a new Human Intelligence Task (HIT). The new HIT is made available for Workers to find and accept on the Amazon Mechanical Turk website. </p> <p> This operation allows you to specify a new HIT by passing in values for the properties of the HIT, such as its title, reward amount and number of assignments. When you pass these values to <code>CreateHIT</code>, a new HIT is created for you, with a new <code>HITTypeID</code>. The HITTypeID can be used to create additional HITs in the future without needing to specify common parameters such as the title, description and reward amount each time.</p> <p> An alternative way to create HITs is to first generate a HITTypeID using the <code>CreateHITType</code> operation and then call the <code>CreateHITWithHITType</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHIT also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>.</p> </note></p>
    fn create_hit(
        &self,
        input: CreateHITRequest,
    ) -> RusotoFuture<CreateHITResponse, CreateHITError>;

    /// <p> The <code>CreateHITType</code> operation creates a new HIT type. This operation allows you to define a standard set of HIT properties to use when creating HITs. If you register a HIT type with values that match an existing HIT type, the HIT type ID of the existing type will be returned. </p>
    fn create_hit_type(
        &self,
        input: CreateHITTypeRequest,
    ) -> RusotoFuture<CreateHITTypeResponse, CreateHITTypeError>;

    /// <p><p> The <code>CreateHITWithHITType</code> operation creates a new Human Intelligence Task (HIT) using an existing HITTypeID generated by the <code>CreateHITType</code> operation. </p> <p> This is an alternative way to create HITs from the <code>CreateHIT</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHITWithHITType also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>. </p> </note></p>
    fn create_hit_with_hit_type(
        &self,
        input: CreateHITWithHITTypeRequest,
    ) -> RusotoFuture<CreateHITWithHITTypeResponse, CreateHITWithHITTypeError>;

    /// <p> The <code>CreateQualificationType</code> operation creates a new Qualification type, which is represented by a <code>QualificationType</code> data structure. </p>
    fn create_qualification_type(
        &self,
        input: CreateQualificationTypeRequest,
    ) -> RusotoFuture<CreateQualificationTypeResponse, CreateQualificationTypeError>;

    /// <p>The <code>CreateWorkerBlock</code> operation allows you to prevent a Worker from working on your HITs. For example, you can block a Worker who is producing poor quality work. You can block up to 100,000 Workers.</p>
    fn create_worker_block(
        &self,
        input: CreateWorkerBlockRequest,
    ) -> RusotoFuture<CreateWorkerBlockResponse, CreateWorkerBlockError>;

    /// <p><p> The <code>DeleteHIT</code> operation is used to delete HIT that is no longer needed. Only the Requester who created the HIT can delete it. </p> <p> You can only dispose of HITs that are in the <code>Reviewable</code> state, with all of their submitted assignments already either approved or rejected. If you call the DeleteHIT operation on a HIT that is not in the <code>Reviewable</code> state (for example, that has not expired, or still has active assignments), or on a HIT that is Reviewable but without all of its submitted assignments already approved or rejected, the service will return an error. </p> <note> <ul> <li> <p> HITs are automatically disposed of after 120 days. </p> </li> <li> <p> After you dispose of a HIT, you can no longer approve the HIT&#39;s rejected assignments. </p> </li> <li> <p> Disposed HITs are not returned in results for the ListHITs operation. </p> </li> <li> <p> Disposing HITs can improve the performance of operations such as ListReviewableHITs and ListHITs. </p> </li> </ul> </note></p>
    fn delete_hit(
        &self,
        input: DeleteHITRequest,
    ) -> RusotoFuture<DeleteHITResponse, DeleteHITError>;

    /// <p><p> The <code>DeleteQualificationType</code> deletes a Qualification type and deletes any HIT types that are associated with the Qualification type. </p> <p>This operation does not revoke Qualifications already assigned to Workers because the Qualifications might be needed for active HITs. If there are any pending requests for the Qualification type, Amazon Mechanical Turk rejects those requests. After you delete a Qualification type, you can no longer use it to create HITs or HIT types.</p> <note> <p>DeleteQualificationType must wait for all the HITs that use the deleted Qualification type to be deleted before completing. It may take up to 48 hours before DeleteQualificationType completes and the unique name of the Qualification type is available for reuse with CreateQualificationType.</p> </note></p>
    fn delete_qualification_type(
        &self,
        input: DeleteQualificationTypeRequest,
    ) -> RusotoFuture<DeleteQualificationTypeResponse, DeleteQualificationTypeError>;

    /// <p>The <code>DeleteWorkerBlock</code> operation allows you to reinstate a blocked Worker to work on your HITs. This operation reverses the effects of the CreateWorkerBlock operation. You need the Worker ID to use this operation. If the Worker ID is missing or invalid, this operation fails and returns the message “WorkerId is invalid.” If the specified Worker is not blocked, this operation returns successfully.</p>
    fn delete_worker_block(
        &self,
        input: DeleteWorkerBlockRequest,
    ) -> RusotoFuture<DeleteWorkerBlockResponse, DeleteWorkerBlockError>;

    /// <p> The <code>DisassociateQualificationFromWorker</code> revokes a previously granted Qualification from a user. </p> <p> You can provide a text message explaining why the Qualification was revoked. The user who had the Qualification can see this message. </p>
    fn disassociate_qualification_from_worker(
        &self,
        input: DisassociateQualificationFromWorkerRequest,
    ) -> RusotoFuture<
        DisassociateQualificationFromWorkerResponse,
        DisassociateQualificationFromWorkerError,
    >;

    /// <p>The <code>GetAccountBalance</code> operation retrieves the amount of money in your Amazon Mechanical Turk account.</p>
    fn get_account_balance(
        &self,
    ) -> RusotoFuture<GetAccountBalanceResponse, GetAccountBalanceError>;

    /// <p> The <code>GetAssignment</code> operation retrieves the details of the specified Assignment. </p>
    fn get_assignment(
        &self,
        input: GetAssignmentRequest,
    ) -> RusotoFuture<GetAssignmentResponse, GetAssignmentError>;

    /// <p> The <code>GetFileUploadURL</code> operation generates and returns a temporary URL. You use the temporary URL to retrieve a file uploaded by a Worker as an answer to a FileUploadAnswer question for a HIT. The temporary URL is generated the instant the GetFileUploadURL operation is called, and is valid for 60 seconds. You can get a temporary file upload URL any time until the HIT is disposed. After the HIT is disposed, any uploaded files are deleted, and cannot be retrieved. Pending Deprecation on December 12, 2017. The Answer Specification structure will no longer support the <code>FileUploadAnswer</code> element to be used for the QuestionForm data structure. Instead, we recommend that Requesters who want to create HITs asking Workers to upload files to use Amazon S3. </p>
    fn get_file_upload_url(
        &self,
        input: GetFileUploadURLRequest,
    ) -> RusotoFuture<GetFileUploadURLResponse, GetFileUploadURLError>;

    /// <p> The <code>GetHIT</code> operation retrieves the details of the specified HIT. </p>
    fn get_hit(&self, input: GetHITRequest) -> RusotoFuture<GetHITResponse, GetHITError>;

    /// <p> The <code>GetQualificationScore</code> operation returns the value of a Worker's Qualification for a given Qualification type. </p> <p> To get a Worker's Qualification, you must know the Worker's ID. The Worker's ID is included in the assignment data returned by the <code>ListAssignmentsForHIT</code> operation. </p> <p>Only the owner of a Qualification type can query the value of a Worker's Qualification of that type.</p>
    fn get_qualification_score(
        &self,
        input: GetQualificationScoreRequest,
    ) -> RusotoFuture<GetQualificationScoreResponse, GetQualificationScoreError>;

    /// <p> The <code>GetQualificationType</code>operation retrieves information about a Qualification type using its ID. </p>
    fn get_qualification_type(
        &self,
        input: GetQualificationTypeRequest,
    ) -> RusotoFuture<GetQualificationTypeResponse, GetQualificationTypeError>;

    /// <p> The <code>ListAssignmentsForHIT</code> operation retrieves completed assignments for a HIT. You can use this operation to retrieve the results for a HIT. </p> <p> You can get assignments for a HIT at any time, even if the HIT is not yet Reviewable. If a HIT requested multiple assignments, and has received some results but has not yet become Reviewable, you can still retrieve the partial results with this operation. </p> <p> Use the AssignmentStatus parameter to control which set of assignments for a HIT are returned. The ListAssignmentsForHIT operation can return submitted assignments awaiting approval, or it can return assignments that have already been approved or rejected. You can set AssignmentStatus=Approved,Rejected to get assignments that have already been approved and rejected together in one result set. </p> <p> Only the Requester who created the HIT can retrieve the assignments for that HIT. </p> <p> Results are sorted and divided into numbered pages and the operation returns a single page of results. You can use the parameters of the operation to control sorting and pagination. </p>
    fn list_assignments_for_hit(
        &self,
        input: ListAssignmentsForHITRequest,
    ) -> RusotoFuture<ListAssignmentsForHITResponse, ListAssignmentsForHITError>;

    /// <p> The <code>ListBonusPayments</code> operation retrieves the amounts of bonuses you have paid to Workers for a given HIT or assignment. </p>
    fn list_bonus_payments(
        &self,
        input: ListBonusPaymentsRequest,
    ) -> RusotoFuture<ListBonusPaymentsResponse, ListBonusPaymentsError>;

    /// <p> The <code>ListHITs</code> operation returns all of a Requester's HITs. The operation returns HITs of any status, except for HITs that have been deleted of with the DeleteHIT operation or that have been auto-deleted. </p>
    fn list_hi_ts(&self, input: ListHITsRequest) -> RusotoFuture<ListHITsResponse, ListHITsError>;

    /// <p> The <code>ListHITsForQualificationType</code> operation returns the HITs that use the given Qualification type for a Qualification requirement. The operation returns HITs of any status, except for HITs that have been deleted with the <code>DeleteHIT</code> operation or that have been auto-deleted. </p>
    fn list_hi_ts_for_qualification_type(
        &self,
        input: ListHITsForQualificationTypeRequest,
    ) -> RusotoFuture<ListHITsForQualificationTypeResponse, ListHITsForQualificationTypeError>;

    /// <p> The <code>ListQualificationRequests</code> operation retrieves requests for Qualifications of a particular Qualification type. The owner of the Qualification type calls this operation to poll for pending requests, and accepts them using the AcceptQualification operation. </p>
    fn list_qualification_requests(
        &self,
        input: ListQualificationRequestsRequest,
    ) -> RusotoFuture<ListQualificationRequestsResponse, ListQualificationRequestsError>;

    /// <p> The <code>ListQualificationTypes</code> operation returns a list of Qualification types, filtered by an optional search term. </p>
    fn list_qualification_types(
        &self,
        input: ListQualificationTypesRequest,
    ) -> RusotoFuture<ListQualificationTypesResponse, ListQualificationTypesError>;

    /// <p> The <code>ListReviewPolicyResultsForHIT</code> operation retrieves the computed results and the actions taken in the course of executing your Review Policies for a given HIT. For information about how to specify Review Policies when you call CreateHIT, see Review Policies. The ListReviewPolicyResultsForHIT operation can return results for both Assignment-level and HIT-level review results. </p>
    fn list_review_policy_results_for_hit(
        &self,
        input: ListReviewPolicyResultsForHITRequest,
    ) -> RusotoFuture<ListReviewPolicyResultsForHITResponse, ListReviewPolicyResultsForHITError>;

    /// <p> The <code>ListReviewableHITs</code> operation retrieves the HITs with Status equal to Reviewable or Status equal to Reviewing that belong to the Requester calling the operation. </p>
    fn list_reviewable_hi_ts(
        &self,
        input: ListReviewableHITsRequest,
    ) -> RusotoFuture<ListReviewableHITsResponse, ListReviewableHITsError>;

    /// <p>The <code>ListWorkersBlocks</code> operation retrieves a list of Workers who are blocked from working on your HITs.</p>
    fn list_worker_blocks(
        &self,
        input: ListWorkerBlocksRequest,
    ) -> RusotoFuture<ListWorkerBlocksResponse, ListWorkerBlocksError>;

    /// <p> The <code>ListWorkersWithQualificationType</code> operation returns all of the Workers that have been associated with a given Qualification type. </p>
    fn list_workers_with_qualification_type(
        &self,
        input: ListWorkersWithQualificationTypeRequest,
    ) -> RusotoFuture<ListWorkersWithQualificationTypeResponse, ListWorkersWithQualificationTypeError>;

    /// <p> The <code>NotifyWorkers</code> operation sends an email to one or more Workers that you specify with the Worker ID. You can specify up to 100 Worker IDs to send the same message with a single call to the NotifyWorkers operation. The NotifyWorkers operation will send a notification email to a Worker only if you have previously approved or rejected work from the Worker. </p>
    fn notify_workers(
        &self,
        input: NotifyWorkersRequest,
    ) -> RusotoFuture<NotifyWorkersResponse, NotifyWorkersError>;

    /// <p> The <code>RejectAssignment</code> operation rejects the results of a completed assignment. </p> <p> You can include an optional feedback message with the rejection, which the Worker can see in the Status section of the web site. When you include a feedback message with the rejection, it helps the Worker understand why the assignment was rejected, and can improve the quality of the results the Worker submits in the future. </p> <p> Only the Requester who created the HIT can reject an assignment for the HIT. </p>
    fn reject_assignment(
        &self,
        input: RejectAssignmentRequest,
    ) -> RusotoFuture<RejectAssignmentResponse, RejectAssignmentError>;

    /// <p> The <code>RejectQualificationRequest</code> operation rejects a user's request for a Qualification. </p> <p> You can provide a text message explaining why the request was rejected. The Worker who made the request can see this message.</p>
    fn reject_qualification_request(
        &self,
        input: RejectQualificationRequestRequest,
    ) -> RusotoFuture<RejectQualificationRequestResponse, RejectQualificationRequestError>;

    /// <p> The <code>SendBonus</code> operation issues a payment of money from your account to a Worker. This payment happens separately from the reward you pay to the Worker when you approve the Worker's assignment. The SendBonus operation requires the Worker's ID and the assignment ID as parameters to initiate payment of the bonus. You must include a message that explains the reason for the bonus payment, as the Worker may not be expecting the payment. Amazon Mechanical Turk collects a fee for bonus payments, similar to the HIT listing fee. This operation fails if your account does not have enough funds to pay for both the bonus and the fees. </p>
    fn send_bonus(
        &self,
        input: SendBonusRequest,
    ) -> RusotoFuture<SendBonusResponse, SendBonusError>;

    /// <p> The <code>SendTestEventNotification</code> operation causes Amazon Mechanical Turk to send a notification message as if a HIT event occurred, according to the provided notification specification. This allows you to test notifications without setting up notifications for a real HIT type and trying to trigger them using the website. When you call this operation, the service attempts to send the test notification immediately. </p>
    fn send_test_event_notification(
        &self,
        input: SendTestEventNotificationRequest,
    ) -> RusotoFuture<SendTestEventNotificationResponse, SendTestEventNotificationError>;

    /// <p> The <code>UpdateExpirationForHIT</code> operation allows you update the expiration time of a HIT. If you update it to a time in the past, the HIT will be immediately expired. </p>
    fn update_expiration_for_hit(
        &self,
        input: UpdateExpirationForHITRequest,
    ) -> RusotoFuture<UpdateExpirationForHITResponse, UpdateExpirationForHITError>;

    /// <p> The <code>UpdateHITReviewStatus</code> operation updates the status of a HIT. If the status is Reviewable, this operation can update the status to Reviewing, or it can revert a Reviewing HIT back to the Reviewable status. </p>
    fn update_hit_review_status(
        &self,
        input: UpdateHITReviewStatusRequest,
    ) -> RusotoFuture<UpdateHITReviewStatusResponse, UpdateHITReviewStatusError>;

    /// <p> The <code>UpdateHITTypeOfHIT</code> operation allows you to change the HITType properties of a HIT. This operation disassociates the HIT from its old HITType properties and associates it with the new HITType properties. The HIT takes on the properties of the new HITType in place of the old ones. </p>
    fn update_hit_type_of_hit(
        &self,
        input: UpdateHITTypeOfHITRequest,
    ) -> RusotoFuture<UpdateHITTypeOfHITResponse, UpdateHITTypeOfHITError>;

    /// <p> The <code>UpdateNotificationSettings</code> operation creates, updates, disables or re-enables notifications for a HIT type. If you call the UpdateNotificationSettings operation for a HIT type that already has a notification specification, the operation replaces the old specification with a new one. You can call the UpdateNotificationSettings operation to enable or disable notifications for the HIT type, without having to modify the notification specification itself by providing updates to the Active status without specifying a new notification specification. To change the Active status of a HIT type's notifications, the HIT type must already have a notification specification, or one must be provided in the same call to <code>UpdateNotificationSettings</code>. </p>
    fn update_notification_settings(
        &self,
        input: UpdateNotificationSettingsRequest,
    ) -> RusotoFuture<UpdateNotificationSettingsResponse, UpdateNotificationSettingsError>;

    /// <p> The <code>UpdateQualificationType</code> operation modifies the attributes of an existing Qualification type, which is represented by a QualificationType data structure. Only the owner of a Qualification type can modify its attributes. </p> <p> Most attributes of a Qualification type can be changed after the type has been created. However, the Name and Keywords fields cannot be modified. The RetryDelayInSeconds parameter can be modified or added to change the delay or to enable retries, but RetryDelayInSeconds cannot be used to disable retries. </p> <p> You can use this operation to update the test for a Qualification type. The test is updated based on the values specified for the Test, TestDurationInSeconds and AnswerKey parameters. All three parameters specify the updated test. If you are updating the test for a type, you must specify the Test and TestDurationInSeconds parameters. The AnswerKey parameter is optional; omitting it specifies that the updated test does not have an answer key. </p> <p> If you omit the Test parameter, the test for the Qualification type is unchanged. There is no way to remove a test from a Qualification type that has one. If the type already has a test, you cannot update it to be AutoGranted. If the Qualification type does not have a test and one is provided by an update, the type will henceforth have a test. </p> <p> If you want to update the test duration or answer key for an existing test without changing the questions, you must specify a Test parameter with the original questions, along with the updated values. </p> <p> If you provide an updated Test but no AnswerKey, the new test will not have an answer key. Requests for such Qualifications must be granted manually. </p> <p> You can also update the AutoGranted and AutoGrantedValue attributes of the Qualification type.</p>
    fn update_qualification_type(
        &self,
        input: UpdateQualificationTypeRequest,
    ) -> RusotoFuture<UpdateQualificationTypeResponse, UpdateQualificationTypeError>;
}
/// A client for the Amazon MTurk API.
pub struct MechanicalTurkClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl MechanicalTurkClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> MechanicalTurkClient {
        MechanicalTurkClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> MechanicalTurkClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        MechanicalTurkClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> MechanicalTurk for MechanicalTurkClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p> The <code>AcceptQualificationRequest</code> operation approves a Worker's request for a Qualification. </p> <p> Only the owner of the Qualification type can grant a Qualification request for that type. </p> <p> A successful request for the <code>AcceptQualificationRequest</code> operation returns with no errors and an empty body. </p>
    fn accept_qualification_request(
        &self,
        input: AcceptQualificationRequestRequest,
    ) -> RusotoFuture<AcceptQualificationRequestResponse, AcceptQualificationRequestError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.AcceptQualificationRequest",
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

                    serde_json::from_str::<AcceptQualificationRequestResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AcceptQualificationRequestError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ApproveAssignment</code> operation approves the results of a completed assignment. </p> <p> Approving an assignment initiates two payments from the Requester's Amazon.com account </p> <ul> <li> <p> The Worker who submitted the results is paid the reward specified in the HIT. </p> </li> <li> <p> Amazon Mechanical Turk fees are debited. </p> </li> </ul> <p> If the Requester's account does not have adequate funds for these payments, the call to ApproveAssignment returns an exception, and the approval is not processed. You can include an optional feedback message with the approval, which the Worker can see in the Status section of the web site. </p> <p> You can also call this operation for assignments that were previous rejected and approve them by explicitly overriding the previous rejection. This only works on rejected assignments that were submitted within the previous 30 days and only if the assignment's related HIT has not been deleted. </p>
    fn approve_assignment(
        &self,
        input: ApproveAssignmentRequest,
    ) -> RusotoFuture<ApproveAssignmentResponse, ApproveAssignmentError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ApproveAssignment",
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

                    serde_json::from_str::<ApproveAssignmentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ApproveAssignmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p> The <code>AssociateQualificationWithWorker</code> operation gives a Worker a Qualification. <code>AssociateQualificationWithWorker</code> does not require that the Worker submit a Qualification request. It gives the Qualification directly to the Worker. </p> <p> You can only assign a Qualification of a Qualification type that you created (using the <code>CreateQualificationType</code> operation). </p> <note> <p> Note: <code>AssociateQualificationWithWorker</code> does not affect any pending Qualification requests for the Qualification by the Worker. If you assign a Qualification to a Worker, then later grant a Qualification request made by the Worker, the granting of the request may modify the Qualification score. To resolve a pending Qualification request without affecting the Qualification the Worker already has, reject the request with the <code>RejectQualificationRequest</code> operation. </p> </note></p>
    fn associate_qualification_with_worker(
        &self,
        input: AssociateQualificationWithWorkerRequest,
    ) -> RusotoFuture<AssociateQualificationWithWorkerResponse, AssociateQualificationWithWorkerError>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.AssociateQualificationWithWorker",
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

                    serde_json::from_str::<AssociateQualificationWithWorkerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateQualificationWithWorkerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p> The <code>CreateAdditionalAssignmentsForHIT</code> operation increases the maximum number of assignments of an existing HIT. </p> <p> To extend the maximum number of assignments, specify the number of additional assignments.</p> <note> <ul> <li> <p>HITs created with fewer than 10 assignments cannot be extended to have 10 or more assignments. Attempting to add assignments in a way that brings the total number of assignments for a HIT from fewer than 10 assignments to 10 or more assignments will result in an <code>AWS.MechanicalTurk.InvalidMaximumAssignmentsIncrease</code> exception.</p> </li> <li> <p>HITs that were created before July 22, 2015 cannot be extended. Attempting to extend HITs that were created before July 22, 2015 will result in an <code>AWS.MechanicalTurk.HITTooOldForExtension</code> exception. </p> </li> </ul> </note></p>
    fn create_additional_assignments_for_hit(
        &self,
        input: CreateAdditionalAssignmentsForHITRequest,
    ) -> RusotoFuture<
        CreateAdditionalAssignmentsForHITResponse,
        CreateAdditionalAssignmentsForHITError,
    > {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateAdditionalAssignmentsForHIT",
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

                    serde_json::from_str::<CreateAdditionalAssignmentsForHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAdditionalAssignmentsForHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>The <code>CreateHIT</code> operation creates a new Human Intelligence Task (HIT). The new HIT is made available for Workers to find and accept on the Amazon Mechanical Turk website. </p> <p> This operation allows you to specify a new HIT by passing in values for the properties of the HIT, such as its title, reward amount and number of assignments. When you pass these values to <code>CreateHIT</code>, a new HIT is created for you, with a new <code>HITTypeID</code>. The HITTypeID can be used to create additional HITs in the future without needing to specify common parameters such as the title, description and reward amount each time.</p> <p> An alternative way to create HITs is to first generate a HITTypeID using the <code>CreateHITType</code> operation and then call the <code>CreateHITWithHITType</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHIT also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>.</p> </note></p>
    fn create_hit(
        &self,
        input: CreateHITRequest,
    ) -> RusotoFuture<CreateHITResponse, CreateHITError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.CreateHIT");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>CreateHITType</code> operation creates a new HIT type. This operation allows you to define a standard set of HIT properties to use when creating HITs. If you register a HIT type with values that match an existing HIT type, the HIT type ID of the existing type will be returned. </p>
    fn create_hit_type(
        &self,
        input: CreateHITTypeRequest,
    ) -> RusotoFuture<CreateHITTypeResponse, CreateHITTypeError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateHITType",
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

                    serde_json::from_str::<CreateHITTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHITTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p> The <code>CreateHITWithHITType</code> operation creates a new Human Intelligence Task (HIT) using an existing HITTypeID generated by the <code>CreateHITType</code> operation. </p> <p> This is an alternative way to create HITs from the <code>CreateHIT</code> operation. This is the recommended best practice for Requesters who are creating large numbers of HITs. </p> <p>CreateHITWithHITType also supports several ways to provide question data: by providing a value for the <code>Question</code> parameter that fully specifies the contents of the HIT, or by providing a <code>HitLayoutId</code> and associated <code>HitLayoutParameters</code>. </p> <note> <p> If a HIT is created with 10 or more maximum assignments, there is an additional fee. For more information, see <a href="https://requester.mturk.com/pricing">Amazon Mechanical Turk Pricing</a>. </p> </note></p>
    fn create_hit_with_hit_type(
        &self,
        input: CreateHITWithHITTypeRequest,
    ) -> RusotoFuture<CreateHITWithHITTypeResponse, CreateHITWithHITTypeError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateHITWithHITType",
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

                    serde_json::from_str::<CreateHITWithHITTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHITWithHITTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>CreateQualificationType</code> operation creates a new Qualification type, which is represented by a <code>QualificationType</code> data structure. </p>
    fn create_qualification_type(
        &self,
        input: CreateQualificationTypeRequest,
    ) -> RusotoFuture<CreateQualificationTypeResponse, CreateQualificationTypeError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateQualificationType",
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

                    serde_json::from_str::<CreateQualificationTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateQualificationTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>CreateWorkerBlock</code> operation allows you to prevent a Worker from working on your HITs. For example, you can block a Worker who is producing poor quality work. You can block up to 100,000 Workers.</p>
    fn create_worker_block(
        &self,
        input: CreateWorkerBlockRequest,
    ) -> RusotoFuture<CreateWorkerBlockResponse, CreateWorkerBlockError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.CreateWorkerBlock",
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

                    serde_json::from_str::<CreateWorkerBlockResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateWorkerBlockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p> The <code>DeleteHIT</code> operation is used to delete HIT that is no longer needed. Only the Requester who created the HIT can delete it. </p> <p> You can only dispose of HITs that are in the <code>Reviewable</code> state, with all of their submitted assignments already either approved or rejected. If you call the DeleteHIT operation on a HIT that is not in the <code>Reviewable</code> state (for example, that has not expired, or still has active assignments), or on a HIT that is Reviewable but without all of its submitted assignments already approved or rejected, the service will return an error. </p> <note> <ul> <li> <p> HITs are automatically disposed of after 120 days. </p> </li> <li> <p> After you dispose of a HIT, you can no longer approve the HIT&#39;s rejected assignments. </p> </li> <li> <p> Disposed HITs are not returned in results for the ListHITs operation. </p> </li> <li> <p> Disposing HITs can improve the performance of operations such as ListReviewableHITs and ListHITs. </p> </li> </ul> </note></p>
    fn delete_hit(
        &self,
        input: DeleteHITRequest,
    ) -> RusotoFuture<DeleteHITResponse, DeleteHITError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.DeleteHIT");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p> The <code>DeleteQualificationType</code> deletes a Qualification type and deletes any HIT types that are associated with the Qualification type. </p> <p>This operation does not revoke Qualifications already assigned to Workers because the Qualifications might be needed for active HITs. If there are any pending requests for the Qualification type, Amazon Mechanical Turk rejects those requests. After you delete a Qualification type, you can no longer use it to create HITs or HIT types.</p> <note> <p>DeleteQualificationType must wait for all the HITs that use the deleted Qualification type to be deleted before completing. It may take up to 48 hours before DeleteQualificationType completes and the unique name of the Qualification type is available for reuse with CreateQualificationType.</p> </note></p>
    fn delete_qualification_type(
        &self,
        input: DeleteQualificationTypeRequest,
    ) -> RusotoFuture<DeleteQualificationTypeResponse, DeleteQualificationTypeError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.DeleteQualificationType",
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

                    serde_json::from_str::<DeleteQualificationTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteQualificationTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>DeleteWorkerBlock</code> operation allows you to reinstate a blocked Worker to work on your HITs. This operation reverses the effects of the CreateWorkerBlock operation. You need the Worker ID to use this operation. If the Worker ID is missing or invalid, this operation fails and returns the message “WorkerId is invalid.” If the specified Worker is not blocked, this operation returns successfully.</p>
    fn delete_worker_block(
        &self,
        input: DeleteWorkerBlockRequest,
    ) -> RusotoFuture<DeleteWorkerBlockResponse, DeleteWorkerBlockError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.DeleteWorkerBlock",
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

                    serde_json::from_str::<DeleteWorkerBlockResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteWorkerBlockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>DisassociateQualificationFromWorker</code> revokes a previously granted Qualification from a user. </p> <p> You can provide a text message explaining why the Qualification was revoked. The user who had the Qualification can see this message. </p>
    fn disassociate_qualification_from_worker(
        &self,
        input: DisassociateQualificationFromWorkerRequest,
    ) -> RusotoFuture<
        DisassociateQualificationFromWorkerResponse,
        DisassociateQualificationFromWorkerError,
    > {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.DisassociateQualificationFromWorker",
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

                    serde_json::from_str::<DisassociateQualificationFromWorkerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateQualificationFromWorkerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>GetAccountBalance</code> operation retrieves the amount of money in your Amazon Mechanical Turk account.</p>
    fn get_account_balance(
        &self,
    ) -> RusotoFuture<GetAccountBalanceResponse, GetAccountBalanceError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetAccountBalance",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAccountBalanceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAccountBalanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>GetAssignment</code> operation retrieves the details of the specified Assignment. </p>
    fn get_assignment(
        &self,
        input: GetAssignmentRequest,
    ) -> RusotoFuture<GetAssignmentResponse, GetAssignmentError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetAssignment",
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

                    serde_json::from_str::<GetAssignmentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAssignmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>GetFileUploadURL</code> operation generates and returns a temporary URL. You use the temporary URL to retrieve a file uploaded by a Worker as an answer to a FileUploadAnswer question for a HIT. The temporary URL is generated the instant the GetFileUploadURL operation is called, and is valid for 60 seconds. You can get a temporary file upload URL any time until the HIT is disposed. After the HIT is disposed, any uploaded files are deleted, and cannot be retrieved. Pending Deprecation on December 12, 2017. The Answer Specification structure will no longer support the <code>FileUploadAnswer</code> element to be used for the QuestionForm data structure. Instead, we recommend that Requesters who want to create HITs asking Workers to upload files to use Amazon S3. </p>
    fn get_file_upload_url(
        &self,
        input: GetFileUploadURLRequest,
    ) -> RusotoFuture<GetFileUploadURLResponse, GetFileUploadURLError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetFileUploadURL",
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

                    serde_json::from_str::<GetFileUploadURLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetFileUploadURLError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>GetHIT</code> operation retrieves the details of the specified HIT. </p>
    fn get_hit(&self, input: GetHITRequest) -> RusotoFuture<GetHITResponse, GetHITError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.GetHIT");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>GetQualificationScore</code> operation returns the value of a Worker's Qualification for a given Qualification type. </p> <p> To get a Worker's Qualification, you must know the Worker's ID. The Worker's ID is included in the assignment data returned by the <code>ListAssignmentsForHIT</code> operation. </p> <p>Only the owner of a Qualification type can query the value of a Worker's Qualification of that type.</p>
    fn get_qualification_score(
        &self,
        input: GetQualificationScoreRequest,
    ) -> RusotoFuture<GetQualificationScoreResponse, GetQualificationScoreError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetQualificationScore",
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

                    serde_json::from_str::<GetQualificationScoreResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetQualificationScoreError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>GetQualificationType</code>operation retrieves information about a Qualification type using its ID. </p>
    fn get_qualification_type(
        &self,
        input: GetQualificationTypeRequest,
    ) -> RusotoFuture<GetQualificationTypeResponse, GetQualificationTypeError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.GetQualificationType",
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

                    serde_json::from_str::<GetQualificationTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetQualificationTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListAssignmentsForHIT</code> operation retrieves completed assignments for a HIT. You can use this operation to retrieve the results for a HIT. </p> <p> You can get assignments for a HIT at any time, even if the HIT is not yet Reviewable. If a HIT requested multiple assignments, and has received some results but has not yet become Reviewable, you can still retrieve the partial results with this operation. </p> <p> Use the AssignmentStatus parameter to control which set of assignments for a HIT are returned. The ListAssignmentsForHIT operation can return submitted assignments awaiting approval, or it can return assignments that have already been approved or rejected. You can set AssignmentStatus=Approved,Rejected to get assignments that have already been approved and rejected together in one result set. </p> <p> Only the Requester who created the HIT can retrieve the assignments for that HIT. </p> <p> Results are sorted and divided into numbered pages and the operation returns a single page of results. You can use the parameters of the operation to control sorting and pagination. </p>
    fn list_assignments_for_hit(
        &self,
        input: ListAssignmentsForHITRequest,
    ) -> RusotoFuture<ListAssignmentsForHITResponse, ListAssignmentsForHITError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListAssignmentsForHIT",
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

                    serde_json::from_str::<ListAssignmentsForHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssignmentsForHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListBonusPayments</code> operation retrieves the amounts of bonuses you have paid to Workers for a given HIT or assignment. </p>
    fn list_bonus_payments(
        &self,
        input: ListBonusPaymentsRequest,
    ) -> RusotoFuture<ListBonusPaymentsResponse, ListBonusPaymentsError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListBonusPayments",
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

                    serde_json::from_str::<ListBonusPaymentsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListBonusPaymentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListHITs</code> operation returns all of a Requester's HITs. The operation returns HITs of any status, except for HITs that have been deleted of with the DeleteHIT operation or that have been auto-deleted. </p>
    fn list_hi_ts(&self, input: ListHITsRequest) -> RusotoFuture<ListHITsResponse, ListHITsError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.ListHITs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListHITsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListHITsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListHITsForQualificationType</code> operation returns the HITs that use the given Qualification type for a Qualification requirement. The operation returns HITs of any status, except for HITs that have been deleted with the <code>DeleteHIT</code> operation or that have been auto-deleted. </p>
    fn list_hi_ts_for_qualification_type(
        &self,
        input: ListHITsForQualificationTypeRequest,
    ) -> RusotoFuture<ListHITsForQualificationTypeResponse, ListHITsForQualificationTypeError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListHITsForQualificationType",
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

                    serde_json::from_str::<ListHITsForQualificationTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListHITsForQualificationTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListQualificationRequests</code> operation retrieves requests for Qualifications of a particular Qualification type. The owner of the Qualification type calls this operation to poll for pending requests, and accepts them using the AcceptQualification operation. </p>
    fn list_qualification_requests(
        &self,
        input: ListQualificationRequestsRequest,
    ) -> RusotoFuture<ListQualificationRequestsResponse, ListQualificationRequestsError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListQualificationRequests",
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

                    serde_json::from_str::<ListQualificationRequestsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListQualificationRequestsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListQualificationTypes</code> operation returns a list of Qualification types, filtered by an optional search term. </p>
    fn list_qualification_types(
        &self,
        input: ListQualificationTypesRequest,
    ) -> RusotoFuture<ListQualificationTypesResponse, ListQualificationTypesError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListQualificationTypes",
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

                    serde_json::from_str::<ListQualificationTypesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListQualificationTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListReviewPolicyResultsForHIT</code> operation retrieves the computed results and the actions taken in the course of executing your Review Policies for a given HIT. For information about how to specify Review Policies when you call CreateHIT, see Review Policies. The ListReviewPolicyResultsForHIT operation can return results for both Assignment-level and HIT-level review results. </p>
    fn list_review_policy_results_for_hit(
        &self,
        input: ListReviewPolicyResultsForHITRequest,
    ) -> RusotoFuture<ListReviewPolicyResultsForHITResponse, ListReviewPolicyResultsForHITError>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListReviewPolicyResultsForHIT",
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

                    serde_json::from_str::<ListReviewPolicyResultsForHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListReviewPolicyResultsForHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListReviewableHITs</code> operation retrieves the HITs with Status equal to Reviewable or Status equal to Reviewing that belong to the Requester calling the operation. </p>
    fn list_reviewable_hi_ts(
        &self,
        input: ListReviewableHITsRequest,
    ) -> RusotoFuture<ListReviewableHITsResponse, ListReviewableHITsError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListReviewableHITs",
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

                    serde_json::from_str::<ListReviewableHITsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListReviewableHITsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>ListWorkersBlocks</code> operation retrieves a list of Workers who are blocked from working on your HITs.</p>
    fn list_worker_blocks(
        &self,
        input: ListWorkerBlocksRequest,
    ) -> RusotoFuture<ListWorkerBlocksResponse, ListWorkerBlocksError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListWorkerBlocks",
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

                    serde_json::from_str::<ListWorkerBlocksResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListWorkerBlocksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>ListWorkersWithQualificationType</code> operation returns all of the Workers that have been associated with a given Qualification type. </p>
    fn list_workers_with_qualification_type(
        &self,
        input: ListWorkersWithQualificationTypeRequest,
    ) -> RusotoFuture<ListWorkersWithQualificationTypeResponse, ListWorkersWithQualificationTypeError>
    {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.ListWorkersWithQualificationType",
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

                    serde_json::from_str::<ListWorkersWithQualificationTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListWorkersWithQualificationTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>NotifyWorkers</code> operation sends an email to one or more Workers that you specify with the Worker ID. You can specify up to 100 Worker IDs to send the same message with a single call to the NotifyWorkers operation. The NotifyWorkers operation will send a notification email to a Worker only if you have previously approved or rejected work from the Worker. </p>
    fn notify_workers(
        &self,
        input: NotifyWorkersRequest,
    ) -> RusotoFuture<NotifyWorkersResponse, NotifyWorkersError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.NotifyWorkers",
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

                    serde_json::from_str::<NotifyWorkersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(NotifyWorkersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>RejectAssignment</code> operation rejects the results of a completed assignment. </p> <p> You can include an optional feedback message with the rejection, which the Worker can see in the Status section of the web site. When you include a feedback message with the rejection, it helps the Worker understand why the assignment was rejected, and can improve the quality of the results the Worker submits in the future. </p> <p> Only the Requester who created the HIT can reject an assignment for the HIT. </p>
    fn reject_assignment(
        &self,
        input: RejectAssignmentRequest,
    ) -> RusotoFuture<RejectAssignmentResponse, RejectAssignmentError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.RejectAssignment",
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

                    serde_json::from_str::<RejectAssignmentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RejectAssignmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>RejectQualificationRequest</code> operation rejects a user's request for a Qualification. </p> <p> You can provide a text message explaining why the request was rejected. The Worker who made the request can see this message.</p>
    fn reject_qualification_request(
        &self,
        input: RejectQualificationRequestRequest,
    ) -> RusotoFuture<RejectQualificationRequestResponse, RejectQualificationRequestError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.RejectQualificationRequest",
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

                    serde_json::from_str::<RejectQualificationRequestResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RejectQualificationRequestError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>SendBonus</code> operation issues a payment of money from your account to a Worker. This payment happens separately from the reward you pay to the Worker when you approve the Worker's assignment. The SendBonus operation requires the Worker's ID and the assignment ID as parameters to initiate payment of the bonus. You must include a message that explains the reason for the bonus payment, as the Worker may not be expecting the payment. Amazon Mechanical Turk collects a fee for bonus payments, similar to the HIT listing fee. This operation fails if your account does not have enough funds to pay for both the bonus and the fees. </p>
    fn send_bonus(
        &self,
        input: SendBonusRequest,
    ) -> RusotoFuture<SendBonusResponse, SendBonusError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MTurkRequesterServiceV20170117.SendBonus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SendBonusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendBonusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>SendTestEventNotification</code> operation causes Amazon Mechanical Turk to send a notification message as if a HIT event occurred, according to the provided notification specification. This allows you to test notifications without setting up notifications for a real HIT type and trying to trigger them using the website. When you call this operation, the service attempts to send the test notification immediately. </p>
    fn send_test_event_notification(
        &self,
        input: SendTestEventNotificationRequest,
    ) -> RusotoFuture<SendTestEventNotificationResponse, SendTestEventNotificationError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.SendTestEventNotification",
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

                    serde_json::from_str::<SendTestEventNotificationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendTestEventNotificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>UpdateExpirationForHIT</code> operation allows you update the expiration time of a HIT. If you update it to a time in the past, the HIT will be immediately expired. </p>
    fn update_expiration_for_hit(
        &self,
        input: UpdateExpirationForHITRequest,
    ) -> RusotoFuture<UpdateExpirationForHITResponse, UpdateExpirationForHITError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateExpirationForHIT",
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

                    serde_json::from_str::<UpdateExpirationForHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateExpirationForHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>UpdateHITReviewStatus</code> operation updates the status of a HIT. If the status is Reviewable, this operation can update the status to Reviewing, or it can revert a Reviewing HIT back to the Reviewable status. </p>
    fn update_hit_review_status(
        &self,
        input: UpdateHITReviewStatusRequest,
    ) -> RusotoFuture<UpdateHITReviewStatusResponse, UpdateHITReviewStatusError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateHITReviewStatus",
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

                    serde_json::from_str::<UpdateHITReviewStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateHITReviewStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>UpdateHITTypeOfHIT</code> operation allows you to change the HITType properties of a HIT. This operation disassociates the HIT from its old HITType properties and associates it with the new HITType properties. The HIT takes on the properties of the new HITType in place of the old ones. </p>
    fn update_hit_type_of_hit(
        &self,
        input: UpdateHITTypeOfHITRequest,
    ) -> RusotoFuture<UpdateHITTypeOfHITResponse, UpdateHITTypeOfHITError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateHITTypeOfHIT",
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

                    serde_json::from_str::<UpdateHITTypeOfHITResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateHITTypeOfHITError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>UpdateNotificationSettings</code> operation creates, updates, disables or re-enables notifications for a HIT type. If you call the UpdateNotificationSettings operation for a HIT type that already has a notification specification, the operation replaces the old specification with a new one. You can call the UpdateNotificationSettings operation to enable or disable notifications for the HIT type, without having to modify the notification specification itself by providing updates to the Active status without specifying a new notification specification. To change the Active status of a HIT type's notifications, the HIT type must already have a notification specification, or one must be provided in the same call to <code>UpdateNotificationSettings</code>. </p>
    fn update_notification_settings(
        &self,
        input: UpdateNotificationSettingsRequest,
    ) -> RusotoFuture<UpdateNotificationSettingsResponse, UpdateNotificationSettingsError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateNotificationSettings",
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

                    serde_json::from_str::<UpdateNotificationSettingsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateNotificationSettingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> The <code>UpdateQualificationType</code> operation modifies the attributes of an existing Qualification type, which is represented by a QualificationType data structure. Only the owner of a Qualification type can modify its attributes. </p> <p> Most attributes of a Qualification type can be changed after the type has been created. However, the Name and Keywords fields cannot be modified. The RetryDelayInSeconds parameter can be modified or added to change the delay or to enable retries, but RetryDelayInSeconds cannot be used to disable retries. </p> <p> You can use this operation to update the test for a Qualification type. The test is updated based on the values specified for the Test, TestDurationInSeconds and AnswerKey parameters. All three parameters specify the updated test. If you are updating the test for a type, you must specify the Test and TestDurationInSeconds parameters. The AnswerKey parameter is optional; omitting it specifies that the updated test does not have an answer key. </p> <p> If you omit the Test parameter, the test for the Qualification type is unchanged. There is no way to remove a test from a Qualification type that has one. If the type already has a test, you cannot update it to be AutoGranted. If the Qualification type does not have a test and one is provided by an update, the type will henceforth have a test. </p> <p> If you want to update the test duration or answer key for an existing test without changing the questions, you must specify a Test parameter with the original questions, along with the updated values. </p> <p> If you provide an updated Test but no AnswerKey, the new test will not have an answer key. Requests for such Qualifications must be granted manually. </p> <p> You can also update the AutoGranted and AutoGrantedValue attributes of the Qualification type.</p>
    fn update_qualification_type(
        &self,
        input: UpdateQualificationTypeRequest,
    ) -> RusotoFuture<UpdateQualificationTypeResponse, UpdateQualificationTypeError> {
        let mut request = SignedRequest::new("POST", "mturk-requester", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MTurkRequesterServiceV20170117.UpdateQualificationType",
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

                    serde_json::from_str::<UpdateQualificationTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateQualificationTypeError::from_body(
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
