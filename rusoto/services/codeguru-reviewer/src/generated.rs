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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateRepositoryRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> <p>To add a new repository association, this parameter specifies a unique identifier for the new repository association that helps ensure idempotency.</p> <p>If you use the AWS CLI or one of the AWS SDKs to call this operation, you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes that in the request. If you don't use the SDK and instead generate a raw HTTP request to the Secrets Manager service endpoint, you must generate a ClientRequestToken yourself for new versions and include that value in the request.</p> <p>You typically interact with this value if you implement your own retry logic and want to ensure that a given repository association is not created twice. We recommend that you generate a UUID-type value to ensure uniqueness within the specified repository association.</p> <p>Amazon CodeGuru Reviewer uses this value to prevent the accidental creation of duplicate repository associations if there are failures and retries. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The repository to associate.</p>
    #[serde(rename = "Repository")]
    pub repository: Repository,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateRepositoryResponse {
    /// <p>Information about the repository association.</p>
    #[serde(rename = "RepositoryAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
}

/// <p>Information about an AWS CodeCommit repository. The CodeCommit repository must be in the same AWS Region and AWS account where its CodeGuru Reviewer code reviews are configured. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CodeCommitRepository {
    /// <p>The name of the AWS CodeCommit repository. For more information, see <a href="https://docs.aws.amazon.com/codecommit/latest/APIReference/API_GetRepository.html#CodeCommit-GetRepository-request-repositoryName">repositoryName</a> in the <i>AWS CodeCommit API Reference</i>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p> Information about a code review. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeReview {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_arn: Option<String>,
    /// <p> The time, in milliseconds since the epoch, when the code review was created. </p>
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p> The time, in milliseconds since the epoch, when the code review was last updated. </p>
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    /// <p> The statistics from the code review. </p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    /// <p> The name of the code review. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the repository. For an AWS CodeCommit repository, this is the AWS account ID of the account that owns the repository. For a GitHub or Bitbucket repository, this is the username for the account that owns the repository.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p> The type of repository that contains the reviewed code (for example, GitHub or Bitbucket). </p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p> The pull request ID for the code review. </p>
    #[serde(rename = "PullRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    /// <p> The name of the repository. </p>
    #[serde(rename = "RepositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p> The type of the source code for the code review. </p>
    #[serde(rename = "SourceCodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_type: Option<SourceCodeType>,
    /// <p><p>The valid code review states are:</p> <ul> <li> <p> <code>Completed</code>: The code review is complete. </p> </li> <li> <p> <code>Pending</code>: The code review started and has not completed or failed. </p> </li> <li> <p> <code>Failed</code>: The code review failed. </p> </li> <li> <p> <code>Deleting</code>: The code review is being deleted. </p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p> The reason for the state of the code review. </p>
    #[serde(rename = "StateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    /// <p> The type of code review. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> Information about the summary of the code review. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeReviewSummary {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_arn: Option<String>,
    /// <p> The time, in milliseconds since the epoch, when the code review was created. </p>
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p> The time, in milliseconds since the epoch, when the code review was last updated. </p>
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    /// <p> The statistics from the code review. </p>
    #[serde(rename = "MetricsSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_summary: Option<MetricsSummary>,
    /// <p> The name of the code review. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the repository. For an AWS CodeCommit repository, this is the AWS account ID of the account that owns the repository. For a GitHub or Bitbucket repository, this is the username for the account that owns the repository.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p> The provider type of the repository association. </p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p> The pull request ID for the code review. </p>
    #[serde(rename = "PullRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    /// <p> The name of the repository. </p>
    #[serde(rename = "RepositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p><p> The state of the code review. </p> <p>The valid code review states are:</p> <ul> <li> <p> <code>Completed</code>: The code review is complete. </p> </li> <li> <p> <code>Pending</code>: The code review started and has not completed or failed. </p> </li> <li> <p> <code>Failed</code>: The code review failed. </p> </li> <li> <p> <code>Deleting</code>: The code review is being deleted. </p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p> The type of the code review. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> The commit diff for the pull request. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommitDiffSourceCodeType {
    /// <p> The SHA of the destination commit. </p>
    #[serde(rename = "DestinationCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit: Option<String>,
    /// <p> The SHA of the source commit. </p>
    #[serde(rename = "SourceCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCodeReviewRequest {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    pub code_review_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCodeReviewResponse {
    /// <p> Information about the code review. </p>
    #[serde(rename = "CodeReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review: Option<CodeReview>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRecommendationFeedbackRequest {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    pub code_review_arn: String,
    /// <p> The recommendation ID that can be used to track the provided recommendations and then to collect the feedback. </p>
    #[serde(rename = "RecommendationId")]
    pub recommendation_id: String,
    /// <p> Optional parameter to describe the feedback for a given user. If this is not supplied, it defaults to the user making the request. </p> <p> The <code>UserId</code> is an IAM principal that can be specified as an AWS account ID or an Amazon Resource Name (ARN). For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#Principal_specifying"> Specifying a Principal</a> in the <i>AWS Identity and Access Management User Guide</i>. </p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRecommendationFeedbackResponse {
    /// <p> The recommendation feedback given by the user. </p>
    #[serde(rename = "RecommendationFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_feedback: Option<RecommendationFeedback>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRepositoryAssociationRequest {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociation.html"> <code>RepositoryAssociation</code> </a> object. You can retrieve this ARN by calling <code>ListRepositories</code>.</p>
    #[serde(rename = "AssociationArn")]
    pub association_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRepositoryAssociationResponse {
    /// <p>Information about the repository association.</p>
    #[serde(rename = "RepositoryAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateRepositoryRequest {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociation.html"> <code>RepositoryAssociation</code> </a> object. </p>
    #[serde(rename = "AssociationArn")]
    pub association_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateRepositoryResponse {
    /// <p>Information about the disassociated repository.</p>
    #[serde(rename = "RepositoryAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCodeReviewsRequest {
    /// <p> The maximum number of results that are returned per call. The default is 100. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> List of provider types for filtering that needs to be applied before displaying the result. For example, <code>providerTypes=[GitHub]</code> lists code reviews from GitHub. </p>
    #[serde(rename = "ProviderTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_types: Option<Vec<String>>,
    /// <p> List of repository names for filtering that needs to be applied before displaying the result. </p>
    #[serde(rename = "RepositoryNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_names: Option<Vec<String>>,
    /// <p><p> List of states for filtering that needs to be applied before displaying the result. For example, <code>states=[Pending]</code> lists code reviews in the Pending state. </p> <p>The valid code review states are:</p> <ul> <li> <p> <code>Completed</code>: The code review is complete. </p> </li> <li> <p> <code>Pending</code>: The code review started and has not completed or failed. </p> </li> <li> <p> <code>Failed</code>: The code review failed. </p> </li> <li> <p> <code>Deleting</code>: The code review is being deleted. </p> </li> </ul></p>
    #[serde(rename = "States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// <p> The type of code reviews to list in the response. </p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCodeReviewsResponse {
    /// <p> A list of code reviews that meet the criteria of the request. </p>
    #[serde(rename = "CodeReviewSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_summaries: Option<Vec<CodeReviewSummary>>,
    /// <p> Pagination token. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRecommendationFeedbackRequest {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    pub code_review_arn: String,
    /// <p> The maximum number of results that are returned per call. The default is 100. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> If <code>nextToken</code> is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> Used to query the recommendation feedback for a given recommendation. </p>
    #[serde(rename = "RecommendationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_ids: Option<Vec<String>>,
    /// <p> An AWS user's account ID or Amazon Resource Name (ARN). Use this ID to query the recommendation feedback for a code review from that user. </p> <p> The <code>UserId</code> is an IAM principal that can be specified as an AWS account ID or an Amazon Resource Name (ARN). For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#Principal_specifying"> Specifying a Principal</a> in the <i>AWS Identity and Access Management User Guide</i>. </p>
    #[serde(rename = "UserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRecommendationFeedbackResponse {
    /// <p> If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> Recommendation feedback summaries corresponding to the code review ARN. </p>
    #[serde(rename = "RecommendationFeedbackSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_feedback_summaries: Option<Vec<RecommendationFeedbackSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRecommendationsRequest {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    pub code_review_arn: String,
    /// <p> The maximum number of results that are returned per call. The default is 100. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRecommendationsResponse {
    /// <p> Pagination token. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> List of recommendations for the requested code review. </p>
    #[serde(rename = "RecommendationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summaries: Option<Vec<RecommendationSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRepositoryAssociationsRequest {
    /// <p>The maximum number of repository association results returned by <code>ListRepositoryAssociations</code> in paginated output. When this parameter is used, <code>ListRepositoryAssociations</code> only returns <code>maxResults</code> results in a single page with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRepositoryAssociations</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, <code>ListRepositoryAssociations</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>List of repository names to use as a filter.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListRepositoryAssociations</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>Treat this token as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of owners to use as a filter. For AWS CodeCommit, it is the name of the CodeCommit account that was used to associate the repository. For other repository source providers, such as Bitbucket, this is name of the account that was used to associate the repository. </p>
    #[serde(rename = "Owners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    /// <p>List of provider types to use as a filter.</p>
    #[serde(rename = "ProviderTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_types: Option<Vec<String>>,
    /// <p><p>List of repository association states to use as a filter.</p> <p>The valid repository association states are:</p> <ul> <li> <p> <b>Associated</b>: The repository association is complete. </p> </li> <li> <p> <b>Associating</b>: CodeGuru Reviewer is: </p> <ul> <li> <p> Setting up pull request notifications. This is required for pull requests to trigger a CodeGuru Reviewer review. </p> <note> <p> If your repository <code>ProviderType</code> is <code>GitHub</code> or <code>Bitbucket</code>, CodeGuru Reviewer creates webhooks in your repository to trigger CodeGuru Reviewer reviews. If you delete these webhooks, reviews of code in your repository cannot be triggered. </p> </note> </li> <li> <p> Setting up source code access. This is required for CodeGuru Reviewer to securely clone code in your repository. </p> </li> </ul> </li> <li> <p> <b>Failed</b>: The repository failed to associate or disassociate. </p> </li> <li> <p> <b>Disassociating</b>: CodeGuru Reviewer is removing the repository&#39;s pull request notifications and source code access. </p> </li> </ul></p>
    #[serde(rename = "States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRepositoryAssociationsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListRecommendations</code> request. When the results of a <code>ListRecommendations</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of repository associations that meet the criteria of the request.</p>
    #[serde(rename = "RepositoryAssociationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association_summaries: Option<Vec<RepositoryAssociationSummary>>,
}

/// <p> Information about the statistics from the code review. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Metrics {
    /// <p> Total number of recommendations found in the code review. </p>
    #[serde(rename = "FindingsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_count: Option<i64>,
    /// <p> Lines of code metered in the code review. For the initial code review pull request and all subsequent revisions, this includes all lines of code in the files added to the pull request. In subsequent revisions, for files that already existed in the pull request, this includes only the changed lines of code. In both cases, this does not include non-code lines such as comments and import statements. For example, if you submit a pull request containing 5 files, each with 500 lines of code, and in a subsequent revision you added a new file with 200 lines of code, and also modified a total of 25 lines across the initial 5 files, <code>MeteredLinesOfCodeCount</code> includes the first 5 files (5 * 500 = 2,500 lines), the new file (200 lines) and the 25 changed lines of code for a total of 2,725 lines of code. </p>
    #[serde(rename = "MeteredLinesOfCodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_lines_of_code_count: Option<i64>,
}

