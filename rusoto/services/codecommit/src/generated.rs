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

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Represents the input of a batch get repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetRepositoriesInput {
    /// <p>The names of the repositories to get information about.</p>
    #[serde(rename = "repositoryNames")]
    pub repository_names: Vec<String>,
}

/// <p>Represents the output of a batch get repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetRepositoriesOutput {
    /// <p>A list of repositories returned by the batch get repositories operation.</p>
    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<RepositoryMetadata>>,
    /// <p>Returns a list of repository names for which information could not be found.</p>
    #[serde(rename = "repositoriesNotFound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories_not_found: Option<Vec<String>>,
}

/// <p>Returns information about a specific Git blob object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BlobMetadata {
    /// <p>The full ID of the blob.</p>
    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    /// <p><p>The file mode permissions of the blob. File mode permission codes include:</p> <ul> <li> <p> <code>100644</code> indicates read/write</p> </li> <li> <p> <code>100755</code> indicates read/write/execute</p> </li> <li> <p> <code>160000</code> indicates a submodule</p> </li> <li> <p> <code>120000</code> indicates a symlink</p> </li> </ul></p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>The path to the blob and any associated file name, if any.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>Returns information about a branch.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BranchInfo {
    /// <p>The name of the branch.</p>
    #[serde(rename = "branchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// <p>The ID of the last commit made to the branch.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
}

/// <p>Returns information about a specific comment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Comment {
    /// <p>The Amazon Resource Name (ARN) of the person who posted the comment.</p>
    #[serde(rename = "authorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    /// <p>A unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The system-generated comment ID.</p>
    #[serde(rename = "commentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// <p>The content of the comment.</p>
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The date and time the comment was created, in timestamp format.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A Boolean value indicating whether the comment has been deleted.</p>
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// <p>The ID of the comment for which this comment is a reply, if any.</p>
    #[serde(rename = "inReplyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    /// <p>The date and time the comment was most recently modified, in timestamp format.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
}

/// <p>Returns information about comments on the comparison between two commits.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CommentsForComparedCommit {
    /// <p>The full blob ID of the commit used to establish the 'after' of the comparison.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>The full commit ID of the commit used to establish the 'after' of the comparison.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full blob ID of the commit used to establish the 'before' of the comparison.</p>
    #[serde(rename = "beforeBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    /// <p>The full commit ID of the commit used to establish the 'before' of the comparison.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>An array of comment objects. Each comment object contains information about a comment on the comparison between commits.</p>
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    /// <p>Location information about the comment on the comparison, including the file name, line number, and whether the version of the file where the comment was made is 'BEFORE' or 'AFTER'.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The name of the repository that contains the compared commits.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Returns information about comments on a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CommentsForPullRequest {
    /// <p>The full blob ID of the file on which you want to comment on the source commit.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>he full commit ID of the commit that was the tip of the source branch at the time the comment was made. </p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full blob ID of the file on which you want to comment on the destination commit.</p>
    #[serde(rename = "beforeBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    /// <p>The full commit ID of the commit that was the tip of the destination branch when the pull request was created. This commit will be superceded by the after commit in the source branch when and if you merge the source branch into the destination branch.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>An array of comment objects. Each comment object contains information about a comment on the pull request.</p>
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    /// <p>Location information about the comment on the pull request, including the file name, line number, and whether the version of the file where the comment was made is 'BEFORE' (destination branch) or 'AFTER' (source branch).</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The system-generated ID of the pull request.</p>
    #[serde(rename = "pullRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    /// <p>The name of the repository that contains the pull request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Returns information about a specific commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Commit {
    /// <p>Any additional data associated with the specified commit.</p>
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    /// <p>Information about the author of the specified commit. Information includes the date in timestamp format with GMT offset, the name of the author, and the email address for the author, as configured in Git.</p>
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserInfo>,
    /// <p>The full SHA of the specified commit. </p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>Information about the person who committed the specified commit, also known as the committer. Information includes the date in timestamp format with GMT offset, the name of the committer, and the email address for the committer, as configured in Git.</p> <p>For more information about the difference between an author and a committer in Git, see <a href="http://git-scm.com/book/ch2-3.html">Viewing the Commit History</a> in Pro Git by Scott Chacon and Ben Straub.</p>
    #[serde(rename = "committer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer: Option<UserInfo>,
    /// <p>The commit message associated with the specified commit.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A list of parent commits for the specified commit. Each parent commit ID is the full commit ID.</p>
    #[serde(rename = "parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    /// <p>Tree information for the specified commit.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

/// <p>Represents the input of a create branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBranchInput {
    /// <p>The name of the new branch to create.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>The ID of the commit to point the new branch to.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The name of the repository in which you want to create the new branch.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePullRequestInput {
    /// <p><p>A unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p> <note> <p>The AWS SDKs prepopulate client request tokens. If using an AWS SDK, you do not have to generate an idempotency token, as this will be done for you.</p> </note></p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>A description of the pull request.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The targets for the pull request, including the source of the code to be reviewed (the source branch), and the destination where the creator of the pull request intends the code to be merged after the pull request is closed (the destination branch).</p>
    #[serde(rename = "targets")]
    pub targets: Vec<Target>,
    /// <p>The title of the pull request. This title will be used to identify the pull request to other users in the repository.</p>
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePullRequestOutput {
    /// <p>Information about the newly created pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

/// <p>Represents the input of a create repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRepositoryInput {
    /// <p><p>A comment or description about the new repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note></p>
    #[serde(rename = "repositoryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    /// <p><p>The name of the new repository to be created.</p> <note> <p>The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. For a full description of the limits on repository names, see <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide. The suffix &quot;.git&quot; is prohibited.</p> </note></p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a create repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateRepositoryOutput {
    /// <p>Information about the newly created repository.</p>
    #[serde(rename = "repositoryMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

/// <p>Represents the input of a delete branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBranchInput {
    /// <p>The name of the branch to delete.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>The name of the repository that contains the branch to be deleted.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a delete branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteBranchOutput {
    /// <p>Information about the branch deleted by the operation, including the branch name and the commit ID that was the tip of the branch.</p>
    #[serde(rename = "deletedBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_branch: Option<BranchInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCommentContentInput {
    /// <p>The unique, system-generated ID of the comment. To get this ID, use <a>GetCommentsForComparedCommit</a> or <a>GetCommentsForPullRequest</a>.</p>
    #[serde(rename = "commentId")]
    pub comment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteCommentContentOutput {
    /// <p>Information about the comment you just deleted.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>Represents the input of a delete repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRepositoryInput {
    /// <p>The name of the repository to delete.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a delete repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteRepositoryOutput {
    /// <p>The ID of the repository that was deleted.</p>
    #[serde(rename = "repositoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePullRequestEventsInput {
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with additional commits or changing the status of a pull request.</p>
    #[serde(rename = "actorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_arn: Option<String>,
    /// <p>A non-negative integer used to limit the number of returned results. The default is 100 events, which is also the maximum number of events that can be returned in a result.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. The pull request event type about which you want to return information.</p>
    #[serde(rename = "pullRequestEventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_event_type: Option<String>,
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePullRequestEventsOutput {
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the pull request events.</p>
    #[serde(rename = "pullRequestEvents")]
    pub pull_request_events: Vec<PullRequestEvent>,
}

/// <p>Returns information about a set of differences for a commit specifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Difference {
    /// <p>Information about an <code>afterBlob</code> data type object, including the ID, the file mode permission code, and the path.</p>
    #[serde(rename = "afterBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob: Option<BlobMetadata>,
    /// <p>Information about a <code>beforeBlob</code> data type object, including the ID, the file mode permission code, and the path.</p>
    #[serde(rename = "beforeBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob: Option<BlobMetadata>,
    /// <p>Whether the change type of the difference is an addition (A), deletion (D), or modification (M).</p>
    #[serde(rename = "changeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
}

/// <p>Represents the input of a get blob operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBlobInput {
    /// <p>The ID of the blob, which is its SHA-1 pointer.</p>
    #[serde(rename = "blobId")]
    pub blob_id: String,
    /// <p>The name of the repository that contains the blob.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a get blob operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBlobOutput {
    /// <p>The content of the blob, usually a file.</p>
    #[serde(rename = "content")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub content: Vec<u8>,
}

/// <p>Represents the input of a get branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBranchInput {
    /// <p>The name of the branch for which you want to retrieve information.</p>
    #[serde(rename = "branchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// <p>The name of the repository that contains the branch for which you want to retrieve information.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Represents the output of a get branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBranchOutput {
    /// <p>The name of the branch.</p>
    #[serde(rename = "branch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCommentInput {
    /// <p>The unique, system-generated ID of the comment. To get this ID, use <a>GetCommentsForComparedCommit</a> or <a>GetCommentsForPullRequest</a>.</p>
    #[serde(rename = "commentId")]
    pub comment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCommentOutput {
    /// <p>The contents of the comment.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCommentsForComparedCommitInput {
    /// <p>To establish the directionality of the comparison, the full commit ID of the 'after' commit.</p>
    #[serde(rename = "afterCommitId")]
    pub after_commit_id: String,
    /// <p>To establish the directionality of the comparison, the full commit ID of the 'before' commit.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>A non-negative integer used to limit the number of returned results. The default is 100 comments, and is configurable up to 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that when provided in a request, returns the next batch of the results. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the repository where you want to compare commits.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCommentsForComparedCommitOutput {
    /// <p>A list of comment objects on the compared commit.</p>
    #[serde(rename = "commentsForComparedCommitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_for_compared_commit_data: Option<Vec<CommentsForComparedCommit>>,
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCommentsForPullRequestInput {
    /// <p>The full commit ID of the commit in the source branch that was the tip of the branch at the time the comment was made.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full commit ID of the commit in the destination branch that was the tip of the branch at the time the pull request was created.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>A non-negative integer used to limit the number of returned results. The default is 100 comments. You can return up to 500 comments with a single request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The name of the repository that contains the pull request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCommentsForPullRequestOutput {
    /// <p>An array of comment objects on the pull request.</p>
    #[serde(rename = "commentsForPullRequestData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_for_pull_request_data: Option<Vec<CommentsForPullRequest>>,
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a get commit operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCommitInput {
    /// <p>The commit ID. Commit IDs are the full SHA of the commit.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The name of the repository to which the commit was made.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a get commit operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCommitOutput {
    /// <p>A commit data type object that contains information about the specified commit.</p>
    #[serde(rename = "commit")]
    pub commit: Commit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDifferencesInput {
    /// <p>A non-negative integer used to limit the number of returned results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit.</p>
    #[serde(rename = "afterCommitSpecifier")]
    pub after_commit_specifier: String,
    /// <p>The file path in which to check differences. Limits the results to this path. Can also be used to specify the changed name of a directory or folder, if it has changed. If not specified, differences will be shown for all paths.</p>
    #[serde(rename = "afterPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_path: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit. For example, the full commit ID. Optional. If not specified, all changes prior to the <code>afterCommitSpecifier</code> value will be shown. If you do not use <code>beforeCommitSpecifier</code> in your request, consider limiting the results with <code>maxResults</code>.</p>
    #[serde(rename = "beforeCommitSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_specifier: Option<String>,
    /// <p>The file path in which to check for differences. Limits the results to this path. Can also be used to specify the previous name of a directory or folder. If <code>beforePath</code> and <code>afterPath</code> are not specified, differences will be shown for all paths.</p>
    #[serde(rename = "beforePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_path: Option<String>,
    /// <p>The name of the repository where you want to get differences.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDifferencesOutput {
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A differences data type object that contains information about the differences, including whether the difference is added, modified, or deleted (A, D, M).</p>
    #[serde(rename = "differences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differences: Option<Vec<Difference>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMergeConflictsInput {
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit. For example, a branch name or a full commit ID.</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The merge option or strategy you want to use to merge the code. The only valid value is FAST_FORWARD_MERGE.</p>
    #[serde(rename = "mergeOption")]
    pub merge_option: String,
    /// <p>The name of the repository where the pull request was created.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit. For example, a branch name or a full commit ID.</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetMergeConflictsOutput {
    /// <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "destinationCommitId")]
    pub destination_commit_id: String,
    /// <p>A Boolean value that indicates whether the code is mergable by the specified merge option.</p>
    #[serde(rename = "mergeable")]
    pub mergeable: bool,
    /// <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "sourceCommitId")]
    pub source_commit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPullRequestInput {
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetPullRequestOutput {
    /// <p>Information about the specified pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

/// <p>Represents the input of a get repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRepositoryInput {
    /// <p>The name of the repository to get information about.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a get repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRepositoryOutput {
    /// <p>Information about the repository.</p>
    #[serde(rename = "repositoryMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

/// <p>Represents the input of a get repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRepositoryTriggersInput {
    /// <p>The name of the repository for which the trigger is configured.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a get repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRepositoryTriggersOutput {
    /// <p>The system-generated unique ID for the trigger.</p>
    #[serde(rename = "configurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    /// <p>The JSON block of configuration information for each trigger.</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RepositoryTrigger>>,
}

/// <p>Represents the input of a list branches operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBranchesInput {
    /// <p>An enumeration token that allows the operation to batch the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the repository that contains the branches.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a list branches operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListBranchesOutput {
    /// <p>The list of branch names.</p>
    #[serde(rename = "branches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    /// <p>An enumeration token that returns the batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPullRequestsInput {
    /// <p>Optional. The Amazon Resource Name (ARN) of the user who created the pull request. If used, this filters the results to pull requests created by that user.</p>
    #[serde(rename = "authorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    /// <p>A non-negative integer used to limit the number of returned results.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. The status of the pull request. If used, this refines the results to the pull requests that match the specified status.</p>
    #[serde(rename = "pullRequestStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
    /// <p>The name of the repository for which you want to list pull requests.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPullRequestsOutput {
    /// <p>An enumeration token that when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The system-generated IDs of the pull requests.</p>
    #[serde(rename = "pullRequestIds")]
    pub pull_request_ids: Vec<String>,
}

/// <p>Represents the input of a list repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRepositoriesInput {
    /// <p>An enumeration token that allows the operation to batch the results of the operation. Batch sizes are 1,000 for list repository operations. When the client sends the token back to AWS CodeCommit, another page of 1,000 records is retrieved.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The order in which to sort the results of a list repositories operation.</p>
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// <p>The criteria used to sort the results of a list repositories operation.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

/// <p>Represents the output of a list repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListRepositoriesOutput {
    /// <p>An enumeration token that allows the operation to batch the results of the operation. Batch sizes are 1,000 for list repository operations. When the client sends the token back to AWS CodeCommit, another page of 1,000 records is retrieved.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Lists the repositories called by the list repositories operation.</p>
    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<RepositoryNameIdPair>>,
}

/// <p>Returns information about the location of a change or comment in the comparison between two commits or a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// <p>The name of the file being compared, including its extension and subdirectory, if any.</p>
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// <p>The position of a change within a compared file, in line number format.</p>
    #[serde(rename = "filePosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_position: Option<i64>,
    /// <p>In a comparison of commits or a pull request, whether the change is in the 'before' or 'after' of that comparison.</p>
    #[serde(rename = "relativeFileVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_file_version: Option<String>,
}

/// <p>Returns information about a merge or potential merge between a source reference and a destination reference in a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MergeMetadata {
    /// <p>A Boolean value indicating whether the merge has been made.</p>
    #[serde(rename = "isMerged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_merged: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the user who merged the branches.</p>
    #[serde(rename = "mergedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MergePullRequestByFastForwardInput {
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The name of the repository where the pull request was created.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The full commit ID of the original or updated commit in the pull request source branch. Pass this value if you want an exception thrown if the current commit ID of the tip of the source branch does not match this commit ID.</p>
    #[serde(rename = "sourceCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MergePullRequestByFastForwardOutput {
    /// <p>Information about the specified pull request, including information about the merge.</p>
    #[serde(rename = "pullRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PostCommentForComparedCommitInput {
    /// <p>To establish the directionality of the comparison, the full commit ID of the 'after' commit.</p>
    #[serde(rename = "afterCommitId")]
    pub after_commit_id: String,
    /// <p>To establish the directionality of the comparison, the full commit ID of the 'before' commit.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>A unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The content of the comment you want to make.</p>
    #[serde(rename = "content")]
    pub content: String,
    /// <p>The location of the comparison where you want to comment.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The name of the repository where you want to post a comment on the comparison between commits.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PostCommentForComparedCommitOutput {
    /// <p>In the directionality you established, the blob ID of the 'after' blob.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>In the directionality you established, the full commit ID of the 'after' commit.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>In the directionality you established, the blob ID of the 'before' blob.</p>
    #[serde(rename = "beforeBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    /// <p>In the directionality you established, the full commit ID of the 'before' commit.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>The content of the comment you posted.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    /// <p>The location of the comment in the comparison between the two commits.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The name of the repository where you posted a comment on the comparison between commits.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PostCommentForPullRequestInput {
    /// <p>The full commit ID of the commit in the source branch that is the current tip of the branch for the pull request when you post the comment.</p>
    #[serde(rename = "afterCommitId")]
    pub after_commit_id: String,
    /// <p>The full commit ID of the commit in the destination branch that was the tip of the branch at the time the pull request was created.</p>
    #[serde(rename = "beforeCommitId")]
    pub before_commit_id: String,
    /// <p>A unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The content of your comment on the change.</p>
    #[serde(rename = "content")]
    pub content: String,
    /// <p>The location of the change where you want to post your comment. If no location is provided, the comment will be posted as a general comment on the pull request difference between the before commit ID and the after commit ID.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The name of the repository where you want to post a comment on a pull request.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PostCommentForPullRequestOutput {
    /// <p>In the directionality of the pull request, the blob ID of the 'after' blob.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>The full commit ID of the commit in the destination branch where the pull request will be merged.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>In the directionality of the pull request, the blob ID of the 'before' blob.</p>
    #[serde(rename = "beforeBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    /// <p>The full commit ID of the commit in the source branch used to create the pull request, or in the case of an updated pull request, the full commit ID of the commit used to update the pull request.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>The content of the comment you posted.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    /// <p>The location of the change where you posted your comment.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The system-generated ID of the pull request. </p>
    #[serde(rename = "pullRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    /// <p>The name of the repository where you posted a comment on a pull request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PostCommentReplyInput {
    /// <p>A unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The contents of your reply to a comment.</p>
    #[serde(rename = "content")]
    pub content: String,
    /// <p>The system-generated ID of the comment to which you want to reply. To get this ID, use <a>GetCommentsForComparedCommit</a> or <a>GetCommentsForPullRequest</a>.</p>
    #[serde(rename = "inReplyTo")]
    pub in_reply_to: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PostCommentReplyOutput {
    /// <p>Information about the reply to a comment.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>Returns information about a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PullRequest {
    /// <p>The Amazon Resource Name (ARN) of the user who created the pull request.</p>
    #[serde(rename = "authorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    /// <p>A unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The date and time the pull request was originally created, in timestamp format.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The user-defined description of the pull request. This description can be used to clarify what should be reviewed and other details of the request.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The day and time of the last user or system activity on the pull request, in timestamp format.</p>
    #[serde(rename = "lastActivityDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity_date: Option<f64>,
    /// <p>The system-generated ID of the pull request. </p>
    #[serde(rename = "pullRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    /// <p>The status of the pull request. Pull request status can only change from <code>OPEN</code> to <code>CLOSED</code>.</p>
    #[serde(rename = "pullRequestStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
    /// <p>The targets of the pull request, including the source branch and destination branch for the pull request.</p>
    #[serde(rename = "pullRequestTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_targets: Option<Vec<PullRequestTarget>>,
    /// <p>The user-defined title of the pull request. This title is displayed in the list of pull requests to other users of the repository.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Returns information about a pull request event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PullRequestEvent {
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with additional commits or changing the status of a pull request.</p>
    #[serde(rename = "actorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_arn: Option<String>,
    /// <p>The day and time of the pull request event, in timestamp format.</p>
    #[serde(rename = "eventDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<f64>,
    /// <p>The type of the pull request event, for example a status change event (PULL_REQUEST_STATUS_CHANGED) or update event (PULL_REQUEST_SOURCE_REFERENCE_UPDATED).</p>
    #[serde(rename = "pullRequestEventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_event_type: Option<String>,
    /// <p>The system-generated ID of the pull request.</p>
    #[serde(rename = "pullRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    /// <p>Information about the change in mergability state for the pull request event.</p>
    #[serde(rename = "pullRequestMergedStateChangedEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_merged_state_changed_event_metadata:
        Option<PullRequestMergedStateChangedEventMetadata>,
    /// <p>Information about the updated source branch for the pull request event. </p>
    #[serde(rename = "pullRequestSourceReferenceUpdatedEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_source_reference_updated_event_metadata:
        Option<PullRequestSourceReferenceUpdatedEventMetadata>,
    /// <p>Information about the change in status for the pull request event.</p>
    #[serde(rename = "pullRequestStatusChangedEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status_changed_event_metadata: Option<PullRequestStatusChangedEventMetadata>,
}

/// <p>Returns information about the change in the merge state for a pull request event. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PullRequestMergedStateChangedEventMetadata {
    /// <p>The name of the branch that the pull request will be merged into.</p>
    #[serde(rename = "destinationReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_reference: Option<String>,
    /// <p>Information about the merge state change event.</p>
    #[serde(rename = "mergeMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_metadata: Option<MergeMetadata>,
    /// <p>The name of the repository where the pull request was created.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Information about an update to the source branch of a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PullRequestSourceReferenceUpdatedEventMetadata {
    /// <p>The full commit ID of the commit in the source branch that was the tip of the branch at the time the pull request was updated.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full commit ID of the commit in the destination branch that was the tip of the branch at the time the pull request was updated.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>The name of the repository where the pull request was updated.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Information about a change to the status of a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PullRequestStatusChangedEventMetadata {
    /// <p>The changed status of the pull request.</p>
    #[serde(rename = "pullRequestStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
}

/// <p>Returns information about a pull request target.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PullRequestTarget {
    /// <p>The full commit ID that is the tip of the destination branch. This is the commit where the pull request was or will be merged.</p>
    #[serde(rename = "destinationCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit: Option<String>,
    /// <p>The branch of the repository where the pull request changes will be merged into. Also known as the destination branch. </p>
    #[serde(rename = "destinationReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_reference: Option<String>,
    /// <p>Returns metadata about the state of the merge, including whether the merge has been made.</p>
    #[serde(rename = "mergeMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_metadata: Option<MergeMetadata>,
    /// <p>The name of the repository that contains the pull request source and destination branches.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p>The full commit ID of the tip of the source branch used to create the pull request. If the pull request branch is updated by a push while the pull request is open, the commit ID will change to reflect the new tip of the branch.</p>
    #[serde(rename = "sourceCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
    /// <p>The branch of the repository that contains the changes for the pull request. Also known as the source branch.</p>
    #[serde(rename = "sourceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutFileInput {
    /// <p>The name of the branch where you want to add or update the file.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>A message about why this file was added or updated. While optional, adding a message is strongly encouraged in order to provide a more useful commit history for your repository.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>An email address for the person adding or updating the file.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The content of the file, in binary object format. </p>
    #[serde(rename = "fileContent")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub file_content: Vec<u8>,
    /// <p>The file mode permissions of the blob. Valid file mode permissions are listed below.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p><p>The name of the file you want to add or update, including the relative path to the file in the repository.</p> <note> <p>If the path does not currently exist in the repository, the path will be created as part of adding the file.</p> </note></p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The name of the person adding or updating the file. While optional, adding a name is strongly encouraged in order to provide a more useful commit history for your repository.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The full commit ID of the head commit in the branch where you want to add or update the file. If the commit ID does not match the ID of the head commit at the time of the operation, an error will occur, and the file will not be added or updated.</p>
    #[serde(rename = "parentCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_commit_id: Option<String>,
    /// <p>The name of the repository where you want to add or update the file.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutFileOutput {
    /// <p>The ID of the blob, which is its SHA-1 pointer.</p>
    #[serde(rename = "blobId")]
    pub blob_id: String,
    /// <p>The full SHA of the commit that contains this file change.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>Tree information for the commit that contains this file change.</p>
    #[serde(rename = "treeId")]
    pub tree_id: String,
}

