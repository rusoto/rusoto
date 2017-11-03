
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Represents the input of a batch get repositories operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchGetRepositoriesInput {
    #[doc="<p>The names of the repositories to get information about.</p>"]
    #[serde(rename="repositoryNames")]
    pub repository_names: Vec<String>,
}

#[doc="<p>Represents the output of a batch get repositories operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchGetRepositoriesOutput {
    #[doc="<p>A list of repositories returned by the batch get repositories operation.</p>"]
    #[serde(rename="repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<RepositoryMetadata>>,
    #[doc="<p>Returns a list of repository names for which information could not be found.</p>"]
    #[serde(rename="repositoriesNotFound")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_not_found: Option<Vec<String>>,
}

#[doc="<p>Returns information about a specific Git blob object.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BlobMetadata {
    #[doc="<p>The full ID of the blob.</p>"]
    #[serde(rename="blobId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blob_id: Option<String>,
    #[doc="<p>The file mode permissions of the blob. File mode permission codes include:</p> <ul> <li> <p> <code>100644</code> indicates read/write</p> </li> <li> <p> <code>100755</code> indicates read/write/execute</p> </li> <li> <p> <code>160000</code> indicates a submodule</p> </li> <li> <p> <code>120000</code> indicates a symlink</p> </li> </ul>"]
    #[serde(rename="mode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,
    #[doc="<p>The path to the blob and any associated file name, if any.</p>"]
    #[serde(rename="path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
}

#[doc="<p>Returns information about a branch.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BranchInfo {
    #[doc="<p>The name of the branch.</p>"]
    #[serde(rename="branchName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch_name: Option<String>,
    #[doc="<p>The ID of the last commit made to the branch.</p>"]
    #[serde(rename="commitId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,
}

#[doc="<p>Returns information about a specific commit.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Commit {
    #[doc="<p>Any additional data associated with the specified commit.</p>"]
    #[serde(rename="additionalData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_data: Option<String>,
    #[doc="<p>Information about the author of the specified commit. Information includes the date in timestamp format with GMT offset, the name of the author, and the email address for the author, as configured in Git.</p>"]
    #[serde(rename="author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<UserInfo>,
    #[doc="<p>Information about the person who committed the specified commit, also known as the committer. Information includes the date in timestamp format with GMT offset, the name of the committer, and the email address for the committer, as configured in Git.</p> <p>For more information about the difference between an author and a committer in Git, see <a href=\"http://git-scm.com/book/ch2-3.html\">Viewing the Commit History</a> in Pro Git by Scott Chacon and Ben Straub.</p>"]
    #[serde(rename="committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<UserInfo>,
    #[doc="<p>The commit message associated with the specified commit.</p>"]
    #[serde(rename="message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>The parent list for the specified commit.</p>"]
    #[serde(rename="parents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<String>>,
    #[doc="<p>Tree information for the specified commit.</p>"]
    #[serde(rename="treeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree_id: Option<String>,
}

#[doc="<p>Represents the input of a create branch operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateBranchInput {
    #[doc="<p>The name of the new branch to create.</p>"]
    #[serde(rename="branchName")]
    pub branch_name: String,
    #[doc="<p>The ID of the commit to point the new branch to.</p>"]
    #[serde(rename="commitId")]
    pub commit_id: String,
    #[doc="<p>The name of the repository in which you want to create the new branch.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the input of a create repository operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateRepositoryInput {
    #[doc="<p>A comment or description about the new repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note>"]
    #[serde(rename="repositoryDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_description: Option<String>,
    #[doc="<p>The name of the new repository to be created.</p> <note> <p>The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. For a full description of the limits on repository names, see <a href=\"http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html\">Limits</a> in the AWS CodeCommit User Guide. The suffix \".git\" is prohibited.</p> </note>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the output of a create repository operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateRepositoryOutput {
    #[doc="<p>Information about the newly created repository.</p>"]
    #[serde(rename="repositoryMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

#[doc="<p>Represents the input of a delete repository operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteRepositoryInput {
    #[doc="<p>The name of the repository to delete.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the output of a delete repository operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteRepositoryOutput {
    #[doc="<p>The ID of the repository that was deleted.</p>"]
    #[serde(rename="repositoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_id: Option<String>,
}

#[doc="<p>Returns information about a set of differences for a commit specifier.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Difference {
    #[doc="<p>Information about an <code>afterBlob</code> data type object, including the ID, the file mode permission code, and the path.</p>"]
    #[serde(rename="afterBlob")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub after_blob: Option<BlobMetadata>,
    #[doc="<p>Information about a <code>beforeBlob</code> data type object, including the ID, the file mode permission code, and the path.</p>"]
    #[serde(rename="beforeBlob")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub before_blob: Option<BlobMetadata>,
    #[doc="<p>Whether the change type of the difference is an addition (A), deletion (D), or modification (M).</p>"]
    #[serde(rename="changeType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_type: Option<String>,
}

#[doc="<p>Represents the input of a get blob operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetBlobInput {
    #[doc="<p>The ID of the blob, which is its SHA-1 pointer.</p>"]
    #[serde(rename="blobId")]
    pub blob_id: String,
    #[doc="<p>The name of the repository that contains the blob.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the output of a get blob operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetBlobOutput {
    #[doc="<p>The content of the blob, usually a file.</p>"]
    #[serde(rename="content")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub content: Vec<u8>,
}

#[doc="<p>Represents the input of a get branch operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetBranchInput {
    #[doc="<p>The name of the branch for which you want to retrieve information.</p>"]
    #[serde(rename="branchName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch_name: Option<String>,
    #[doc="<p>The name of the repository that contains the branch for which you want to retrieve information.</p>"]
    #[serde(rename="repositoryName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_name: Option<String>,
}

#[doc="<p>Represents the output of a get branch operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetBranchOutput {
    #[doc="<p>The name of the branch.</p>"]
    #[serde(rename="branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch: Option<BranchInfo>,
}

#[doc="<p>Represents the input of a get commit operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCommitInput {
    #[doc="<p>The commit ID.</p>"]
    #[serde(rename="commitId")]
    pub commit_id: String,
    #[doc="<p>The name of the repository to which the commit was made.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the output of a get commit operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCommitOutput {
    #[doc="<p>A commit data type object that contains information about the specified commit.</p>"]
    #[serde(rename="commit")]
    pub commit: Commit,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDifferencesInput {
    #[doc="<p>A non-negative integer used to limit the number of returned results.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>An enumeration token that when provided in a request, returns the next batch of the results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit.</p>"]
    #[serde(rename="afterCommitSpecifier")]
    pub after_commit_specifier: String,
    #[doc="<p>The file path in which to check differences. Limits the results to this path. Can also be used to specify the changed name of a directory or folder, if it has changed. If not specified, differences will be shown for all paths.</p>"]
    #[serde(rename="afterPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub after_path: Option<String>,
    #[doc="<p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit. For example, the full commit ID. Optional. If not specified, all changes prior to the <code>afterCommitSpecifier</code> value will be shown. If you do not use <code>beforeCommitSpecifier</code> in your request, consider limiting the results with <code>maxResults</code>.</p>"]
    #[serde(rename="beforeCommitSpecifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub before_commit_specifier: Option<String>,
    #[doc="<p>The file path in which to check for differences. Limits the results to this path. Can also be used to specify the previous name of a directory or folder. If <code>beforePath</code> and <code>afterPath</code> are not specified, differences will be shown for all paths.</p>"]
    #[serde(rename="beforePath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub before_path: Option<String>,
    #[doc="<p>The name of the repository where you want to get differences.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDifferencesOutput {
    #[doc="<p>An enumeration token that can be used in a request to return the next batch of the results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A differences data type object that contains information about the differences, including whether the difference is added, modified, or deleted (A, D, M).</p>"]
    #[serde(rename="differences")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub differences: Option<Vec<Difference>>,
}