/// <p> Information about metrics summaries. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MetricsSummary {
    /// <p> Total number of recommendations found in the code review. </p>
    #[serde(rename = "FindingsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_count: Option<i64>,
    /// <p> Lines of code metered in the code review. For the initial code review pull request and all subsequent revisions, this includes all lines of code in the files added to the pull request. In subsequent revisions, for files that already existed in the pull request, this includes only the changed lines of code. In both cases, this does not include non-code lines such as comments and import statements. For example, if you submit a pull request containing 5 files, each with 500 lines of code, and in a subsequent revision you added a new file with 200 lines of code, and also modified a total of 25 lines across the initial 5 files, <code>MeteredLinesOfCodeCount</code> includes the first 5 files (5 * 500 = 2,500 lines), the new file (200 lines) and the 25 changed lines of code for a total of 2,725 lines of code. </p>
    #[serde(rename = "MeteredLinesOfCodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_lines_of_code_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRecommendationFeedbackRequest {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    pub code_review_arn: String,
    /// <p> List for storing reactions. Reactions are utf-8 text code for emojis. If you send an empty list it clears all your feedback. </p>
    #[serde(rename = "Reactions")]
    pub reactions: Vec<String>,
    /// <p> The recommendation ID that can be used to track the provided recommendations and then to collect the feedback. </p>
    #[serde(rename = "RecommendationId")]
    pub recommendation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRecommendationFeedbackResponse {}

/// <p> Information about the recommendation feedback. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationFeedback {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html"> <code>CodeReview</code> </a> object. </p>
    #[serde(rename = "CodeReviewArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_arn: Option<String>,
    /// <p> The time at which the feedback was created. </p>
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p> The time at which the feedback was last updated. </p>
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    /// <p> List for storing reactions. Reactions are utf-8 text code for emojis. You can send an empty list to clear off all your feedback. </p>
    #[serde(rename = "Reactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<String>>,
    /// <p> The recommendation ID that can be used to track the provided recommendations. Later on it can be used to collect the feedback. </p>
    #[serde(rename = "RecommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    /// <p> The ID of the user that made the API call. </p> <p> The <code>UserId</code> is an IAM principal that can be specified as an AWS account ID or an Amazon Resource Name (ARN). For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#Principal_specifying"> Specifying a Principal</a> in the <i>AWS Identity and Access Management User Guide</i>. </p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p> Information about recommendation feedback summaries. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationFeedbackSummary {
    /// <p> List for storing reactions. Reactions are utf-8 text code for emojis. </p>
    #[serde(rename = "Reactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<String>>,
    /// <p> The recommendation ID that can be used to track the provided recommendations. Later on it can be used to collect the feedback. </p>
    #[serde(rename = "RecommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    /// <p> The ID of the user that gave the feedback. </p> <p> The <code>UserId</code> is an IAM principal that can be specified as an AWS account ID or an Amazon Resource Name (ARN). For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#Principal_specifying"> Specifying a Principal</a> in the <i>AWS Identity and Access Management User Guide</i>. </p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p> Information about recommendations. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationSummary {
    /// <p> A description of the recommendation generated by CodeGuru Reviewer for the lines of code between the start line and the end line. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Last line where the recommendation is applicable in the source commit or source branch. For a single line comment the start line and end line values are the same. </p>
    #[serde(rename = "EndLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    /// <p>Name of the file on which a recommendation is provided.</p>
    #[serde(rename = "FilePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// <p> The recommendation ID that can be used to track the provided recommendations. Later on it can be used to collect the feedback. </p>
    #[serde(rename = "RecommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    /// <p> Start line from where the recommendation is applicable in the source commit or source branch. </p>
    #[serde(rename = "StartLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
}

