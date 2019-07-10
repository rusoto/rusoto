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
use rusoto_core::compression::CompressRequestPayload;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Represents the input of a batch get repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetRepositoriesInput {
    /// <p>The names of the repositories to get information about.</p>
    #[serde(rename = "repositoryNames")]
    pub repository_names: Vec<String>,
}

/// <p>Represents the output of a batch get repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct CreateCommitInput {
    /// <p>The name of the author who created the commit. This information will be used as both the author and committer for the commit.</p>
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// <p>The name of the branch where you will create the commit.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>The commit message you want to include as part of creating the commit. Commit messages are limited to 256 KB. If no message is specified, a default message will be used.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The files to delete in this commit. These files will still exist in prior commits.</p>
    #[serde(rename = "deleteFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<Vec<DeleteFileEntry>>,
    /// <p>The email address of the person who created the commit.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If this is specified as true, a .gitkeep file will be created for empty folders.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    /// <p>The ID of the commit that is the parent of the commit you will create. If this is an empty repository, this is not required.</p>
    #[serde(rename = "parentCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_commit_id: Option<String>,
    /// <p>The files to add or update in this commit.</p>
    #[serde(rename = "putFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_files: Option<Vec<PutFileEntry>>,
    /// <p>The name of the repository where you will create the commit.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The file modes to update for files in this commit.</p>
    #[serde(rename = "setFileModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_file_modes: Option<Vec<SetFileModeEntry>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCommitOutput {
    /// <p>The full commit ID of the commit that contains your committed file changes.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>The files added as part of the committed file changes.</p>
    #[serde(rename = "filesAdded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_added: Option<Vec<FileMetadata>>,
    /// <p>The files deleted as part of the committed file changes.</p>
    #[serde(rename = "filesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_deleted: Option<Vec<FileMetadata>>,
    /// <p>The files updated as part of the commited file changes.</p>
    #[serde(rename = "filesUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_updated: Option<Vec<FileMetadata>>,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains the commited file changes.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p><p>The name of the new repository to be created.</p> <note> <p>The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. For a full description of the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide. The suffix &quot;.git&quot; is prohibited.</p> </note></p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>One or more tag key-value pairs to use when tagging this repository.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents the output of a create repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCommentContentOutput {
    /// <p>Information about the comment you just deleted.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>A file that will be deleted as part of a commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFileEntry {
    /// <p>The full path of the file that will be deleted, including the name of the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFileInput {
    /// <p>The name of the branch where the commit will be made deleting the file.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>The commit message you want to include as part of deleting the file. Commit messages are limited to 256 KB. If no message is specified, a default message will be used.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The email address for the commit that deletes the file. If no email address is specified, the email address will be left blank.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The fully-qualified path to the file that will be deleted, including the full name and extension of that file. For example, /examples/file.md is a fully qualified path to a file named file.md in a folder named examples.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>Specifies whether to delete the folder or directory that contains the file you want to delete if that file is the only object in the folder or directory. By default, empty folders will be deleted. This includes empty folders that are part of the directory structure. For example, if the path to a file is dir1/dir2/dir3/dir4, and dir2 and dir3 are empty, deleting the last file in dir4 will also delete the empty folders dir4, dir3, and dir2.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    /// <p>The name of the author of the commit that deletes the file. If no name is specified, the user's ARN will be used as the author name and committer name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the commit that is the tip of the branch where you want to create the commit that will delete the file. This must be the HEAD commit for the branch. The commit that deletes the file will be created from this commit ID.</p>
    #[serde(rename = "parentCommitId")]
    pub parent_commit_id: String,
    /// <p>The name of the repository that contains the file to delete.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteFileOutput {
    /// <p>The blob ID removed from the tree as part of deleting the file.</p>
    #[serde(rename = "blobId")]
    pub blob_id: String,
    /// <p>The full commit ID of the commit that contains the change that deletes the file.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The fully-qualified path to the file that will be deleted, including the full name and extension of that file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains the delete file change.</p>
    #[serde(rename = "treeId")]
    pub tree_id: String,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Returns information about a file in a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct File {
    /// <p>The fully-qualified path to the file in the repository.</p>
    #[serde(rename = "absolutePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    /// <p>The blob ID that contains the file information.</p>
    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    /// <p>The extrapolated file mode permissions for the file. Valid values include EXECUTABLE and NORMAL.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p>The relative path of the file from the folder where the query originated.</p>
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

/// <p>A file that will be added, updated, or deleted as part of a commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FileMetadata {
    /// <p>The full path to the file that will be added or updated, including the name of the file.</p>
    #[serde(rename = "absolutePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    /// <p>The blob ID that contains the file information.</p>
    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    /// <p>The extrapolated file mode permissions for the file. Valid values include EXECUTABLE and NORMAL.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
}

/// <p>Returns information about a folder in a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Folder {
    /// <p>The fully-qualified path of the folder in the repository.</p>
    #[serde(rename = "absolutePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    /// <p>The relative path of the specified folder from the folder where the query originated.</p>
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains the folder.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct GetBlobOutput {
    /// <p>The content of the blob, usually a file.</p>
    #[serde(rename = "content")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub content: bytes::Bytes,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct GetFileInput {
    /// <p>The fully-quaified reference that identifies the commit that contains the file. For example, you could specify a full commit ID, a tag, a branch name, or a reference such as refs/heads/master. If none is provided, then the head commit will be used.</p>
    #[serde(rename = "commitSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_specifier: Option<String>,
    /// <p>The fully-qualified path to the file, including the full name and extension of the file. For example, /examples/file.md is the fully-qualified path to a file named file.md in a folder named examples.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The name of the repository that contains the file.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFileOutput {
    /// <p>The blob ID of the object that represents the file content.</p>
    #[serde(rename = "blobId")]
    pub blob_id: String,
    /// <p>The full commit ID of the commit that contains the content returned by GetFile.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The base-64 encoded binary data object that represents the content of the file.</p>
    #[serde(rename = "fileContent")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub file_content: bytes::Bytes,
    /// <p><p>The extrapolated file mode permissions of the blob. Valid values include strings such as EXECUTABLE and not numeric values.</p> <note> <p>The file mode permissions returned by this API are not the standard file mode permission values, such as 100644, but rather extrapolated values. See below for a full list of supported return values.</p> </note></p>
    #[serde(rename = "fileMode")]
    pub file_mode: String,
    /// <p>The fully qualified path to the specified file. This returns the name and extension of the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The size of the contents of the file, in bytes.</p>
    #[serde(rename = "fileSize")]
    pub file_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFolderInput {
    /// <p>A fully-qualified reference used to identify a commit that contains the version of the folder's content to return. A fully-qualified reference can be a commit ID, branch name, tag, or reference such as HEAD. If no specifier is provided, the folder content will be returned as it exists in the HEAD commit.</p>
    #[serde(rename = "commitSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_specifier: Option<String>,
    /// <p>The fully-qualified path to the folder whose contents will be returned, including the folder name. For example, /examples is a fully-qualified path to a folder named examples that was created off of the root directory (/) of a repository. </p>
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    /// <p>The name of the repository.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFolderOutput {
    /// <p>The full commit ID used as a reference for which version of the folder content is returned.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The list of files that exist in the specified folder, if any.</p>
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    /// <p>The fully-qualified path of the folder whose contents are returned.</p>
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    /// <p>The list of folders that exist beneath the specified folder, if any.</p>
    #[serde(rename = "subFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_folders: Option<Vec<Folder>>,
    /// <p>The list of submodules that exist in the specified folder, if any.</p>
    #[serde(rename = "subModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_modules: Option<Vec<SubModule>>,
    /// <p>The list of symbolic links to other files and folders that exist in the specified folder, if any.</p>
    #[serde(rename = "symbolicLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbolic_links: Option<Vec<SymbolicLink>>,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains the folder.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceInput {
    /// <p>An enumeration token that when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to get information about tags, if any.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>An enumeration token that allows the operation to batch the next results of the operation.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tag key and value pairs associated with the specified resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct PostCommentReplyOutput {
    /// <p>Information about the reply to a comment.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>Returns information about a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Metadata about the pull request that is used when comparing the pull request source with its destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PullRequestCreatedEventMetadata {
    /// <p>The commit ID of the tip of the branch specified as the destination branch when the pull request was created.</p>
    #[serde(rename = "destinationCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    /// <p>The commit ID of the most recent commit that the source branch and the destination branch have in common.</p>
    #[serde(rename = "mergeBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_base: Option<String>,
    /// <p>The name of the repository where the pull request was created.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p>The commit ID on the source branch used when the pull request was created.</p>
    #[serde(rename = "sourceCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

/// <p>Returns information about a pull request event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PullRequestEvent {
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with additional commits or changing the status of a pull request.</p>
    #[serde(rename = "actorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_arn: Option<String>,
    /// <p>The day and time of the pull request event, in timestamp format.</p>
    #[serde(rename = "eventDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<f64>,
    /// <p>Information about the source and destination branches for the pull request.</p>
    #[serde(rename = "pullRequestCreatedEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_created_event_metadata: Option<PullRequestCreatedEventMetadata>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct PullRequestSourceReferenceUpdatedEventMetadata {
    /// <p>The full commit ID of the commit in the source branch that was the tip of the branch at the time the pull request was updated.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full commit ID of the commit in the destination branch that was the tip of the branch at the time the pull request was updated.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>The commit ID of the most recent commit that the source branch and the destination branch have in common.</p>
    #[serde(rename = "mergeBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_base: Option<String>,
    /// <p>The name of the repository where the pull request was updated.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>Information about a change to the status of a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PullRequestStatusChangedEventMetadata {
    /// <p>The changed status of the pull request.</p>
    #[serde(rename = "pullRequestStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
}

/// <p>Returns information about a pull request target.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PullRequestTarget {
    /// <p>The full commit ID that is the tip of the destination branch. This is the commit where the pull request was or will be merged.</p>
    #[serde(rename = "destinationCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit: Option<String>,
    /// <p>The branch of the repository where the pull request changes will be merged into. Also known as the destination branch. </p>
    #[serde(rename = "destinationReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_reference: Option<String>,
    /// <p>The commit ID of the most recent commit that the source branch and the destination branch have in common.</p>
    #[serde(rename = "mergeBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_base: Option<String>,
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

/// <p>Information about a file that will be added or updated as part of a commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutFileEntry {
    /// <p>The content of the file, if a source file is not specified.</p>
    #[serde(rename = "fileContent")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content: Option<bytes::Bytes>,
    /// <p>The extrapolated file mode permissions for the file. Valid values include EXECUTABLE and NORMAL.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p>The full path to the file in the repository, including the name of the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The name and full path of the file that contains the changes you want to make as part of the commit, if you are not providing the file content directly.</p>
    #[serde(rename = "sourceFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file: Option<SourceFileSpecifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutFileInput {
    /// <p>The name of the branch where you want to add or update the file. If this is an empty repository, this branch will be created.</p>
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
    pub file_content: bytes::Bytes,
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
    /// <p>The full commit ID of the head commit in the branch where you want to add or update the file. If this is an empty repository, no commit ID is required. If this is not an empty repository, a commit ID is required. </p> <p>The commit ID must match the ID of the head commit at the time of the operation, or an error will occur, and the file will not be added or updated.</p>
    #[serde(rename = "parentCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_commit_id: Option<String>,
    /// <p>The name of the repository where you want to add or update the file.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutFileOutput {
    /// <p>The ID of the blob, which is its SHA-1 pointer.</p>
    #[serde(rename = "blobId")]
    pub blob_id: String,
    /// <p>The full SHA of the commit that contains this file change.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains this file change.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct PutRepositoryTriggersOutput {
    /// <p>The system-generated unique ID for the create or update operation.</p>
    #[serde(rename = "configurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

/// <p>Information about a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Information about the file mode changes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetFileModeEntry {
    /// <p>The file mode for the file.</p>
    #[serde(rename = "fileMode")]
    pub file_mode: String,
    /// <p>The full path to the file, including the name of the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
}