#[doc="<p>Represents the input of a get repository operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRepositoryInput {
    #[doc="<p>The name of the repository to get information about.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the output of a get repository operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRepositoryOutput {
    #[doc="<p>Information about the repository.</p>"]
    #[serde(rename="repositoryMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

#[doc="<p>Represents the input of a get repository triggers operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRepositoryTriggersInput {
    #[doc="<p>The name of the repository for which the trigger is configured.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the output of a get repository triggers operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRepositoryTriggersOutput {
    #[doc="<p>The system-generated unique ID for the trigger.</p>"]
    #[serde(rename="configurationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_id: Option<String>,
    #[doc="<p>The JSON block of configuration information for each trigger.</p>"]
    #[serde(rename="triggers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub triggers: Option<Vec<RepositoryTrigger>>,
}

#[doc="<p>Represents the input of a list branches operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListBranchesInput {
    #[doc="<p>An enumeration token that allows the operation to batch the results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the repository that contains the branches.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the output of a list branches operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListBranchesOutput {
    #[doc="<p>The list of branch names.</p>"]
    #[serde(rename="branches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches: Option<Vec<String>>,
    #[doc="<p>An enumeration token that returns the batch of the results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the input of a list repositories operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRepositoriesInput {
    #[doc="<p>An enumeration token that allows the operation to batch the results of the operation. Batch sizes are 1,000 for list repository operations. When the client sends the token back to AWS CodeCommit, another page of 1,000 records is retrieved.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The order in which to sort the results of a list repositories operation.</p>"]
    #[serde(rename="order")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub order: Option<String>,
    #[doc="<p>The criteria used to sort the results of a list repositories operation.</p>"]
    #[serde(rename="sortBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_by: Option<String>,
}

#[doc="<p>Represents the output of a list repositories operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListRepositoriesOutput {
    #[doc="<p>An enumeration token that allows the operation to batch the results of the operation. Batch sizes are 1,000 for list repository operations. When the client sends the token back to AWS CodeCommit, another page of 1,000 records is retrieved.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Lists the repositories called by the list repositories operation.</p>"]
    #[serde(rename="repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<RepositoryNameIdPair>>,
}

#[doc="<p>Represents the input ofa put repository triggers operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRepositoryTriggersInput {
    #[doc="<p>The name of the repository where you want to create or update the trigger.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
    #[doc="<p>The JSON block of configuration information for each trigger.</p>"]
    #[serde(rename="triggers")]
    pub triggers: Vec<RepositoryTrigger>,
}

#[doc="<p>Represents the output of a put repository triggers operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRepositoryTriggersOutput {
    #[doc="<p>The system-generated unique ID for the create or update operation.</p>"]
    #[serde(rename="configurationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_id: Option<String>,
}

#[doc="<p>Information about a repository.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RepositoryMetadata {
    #[doc="<p>The Amazon Resource Name (ARN) of the repository.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>The ID of the AWS account associated with the repository.</p>"]
    #[serde(rename="accountId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account_id: Option<String>,
    #[doc="<p>The URL to use for cloning the repository over HTTPS.</p>"]
    #[serde(rename="cloneUrlHttp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url_http: Option<String>,
    #[doc="<p>The URL to use for cloning the repository over SSH.</p>"]
    #[serde(rename="cloneUrlSsh")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url_ssh: Option<String>,
    #[doc="<p>The date and time the repository was created, in timestamp format.</p>"]
    #[serde(rename="creationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The repository's default branch name.</p>"]
    #[serde(rename="defaultBranch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,
    #[doc="<p>The date and time the repository was last modified, in timestamp format.</p>"]
    #[serde(rename="lastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>A comment or description about the repository.</p>"]
    #[serde(rename="repositoryDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_description: Option<String>,
    #[doc="<p>The ID of the repository.</p>"]
    #[serde(rename="repositoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_id: Option<String>,
    #[doc="<p>The repository's name.</p>"]
    #[serde(rename="repositoryName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_name: Option<String>,
}

#[doc="<p>Information about a repository name and ID.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RepositoryNameIdPair {
    #[doc="<p>The ID associated with the repository.</p>"]
    #[serde(rename="repositoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_id: Option<String>,
    #[doc="<p>The name associated with the repository.</p>"]
    #[serde(rename="repositoryName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_name: Option<String>,
}

#[doc="<p>Information about a trigger for a repository.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RepositoryTrigger {
    #[doc="<p>The branches that will be included in the trigger configuration. If no branches are specified, the trigger will apply to all branches.</p>"]
    #[serde(rename="branches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches: Option<Vec<String>>,
    #[doc="<p>Any custom data associated with the trigger that will be included in the information sent to the target of the trigger.</p>"]
    #[serde(rename="customData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_data: Option<String>,
    #[doc="<p>The ARN of the resource that is the target for a trigger. For example, the ARN of a topic in Amazon Simple Notification Service (SNS).</p>"]
    #[serde(rename="destinationArn")]
    pub destination_arn: String,
    #[doc="<p>The repository events that will cause the trigger to run actions in another service, such as sending a notification through Amazon Simple Notification Service (SNS). </p> <note> <p>The valid value \"all\" cannot be used with any other values.</p> </note>"]
    #[serde(rename="events")]
    pub events: Vec<String>,
    #[doc="<p>The name of the trigger.</p>"]
    #[serde(rename="name")]
    pub name: String,
}

#[doc="<p>A trigger failed to run.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RepositoryTriggerExecutionFailure {
    #[doc="<p>Additional message information about the trigger that did not run.</p>"]
    #[serde(rename="failureMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failure_message: Option<String>,
    #[doc="<p>The name of the trigger that did not run.</p>"]
    #[serde(rename="trigger")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trigger: Option<String>,
}

#[doc="<p>Represents the input of a test repository triggers operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TestRepositoryTriggersInput {
    #[doc="<p>The name of the repository in which to test the triggers.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
    #[doc="<p>The list of triggers to test.</p>"]
    #[serde(rename="triggers")]
    pub triggers: Vec<RepositoryTrigger>,
}

#[doc="<p>Represents the output of a test repository triggers operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TestRepositoryTriggersOutput {
    #[doc="<p>The list of triggers that were not able to be tested. This list provides the names of the triggers that could not be tested, separated by commas.</p>"]
    #[serde(rename="failedExecutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_executions: Option<Vec<RepositoryTriggerExecutionFailure>>,
    #[doc="<p>The list of triggers that were successfully tested. This list provides the names of the triggers that were successfully tested, separated by commas.</p>"]
    #[serde(rename="successfulExecutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub successful_executions: Option<Vec<String>>,
}