/// <p> Information about an associated AWS CodeCommit repository or an associated repository that is managed by AWS CodeStar Connections (for example, Bitbucket). This <code>Repository</code> object is not used if your source code is in an associated GitHub repository. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Repository {
    /// <p> Information about a Bitbucket repository. </p>
    #[serde(rename = "Bitbucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitbucket: Option<ThirdPartySourceRepository>,
    /// <p>Information about an AWS CodeCommit repository.</p>
    #[serde(rename = "CodeCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_commit: Option<CodeCommitRepository>,
    /// <p> Information about a GitHub Enterprise Server repository. </p>
    #[serde(rename = "GitHubEnterpriseServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub_enterprise_server: Option<ThirdPartySourceRepository>,
}

/// <p>Information about a repository association. The <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_DescribeRepositoryAssociation.html"> <code>DescribeRepositoryAssociation</code> </a> operation returns a <code>RepositoryAssociation</code> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RepositoryAssociation {
    /// <p>The Amazon Resource Name (ARN) identifying the repository association.</p>
    #[serde(rename = "AssociationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    /// <p>The ID of the repository association.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection. Its format is <code>arn:aws:codestar-connections:region-id:aws-account_id:connection/connection-id</code>. For more information, see <a href="https://docs.aws.amazon.com/codestar-connections/latest/APIReference/API_Connection.html">Connection</a> in the <i>AWS CodeStar Connections API Reference</i>. </p>
    #[serde(rename = "ConnectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the repository association was created.</p>
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the repository association was last updated.</p>
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    /// <p>The name of the repository.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the repository. For an AWS CodeCommit repository, this is the AWS account ID of the account that owns the repository. For a GitHub or Bitbucket repository, this is the username for the account that owns the repository.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The provider type of the repository association.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p><p>The state of the repository association.</p> <p>The valid repository association states are:</p> <ul> <li> <p> <b>Associated</b>: The repository association is complete. </p> </li> <li> <p> <b>Associating</b>: CodeGuru Reviewer is: </p> <ul> <li> <p> Setting up pull request notifications. This is required for pull requests to trigger a CodeGuru Reviewer review. </p> <note> <p> If your repository <code>ProviderType</code> is <code>GitHub</code> or <code>Bitbucket</code>, CodeGuru Reviewer creates webhooks in your repository to trigger CodeGuru Reviewer reviews. If you delete these webhooks, reviews of code in your repository cannot be triggered. </p> </note> </li> <li> <p> Setting up source code access. This is required for CodeGuru Reviewer to securely clone code in your repository. </p> </li> </ul> </li> <li> <p> <b>Failed</b>: The repository failed to associate or disassociate. </p> </li> <li> <p> <b>Disassociating</b>: CodeGuru Reviewer is removing the repository&#39;s pull request notifications and source code access. </p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of why the repository association is in the current state.</p>
    #[serde(rename = "StateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>Summary information about a repository association. The <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_ListRepositoryAssociations.html"> <code>ListRepositoryAssociations</code> </a> operation returns a list of <code>RepositoryAssociationSummary</code> objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RepositoryAssociationSummary {
    /// <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociation.html"> <code>RepositoryAssociation</code> </a> object. </p>
    #[serde(rename = "AssociationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    /// <p> The repository association ID. </p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection. Its format is <code>arn:aws:codestar-connections:region-id:aws-account_id:connection/connection-id</code>. For more information, see <a href="https://docs.aws.amazon.com/codestar-connections/latest/APIReference/API_Connection.html">Connection</a> in the <i>AWS CodeStar Connections API Reference</i>. </p>
    #[serde(rename = "ConnectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, since the repository association was last updated. </p>
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    /// <p>The name of the repository association.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the repository. For an AWS CodeCommit repository, this is the AWS account ID of the account that owns the repository. For a GitHub or Bitbucket repository, this is the username for the account that owns the repository.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The provider type of the repository association.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p><p>The state of the repository association.</p> <p>The valid repository association states are:</p> <ul> <li> <p> <b>Associated</b>: The repository association is complete. </p> </li> <li> <p> <b>Associating</b>: CodeGuru Reviewer is: </p> <ul> <li> <p> Setting up pull request notifications. This is required for pull requests to trigger a CodeGuru Reviewer review. </p> <note> <p> If your repository <code>ProviderType</code> is <code>GitHub</code> or <code>Bitbucket</code>, CodeGuru Reviewer creates webhooks in your repository to trigger CodeGuru Reviewer reviews. If you delete these webhooks, reviews of code in your repository cannot be triggered. </p> </note> </li> <li> <p> Setting up source code access. This is required for CodeGuru Reviewer to securely clone code in your repository. </p> </li> </ul> </li> <li> <p> <b>Failed</b>: The repository failed to associate or disassociate. </p> </li> <li> <p> <b>Disassociating</b>: CodeGuru Reviewer is removing the repository&#39;s pull request notifications and source code access. </p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p> Information about the source code type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SourceCodeType {
    /// <p> The commit diff for the pull request. </p>
    #[serde(rename = "CommitDiff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_diff: Option<CommitDiffSourceCodeType>,
}