/// <p>Information about a source file that is part of changes made in a commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SourceFileSpecifier {
    /// <p>The full path to the file, including the name of the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>Whether to remove the source file from the parent commit.</p>
    #[serde(rename = "isMove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_move: Option<bool>,
}

/// <p>Returns information about a submodule reference in a repository folder.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubModule {
    /// <p>The fully qualified path to the folder that contains the reference to the submodule.</p>
    #[serde(rename = "absolutePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    /// <p>The commit ID that contains the reference to the submodule.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>The relative path of the submodule from the folder where the query originated.</p>
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

/// <p>Returns information about a symbolic link in a repository folder.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SymbolicLink {
    /// <p>The fully-qualified path to the folder that contains the symbolic link.</p>
    #[serde(rename = "absolutePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    /// <p>The blob ID that contains the information about the symbolic link.</p>
    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    /// <p>The file mode permissions of the blob that cotains information about the symbolic link.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p>The relative path of the symbolic link from the folder where the query originated.</p>
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to which you want to add or update tags.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The key-value pair to use when tagging this repository.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to which you want to remove tags.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag key for each tag that you want to remove from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
}

impl BatchGetRepositoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetRepositoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        BatchGetRepositoriesError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        BatchGetRepositoriesError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(BatchGetRepositoriesError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(BatchGetRepositoriesError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        BatchGetRepositoriesError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(BatchGetRepositoriesError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "MaximumRepositoryNamesExceededException" => {
                    return RusotoError::Service(
                        BatchGetRepositoriesError::MaximumRepositoryNamesExceeded(err.msg),
                    )
                }
                "RepositoryNamesRequiredException" => {
                    return RusotoError::Service(
                        BatchGetRepositoriesError::RepositoryNamesRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBranchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchNameExistsException" => {
                    return RusotoError::Service(CreateBranchError::BranchNameExists(err.msg))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(CreateBranchError::BranchNameRequired(err.msg))
                }
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(CreateBranchError::CommitDoesNotExist(err.msg))
                }
                "CommitIdRequiredException" => {
                    return RusotoError::Service(CreateBranchError::CommitIdRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        CreateBranchError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(CreateBranchError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(CreateBranchError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(CreateBranchError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(CreateBranchError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(CreateBranchError::InvalidBranchName(err.msg))
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(CreateBranchError::InvalidCommitId(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(CreateBranchError::InvalidRepositoryName(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(CreateBranchError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(CreateBranchError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateCommit
#[derive(Debug, PartialEq)]
pub enum CreateCommitError {
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
    /// <p>The commit cannot be created because both a source file and file content have been specified for the same file. You cannot provide both. Either specify a source file, or provide the file content directly.</p>
    FileContentAndSourceFileSpecified(String),
    /// <p>The file cannot be added because it is too large. The maximum file size that can be added using PutFile is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>The specified file does not exist. Verify that you have provided the correct name of the file, including its full path and extension.</p>
    FileDoesNotExist(String),
    /// <p>The commit cannot be created because no files have been specified as added, updated, or changed (PutFile or DeleteFile) for the commit.</p>
    FileEntryRequired(String),
    /// <p>The commit cannot be created because a file mode is required to update mode permissions for an existing file, but no file mode has been specified.</p>
    FileModeRequired(String),
    /// <p>A file cannot be added to the repository because the specified file name has the same name as a directory in this repository. Either provide another name for the file, or add the file in a directory that does not match the file name.</p>
    FileNameConflictsWithDirectoryName(String),
    /// <p>The commit cannot be created because a specified file path points to a submodule. Verify that the destination files have valid file paths that do not point to a submodule.</p>
    FilePathConflictsWithSubmodulePath(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit result in a folder contents exceeding the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
    FolderContentSizeLimitExceeded(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified deletion parameter is not valid.</p>
    InvalidDeletionParameter(String),
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
    /// <p>The number of specified files to change as part of this commit exceeds the maximum number of files that can be changed in a single commit. Consider using a Git client for these changes.</p>
    MaximumFileEntriesExceeded(String),
    /// <p>The user name is not valid because it has exceeded the character limit for file names. File names, including the path to the file, cannot exceed the character limit. </p>
    NameLengthExceeded(String),
    /// <p>The commit cannot be created because no changes will be made to the repository as a result of this commit. A commit must contain at least one change.</p>
    NoChange(String),
    /// <p>The parent commit ID is not valid because it does not exist. The specified parent commit ID does not exist in the specified branch of the repository.</p>
    ParentCommitDoesNotExist(String),
    /// <p>The file could not be added because the provided parent commit ID is not the current tip of the specified branch. To view the full commit ID of the current head of the branch, use <a>GetBranch</a>.</p>
    ParentCommitIdOutdated(String),
    /// <p>A parent commit ID is required. To view the full commit ID of a branch in a repository, use <a>GetBranch</a> or a Git command (for example, git pull or git log).</p>
    ParentCommitIdRequired(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The commit cannot be created because one or more files specified in the commit reference both a file and a folder.</p>
    PutFileEntryConflict(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The commit cannot be created because one of the changes specifies copying or moving a .gitkeep file.</p>
    RestrictedSourceFile(String),
    /// <p>The commit cannot be created because one or more changes in this commit duplicate actions in the same file path. For example, you cannot make the same delete request to the same file in the same file path twice, or make a delete request and a move request to the same file as part of the same commit.</p>
    SamePathRequest(String),
    /// <p>The commit cannot be created because no source files or file content have been specified for the commit.</p>
    SourceFileOrContentRequired(String),
}

impl CreateCommitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCommitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(CreateCommitError::BranchDoesNotExist(err.msg))
                }
                "BranchNameIsTagNameException" => {
                    return RusotoError::Service(CreateCommitError::BranchNameIsTagName(err.msg))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(CreateCommitError::BranchNameRequired(err.msg))
                }
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(CreateCommitError::CommitMessageLengthExceeded(
                        err.msg,
                    ))
                }
                "DirectoryNameConflictsWithFileNameException" => {
                    return RusotoError::Service(
                        CreateCommitError::DirectoryNameConflictsWithFileName(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        CreateCommitError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(CreateCommitError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(CreateCommitError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(CreateCommitError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(CreateCommitError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "FileContentAndSourceFileSpecifiedException" => {
                    return RusotoError::Service(
                        CreateCommitError::FileContentAndSourceFileSpecified(err.msg),
                    )
                }
                "FileContentSizeLimitExceededException" => {
                    return RusotoError::Service(CreateCommitError::FileContentSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "FileDoesNotExistException" => {
                    return RusotoError::Service(CreateCommitError::FileDoesNotExist(err.msg))
                }
                "FileEntryRequiredException" => {
                    return RusotoError::Service(CreateCommitError::FileEntryRequired(err.msg))
                }
                "FileModeRequiredException" => {
                    return RusotoError::Service(CreateCommitError::FileModeRequired(err.msg))
                }
                "FileNameConflictsWithDirectoryNameException" => {
                    return RusotoError::Service(
                        CreateCommitError::FileNameConflictsWithDirectoryName(err.msg),
                    )
                }
                "FilePathConflictsWithSubmodulePathException" => {
                    return RusotoError::Service(
                        CreateCommitError::FilePathConflictsWithSubmodulePath(err.msg),
                    )
                }
                "FolderContentSizeLimitExceededException" => {
                    return RusotoError::Service(CreateCommitError::FolderContentSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(CreateCommitError::InvalidBranchName(err.msg))
                }
                "InvalidDeletionParameterException" => {
                    return RusotoError::Service(CreateCommitError::InvalidDeletionParameter(
                        err.msg,
                    ))
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(CreateCommitError::InvalidEmail(err.msg))
                }
                "InvalidFileModeException" => {
                    return RusotoError::Service(CreateCommitError::InvalidFileMode(err.msg))
                }
                "InvalidParentCommitIdException" => {
                    return RusotoError::Service(CreateCommitError::InvalidParentCommitId(err.msg))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(CreateCommitError::InvalidPath(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(CreateCommitError::InvalidRepositoryName(err.msg))
                }
                "MaximumFileEntriesExceededException" => {
                    return RusotoError::Service(CreateCommitError::MaximumFileEntriesExceeded(
                        err.msg,
                    ))
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(CreateCommitError::NameLengthExceeded(err.msg))
                }
                "NoChangeException" => {
                    return RusotoError::Service(CreateCommitError::NoChange(err.msg))
                }
                "ParentCommitDoesNotExistException" => {
                    return RusotoError::Service(CreateCommitError::ParentCommitDoesNotExist(
                        err.msg,
                    ))
                }
                "ParentCommitIdOutdatedException" => {
                    return RusotoError::Service(CreateCommitError::ParentCommitIdOutdated(err.msg))
                }
                "ParentCommitIdRequiredException" => {
                    return RusotoError::Service(CreateCommitError::ParentCommitIdRequired(err.msg))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(CreateCommitError::PathRequired(err.msg))
                }
                "PutFileEntryConflictException" => {
                    return RusotoError::Service(CreateCommitError::PutFileEntryConflict(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(CreateCommitError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(CreateCommitError::RepositoryNameRequired(err.msg))
                }
                "RestrictedSourceFileException" => {
                    return RusotoError::Service(CreateCommitError::RestrictedSourceFile(err.msg))
                }
                "SamePathRequestException" => {
                    return RusotoError::Service(CreateCommitError::SamePathRequest(err.msg))
                }
                "SourceFileOrContentRequiredException" => {
                    return RusotoError::Service(CreateCommitError::SourceFileOrContentRequired(
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
impl fmt::Display for CreateCommitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCommitError {
    fn description(&self) -> &str {
        match *self {
            CreateCommitError::BranchDoesNotExist(ref cause) => cause,
            CreateCommitError::BranchNameIsTagName(ref cause) => cause,
            CreateCommitError::BranchNameRequired(ref cause) => cause,
            CreateCommitError::CommitMessageLengthExceeded(ref cause) => cause,
            CreateCommitError::DirectoryNameConflictsWithFileName(ref cause) => cause,
            CreateCommitError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            CreateCommitError::EncryptionKeyAccessDenied(ref cause) => cause,
            CreateCommitError::EncryptionKeyDisabled(ref cause) => cause,
            CreateCommitError::EncryptionKeyNotFound(ref cause) => cause,
            CreateCommitError::EncryptionKeyUnavailable(ref cause) => cause,
            CreateCommitError::FileContentAndSourceFileSpecified(ref cause) => cause,
            CreateCommitError::FileContentSizeLimitExceeded(ref cause) => cause,
            CreateCommitError::FileDoesNotExist(ref cause) => cause,
            CreateCommitError::FileEntryRequired(ref cause) => cause,
            CreateCommitError::FileModeRequired(ref cause) => cause,
            CreateCommitError::FileNameConflictsWithDirectoryName(ref cause) => cause,
            CreateCommitError::FilePathConflictsWithSubmodulePath(ref cause) => cause,
            CreateCommitError::FolderContentSizeLimitExceeded(ref cause) => cause,
            CreateCommitError::InvalidBranchName(ref cause) => cause,
            CreateCommitError::InvalidDeletionParameter(ref cause) => cause,
            CreateCommitError::InvalidEmail(ref cause) => cause,
            CreateCommitError::InvalidFileMode(ref cause) => cause,
            CreateCommitError::InvalidParentCommitId(ref cause) => cause,
            CreateCommitError::InvalidPath(ref cause) => cause,
            CreateCommitError::InvalidRepositoryName(ref cause) => cause,
            CreateCommitError::MaximumFileEntriesExceeded(ref cause) => cause,
            CreateCommitError::NameLengthExceeded(ref cause) => cause,
            CreateCommitError::NoChange(ref cause) => cause,
            CreateCommitError::ParentCommitDoesNotExist(ref cause) => cause,
            CreateCommitError::ParentCommitIdOutdated(ref cause) => cause,
            CreateCommitError::ParentCommitIdRequired(ref cause) => cause,
            CreateCommitError::PathRequired(ref cause) => cause,
            CreateCommitError::PutFileEntryConflict(ref cause) => cause,
            CreateCommitError::RepositoryDoesNotExist(ref cause) => cause,
            CreateCommitError::RepositoryNameRequired(ref cause) => cause,
            CreateCommitError::RestrictedSourceFile(ref cause) => cause,
            CreateCommitError::SamePathRequest(ref cause) => cause,
            CreateCommitError::SourceFileOrContentRequired(ref cause) => cause,
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
}

impl CreatePullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePullRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientRequestTokenRequiredException" => {
                    return RusotoError::Service(
                        CreatePullRequestError::ClientRequestTokenRequired(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        CreatePullRequestError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(CreatePullRequestError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(CreatePullRequestError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(CreatePullRequestError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(CreatePullRequestError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "IdempotencyParameterMismatchException" => {
                    return RusotoError::Service(
                        CreatePullRequestError::IdempotencyParameterMismatch(err.msg),
                    )
                }
                "InvalidClientRequestTokenException" => {
                    return RusotoError::Service(CreatePullRequestError::InvalidClientRequestToken(
                        err.msg,
                    ))
                }
                "InvalidDescriptionException" => {
                    return RusotoError::Service(CreatePullRequestError::InvalidDescription(
                        err.msg,
                    ))
                }
                "InvalidReferenceNameException" => {
                    return RusotoError::Service(CreatePullRequestError::InvalidReferenceName(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(CreatePullRequestError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "InvalidTargetException" => {
                    return RusotoError::Service(CreatePullRequestError::InvalidTarget(err.msg))
                }
                "InvalidTargetsException" => {
                    return RusotoError::Service(CreatePullRequestError::InvalidTargets(err.msg))
                }
                "InvalidTitleException" => {
                    return RusotoError::Service(CreatePullRequestError::InvalidTitle(err.msg))
                }
                "MaximumOpenPullRequestsExceededException" => {
                    return RusotoError::Service(
                        CreatePullRequestError::MaximumOpenPullRequestsExceeded(err.msg),
                    )
                }
                "MultipleRepositoriesInPullRequestException" => {
                    return RusotoError::Service(
                        CreatePullRequestError::MultipleRepositoriesInPullRequest(err.msg),
                    )
                }
                "ReferenceDoesNotExistException" => {
                    return RusotoError::Service(CreatePullRequestError::ReferenceDoesNotExist(
                        err.msg,
                    ))
                }
                "ReferenceNameRequiredException" => {
                    return RusotoError::Service(CreatePullRequestError::ReferenceNameRequired(
                        err.msg,
                    ))
                }
                "ReferenceTypeNotSupportedException" => {
                    return RusotoError::Service(CreatePullRequestError::ReferenceTypeNotSupported(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(CreatePullRequestError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(CreatePullRequestError::RepositoryNameRequired(
                        err.msg,
                    ))
                }
                "SourceAndDestinationAreSameException" => {
                    return RusotoError::Service(
                        CreatePullRequestError::SourceAndDestinationAreSame(err.msg),
                    )
                }
                "TargetRequiredException" => {
                    return RusotoError::Service(CreatePullRequestError::TargetRequired(err.msg))
                }
                "TargetsRequiredException" => {
                    return RusotoError::Service(CreatePullRequestError::TargetsRequired(err.msg))
                }
                "TitleRequiredException" => {
                    return RusotoError::Service(CreatePullRequestError::TitleRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The specified tag is not valid. Key names cannot be prefixed with aws:.</p>
    InvalidSystemTagUsage(String),
    /// <p>The map of tags is not valid.</p>
    InvalidTagsMap(String),
    /// <p>A repository resource limit was exceeded.</p>
    RepositoryLimitExceeded(String),
    /// <p>The specified repository name already exists.</p>
    RepositoryNameExists(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The tag policy is not valid.</p>
    TagPolicy(String),
    /// <p>The maximum number of tags for an AWS CodeCommit resource has been exceeded.</p>
    TooManyTags(String),
}

impl CreateRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        CreateRepositoryError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(CreateRepositoryError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(CreateRepositoryError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(CreateRepositoryError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(CreateRepositoryError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidRepositoryDescriptionException" => {
                    return RusotoError::Service(
                        CreateRepositoryError::InvalidRepositoryDescription(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(CreateRepositoryError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "InvalidSystemTagUsageException" => {
                    return RusotoError::Service(CreateRepositoryError::InvalidSystemTagUsage(
                        err.msg,
                    ))
                }
                "InvalidTagsMapException" => {
                    return RusotoError::Service(CreateRepositoryError::InvalidTagsMap(err.msg))
                }
                "RepositoryLimitExceededException" => {
                    return RusotoError::Service(CreateRepositoryError::RepositoryLimitExceeded(
                        err.msg,
                    ))
                }
                "RepositoryNameExistsException" => {
                    return RusotoError::Service(CreateRepositoryError::RepositoryNameExists(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(CreateRepositoryError::RepositoryNameRequired(
                        err.msg,
                    ))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(CreateRepositoryError::TagPolicy(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateRepositoryError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateRepositoryError::InvalidSystemTagUsage(ref cause) => cause,
            CreateRepositoryError::InvalidTagsMap(ref cause) => cause,
            CreateRepositoryError::RepositoryLimitExceeded(ref cause) => cause,
            CreateRepositoryError::RepositoryNameExists(ref cause) => cause,
            CreateRepositoryError::RepositoryNameRequired(ref cause) => cause,
            CreateRepositoryError::TagPolicy(ref cause) => cause,
            CreateRepositoryError::TooManyTags(ref cause) => cause,
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
}

impl DeleteBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBranchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchNameRequiredException" => {
                    return RusotoError::Service(DeleteBranchError::BranchNameRequired(err.msg))
                }
                "DefaultBranchCannotBeDeletedException" => {
                    return RusotoError::Service(DeleteBranchError::DefaultBranchCannotBeDeleted(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        DeleteBranchError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(DeleteBranchError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(DeleteBranchError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(DeleteBranchError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(DeleteBranchError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(DeleteBranchError::InvalidBranchName(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(DeleteBranchError::InvalidRepositoryName(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(DeleteBranchError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(DeleteBranchError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl DeleteCommentContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCommentContentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommentDeletedException" => {
                    return RusotoError::Service(DeleteCommentContentError::CommentDeleted(err.msg))
                }
                "CommentDoesNotExistException" => {
                    return RusotoError::Service(DeleteCommentContentError::CommentDoesNotExist(
                        err.msg,
                    ))
                }
                "CommentIdRequiredException" => {
                    return RusotoError::Service(DeleteCommentContentError::CommentIdRequired(
                        err.msg,
                    ))
                }
                "InvalidCommentIdException" => {
                    return RusotoError::Service(DeleteCommentContentError::InvalidCommentId(
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
        }
    }
}
/// Errors returned by DeleteFile
#[derive(Debug, PartialEq)]
pub enum DeleteFileError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Type the name of a current branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    /// <p>The commit message is too long. Provide a shorter string. </p>
    CommitMessageLengthExceeded(String),
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
    /// <p>The specified file does not exist. Verify that you have provided the correct name of the file, including its full path and extension.</p>
    FileDoesNotExist(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The parent commit ID is not valid. The commit ID cannot be empty, and must match the head commit ID for the branch of the repository where you want to add or update a file.</p>
    InvalidParentCommitId(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The user name is not valid because it has exceeded the character limit for file names. File names, including the path to the file, cannot exceed the character limit. </p>
    NameLengthExceeded(String),
    /// <p>The parent commit ID is not valid because it does not exist. The specified parent commit ID does not exist in the specified branch of the repository.</p>
    ParentCommitDoesNotExist(String),
    /// <p>The file could not be added because the provided parent commit ID is not the current tip of the specified branch. To view the full commit ID of the current head of the branch, use <a>GetBranch</a>.</p>
    ParentCommitIdOutdated(String),
    /// <p>A parent commit ID is required. To view the full commit ID of a branch in a repository, use <a>GetBranch</a> or a Git command (for example, git pull or git log).</p>
    ParentCommitIdRequired(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
}

impl DeleteFileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(DeleteFileError::BranchDoesNotExist(err.msg))
                }
                "BranchNameIsTagNameException" => {
                    return RusotoError::Service(DeleteFileError::BranchNameIsTagName(err.msg))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(DeleteFileError::BranchNameRequired(err.msg))
                }
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(DeleteFileError::CommitMessageLengthExceeded(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(DeleteFileError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(DeleteFileError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(DeleteFileError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(DeleteFileError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(DeleteFileError::EncryptionKeyUnavailable(err.msg))
                }
                "FileDoesNotExistException" => {
                    return RusotoError::Service(DeleteFileError::FileDoesNotExist(err.msg))
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(DeleteFileError::InvalidBranchName(err.msg))
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(DeleteFileError::InvalidEmail(err.msg))
                }
                "InvalidParentCommitIdException" => {
                    return RusotoError::Service(DeleteFileError::InvalidParentCommitId(err.msg))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(DeleteFileError::InvalidPath(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(DeleteFileError::InvalidRepositoryName(err.msg))
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(DeleteFileError::NameLengthExceeded(err.msg))
                }
                "ParentCommitDoesNotExistException" => {
                    return RusotoError::Service(DeleteFileError::ParentCommitDoesNotExist(err.msg))
                }
                "ParentCommitIdOutdatedException" => {
                    return RusotoError::Service(DeleteFileError::ParentCommitIdOutdated(err.msg))
                }
                "ParentCommitIdRequiredException" => {
                    return RusotoError::Service(DeleteFileError::ParentCommitIdRequired(err.msg))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(DeleteFileError::PathRequired(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(DeleteFileError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(DeleteFileError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFileError {
    fn description(&self) -> &str {
        match *self {
            DeleteFileError::BranchDoesNotExist(ref cause) => cause,
            DeleteFileError::BranchNameIsTagName(ref cause) => cause,
            DeleteFileError::BranchNameRequired(ref cause) => cause,
            DeleteFileError::CommitMessageLengthExceeded(ref cause) => cause,
            DeleteFileError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            DeleteFileError::EncryptionKeyAccessDenied(ref cause) => cause,
            DeleteFileError::EncryptionKeyDisabled(ref cause) => cause,
            DeleteFileError::EncryptionKeyNotFound(ref cause) => cause,
            DeleteFileError::EncryptionKeyUnavailable(ref cause) => cause,
            DeleteFileError::FileDoesNotExist(ref cause) => cause,
            DeleteFileError::InvalidBranchName(ref cause) => cause,
            DeleteFileError::InvalidEmail(ref cause) => cause,
            DeleteFileError::InvalidParentCommitId(ref cause) => cause,
            DeleteFileError::InvalidPath(ref cause) => cause,
            DeleteFileError::InvalidRepositoryName(ref cause) => cause,
            DeleteFileError::NameLengthExceeded(ref cause) => cause,
            DeleteFileError::ParentCommitDoesNotExist(ref cause) => cause,
            DeleteFileError::ParentCommitIdOutdated(ref cause) => cause,
            DeleteFileError::ParentCommitIdRequired(ref cause) => cause,
            DeleteFileError::PathRequired(ref cause) => cause,
            DeleteFileError::RepositoryDoesNotExist(ref cause) => cause,
            DeleteFileError::RepositoryNameRequired(ref cause) => cause,
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
}

impl DeleteRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        DeleteRepositoryError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(DeleteRepositoryError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(DeleteRepositoryError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(DeleteRepositoryError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(DeleteRepositoryError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(DeleteRepositoryError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(DeleteRepositoryError::RepositoryNameRequired(
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
}

impl DescribePullRequestEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePullRequestEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActorDoesNotExistException" => {
                    return RusotoError::Service(DescribePullRequestEventsError::ActorDoesNotExist(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidActorArnException" => {
                    return RusotoError::Service(DescribePullRequestEventsError::InvalidActorArn(
                        err.msg,
                    ))
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::InvalidContinuationToken(err.msg),
                    )
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(DescribePullRequestEventsError::InvalidMaxResults(
                        err.msg,
                    ))
                }
                "InvalidPullRequestEventTypeException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::InvalidPullRequestEventType(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::InvalidPullRequestId(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        DescribePullRequestEventsError::PullRequestIdRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The specified file exceeds the file size limit for AWS CodeCommit. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    FileTooLarge(String),
    /// <p>The specified blob is not valid.</p>
    InvalidBlobId(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
}

impl GetBlobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBlobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BlobIdDoesNotExistException" => {
                    return RusotoError::Service(GetBlobError::BlobIdDoesNotExist(err.msg))
                }
                "BlobIdRequiredException" => {
                    return RusotoError::Service(GetBlobError::BlobIdRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(GetBlobError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetBlobError::EncryptionKeyAccessDenied(err.msg))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetBlobError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetBlobError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetBlobError::EncryptionKeyUnavailable(err.msg))
                }
                "FileTooLargeException" => {
                    return RusotoError::Service(GetBlobError::FileTooLarge(err.msg))
                }
                "InvalidBlobIdException" => {
                    return RusotoError::Service(GetBlobError::InvalidBlobId(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetBlobError::InvalidRepositoryName(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetBlobError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetBlobError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBranchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(GetBranchError::BranchDoesNotExist(err.msg))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(GetBranchError::BranchNameRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(GetBranchError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetBranchError::EncryptionKeyAccessDenied(err.msg))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetBranchError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetBranchError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetBranchError::EncryptionKeyUnavailable(err.msg))
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(GetBranchError::InvalidBranchName(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetBranchError::InvalidRepositoryName(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetBranchError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetBranchError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetCommentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCommentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommentDeletedException" => {
                    return RusotoError::Service(GetCommentError::CommentDeleted(err.msg))
                }
                "CommentDoesNotExistException" => {
                    return RusotoError::Service(GetCommentError::CommentDoesNotExist(err.msg))
                }
                "CommentIdRequiredException" => {
                    return RusotoError::Service(GetCommentError::CommentIdRequired(err.msg))
                }
                "InvalidCommentIdException" => {
                    return RusotoError::Service(GetCommentError::InvalidCommentId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetCommentsForComparedCommitError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCommentsForComparedCommitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::CommitDoesNotExist(err.msg),
                    )
                }
                "CommitIdRequiredException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::CommitIdRequired(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::InvalidCommitId(err.msg),
                    )
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::InvalidContinuationToken(err.msg),
                    )
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::InvalidMaxResults(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::InvalidRepositoryName(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        GetCommentsForComparedCommitError::RepositoryNameRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The repository does not contain any pull requests with that pull request ID. Use GetPullRequest to verify the correct repository name for the pull request ID.</p>
    RepositoryNotAssociatedWithPullRequest(String),
}

impl GetCommentsForPullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCommentsForPullRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::CommitDoesNotExist(err.msg),
                    )
                }
                "CommitIdRequiredException" => {
                    return RusotoError::Service(GetCommentsForPullRequestError::CommitIdRequired(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(GetCommentsForPullRequestError::InvalidCommitId(
                        err.msg,
                    ))
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::InvalidContinuationToken(err.msg),
                    )
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(GetCommentsForPullRequestError::InvalidMaxResults(
                        err.msg,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::InvalidRepositoryName(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::PullRequestIdRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::RepositoryNameRequired(err.msg),
                    )
                }
                "RepositoryNotAssociatedWithPullRequestException" => {
                    return RusotoError::Service(
                        GetCommentsForPullRequestError::RepositoryNotAssociatedWithPullRequest(
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
}

impl GetCommitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCommitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitIdDoesNotExistException" => {
                    return RusotoError::Service(GetCommitError::CommitIdDoesNotExist(err.msg))
                }
                "CommitIdRequiredException" => {
                    return RusotoError::Service(GetCommitError::CommitIdRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(GetCommitError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetCommitError::EncryptionKeyAccessDenied(err.msg))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetCommitError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetCommitError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetCommitError::EncryptionKeyUnavailable(err.msg))
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(GetCommitError::InvalidCommitId(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetCommitError::InvalidRepositoryName(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetCommitError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetCommitError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetDifferencesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDifferencesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(GetDifferencesError::CommitDoesNotExist(err.msg))
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(GetDifferencesError::CommitRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetDifferencesError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetDifferencesError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetDifferencesError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetDifferencesError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetDifferencesError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(GetDifferencesError::InvalidCommit(err.msg))
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(GetDifferencesError::InvalidCommitId(err.msg))
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(GetDifferencesError::InvalidContinuationToken(
                        err.msg,
                    ))
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(GetDifferencesError::InvalidMaxResults(err.msg))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(GetDifferencesError::InvalidPath(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetDifferencesError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "PathDoesNotExistException" => {
                    return RusotoError::Service(GetDifferencesError::PathDoesNotExist(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetDifferencesError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetDifferencesError::RepositoryNameRequired(
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
        }
    }
}
/// Errors returned by GetFile
#[derive(Debug, PartialEq)]
pub enum GetFileError {
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
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
    /// <p>The specified file does not exist. Verify that you have provided the correct name of the file, including its full path and extension.</p>
    FileDoesNotExist(String),
    /// <p>The specified file exceeds the file size limit for AWS CodeCommit. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    FileTooLarge(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
}

impl GetFileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(GetFileError::CommitDoesNotExist(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(GetFileError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetFileError::EncryptionKeyAccessDenied(err.msg))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetFileError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetFileError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetFileError::EncryptionKeyUnavailable(err.msg))
                }
                "FileDoesNotExistException" => {
                    return RusotoError::Service(GetFileError::FileDoesNotExist(err.msg))
                }
                "FileTooLargeException" => {
                    return RusotoError::Service(GetFileError::FileTooLarge(err.msg))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(GetFileError::InvalidCommit(err.msg))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(GetFileError::InvalidPath(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetFileError::InvalidRepositoryName(err.msg))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(GetFileError::PathRequired(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetFileError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetFileError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFileError {
    fn description(&self) -> &str {
        match *self {
            GetFileError::CommitDoesNotExist(ref cause) => cause,
            GetFileError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetFileError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetFileError::EncryptionKeyDisabled(ref cause) => cause,
            GetFileError::EncryptionKeyNotFound(ref cause) => cause,
            GetFileError::EncryptionKeyUnavailable(ref cause) => cause,
            GetFileError::FileDoesNotExist(ref cause) => cause,
            GetFileError::FileTooLarge(ref cause) => cause,
            GetFileError::InvalidCommit(ref cause) => cause,
            GetFileError::InvalidPath(ref cause) => cause,
            GetFileError::InvalidRepositoryName(ref cause) => cause,
            GetFileError::PathRequired(ref cause) => cause,
            GetFileError::RepositoryDoesNotExist(ref cause) => cause,
            GetFileError::RepositoryNameRequired(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFolder
#[derive(Debug, PartialEq)]
pub enum GetFolderError {
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
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
    /// <p>The specified folder does not exist. Either the folder name is not correct, or you did not provide the full path to the folder.</p>
    FolderDoesNotExist(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
}

impl GetFolderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFolderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(GetFolderError::CommitDoesNotExist(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(GetFolderError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetFolderError::EncryptionKeyAccessDenied(err.msg))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetFolderError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetFolderError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetFolderError::EncryptionKeyUnavailable(err.msg))
                }
                "FolderDoesNotExistException" => {
                    return RusotoError::Service(GetFolderError::FolderDoesNotExist(err.msg))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(GetFolderError::InvalidCommit(err.msg))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(GetFolderError::InvalidPath(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetFolderError::InvalidRepositoryName(err.msg))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(GetFolderError::PathRequired(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetFolderError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetFolderError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFolderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFolderError {
    fn description(&self) -> &str {
        match *self {
            GetFolderError::CommitDoesNotExist(ref cause) => cause,
            GetFolderError::EncryptionIntegrityChecksFailed(ref cause) => cause,
            GetFolderError::EncryptionKeyAccessDenied(ref cause) => cause,
            GetFolderError::EncryptionKeyDisabled(ref cause) => cause,
            GetFolderError::EncryptionKeyNotFound(ref cause) => cause,
            GetFolderError::EncryptionKeyUnavailable(ref cause) => cause,
            GetFolderError::FolderDoesNotExist(ref cause) => cause,
            GetFolderError::InvalidCommit(ref cause) => cause,
            GetFolderError::InvalidPath(ref cause) => cause,
            GetFolderError::InvalidRepositoryName(ref cause) => cause,
            GetFolderError::PathRequired(ref cause) => cause,
            GetFolderError::RepositoryDoesNotExist(ref cause) => cause,
            GetFolderError::RepositoryNameRequired(ref cause) => cause,
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
}

impl GetMergeConflictsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMergeConflictsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(GetMergeConflictsError::CommitDoesNotExist(
                        err.msg,
                    ))
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(GetMergeConflictsError::CommitRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetMergeConflictsError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetMergeConflictsError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetMergeConflictsError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetMergeConflictsError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(GetMergeConflictsError::InvalidCommit(err.msg))
                }
                "InvalidDestinationCommitSpecifierException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::InvalidDestinationCommitSpecifier(err.msg),
                    )
                }
                "InvalidMergeOptionException" => {
                    return RusotoError::Service(GetMergeConflictsError::InvalidMergeOption(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetMergeConflictsError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "InvalidSourceCommitSpecifierException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::InvalidSourceCommitSpecifier(err.msg),
                    )
                }
                "MergeOptionRequiredException" => {
                    return RusotoError::Service(GetMergeConflictsError::MergeOptionRequired(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetMergeConflictsError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetMergeConflictsError::RepositoryNameRequired(
                        err.msg,
                    ))
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(GetMergeConflictsError::TipsDivergenceExceeded(
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
}

impl GetPullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPullRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetPullRequestError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetPullRequestError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetPullRequestError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetPullRequestError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetPullRequestError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(GetPullRequestError::InvalidPullRequestId(err.msg))
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(GetPullRequestError::PullRequestDoesNotExist(
                        err.msg,
                    ))
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(GetPullRequestError::PullRequestIdRequired(
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
}

impl GetRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetRepositoryError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetRepositoryError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetRepositoryError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetRepositoryError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetRepositoryError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetRepositoryError::InvalidRepositoryName(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetRepositoryError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetRepositoryError::RepositoryNameRequired(
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
}

impl GetRepositoryTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRepositoryTriggersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetRepositoryTriggersError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        GetRepositoryTriggersError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetRepositoryTriggersError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetRepositoryTriggersError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        GetRepositoryTriggersError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetRepositoryTriggersError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        GetRepositoryTriggersError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        GetRepositoryTriggersError::RepositoryNameRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListBranchesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBranchesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        ListBranchesError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(ListBranchesError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(ListBranchesError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(ListBranchesError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(ListBranchesError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(ListBranchesError::InvalidContinuationToken(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(ListBranchesError::InvalidRepositoryName(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(ListBranchesError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(ListBranchesError::RepositoryNameRequired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListPullRequestsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPullRequestsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorDoesNotExistException" => {
                    return RusotoError::Service(ListPullRequestsError::AuthorDoesNotExist(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        ListPullRequestsError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(ListPullRequestsError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(ListPullRequestsError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(ListPullRequestsError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(ListPullRequestsError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidAuthorArnException" => {
                    return RusotoError::Service(ListPullRequestsError::InvalidAuthorArn(err.msg))
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(ListPullRequestsError::InvalidContinuationToken(
                        err.msg,
                    ))
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(ListPullRequestsError::InvalidMaxResults(err.msg))
                }
                "InvalidPullRequestStatusException" => {
                    return RusotoError::Service(ListPullRequestsError::InvalidPullRequestStatus(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(ListPullRequestsError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(ListPullRequestsError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(ListPullRequestsError::RepositoryNameRequired(
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
}

impl ListRepositoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRepositoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(ListRepositoriesError::InvalidContinuationToken(
                        err.msg,
                    ))
                }
                "InvalidOrderException" => {
                    return RusotoError::Service(ListRepositoriesError::InvalidOrder(err.msg))
                }
                "InvalidSortByException" => {
                    return RusotoError::Service(ListRepositoriesError::InvalidSortBy(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The value for the resource ARN is not valid. For more information about resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    InvalidResourceArn(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A valid Amazon Resource Name (ARN) for an AWS CodeCommit resource is required. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    ResourceArnRequired(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "InvalidResourceArnException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidResourceArn(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(ListTagsForResourceError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "ResourceArnRequiredException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceArnRequired(
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
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::InvalidRepositoryName(ref cause) => cause,
            ListTagsForResourceError::InvalidResourceArn(ref cause) => cause,
            ListTagsForResourceError::RepositoryDoesNotExist(ref cause) => cause,
            ListTagsForResourceError::ResourceArnRequired(ref cause) => cause,
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
}

impl MergePullRequestByFastForwardError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<MergePullRequestByFastForwardError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::InvalidCommitId(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::InvalidRepositoryName(err.msg),
                    )
                }
                "ManualMergeRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::ManualMergeRequired(err.msg),
                    )
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::PullRequestIdRequired(err.msg),
                    )
                }
                "ReferenceDoesNotExistException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::ReferenceDoesNotExist(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::RepositoryNameRequired(err.msg),
                    )
                }
                "TipOfSourceReferenceIsDifferentException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::TipOfSourceReferenceIsDifferent(
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
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
}

impl PostCommentForComparedCommitError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PostCommentForComparedCommitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BeforeCommitIdAndAfterCommitIdAreSameException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::BeforeCommitIdAndAfterCommitIdAreSame(
                            err.msg,
                        ),
                    )
                }
                "ClientRequestTokenRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::ClientRequestTokenRequired(err.msg),
                    )
                }
                "CommentContentRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::CommentContentRequired(err.msg),
                    )
                }
                "CommentContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::CommentContentSizeLimitExceeded(err.msg),
                    )
                }
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::CommitDoesNotExist(err.msg),
                    )
                }
                "CommitIdRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::CommitIdRequired(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "IdempotencyParameterMismatchException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::IdempotencyParameterMismatch(err.msg),
                    )
                }
                "InvalidClientRequestTokenException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::InvalidClientRequestToken(err.msg),
                    )
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::InvalidCommitId(err.msg),
                    )
                }
                "InvalidFileLocationException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::InvalidFileLocation(err.msg),
                    )
                }
                "InvalidFilePositionException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::InvalidFilePosition(err.msg),
                    )
                }
                "InvalidPathException" => {
                    return RusotoError::Service(PostCommentForComparedCommitError::InvalidPath(
                        err.msg,
                    ))
                }
                "InvalidRelativeFileVersionEnumException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::InvalidRelativeFileVersionEnum(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::InvalidRepositoryName(err.msg),
                    )
                }
                "PathDoesNotExistException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::PathDoesNotExist(err.msg),
                    )
                }
                "PathRequiredException" => {
                    return RusotoError::Service(PostCommentForComparedCommitError::PathRequired(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForComparedCommitError::RepositoryNameRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The repository does not contain any pull requests with that pull request ID. Use GetPullRequest to verify the correct repository name for the pull request ID.</p>
    RepositoryNotAssociatedWithPullRequest(String),
}

impl PostCommentForPullRequestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PostCommentForPullRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BeforeCommitIdAndAfterCommitIdAreSameException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::BeforeCommitIdAndAfterCommitIdAreSame(
                            err.msg,
                        ),
                    )
                }
                "ClientRequestTokenRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::ClientRequestTokenRequired(err.msg),
                    )
                }
                "CommentContentRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::CommentContentRequired(err.msg),
                    )
                }
                "CommentContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::CommentContentSizeLimitExceeded(err.msg),
                    )
                }
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::CommitDoesNotExist(err.msg),
                    )
                }
                "CommitIdRequiredException" => {
                    return RusotoError::Service(PostCommentForPullRequestError::CommitIdRequired(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "IdempotencyParameterMismatchException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::IdempotencyParameterMismatch(err.msg),
                    )
                }
                "InvalidClientRequestTokenException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::InvalidClientRequestToken(err.msg),
                    )
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(PostCommentForPullRequestError::InvalidCommitId(
                        err.msg,
                    ))
                }
                "InvalidFileLocationException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::InvalidFileLocation(err.msg),
                    )
                }
                "InvalidFilePositionException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::InvalidFilePosition(err.msg),
                    )
                }
                "InvalidPathException" => {
                    return RusotoError::Service(PostCommentForPullRequestError::InvalidPath(
                        err.msg,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRelativeFileVersionEnumException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::InvalidRelativeFileVersionEnum(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::InvalidRepositoryName(err.msg),
                    )
                }
                "PathDoesNotExistException" => {
                    return RusotoError::Service(PostCommentForPullRequestError::PathDoesNotExist(
                        err.msg,
                    ))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(PostCommentForPullRequestError::PathRequired(
                        err.msg,
                    ))
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::PullRequestIdRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::RepositoryNameRequired(err.msg),
                    )
                }
                "RepositoryNotAssociatedWithPullRequestException" => {
                    return RusotoError::Service(
                        PostCommentForPullRequestError::RepositoryNotAssociatedWithPullRequest(
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
}

impl PostCommentReplyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PostCommentReplyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientRequestTokenRequiredException" => {
                    return RusotoError::Service(PostCommentReplyError::ClientRequestTokenRequired(
                        err.msg,
                    ))
                }
                "CommentContentRequiredException" => {
                    return RusotoError::Service(PostCommentReplyError::CommentContentRequired(
                        err.msg,
                    ))
                }
                "CommentContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        PostCommentReplyError::CommentContentSizeLimitExceeded(err.msg),
                    )
                }
                "CommentDoesNotExistException" => {
                    return RusotoError::Service(PostCommentReplyError::CommentDoesNotExist(
                        err.msg,
                    ))
                }
                "CommentIdRequiredException" => {
                    return RusotoError::Service(PostCommentReplyError::CommentIdRequired(err.msg))
                }
                "IdempotencyParameterMismatchException" => {
                    return RusotoError::Service(
                        PostCommentReplyError::IdempotencyParameterMismatch(err.msg),
                    )
                }
                "InvalidClientRequestTokenException" => {
                    return RusotoError::Service(PostCommentReplyError::InvalidClientRequestToken(
                        err.msg,
                    ))
                }
                "InvalidCommentIdException" => {
                    return RusotoError::Service(PostCommentReplyError::InvalidCommentId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The file cannot be added because it is too large. The maximum file size that can be added using PutFile is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>A file cannot be added to the repository because the specified file name has the same name as a directory in this repository. Either provide another name for the file, or add the file in a directory that does not match the file name.</p>
    FileNameConflictsWithDirectoryName(String),
    /// <p>The commit cannot be created because a specified file path points to a submodule. Verify that the destination files have valid file paths that do not point to a submodule.</p>
    FilePathConflictsWithSubmodulePath(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit result in a folder contents exceeding the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
    FolderContentSizeLimitExceeded(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified deletion parameter is not valid.</p>
    InvalidDeletionParameter(String),
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
    /// <p>The user name is not valid because it has exceeded the character limit for file names. File names, including the path to the file, cannot exceed the character limit. </p>
    NameLengthExceeded(String),
    /// <p>The parent commit ID is not valid because it does not exist. The specified parent commit ID does not exist in the specified branch of the repository.</p>
    ParentCommitDoesNotExist(String),
    /// <p>The file could not be added because the provided parent commit ID is not the current tip of the specified branch. To view the full commit ID of the current head of the branch, use <a>GetBranch</a>.</p>
    ParentCommitIdOutdated(String),
    /// <p>A parent commit ID is required. To view the full commit ID of a branch in a repository, use <a>GetBranch</a> or a Git command (for example, git pull or git log).</p>
    ParentCommitIdRequired(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The file was not added or updated because the content of the file is exactly the same as the content of that file in the repository and branch that you specified.</p>
    SameFileContent(String),
}

impl PutFileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutFileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(PutFileError::BranchDoesNotExist(err.msg))
                }
                "BranchNameIsTagNameException" => {
                    return RusotoError::Service(PutFileError::BranchNameIsTagName(err.msg))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(PutFileError::BranchNameRequired(err.msg))
                }
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(PutFileError::CommitMessageLengthExceeded(err.msg))
                }
                "DirectoryNameConflictsWithFileNameException" => {
                    return RusotoError::Service(PutFileError::DirectoryNameConflictsWithFileName(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(PutFileError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ))
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(PutFileError::EncryptionKeyAccessDenied(err.msg))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(PutFileError::EncryptionKeyDisabled(err.msg))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(PutFileError::EncryptionKeyNotFound(err.msg))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(PutFileError::EncryptionKeyUnavailable(err.msg))
                }
                "FileContentRequiredException" => {
                    return RusotoError::Service(PutFileError::FileContentRequired(err.msg))
                }
                "FileContentSizeLimitExceededException" => {
                    return RusotoError::Service(PutFileError::FileContentSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "FileNameConflictsWithDirectoryNameException" => {
                    return RusotoError::Service(PutFileError::FileNameConflictsWithDirectoryName(
                        err.msg,
                    ))
                }
                "FilePathConflictsWithSubmodulePathException" => {
                    return RusotoError::Service(PutFileError::FilePathConflictsWithSubmodulePath(
                        err.msg,
                    ))
                }
                "FolderContentSizeLimitExceededException" => {
                    return RusotoError::Service(PutFileError::FolderContentSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(PutFileError::InvalidBranchName(err.msg))
                }
                "InvalidDeletionParameterException" => {
                    return RusotoError::Service(PutFileError::InvalidDeletionParameter(err.msg))
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(PutFileError::InvalidEmail(err.msg))
                }
                "InvalidFileModeException" => {
                    return RusotoError::Service(PutFileError::InvalidFileMode(err.msg))
                }
                "InvalidParentCommitIdException" => {
                    return RusotoError::Service(PutFileError::InvalidParentCommitId(err.msg))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(PutFileError::InvalidPath(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(PutFileError::InvalidRepositoryName(err.msg))
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(PutFileError::NameLengthExceeded(err.msg))
                }
                "ParentCommitDoesNotExistException" => {
                    return RusotoError::Service(PutFileError::ParentCommitDoesNotExist(err.msg))
                }
                "ParentCommitIdOutdatedException" => {
                    return RusotoError::Service(PutFileError::ParentCommitIdOutdated(err.msg))
                }
                "ParentCommitIdRequiredException" => {
                    return RusotoError::Service(PutFileError::ParentCommitIdRequired(err.msg))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(PutFileError::PathRequired(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(PutFileError::RepositoryDoesNotExist(err.msg))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(PutFileError::RepositoryNameRequired(err.msg))
                }
                "SameFileContentException" => {
                    return RusotoError::Service(PutFileError::SameFileContent(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            PutFileError::FilePathConflictsWithSubmodulePath(ref cause) => cause,
            PutFileError::FolderContentSizeLimitExceeded(ref cause) => cause,
            PutFileError::InvalidBranchName(ref cause) => cause,
            PutFileError::InvalidDeletionParameter(ref cause) => cause,
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
}

impl PutRepositoryTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRepositoryTriggersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(PutRepositoryTriggersError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(PutRepositoryTriggersError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(PutRepositoryTriggersError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "InvalidRepositoryTriggerBranchNameException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::InvalidRepositoryTriggerBranchName(err.msg),
                    )
                }
                "InvalidRepositoryTriggerCustomDataException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::InvalidRepositoryTriggerCustomData(err.msg),
                    )
                }
                "InvalidRepositoryTriggerDestinationArnException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(err.msg),
                    )
                }
                "InvalidRepositoryTriggerEventsException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::InvalidRepositoryTriggerEvents(err.msg),
                    )
                }
                "InvalidRepositoryTriggerNameException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::InvalidRepositoryTriggerName(err.msg),
                    )
                }
                "InvalidRepositoryTriggerRegionException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::InvalidRepositoryTriggerRegion(err.msg),
                    )
                }
                "MaximumBranchesExceededException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::MaximumBranchesExceeded(err.msg),
                    )
                }
                "MaximumRepositoryTriggersExceededException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::MaximumRepositoryTriggersExceeded(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::RepositoryNameRequired(err.msg),
                    )
                }
                "RepositoryTriggerBranchNameListRequiredException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(
                            err.msg,
                        ),
                    )
                }
                "RepositoryTriggerDestinationArnRequiredException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(
                            err.msg,
                        ),
                    )
                }
                "RepositoryTriggerEventsListRequiredException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::RepositoryTriggerEventsListRequired(err.msg),
                    )
                }
                "RepositoryTriggerNameRequiredException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::RepositoryTriggerNameRequired(err.msg),
                    )
                }
                "RepositoryTriggersListRequiredException" => {
                    return RusotoError::Service(
                        PutRepositoryTriggersError::RepositoryTriggersListRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The value for the resource ARN is not valid. For more information about resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    InvalidResourceArn(String),
    /// <p>The specified tag is not valid. Key names cannot be prefixed with aws:.</p>
    InvalidSystemTagUsage(String),
    /// <p>The map of tags is not valid.</p>
    InvalidTagsMap(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A valid Amazon Resource Name (ARN) for an AWS CodeCommit resource is required. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    ResourceArnRequired(String),
    /// <p>The tag policy is not valid.</p>
    TagPolicy(String),
    /// <p>A map of tags is required.</p>
    TagsMapRequired(String),
    /// <p>The maximum number of tags for an AWS CodeCommit resource has been exceeded.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(TagResourceError::InvalidRepositoryName(err.msg))
                }
                "InvalidResourceArnException" => {
                    return RusotoError::Service(TagResourceError::InvalidResourceArn(err.msg))
                }
                "InvalidSystemTagUsageException" => {
                    return RusotoError::Service(TagResourceError::InvalidSystemTagUsage(err.msg))
                }
                "InvalidTagsMapException" => {
                    return RusotoError::Service(TagResourceError::InvalidTagsMap(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(TagResourceError::RepositoryDoesNotExist(err.msg))
                }
                "ResourceArnRequiredException" => {
                    return RusotoError::Service(TagResourceError::ResourceArnRequired(err.msg))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(TagResourceError::TagPolicy(err.msg))
                }
                "TagsMapRequiredException" => {
                    return RusotoError::Service(TagResourceError::TagsMapRequired(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::InvalidRepositoryName(ref cause) => cause,
            TagResourceError::InvalidResourceArn(ref cause) => cause,
            TagResourceError::InvalidSystemTagUsage(ref cause) => cause,
            TagResourceError::InvalidTagsMap(ref cause) => cause,
            TagResourceError::RepositoryDoesNotExist(ref cause) => cause,
            TagResourceError::ResourceArnRequired(ref cause) => cause,
            TagResourceError::TagPolicy(ref cause) => cause,
            TagResourceError::TagsMapRequired(ref cause) => cause,
            TagResourceError::TooManyTags(ref cause) => cause,
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
}

impl TestRepositoryTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestRepositoryTriggersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::InvalidRepositoryName(err.msg),
                    )
                }
                "InvalidRepositoryTriggerBranchNameException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::InvalidRepositoryTriggerBranchName(err.msg),
                    )
                }
                "InvalidRepositoryTriggerCustomDataException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::InvalidRepositoryTriggerCustomData(err.msg),
                    )
                }
                "InvalidRepositoryTriggerDestinationArnException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(
                            err.msg,
                        ),
                    )
                }
                "InvalidRepositoryTriggerEventsException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::InvalidRepositoryTriggerEvents(err.msg),
                    )
                }
                "InvalidRepositoryTriggerNameException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::InvalidRepositoryTriggerName(err.msg),
                    )
                }
                "InvalidRepositoryTriggerRegionException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::InvalidRepositoryTriggerRegion(err.msg),
                    )
                }
                "MaximumBranchesExceededException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::MaximumBranchesExceeded(err.msg),
                    )
                }
                "MaximumRepositoryTriggersExceededException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::MaximumRepositoryTriggersExceeded(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::RepositoryNameRequired(err.msg),
                    )
                }
                "RepositoryTriggerBranchNameListRequiredException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(
                            err.msg,
                        ),
                    )
                }
                "RepositoryTriggerDestinationArnRequiredException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(
                            err.msg,
                        ),
                    )
                }
                "RepositoryTriggerEventsListRequiredException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::RepositoryTriggerEventsListRequired(err.msg),
                    )
                }
                "RepositoryTriggerNameRequiredException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::RepositoryTriggerNameRequired(err.msg),
                    )
                }
                "RepositoryTriggersListRequiredException" => {
                    return RusotoError::Service(
                        TestRepositoryTriggersError::RepositoryTriggersListRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p><p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The value for the resource ARN is not valid. For more information about resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    InvalidResourceArn(String),
    /// <p>The specified tag is not valid. Key names cannot be prefixed with aws:.</p>
    InvalidSystemTagUsage(String),
    /// <p>The list of tags is not valid.</p>
    InvalidTagKeysList(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A valid Amazon Resource Name (ARN) for an AWS CodeCommit resource is required. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    ResourceArnRequired(String),
    /// <p>A list of tag keys is required. The list cannot be empty or null.</p>
    TagKeysListRequired(String),
    /// <p>The tag policy is not valid.</p>
    TagPolicy(String),
    /// <p>The maximum number of tags for an AWS CodeCommit resource has been exceeded.</p>
    TooManyTags(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRepositoryName(err.msg))
                }
                "InvalidResourceArnException" => {
                    return RusotoError::Service(UntagResourceError::InvalidResourceArn(err.msg))
                }
                "InvalidSystemTagUsageException" => {
                    return RusotoError::Service(UntagResourceError::InvalidSystemTagUsage(err.msg))
                }
                "InvalidTagKeysListException" => {
                    return RusotoError::Service(UntagResourceError::InvalidTagKeysList(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(UntagResourceError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "ResourceArnRequiredException" => {
                    return RusotoError::Service(UntagResourceError::ResourceArnRequired(err.msg))
                }
                "TagKeysListRequiredException" => {
                    return RusotoError::Service(UntagResourceError::TagKeysListRequired(err.msg))
                }
                "TagPolicyException" => {
                    return RusotoError::Service(UntagResourceError::TagPolicy(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::InvalidRepositoryName(ref cause) => cause,
            UntagResourceError::InvalidResourceArn(ref cause) => cause,
            UntagResourceError::InvalidSystemTagUsage(ref cause) => cause,
            UntagResourceError::InvalidTagKeysList(ref cause) => cause,
            UntagResourceError::RepositoryDoesNotExist(ref cause) => cause,
            UntagResourceError::ResourceArnRequired(ref cause) => cause,
            UntagResourceError::TagKeysListRequired(ref cause) => cause,
            UntagResourceError::TagPolicy(ref cause) => cause,
            UntagResourceError::TooManyTags(ref cause) => cause,
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
}

impl UpdateCommentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCommentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommentContentRequiredException" => {
                    return RusotoError::Service(UpdateCommentError::CommentContentRequired(
                        err.msg,
                    ))
                }
                "CommentContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateCommentError::CommentContentSizeLimitExceeded(err.msg),
                    )
                }
                "CommentDeletedException" => {
                    return RusotoError::Service(UpdateCommentError::CommentDeleted(err.msg))
                }
                "CommentDoesNotExistException" => {
                    return RusotoError::Service(UpdateCommentError::CommentDoesNotExist(err.msg))
                }
                "CommentIdRequiredException" => {
                    return RusotoError::Service(UpdateCommentError::CommentIdRequired(err.msg))
                }
                "CommentNotCreatedByCallerException" => {
                    return RusotoError::Service(UpdateCommentError::CommentNotCreatedByCaller(
                        err.msg,
                    ))
                }
                "InvalidCommentIdException" => {
                    return RusotoError::Service(UpdateCommentError::InvalidCommentId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdateDefaultBranchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDefaultBranchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::BranchDoesNotExist(
                        err.msg,
                    ))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::BranchNameRequired(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        UpdateDefaultBranchError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        UpdateDefaultBranchError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        UpdateDefaultBranchError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::InvalidBranchName(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(UpdateDefaultBranchError::RepositoryNameRequired(
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
}

impl UpdatePullRequestDescriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdatePullRequestDescriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidDescriptionException" => {
                    return RusotoError::Service(
                        UpdatePullRequestDescriptionError::InvalidDescription(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        UpdatePullRequestDescriptionError::InvalidPullRequestId(err.msg),
                    )
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestDescriptionError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdatePullRequestDescriptionError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestDescriptionError::PullRequestIdRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdatePullRequestStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePullRequestStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidPullRequestStatusException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::InvalidPullRequestStatus(err.msg),
                    )
                }
                "InvalidPullRequestStatusUpdateException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::InvalidPullRequestStatusUpdate(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::PullRequestIdRequired(err.msg),
                    )
                }
                "PullRequestStatusRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestStatusError::PullRequestStatusRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdatePullRequestTitleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePullRequestTitleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(UpdatePullRequestTitleError::InvalidPullRequestId(
                        err.msg,
                    ))
                }
                "InvalidTitleException" => {
                    return RusotoError::Service(UpdatePullRequestTitleError::InvalidTitle(err.msg))
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestTitleError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdatePullRequestTitleError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestTitleError::PullRequestIdRequired(err.msg),
                    )
                }
                "TitleRequiredException" => {
                    return RusotoError::Service(UpdatePullRequestTitleError::TitleRequired(
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
}

impl UpdateRepositoryDescriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRepositoryDescriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidRepositoryDescriptionException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::InvalidRepositoryDescription(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::InvalidRepositoryName(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        UpdateRepositoryDescriptionError::RepositoryNameRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UpdateRepositoryNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRepositoryNameError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(UpdateRepositoryNameError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(UpdateRepositoryNameError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameExistsException" => {
                    return RusotoError::Service(UpdateRepositoryNameError::RepositoryNameExists(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(UpdateRepositoryNameError::RepositoryNameRequired(
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

    /// <p>Creates a commit for a repository on the tip of a specified branch.</p>
    fn create_commit(
        &self,
        input: CreateCommitInput,
    ) -> RusotoFuture<CreateCommitOutput, CreateCommitError>;

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

    /// <p>Deletes a specified file from a specified branch. A commit is created on the branch that contains the revision. The file will still exist in the commits prior to the commit that contains the deletion.</p>
    fn delete_file(
        &self,
        input: DeleteFileInput,
    ) -> RusotoFuture<DeleteFileOutput, DeleteFileError>;

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

    /// <p>Returns the base-64 encoded contents of a specified file and its metadata.</p>
    fn get_file(&self, input: GetFileInput) -> RusotoFuture<GetFileOutput, GetFileError>;

    /// <p>Returns the contents of a specified folder in a repository.</p>
    fn get_folder(&self, input: GetFolderInput) -> RusotoFuture<GetFolderOutput, GetFolderError>;

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

    /// <p>Gets information about AWS tags for a specified Amazon Resource Name (ARN) in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> RusotoFuture<ListTagsForResourceOutput, ListTagsForResourceError>;

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

    /// <p>Adds or updates a file in a branch in an AWS CodeCommit repository, and generates a commit for the addition in the specified branch.</p>
    fn put_file(&self, input: PutFileInput) -> RusotoFuture<PutFileOutput, PutFileError>;

    /// <p>Replaces all triggers for a repository. This can be used to create or delete triggers.</p>
    fn put_repository_triggers(
        &self,
        input: PutRepositoryTriggersInput,
    ) -> RusotoFuture<PutRepositoryTriggersOutput, PutRepositoryTriggersError>;

    /// <p>Adds or updates tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    fn tag_resource(&self, input: TagResourceInput) -> RusotoFuture<(), TagResourceError>;

    /// <p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test will send data from the last commit. If no data is available, sample data will be generated.</p>
    fn test_repository_triggers(
        &self,
        input: TestRepositoryTriggersInput,
    ) -> RusotoFuture<TestRepositoryTriggersOutput, TestRepositoryTriggersError>;

    /// <p>Removes tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    fn untag_resource(&self, input: UntagResourceInput) -> RusotoFuture<(), UntagResourceError>;

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

    /// <p>Renames a repository. The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix ".git" is prohibited. For a full description of the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide.</p>
    fn update_repository_name(
        &self,
        input: UpdateRepositoryNameInput,
    ) -> RusotoFuture<(), UpdateRepositoryNameError>;
}
/// A client for the CodeCommit API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D, C>(
        request_dispatcher: D,
        credentials_provider: P,
        payload_compressor: Option<C>,
        region: region::Region,
    ) -> CodeCommitClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
        C: CompressRequestPayload + Send + Sync + 'static,
    {
        CodeCommitClient {
            client: Client::new_with(credentials_provider, request_dispatcher, payload_compressor),
            region,
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetRepositoriesOutput, _>()
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
        request.set_payload(Some(encoded));

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

    /// <p>Creates a commit for a repository on the tip of a specified branch.</p>
    fn create_commit(
        &self,
        input: CreateCommitInput,
    ) -> RusotoFuture<CreateCommitOutput, CreateCommitError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateCommit");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCommitOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateCommitError::from_response(response))),
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePullRequestOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRepositoryOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteBranchOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCommentContentOutput, _>()
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

    /// <p>Deletes a specified file from a specified branch. A commit is created on the branch that contains the revision. The file will still exist in the commits prior to the commit that contains the deletion.</p>
    fn delete_file(
        &self,
        input: DeleteFileInput,
    ) -> RusotoFuture<DeleteFileOutput, DeleteFileError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteFile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteFileOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFileError::from_response(response))),
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteRepositoryOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePullRequestEventsOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetBlobOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetBranchOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCommentOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCommentsForComparedCommitOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCommentsForPullRequestOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetCommitOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDifferencesOutput, _>()
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

    /// <p>Returns the base-64 encoded contents of a specified file and its metadata.</p>
    fn get_file(&self, input: GetFileInput) -> RusotoFuture<GetFileOutput, GetFileError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetFile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetFileOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFileError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the contents of a specified folder in a repository.</p>
    fn get_folder(&self, input: GetFolderInput) -> RusotoFuture<GetFolderOutput, GetFolderError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetFolder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetFolderOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFolderError::from_response(response))),
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMergeConflictsOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPullRequestOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRepositoryOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRepositoryTriggersOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBranchesOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPullRequestsOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListRepositoriesOutput, _>()
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

    /// <p>Gets information about AWS tags for a specified Amazon Resource Name (ARN) in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> RusotoFuture<ListTagsForResourceOutput, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<MergePullRequestByFastForwardOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PostCommentForComparedCommitOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PostCommentForPullRequestOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PostCommentReplyOutput, _>()
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

    /// <p>Adds or updates a file in a branch in an AWS CodeCommit repository, and generates a commit for the addition in the specified branch.</p>
    fn put_file(&self, input: PutFileInput) -> RusotoFuture<PutFileOutput, PutFileError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PutFile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<PutFileOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutRepositoryTriggersOutput, _>()
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

    /// <p>Adds or updates tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    fn tag_resource(&self, input: TagResourceInput) -> RusotoFuture<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TestRepositoryTriggersOutput, _>()
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

    /// <p>Removes tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the AWS CodeCommit User Guide.</p>
    fn untag_resource(&self, input: UntagResourceInput) -> RusotoFuture<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCommentOutput, _>()
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
        request.set_payload(Some(encoded));

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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePullRequestDescriptionOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePullRequestStatusOutput, _>()
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePullRequestTitleOutput, _>()
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
        request.set_payload(Some(encoded));

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

    /// <p>Renames a repository. The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix ".git" is prohibited. For a full description of the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide.</p>
    fn update_repository_name(
        &self,
        input: UpdateRepositoryNameInput,
    ) -> RusotoFuture<(), UpdateRepositoryNameError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateRepositoryName");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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