#[doc="<p>Represents the input of an update default branch operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateDefaultBranchInput {
    #[doc="<p>The name of the branch to set as the default.</p>"]
    #[serde(rename="defaultBranchName")]
    pub default_branch_name: String,
    #[doc="<p>The name of the repository to set or change the default branch for.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the input of an update repository description operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRepositoryDescriptionInput {
    #[doc="<p>The new comment or description for the specified repository. Repository descriptions are limited to 1,000 characters.</p>"]
    #[serde(rename="repositoryDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_description: Option<String>,
    #[doc="<p>The name of the repository to set or change the comment or description for.</p>"]
    #[serde(rename="repositoryName")]
    pub repository_name: String,
}

#[doc="<p>Represents the input of an update repository description operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRepositoryNameInput {
    #[doc="<p>The new name for the repository.</p>"]
    #[serde(rename="newName")]
    pub new_name: String,
    #[doc="<p>The existing name of the repository.</p>"]
    #[serde(rename="oldName")]
    pub old_name: String,
}

#[doc="<p>Information about the user who made a specified commit.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserInfo {
    #[doc="<p>The date when the specified commit was pushed to the repository.</p>"]
    #[serde(rename="date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
    #[doc="<p>The email address associated with the user who made the commit, if any.</p>"]
    #[serde(rename="email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[doc="<p>The name of the user who made the specified commit.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by BatchGetRepositories