/// <p>Represents the input ofa put repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRepositoryTriggersInput {
    /// <p>The name of the repository where you want to create or update the trigger.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The JSON block of configuration information for each trigger.</p>
    #[serde(rename = "triggers")]
    pub triggers: Vec<RepositoryTrigger>,
}

/// <p>Represents the output of a put repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutRepositoryTriggersOutput {
    /// <p>The system-generated unique ID for the create or update operation.</p>
    #[serde(rename = "configurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

/// <p>Information about a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RepositoryMetadata {
    /// <p>The Amazon Resource Name (ARN) of the repository.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the AWS account associated with the repository.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The URL to use for cloning the repository over HTTPS.</p>
    #[serde(rename = "cloneUrlHttp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url_http: Option<String>,
    /// <p>The URL to use for cloning the repository over SSH.</p>
    #[serde(rename = "cloneUrlSsh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url_ssh: Option<String>,
    /// <p>The date and time the repository was created, in timestamp format.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The repository's default branch name.</p>
    #[serde(rename = "defaultBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    /// <p>The date and time the repository was last modified, in timestamp format.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>A comment or description about the repository.</p>
    #[serde(rename = "repositoryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    /// <p>The ID of the repository.</p>
    #[serde(rename = "repositoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    /// <p>The repository's name.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Information about a repository name and ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RepositoryNameIdPair {
    /// <p>The ID associated with the repository.</p>
    #[serde(rename = "repositoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    /// <p>The name associated with the repository.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Information about a trigger for a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryTrigger {
    /// <p><p>The branches that will be included in the trigger configuration. If you specify an empty array, the trigger will apply to all branches.</p> <note> <p>While no content is required in the array, you must include the array itself.</p> </note></p>
    #[serde(rename = "branches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    /// <p>Any custom data associated with the trigger that will be included in the information sent to the target of the trigger.</p>
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    /// <p>The ARN of the resource that is the target for a trigger. For example, the ARN of a topic in Amazon Simple Notification Service (SNS).</p>
    #[serde(rename = "destinationArn")]
    pub destination_arn: String,
    /// <p><p>The repository events that will cause the trigger to run actions in another service, such as sending a notification through Amazon Simple Notification Service (SNS). </p> <note> <p>The valid value &quot;all&quot; cannot be used with any other values.</p> </note></p>
    #[serde(rename = "events")]
    pub events: Vec<String>,
    /// <p>The name of the trigger.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>A trigger failed to run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RepositoryTriggerExecutionFailure {
    /// <p>Additional message information about the trigger that did not run.</p>
    #[serde(rename = "failureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>The name of the trigger that did not run.</p>
    #[serde(rename = "trigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

/// <p>Returns information about a target for a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Target {
    /// <p>The branch of the repository where the pull request changes will be merged into. Also known as the destination branch.</p>
    #[serde(rename = "destinationReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_reference: Option<String>,
    /// <p>The name of the repository that contains the pull request.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch of the repository that contains the changes for the pull request. Also known as the source branch.</p>
    #[serde(rename = "sourceReference")]
    pub source_reference: String,
}

/// <p>Represents the input of a test repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestRepositoryTriggersInput {
    /// <p>The name of the repository in which to test the triggers.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The list of triggers to test.</p>
    #[serde(rename = "triggers")]
    pub triggers: Vec<RepositoryTrigger>,
}