/// <p> Information about a third-party source repository connected to CodeGuru Reviewer. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ThirdPartySourceRepository {
    /// <p> The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection. Its format is <code>arn:aws:codestar-connections:region-id:aws-account_id:connection/connection-id</code>. For more information, see <a href="https://docs.aws.amazon.com/codestar-connections/latest/APIReference/API_Connection.html">Connection</a> in the <i>AWS CodeStar Connections API Reference</i>. </p>
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: String,
    /// <p> The name of the third party source repository. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p> The owner of the repository. For a GitHub, GitHub Enterprise, or Bitbucket repository, this is the username for the account that owns the repository. </p>
    #[serde(rename = "Owner")]
    pub owner: String,
}

/// Errors returned by AssociateRepository
#[derive(Debug, PartialEq)]
pub enum AssociateRepositoryError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl AssociateRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateRepositoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateRepositoryError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociateRepositoryError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(AssociateRepositoryError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateRepositoryError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateRepositoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateRepositoryError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociateRepositoryError::InternalServer(ref cause) => write!(f, "{}", cause),
            AssociateRepositoryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateRepositoryError {}
/// Errors returned by DescribeCodeReview
#[derive(Debug, PartialEq)]
pub enum DescribeCodeReviewError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p> The resource specified in the request was not found. </p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeCodeReviewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCodeReviewError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeCodeReviewError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeCodeReviewError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCodeReviewError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeCodeReviewError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCodeReviewError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCodeReviewError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeCodeReviewError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeCodeReviewError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeCodeReviewError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCodeReviewError {}
/// Errors returned by DescribeRecommendationFeedback
#[derive(Debug, PartialEq)]
pub enum DescribeRecommendationFeedbackError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p> The resource specified in the request was not found. </p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeRecommendationFeedbackError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRecommendationFeedbackError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeRecommendationFeedbackError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeRecommendationFeedbackError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeRecommendationFeedbackError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeRecommendationFeedbackError::Throttling(
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
impl fmt::Display for DescribeRecommendationFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRecommendationFeedbackError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeRecommendationFeedbackError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationFeedbackError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationFeedbackError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRecommendationFeedbackError {}
/// Errors returned by DescribeRepositoryAssociation
#[derive(Debug, PartialEq)]
pub enum DescribeRepositoryAssociationError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request was not found.</p>
    NotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeRepositoryAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRepositoryAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeRepositoryAssociationError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeRepositoryAssociationError::InternalServer(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeRepositoryAssociationError::NotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeRepositoryAssociationError::Throttling(
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
impl fmt::Display for DescribeRepositoryAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRepositoryAssociationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeRepositoryAssociationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeRepositoryAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeRepositoryAssociationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRepositoryAssociationError {}
/// Errors returned by DisassociateRepository
#[derive(Debug, PartialEq)]
pub enum DisassociateRepositoryError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request was not found.</p>
    NotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DisassociateRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateRepositoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateRepositoryError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DisassociateRepositoryError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DisassociateRepositoryError::InternalServer(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisassociateRepositoryError::NotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateRepositoryError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateRepositoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::Conflict(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::InternalServer(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::NotFound(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateRepositoryError {}
/// Errors returned by ListCodeReviews
#[derive(Debug, PartialEq)]
pub enum ListCodeReviewsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListCodeReviewsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCodeReviewsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListCodeReviewsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListCodeReviewsError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListCodeReviewsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCodeReviewsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCodeReviewsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListCodeReviewsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListCodeReviewsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCodeReviewsError {}
/// Errors returned by ListRecommendationFeedback
#[derive(Debug, PartialEq)]
pub enum ListRecommendationFeedbackError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p> The resource specified in the request was not found. </p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListRecommendationFeedbackError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRecommendationFeedbackError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListRecommendationFeedbackError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListRecommendationFeedbackError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRecommendationFeedbackError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRecommendationFeedbackError::Throttling(
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
impl fmt::Display for ListRecommendationFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRecommendationFeedbackError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListRecommendationFeedbackError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRecommendationFeedbackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRecommendationFeedbackError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRecommendationFeedbackError {}
/// Errors returned by ListRecommendations
#[derive(Debug, PartialEq)]
pub enum ListRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p> The resource specified in the request was not found. </p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListRecommendationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRecommendationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListRecommendationsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListRecommendationsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRecommendationsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRecommendationsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRecommendationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListRecommendationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRecommendationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRecommendationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRecommendationsError {}
/// Errors returned by ListRepositoryAssociations
#[derive(Debug, PartialEq)]
pub enum ListRepositoryAssociationsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListRepositoryAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRepositoryAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListRepositoryAssociationsError::InternalServer(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRepositoryAssociationsError::Throttling(
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
impl fmt::Display for ListRepositoryAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRepositoryAssociationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRepositoryAssociationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRepositoryAssociationsError {}
/// Errors returned by PutRecommendationFeedback
#[derive(Debug, PartialEq)]
pub enum PutRecommendationFeedbackError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p> The resource specified in the request was not found. </p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl PutRecommendationFeedbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRecommendationFeedbackError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutRecommendationFeedbackError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutRecommendationFeedbackError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRecommendationFeedbackError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutRecommendationFeedbackError::Throttling(
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
impl fmt::Display for PutRecommendationFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRecommendationFeedbackError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutRecommendationFeedbackError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutRecommendationFeedbackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutRecommendationFeedbackError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRecommendationFeedbackError {}
/// Trait representing the capabilities of the CodeGuruReviewer API. CodeGuruReviewer clients implement this trait.
#[async_trait]
pub trait CodeGuruReviewer {
    /// <p><p> Use to associate an AWS CodeCommit repository or a repostory managed by AWS CodeStar Connections with Amazon CodeGuru Reviewer. When you associate a repository, CodeGuru Reviewer reviews source code changes in the repository&#39;s pull requests and provides automatic recommendations. You can view recommendations using the CodeGuru Reviewer console. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/recommendations.html">Recommendations in Amazon CodeGuru Reviewer</a> in the <i>Amazon CodeGuru Reviewer User Guide.</i> </p> <p>If you associate a CodeCommit repository, it must be in the same AWS Region and AWS account where its CodeGuru Reviewer code reviews are configured.</p> <p> Bitbucket and GitHub Enterprise Server repositories are managed by AWS CodeStar Connections to connect to CodeGuru Reviewer. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/reviewer-ug/step-one.html#select-repository-source-provider">Connect to a repository source provider</a> in the <i>Amazon CodeGuru Reviewer User Guide.</i> </p> <note> <p> You cannot use the CodeGuru Reviewer SDK or the AWS CLI to associate a GitHub repository with Amazon CodeGuru Reviewer. To associate a GitHub repository, use the console. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/getting-started-with-guru.html">Getting started with CodeGuru Reviewer</a> in the <i>CodeGuru Reviewer User Guide.</i> </p> </note></p>
    async fn associate_repository(
        &self,
        input: AssociateRepositoryRequest,
    ) -> Result<AssociateRepositoryResponse, RusotoError<AssociateRepositoryError>>;

    /// <p> Returns the metadata associated with the code review along with its status.</p>
    async fn describe_code_review(
        &self,
        input: DescribeCodeReviewRequest,
    ) -> Result<DescribeCodeReviewResponse, RusotoError<DescribeCodeReviewError>>;

    /// <p> Describes the customer feedback for a CodeGuru Reviewer recommendation. </p>
    async fn describe_recommendation_feedback(
        &self,
        input: DescribeRecommendationFeedbackRequest,
    ) -> Result<
        DescribeRecommendationFeedbackResponse,
        RusotoError<DescribeRecommendationFeedbackError>,
    >;

    /// <p> Returns a <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociation.html"> <code>RepositoryAssociation</code> </a> object that contains information about the requested repository association. </p>
    async fn describe_repository_association(
        &self,
        input: DescribeRepositoryAssociationRequest,
    ) -> Result<
        DescribeRepositoryAssociationResponse,
        RusotoError<DescribeRepositoryAssociationError>,
    >;

    /// <p>Removes the association between Amazon CodeGuru Reviewer and a repository.</p>
    async fn disassociate_repository(
        &self,
        input: DisassociateRepositoryRequest,
    ) -> Result<DisassociateRepositoryResponse, RusotoError<DisassociateRepositoryError>>;

    /// <p> Lists all the code reviews that the customer has created in the past 90 days. </p>
    async fn list_code_reviews(
        &self,
        input: ListCodeReviewsRequest,
    ) -> Result<ListCodeReviewsResponse, RusotoError<ListCodeReviewsError>>;

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RecommendationFeedbackSummary.html"> <code>RecommendationFeedbackSummary</code> </a> objects that contain customer recommendation feedback for all CodeGuru Reviewer users. </p>
    async fn list_recommendation_feedback(
        &self,
        input: ListRecommendationFeedbackRequest,
    ) -> Result<ListRecommendationFeedbackResponse, RusotoError<ListRecommendationFeedbackError>>;

    /// <p> Returns the list of all recommendations for a completed code review. </p>
    async fn list_recommendations(
        &self,
        input: ListRecommendationsRequest,
    ) -> Result<ListRecommendationsResponse, RusotoError<ListRecommendationsError>>;

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html"> <code>RepositoryAssociationSummary</code> </a> objects that contain summary information about a repository association. You can filter the returned list by <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-ProviderType"> <code>ProviderType</code> </a>, <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-Name"> <code>Name</code> </a>, <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-State"> <code>State</code> </a>, and <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-Owner"> <code>Owner</code> </a>. </p>
    async fn list_repository_associations(
        &self,
        input: ListRepositoryAssociationsRequest,
    ) -> Result<ListRepositoryAssociationsResponse, RusotoError<ListRepositoryAssociationsError>>;

    /// <p> Stores customer feedback for a CodeGuru Reviewer recommendation. When this API is called again with different reactions the previous feedback is overwritten. </p>
    async fn put_recommendation_feedback(
        &self,
        input: PutRecommendationFeedbackRequest,
    ) -> Result<PutRecommendationFeedbackResponse, RusotoError<PutRecommendationFeedbackError>>;
}
/// A client for the CodeGuruReviewer API.
#[derive(Clone)]
pub struct CodeGuruReviewerClient {
    client: Client,
    region: region::Region,
}

impl CodeGuruReviewerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeGuruReviewerClient {
        CodeGuruReviewerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeGuruReviewerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeGuruReviewerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeGuruReviewerClient {
        CodeGuruReviewerClient { client, region }
    }
}

#[async_trait]
impl CodeGuruReviewer for CodeGuruReviewerClient {
    /// <p><p> Use to associate an AWS CodeCommit repository or a repostory managed by AWS CodeStar Connections with Amazon CodeGuru Reviewer. When you associate a repository, CodeGuru Reviewer reviews source code changes in the repository&#39;s pull requests and provides automatic recommendations. You can view recommendations using the CodeGuru Reviewer console. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/recommendations.html">Recommendations in Amazon CodeGuru Reviewer</a> in the <i>Amazon CodeGuru Reviewer User Guide.</i> </p> <p>If you associate a CodeCommit repository, it must be in the same AWS Region and AWS account where its CodeGuru Reviewer code reviews are configured.</p> <p> Bitbucket and GitHub Enterprise Server repositories are managed by AWS CodeStar Connections to connect to CodeGuru Reviewer. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/reviewer-ug/step-one.html#select-repository-source-provider">Connect to a repository source provider</a> in the <i>Amazon CodeGuru Reviewer User Guide.</i> </p> <note> <p> You cannot use the CodeGuru Reviewer SDK or the AWS CLI to associate a GitHub repository with Amazon CodeGuru Reviewer. To associate a GitHub repository, use the console. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/getting-started-with-guru.html">Getting started with CodeGuru Reviewer</a> in the <i>CodeGuru Reviewer User Guide.</i> </p> </note></p>
    async fn associate_repository(
        &self,
        input: AssociateRepositoryRequest,
    ) -> Result<AssociateRepositoryResponse, RusotoError<AssociateRepositoryError>> {
        let request_uri = "/associations";

        let mut request =
            SignedRequest::new("POST", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateRepositoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateRepositoryError::from_response(response))
        }
    }

    /// <p> Returns the metadata associated with the code review along with its status.</p>
    async fn describe_code_review(
        &self,
        input: DescribeCodeReviewRequest,
    ) -> Result<DescribeCodeReviewResponse, RusotoError<DescribeCodeReviewError>> {
        let request_uri = format!(
            "/codereviews/{code_review_arn}",
            code_review_arn = input.code_review_arn
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeCodeReviewResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCodeReviewError::from_response(response))
        }
    }

    /// <p> Describes the customer feedback for a CodeGuru Reviewer recommendation. </p>
    async fn describe_recommendation_feedback(
        &self,
        input: DescribeRecommendationFeedbackRequest,
    ) -> Result<
        DescribeRecommendationFeedbackResponse,
        RusotoError<DescribeRecommendationFeedbackError>,
    > {
        let request_uri = format!(
            "/feedback/{code_review_arn}",
            code_review_arn = input.code_review_arn
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("RecommendationId", &input.recommendation_id);
        if let Some(ref x) = input.user_id {
            params.put("UserId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRecommendationFeedbackResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRecommendationFeedbackError::from_response(response))
        }
    }

    /// <p> Returns a <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociation.html"> <code>RepositoryAssociation</code> </a> object that contains information about the requested repository association. </p>
    async fn describe_repository_association(
        &self,
        input: DescribeRepositoryAssociationRequest,
    ) -> Result<
        DescribeRepositoryAssociationResponse,
        RusotoError<DescribeRepositoryAssociationError>,
    > {
        let request_uri = format!(
            "/associations/{association_arn}",
            association_arn = input.association_arn
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRepositoryAssociationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRepositoryAssociationError::from_response(response))
        }
    }

    /// <p>Removes the association between Amazon CodeGuru Reviewer and a repository.</p>
    async fn disassociate_repository(
        &self,
        input: DisassociateRepositoryRequest,
    ) -> Result<DisassociateRepositoryResponse, RusotoError<DisassociateRepositoryError>> {
        let request_uri = format!(
            "/associations/{association_arn}",
            association_arn = input.association_arn
        );

        let mut request =
            SignedRequest::new("DELETE", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateRepositoryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateRepositoryError::from_response(response))
        }
    }

    /// <p> Lists all the code reviews that the customer has created in the past 90 days. </p>
    async fn list_code_reviews(
        &self,
        input: ListCodeReviewsRequest,
    ) -> Result<ListCodeReviewsResponse, RusotoError<ListCodeReviewsError>> {
        let request_uri = "/codereviews";

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.provider_types {
            for item in x.iter() {
                params.put("ProviderTypes", item);
            }
        }
        if let Some(ref x) = input.repository_names {
            for item in x.iter() {
                params.put("RepositoryNames", item);
            }
        }
        if let Some(ref x) = input.states {
            for item in x.iter() {
                params.put("States", item);
            }
        }
        params.put("Type", &input.type_);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListCodeReviewsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListCodeReviewsError::from_response(response))
        }
    }

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RecommendationFeedbackSummary.html"> <code>RecommendationFeedbackSummary</code> </a> objects that contain customer recommendation feedback for all CodeGuru Reviewer users. </p>
    async fn list_recommendation_feedback(
        &self,
        input: ListRecommendationFeedbackRequest,
    ) -> Result<ListRecommendationFeedbackResponse, RusotoError<ListRecommendationFeedbackError>>
    {
        let request_uri = format!(
            "/feedback/{code_review_arn}/RecommendationFeedback",
            code_review_arn = input.code_review_arn
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.recommendation_ids {
            for item in x.iter() {
                params.put("RecommendationIds", item);
            }
        }
        if let Some(ref x) = input.user_ids {
            for item in x.iter() {
                params.put("UserIds", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRecommendationFeedbackResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRecommendationFeedbackError::from_response(response))
        }
    }

    /// <p> Returns the list of all recommendations for a completed code review. </p>
    async fn list_recommendations(
        &self,
        input: ListRecommendationsRequest,
    ) -> Result<ListRecommendationsResponse, RusotoError<ListRecommendationsError>> {
        let request_uri = format!(
            "/codereviews/{code_review_arn}/Recommendations",
            code_review_arn = input.code_review_arn
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRecommendationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRecommendationsError::from_response(response))
        }
    }

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html"> <code>RepositoryAssociationSummary</code> </a> objects that contain summary information about a repository association. You can filter the returned list by <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-ProviderType"> <code>ProviderType</code> </a>, <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-Name"> <code>Name</code> </a>, <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-State"> <code>State</code> </a>, and <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-Owner"> <code>Owner</code> </a>. </p>
    async fn list_repository_associations(
        &self,
        input: ListRepositoryAssociationsRequest,
    ) -> Result<ListRepositoryAssociationsResponse, RusotoError<ListRepositoryAssociationsError>>
    {
        let request_uri = "/associations";

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.names {
            for item in x.iter() {
                params.put("Name", item);
            }
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.owners {
            for item in x.iter() {
                params.put("Owner", item);
            }
        }
        if let Some(ref x) = input.provider_types {
            for item in x.iter() {
                params.put("ProviderType", item);
            }
        }
        if let Some(ref x) = input.states {
            for item in x.iter() {
                params.put("State", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRepositoryAssociationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRepositoryAssociationsError::from_response(response))
        }
    }

    /// <p> Stores customer feedback for a CodeGuru Reviewer recommendation. When this API is called again with different reactions the previous feedback is overwritten. </p>
    async fn put_recommendation_feedback(
        &self,
        input: PutRecommendationFeedbackRequest,
    ) -> Result<PutRecommendationFeedbackResponse, RusotoError<PutRecommendationFeedbackError>>
    {
        let request_uri = "/feedback";

        let mut request =
            SignedRequest::new("PUT", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutRecommendationFeedbackResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutRecommendationFeedbackError::from_response(response))
        }
    }
}