#[derive(Debug, PartialEq)]
pub enum BatchGetRepositoriesError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The maximum number of allowed repository names was exceeded. Currently, this number is 25.</p>
    MaximumRepositoryNamesExceeded(String),
    ///<p>A repository names object is required but was not specified.</p>
    RepositoryNamesRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchGetRepositoriesError {
    pub fn from_body(body: &str) -> BatchGetRepositoriesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => BatchGetRepositoriesError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => BatchGetRepositoriesError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => BatchGetRepositoriesError::EncryptionKeyDisabled(String::from(error_message)),
                    "EncryptionKeyNotFoundException" => BatchGetRepositoriesError::EncryptionKeyNotFound(String::from(error_message)),
                    "EncryptionKeyUnavailableException" => BatchGetRepositoriesError::EncryptionKeyUnavailable(String::from(error_message)),
                    "InvalidRepositoryNameException" => BatchGetRepositoriesError::InvalidRepositoryName(String::from(error_message)),
                    "MaximumRepositoryNamesExceededException" => BatchGetRepositoriesError::MaximumRepositoryNamesExceeded(String::from(error_message)),
                    "RepositoryNamesRequiredException" => BatchGetRepositoriesError::RepositoryNamesRequired(String::from(error_message)),
                    "ValidationException" => {
                        BatchGetRepositoriesError::Validation(error_message.to_string())
                    }
                    _ => BatchGetRepositoriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetRepositoriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetRepositoriesError {
    fn from(err: serde_json::error::Error) -> BatchGetRepositoriesError {
        BatchGetRepositoriesError::Unknown(err.description().to_string())
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
            BatchGetRepositoriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBranch
#[derive(Debug, PartialEq)]
pub enum CreateBranchError {
    ///<p>The specified branch name already exists.</p>
    BranchNameExists(String),
    ///<p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    ///<p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    ///<p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified branch name is not valid.</p>
    InvalidBranchName(String),
    ///<p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateBranchError {
    pub fn from_body(body: &str) -> CreateBranchError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BranchNameExistsException" => {
                        CreateBranchError::BranchNameExists(String::from(error_message))
                    }
                    "BranchNameRequiredException" => {
                        CreateBranchError::BranchNameRequired(String::from(error_message))
                    }
                    "CommitDoesNotExistException" => {
                        CreateBranchError::CommitDoesNotExist(String::from(error_message))
                    }
                    "CommitIdRequiredException" => {
                        CreateBranchError::CommitIdRequired(String::from(error_message))
                    }
                    "EncryptionIntegrityChecksFailedException" => CreateBranchError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => {
                        CreateBranchError::EncryptionKeyAccessDenied(String::from(error_message))
                    }
                    "EncryptionKeyDisabledException" => {
                        CreateBranchError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        CreateBranchError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        CreateBranchError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidBranchNameException" => {
                        CreateBranchError::InvalidBranchName(String::from(error_message))
                    }
                    "InvalidCommitIdException" => {
                        CreateBranchError::InvalidCommitId(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        CreateBranchError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => {
                        CreateBranchError::RepositoryDoesNotExist(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        CreateBranchError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBranchError::Validation(error_message.to_string())
                    }
                    _ => CreateBranchError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBranchError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBranchError {
    fn from(err: serde_json::error::Error) -> CreateBranchError {
        CreateBranchError::Unknown(err.description().to_string())
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
            CreateBranchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRepository
#[derive(Debug, PartialEq)]
pub enum CreateRepositoryError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified repository description is not valid.</p>
    InvalidRepositoryDescription(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>A repository resource limit was exceeded.</p>
    RepositoryLimitExceeded(String),
    ///<p>The specified repository name already exists.</p>
    RepositoryNameExists(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateRepositoryError {
    pub fn from_body(body: &str) -> CreateRepositoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => CreateRepositoryError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => CreateRepositoryError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => {
                        CreateRepositoryError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        CreateRepositoryError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        CreateRepositoryError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidRepositoryDescriptionException" => CreateRepositoryError::InvalidRepositoryDescription(String::from(error_message)),
                    "InvalidRepositoryNameException" => {
                        CreateRepositoryError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryLimitExceededException" => {
                        CreateRepositoryError::RepositoryLimitExceeded(String::from(error_message))
                    }
                    "RepositoryNameExistsException" => {
                        CreateRepositoryError::RepositoryNameExists(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        CreateRepositoryError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateRepositoryError::Validation(error_message.to_string())
                    }
                    _ => CreateRepositoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRepositoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRepositoryError {
    fn from(err: serde_json::error::Error) -> CreateRepositoryError {
        CreateRepositoryError::Unknown(err.description().to_string())
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
            CreateRepositoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRepository
#[derive(Debug, PartialEq)]
pub enum DeleteRepositoryError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteRepositoryError {
    pub fn from_body(body: &str) -> DeleteRepositoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => DeleteRepositoryError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => DeleteRepositoryError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => {
                        DeleteRepositoryError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        DeleteRepositoryError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        DeleteRepositoryError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        DeleteRepositoryError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        DeleteRepositoryError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRepositoryError::Validation(error_message.to_string())
                    }
                    _ => DeleteRepositoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRepositoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRepositoryError {
    fn from(err: serde_json::error::Error) -> DeleteRepositoryError {
        DeleteRepositoryError::Unknown(err.description().to_string())
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
            DeleteRepositoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBlob
#[derive(Debug, PartialEq)]
pub enum GetBlobError {
    ///<p>The specified blob does not exist.</p>
    BlobIdDoesNotExist(String),
    ///<p>A blob ID is required but was not specified.</p>
    BlobIdRequired(String),
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified file exceeds the file size limit for AWS CodeCommit. For more information about limits in AWS CodeCommit, see <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    FileTooLarge(String),
    ///<p>The specified blob is not valid.</p>
    InvalidBlobId(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBlobError {
    pub fn from_body(body: &str) -> GetBlobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BlobIdDoesNotExistException" => {
                        GetBlobError::BlobIdDoesNotExist(String::from(error_message))
                    }
                    "BlobIdRequiredException" => {
                        GetBlobError::BlobIdRequired(String::from(error_message))
                    }
                    "EncryptionIntegrityChecksFailedException" => {
                        GetBlobError::EncryptionIntegrityChecksFailed(String::from(error_message))
                    }
                    "EncryptionKeyAccessDeniedException" => {
                        GetBlobError::EncryptionKeyAccessDenied(String::from(error_message))
                    }
                    "EncryptionKeyDisabledException" => {
                        GetBlobError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        GetBlobError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        GetBlobError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "FileTooLargeException" => {
                        GetBlobError::FileTooLarge(String::from(error_message))
                    }
                    "InvalidBlobIdException" => {
                        GetBlobError::InvalidBlobId(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        GetBlobError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => {
                        GetBlobError::RepositoryDoesNotExist(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        GetBlobError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => GetBlobError::Validation(error_message.to_string()),
                    _ => GetBlobError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBlobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBlobError {
    fn from(err: serde_json::error::Error) -> GetBlobError {
        GetBlobError::Unknown(err.description().to_string())
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
            GetBlobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBranch
#[derive(Debug, PartialEq)]
pub enum GetBranchError {
    ///<p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    ///<p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified branch name is not valid.</p>
    InvalidBranchName(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBranchError {
    pub fn from_body(body: &str) -> GetBranchError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BranchDoesNotExistException" => {
                        GetBranchError::BranchDoesNotExist(String::from(error_message))
                    }
                    "BranchNameRequiredException" => {
                        GetBranchError::BranchNameRequired(String::from(error_message))
                    }
                    "EncryptionIntegrityChecksFailedException" => {
                        GetBranchError::EncryptionIntegrityChecksFailed(String::from(error_message))
                    }
                    "EncryptionKeyAccessDeniedException" => {
                        GetBranchError::EncryptionKeyAccessDenied(String::from(error_message))
                    }
                    "EncryptionKeyDisabledException" => {
                        GetBranchError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        GetBranchError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        GetBranchError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidBranchNameException" => {
                        GetBranchError::InvalidBranchName(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        GetBranchError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => {
                        GetBranchError::RepositoryDoesNotExist(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        GetBranchError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => GetBranchError::Validation(error_message.to_string()),
                    _ => GetBranchError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBranchError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBranchError {
    fn from(err: serde_json::error::Error) -> GetBranchError {
        GetBranchError::Unknown(err.description().to_string())
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
            GetBranchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCommit
#[derive(Debug, PartialEq)]
pub enum GetCommitError {
    ///<p>The specified commit ID does not exist.</p>
    CommitIdDoesNotExist(String),
    ///<p>A commit ID was not specified.</p>
    CommitIdRequired(String),
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCommitError {
    pub fn from_body(body: &str) -> GetCommitError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CommitIdDoesNotExistException" => {
                        GetCommitError::CommitIdDoesNotExist(String::from(error_message))
                    }
                    "CommitIdRequiredException" => {
                        GetCommitError::CommitIdRequired(String::from(error_message))
                    }
                    "EncryptionIntegrityChecksFailedException" => {
                        GetCommitError::EncryptionIntegrityChecksFailed(String::from(error_message))
                    }
                    "EncryptionKeyAccessDeniedException" => {
                        GetCommitError::EncryptionKeyAccessDenied(String::from(error_message))
                    }
                    "EncryptionKeyDisabledException" => {
                        GetCommitError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        GetCommitError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        GetCommitError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidCommitIdException" => {
                        GetCommitError::InvalidCommitId(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        GetCommitError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => {
                        GetCommitError::RepositoryDoesNotExist(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        GetCommitError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => GetCommitError::Validation(error_message.to_string()),
                    _ => GetCommitError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCommitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCommitError {
    fn from(err: serde_json::error::Error) -> GetCommitError {
        GetCommitError::Unknown(err.description().to_string())
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
            GetCommitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDifferences
#[derive(Debug, PartialEq)]
pub enum GetDifferencesError {
    ///<p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    ///<p>A commit was not specified.</p>
    CommitRequired(String),
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified commit is not valid.</p>
    InvalidCommit(String),
    ///<p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    ///<p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    ///<p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
    ///<p>The specified path is not valid.</p>
    InvalidPath(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified path does not exist.</p>
    PathDoesNotExist(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDifferencesError {
    pub fn from_body(body: &str) -> GetDifferencesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CommitDoesNotExistException" => {
                        GetDifferencesError::CommitDoesNotExist(String::from(error_message))
                    }
                    "CommitRequiredException" => {
                        GetDifferencesError::CommitRequired(String::from(error_message))
                    }
                    "EncryptionIntegrityChecksFailedException" => GetDifferencesError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => {
                        GetDifferencesError::EncryptionKeyAccessDenied(String::from(error_message))
                    }
                    "EncryptionKeyDisabledException" => {
                        GetDifferencesError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        GetDifferencesError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        GetDifferencesError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidCommitException" => {
                        GetDifferencesError::InvalidCommit(String::from(error_message))
                    }
                    "InvalidCommitIdException" => {
                        GetDifferencesError::InvalidCommitId(String::from(error_message))
                    }
                    "InvalidContinuationTokenException" => {
                        GetDifferencesError::InvalidContinuationToken(String::from(error_message))
                    }
                    "InvalidMaxResultsException" => {
                        GetDifferencesError::InvalidMaxResults(String::from(error_message))
                    }
                    "InvalidPathException" => {
                        GetDifferencesError::InvalidPath(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        GetDifferencesError::InvalidRepositoryName(String::from(error_message))
                    }
                    "PathDoesNotExistException" => {
                        GetDifferencesError::PathDoesNotExist(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => {
                        GetDifferencesError::RepositoryDoesNotExist(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        GetDifferencesError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDifferencesError::Validation(error_message.to_string())
                    }
                    _ => GetDifferencesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDifferencesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDifferencesError {
    fn from(err: serde_json::error::Error) -> GetDifferencesError {
        GetDifferencesError::Unknown(err.description().to_string())
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
            GetDifferencesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRepository
#[derive(Debug, PartialEq)]
pub enum GetRepositoryError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetRepositoryError {
    pub fn from_body(body: &str) -> GetRepositoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => GetRepositoryError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => {
                        GetRepositoryError::EncryptionKeyAccessDenied(String::from(error_message))
                    }
                    "EncryptionKeyDisabledException" => {
                        GetRepositoryError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        GetRepositoryError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        GetRepositoryError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        GetRepositoryError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => {
                        GetRepositoryError::RepositoryDoesNotExist(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        GetRepositoryError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRepositoryError::Validation(error_message.to_string())
                    }
                    _ => GetRepositoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRepositoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRepositoryError {
    fn from(err: serde_json::error::Error) -> GetRepositoryError {
        GetRepositoryError::Unknown(err.description().to_string())
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
            GetRepositoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRepositoryTriggers
#[derive(Debug, PartialEq)]
pub enum GetRepositoryTriggersError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetRepositoryTriggersError {
    pub fn from_body(body: &str) -> GetRepositoryTriggersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => GetRepositoryTriggersError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => GetRepositoryTriggersError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => GetRepositoryTriggersError::EncryptionKeyDisabled(String::from(error_message)),
                    "EncryptionKeyNotFoundException" => GetRepositoryTriggersError::EncryptionKeyNotFound(String::from(error_message)),
                    "EncryptionKeyUnavailableException" => GetRepositoryTriggersError::EncryptionKeyUnavailable(String::from(error_message)),
                    "InvalidRepositoryNameException" => GetRepositoryTriggersError::InvalidRepositoryName(String::from(error_message)),
                    "RepositoryDoesNotExistException" => GetRepositoryTriggersError::RepositoryDoesNotExist(String::from(error_message)),
                    "RepositoryNameRequiredException" => GetRepositoryTriggersError::RepositoryNameRequired(String::from(error_message)),
                    "ValidationException" => {
                        GetRepositoryTriggersError::Validation(error_message.to_string())
                    }
                    _ => GetRepositoryTriggersError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRepositoryTriggersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRepositoryTriggersError {
    fn from(err: serde_json::error::Error) -> GetRepositoryTriggersError {
        GetRepositoryTriggersError::Unknown(err.description().to_string())
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
            GetRepositoryTriggersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBranches
#[derive(Debug, PartialEq)]
pub enum ListBranchesError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBranchesError {
    pub fn from_body(body: &str) -> ListBranchesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => ListBranchesError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => {
                        ListBranchesError::EncryptionKeyAccessDenied(String::from(error_message))
                    }
                    "EncryptionKeyDisabledException" => {
                        ListBranchesError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        ListBranchesError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => {
                        ListBranchesError::EncryptionKeyUnavailable(String::from(error_message))
                    }
                    "InvalidContinuationTokenException" => {
                        ListBranchesError::InvalidContinuationToken(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        ListBranchesError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => {
                        ListBranchesError::RepositoryDoesNotExist(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => {
                        ListBranchesError::RepositoryNameRequired(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListBranchesError::Validation(error_message.to_string())
                    }
                    _ => ListBranchesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBranchesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBranchesError {
    fn from(err: serde_json::error::Error) -> ListBranchesError {
        ListBranchesError::Unknown(err.description().to_string())
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
            ListBranchesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRepositories
#[derive(Debug, PartialEq)]
pub enum ListRepositoriesError {
    ///<p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    ///<p>The specified sort order is not valid.</p>
    InvalidOrder(String),
    ///<p>The specified sort by value is not valid.</p>
    InvalidSortBy(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRepositoriesError {
    pub fn from_body(body: &str) -> ListRepositoriesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidContinuationTokenException" => {
                        ListRepositoriesError::InvalidContinuationToken(String::from(error_message))
                    }
                    "InvalidOrderException" => {
                        ListRepositoriesError::InvalidOrder(String::from(error_message))
                    }
                    "InvalidSortByException" => {
                        ListRepositoriesError::InvalidSortBy(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRepositoriesError::Validation(error_message.to_string())
                    }
                    _ => ListRepositoriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRepositoriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRepositoriesError {
    fn from(err: serde_json::error::Error) -> ListRepositoriesError {
        ListRepositoriesError::Unknown(err.description().to_string())
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
            ListRepositoriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRepositoryTriggers
#[derive(Debug, PartialEq)]
pub enum PutRepositoryTriggersError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>One or more branch names specified for the trigger is not valid.</p>
    InvalidRepositoryTriggerBranchName(String),
    ///<p>The custom data provided for the trigger is not valid.</p>
    InvalidRepositoryTriggerCustomData(String),
    ///<p>The Amazon Resource Name (ARN) for the trigger is not valid for the specified destination. The most common reason for this error is that the ARN does not meet the requirements for the service type.</p>
    InvalidRepositoryTriggerDestinationArn(String),
    ///<p>One or more events specified for the trigger is not valid. Check to make sure that all events specified match the requirements for allowed events.</p>
    InvalidRepositoryTriggerEvents(String),
    ///<p>The name of the trigger is not valid.</p>
    InvalidRepositoryTriggerName(String),
    ///<p>The region for the trigger target does not match the region for the repository. Triggers must be created in the same region as the target for the trigger.</p>
    InvalidRepositoryTriggerRegion(String),
    ///<p>The number of branches for the trigger was exceeded.</p>
    MaximumBranchesExceeded(String),
    ///<p>The number of triggers allowed for the repository was exceeded.</p>
    MaximumRepositoryTriggersExceeded(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    ///<p>At least one branch name is required but was not specified in the trigger configuration.</p>
    RepositoryTriggerBranchNameListRequired(String),
    ///<p>A destination ARN for the target service for the trigger is required but was not specified.</p>
    RepositoryTriggerDestinationArnRequired(String),
    ///<p>At least one event for the trigger is required but was not specified.</p>
    RepositoryTriggerEventsListRequired(String),
    ///<p>A name for the trigger is required but was not specified.</p>
    RepositoryTriggerNameRequired(String),
    ///<p>The list of triggers for the repository is required but was not specified.</p>
    RepositoryTriggersListRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutRepositoryTriggersError {
    pub fn from_body(body: &str) -> PutRepositoryTriggersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => PutRepositoryTriggersError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => PutRepositoryTriggersError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => PutRepositoryTriggersError::EncryptionKeyDisabled(String::from(error_message)),
                    "EncryptionKeyNotFoundException" => PutRepositoryTriggersError::EncryptionKeyNotFound(String::from(error_message)),
                    "EncryptionKeyUnavailableException" => PutRepositoryTriggersError::EncryptionKeyUnavailable(String::from(error_message)),
                    "InvalidRepositoryNameException" => PutRepositoryTriggersError::InvalidRepositoryName(String::from(error_message)),
                    "InvalidRepositoryTriggerBranchNameException" => PutRepositoryTriggersError::InvalidRepositoryTriggerBranchName(String::from(error_message)),
                    "InvalidRepositoryTriggerCustomDataException" => PutRepositoryTriggersError::InvalidRepositoryTriggerCustomData(String::from(error_message)),
                    "InvalidRepositoryTriggerDestinationArnException" => PutRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(String::from(error_message)),
                    "InvalidRepositoryTriggerEventsException" => PutRepositoryTriggersError::InvalidRepositoryTriggerEvents(String::from(error_message)),
                    "InvalidRepositoryTriggerNameException" => PutRepositoryTriggersError::InvalidRepositoryTriggerName(String::from(error_message)),
                    "InvalidRepositoryTriggerRegionException" => PutRepositoryTriggersError::InvalidRepositoryTriggerRegion(String::from(error_message)),
                    "MaximumBranchesExceededException" => PutRepositoryTriggersError::MaximumBranchesExceeded(String::from(error_message)),
                    "MaximumRepositoryTriggersExceededException" => PutRepositoryTriggersError::MaximumRepositoryTriggersExceeded(String::from(error_message)),
                    "RepositoryDoesNotExistException" => PutRepositoryTriggersError::RepositoryDoesNotExist(String::from(error_message)),
                    "RepositoryNameRequiredException" => PutRepositoryTriggersError::RepositoryNameRequired(String::from(error_message)),
                    "RepositoryTriggerBranchNameListRequiredException" => PutRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(String::from(error_message)),
                    "RepositoryTriggerDestinationArnRequiredException" => PutRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(String::from(error_message)),
                    "RepositoryTriggerEventsListRequiredException" => PutRepositoryTriggersError::RepositoryTriggerEventsListRequired(String::from(error_message)),
                    "RepositoryTriggerNameRequiredException" => PutRepositoryTriggersError::RepositoryTriggerNameRequired(String::from(error_message)),
                    "RepositoryTriggersListRequiredException" => PutRepositoryTriggersError::RepositoryTriggersListRequired(String::from(error_message)),
                    "ValidationException" => {
                        PutRepositoryTriggersError::Validation(error_message.to_string())
                    }
                    _ => PutRepositoryTriggersError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRepositoryTriggersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRepositoryTriggersError {
    fn from(err: serde_json::error::Error) -> PutRepositoryTriggersError {
        PutRepositoryTriggersError::Unknown(err.description().to_string())
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
            PutRepositoryTriggersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestRepositoryTriggers
#[derive(Debug, PartialEq)]
pub enum TestRepositoryTriggersError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>One or more branch names specified for the trigger is not valid.</p>
    InvalidRepositoryTriggerBranchName(String),
    ///<p>The custom data provided for the trigger is not valid.</p>
    InvalidRepositoryTriggerCustomData(String),
    ///<p>The Amazon Resource Name (ARN) for the trigger is not valid for the specified destination. The most common reason for this error is that the ARN does not meet the requirements for the service type.</p>
    InvalidRepositoryTriggerDestinationArn(String),
    ///<p>One or more events specified for the trigger is not valid. Check to make sure that all events specified match the requirements for allowed events.</p>
    InvalidRepositoryTriggerEvents(String),
    ///<p>The name of the trigger is not valid.</p>
    InvalidRepositoryTriggerName(String),
    ///<p>The region for the trigger target does not match the region for the repository. Triggers must be created in the same region as the target for the trigger.</p>
    InvalidRepositoryTriggerRegion(String),
    ///<p>The number of branches for the trigger was exceeded.</p>
    MaximumBranchesExceeded(String),
    ///<p>The number of triggers allowed for the repository was exceeded.</p>
    MaximumRepositoryTriggersExceeded(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    ///<p>At least one branch name is required but was not specified in the trigger configuration.</p>
    RepositoryTriggerBranchNameListRequired(String),
    ///<p>A destination ARN for the target service for the trigger is required but was not specified.</p>
    RepositoryTriggerDestinationArnRequired(String),
    ///<p>At least one event for the trigger is required but was not specified.</p>
    RepositoryTriggerEventsListRequired(String),
    ///<p>A name for the trigger is required but was not specified.</p>
    RepositoryTriggerNameRequired(String),
    ///<p>The list of triggers for the repository is required but was not specified.</p>
    RepositoryTriggersListRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TestRepositoryTriggersError {
    pub fn from_body(body: &str) -> TestRepositoryTriggersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => TestRepositoryTriggersError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => TestRepositoryTriggersError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => TestRepositoryTriggersError::EncryptionKeyDisabled(String::from(error_message)),
                    "EncryptionKeyNotFoundException" => TestRepositoryTriggersError::EncryptionKeyNotFound(String::from(error_message)),
                    "EncryptionKeyUnavailableException" => TestRepositoryTriggersError::EncryptionKeyUnavailable(String::from(error_message)),
                    "InvalidRepositoryNameException" => TestRepositoryTriggersError::InvalidRepositoryName(String::from(error_message)),
                    "InvalidRepositoryTriggerBranchNameException" => TestRepositoryTriggersError::InvalidRepositoryTriggerBranchName(String::from(error_message)),
                    "InvalidRepositoryTriggerCustomDataException" => TestRepositoryTriggersError::InvalidRepositoryTriggerCustomData(String::from(error_message)),
                    "InvalidRepositoryTriggerDestinationArnException" => TestRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(String::from(error_message)),
                    "InvalidRepositoryTriggerEventsException" => TestRepositoryTriggersError::InvalidRepositoryTriggerEvents(String::from(error_message)),
                    "InvalidRepositoryTriggerNameException" => TestRepositoryTriggersError::InvalidRepositoryTriggerName(String::from(error_message)),
                    "InvalidRepositoryTriggerRegionException" => TestRepositoryTriggersError::InvalidRepositoryTriggerRegion(String::from(error_message)),
                    "MaximumBranchesExceededException" => TestRepositoryTriggersError::MaximumBranchesExceeded(String::from(error_message)),
                    "MaximumRepositoryTriggersExceededException" => TestRepositoryTriggersError::MaximumRepositoryTriggersExceeded(String::from(error_message)),
                    "RepositoryDoesNotExistException" => TestRepositoryTriggersError::RepositoryDoesNotExist(String::from(error_message)),
                    "RepositoryNameRequiredException" => TestRepositoryTriggersError::RepositoryNameRequired(String::from(error_message)),
                    "RepositoryTriggerBranchNameListRequiredException" => TestRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(String::from(error_message)),
                    "RepositoryTriggerDestinationArnRequiredException" => TestRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(String::from(error_message)),
                    "RepositoryTriggerEventsListRequiredException" => TestRepositoryTriggersError::RepositoryTriggerEventsListRequired(String::from(error_message)),
                    "RepositoryTriggerNameRequiredException" => TestRepositoryTriggersError::RepositoryTriggerNameRequired(String::from(error_message)),
                    "RepositoryTriggersListRequiredException" => TestRepositoryTriggersError::RepositoryTriggersListRequired(String::from(error_message)),
                    "ValidationException" => {
                        TestRepositoryTriggersError::Validation(error_message.to_string())
                    }
                    _ => TestRepositoryTriggersError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestRepositoryTriggersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestRepositoryTriggersError {
    fn from(err: serde_json::error::Error) -> TestRepositoryTriggersError {
        TestRepositoryTriggersError::Unknown(err.description().to_string())
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
            TestRepositoryTriggersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDefaultBranch
#[derive(Debug, PartialEq)]
pub enum UpdateDefaultBranchError {
    ///<p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    ///<p>A branch name is required but was not specified.</p>
    BranchNameRequired(String),
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified branch name is not valid.</p>
    InvalidBranchName(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateDefaultBranchError {
    pub fn from_body(body: &str) -> UpdateDefaultBranchError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BranchDoesNotExistException" => {
                        UpdateDefaultBranchError::BranchDoesNotExist(String::from(error_message))
                    }
                    "BranchNameRequiredException" => {
                        UpdateDefaultBranchError::BranchNameRequired(String::from(error_message))
                    }
                    "EncryptionIntegrityChecksFailedException" => UpdateDefaultBranchError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => UpdateDefaultBranchError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => {
                        UpdateDefaultBranchError::EncryptionKeyDisabled(String::from(error_message))
                    }
                    "EncryptionKeyNotFoundException" => {
                        UpdateDefaultBranchError::EncryptionKeyNotFound(String::from(error_message))
                    }
                    "EncryptionKeyUnavailableException" => UpdateDefaultBranchError::EncryptionKeyUnavailable(String::from(error_message)),
                    "InvalidBranchNameException" => {
                        UpdateDefaultBranchError::InvalidBranchName(String::from(error_message))
                    }
                    "InvalidRepositoryNameException" => {
                        UpdateDefaultBranchError::InvalidRepositoryName(String::from(error_message))
                    }
                    "RepositoryDoesNotExistException" => UpdateDefaultBranchError::RepositoryDoesNotExist(String::from(error_message)),
                    "RepositoryNameRequiredException" => UpdateDefaultBranchError::RepositoryNameRequired(String::from(error_message)),
                    "ValidationException" => {
                        UpdateDefaultBranchError::Validation(error_message.to_string())
                    }
                    _ => UpdateDefaultBranchError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDefaultBranchError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDefaultBranchError {
    fn from(err: serde_json::error::Error) -> UpdateDefaultBranchError {
        UpdateDefaultBranchError::Unknown(err.description().to_string())
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
            UpdateDefaultBranchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRepositoryDescription
#[derive(Debug, PartialEq)]
pub enum UpdateRepositoryDescriptionError {
    ///<p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailed(String),
    ///<p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDenied(String),
    ///<p>The encryption key is disabled.</p>
    EncryptionKeyDisabled(String),
    ///<p>No encryption key was found.</p>
    EncryptionKeyNotFound(String),
    ///<p>The encryption key is not available.</p>
    EncryptionKeyUnavailable(String),
    ///<p>The specified repository description is not valid.</p>
    InvalidRepositoryDescription(String),
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRepositoryDescriptionError {
    pub fn from_body(body: &str) -> UpdateRepositoryDescriptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EncryptionIntegrityChecksFailedException" => UpdateRepositoryDescriptionError::EncryptionIntegrityChecksFailed(String::from(error_message)),
                    "EncryptionKeyAccessDeniedException" => UpdateRepositoryDescriptionError::EncryptionKeyAccessDenied(String::from(error_message)),
                    "EncryptionKeyDisabledException" => UpdateRepositoryDescriptionError::EncryptionKeyDisabled(String::from(error_message)),
                    "EncryptionKeyNotFoundException" => UpdateRepositoryDescriptionError::EncryptionKeyNotFound(String::from(error_message)),
                    "EncryptionKeyUnavailableException" => UpdateRepositoryDescriptionError::EncryptionKeyUnavailable(String::from(error_message)),
                    "InvalidRepositoryDescriptionException" => UpdateRepositoryDescriptionError::InvalidRepositoryDescription(String::from(error_message)),
                    "InvalidRepositoryNameException" => UpdateRepositoryDescriptionError::InvalidRepositoryName(String::from(error_message)),
                    "RepositoryDoesNotExistException" => UpdateRepositoryDescriptionError::RepositoryDoesNotExist(String::from(error_message)),
                    "RepositoryNameRequiredException" => UpdateRepositoryDescriptionError::RepositoryNameRequired(String::from(error_message)),
                    "ValidationException" => {
                        UpdateRepositoryDescriptionError::Validation(error_message.to_string())
                    }
                    _ => UpdateRepositoryDescriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRepositoryDescriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRepositoryDescriptionError {
    fn from(err: serde_json::error::Error) -> UpdateRepositoryDescriptionError {
        UpdateRepositoryDescriptionError::Unknown(err.description().to_string())
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
            UpdateRepositoryDescriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRepositoryName
#[derive(Debug, PartialEq)]
pub enum UpdateRepositoryNameError {
    ///<p>At least one specified repository name is not valid.</p> <note> <p>This exception only occurs when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note>
    InvalidRepositoryName(String),
    ///<p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    ///<p>The specified repository name already exists.</p>
    RepositoryNameExists(String),
    ///<p>A repository name is required but was not specified.</p>
    RepositoryNameRequired(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRepositoryNameError {
    pub fn from_body(body: &str) -> UpdateRepositoryNameError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRepositoryNameException" => UpdateRepositoryNameError::InvalidRepositoryName(String::from(error_message)),
                    "RepositoryDoesNotExistException" => UpdateRepositoryNameError::RepositoryDoesNotExist(String::from(error_message)),
                    "RepositoryNameExistsException" => {
                        UpdateRepositoryNameError::RepositoryNameExists(String::from(error_message))
                    }
                    "RepositoryNameRequiredException" => UpdateRepositoryNameError::RepositoryNameRequired(String::from(error_message)),
                    "ValidationException" => {
                        UpdateRepositoryNameError::Validation(error_message.to_string())
                    }
                    _ => UpdateRepositoryNameError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRepositoryNameError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRepositoryNameError {
    fn from(err: serde_json::error::Error) -> UpdateRepositoryNameError {
        UpdateRepositoryNameError::Unknown(err.description().to_string())
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
            UpdateRepositoryNameError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CodeCommit API. CodeCommit clients implement this trait.
pub trait CodeCommit {
    #[doc="<p>Returns information about one or more repositories.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note>"]
    fn batch_get_repositories(&self,
                              input: &BatchGetRepositoriesInput)
                              -> Result<BatchGetRepositoriesOutput, BatchGetRepositoriesError>;


    #[doc="<p>Creates a new branch in a repository and points the branch to a commit.</p> <note> <p>Calling the create branch operation does not set a repository's default branch. To do this, call the update default branch operation.</p> </note>"]
    fn create_branch(&self, input: &CreateBranchInput) -> Result<(), CreateBranchError>;


    #[doc="<p>Creates a new, empty repository.</p>"]
    fn create_repository(&self,
                         input: &CreateRepositoryInput)
                         -> Result<CreateRepositoryOutput, CreateRepositoryError>;


    #[doc="<p>Deletes a repository. If a specified repository was already deleted, a null repository ID will be returned.</p> <important><p>Deleting a repository also deletes all associated objects and metadata. After a repository is deleted, all future push calls to the deleted repository will fail.</p> </important>"]
    fn delete_repository(&self,
                         input: &DeleteRepositoryInput)
                         -> Result<DeleteRepositoryOutput, DeleteRepositoryError>;


    #[doc="<p>Returns the base-64 encoded content of an individual blob within a repository.</p>"]
    fn get_blob(&self, input: &GetBlobInput) -> Result<GetBlobOutput, GetBlobError>;


    #[doc="<p>Returns information about a repository branch, including its name and the last commit ID.</p>"]
    fn get_branch(&self, input: &GetBranchInput) -> Result<GetBranchOutput, GetBranchError>;


    #[doc="<p>Returns information about a commit, including commit message and committer information.</p>"]
    fn get_commit(&self, input: &GetCommitInput) -> Result<GetCommitOutput, GetCommitError>;


    #[doc="<p>Returns information about the differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID or other fully qualified reference). Results can be limited to a specified path.</p>"]
    fn get_differences(&self,
                       input: &GetDifferencesInput)
                       -> Result<GetDifferencesOutput, GetDifferencesError>;


    #[doc="<p>Returns information about a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note>"]
    fn get_repository(&self,
                      input: &GetRepositoryInput)
                      -> Result<GetRepositoryOutput, GetRepositoryError>;


    #[doc="<p>Gets information about triggers configured for a repository.</p>"]
    fn get_repository_triggers
        (&self,
         input: &GetRepositoryTriggersInput)
         -> Result<GetRepositoryTriggersOutput, GetRepositoryTriggersError>;


    #[doc="<p>Gets information about one or more branches in a repository.</p>"]
    fn list_branches(&self,
                     input: &ListBranchesInput)
                     -> Result<ListBranchesOutput, ListBranchesError>;


    #[doc="<p>Gets information about one or more repositories.</p>"]
    fn list_repositories(&self,
                         input: &ListRepositoriesInput)
                         -> Result<ListRepositoriesOutput, ListRepositoriesError>;


    #[doc="<p>Replaces all triggers for a repository. This can be used to create or delete triggers.</p>"]
    fn put_repository_triggers
        (&self,
         input: &PutRepositoryTriggersInput)
         -> Result<PutRepositoryTriggersOutput, PutRepositoryTriggersError>;


    #[doc="<p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test will send data from the last commit. If no data is available, sample data will be generated.</p>"]
    fn test_repository_triggers
        (&self,
         input: &TestRepositoryTriggersInput)
         -> Result<TestRepositoryTriggersOutput, TestRepositoryTriggersError>;


    #[doc="<p>Sets or changes the default branch name for the specified repository.</p> <note> <p>If you use this operation to change the default branch name to the current default branch name, a success message is returned even though the default branch did not change.</p> </note>"]
    fn update_default_branch(&self,
                             input: &UpdateDefaultBranchInput)
                             -> Result<(), UpdateDefaultBranchError>;


    #[doc="<p>Sets or changes the comment or description for a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note>"]
    fn update_repository_description(&self,
                                     input: &UpdateRepositoryDescriptionInput)
                                     -> Result<(), UpdateRepositoryDescriptionError>;


    #[doc="<p>Renames a repository. The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix \".git\" is prohibited. For a full description of the limits on repository names, see <a href=\"http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html\">Limits</a> in the AWS CodeCommit User Guide.</p>"]
    fn update_repository_name(&self,
                              input: &UpdateRepositoryNameInput)
                              -> Result<(), UpdateRepositoryNameError>;
}
/// A client for the CodeCommit API.
pub struct CodeCommitClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CodeCommitClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CodeCommitClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CodeCommit for CodeCommitClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Returns information about one or more repositories.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note>"]
    fn batch_get_repositories(&self,
                              input: &BatchGetRepositoriesInput)
                              -> Result<BatchGetRepositoriesOutput, BatchGetRepositoriesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.BatchGetRepositories");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<BatchGetRepositoriesOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BatchGetRepositoriesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new branch in a repository and points the branch to a commit.</p> <note> <p>Calling the create branch operation does not set a repository's default branch. To do this, call the update default branch operation.</p> </note>"]
    fn create_branch(&self, input: &CreateBranchInput) -> Result<(), CreateBranchError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateBranch");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateBranchError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new, empty repository.</p>"]
    fn create_repository(&self,
                         input: &CreateRepositoryInput)
                         -> Result<CreateRepositoryOutput, CreateRepositoryError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateRepository");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateRepositoryOutput>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateRepositoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a repository. If a specified repository was already deleted, a null repository ID will be returned.</p> <important><p>Deleting a repository also deletes all associated objects and metadata. After a repository is deleted, all future push calls to the deleted repository will fail.</p> </important>"]
    fn delete_repository(&self,
                         input: &DeleteRepositoryInput)
                         -> Result<DeleteRepositoryOutput, DeleteRepositoryError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteRepository");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteRepositoryOutput>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteRepositoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the base-64 encoded content of an individual blob within a repository.</p>"]
    fn get_blob(&self, input: &GetBlobInput) -> Result<GetBlobOutput, GetBlobError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetBlob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetBlobOutput>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBlobError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about a repository branch, including its name and the last commit ID.</p>"]
    fn get_branch(&self, input: &GetBranchInput) -> Result<GetBranchOutput, GetBranchError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetBranch");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetBranchOutput>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBranchError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about a commit, including commit message and committer information.</p>"]
    fn get_commit(&self, input: &GetCommitInput) -> Result<GetCommitOutput, GetCommitError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetCommit");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetCommitOutput>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCommitError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about the differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID or other fully qualified reference). Results can be limited to a specified path.</p>"]
    fn get_differences(&self,
                       input: &GetDifferencesInput)
                       -> Result<GetDifferencesOutput, GetDifferencesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetDifferences");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetDifferencesOutput>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDifferencesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note>"]
    fn get_repository(&self,
                      input: &GetRepositoryInput)
                      -> Result<GetRepositoryOutput, GetRepositoryError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetRepository");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetRepositoryOutput>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetRepositoryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about triggers configured for a repository.</p>"]
    fn get_repository_triggers
        (&self,
         input: &GetRepositoryTriggersInput)
         -> Result<GetRepositoryTriggersOutput, GetRepositoryTriggersError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetRepositoryTriggers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetRepositoryTriggersOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetRepositoryTriggersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about one or more branches in a repository.</p>"]
    fn list_branches(&self,
                     input: &ListBranchesInput)
                     -> Result<ListBranchesOutput, ListBranchesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListBranches");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListBranchesOutput>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListBranchesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about one or more repositories.</p>"]
    fn list_repositories(&self,
                         input: &ListRepositoriesInput)
                         -> Result<ListRepositoriesOutput, ListRepositoriesError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListRepositories");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListRepositoriesOutput>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListRepositoriesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Replaces all triggers for a repository. This can be used to create or delete triggers.</p>"]
    fn put_repository_triggers
        (&self,
         input: &PutRepositoryTriggersInput)
         -> Result<PutRepositoryTriggersOutput, PutRepositoryTriggersError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PutRepositoryTriggers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutRepositoryTriggersOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutRepositoryTriggersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test will send data from the last commit. If no data is available, sample data will be generated.</p>"]
    fn test_repository_triggers
        (&self,
         input: &TestRepositoryTriggersInput)
         -> Result<TestRepositoryTriggersOutput, TestRepositoryTriggersError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.TestRepositoryTriggers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<TestRepositoryTriggersOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(TestRepositoryTriggersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sets or changes the default branch name for the specified repository.</p> <note> <p>If you use this operation to change the default branch name to the current default branch name, a success message is returned even though the default branch did not change.</p> </note>"]
    fn update_default_branch(&self,
                             input: &UpdateDefaultBranchInput)
                             -> Result<(), UpdateDefaultBranchError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateDefaultBranch");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateDefaultBranchError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sets or changes the comment or description for a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a web page could expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a web page.</p> </note>"]
    fn update_repository_description(&self,
                                     input: &UpdateRepositoryDescriptionInput)
                                     -> Result<(), UpdateRepositoryDescriptionError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "CodeCommit_20150413.UpdateRepositoryDescription");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateRepositoryDescriptionError::from_body(String::from_utf8_lossy(&body)
                                                                    .as_ref()))
            }
        }
    }


    #[doc="<p>Renames a repository. The repository name must be unique across the calling AWS account. In addition, repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix \".git\" is prohibited. For a full description of the limits on repository names, see <a href=\"http://docs.aws.amazon.com/codecommit/latest/userguide/limits.html\">Limits</a> in the AWS CodeCommit User Guide.</p>"]
    fn update_repository_name(&self,
                              input: &UpdateRepositoryNameInput)
                              -> Result<(), UpdateRepositoryNameError> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateRepositoryName");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateRepositoryNameError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