/// <p>Represents the output of a test repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TestRepositoryTriggersOutput {
    /// <p>The list of triggers that were not able to be tested. This list provides the names of the triggers that could not be tested, separated by commas.</p>
    #[serde(rename = "failedExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_executions: Option<Vec<RepositoryTriggerExecutionFailure>>,
    /// <p>The list of triggers that were successfully tested. This list provides the names of the triggers that were successfully tested, separated by commas.</p>
    #[serde(rename = "successfulExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_executions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCommentInput {
    /// <p>The system-generated ID of the comment you want to update. To get this ID, use <a>GetCommentsForComparedCommit</a> or <a>GetCommentsForPullRequest</a>.</p>
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// <p>The updated content with which you want to replace the existing content of the comment.</p>
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateCommentOutput {
    /// <p>Information about the updated comment.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>Represents the input of an update default branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDefaultBranchInput {
    /// <p>The name of the branch to set as the default.</p>
    #[serde(rename = "defaultBranchName")]
    pub default_branch_name: String,
    /// <p>The name of the repository to set or change the default branch for.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePullRequestDescriptionInput {
    /// <p>The updated content of the description for the pull request. This content will replace the existing description.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdatePullRequestDescriptionOutput {
    /// <p>Information about the updated pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePullRequestStatusInput {
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The status of the pull request. The only valid operations are to update the status from <code>OPEN</code> to <code>OPEN</code>, <code>OPEN</code> to <code>CLOSED</code> or from from <code>CLOSED</code> to <code>CLOSED</code>.</p>
    #[serde(rename = "pullRequestStatus")]
    pub pull_request_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdatePullRequestStatusOutput {
    /// <p>Information about the pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePullRequestTitleInput {
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The updated title of the pull request. This will replace the existing title.</p>
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdatePullRequestTitleOutput {
    /// <p>Information about the updated pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

/// <p>Represents the input of an update repository description operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRepositoryDescriptionInput {
    /// <p>The new comment or description for the specified repository. Repository descriptions are limited to 1,000 characters.</p>
    #[serde(rename = "repositoryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    /// <p>The name of the repository to set or change the comment or description for.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the input of an update repository description operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRepositoryNameInput {
    /// <p>The new name for the repository.</p>
    #[serde(rename = "newName")]
    pub new_name: String,
    /// <p>The existing name of the repository.</p>
    #[serde(rename = "oldName")]
    pub old_name: String,
}

/// <p>Information about the user who made a specified commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UserInfo {
    /// <p>The date when the specified commit was commited, in timestamp format with GMT offset.</p>
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// <p>The email address associated with the user who made the commit, if any.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The name of the user who made the specified commit.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by BatchGetRepositories
#[derive(Debug, PartialEq)]
pub enum BatchGetRepositoriesError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The maximum number of allowed repository names was exceeded. Currently, this number is 25.</p>
    MaximumRepositoryNamesExceeded(String),
    /// <p>A repository names object is required but was not specified.</p>
    RepositoryNamesRequired(String),
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

impl BatchGetRepositoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetRepositoriesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return BatchGetRepositoriesError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return BatchGetRepositoriesError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return BatchGetRepositoriesError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return BatchGetRepositoriesError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return BatchGetRepositoriesError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return BatchGetRepositoriesError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "MaximumRepositoryNamesExceededException" => {
                    return BatchGetRepositoriesError::MaximumRepositoryNamesExceeded(String::from(
                        error_message,
                    ))
                }
                "RepositoryNamesRequiredException" => {
                    return BatchGetRepositoriesError::RepositoryNamesRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchGetRepositoriesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchGetRepositoriesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetRepositoriesError {
    fn from(err: serde_json::error::Error) -> BatchGetRepositoriesError {
        BatchGetRepositoriesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetRepositoriesError {
    fn from(err: CredentialsError) -> BatchGetRepositoriesError {
        BatchGetRepositoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetRepositoriesError {
    fn from(err: HttpDispatchError) -> BatchGetRepositoriesError {
        BatchGetRepositoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetRepositoriesError {
    fn from(err: io::Error) -> BatchGetRepositoriesError {
        BatchGetRepositoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetRepositoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetRepositoriesError {
    fn description(&self) -> &str {
        match *self {
            BatchGetRepositoriesError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            BatchGetRepositoriesError::EncryptionKeyAccessDenied(ref cause) => cause,
            BatchGetRepositoriesError::EncryptionKeyDisabled(ref cause) => cause,
            BatchGetRepositoriesError::EncryptionKeyNotFound(ref cause) => cause,
            BatchGetRepositoriesError::EncryptionKeyUnavailable(ref cause) => cause,
            BatchGetRepositoriesError::InvalidRepositoryName(ref cause) => cause,
            BatchGetRepositoriesError::MaximumRepositoryNamesExceeded(ref cause) => cause,
            BatchGetRepositoriesError::RepositoryNamesRequired(ref cause) => cause,
            BatchGetRepositoriesError::Validation(ref cause) => cause,
            BatchGetRepositoriesError::Credentials(ref err) => err.description(),
            BatchGetRepositoriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetRepositoriesError::ParseError(ref cause) => cause,
            BatchGetRepositoriesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateBranch
#[derive(Debug, PartialEq)]
pub enum CreateBranchError {
    /// <p>The specified branch name already exists.</p>
    BranchNameExists(String),
    /// <p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl CreateBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BranchNameExistsException" => {
                    return CreateBranchError::BranchNameExists(String::from(error_message))
                }
                "BranchNameRequiredException" => {
                    return CreateBranchError::BranchNameRequired(String::from(error_message))
                }
                "CommitDoesNotExistException" => {
                    return CreateBranchError::CommitDoesNotExist(String::from(error_message))
                }
                "CommitIdRequiredException" => {
                    return CreateBranchError::CommitIdRequired(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return CreateBranchError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return CreateBranchError::EncryptionKeyAccessDenied(String::from(error_message))
                }
                "EncryptionKeyDisabledException" => {
                    return CreateBranchError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return CreateBranchError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return CreateBranchError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "InvalidBranchNameException" => {
                    return CreateBranchError::InvalidBranchName(String::from(error_message))
                }
                "InvalidCommitIdException" => {
                    return CreateBranchError::InvalidCommitId(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return CreateBranchError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return CreateBranchError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return CreateBranchError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateBranchError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateBranchError {
    fn from(err: serde_json::error::Error) -> CreateBranchError {
        CreateBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBranchError {
    fn from(err: CredentialsError) -> CreateBranchError {
        CreateBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBranchError {
    fn from(err: HttpDispatchError) -> CreateBranchError {
        CreateBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBranchError {
    fn from(err: io::Error) -> CreateBranchError {
        CreateBranchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBranchError {
    fn description(&self) -> &str {
        match *self {
            CreateBranchError::BranchNameExists(ref cause) => cause,
            CreateBranchError::BranchNameRequired(ref cause) => cause,
            CreateBranchError::CommitDoesNotExist(ref cause) => cause,
            CreateBranchError::CommitIdRequired(ref cause) => cause,
            CreateBranchError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            CreateBranchError::EncryptionKeyAccessDenied(ref cause) => cause,
            CreateBranchError::EncryptionKeyDisabled(ref cause) => cause,
            CreateBranchError::EncryptionKeyNotFound(ref cause) => cause,
            CreateBranchError::EncryptionKeyUnavailable(ref cause) => cause,
            CreateBranchError::InvalidBranchName(ref cause) => cause,
            CreateBranchError::InvalidCommitId(ref cause) => cause,
            CreateBranchError::InvalidRepositoryName(ref cause) => cause,
            CreateBranchError::RepositoryDoesNotExist(ref cause) => cause,
            CreateBranchError::RepositoryNameRequired(ref cause) => cause,
            CreateBranchError::Validation(ref cause) => cause,
            CreateBranchError::Credentials(ref err) => err.description(),
            CreateBranchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBranchError::ParseError(ref cause) => cause,
            CreateBranchError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePullRequest
#[derive(Debug, PartialEq)]
pub enum CreatePullRequestError {
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    ClientRequestTokenRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be re-used.</p>
    IdempotencyParameterMismatch(String),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestToken(String),
    /// <p>The pull request description is not valid. Descriptions are limited to 1,000 characters in length.</p>
    InvalidDescription(String),
    /// <p>The specified reference name format is not valid. Reference names must conform to the Git references format, for example refs/heads/master. For more information, see <a href="https://git-scm.com/book/en/v2/Git-Internals-Git-References">Git Internals - Git References</a> or consult your Git documentation.</p>
    InvalidReferenceName(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The target for the pull request is not valid. A target must contain the full values for the repository name, source branch, and destination branch for the pull request.</p>
    InvalidTarget(String),
    /// <p>The targets for the pull request is not valid or not in a valid format. Targets are a list of target objects. Each target object must contain the full values for the repository name, source branch, and destination branch for a pull request.</p>
    InvalidTargets(String),
    /// <p>The title of the pull request is not valid. Pull request titles cannot exceed 100 characters in length.</p>
    InvalidTitle(String),
    /// <p>You cannot create the pull request because the repository has too many open pull requests. The maximum number of open pull requests for a repository is 1,000. Close one or more open pull requests, and then try again.</p>
    MaximumOpenPullRequestsExceeded(String),
    /// <p>You cannot include more than one repository in a pull request. Make sure you have specified only one repository name in your request, and then try again.</p>
    MultipleRepositoriesInPullRequest(String),
    /// <p>The specified reference does not exist. You must provide a full commit ID.</p>
    ReferenceDoesNotExist(String),
    /// <p>A reference name is required, but none was provided.</p>
    ReferenceNameRequired(String),
    /// <p>The specified reference is not a supported type. </p>
    ReferenceTypeNotSupported(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The source branch and the destination branch for the pull request are the same. You must specify different branches for the source and destination.</p>
    SourceAndDestinationAreSame(String),
    /// <p>A pull request target is required. It cannot be empty or null. A pull request target must contain the full values for the repository name, source branch, and destination branch for the pull request.</p>
    TargetRequired(String),
    /// <p>An array of target objects is required. It cannot be empty or null.</p>
    TargetsRequired(String),
    /// <p>A pull request title is required. It cannot be empty or null.</p>
    TitleRequired(String),
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

impl CreatePullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePullRequestError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ClientRequestTokenRequiredException" => {
                    return CreatePullRequestError::ClientRequestTokenRequired(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return CreatePullRequestError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return CreatePullRequestError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return CreatePullRequestError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return CreatePullRequestError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return CreatePullRequestError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "IdempotencyParameterMismatchException" => {
                    return CreatePullRequestError::IdempotencyParameterMismatch(String::from(
                        error_message,
                    ))
                }
                "InvalidClientRequestTokenException" => {
                    return CreatePullRequestError::InvalidClientRequestToken(String::from(
                        error_message,
                    ))
                }
                "InvalidDescriptionException" => {
                    return CreatePullRequestError::InvalidDescription(String::from(error_message))
                }
                "InvalidReferenceNameException" => {
                    return CreatePullRequestError::InvalidReferenceName(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return CreatePullRequestError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "InvalidTargetException" => {
                    return CreatePullRequestError::InvalidTarget(String::from(error_message))
                }
                "InvalidTargetsException" => {
                    return CreatePullRequestError::InvalidTargets(String::from(error_message))
                }
                "InvalidTitleException" => {
                    return CreatePullRequestError::InvalidTitle(String::from(error_message))
                }
                "MaximumOpenPullRequestsExceededException" => {
                    return CreatePullRequestError::MaximumOpenPullRequestsExceeded(String::from(
                        error_message,
                    ))
                }
                "MultipleRepositoriesInPullRequestException" => {
                    return CreatePullRequestError::MultipleRepositoriesInPullRequest(String::from(
                        error_message,
                    ))
                }
                "ReferenceDoesNotExistException" => {
                    return CreatePullRequestError::ReferenceDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ReferenceNameRequiredException" => {
                    return CreatePullRequestError::ReferenceNameRequired(String::from(
                        error_message,
                    ))
                }
                "ReferenceTypeNotSupportedException" => {
                    return CreatePullRequestError::ReferenceTypeNotSupported(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return CreatePullRequestError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return CreatePullRequestError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "SourceAndDestinationAreSameException" => {
                    return CreatePullRequestError::SourceAndDestinationAreSame(String::from(
                        error_message,
                    ))
                }
                "TargetRequiredException" => {
                    return CreatePullRequestError::TargetRequired(String::from(error_message))
                }
                "TargetsRequiredException" => {
                    return CreatePullRequestError::TargetsRequired(String::from(error_message))
                }
                "TitleRequiredException" => {
                    return CreatePullRequestError::TitleRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return CreatePullRequestError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreatePullRequestError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePullRequestError {
    fn from(err: serde_json::error::Error) -> CreatePullRequestError {
        CreatePullRequestError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePullRequestError {
    fn from(err: CredentialsError) -> CreatePullRequestError {
        CreatePullRequestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePullRequestError {
    fn from(err: HttpDispatchError) -> CreatePullRequestError {
        CreatePullRequestError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePullRequestError {
    fn from(err: io::Error) -> CreatePullRequestError {
        CreatePullRequestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePullRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePullRequestError {
    fn description(&self) -> &str {
        match *self {
            CreatePullRequestError::ClientRequestTokenRequired(ref cause) => cause,
            CreatePullRequestError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            CreatePullRequestError::EncryptionKeyAccessDenied(ref cause) => cause,
            CreatePullRequestError::EncryptionKeyDisabled(ref cause) => cause,
            CreatePullRequestError::EncryptionKeyNotFound(ref cause) => cause,
            CreatePullRequestError::EncryptionKeyUnavailable(ref cause) => cause,
            CreatePullRequestError::IdempotencyParameterMismatch(ref cause) => cause,
            CreatePullRequestError::InvalidClientRequestToken(ref cause) => cause,
            CreatePullRequestError::InvalidDescription(ref cause) => cause,
            CreatePullRequestError::InvalidReferenceName(ref cause) => cause,
            CreatePullRequestError::InvalidRepositoryName(ref cause) => cause,
            CreatePullRequestError::InvalidTarget(ref cause) => cause,
            CreatePullRequestError::InvalidTargets(ref cause) => cause,
            CreatePullRequestError::InvalidTitle(ref cause) => cause,
            CreatePullRequestError::MaximumOpenPullRequestsExceeded(ref cause) => cause,
            CreatePullRequestError::MultipleRepositoriesInPullRequest(ref cause) => cause,
            CreatePullRequestError::ReferenceDoesNotExist(ref cause) => cause,
            CreatePullRequestError::ReferenceNameRequired(ref cause) => cause,
            CreatePullRequestError::ReferenceTypeNotSupported(ref cause) => cause,
            CreatePullRequestError::RepositoryDoesNotExist(ref cause) => cause,
            CreatePullRequestError::RepositoryNameRequired(ref cause) => cause,
            CreatePullRequestError::SourceAndDestinationAreSame(ref cause) => cause,
            CreatePullRequestError::TargetRequired(ref cause) => cause,
            CreatePullRequestError::TargetsRequired(ref cause) => cause,
            CreatePullRequestError::TitleRequired(ref cause) => cause,
            CreatePullRequestError::Validation(ref cause) => cause,
            CreatePullRequestError::Credentials(ref err) => err.description(),
            CreatePullRequestError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePullRequestError::ParseError(ref cause) => cause,
            CreatePullRequestError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateRepository
#[derive(Debug, PartialEq)]
pub enum CreateRepositoryError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified repository description is not valid.</p>
    InvalidRepositoryDescription(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>A repository resource limit was exceeded.</p>
    RepositoryLimitExceeded(String),
    /// <p>The specified repository name already exists.</p>
    RepositoryNameExists(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl CreateRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateRepositoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return CreateRepositoryError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return CreateRepositoryError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return CreateRepositoryError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return CreateRepositoryError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return CreateRepositoryError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryDescriptionException" => {
                    return CreateRepositoryError::InvalidRepositoryDescription(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return CreateRepositoryError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryLimitExceededException" => {
                    return CreateRepositoryError::RepositoryLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameExistsException" => {
                    return CreateRepositoryError::RepositoryNameExists(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return CreateRepositoryError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateRepositoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateRepositoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateRepositoryError {
    fn from(err: serde_json::error::Error) -> CreateRepositoryError {
        CreateRepositoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRepositoryError {
    fn from(err: CredentialsError) -> CreateRepositoryError {
        CreateRepositoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRepositoryError {
    fn from(err: HttpDispatchError) -> CreateRepositoryError {
        CreateRepositoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRepositoryError {
    fn from(err: io::Error) -> CreateRepositoryError {
        CreateRepositoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRepositoryError {
    fn description(&self) -> &str {
        match *self {
            CreateRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            CreateRepositoryError::EncryptionKeyAccessDenied(ref cause) => cause,
            CreateRepositoryError::EncryptionKeyDisabled(ref cause) => cause,
            CreateRepositoryError::EncryptionKeyNotFound(ref cause) => cause,
            CreateRepositoryError::EncryptionKeyUnavailable(ref cause) => cause,
            CreateRepositoryError::InvalidRepositoryDescription(ref cause) => cause,
            CreateRepositoryError::InvalidRepositoryName(ref cause) => cause,
            CreateRepositoryError::RepositoryLimitExceeded(ref cause) => cause,
            CreateRepositoryError::RepositoryNameExists(ref cause) => cause,
            CreateRepositoryError::RepositoryNameRequired(ref cause) => cause,
            CreateRepositoryError::Validation(ref cause) => cause,
            CreateRepositoryError::Credentials(ref err) => err.description(),
            CreateRepositoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateRepositoryError::ParseError(ref cause) => cause,
            CreateRepositoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBranch
#[derive(Debug, PartialEq)]
pub enum DeleteBranchError {
    /// <p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    /// <p>The specified branch is the default branch for the repository, and cannot be deleted. To delete this branch, you must first set another branch as the default branch.</p>
    DefaultBranchCannotBeDeleted(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl DeleteBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BranchNameRequiredException" => {
                    return DeleteBranchError::BranchNameRequired(String::from(error_message))
                }
                "DefaultBranchCannotBeDeletedException" => {
                    return DeleteBranchError::DefaultBranchCannotBeDeleted(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return DeleteBranchError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return DeleteBranchError::EncryptionKeyAccessDenied(String::from(error_message))
                }
                "EncryptionKeyDisabledException" => {
                    return DeleteBranchError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return DeleteBranchError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return DeleteBranchError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "InvalidBranchNameException" => {
                    return DeleteBranchError::InvalidBranchName(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return DeleteBranchError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return DeleteBranchError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return DeleteBranchError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteBranchError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBranchError {
    fn from(err: serde_json::error::Error) -> DeleteBranchError {
        DeleteBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBranchError {
    fn from(err: CredentialsError) -> DeleteBranchError {
        DeleteBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBranchError {
    fn from(err: HttpDispatchError) -> DeleteBranchError {
        DeleteBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBranchError {
    fn from(err: io::Error) -> DeleteBranchError {
        DeleteBranchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBranchError {
    fn description(&self) -> &str {
        match *self {
            DeleteBranchError::BranchNameRequired(ref cause) => cause,
            DeleteBranchError::DefaultBranchCannotBeDeleted(ref cause) => cause,
            DeleteBranchError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            DeleteBranchError::EncryptionKeyAccessDenied(ref cause) => cause,
            DeleteBranchError::EncryptionKeyDisabled(ref cause) => cause,
            DeleteBranchError::EncryptionKeyNotFound(ref cause) => cause,
            DeleteBranchError::EncryptionKeyUnavailable(ref cause) => cause,
            DeleteBranchError::InvalidBranchName(ref cause) => cause,
            DeleteBranchError::InvalidRepositoryName(ref cause) => cause,
            DeleteBranchError::RepositoryDoesNotExist(ref cause) => cause,
            DeleteBranchError::RepositoryNameRequired(ref cause) => cause,
            DeleteBranchError::Validation(ref cause) => cause,
            DeleteBranchError::Credentials(ref err) => err.description(),
            DeleteBranchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBranchError::ParseError(ref cause) => cause,
            DeleteBranchError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteCommentContent
#[derive(Debug, PartialEq)]
pub enum DeleteCommentContentError {
    /// <p>This comment has already been deleted. You cannot edit or delete a deleted comment.</p>
    CommentDeleted(String),
    /// <p>No comment exists with the provided ID. Verify that you have provided the correct ID, and then try again.</p>
    CommentDoesNotExist(String),
    /// <p>The comment ID is missing or null. A comment ID is required.</p>
    CommentIdRequired(String),
    /// <p>The comment ID is not in a valid format. Make sure that you have provided the full comment ID.</p>
    InvalidCommentId(String),
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

impl DeleteCommentContentError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteCommentContentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommentDeletedException" => {
                    return DeleteCommentContentError::CommentDeleted(String::from(error_message))
                }
                "CommentDoesNotExistException" => {
                    return DeleteCommentContentError::CommentDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "CommentIdRequiredException" => {
                    return DeleteCommentContentError::CommentIdRequired(String::from(error_message))
                }
                "InvalidCommentIdException" => {
                    return DeleteCommentContentError::InvalidCommentId(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteCommentContentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteCommentContentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteCommentContentError {
    fn from(err: serde_json::error::Error) -> DeleteCommentContentError {
        DeleteCommentContentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCommentContentError {
    fn from(err: CredentialsError) -> DeleteCommentContentError {
        DeleteCommentContentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCommentContentError {
    fn from(err: HttpDispatchError) -> DeleteCommentContentError {
        DeleteCommentContentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCommentContentError {
    fn from(err: io::Error) -> DeleteCommentContentError {
        DeleteCommentContentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCommentContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCommentContentError {
    fn description(&self) -> &str {
        match *self {
            DeleteCommentContentError::CommentDeleted(ref cause) => cause,
            DeleteCommentContentError::CommentDoesNotExist(ref cause) => cause,
            DeleteCommentContentError::CommentIdRequired(ref cause) => cause,
            DeleteCommentContentError::InvalidCommentId(ref cause) => cause,
            DeleteCommentContentError::Validation(ref cause) => cause,
            DeleteCommentContentError::Credentials(ref err) => err.description(),
            DeleteCommentContentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCommentContentError::ParseError(ref cause) => cause,
            DeleteCommentContentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteRepository
#[derive(Debug, PartialEq)]
pub enum DeleteRepositoryError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl DeleteRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteRepositoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return DeleteRepositoryError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return DeleteRepositoryError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return DeleteRepositoryError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return DeleteRepositoryError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return DeleteRepositoryError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return DeleteRepositoryError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return DeleteRepositoryError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteRepositoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteRepositoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteRepositoryError {
    fn from(err: serde_json::error::Error) -> DeleteRepositoryError {
        DeleteRepositoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRepositoryError {
    fn from(err: CredentialsError) -> DeleteRepositoryError {
        DeleteRepositoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRepositoryError {
    fn from(err: HttpDispatchError) -> DeleteRepositoryError {
        DeleteRepositoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRepositoryError {
    fn from(err: io::Error) -> DeleteRepositoryError {
        DeleteRepositoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRepositoryError {
    fn description(&self) -> &str {
        match *self {
            DeleteRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            DeleteRepositoryError::EncryptionKeyAccessDenied(ref cause) => cause,
            DeleteRepositoryError::EncryptionKeyDisabled(ref cause) => cause,
            DeleteRepositoryError::EncryptionKeyNotFound(ref cause) => cause,
            DeleteRepositoryError::EncryptionKeyUnavailable(ref cause) => cause,
            DeleteRepositoryError::InvalidRepositoryName(ref cause) => cause,
            DeleteRepositoryError::RepositoryNameRequired(ref cause) => cause,
            DeleteRepositoryError::Validation(ref cause) => cause,
            DeleteRepositoryError::Credentials(ref err) => err.description(),
            DeleteRepositoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRepositoryError::ParseError(ref cause) => cause,
            DeleteRepositoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribePullRequestEvents
#[derive(Debug, PartialEq)]
pub enum DescribePullRequestEventsError {
    /// <p>The specified Amazon Resource Name (ARN) does not exist in the AWS account.</p>
    ActorDoesNotExist(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The Amazon Resource Name (ARN) is not valid. Make sure that you have provided the full ARN for the user who initiated the change for the pull request, and then try again.</p>
    InvalidActorArn(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
    /// <p>The pull request event type is not valid. </p>
    InvalidPullRequestEventType(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
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

impl DescribePullRequestEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribePullRequestEventsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ActorDoesNotExistException" => {
                    return DescribePullRequestEventsError::ActorDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return DescribePullRequestEventsError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return DescribePullRequestEventsError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return DescribePullRequestEventsError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return DescribePullRequestEventsError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return DescribePullRequestEventsError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidActorArnException" => {
                    return DescribePullRequestEventsError::InvalidActorArn(String::from(
                        error_message,
                    ))
                }
                "InvalidContinuationTokenException" => {
                    return DescribePullRequestEventsError::InvalidContinuationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidMaxResultsException" => {
                    return DescribePullRequestEventsError::InvalidMaxResults(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestEventTypeException" => {
                    return DescribePullRequestEventsError::InvalidPullRequestEventType(
                        String::from(error_message),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return DescribePullRequestEventsError::InvalidPullRequestId(String::from(
                        error_message,
                    ))
                }
                "PullRequestDoesNotExistException" => {
                    return DescribePullRequestEventsError::PullRequestDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PullRequestIdRequiredException" => {
                    return DescribePullRequestEventsError::PullRequestIdRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribePullRequestEventsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribePullRequestEventsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribePullRequestEventsError {
    fn from(err: serde_json::error::Error) -> DescribePullRequestEventsError {
        DescribePullRequestEventsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePullRequestEventsError {
    fn from(err: CredentialsError) -> DescribePullRequestEventsError {
        DescribePullRequestEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePullRequestEventsError {
    fn from(err: HttpDispatchError) -> DescribePullRequestEventsError {
        DescribePullRequestEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePullRequestEventsError {
    fn from(err: io::Error) -> DescribePullRequestEventsError {
        DescribePullRequestEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePullRequestEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePullRequestEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribePullRequestEventsError::ActorDoesNotExist(ref cause) => cause,
            DescribePullRequestEventsError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            DescribePullRequestEventsError::EncryptionKeyAccessDenied(ref cause) => cause,
            DescribePullRequestEventsError::EncryptionKeyDisabled(ref cause) => cause,
            DescribePullRequestEventsError::EncryptionKeyNotFound(ref cause) => cause,
            DescribePullRequestEventsError::EncryptionKeyUnavailable(ref cause) => cause,
            DescribePullRequestEventsError::InvalidActorArn(ref cause) => cause,
            DescribePullRequestEventsError::InvalidContinuationToken(ref cause) => cause,
            DescribePullRequestEventsError::InvalidMaxResults(ref cause) => cause,
            DescribePullRequestEventsError::InvalidPullRequestEventType(ref cause) => cause,
            DescribePullRequestEventsError::InvalidPullRequestId(ref cause) => cause,
            DescribePullRequestEventsError::PullRequestDoesNotExist(ref cause) => cause,
            DescribePullRequestEventsError::PullRequestIdRequired(ref cause) => cause,
            DescribePullRequestEventsError::Validation(ref cause) => cause,
            DescribePullRequestEventsError::Credentials(ref err) => err.description(),
            DescribePullRequestEventsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePullRequestEventsError::ParseError(ref cause) => cause,
            DescribePullRequestEventsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBlob
#[derive(Debug, PartialEq)]
pub enum GetBlobError {
    /// <p>The specified blob does not exist.</p>
    BlobIdDoesNotExist(String),
    /// <p>A blob ID is required but was not specified.</p>
    BlobIdRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified file exceeds the file size limit for AWS CodeCommit. For more information about limits in AWS CodeCommit, see <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    FileTooLarge(String),
    /// <p>The specified blob is not valid.</p>
    InvalidBlobId(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl GetBlobError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBlobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BlobIdDoesNotExistException" => {
                    return GetBlobError::BlobIdDoesNotExist(String::from(error_message))
                }
                "BlobIdRequiredException" => {
                    return GetBlobError::BlobIdRequired(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return GetBlobError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetBlobError::EncryptionKeyAccessDenied(String::from(error_message))
                }
                "EncryptionKeyDisabledException" => {
                    return GetBlobError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetBlobError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetBlobError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "FileTooLargeException" => {
                    return GetBlobError::FileTooLarge(String::from(error_message))
                }
                "InvalidBlobIdException" => {
                    return GetBlobError::InvalidBlobId(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return GetBlobError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return GetBlobError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return GetBlobError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => return GetBlobError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetBlobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBlobError {
    fn from(err: serde_json::error::Error) -> GetBlobError {
        GetBlobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBlobError {
    fn from(err: CredentialsError) -> GetBlobError {
        GetBlobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBlobError {
    fn from(err: HttpDispatchError) -> GetBlobError {
        GetBlobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBlobError {
    fn from(err: io::Error) -> GetBlobError {
        GetBlobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBlobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBlobError {
    fn description(&self) -> &str {
        match *self {
            GetBlobError::BlobIdDoesNotExist(ref cause) => cause,
            GetBlobError::BlobIdRequired(ref cause) => cause,
            GetBlobError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetBlobError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetBlobError::EncryptionKeyDisabled(ref cause) => cause,
            GetBlobError::EncryptionKeyNotFound(ref cause) => cause,
            GetBlobError::EncryptionKeyUnavailable(ref cause) => cause,
            GetBlobError::FileTooLarge(ref cause) => cause,
            GetBlobError::InvalidBlobId(ref cause) => cause,
            GetBlobError::InvalidRepositoryName(ref cause) => cause,
            GetBlobError::RepositoryDoesNotExist(ref cause) => cause,
            GetBlobError::RepositoryNameRequired(ref cause) => cause,
            GetBlobError::Validation(ref cause) => cause,
            GetBlobError::Credentials(ref err) => err.description(),
            GetBlobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBlobError::ParseError(ref cause) => cause,
            GetBlobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBranch
#[derive(Debug, PartialEq)]
pub enum GetBranchError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl GetBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BranchDoesNotExistException" => {
                    return GetBranchError::BranchDoesNotExist(String::from(error_message))
                }
                "BranchNameRequiredException" => {
                    return GetBranchError::BranchNameRequired(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return GetBranchError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetBranchError::EncryptionKeyAccessDenied(String::from(error_message))
                }
                "EncryptionKeyDisabledException" => {
                    return GetBranchError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetBranchError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetBranchError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "InvalidBranchNameException" => {
                    return GetBranchError::InvalidBranchName(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return GetBranchError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return GetBranchError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return GetBranchError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBranchError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBranchError {
    fn from(err: serde_json::error::Error) -> GetBranchError {
        GetBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBranchError {
    fn from(err: CredentialsError) -> GetBranchError {
        GetBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBranchError {
    fn from(err: HttpDispatchError) -> GetBranchError {
        GetBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBranchError {
    fn from(err: io::Error) -> GetBranchError {
        GetBranchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBranchError {
    fn description(&self) -> &str {
        match *self {
            GetBranchError::BranchDoesNotExist(ref cause) => cause,
            GetBranchError::BranchNameRequired(ref cause) => cause,
            GetBranchError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetBranchError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetBranchError::EncryptionKeyDisabled(ref cause) => cause,
            GetBranchError::EncryptionKeyNotFound(ref cause) => cause,
            GetBranchError::EncryptionKeyUnavailable(ref cause) => cause,
            GetBranchError::InvalidBranchName(ref cause) => cause,
            GetBranchError::InvalidRepositoryName(ref cause) => cause,
            GetBranchError::RepositoryDoesNotExist(ref cause) => cause,
            GetBranchError::RepositoryNameRequired(ref cause) => cause,
            GetBranchError::Validation(ref cause) => cause,
            GetBranchError::Credentials(ref err) => err.description(),
            GetBranchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBranchError::ParseError(ref cause) => cause,
            GetBranchError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetComment
#[derive(Debug, PartialEq)]
pub enum GetCommentError {
    /// <p>This comment has already been deleted. You cannot edit or delete a deleted comment.</p>
    CommentDeleted(String),
    /// <p>No comment exists with the provided ID. Verify that you have provided the correct ID, and then try again.</p>
    CommentDoesNotExist(String),
    /// <p>The comment ID is missing or null. A comment ID is required.</p>
    CommentIdRequired(String),
    /// <p>The comment ID is not in a valid format. Make sure that you have provided the full comment ID.</p>
    InvalidCommentId(String),
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

impl GetCommentError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCommentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommentDeletedException" => {
                    return GetCommentError::CommentDeleted(String::from(error_message))
                }
                "CommentDoesNotExistException" => {
                    return GetCommentError::CommentDoesNotExist(String::from(error_message))
                }
                "CommentIdRequiredException" => {
                    return GetCommentError::CommentIdRequired(String::from(error_message))
                }
                "InvalidCommentIdException" => {
                    return GetCommentError::InvalidCommentId(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCommentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCommentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCommentError {
    fn from(err: serde_json::error::Error) -> GetCommentError {
        GetCommentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCommentError {
    fn from(err: CredentialsError) -> GetCommentError {
        GetCommentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCommentError {
    fn from(err: HttpDispatchError) -> GetCommentError {
        GetCommentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCommentError {
    fn from(err: io::Error) -> GetCommentError {
        GetCommentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCommentError {
    fn description(&self) -> &str {
        match *self {
            GetCommentError::CommentDeleted(ref cause) => cause,
            GetCommentError::CommentDoesNotExist(ref cause) => cause,
            GetCommentError::CommentIdRequired(ref cause) => cause,
            GetCommentError::InvalidCommentId(ref cause) => cause,
            GetCommentError::Validation(ref cause) => cause,
            GetCommentError::Credentials(ref err) => err.description(),
            GetCommentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCommentError::ParseError(ref cause) => cause,
            GetCommentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCommentsForComparedCommit
#[derive(Debug, PartialEq)]
pub enum GetCommentsForComparedCommitError {
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl GetCommentsForComparedCommitError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCommentsForComparedCommitError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommitDoesNotExistException" => {
                    return GetCommentsForComparedCommitError::CommitDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "CommitIdRequiredException" => {
                    return GetCommentsForComparedCommitError::CommitIdRequired(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return GetCommentsForComparedCommitError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetCommentsForComparedCommitError::EncryptionKeyAccessDenied(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return GetCommentsForComparedCommitError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetCommentsForComparedCommitError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetCommentsForComparedCommitError::EncryptionKeyUnavailable(
                        String::from(error_message),
                    )
                }
                "InvalidCommitIdException" => {
                    return GetCommentsForComparedCommitError::InvalidCommitId(String::from(
                        error_message,
                    ))
                }
                "InvalidContinuationTokenException" => {
                    return GetCommentsForComparedCommitError::InvalidContinuationToken(
                        String::from(error_message),
                    )
                }
                "InvalidMaxResultsException" => {
                    return GetCommentsForComparedCommitError::InvalidMaxResults(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return GetCommentsForComparedCommitError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return GetCommentsForComparedCommitError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return GetCommentsForComparedCommitError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetCommentsForComparedCommitError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCommentsForComparedCommitError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCommentsForComparedCommitError {
    fn from(err: serde_json::error::Error) -> GetCommentsForComparedCommitError {
        GetCommentsForComparedCommitError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCommentsForComparedCommitError {
    fn from(err: CredentialsError) -> GetCommentsForComparedCommitError {
        GetCommentsForComparedCommitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCommentsForComparedCommitError {
    fn from(err: HttpDispatchError) -> GetCommentsForComparedCommitError {
        GetCommentsForComparedCommitError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCommentsForComparedCommitError {
    fn from(err: io::Error) -> GetCommentsForComparedCommitError {
        GetCommentsForComparedCommitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCommentsForComparedCommitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCommentsForComparedCommitError {
    fn description(&self) -> &str {
        match *self {
            GetCommentsForComparedCommitError::CommitDoesNotExist(ref cause) => cause,
            GetCommentsForComparedCommitError::CommitIdRequired(ref cause) => cause,
            GetCommentsForComparedCommitError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetCommentsForComparedCommitError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetCommentsForComparedCommitError::EncryptionKeyDisabled(ref cause) => cause,
            GetCommentsForComparedCommitError::EncryptionKeyNotFound(ref cause) => cause,
            GetCommentsForComparedCommitError::EncryptionKeyUnavailable(ref cause) => cause,
            GetCommentsForComparedCommitError::InvalidCommitId(ref cause) => cause,
            GetCommentsForComparedCommitError::InvalidContinuationToken(ref cause) => cause,
            GetCommentsForComparedCommitError::InvalidMaxResults(ref cause) => cause,
            GetCommentsForComparedCommitError::InvalidRepositoryName(ref cause) => cause,
            GetCommentsForComparedCommitError::RepositoryDoesNotExist(ref cause) => cause,
            GetCommentsForComparedCommitError::RepositoryNameRequired(ref cause) => cause,
            GetCommentsForComparedCommitError::Validation(ref cause) => cause,
            GetCommentsForComparedCommitError::Credentials(ref err) => err.description(),
            GetCommentsForComparedCommitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCommentsForComparedCommitError::ParseError(ref cause) => cause,
            GetCommentsForComparedCommitError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCommentsForPullRequest
#[derive(Debug, PartialEq)]
pub enum GetCommentsForPullRequestError {
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The repository does not contain any pull requests with that pull request ID. Check to make sure you have provided the correct repository name for the pull request.</p>
    RepositoryNotAssociatedWithPullRequest(String),
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

impl GetCommentsForPullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCommentsForPullRequestError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommitDoesNotExistException" => {
                    return GetCommentsForPullRequestError::CommitDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "CommitIdRequiredException" => {
                    return GetCommentsForPullRequestError::CommitIdRequired(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return GetCommentsForPullRequestError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetCommentsForPullRequestError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return GetCommentsForPullRequestError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetCommentsForPullRequestError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetCommentsForPullRequestError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidCommitIdException" => {
                    return GetCommentsForPullRequestError::InvalidCommitId(String::from(
                        error_message,
                    ))
                }
                "InvalidContinuationTokenException" => {
                    return GetCommentsForPullRequestError::InvalidContinuationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidMaxResultsException" => {
                    return GetCommentsForPullRequestError::InvalidMaxResults(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return GetCommentsForPullRequestError::InvalidPullRequestId(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return GetCommentsForPullRequestError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "PullRequestDoesNotExistException" => {
                    return GetCommentsForPullRequestError::PullRequestDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PullRequestIdRequiredException" => {
                    return GetCommentsForPullRequestError::PullRequestIdRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return GetCommentsForPullRequestError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return GetCommentsForPullRequestError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryNotAssociatedWithPullRequestException" => {
                    return GetCommentsForPullRequestError::RepositoryNotAssociatedWithPullRequest(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return GetCommentsForPullRequestError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCommentsForPullRequestError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCommentsForPullRequestError {
    fn from(err: serde_json::error::Error) -> GetCommentsForPullRequestError {
        GetCommentsForPullRequestError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCommentsForPullRequestError {
    fn from(err: CredentialsError) -> GetCommentsForPullRequestError {
        GetCommentsForPullRequestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCommentsForPullRequestError {
    fn from(err: HttpDispatchError) -> GetCommentsForPullRequestError {
        GetCommentsForPullRequestError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCommentsForPullRequestError {
    fn from(err: io::Error) -> GetCommentsForPullRequestError {
        GetCommentsForPullRequestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCommentsForPullRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCommentsForPullRequestError {
    fn description(&self) -> &str {
        match *self {
            GetCommentsForPullRequestError::CommitDoesNotExist(ref cause) => cause,
            GetCommentsForPullRequestError::CommitIdRequired(ref cause) => cause,
            GetCommentsForPullRequestError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetCommentsForPullRequestError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetCommentsForPullRequestError::EncryptionKeyDisabled(ref cause) => cause,
            GetCommentsForPullRequestError::EncryptionKeyNotFound(ref cause) => cause,
            GetCommentsForPullRequestError::EncryptionKeyUnavailable(ref cause) => cause,
            GetCommentsForPullRequestError::InvalidCommitId(ref cause) => cause,
            GetCommentsForPullRequestError::InvalidContinuationToken(ref cause) => cause,
            GetCommentsForPullRequestError::InvalidMaxResults(ref cause) => cause,
            GetCommentsForPullRequestError::InvalidPullRequestId(ref cause) => cause,
            GetCommentsForPullRequestError::InvalidRepositoryName(ref cause) => cause,
            GetCommentsForPullRequestError::PullRequestDoesNotExist(ref cause) => cause,
            GetCommentsForPullRequestError::PullRequestIdRequired(ref cause) => cause,
            GetCommentsForPullRequestError::RepositoryDoesNotExist(ref cause) => cause,
            GetCommentsForPullRequestError::RepositoryNameRequired(ref cause) => cause,
            GetCommentsForPullRequestError::RepositoryNotAssociatedWithPullRequest(ref cause) => {
                cause
            }
            GetCommentsForPullRequestError::Validation(ref cause) => cause,
            GetCommentsForPullRequestError::Credentials(ref err) => err.description(),
            GetCommentsForPullRequestError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCommentsForPullRequestError::ParseError(ref cause) => cause,
            GetCommentsForPullRequestError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCommit
#[derive(Debug, PartialEq)]
pub enum GetCommitError {
    /// <p>The specified commit ID does not exist.</p>
    CommitIdDoesNotExist(String),
    /// <p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl GetCommitError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCommitError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommitIdDoesNotExistException" => {
                    return GetCommitError::CommitIdDoesNotExist(String::from(error_message))
                }
                "CommitIdRequiredException" => {
                    return GetCommitError::CommitIdRequired(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return GetCommitError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetCommitError::EncryptionKeyAccessDenied(String::from(error_message))
                }
                "EncryptionKeyDisabledException" => {
                    return GetCommitError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetCommitError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetCommitError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "InvalidCommitIdException" => {
                    return GetCommitError::InvalidCommitId(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return GetCommitError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return GetCommitError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return GetCommitError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCommitError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCommitError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCommitError {
    fn from(err: serde_json::error::Error) -> GetCommitError {
        GetCommitError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCommitError {
    fn from(err: CredentialsError) -> GetCommitError {
        GetCommitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCommitError {
    fn from(err: HttpDispatchError) -> GetCommitError {
        GetCommitError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCommitError {
    fn from(err: io::Error) -> GetCommitError {
        GetCommitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCommitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCommitError {
    fn description(&self) -> &str {
        match *self {
            GetCommitError::CommitIdDoesNotExist(ref cause) => cause,
            GetCommitError::CommitIdRequired(ref cause) => cause,
            GetCommitError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetCommitError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetCommitError::EncryptionKeyDisabled(ref cause) => cause,
            GetCommitError::EncryptionKeyNotFound(ref cause) => cause,
            GetCommitError::EncryptionKeyUnavailable(ref cause) => cause,
            GetCommitError::InvalidCommitId(ref cause) => cause,
            GetCommitError::InvalidRepositoryName(ref cause) => cause,
            GetCommitError::RepositoryDoesNotExist(ref cause) => cause,
            GetCommitError::RepositoryNameRequired(ref cause) => cause,
            GetCommitError::Validation(ref cause) => cause,
            GetCommitError::Credentials(ref err) => err.description(),
            GetCommitError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCommitError::ParseError(ref cause) => cause,
            GetCommitError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDifferences
#[derive(Debug, PartialEq)]
pub enum GetDifferencesError {
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit was not specified.</p>
    CommitRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified path does not exist.</p>
    PathDoesNotExist(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl GetDifferencesError {
    pub fn from_response(res: BufferedHttpResponse) -> GetDifferencesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommitDoesNotExistException" => {
                    return GetDifferencesError::CommitDoesNotExist(String::from(error_message))
                }
                "CommitRequiredException" => {
                    return GetDifferencesError::CommitRequired(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return GetDifferencesError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetDifferencesError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return GetDifferencesError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetDifferencesError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetDifferencesError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidCommitException" => {
                    return GetDifferencesError::InvalidCommit(String::from(error_message))
                }
                "InvalidCommitIdException" => {
                    return GetDifferencesError::InvalidCommitId(String::from(error_message))
                }
                "InvalidContinuationTokenException" => {
                    return GetDifferencesError::InvalidContinuationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidMaxResultsException" => {
                    return GetDifferencesError::InvalidMaxResults(String::from(error_message))
                }
                "InvalidPathException" => {
                    return GetDifferencesError::InvalidPath(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return GetDifferencesError::InvalidRepositoryName(String::from(error_message))
                }
                "PathDoesNotExistException" => {
                    return GetDifferencesError::PathDoesNotExist(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return GetDifferencesError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return GetDifferencesError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return GetDifferencesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetDifferencesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDifferencesError {
    fn from(err: serde_json::error::Error) -> GetDifferencesError {
        GetDifferencesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDifferencesError {
    fn from(err: CredentialsError) -> GetDifferencesError {
        GetDifferencesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDifferencesError {
    fn from(err: HttpDispatchError) -> GetDifferencesError {
        GetDifferencesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDifferencesError {
    fn from(err: io::Error) -> GetDifferencesError {
        GetDifferencesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDifferencesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDifferencesError {
    fn description(&self) -> &str {
        match *self {
            GetDifferencesError::CommitDoesNotExist(ref cause) => cause,
            GetDifferencesError::CommitRequired(ref cause) => cause,
            GetDifferencesError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetDifferencesError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetDifferencesError::EncryptionKeyDisabled(ref cause) => cause,
            GetDifferencesError::EncryptionKeyNotFound(ref cause) => cause,
            GetDifferencesError::EncryptionKeyUnavailable(ref cause) => cause,
            GetDifferencesError::InvalidCommit(ref cause) => cause,
            GetDifferencesError::InvalidCommitId(ref cause) => cause,
            GetDifferencesError::InvalidContinuationToken(ref cause) => cause,
            GetDifferencesError::InvalidMaxResults(ref cause) => cause,
            GetDifferencesError::InvalidPath(ref cause) => cause,
            GetDifferencesError::InvalidRepositoryName(ref cause) => cause,
            GetDifferencesError::PathDoesNotExist(ref cause) => cause,
            GetDifferencesError::RepositoryDoesNotExist(ref cause) => cause,
            GetDifferencesError::RepositoryNameRequired(ref cause) => cause,
            GetDifferencesError::Validation(ref cause) => cause,
            GetDifferencesError::Credentials(ref err) => err.description(),
            GetDifferencesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDifferencesError::ParseError(ref cause) => cause,
            GetDifferencesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetMergeConflicts
#[derive(Debug, PartialEq)]
pub enum GetMergeConflictsError {
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit was not specified.</p>
    CommitRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The destination commit specifier is not valid. You must provide a valid branch name, tag, or full commit ID. </p>
    InvalidDestinationCommitSpecifier(String),
    /// <p>The specified merge option is not valid. The only valid value is FAST_FORWARD_MERGE.</p>
    InvalidMergeOption(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The source commit specifier is not valid. You must provide a valid branch name, tag, or full commit ID.</p>
    InvalidSourceCommitSpecifier(String),
    /// <p>A merge option or stategy is required, and none was provided.</p>
    MergeOptionRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
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

impl GetMergeConflictsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetMergeConflictsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommitDoesNotExistException" => {
                    return GetMergeConflictsError::CommitDoesNotExist(String::from(error_message))
                }
                "CommitRequiredException" => {
                    return GetMergeConflictsError::CommitRequired(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return GetMergeConflictsError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetMergeConflictsError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return GetMergeConflictsError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetMergeConflictsError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetMergeConflictsError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidCommitException" => {
                    return GetMergeConflictsError::InvalidCommit(String::from(error_message))
                }
                "InvalidDestinationCommitSpecifierException" => {
                    return GetMergeConflictsError::InvalidDestinationCommitSpecifier(String::from(
                        error_message,
                    ))
                }
                "InvalidMergeOptionException" => {
                    return GetMergeConflictsError::InvalidMergeOption(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return GetMergeConflictsError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "InvalidSourceCommitSpecifierException" => {
                    return GetMergeConflictsError::InvalidSourceCommitSpecifier(String::from(
                        error_message,
                    ))
                }
                "MergeOptionRequiredException" => {
                    return GetMergeConflictsError::MergeOptionRequired(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return GetMergeConflictsError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return GetMergeConflictsError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "TipsDivergenceExceededException" => {
                    return GetMergeConflictsError::TipsDivergenceExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetMergeConflictsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetMergeConflictsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetMergeConflictsError {
    fn from(err: serde_json::error::Error) -> GetMergeConflictsError {
        GetMergeConflictsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMergeConflictsError {
    fn from(err: CredentialsError) -> GetMergeConflictsError {
        GetMergeConflictsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMergeConflictsError {
    fn from(err: HttpDispatchError) -> GetMergeConflictsError {
        GetMergeConflictsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMergeConflictsError {
    fn from(err: io::Error) -> GetMergeConflictsError {
        GetMergeConflictsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMergeConflictsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMergeConflictsError {
    fn description(&self) -> &str {
        match *self {
            GetMergeConflictsError::CommitDoesNotExist(ref cause) => cause,
            GetMergeConflictsError::CommitRequired(ref cause) => cause,
            GetMergeConflictsError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetMergeConflictsError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetMergeConflictsError::EncryptionKeyDisabled(ref cause) => cause,
            GetMergeConflictsError::EncryptionKeyNotFound(ref cause) => cause,
            GetMergeConflictsError::EncryptionKeyUnavailable(ref cause) => cause,
            GetMergeConflictsError::InvalidCommit(ref cause) => cause,
            GetMergeConflictsError::InvalidDestinationCommitSpecifier(ref cause) => cause,
            GetMergeConflictsError::InvalidMergeOption(ref cause) => cause,
            GetMergeConflictsError::InvalidRepositoryName(ref cause) => cause,
            GetMergeConflictsError::InvalidSourceCommitSpecifier(ref cause) => cause,
            GetMergeConflictsError::MergeOptionRequired(ref cause) => cause,
            GetMergeConflictsError::RepositoryDoesNotExist(ref cause) => cause,
            GetMergeConflictsError::RepositoryNameRequired(ref cause) => cause,
            GetMergeConflictsError::TipsDivergenceExceeded(ref cause) => cause,
            GetMergeConflictsError::Validation(ref cause) => cause,
            GetMergeConflictsError::Credentials(ref err) => err.description(),
            GetMergeConflictsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMergeConflictsError::ParseError(ref cause) => cause,
            GetMergeConflictsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetPullRequest
#[derive(Debug, PartialEq)]
pub enum GetPullRequestError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
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

impl GetPullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> GetPullRequestError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return GetPullRequestError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetPullRequestError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return GetPullRequestError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetPullRequestError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetPullRequestError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return GetPullRequestError::InvalidPullRequestId(String::from(error_message))
                }
                "PullRequestDoesNotExistException" => {
                    return GetPullRequestError::PullRequestDoesNotExist(String::from(error_message))
                }
                "PullRequestIdRequiredException" => {
                    return GetPullRequestError::PullRequestIdRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return GetPullRequestError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetPullRequestError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPullRequestError {
    fn from(err: serde_json::error::Error) -> GetPullRequestError {
        GetPullRequestError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPullRequestError {
    fn from(err: CredentialsError) -> GetPullRequestError {
        GetPullRequestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPullRequestError {
    fn from(err: HttpDispatchError) -> GetPullRequestError {
        GetPullRequestError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPullRequestError {
    fn from(err: io::Error) -> GetPullRequestError {
        GetPullRequestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPullRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPullRequestError {
    fn description(&self) -> &str {
        match *self {
            GetPullRequestError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetPullRequestError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetPullRequestError::EncryptionKeyDisabled(ref cause) => cause,
            GetPullRequestError::EncryptionKeyNotFound(ref cause) => cause,
            GetPullRequestError::EncryptionKeyUnavailable(ref cause) => cause,
            GetPullRequestError::InvalidPullRequestId(ref cause) => cause,
            GetPullRequestError::PullRequestDoesNotExist(ref cause) => cause,
            GetPullRequestError::PullRequestIdRequired(ref cause) => cause,
            GetPullRequestError::Validation(ref cause) => cause,
            GetPullRequestError::Credentials(ref err) => err.description(),
            GetPullRequestError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPullRequestError::ParseError(ref cause) => cause,
            GetPullRequestError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetRepository
#[derive(Debug, PartialEq)]
pub enum GetRepositoryError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl GetRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> GetRepositoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return GetRepositoryError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetRepositoryError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return GetRepositoryError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetRepositoryError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetRepositoryError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return GetRepositoryError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return GetRepositoryError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return GetRepositoryError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return GetRepositoryError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetRepositoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetRepositoryError {
    fn from(err: serde_json::error::Error) -> GetRepositoryError {
        GetRepositoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRepositoryError {
    fn from(err: CredentialsError) -> GetRepositoryError {
        GetRepositoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRepositoryError {
    fn from(err: HttpDispatchError) -> GetRepositoryError {
        GetRepositoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRepositoryError {
    fn from(err: io::Error) -> GetRepositoryError {
        GetRepositoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRepositoryError {
    fn description(&self) -> &str {
        match *self {
            GetRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetRepositoryError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetRepositoryError::EncryptionKeyDisabled(ref cause) => cause,
            GetRepositoryError::EncryptionKeyNotFound(ref cause) => cause,
            GetRepositoryError::EncryptionKeyUnavailable(ref cause) => cause,
            GetRepositoryError::InvalidRepositoryName(ref cause) => cause,
            GetRepositoryError::RepositoryDoesNotExist(ref cause) => cause,
            GetRepositoryError::RepositoryNameRequired(ref cause) => cause,
            GetRepositoryError::Validation(ref cause) => cause,
            GetRepositoryError::Credentials(ref err) => err.description(),
            GetRepositoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRepositoryError::ParseError(ref cause) => cause,
            GetRepositoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetRepositoryTriggers
#[derive(Debug, PartialEq)]
pub enum GetRepositoryTriggersError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl GetRepositoryTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> GetRepositoryTriggersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return GetRepositoryTriggersError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return GetRepositoryTriggersError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return GetRepositoryTriggersError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return GetRepositoryTriggersError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return GetRepositoryTriggersError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return GetRepositoryTriggersError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return GetRepositoryTriggersError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return GetRepositoryTriggersError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetRepositoryTriggersError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetRepositoryTriggersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetRepositoryTriggersError {
    fn from(err: serde_json::error::Error) -> GetRepositoryTriggersError {
        GetRepositoryTriggersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRepositoryTriggersError {
    fn from(err: CredentialsError) -> GetRepositoryTriggersError {
        GetRepositoryTriggersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRepositoryTriggersError {
    fn from(err: HttpDispatchError) -> GetRepositoryTriggersError {
        GetRepositoryTriggersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRepositoryTriggersError {
    fn from(err: io::Error) -> GetRepositoryTriggersError {
        GetRepositoryTriggersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRepositoryTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRepositoryTriggersError {
    fn description(&self) -> &str {
        match *self {
            GetRepositoryTriggersError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetRepositoryTriggersError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetRepositoryTriggersError::EncryptionKeyDisabled(ref cause) => cause,
            GetRepositoryTriggersError::EncryptionKeyNotFound(ref cause) => cause,
            GetRepositoryTriggersError::EncryptionKeyUnavailable(ref cause) => cause,
            GetRepositoryTriggersError::InvalidRepositoryName(ref cause) => cause,
            GetRepositoryTriggersError::RepositoryDoesNotExist(ref cause) => cause,
            GetRepositoryTriggersError::RepositoryNameRequired(ref cause) => cause,
            GetRepositoryTriggersError::Validation(ref cause) => cause,
            GetRepositoryTriggersError::Credentials(ref err) => err.description(),
            GetRepositoryTriggersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRepositoryTriggersError::ParseError(ref cause) => cause,
            GetRepositoryTriggersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListBranches
#[derive(Debug, PartialEq)]
pub enum ListBranchesError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl ListBranchesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListBranchesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return ListBranchesError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return ListBranchesError::EncryptionKeyAccessDenied(String::from(error_message))
                }
                "EncryptionKeyDisabledException" => {
                    return ListBranchesError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return ListBranchesError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return ListBranchesError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "InvalidContinuationTokenException" => {
                    return ListBranchesError::InvalidContinuationToken(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return ListBranchesError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return ListBranchesError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return ListBranchesError::RepositoryNameRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return ListBranchesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListBranchesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListBranchesError {
    fn from(err: serde_json::error::Error) -> ListBranchesError {
        ListBranchesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBranchesError {
    fn from(err: CredentialsError) -> ListBranchesError {
        ListBranchesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBranchesError {
    fn from(err: HttpDispatchError) -> ListBranchesError {
        ListBranchesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBranchesError {
    fn from(err: io::Error) -> ListBranchesError {
        ListBranchesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBranchesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBranchesError {
    fn description(&self) -> &str {
        match *self {
            ListBranchesError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            ListBranchesError::EncryptionKeyAccessDenied(ref cause) => cause,
            ListBranchesError::EncryptionKeyDisabled(ref cause) => cause,
            ListBranchesError::EncryptionKeyNotFound(ref cause) => cause,
            ListBranchesError::EncryptionKeyUnavailable(ref cause) => cause,
            ListBranchesError::InvalidContinuationToken(ref cause) => cause,
            ListBranchesError::InvalidRepositoryName(ref cause) => cause,
            ListBranchesError::RepositoryDoesNotExist(ref cause) => cause,
            ListBranchesError::RepositoryNameRequired(ref cause) => cause,
            ListBranchesError::Validation(ref cause) => cause,
            ListBranchesError::Credentials(ref err) => err.description(),
            ListBranchesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBranchesError::ParseError(ref cause) => cause,
            ListBranchesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListPullRequests
#[derive(Debug, PartialEq)]
pub enum ListPullRequestsError {
    /// <p>The specified Amazon Resource Name (ARN) does not exist in the AWS account.</p>
    AuthorDoesNotExist(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The Amazon Resource Name (ARN) is not valid. Make sure that you have provided the full ARN for the author of the pull request, and then try again.</p>
    InvalidAuthorArn(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
    /// <p>The pull request status is not valid. The only valid values are <code>OPEN</code> and <code>CLOSED</code>.</p>
    InvalidPullRequestStatus(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl ListPullRequestsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListPullRequestsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AuthorDoesNotExistException" => {
                    return ListPullRequestsError::AuthorDoesNotExist(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return ListPullRequestsError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return ListPullRequestsError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return ListPullRequestsError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return ListPullRequestsError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return ListPullRequestsError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidAuthorArnException" => {
                    return ListPullRequestsError::InvalidAuthorArn(String::from(error_message))
                }
                "InvalidContinuationTokenException" => {
                    return ListPullRequestsError::InvalidContinuationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidMaxResultsException" => {
                    return ListPullRequestsError::InvalidMaxResults(String::from(error_message))
                }
                "InvalidPullRequestStatusException" => {
                    return ListPullRequestsError::InvalidPullRequestStatus(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return ListPullRequestsError::InvalidRepositoryName(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return ListPullRequestsError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return ListPullRequestsError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListPullRequestsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListPullRequestsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPullRequestsError {
    fn from(err: serde_json::error::Error) -> ListPullRequestsError {
        ListPullRequestsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPullRequestsError {
    fn from(err: CredentialsError) -> ListPullRequestsError {
        ListPullRequestsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPullRequestsError {
    fn from(err: HttpDispatchError) -> ListPullRequestsError {
        ListPullRequestsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPullRequestsError {
    fn from(err: io::Error) -> ListPullRequestsError {
        ListPullRequestsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPullRequestsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPullRequestsError {
    fn description(&self) -> &str {
        match *self {
            ListPullRequestsError::AuthorDoesNotExist(ref cause) => cause,
            ListPullRequestsError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            ListPullRequestsError::EncryptionKeyAccessDenied(ref cause) => cause,
            ListPullRequestsError::EncryptionKeyDisabled(ref cause) => cause,
            ListPullRequestsError::EncryptionKeyNotFound(ref cause) => cause,
            ListPullRequestsError::EncryptionKeyUnavailable(ref cause) => cause,
            ListPullRequestsError::InvalidAuthorArn(ref cause) => cause,
            ListPullRequestsError::InvalidContinuationToken(ref cause) => cause,
            ListPullRequestsError::InvalidMaxResults(ref cause) => cause,
            ListPullRequestsError::InvalidPullRequestStatus(ref cause) => cause,
            ListPullRequestsError::InvalidRepositoryName(ref cause) => cause,
            ListPullRequestsError::RepositoryDoesNotExist(ref cause) => cause,
            ListPullRequestsError::RepositoryNameRequired(ref cause) => cause,
            ListPullRequestsError::Validation(ref cause) => cause,
            ListPullRequestsError::Credentials(ref err) => err.description(),
            ListPullRequestsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPullRequestsError::ParseError(ref cause) => cause,
            ListPullRequestsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListRepositories
#[derive(Debug, PartialEq)]
pub enum ListRepositoriesError {
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified sort order is not valid.</p>
    InvalidOrder(String),
    /// <p>The specified sort by value is not valid.</p>
    InvalidSortBy(String),
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

impl ListRepositoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListRepositoriesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidContinuationTokenException" => {
                    return ListRepositoriesError::InvalidContinuationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidOrderException" => {
                    return ListRepositoriesError::InvalidOrder(String::from(error_message))
                }
                "InvalidSortByException" => {
                    return ListRepositoriesError::InvalidSortBy(String::from(error_message))
                }
                "ValidationException" => {
                    return ListRepositoriesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListRepositoriesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListRepositoriesError {
    fn from(err: serde_json::error::Error) -> ListRepositoriesError {
        ListRepositoriesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRepositoriesError {
    fn from(err: CredentialsError) -> ListRepositoriesError {
        ListRepositoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRepositoriesError {
    fn from(err: HttpDispatchError) -> ListRepositoriesError {
        ListRepositoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRepositoriesError {
    fn from(err: io::Error) -> ListRepositoriesError {
        ListRepositoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRepositoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRepositoriesError {
    fn description(&self) -> &str {
        match *self {
            ListRepositoriesError::InvalidContinuationToken(ref cause) => cause,
            ListRepositoriesError::InvalidOrder(ref cause) => cause,
            ListRepositoriesError::InvalidSortBy(ref cause) => cause,
            ListRepositoriesError::Validation(ref cause) => cause,
            ListRepositoriesError::Credentials(ref err) => err.description(),
            ListRepositoriesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRepositoriesError::ParseError(ref cause) => cause,
            ListRepositoriesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by MergePullRequestByFastForward
#[derive(Debug, PartialEq)]
pub enum MergePullRequestByFastForwardError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>The specified reference does not exist. You must provide a full commit ID.</p>
    ReferenceDoesNotExist(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The tip of the source branch in the destination repository does not match the tip of the source branch specified in your request. The pull request might have been updated. Make sure that you have the latest changes.</p>
    TipOfSourceReferenceIsDifferent(String),
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

impl MergePullRequestByFastForwardError {
    pub fn from_response(res: BufferedHttpResponse) -> MergePullRequestByFastForwardError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return MergePullRequestByFastForwardError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return MergePullRequestByFastForwardError::EncryptionKeyAccessDenied(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return MergePullRequestByFastForwardError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return MergePullRequestByFastForwardError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return MergePullRequestByFastForwardError::EncryptionKeyUnavailable(
                        String::from(error_message),
                    )
                }
                "InvalidCommitIdException" => {
                    return MergePullRequestByFastForwardError::InvalidCommitId(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return MergePullRequestByFastForwardError::InvalidPullRequestId(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return MergePullRequestByFastForwardError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "ManualMergeRequiredException" => {
                    return MergePullRequestByFastForwardError::ManualMergeRequired(String::from(
                        error_message,
                    ))
                }
                "PullRequestAlreadyClosedException" => {
                    return MergePullRequestByFastForwardError::PullRequestAlreadyClosed(
                        String::from(error_message),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return MergePullRequestByFastForwardError::PullRequestDoesNotExist(
                        String::from(error_message),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return MergePullRequestByFastForwardError::PullRequestIdRequired(String::from(
                        error_message,
                    ))
                }
                "ReferenceDoesNotExistException" => {
                    return MergePullRequestByFastForwardError::ReferenceDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return MergePullRequestByFastForwardError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return MergePullRequestByFastForwardError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "TipOfSourceReferenceIsDifferentException" => {
                    return MergePullRequestByFastForwardError::TipOfSourceReferenceIsDifferent(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return MergePullRequestByFastForwardError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return MergePullRequestByFastForwardError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for MergePullRequestByFastForwardError {
    fn from(err: serde_json::error::Error) -> MergePullRequestByFastForwardError {
        MergePullRequestByFastForwardError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for MergePullRequestByFastForwardError {
    fn from(err: CredentialsError) -> MergePullRequestByFastForwardError {
        MergePullRequestByFastForwardError::Credentials(err)
    }
}
impl From<HttpDispatchError> for MergePullRequestByFastForwardError {
    fn from(err: HttpDispatchError) -> MergePullRequestByFastForwardError {
        MergePullRequestByFastForwardError::HttpDispatch(err)
    }
}
impl From<io::Error> for MergePullRequestByFastForwardError {
    fn from(err: io::Error) -> MergePullRequestByFastForwardError {
        MergePullRequestByFastForwardError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for MergePullRequestByFastForwardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for MergePullRequestByFastForwardError {
    fn description(&self) -> &str {
        match *self {
            MergePullRequestByFastForwardError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            MergePullRequestByFastForwardError::EncryptionKeyAccessDenied(ref cause) => cause,
            MergePullRequestByFastForwardError::EncryptionKeyDisabled(ref cause) => cause,
            MergePullRequestByFastForwardError::EncryptionKeyNotFound(ref cause) => cause,
            MergePullRequestByFastForwardError::EncryptionKeyUnavailable(ref cause) => cause,
            MergePullRequestByFastForwardError::InvalidCommitId(ref cause) => cause,
            MergePullRequestByFastForwardError::InvalidPullRequestId(ref cause) => cause,
            MergePullRequestByFastForwardError::InvalidRepositoryName(ref cause) => cause,
            MergePullRequestByFastForwardError::ManualMergeRequired(ref cause) => cause,
            MergePullRequestByFastForwardError::PullRequestAlreadyClosed(ref cause) => cause,
            MergePullRequestByFastForwardError::PullRequestDoesNotExist(ref cause) => cause,
            MergePullRequestByFastForwardError::PullRequestIdRequired(ref cause) => cause,
            MergePullRequestByFastForwardError::ReferenceDoesNotExist(ref cause) => cause,
            MergePullRequestByFastForwardError::RepositoryDoesNotExist(ref cause) => cause,
            MergePullRequestByFastForwardError::RepositoryNameRequired(ref cause) => cause,
            MergePullRequestByFastForwardError::TipOfSourceReferenceIsDifferent(ref cause) => cause,
            MergePullRequestByFastForwardError::Validation(ref cause) => cause,
            MergePullRequestByFastForwardError::Credentials(ref err) => err.description(),
            MergePullRequestByFastForwardError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            MergePullRequestByFastForwardError::ParseError(ref cause) => cause,
            MergePullRequestByFastForwardError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PostCommentForComparedCommit
#[derive(Debug, PartialEq)]
pub enum PostCommentForComparedCommitError {
    /// <p>The before commit ID and the after commit ID are the same, which is not valid. The before commit ID and the after commit ID must be different commit IDs.</p>
    BeforeCommitIdAndAfterCommitIdAreSame(String),
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    ClientRequestTokenRequired(String),
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequired(String),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceeded(String),
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be re-used.</p>
    IdempotencyParameterMismatch(String),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestToken(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The location of the file is not valid. Make sure that you include the extension of the file as well as the file name.</p>
    InvalidFileLocation(String),
    /// <p>The position is not valid. Make sure that the line number exists in the version of the file you want to comment on.</p>
    InvalidFilePosition(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>Either the enum is not in a valid format, or the specified file version enum is not valid in respect to the current file version.</p>
    InvalidRelativeFileVersionEnum(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified path does not exist.</p>
    PathDoesNotExist(String),
    /// <p>The filePath for a location cannot be empty or null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl PostCommentForComparedCommitError {
    pub fn from_response(res: BufferedHttpResponse) -> PostCommentForComparedCommitError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BeforeCommitIdAndAfterCommitIdAreSameException" => {
                    return PostCommentForComparedCommitError::BeforeCommitIdAndAfterCommitIdAreSame(
                        String::from(error_message),
                    )
                }
                "ClientRequestTokenRequiredException" => {
                    return PostCommentForComparedCommitError::ClientRequestTokenRequired(
                        String::from(error_message),
                    )
                }
                "CommentContentRequiredException" => {
                    return PostCommentForComparedCommitError::CommentContentRequired(String::from(
                        error_message,
                    ))
                }
                "CommentContentSizeLimitExceededException" => {
                    return PostCommentForComparedCommitError::CommentContentSizeLimitExceeded(
                        String::from(error_message),
                    )
                }
                "CommitDoesNotExistException" => {
                    return PostCommentForComparedCommitError::CommitDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "CommitIdRequiredException" => {
                    return PostCommentForComparedCommitError::CommitIdRequired(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return PostCommentForComparedCommitError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return PostCommentForComparedCommitError::EncryptionKeyAccessDenied(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return PostCommentForComparedCommitError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return PostCommentForComparedCommitError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return PostCommentForComparedCommitError::EncryptionKeyUnavailable(
                        String::from(error_message),
                    )
                }
                "IdempotencyParameterMismatchException" => {
                    return PostCommentForComparedCommitError::IdempotencyParameterMismatch(
                        String::from(error_message),
                    )
                }
                "InvalidClientRequestTokenException" => {
                    return PostCommentForComparedCommitError::InvalidClientRequestToken(
                        String::from(error_message),
                    )
                }
                "InvalidCommitIdException" => {
                    return PostCommentForComparedCommitError::InvalidCommitId(String::from(
                        error_message,
                    ))
                }
                "InvalidFileLocationException" => {
                    return PostCommentForComparedCommitError::InvalidFileLocation(String::from(
                        error_message,
                    ))
                }
                "InvalidFilePositionException" => {
                    return PostCommentForComparedCommitError::InvalidFilePosition(String::from(
                        error_message,
                    ))
                }
                "InvalidPathException" => {
                    return PostCommentForComparedCommitError::InvalidPath(String::from(
                        error_message,
                    ))
                }
                "InvalidRelativeFileVersionEnumException" => {
                    return PostCommentForComparedCommitError::InvalidRelativeFileVersionEnum(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return PostCommentForComparedCommitError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "PathDoesNotExistException" => {
                    return PostCommentForComparedCommitError::PathDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PathRequiredException" => {
                    return PostCommentForComparedCommitError::PathRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return PostCommentForComparedCommitError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return PostCommentForComparedCommitError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return PostCommentForComparedCommitError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PostCommentForComparedCommitError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PostCommentForComparedCommitError {
    fn from(err: serde_json::error::Error) -> PostCommentForComparedCommitError {
        PostCommentForComparedCommitError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PostCommentForComparedCommitError {
    fn from(err: CredentialsError) -> PostCommentForComparedCommitError {
        PostCommentForComparedCommitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PostCommentForComparedCommitError {
    fn from(err: HttpDispatchError) -> PostCommentForComparedCommitError {
        PostCommentForComparedCommitError::HttpDispatch(err)
    }
}
impl From<io::Error> for PostCommentForComparedCommitError {
    fn from(err: io::Error) -> PostCommentForComparedCommitError {
        PostCommentForComparedCommitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PostCommentForComparedCommitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PostCommentForComparedCommitError {
    fn description(&self) -> &str {
        match *self {
            PostCommentForComparedCommitError::BeforeCommitIdAndAfterCommitIdAreSame(ref cause) => {
                cause
            }
            PostCommentForComparedCommitError::ClientRequestTokenRequired(ref cause) => cause,
            PostCommentForComparedCommitError::CommentContentRequired(ref cause) => cause,
            PostCommentForComparedCommitError::CommentContentSizeLimitExceeded(ref cause) => cause,
            PostCommentForComparedCommitError::CommitDoesNotExist(ref cause) => cause,
            PostCommentForComparedCommitError::CommitIdRequired(ref cause) => cause,
            PostCommentForComparedCommitError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            PostCommentForComparedCommitError::EncryptionKeyAccessDenied(ref cause) => cause,
            PostCommentForComparedCommitError::EncryptionKeyDisabled(ref cause) => cause,
            PostCommentForComparedCommitError::EncryptionKeyNotFound(ref cause) => cause,
            PostCommentForComparedCommitError::EncryptionKeyUnavailable(ref cause) => cause,
            PostCommentForComparedCommitError::IdempotencyParameterMismatch(ref cause) => cause,
            PostCommentForComparedCommitError::InvalidClientRequestToken(ref cause) => cause,
            PostCommentForComparedCommitError::InvalidCommitId(ref cause) => cause,
            PostCommentForComparedCommitError::InvalidFileLocation(ref cause) => cause,
            PostCommentForComparedCommitError::InvalidFilePosition(ref cause) => cause,
            PostCommentForComparedCommitError::InvalidPath(ref cause) => cause,
            PostCommentForComparedCommitError::InvalidRelativeFileVersionEnum(ref cause) => cause,
            PostCommentForComparedCommitError::InvalidRepositoryName(ref cause) => cause,
            PostCommentForComparedCommitError::PathDoesNotExist(ref cause) => cause,
            PostCommentForComparedCommitError::PathRequired(ref cause) => cause,
            PostCommentForComparedCommitError::RepositoryDoesNotExist(ref cause) => cause,
            PostCommentForComparedCommitError::RepositoryNameRequired(ref cause) => cause,
            PostCommentForComparedCommitError::Validation(ref cause) => cause,
            PostCommentForComparedCommitError::Credentials(ref err) => err.description(),
            PostCommentForComparedCommitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PostCommentForComparedCommitError::ParseError(ref cause) => cause,
            PostCommentForComparedCommitError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PostCommentForPullRequest
#[derive(Debug, PartialEq)]
pub enum PostCommentForPullRequestError {
    /// <p>The before commit ID and the after commit ID are the same, which is not valid. The before commit ID and the after commit ID must be different commit IDs.</p>
    BeforeCommitIdAndAfterCommitIdAreSame(String),
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    ClientRequestTokenRequired(String),
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequired(String),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceeded(String),
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be re-used.</p>
    IdempotencyParameterMismatch(String),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestToken(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The location of the file is not valid. Make sure that you include the extension of the file as well as the file name.</p>
    InvalidFileLocation(String),
    /// <p>The position is not valid. Make sure that the line number exists in the version of the file you want to comment on.</p>
    InvalidFilePosition(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>Either the enum is not in a valid format, or the specified file version enum is not valid in respect to the current file version.</p>
    InvalidRelativeFileVersionEnum(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified path does not exist.</p>
    PathDoesNotExist(String),
    /// <p>The filePath for a location cannot be empty or null.</p>
    PathRequired(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The repository does not contain any pull requests with that pull request ID. Check to make sure you have provided the correct repository name for the pull request.</p>
    RepositoryNotAssociatedWithPullRequest(String),
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

impl PostCommentForPullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> PostCommentForPullRequestError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BeforeCommitIdAndAfterCommitIdAreSameException" => {
                    return PostCommentForPullRequestError::BeforeCommitIdAndAfterCommitIdAreSame(
                        String::from(error_message),
                    )
                }
                "ClientRequestTokenRequiredException" => {
                    return PostCommentForPullRequestError::ClientRequestTokenRequired(String::from(
                        error_message,
                    ))
                }
                "CommentContentRequiredException" => {
                    return PostCommentForPullRequestError::CommentContentRequired(String::from(
                        error_message,
                    ))
                }
                "CommentContentSizeLimitExceededException" => {
                    return PostCommentForPullRequestError::CommentContentSizeLimitExceeded(
                        String::from(error_message),
                    )
                }
                "CommitDoesNotExistException" => {
                    return PostCommentForPullRequestError::CommitDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "CommitIdRequiredException" => {
                    return PostCommentForPullRequestError::CommitIdRequired(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return PostCommentForPullRequestError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return PostCommentForPullRequestError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return PostCommentForPullRequestError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return PostCommentForPullRequestError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return PostCommentForPullRequestError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "IdempotencyParameterMismatchException" => {
                    return PostCommentForPullRequestError::IdempotencyParameterMismatch(
                        String::from(error_message),
                    )
                }
                "InvalidClientRequestTokenException" => {
                    return PostCommentForPullRequestError::InvalidClientRequestToken(String::from(
                        error_message,
                    ))
                }
                "InvalidCommitIdException" => {
                    return PostCommentForPullRequestError::InvalidCommitId(String::from(
                        error_message,
                    ))
                }
                "InvalidFileLocationException" => {
                    return PostCommentForPullRequestError::InvalidFileLocation(String::from(
                        error_message,
                    ))
                }
                "InvalidFilePositionException" => {
                    return PostCommentForPullRequestError::InvalidFilePosition(String::from(
                        error_message,
                    ))
                }
                "InvalidPathException" => {
                    return PostCommentForPullRequestError::InvalidPath(String::from(error_message))
                }
                "InvalidPullRequestIdException" => {
                    return PostCommentForPullRequestError::InvalidPullRequestId(String::from(
                        error_message,
                    ))
                }
                "InvalidRelativeFileVersionEnumException" => {
                    return PostCommentForPullRequestError::InvalidRelativeFileVersionEnum(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return PostCommentForPullRequestError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "PathDoesNotExistException" => {
                    return PostCommentForPullRequestError::PathDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PathRequiredException" => {
                    return PostCommentForPullRequestError::PathRequired(String::from(error_message))
                }
                "PullRequestDoesNotExistException" => {
                    return PostCommentForPullRequestError::PullRequestDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PullRequestIdRequiredException" => {
                    return PostCommentForPullRequestError::PullRequestIdRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return PostCommentForPullRequestError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return PostCommentForPullRequestError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryNotAssociatedWithPullRequestException" => {
                    return PostCommentForPullRequestError::RepositoryNotAssociatedWithPullRequest(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return PostCommentForPullRequestError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PostCommentForPullRequestError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PostCommentForPullRequestError {
    fn from(err: serde_json::error::Error) -> PostCommentForPullRequestError {
        PostCommentForPullRequestError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PostCommentForPullRequestError {
    fn from(err: CredentialsError) -> PostCommentForPullRequestError {
        PostCommentForPullRequestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PostCommentForPullRequestError {
    fn from(err: HttpDispatchError) -> PostCommentForPullRequestError {
        PostCommentForPullRequestError::HttpDispatch(err)
    }
}
impl From<io::Error> for PostCommentForPullRequestError {
    fn from(err: io::Error) -> PostCommentForPullRequestError {
        PostCommentForPullRequestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PostCommentForPullRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PostCommentForPullRequestError {
    fn description(&self) -> &str {
        match *self {
            PostCommentForPullRequestError::BeforeCommitIdAndAfterCommitIdAreSame(ref cause) => {
                cause
            }
            PostCommentForPullRequestError::ClientRequestTokenRequired(ref cause) => cause,
            PostCommentForPullRequestError::CommentContentRequired(ref cause) => cause,
            PostCommentForPullRequestError::CommentContentSizeLimitExceeded(ref cause) => cause,
            PostCommentForPullRequestError::CommitDoesNotExist(ref cause) => cause,
            PostCommentForPullRequestError::CommitIdRequired(ref cause) => cause,
            PostCommentForPullRequestError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            PostCommentForPullRequestError::EncryptionKeyAccessDenied(ref cause) => cause,
            PostCommentForPullRequestError::EncryptionKeyDisabled(ref cause) => cause,
            PostCommentForPullRequestError::EncryptionKeyNotFound(ref cause) => cause,
            PostCommentForPullRequestError::EncryptionKeyUnavailable(ref cause) => cause,
            PostCommentForPullRequestError::IdempotencyParameterMismatch(ref cause) => cause,
            PostCommentForPullRequestError::InvalidClientRequestToken(ref cause) => cause,
            PostCommentForPullRequestError::InvalidCommitId(ref cause) => cause,
            PostCommentForPullRequestError::InvalidFileLocation(ref cause) => cause,
            PostCommentForPullRequestError::InvalidFilePosition(ref cause) => cause,
            PostCommentForPullRequestError::InvalidPath(ref cause) => cause,
            PostCommentForPullRequestError::InvalidPullRequestId(ref cause) => cause,
            PostCommentForPullRequestError::InvalidRelativeFileVersionEnum(ref cause) => cause,
            PostCommentForPullRequestError::InvalidRepositoryName(ref cause) => cause,
            PostCommentForPullRequestError::PathDoesNotExist(ref cause) => cause,
            PostCommentForPullRequestError::PathRequired(ref cause) => cause,
            PostCommentForPullRequestError::PullRequestDoesNotExist(ref cause) => cause,
            PostCommentForPullRequestError::PullRequestIdRequired(ref cause) => cause,
            PostCommentForPullRequestError::RepositoryDoesNotExist(ref cause) => cause,
            PostCommentForPullRequestError::RepositoryNameRequired(ref cause) => cause,
            PostCommentForPullRequestError::RepositoryNotAssociatedWithPullRequest(ref cause) => {
                cause
            }
            PostCommentForPullRequestError::Validation(ref cause) => cause,
            PostCommentForPullRequestError::Credentials(ref err) => err.description(),
            PostCommentForPullRequestError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PostCommentForPullRequestError::ParseError(ref cause) => cause,
            PostCommentForPullRequestError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PostCommentReply
#[derive(Debug, PartialEq)]
pub enum PostCommentReplyError {
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request will return information about the initial request that used that token.</p>
    ClientRequestTokenRequired(String),
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequired(String),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceeded(String),
    /// <p>No comment exists with the provided ID. Verify that you have provided the correct ID, and then try again.</p>
    CommentDoesNotExist(String),
    /// <p>The comment ID is missing or null. A comment ID is required.</p>
    CommentIdRequired(String),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be re-used.</p>
    IdempotencyParameterMismatch(String),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestToken(String),
    /// <p>The comment ID is not in a valid format. Make sure that you have provided the full comment ID.</p>
    InvalidCommentId(String),
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

impl PostCommentReplyError {
    pub fn from_response(res: BufferedHttpResponse) -> PostCommentReplyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ClientRequestTokenRequiredException" => {
                    return PostCommentReplyError::ClientRequestTokenRequired(String::from(
                        error_message,
                    ))
                }
                "CommentContentRequiredException" => {
                    return PostCommentReplyError::CommentContentRequired(String::from(
                        error_message,
                    ))
                }
                "CommentContentSizeLimitExceededException" => {
                    return PostCommentReplyError::CommentContentSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "CommentDoesNotExistException" => {
                    return PostCommentReplyError::CommentDoesNotExist(String::from(error_message))
                }
                "CommentIdRequiredException" => {
                    return PostCommentReplyError::CommentIdRequired(String::from(error_message))
                }
                "IdempotencyParameterMismatchException" => {
                    return PostCommentReplyError::IdempotencyParameterMismatch(String::from(
                        error_message,
                    ))
                }
                "InvalidClientRequestTokenException" => {
                    return PostCommentReplyError::InvalidClientRequestToken(String::from(
                        error_message,
                    ))
                }
                "InvalidCommentIdException" => {
                    return PostCommentReplyError::InvalidCommentId(String::from(error_message))
                }
                "ValidationException" => {
                    return PostCommentReplyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PostCommentReplyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PostCommentReplyError {
    fn from(err: serde_json::error::Error) -> PostCommentReplyError {
        PostCommentReplyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PostCommentReplyError {
    fn from(err: CredentialsError) -> PostCommentReplyError {
        PostCommentReplyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PostCommentReplyError {
    fn from(err: HttpDispatchError) -> PostCommentReplyError {
        PostCommentReplyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PostCommentReplyError {
    fn from(err: io::Error) -> PostCommentReplyError {
        PostCommentReplyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PostCommentReplyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PostCommentReplyError {
    fn description(&self) -> &str {
        match *self {
            PostCommentReplyError::ClientRequestTokenRequired(ref cause) => cause,
            PostCommentReplyError::CommentContentRequired(ref cause) => cause,
            PostCommentReplyError::CommentContentSizeLimitExceeded(ref cause) => cause,
            PostCommentReplyError::CommentDoesNotExist(ref cause) => cause,
            PostCommentReplyError::CommentIdRequired(ref cause) => cause,
            PostCommentReplyError::IdempotencyParameterMismatch(ref cause) => cause,
            PostCommentReplyError::InvalidClientRequestToken(ref cause) => cause,
            PostCommentReplyError::InvalidCommentId(ref cause) => cause,
            PostCommentReplyError::Validation(ref cause) => cause,
            PostCommentReplyError::Credentials(ref err) => err.description(),
            PostCommentReplyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PostCommentReplyError::ParseError(ref cause) => cause,
            PostCommentReplyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutFile
#[derive(Debug, PartialEq)]
pub enum PutFileError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Type the name of a current branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    /// <p>The commit message is too long. Provide a shorter string. </p>
    CommitMessageLengthExceeded(String),
    /// <p>A file cannot be added to the repository because the specified path name has the same name as a file that already exists in this repository. Either provide a different name for the file, or specify a different path for the file.</p>
    DirectoryNameConflictsWithFileName(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The file cannot be added because it is empty. Empty files cannot be added to the repository with this API.</p>
    FileContentRequired(String),
    /// <p>The file cannot be added because it is too large. The maximum file size that can be added using PutFile is 6 MB. For files larger than 6 MB but smaller than 2 GB, add them using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>A file cannot be added to the repository because the specified file name has the same name as a directory in this repository. Either provide another name for the file, or add the file in a directory that does not match the file name.</p>
    FileNameConflictsWithDirectoryName(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The specified file mode permission is not valid. For a list of valid file mode permissions, see <a>PutFile</a>. </p>
    InvalidFileMode(String),
    /// <p>The parent commit ID is not valid. The commit ID cannot be empty, and must match the head commit ID for the branch of the repository where you want to add or update a file.</p>
    InvalidParentCommitId(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The file name is not valid because it has exceeded the character limit for file names. File names, including the path to the file, cannot exceed the character limit. </p>
    NameLengthExceeded(String),
    /// <p>The parent commit ID is not valid. The specified parent commit ID does not exist in the specified branch of the repository.</p>
    ParentCommitDoesNotExist(String),
    /// <p>The file could not be added because the provided parent commit ID is not the current tip of the specified branch. To view the full commit ID of the current head of the branch, use <a>GetBranch</a>.</p>
    ParentCommitIdOutdated(String),
    /// <p>A parent commit ID is required. To view the full commit ID of a branch in a repository, use <a>GetBranch</a> or a Git command (for example, git pull or git log).</p>
    ParentCommitIdRequired(String),
    /// <p>The filePath for a location cannot be empty or null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The file was not added or updated because the content of the file is exactly the same as the content of that file in the repository and branch that you specified.</p>
    SameFileContent(String),
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

impl PutFileError {
    pub fn from_response(res: BufferedHttpResponse) -> PutFileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BranchDoesNotExistException" => {
                    return PutFileError::BranchDoesNotExist(String::from(error_message))
                }
                "BranchNameIsTagNameException" => {
                    return PutFileError::BranchNameIsTagName(String::from(error_message))
                }
                "BranchNameRequiredException" => {
                    return PutFileError::BranchNameRequired(String::from(error_message))
                }
                "CommitMessageLengthExceededException" => {
                    return PutFileError::CommitMessageLengthExceeded(String::from(error_message))
                }
                "DirectoryNameConflictsWithFileNameException" => {
                    return PutFileError::DirectoryNameConflictsWithFileName(String::from(
                        error_message,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return PutFileError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return PutFileError::EncryptionKeyAccessDenied(String::from(error_message))
                }
                "EncryptionKeyDisabledException" => {
                    return PutFileError::EncryptionKeyDisabled(String::from(error_message))
                }
                "EncryptionKeyNotFoundException" => {
                    return PutFileError::EncryptionKeyNotFound(String::from(error_message))
                }
                "EncryptionKeyUnavailableException" => {
                    return PutFileError::EncryptionKeyUnavailable(String::from(error_message))
                }
                "FileContentRequiredException" => {
                    return PutFileError::FileContentRequired(String::from(error_message))
                }
                "FileContentSizeLimitExceededException" => {
                    return PutFileError::FileContentSizeLimitExceeded(String::from(error_message))
                }
                "FileNameConflictsWithDirectoryNameException" => {
                    return PutFileError::FileNameConflictsWithDirectoryName(String::from(
                        error_message,
                    ))
                }
                "InvalidBranchNameException" => {
                    return PutFileError::InvalidBranchName(String::from(error_message))
                }
                "InvalidEmailException" => {
                    return PutFileError::InvalidEmail(String::from(error_message))
                }
                "InvalidFileModeException" => {
                    return PutFileError::InvalidFileMode(String::from(error_message))
                }
                "InvalidParentCommitIdException" => {
                    return PutFileError::InvalidParentCommitId(String::from(error_message))
                }
                "InvalidPathException" => {
                    return PutFileError::InvalidPath(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return PutFileError::InvalidRepositoryName(String::from(error_message))
                }
                "NameLengthExceededException" => {
                    return PutFileError::NameLengthExceeded(String::from(error_message))
                }
                "ParentCommitDoesNotExistException" => {
                    return PutFileError::ParentCommitDoesNotExist(String::from(error_message))
                }
                "ParentCommitIdOutdatedException" => {
                    return PutFileError::ParentCommitIdOutdated(String::from(error_message))
                }
                "ParentCommitIdRequiredException" => {
                    return PutFileError::ParentCommitIdRequired(String::from(error_message))
                }
                "PathRequiredException" => {
                    return PutFileError::PathRequired(String::from(error_message))
                }
                "RepositoryDoesNotExistException" => {
                    return PutFileError::RepositoryDoesNotExist(String::from(error_message))
                }
                "RepositoryNameRequiredException" => {
                    return PutFileError::RepositoryNameRequired(String::from(error_message))
                }
                "SameFileContentException" => {
                    return PutFileError::SameFileContent(String::from(error_message))
                }
                "ValidationException" => return PutFileError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return PutFileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutFileError {
    fn from(err: serde_json::error::Error) -> PutFileError {
        PutFileError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutFileError {
    fn from(err: CredentialsError) -> PutFileError {
        PutFileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutFileError {
    fn from(err: HttpDispatchError) -> PutFileError {
        PutFileError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutFileError {
    fn from(err: io::Error) -> PutFileError {
        PutFileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutFileError {
    fn description(&self) -> &str {
        match *self {
            PutFileError::BranchDoesNotExist(ref cause) => cause,
            PutFileError::BranchNameIsTagName(ref cause) => cause,
            PutFileError::BranchNameRequired(ref cause) => cause,
            PutFileError::CommitMessageLengthExceeded(ref cause) => cause,
            PutFileError::DirectoryNameConflictsWithFileName(ref cause) => cause,
            PutFileError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            PutFileError::EncryptionKeyAccessDenied(ref cause) => cause,
            PutFileError::EncryptionKeyDisabled(ref cause) => cause,
            PutFileError::EncryptionKeyNotFound(ref cause) => cause,
            PutFileError::EncryptionKeyUnavailable(ref cause) => cause,
            PutFileError::FileContentRequired(ref cause) => cause,
            PutFileError::FileContentSizeLimitExceeded(ref cause) => cause,
            PutFileError::FileNameConflictsWithDirectoryName(ref cause) => cause,
            PutFileError::InvalidBranchName(ref cause) => cause,
            PutFileError::InvalidEmail(ref cause) => cause,
            PutFileError::InvalidFileMode(ref cause) => cause,
            PutFileError::InvalidParentCommitId(ref cause) => cause,
            PutFileError::InvalidPath(ref cause) => cause,
            PutFileError::InvalidRepositoryName(ref cause) => cause,
            PutFileError::NameLengthExceeded(ref cause) => cause,
            PutFileError::ParentCommitDoesNotExist(ref cause) => cause,
            PutFileError::ParentCommitIdOutdated(ref cause) => cause,
            PutFileError::ParentCommitIdRequired(ref cause) => cause,
            PutFileError::PathRequired(ref cause) => cause,
            PutFileError::RepositoryDoesNotExist(ref cause) => cause,
            PutFileError::RepositoryNameRequired(ref cause) => cause,
            PutFileError::SameFileContent(ref cause) => cause,
            PutFileError::Validation(ref cause) => cause,
            PutFileError::Credentials(ref err) => err.description(),
            PutFileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutFileError::ParseError(ref cause) => cause,
            PutFileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutRepositoryTriggers
#[derive(Debug, PartialEq)]
pub enum PutRepositoryTriggersError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>One or more branch names specified for the trigger is not valid.</p>
    InvalidRepositoryTriggerBranchName(String),
    /// <p>The custom data provided for the trigger is not valid.</p>
    InvalidRepositoryTriggerCustomData(String),
    /// <p>The Amazon Resource Name (ARN) for the trigger is not valid for the specified destination. The most common reason for this error is that the ARN does not meet the requirements for the service type.</p>
    InvalidRepositoryTriggerDestinationArn(String),
    /// <p>One or more events specified for the trigger is not valid. Check to make sure that all events specified match the requirements for allowed events.</p>
    InvalidRepositoryTriggerEvents(String),
    /// <p>The name of the trigger is not valid.</p>
    InvalidRepositoryTriggerName(String),
    /// <p>The region for the trigger target does not match the region for the repository. Triggers must be created in the same region as the target for the trigger.</p>
    InvalidRepositoryTriggerRegion(String),
    /// <p>The number of branches for the trigger was exceeded.</p>
    MaximumBranchesExceeded(String),
    /// <p>The number of triggers allowed for the repository was exceeded.</p>
    MaximumRepositoryTriggersExceeded(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>At least one branch name is required but was not specified in the trigger configuration.</p>
    RepositoryTriggerBranchNameListRequired(String),
    /// <p>A destination ARN for the target service for the trigger is required but was not specified.</p>
    RepositoryTriggerDestinationArnRequired(String),
    /// <p>At least one event for the trigger is required but was not specified.</p>
    RepositoryTriggerEventsListRequired(String),
    /// <p>A name for the trigger is required but was not specified.</p>
    RepositoryTriggerNameRequired(String),
    /// <p>The list of triggers for the repository is required but was not specified.</p>
    RepositoryTriggersListRequired(String),
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

impl PutRepositoryTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> PutRepositoryTriggersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return PutRepositoryTriggersError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return PutRepositoryTriggersError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return PutRepositoryTriggersError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return PutRepositoryTriggersError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return PutRepositoryTriggersError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return PutRepositoryTriggersError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryTriggerBranchNameException" => {
                    return PutRepositoryTriggersError::InvalidRepositoryTriggerBranchName(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryTriggerCustomDataException" => {
                    return PutRepositoryTriggersError::InvalidRepositoryTriggerCustomData(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryTriggerDestinationArnException" => {
                    return PutRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryTriggerEventsException" => {
                    return PutRepositoryTriggersError::InvalidRepositoryTriggerEvents(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryTriggerNameException" => {
                    return PutRepositoryTriggersError::InvalidRepositoryTriggerName(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryTriggerRegionException" => {
                    return PutRepositoryTriggersError::InvalidRepositoryTriggerRegion(String::from(
                        error_message,
                    ))
                }
                "MaximumBranchesExceededException" => {
                    return PutRepositoryTriggersError::MaximumBranchesExceeded(String::from(
                        error_message,
                    ))
                }
                "MaximumRepositoryTriggersExceededException" => {
                    return PutRepositoryTriggersError::MaximumRepositoryTriggersExceeded(
                        String::from(error_message),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return PutRepositoryTriggersError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return PutRepositoryTriggersError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryTriggerBranchNameListRequiredException" => {
                    return PutRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(
                        String::from(error_message),
                    )
                }
                "RepositoryTriggerDestinationArnRequiredException" => {
                    return PutRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(
                        String::from(error_message),
                    )
                }
                "RepositoryTriggerEventsListRequiredException" => {
                    return PutRepositoryTriggersError::RepositoryTriggerEventsListRequired(
                        String::from(error_message),
                    )
                }
                "RepositoryTriggerNameRequiredException" => {
                    return PutRepositoryTriggersError::RepositoryTriggerNameRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryTriggersListRequiredException" => {
                    return PutRepositoryTriggersError::RepositoryTriggersListRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return PutRepositoryTriggersError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutRepositoryTriggersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutRepositoryTriggersError {
    fn from(err: serde_json::error::Error) -> PutRepositoryTriggersError {
        PutRepositoryTriggersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRepositoryTriggersError {
    fn from(err: CredentialsError) -> PutRepositoryTriggersError {
        PutRepositoryTriggersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRepositoryTriggersError {
    fn from(err: HttpDispatchError) -> PutRepositoryTriggersError {
        PutRepositoryTriggersError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRepositoryTriggersError {
    fn from(err: io::Error) -> PutRepositoryTriggersError {
        PutRepositoryTriggersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRepositoryTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRepositoryTriggersError {
    fn description(&self) -> &str {
        match *self {
            PutRepositoryTriggersError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            PutRepositoryTriggersError::EncryptionKeyAccessDenied(ref cause) => cause,
            PutRepositoryTriggersError::EncryptionKeyDisabled(ref cause) => cause,
            PutRepositoryTriggersError::EncryptionKeyNotFound(ref cause) => cause,
            PutRepositoryTriggersError::EncryptionKeyUnavailable(ref cause) => cause,
            PutRepositoryTriggersError::InvalidRepositoryName(ref cause) => cause,
            PutRepositoryTriggersError::InvalidRepositoryTriggerBranchName(ref cause) => cause,
            PutRepositoryTriggersError::InvalidRepositoryTriggerCustomData(ref cause) => cause,
            PutRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(ref cause) => cause,
            PutRepositoryTriggersError::InvalidRepositoryTriggerEvents(ref cause) => cause,
            PutRepositoryTriggersError::InvalidRepositoryTriggerName(ref cause) => cause,
            PutRepositoryTriggersError::InvalidRepositoryTriggerRegion(ref cause) => cause,
            PutRepositoryTriggersError::MaximumBranchesExceeded(ref cause) => cause,
            PutRepositoryTriggersError::MaximumRepositoryTriggersExceeded(ref cause) => cause,
            PutRepositoryTriggersError::RepositoryDoesNotExist(ref cause) => cause,
            PutRepositoryTriggersError::RepositoryNameRequired(ref cause) => cause,
            PutRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(ref cause) => cause,
            PutRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(ref cause) => cause,
            PutRepositoryTriggersError::RepositoryTriggerEventsListRequired(ref cause) => cause,
            PutRepositoryTriggersError::RepositoryTriggerNameRequired(ref cause) => cause,
            PutRepositoryTriggersError::RepositoryTriggersListRequired(ref cause) => cause,
            PutRepositoryTriggersError::Validation(ref cause) => cause,
            PutRepositoryTriggersError::Credentials(ref err) => err.description(),
            PutRepositoryTriggersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutRepositoryTriggersError::ParseError(ref cause) => cause,
            PutRepositoryTriggersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TestRepositoryTriggers
#[derive(Debug, PartialEq)]
pub enum TestRepositoryTriggersError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>One or more branch names specified for the trigger is not valid.</p>
    InvalidRepositoryTriggerBranchName(String),
    /// <p>The custom data provided for the trigger is not valid.</p>
    InvalidRepositoryTriggerCustomData(String),
    /// <p>The Amazon Resource Name (ARN) for the trigger is not valid for the specified destination. The most common reason for this error is that the ARN does not meet the requirements for the service type.</p>
    InvalidRepositoryTriggerDestinationArn(String),
    /// <p>One or more events specified for the trigger is not valid. Check to make sure that all events specified match the requirements for allowed events.</p>
    InvalidRepositoryTriggerEvents(String),
    /// <p>The name of the trigger is not valid.</p>
    InvalidRepositoryTriggerName(String),
    /// <p>The region for the trigger target does not match the region for the repository. Triggers must be created in the same region as the target for the trigger.</p>
    InvalidRepositoryTriggerRegion(String),
    /// <p>The number of branches for the trigger was exceeded.</p>
    MaximumBranchesExceeded(String),
    /// <p>The number of triggers allowed for the repository was exceeded.</p>
    MaximumRepositoryTriggersExceeded(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>At least one branch name is required but was not specified in the trigger configuration.</p>
    RepositoryTriggerBranchNameListRequired(String),
    /// <p>A destination ARN for the target service for the trigger is required but was not specified.</p>
    RepositoryTriggerDestinationArnRequired(String),
    /// <p>At least one event for the trigger is required but was not specified.</p>
    RepositoryTriggerEventsListRequired(String),
    /// <p>A name for the trigger is required but was not specified.</p>
    RepositoryTriggerNameRequired(String),
    /// <p>The list of triggers for the repository is required but was not specified.</p>
    RepositoryTriggersListRequired(String),
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

impl TestRepositoryTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> TestRepositoryTriggersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return TestRepositoryTriggersError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return TestRepositoryTriggersError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return TestRepositoryTriggersError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return TestRepositoryTriggersError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return TestRepositoryTriggersError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return TestRepositoryTriggersError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryTriggerBranchNameException" => {
                    return TestRepositoryTriggersError::InvalidRepositoryTriggerBranchName(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryTriggerCustomDataException" => {
                    return TestRepositoryTriggersError::InvalidRepositoryTriggerCustomData(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryTriggerDestinationArnException" => {
                    return TestRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryTriggerEventsException" => {
                    return TestRepositoryTriggersError::InvalidRepositoryTriggerEvents(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryTriggerNameException" => {
                    return TestRepositoryTriggersError::InvalidRepositoryTriggerName(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryTriggerRegionException" => {
                    return TestRepositoryTriggersError::InvalidRepositoryTriggerRegion(
                        String::from(error_message),
                    )
                }
                "MaximumBranchesExceededException" => {
                    return TestRepositoryTriggersError::MaximumBranchesExceeded(String::from(
                        error_message,
                    ))
                }
                "MaximumRepositoryTriggersExceededException" => {
                    return TestRepositoryTriggersError::MaximumRepositoryTriggersExceeded(
                        String::from(error_message),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return TestRepositoryTriggersError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return TestRepositoryTriggersError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryTriggerBranchNameListRequiredException" => {
                    return TestRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(
                        String::from(error_message),
                    )
                }
                "RepositoryTriggerDestinationArnRequiredException" => {
                    return TestRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(
                        String::from(error_message),
                    )
                }
                "RepositoryTriggerEventsListRequiredException" => {
                    return TestRepositoryTriggersError::RepositoryTriggerEventsListRequired(
                        String::from(error_message),
                    )
                }
                "RepositoryTriggerNameRequiredException" => {
                    return TestRepositoryTriggersError::RepositoryTriggerNameRequired(String::from(
                        error_message,
                    ))
                }
                "RepositoryTriggersListRequiredException" => {
                    return TestRepositoryTriggersError::RepositoryTriggersListRequired(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return TestRepositoryTriggersError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return TestRepositoryTriggersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TestRepositoryTriggersError {
    fn from(err: serde_json::error::Error) -> TestRepositoryTriggersError {
        TestRepositoryTriggersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TestRepositoryTriggersError {
    fn from(err: CredentialsError) -> TestRepositoryTriggersError {
        TestRepositoryTriggersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestRepositoryTriggersError {
    fn from(err: HttpDispatchError) -> TestRepositoryTriggersError {
        TestRepositoryTriggersError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestRepositoryTriggersError {
    fn from(err: io::Error) -> TestRepositoryTriggersError {
        TestRepositoryTriggersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestRepositoryTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestRepositoryTriggersError {
    fn description(&self) -> &str {
        match *self {
            TestRepositoryTriggersError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            TestRepositoryTriggersError::EncryptionKeyAccessDenied(ref cause) => cause,
            TestRepositoryTriggersError::EncryptionKeyDisabled(ref cause) => cause,
            TestRepositoryTriggersError::EncryptionKeyNotFound(ref cause) => cause,
            TestRepositoryTriggersError::EncryptionKeyUnavailable(ref cause) => cause,
            TestRepositoryTriggersError::InvalidRepositoryName(ref cause) => cause,
            TestRepositoryTriggersError::InvalidRepositoryTriggerBranchName(ref cause) => cause,
            TestRepositoryTriggersError::InvalidRepositoryTriggerCustomData(ref cause) => cause,
            TestRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(ref cause) => cause,
            TestRepositoryTriggersError::InvalidRepositoryTriggerEvents(ref cause) => cause,
            TestRepositoryTriggersError::InvalidRepositoryTriggerName(ref cause) => cause,
            TestRepositoryTriggersError::InvalidRepositoryTriggerRegion(ref cause) => cause,
            TestRepositoryTriggersError::MaximumBranchesExceeded(ref cause) => cause,
            TestRepositoryTriggersError::MaximumRepositoryTriggersExceeded(ref cause) => cause,
            TestRepositoryTriggersError::RepositoryDoesNotExist(ref cause) => cause,
            TestRepositoryTriggersError::RepositoryNameRequired(ref cause) => cause,
            TestRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(ref cause) => {
                cause
            }
            TestRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(ref cause) => {
                cause
            }
            TestRepositoryTriggersError::RepositoryTriggerEventsListRequired(ref cause) => cause,
            TestRepositoryTriggersError::RepositoryTriggerNameRequired(ref cause) => cause,
            TestRepositoryTriggersError::RepositoryTriggersListRequired(ref cause) => cause,
            TestRepositoryTriggersError::Validation(ref cause) => cause,
            TestRepositoryTriggersError::Credentials(ref err) => err.description(),
            TestRepositoryTriggersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TestRepositoryTriggersError::ParseError(ref cause) => cause,
            TestRepositoryTriggersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateComment
#[derive(Debug, PartialEq)]
pub enum UpdateCommentError {
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequired(String),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceeded(String),
    /// <p>This comment has already been deleted. You cannot edit or delete a deleted comment.</p>
    CommentDeleted(String),
    /// <p>No comment exists with the provided ID. Verify that you have provided the correct ID, and then try again.</p>
    CommentDoesNotExist(String),
    /// <p>The comment ID is missing or null. A comment ID is required.</p>
    CommentIdRequired(String),
    /// <p>You cannot modify or delete this comment. Only comment authors can modify or delete their comments.</p>
    CommentNotCreatedByCaller(String),
    /// <p>The comment ID is not in a valid format. Make sure that you have provided the full comment ID.</p>
    InvalidCommentId(String),
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

impl UpdateCommentError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateCommentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CommentContentRequiredException" => {
                    return UpdateCommentError::CommentContentRequired(String::from(error_message))
                }
                "CommentContentSizeLimitExceededException" => {
                    return UpdateCommentError::CommentContentSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "CommentDeletedException" => {
                    return UpdateCommentError::CommentDeleted(String::from(error_message))
                }
                "CommentDoesNotExistException" => {
                    return UpdateCommentError::CommentDoesNotExist(String::from(error_message))
                }
                "CommentIdRequiredException" => {
                    return UpdateCommentError::CommentIdRequired(String::from(error_message))
                }
                "CommentNotCreatedByCallerException" => {
                    return UpdateCommentError::CommentNotCreatedByCaller(String::from(
                        error_message,
                    ))
                }
                "InvalidCommentIdException" => {
                    return UpdateCommentError::InvalidCommentId(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateCommentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateCommentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateCommentError {
    fn from(err: serde_json::error::Error) -> UpdateCommentError {
        UpdateCommentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCommentError {
    fn from(err: CredentialsError) -> UpdateCommentError {
        UpdateCommentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCommentError {
    fn from(err: HttpDispatchError) -> UpdateCommentError {
        UpdateCommentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCommentError {
    fn from(err: io::Error) -> UpdateCommentError {
        UpdateCommentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCommentError {
    fn description(&self) -> &str {
        match *self {
            UpdateCommentError::CommentContentRequired(ref cause) => cause,
            UpdateCommentError::CommentContentSizeLimitExceeded(ref cause) => cause,
            UpdateCommentError::CommentDeleted(ref cause) => cause,
            UpdateCommentError::CommentDoesNotExist(ref cause) => cause,
            UpdateCommentError::CommentIdRequired(ref cause) => cause,
            UpdateCommentError::CommentNotCreatedByCaller(ref cause) => cause,
            UpdateCommentError::InvalidCommentId(ref cause) => cause,
            UpdateCommentError::Validation(ref cause) => cause,
            UpdateCommentError::Credentials(ref err) => err.description(),
            UpdateCommentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateCommentError::ParseError(ref cause) => cause,
            UpdateCommentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateDefaultBranch
#[derive(Debug, PartialEq)]
pub enum UpdateDefaultBranchError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl UpdateDefaultBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDefaultBranchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BranchDoesNotExistException" => {
                    return UpdateDefaultBranchError::BranchDoesNotExist(String::from(error_message))
                }
                "BranchNameRequiredException" => {
                    return UpdateDefaultBranchError::BranchNameRequired(String::from(error_message))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return UpdateDefaultBranchError::EncryptionIntegrityChecksFailed(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return UpdateDefaultBranchError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return UpdateDefaultBranchError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return UpdateDefaultBranchError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return UpdateDefaultBranchError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidBranchNameException" => {
                    return UpdateDefaultBranchError::InvalidBranchName(String::from(error_message))
                }
                "InvalidRepositoryNameException" => {
                    return UpdateDefaultBranchError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return UpdateDefaultBranchError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return UpdateDefaultBranchError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateDefaultBranchError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateDefaultBranchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDefaultBranchError {
    fn from(err: serde_json::error::Error) -> UpdateDefaultBranchError {
        UpdateDefaultBranchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDefaultBranchError {
    fn from(err: CredentialsError) -> UpdateDefaultBranchError {
        UpdateDefaultBranchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDefaultBranchError {
    fn from(err: HttpDispatchError) -> UpdateDefaultBranchError {
        UpdateDefaultBranchError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDefaultBranchError {
    fn from(err: io::Error) -> UpdateDefaultBranchError {
        UpdateDefaultBranchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDefaultBranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDefaultBranchError {
    fn description(&self) -> &str {
        match *self {
            UpdateDefaultBranchError::BranchDoesNotExist(ref cause) => cause,
            UpdateDefaultBranchError::BranchNameRequired(ref cause) => cause,
            UpdateDefaultBranchError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            UpdateDefaultBranchError::EncryptionKeyAccessDenied(ref cause) => cause,
            UpdateDefaultBranchError::EncryptionKeyDisabled(ref cause) => cause,
            UpdateDefaultBranchError::EncryptionKeyNotFound(ref cause) => cause,
            UpdateDefaultBranchError::EncryptionKeyUnavailable(ref cause) => cause,
            UpdateDefaultBranchError::InvalidBranchName(ref cause) => cause,
            UpdateDefaultBranchError::InvalidRepositoryName(ref cause) => cause,
            UpdateDefaultBranchError::RepositoryDoesNotExist(ref cause) => cause,
            UpdateDefaultBranchError::RepositoryNameRequired(ref cause) => cause,
            UpdateDefaultBranchError::Validation(ref cause) => cause,
            UpdateDefaultBranchError::Credentials(ref err) => err.description(),
            UpdateDefaultBranchError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDefaultBranchError::ParseError(ref cause) => cause,
            UpdateDefaultBranchError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdatePullRequestDescription
#[derive(Debug, PartialEq)]
pub enum UpdatePullRequestDescriptionError {
    /// <p>The pull request description is not valid. Descriptions are limited to 1,000 characters in length.</p>
    InvalidDescription(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
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

impl UpdatePullRequestDescriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdatePullRequestDescriptionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidDescriptionException" => {
                    return UpdatePullRequestDescriptionError::InvalidDescription(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return UpdatePullRequestDescriptionError::InvalidPullRequestId(String::from(
                        error_message,
                    ))
                }
                "PullRequestAlreadyClosedException" => {
                    return UpdatePullRequestDescriptionError::PullRequestAlreadyClosed(
                        String::from(error_message),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return UpdatePullRequestDescriptionError::PullRequestDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PullRequestIdRequiredException" => {
                    return UpdatePullRequestDescriptionError::PullRequestIdRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdatePullRequestDescriptionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdatePullRequestDescriptionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdatePullRequestDescriptionError {
    fn from(err: serde_json::error::Error) -> UpdatePullRequestDescriptionError {
        UpdatePullRequestDescriptionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePullRequestDescriptionError {
    fn from(err: CredentialsError) -> UpdatePullRequestDescriptionError {
        UpdatePullRequestDescriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePullRequestDescriptionError {
    fn from(err: HttpDispatchError) -> UpdatePullRequestDescriptionError {
        UpdatePullRequestDescriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePullRequestDescriptionError {
    fn from(err: io::Error) -> UpdatePullRequestDescriptionError {
        UpdatePullRequestDescriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePullRequestDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePullRequestDescriptionError {
    fn description(&self) -> &str {
        match *self {
            UpdatePullRequestDescriptionError::InvalidDescription(ref cause) => cause,
            UpdatePullRequestDescriptionError::InvalidPullRequestId(ref cause) => cause,
            UpdatePullRequestDescriptionError::PullRequestAlreadyClosed(ref cause) => cause,
            UpdatePullRequestDescriptionError::PullRequestDoesNotExist(ref cause) => cause,
            UpdatePullRequestDescriptionError::PullRequestIdRequired(ref cause) => cause,
            UpdatePullRequestDescriptionError::Validation(ref cause) => cause,
            UpdatePullRequestDescriptionError::Credentials(ref err) => err.description(),
            UpdatePullRequestDescriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdatePullRequestDescriptionError::ParseError(ref cause) => cause,
            UpdatePullRequestDescriptionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdatePullRequestStatus
#[derive(Debug, PartialEq)]
pub enum UpdatePullRequestStatusError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The pull request status is not valid. The only valid values are <code>OPEN</code> and <code>CLOSED</code>.</p>
    InvalidPullRequestStatus(String),
    /// <p>The pull request status update is not valid. The only valid update is from <code>OPEN</code> to <code>CLOSED</code>.</p>
    InvalidPullRequestStatusUpdate(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>A pull request status is required, but none was provided.</p>
    PullRequestStatusRequired(String),
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

impl UpdatePullRequestStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdatePullRequestStatusError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return UpdatePullRequestStatusError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return UpdatePullRequestStatusError::EncryptionKeyAccessDenied(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return UpdatePullRequestStatusError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return UpdatePullRequestStatusError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return UpdatePullRequestStatusError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return UpdatePullRequestStatusError::InvalidPullRequestId(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestStatusException" => {
                    return UpdatePullRequestStatusError::InvalidPullRequestStatus(String::from(
                        error_message,
                    ))
                }
                "InvalidPullRequestStatusUpdateException" => {
                    return UpdatePullRequestStatusError::InvalidPullRequestStatusUpdate(
                        String::from(error_message),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return UpdatePullRequestStatusError::PullRequestDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PullRequestIdRequiredException" => {
                    return UpdatePullRequestStatusError::PullRequestIdRequired(String::from(
                        error_message,
                    ))
                }
                "PullRequestStatusRequiredException" => {
                    return UpdatePullRequestStatusError::PullRequestStatusRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdatePullRequestStatusError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdatePullRequestStatusError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdatePullRequestStatusError {
    fn from(err: serde_json::error::Error) -> UpdatePullRequestStatusError {
        UpdatePullRequestStatusError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePullRequestStatusError {
    fn from(err: CredentialsError) -> UpdatePullRequestStatusError {
        UpdatePullRequestStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePullRequestStatusError {
    fn from(err: HttpDispatchError) -> UpdatePullRequestStatusError {
        UpdatePullRequestStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePullRequestStatusError {
    fn from(err: io::Error) -> UpdatePullRequestStatusError {
        UpdatePullRequestStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePullRequestStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePullRequestStatusError {
    fn description(&self) -> &str {
        match *self {
            UpdatePullRequestStatusError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            UpdatePullRequestStatusError::EncryptionKeyAccessDenied(ref cause) => cause,
            UpdatePullRequestStatusError::EncryptionKeyDisabled(ref cause) => cause,
            UpdatePullRequestStatusError::EncryptionKeyNotFound(ref cause) => cause,
            UpdatePullRequestStatusError::EncryptionKeyUnavailable(ref cause) => cause,
            UpdatePullRequestStatusError::InvalidPullRequestId(ref cause) => cause,
            UpdatePullRequestStatusError::InvalidPullRequestStatus(ref cause) => cause,
            UpdatePullRequestStatusError::InvalidPullRequestStatusUpdate(ref cause) => cause,
            UpdatePullRequestStatusError::PullRequestDoesNotExist(ref cause) => cause,
            UpdatePullRequestStatusError::PullRequestIdRequired(ref cause) => cause,
            UpdatePullRequestStatusError::PullRequestStatusRequired(ref cause) => cause,
            UpdatePullRequestStatusError::Validation(ref cause) => cause,
            UpdatePullRequestStatusError::Credentials(ref err) => err.description(),
            UpdatePullRequestStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdatePullRequestStatusError::ParseError(ref cause) => cause,
            UpdatePullRequestStatusError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdatePullRequestTitle
#[derive(Debug, PartialEq)]
pub enum UpdatePullRequestTitleError {
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The title of the pull request is not valid. Pull request titles cannot exceed 100 characters in length.</p>
    InvalidTitle(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>A pull request title is required. It cannot be empty or null.</p>
    TitleRequired(String),
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

impl UpdatePullRequestTitleError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdatePullRequestTitleError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidPullRequestIdException" => {
                    return UpdatePullRequestTitleError::InvalidPullRequestId(String::from(
                        error_message,
                    ))
                }
                "InvalidTitleException" => {
                    return UpdatePullRequestTitleError::InvalidTitle(String::from(error_message))
                }
                "PullRequestAlreadyClosedException" => {
                    return UpdatePullRequestTitleError::PullRequestAlreadyClosed(String::from(
                        error_message,
                    ))
                }
                "PullRequestDoesNotExistException" => {
                    return UpdatePullRequestTitleError::PullRequestDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "PullRequestIdRequiredException" => {
                    return UpdatePullRequestTitleError::PullRequestIdRequired(String::from(
                        error_message,
                    ))
                }
                "TitleRequiredException" => {
                    return UpdatePullRequestTitleError::TitleRequired(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdatePullRequestTitleError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdatePullRequestTitleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdatePullRequestTitleError {
    fn from(err: serde_json::error::Error) -> UpdatePullRequestTitleError {
        UpdatePullRequestTitleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePullRequestTitleError {
    fn from(err: CredentialsError) -> UpdatePullRequestTitleError {
        UpdatePullRequestTitleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePullRequestTitleError {
    fn from(err: HttpDispatchError) -> UpdatePullRequestTitleError {
        UpdatePullRequestTitleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePullRequestTitleError {
    fn from(err: io::Error) -> UpdatePullRequestTitleError {
        UpdatePullRequestTitleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePullRequestTitleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePullRequestTitleError {
    fn description(&self) -> &str {
        match *self {
            UpdatePullRequestTitleError::InvalidPullRequestId(ref cause) => cause,
            UpdatePullRequestTitleError::InvalidTitle(ref cause) => cause,
            UpdatePullRequestTitleError::PullRequestAlreadyClosed(ref cause) => cause,
            UpdatePullRequestTitleError::PullRequestDoesNotExist(ref cause) => cause,
            UpdatePullRequestTitleError::PullRequestIdRequired(ref cause) => cause,
            UpdatePullRequestTitleError::TitleRequired(ref cause) => cause,
            UpdatePullRequestTitleError::Validation(ref cause) => cause,
            UpdatePullRequestTitleError::Credentials(ref err) => err.description(),
            UpdatePullRequestTitleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdatePullRequestTitleError::ParseError(ref cause) => cause,
            UpdatePullRequestTitleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateRepositoryDescription
#[derive(Debug, PartialEq)]
pub enum UpdateRepositoryDescriptionError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    /// <p>The specified repository description is not valid.</p>
    InvalidRepositoryDescription(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl UpdateRepositoryDescriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateRepositoryDescriptionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "EncryptionIntegrityChecksFailedException" => {
                    return UpdateRepositoryDescriptionError::EncryptionIntegrityChecksFailed(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return UpdateRepositoryDescriptionError::EncryptionKeyAccessDenied(
                        String::from(error_message),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return UpdateRepositoryDescriptionError::EncryptionKeyDisabled(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return UpdateRepositoryDescriptionError::EncryptionKeyNotFound(String::from(
                        error_message,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return UpdateRepositoryDescriptionError::EncryptionKeyUnavailable(String::from(
                        error_message,
                    ))
                }
                "InvalidRepositoryDescriptionException" => {
                    return UpdateRepositoryDescriptionError::InvalidRepositoryDescription(
                        String::from(error_message),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return UpdateRepositoryDescriptionError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return UpdateRepositoryDescriptionError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return UpdateRepositoryDescriptionError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateRepositoryDescriptionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateRepositoryDescriptionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateRepositoryDescriptionError {
    fn from(err: serde_json::error::Error) -> UpdateRepositoryDescriptionError {
        UpdateRepositoryDescriptionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRepositoryDescriptionError {
    fn from(err: CredentialsError) -> UpdateRepositoryDescriptionError {
        UpdateRepositoryDescriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRepositoryDescriptionError {
    fn from(err: HttpDispatchError) -> UpdateRepositoryDescriptionError {
        UpdateRepositoryDescriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRepositoryDescriptionError {
    fn from(err: io::Error) -> UpdateRepositoryDescriptionError {
        UpdateRepositoryDescriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRepositoryDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRepositoryDescriptionError {
    fn description(&self) -> &str {
        match *self {
            UpdateRepositoryDescriptionError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            UpdateRepositoryDescriptionError::EncryptionKeyAccessDenied(ref cause) => cause,
            UpdateRepositoryDescriptionError::EncryptionKeyDisabled(ref cause) => cause,
            UpdateRepositoryDescriptionError::EncryptionKeyNotFound(ref cause) => cause,
            UpdateRepositoryDescriptionError::EncryptionKeyUnavailable(ref cause) => cause,
            UpdateRepositoryDescriptionError::InvalidRepositoryDescription(ref cause) => cause,
            UpdateRepositoryDescriptionError::InvalidRepositoryName(ref cause) => cause,
            UpdateRepositoryDescriptionError::RepositoryDoesNotExist(ref cause) => cause,
            UpdateRepositoryDescriptionError::RepositoryNameRequired(ref cause) => cause,
            UpdateRepositoryDescriptionError::Validation(ref cause) => cause,
            UpdateRepositoryDescriptionError::Credentials(ref err) => err.description(),
            UpdateRepositoryDescriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateRepositoryDescriptionError::ParseError(ref cause) => cause,
            UpdateRepositoryDescriptionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateRepositoryName
#[derive(Debug, PartialEq)]
pub enum UpdateRepositoryNameError {
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>The specified repository name already exists.</p>
    RepositoryNameExists(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
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

impl UpdateRepositoryNameError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateRepositoryNameError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidRepositoryNameException" => {
                    return UpdateRepositoryNameError::InvalidRepositoryName(String::from(
                        error_message,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return UpdateRepositoryNameError::RepositoryDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameExistsException" => {
                    return UpdateRepositoryNameError::RepositoryNameExists(String::from(
                        error_message,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return UpdateRepositoryNameError::RepositoryNameRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateRepositoryNameError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateRepositoryNameError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateRepositoryNameError {
    fn from(err: serde_json::error::Error) -> UpdateRepositoryNameError {
        UpdateRepositoryNameError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRepositoryNameError {
    fn from(err: CredentialsError) -> UpdateRepositoryNameError {
        UpdateRepositoryNameError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRepositoryNameError {
    fn from(err: HttpDispatchError) -> UpdateRepositoryNameError {
        UpdateRepositoryNameError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRepositoryNameError {
    fn from(err: io::Error) -> UpdateRepositoryNameError {
        UpdateRepositoryNameError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRepositoryNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRepositoryNameError {
    fn description(&self) -> &str {
        match *self {
            UpdateRepositoryNameError::InvalidRepositoryName(ref cause) => cause,
            UpdateRepositoryNameError::RepositoryDoesNotExist(ref cause) => cause,
            UpdateRepositoryNameError::RepositoryNameExists(ref cause) => cause,
            UpdateRepositoryNameError::RepositoryNameRequired(ref cause) => cause,
            UpdateRepositoryNameError::Validation(ref cause) => cause,
            UpdateRepositoryNameError::Credentials(ref err) => err.description(),
            UpdateRepositoryNameError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateRepositoryNameError::ParseError(ref cause) => cause,
            UpdateRepositoryNameError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the CodeCommit API. CodeCommit clients implement this trait.
pub trait CodeCommit {
    /// <p><p>Returns information about one or more repositories.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note></p>
    fn batch_get_repositories(
        &self,
        input: BatchGetRepositoriesInput,
    ) -> RusotoFuture<BatchGetRepositoriesOutput, BatchGetRepositoriesError>;

    /// <p><p>Creates a new branch in a repository and points the branch to a commit.</p> <note> <p>Calling the create branch operation does not set a repository&#39;s default branch. To do this, call the update default branch operation.</p> </note></p>
    fn create_branch(&self, input: CreateBranchInput) -> RusotoFuture<(), CreateBranchError>;

    /// <p>Creates a pull request in the specified repository.</p>
    fn create_pull_request(
        &self,
        input: CreatePullRequestInput,
    ) -> RusotoFuture<CreatePullRequestOutput, CreatePullRequestError>;

    /// <p>Creates a new, empty repository.</p>
    fn create_repository(
        &self,
        input: CreateRepositoryInput,
    ) -> RusotoFuture<CreateRepositoryOutput, CreateRepositoryError>;

    /// <p>Deletes a branch from a repository, unless that branch is the default branch for the repository. </p>
    fn delete_branch(
        &self,
        input: DeleteBranchInput,
    ) -> RusotoFuture<DeleteBranchOutput, DeleteBranchError>;

    /// <p>Deletes the content of a comment made on a change, file, or commit in a repository.</p>
    fn delete_comment_content(
        &self,
        input: DeleteCommentContentInput,
    ) -> RusotoFuture<DeleteCommentContentOutput, DeleteCommentContentError>;

    /// <p><p>Deletes a repository. If a specified repository was already deleted, a null repository ID will be returned.</p> <important> <p>Deleting a repository also deletes all associated objects and metadata. After a repository is deleted, all future push calls to the deleted repository will fail.</p> </important></p>
    fn delete_repository(
        &self,
        input: DeleteRepositoryInput,
    ) -> RusotoFuture<DeleteRepositoryOutput, DeleteRepositoryError>;

    /// <p>Returns information about one or more pull request events.</p>
    fn describe_pull_request_events(
        &self,
        input: DescribePullRequestEventsInput,
    ) -> RusotoFuture<DescribePullRequestEventsOutput, DescribePullRequestEventsError>;

    /// <p>Returns the base-64 encoded content of an individual blob within a repository.</p>
    fn get_blob(&self, input: GetBlobInput) -> RusotoFuture<GetBlobOutput, GetBlobError>;

    /// <p>Returns information about a repository branch, including its name and the last commit ID.</p>
    fn get_branch(&self, input: GetBranchInput) -> RusotoFuture<GetBranchOutput, GetBranchError>;

    /// <p>Returns the content of a comment made on a change, file, or commit in a repository.</p>
    fn get_comment(
        &self,
        input: GetCommentInput,
    ) -> RusotoFuture<GetCommentOutput, GetCommentError>;

    /// <p>Returns information about comments made on the comparison between two commits.</p>
    fn get_comments_for_compared_commit(
        &self,
        input: GetCommentsForComparedCommitInput,
    ) -> RusotoFuture<GetCommentsForComparedCommitOutput, GetCommentsForComparedCommitError>;

    /// <p>Returns comments made on a pull request.</p>
    fn get_comments_for_pull_request(
        &self,
        input: GetCommentsForPullRequestInput,
    ) -> RusotoFuture<GetCommentsForPullRequestOutput, GetCommentsForPullRequestError>;

    /// <p>Returns information about a commit, including commit message and committer information.</p>
    fn get_commit(&self, input: GetCommitInput) -> RusotoFuture<GetCommitOutput, GetCommitError>;

    /// <p>Returns information about the differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID or other fully qualified reference). Results can be limited to a specified path.</p>
    fn get_differences(
        &self,
        input: GetDifferencesInput,
    ) -> RusotoFuture<GetDifferencesOutput, GetDifferencesError>;

    /// <p>Returns information about merge conflicts between the before and after commit IDs for a pull request in a repository.</p>
    fn get_merge_conflicts(
        &self,
        input: GetMergeConflictsInput,
    ) -> RusotoFuture<GetMergeConflictsOutput, GetMergeConflictsError>;

    /// <p>Gets information about a pull request in a specified repository.</p>
    fn get_pull_request(
        &self,
        input: GetPullRequestInput,
    ) -> RusotoFuture<GetPullRequestOutput, GetPullRequestError>;

    /// <p><p>Returns information about a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note></p>
    fn get_repository(
        &self,
        input: GetRepositoryInput,
    ) -> RusotoFuture<GetRepositoryOutput, GetRepositoryError>;

    /// <p>Gets information about triggers configured for a repository.</p>
    fn get_repository_triggers(
        &self,
        input: GetRepositoryTriggersInput,
    ) -> RusotoFuture<GetRepositoryTriggersOutput, GetRepositoryTriggersError>;

    /// <p>Gets information about one or more branches in a repository.</p>
    fn list_branches(
        &self,
        input: ListBranchesInput,
    ) -> RusotoFuture<ListBranchesOutput, ListBranchesError>;

    /// <p>Returns a list of pull requests for a specified repository. The return list can be refined by pull request status or pull request author ARN.</p>
    fn list_pull_requests(
        &self,
        input: ListPullRequestsInput,
    ) -> RusotoFuture<ListPullRequestsOutput, ListPullRequestsError>;

    /// <p>Gets information about one or more repositories.</p>
    fn list_repositories(
        &self,
        input: ListRepositoriesInput,
    ) -> RusotoFuture<ListRepositoriesOutput, ListRepositoriesError>;

    /// <p>Closes a pull request and attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the fast-forward merge option.</p>
    fn merge_pull_request_by_fast_forward(
        &self,
        input: MergePullRequestByFastForwardInput,
    ) -> RusotoFuture<MergePullRequestByFastForwardOutput, MergePullRequestByFastForwardError>;

    /// <p>Posts a comment on the comparison between two commits.</p>
    fn post_comment_for_compared_commit(
        &self,
        input: PostCommentForComparedCommitInput,
    ) -> RusotoFuture<PostCommentForComparedCommitOutput, PostCommentForComparedCommitError>;

    /// <p>Posts a comment on a pull request.</p>
    fn post_comment_for_pull_request(
        &self,
        input: PostCommentForPullRequestInput,
    ) -> RusotoFuture<PostCommentForPullRequestOutput, PostCommentForPullRequestError>;

    /// <p>Posts a comment in reply to an existing comment on a comparison between commits or a pull request.</p>
    fn post_comment_reply(
        &self,
        input: PostCommentReplyInput,
    ) -> RusotoFuture<PostCommentReplyOutput, PostCommentReplyError>;

    /// <p>Adds or updates a file in an AWS CodeCommit repository.</p>
    fn put_file(&self, input: PutFileInput) -> RusotoFuture<PutFileOutput, PutFileError>;

    /// <p>Replaces all triggers for a repository. This can be used to create or delete triggers.</p>
    fn put_repository_triggers(
        &self,
        input: PutRepositoryTriggersInput,
    ) -> RusotoFuture<PutRepositoryTriggersOutput, PutRepositoryTriggersError>;

    /// <p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test will send data from the last commit. If no data is available, sample data will be generated.</p>
    fn test_repository_triggers(
        &self,
        input: TestRepositoryTriggersInput,
    ) -> RusotoFuture<TestRepositoryTriggersOutput, TestRepositoryTriggersError>;

    /// <p>Replaces the contents of a comment.</p>
    fn update_comment(
        &self,
        input: UpdateCommentInput,
    ) -> RusotoFuture<UpdateCommentOutput, UpdateCommentError>;

    /// <p><p>Sets or changes the default branch name for the specified repository.</p> <note> <p>If you use this operation to change the default branch name to the current default branch name, a success message is returned even though the default branch did not change.</p> </note></p>
    fn update_default_branch(
        &self,
        input: UpdateDefaultBranchInput,
    ) -> RusotoFuture<(), UpdateDefaultBranchError>;

    /// <p>Replaces the contents of the description of a pull request.</p>
    fn update_pull_request_description(
        &self,
        input: UpdatePullRequestDescriptionInput,
    ) -> RusotoFuture<UpdatePullRequestDescriptionOutput, UpdatePullRequestDescriptionError>;

    /// <p>Updates the status of a pull request. </p>
    fn update_pull_request_status(
        &self,
        input: UpdatePullRequestStatusInput,
    ) -> RusotoFuture<UpdatePullRequestStatusOutput, UpdatePullRequestStatusError>;

    /// <p>Replaces the title of a pull request.</p>
    fn update_pull_request_title(
        &self,
        input: UpdatePullRequestTitleInput,
    ) -> RusotoFuture<UpdatePullRequestTitleOutput, UpdatePullRequestTitleError>;

    /// <p><p>Sets or changes the comment or description for a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note></p>
    fn update_repository_description(
        &self,
        input: UpdateRepositoryDescriptionInput,
    ) -> RusotoFuture<(), UpdateRepositoryDescriptionError>;

    /// <p>Renames a repository. The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix ".git" is prohibited. For a full description of the limits on repository names, see <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide.</p>
    fn update_repository_name(
        &self,
        input: UpdateRepositoryNameInput,
    ) -> RusotoFuture<(), UpdateRepositoryNameError>;
}
/// A client for the CodeCommit API.
pub struct CodeCommitClient {
    client: Client,
    region: region::Region,
}

impl CodeCommitClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeCommitClient {
        CodeCommitClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeCommitClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CodeCommitClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CodeCommit for CodeCommitClient {
    /// <p><p>Returns information about one or more repositories.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note></p>
    fn batch_get_repositories(
        &self,
        input: BatchGetRepositoriesInput,
    ) -> RusotoFuture<BatchGetRepositoriesOutput, BatchGetRepositoriesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.BatchGetRepositories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetRepositoriesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchGetRepositoriesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Creates a new branch in a repository and points the branch to a commit.</p> <note> <p>Calling the create branch operation does not set a repository&#39;s default branch. To do this, call the update default branch operation.</p> </note></p>
    fn create_branch(&self, input: CreateBranchInput) -> RusotoFuture<(), CreateBranchError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBranchError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a pull request in the specified repository.</p>
    fn create_pull_request(
        &self,
        input: CreatePullRequestInput,
    ) -> RusotoFuture<CreatePullRequestOutput, CreatePullRequestError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreatePullRequest");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreatePullRequestOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePullRequestError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new, empty repository.</p>
    fn create_repository(
        &self,
        input: CreateRepositoryInput,
    ) -> RusotoFuture<CreateRepositoryOutput, CreateRepositoryError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRepositoryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateRepositoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a branch from a repository, unless that branch is the default branch for the repository. </p>
    fn delete_branch(
        &self,
        input: DeleteBranchInput,
    ) -> RusotoFuture<DeleteBranchOutput, DeleteBranchError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteBranchOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBranchError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the content of a comment made on a change, file, or commit in a repository.</p>
    fn delete_comment_content(
        &self,
        input: DeleteCommentContentInput,
    ) -> RusotoFuture<DeleteCommentContentOutput, DeleteCommentContentError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteCommentContent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteCommentContentOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteCommentContentError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Deletes a repository. If a specified repository was already deleted, a null repository ID will be returned.</p> <important> <p>Deleting a repository also deletes all associated objects and metadata. After a repository is deleted, all future push calls to the deleted repository will fail.</p> </important></p>
    fn delete_repository(
        &self,
        input: DeleteRepositoryInput,
    ) -> RusotoFuture<DeleteRepositoryOutput, DeleteRepositoryError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRepositoryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteRepositoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about one or more pull request events.</p>
    fn describe_pull_request_events(
        &self,
        input: DescribePullRequestEventsInput,
    ) -> RusotoFuture<DescribePullRequestEventsOutput, DescribePullRequestEventsError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.DescribePullRequestEvents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribePullRequestEventsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePullRequestEventsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the base-64 encoded content of an individual blob within a repository.</p>
    fn get_blob(&self, input: GetBlobInput) -> RusotoFuture<GetBlobOutput, GetBlobError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetBlob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetBlobOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBlobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a repository branch, including its name and the last commit ID.</p>
    fn get_branch(&self, input: GetBranchInput) -> RusotoFuture<GetBranchOutput, GetBranchError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetBranchOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBranchError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the content of a comment made on a change, file, or commit in a repository.</p>
    fn get_comment(
        &self,
        input: GetCommentInput,
    ) -> RusotoFuture<GetCommentOutput, GetCommentError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetComment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCommentOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCommentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about comments made on the comparison between two commits.</p>
    fn get_comments_for_compared_commit(
        &self,
        input: GetCommentsForComparedCommitInput,
    ) -> RusotoFuture<GetCommentsForComparedCommitOutput, GetCommentsForComparedCommitError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.GetCommentsForComparedCommit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCommentsForComparedCommitOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCommentsForComparedCommitError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns comments made on a pull request.</p>
    fn get_comments_for_pull_request(
        &self,
        input: GetCommentsForPullRequestInput,
    ) -> RusotoFuture<GetCommentsForPullRequestOutput, GetCommentsForPullRequestError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.GetCommentsForPullRequest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCommentsForPullRequestOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCommentsForPullRequestError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about a commit, including commit message and committer information.</p>
    fn get_commit(&self, input: GetCommitInput) -> RusotoFuture<GetCommitOutput, GetCommitError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetCommit");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCommitOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCommitError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID or other fully qualified reference). Results can be limited to a specified path.</p>
    fn get_differences(
        &self,
        input: GetDifferencesInput,
    ) -> RusotoFuture<GetDifferencesOutput, GetDifferencesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetDifferences");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDifferencesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDifferencesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about merge conflicts between the before and after commit IDs for a pull request in a repository.</p>
    fn get_merge_conflicts(
        &self,
        input: GetMergeConflictsInput,
    ) -> RusotoFuture<GetMergeConflictsOutput, GetMergeConflictsError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetMergeConflicts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetMergeConflictsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMergeConflictsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about a pull request in a specified repository.</p>
    fn get_pull_request(
        &self,
        input: GetPullRequestInput,
    ) -> RusotoFuture<GetPullRequestOutput, GetPullRequestError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetPullRequest");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPullRequestOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPullRequestError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Returns information about a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note></p>
    fn get_repository(
        &self,
        input: GetRepositoryInput,
    ) -> RusotoFuture<GetRepositoryOutput, GetRepositoryError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRepositoryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRepositoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about triggers configured for a repository.</p>
    fn get_repository_triggers(
        &self,
        input: GetRepositoryTriggersInput,
    ) -> RusotoFuture<GetRepositoryTriggersOutput, GetRepositoryTriggersError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetRepositoryTriggers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRepositoryTriggersOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetRepositoryTriggersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about one or more branches in a repository.</p>
    fn list_branches(
        &self,
        input: ListBranchesInput,
    ) -> RusotoFuture<ListBranchesOutput, ListBranchesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListBranches");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListBranchesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListBranchesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of pull requests for a specified repository. The return list can be refined by pull request status or pull request author ARN.</p>
    fn list_pull_requests(
        &self,
        input: ListPullRequestsInput,
    ) -> RusotoFuture<ListPullRequestsOutput, ListPullRequestsError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListPullRequests");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPullRequestsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPullRequestsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about one or more repositories.</p>
    fn list_repositories(
        &self,
        input: ListRepositoriesInput,
    ) -> RusotoFuture<ListRepositoriesOutput, ListRepositoriesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListRepositories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRepositoriesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRepositoriesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Closes a pull request and attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the fast-forward merge option.</p>
    fn merge_pull_request_by_fast_forward(
        &self,
        input: MergePullRequestByFastForwardInput,
    ) -> RusotoFuture<MergePullRequestByFastForwardOutput, MergePullRequestByFastForwardError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.MergePullRequestByFastForward",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<MergePullRequestByFastForwardOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(MergePullRequestByFastForwardError::from_response(response))
                }))
            }
        })
    }

    /// <p>Posts a comment on the comparison between two commits.</p>
    fn post_comment_for_compared_commit(
        &self,
        input: PostCommentForComparedCommitInput,
    ) -> RusotoFuture<PostCommentForComparedCommitOutput, PostCommentForComparedCommitError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.PostCommentForComparedCommit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PostCommentForComparedCommitOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PostCommentForComparedCommitError::from_response(response))
                }))
            }
        })
    }

    /// <p>Posts a comment on a pull request.</p>
    fn post_comment_for_pull_request(
        &self,
        input: PostCommentForPullRequestInput,
    ) -> RusotoFuture<PostCommentForPullRequestOutput, PostCommentForPullRequestError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.PostCommentForPullRequest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PostCommentForPullRequestOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PostCommentForPullRequestError::from_response(response))
                }))
            }
        })
    }

    /// <p>Posts a comment in reply to an existing comment on a comparison between commits or a pull request.</p>
    fn post_comment_reply(
        &self,
        input: PostCommentReplyInput,
    ) -> RusotoFuture<PostCommentReplyOutput, PostCommentReplyError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PostCommentReply");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PostCommentReplyOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PostCommentReplyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds or updates a file in an AWS CodeCommit repository.</p>
    fn put_file(&self, input: PutFileInput) -> RusotoFuture<PutFileOutput, PutFileError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PutFile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutFileOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutFileError::from_response(response))),
                )
            }
        })
    }

    /// <p>Replaces all triggers for a repository. This can be used to create or delete triggers.</p>
    fn put_repository_triggers(
        &self,
        input: PutRepositoryTriggersInput,
    ) -> RusotoFuture<PutRepositoryTriggersOutput, PutRepositoryTriggersError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PutRepositoryTriggers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRepositoryTriggersOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutRepositoryTriggersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test will send data from the last commit. If no data is available, sample data will be generated.</p>
    fn test_repository_triggers(
        &self,
        input: TestRepositoryTriggersInput,
    ) -> RusotoFuture<TestRepositoryTriggersOutput, TestRepositoryTriggersError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.TestRepositoryTriggers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TestRepositoryTriggersOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(TestRepositoryTriggersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Replaces the contents of a comment.</p>
    fn update_comment(
        &self,
        input: UpdateCommentInput,
    ) -> RusotoFuture<UpdateCommentOutput, UpdateCommentError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateComment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateCommentOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateCommentError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Sets or changes the default branch name for the specified repository.</p> <note> <p>If you use this operation to change the default branch name to the current default branch name, a success message is returned even though the default branch did not change.</p> </note></p>
    fn update_default_branch(
        &self,
        input: UpdateDefaultBranchInput,
    ) -> RusotoFuture<(), UpdateDefaultBranchError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateDefaultBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateDefaultBranchError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Replaces the contents of the description of a pull request.</p>
    fn update_pull_request_description(
        &self,
        input: UpdatePullRequestDescriptionInput,
    ) -> RusotoFuture<UpdatePullRequestDescriptionOutput, UpdatePullRequestDescriptionError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdatePullRequestDescription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePullRequestDescriptionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdatePullRequestDescriptionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the status of a pull request. </p>
    fn update_pull_request_status(
        &self,
        input: UpdatePullRequestStatusInput,
    ) -> RusotoFuture<UpdatePullRequestStatusOutput, UpdatePullRequestStatusError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdatePullRequestStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePullRequestStatusOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdatePullRequestStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Replaces the title of a pull request.</p>
    fn update_pull_request_title(
        &self,
        input: UpdatePullRequestTitleInput,
    ) -> RusotoFuture<UpdatePullRequestTitleOutput, UpdatePullRequestTitleError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdatePullRequestTitle");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePullRequestTitleOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdatePullRequestTitleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Sets or changes the comment or description for a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note></p>
    fn update_repository_description(
        &self,
        input: UpdateRepositoryDescriptionInput,
    ) -> RusotoFuture<(), UpdateRepositoryDescriptionError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdateRepositoryDescription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateRepositoryDescriptionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Renames a repository. The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix ".git" is prohibited. For a full description of the limits on repository names, see <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide.</p>
    fn update_repository_name(
        &self,
        input: UpdateRepositoryNameInput,
    ) -> RusotoFuture<(), UpdateRepositoryNameError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateRepositoryName");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRepositoryNameError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
