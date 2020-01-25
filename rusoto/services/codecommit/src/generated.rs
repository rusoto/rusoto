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
/// <p>Returns information about a specific approval on a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Approval {
    /// <p>The state of the approval, APPROVE or REVOKE. REVOKE states are not stored.</p>
    #[serde(rename = "approvalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_state: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user.</p>
    #[serde(rename = "userArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

/// <p>Returns information about an approval rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApprovalRule {
    /// <p>The content of the approval rule.</p>
    #[serde(rename = "approvalRuleContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_content: Option<String>,
    /// <p>The system-generated ID of the approval rule.</p>
    #[serde(rename = "approvalRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_id: Option<String>,
    /// <p>The name of the approval rule.</p>
    #[serde(rename = "approvalRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_name: Option<String>,
    /// <p>The date the approval rule was created, in timestamp format.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date the approval rule was most recently changed, in timestamp format.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the user who made the most recent changes to the approval rule.</p>
    #[serde(rename = "lastModifiedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// <p>The approval rule template used to create the rule.</p>
    #[serde(rename = "originApprovalRuleTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_approval_rule_template: Option<OriginApprovalRuleTemplate>,
    /// <p>The SHA-256 hash signature for the content of the approval rule.</p>
    #[serde(rename = "ruleContentSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_content_sha_256: Option<String>,
}

/// <p>Returns information about an event for an approval rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApprovalRuleEventMetadata {
    /// <p>The content of the approval rule.</p>
    #[serde(rename = "approvalRuleContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_content: Option<String>,
    /// <p>The system-generated ID of the approval rule.</p>
    #[serde(rename = "approvalRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_id: Option<String>,
    /// <p>The name of the approval rule.</p>
    #[serde(rename = "approvalRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_name: Option<String>,
}

/// <p>Returns information about an override event for approval rules for a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApprovalRuleOverriddenEventMetadata {
    /// <p>The status of the override event.</p>
    #[serde(rename = "overrideStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_status: Option<String>,
    /// <p>The revision ID of the pull request when the override event occurred.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

/// <p>Returns information about an approval rule template.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApprovalRuleTemplate {
    /// <p>The content of the approval rule template.</p>
    #[serde(rename = "approvalRuleTemplateContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_content: Option<String>,
    /// <p>The description of the approval rule template.</p>
    #[serde(rename = "approvalRuleTemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_description: Option<String>,
    /// <p>The system-generated ID of the approval rule template.</p>
    #[serde(rename = "approvalRuleTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_id: Option<String>,
    /// <p>The name of the approval rule template.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_name: Option<String>,
    /// <p>The date the approval rule template was created, in timestamp format.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date the approval rule template was most recently changed, in timestamp format.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the user who made the most recent changes to the approval rule template.</p>
    #[serde(rename = "lastModifiedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// <p>The SHA-256 hash signature for the content of the approval rule template.</p>
    #[serde(rename = "ruleContentSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_content_sha_256: Option<String>,
}

/// <p>Returns information about a change in the approval state for a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApprovalStateChangedEventMetadata {
    /// <p>The approval status for the pull request.</p>
    #[serde(rename = "approvalStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<String>,
    /// <p>The revision ID of the pull request when the approval state changed.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateApprovalRuleTemplateWithRepositoryInput {
    /// <p>The name for the approval rule template. </p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
    /// <p>The name of the repository that you want to associate with the template.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Returns information about errors in a BatchAssociateApprovalRuleTemplateWithRepositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeCommitBatchAssociateApprovalRuleTemplateWithRepositoriesError {
    /// <p>An error code that specifies whether the repository name was not valid or not found.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error message that provides details about why the repository name was not found or not valid.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The name of the repository where the association was not made.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchAssociateApprovalRuleTemplateWithRepositoriesInput {
    /// <p>The name of the template you want to associate with one or more repositories.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
    /// <p><p>The names of the repositories you want to associate with the template.</p> <note> <p>The length constraint limit is for each string in the array. The array itself can be empty.</p> </note></p>
    #[serde(rename = "repositoryNames")]
    pub repository_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAssociateApprovalRuleTemplateWithRepositoriesOutput {
    /// <p>A list of names of the repositories that have been associated with the template.</p>
    #[serde(rename = "associatedRepositoryNames")]
    pub associated_repository_names: Vec<String>,
    /// <p>A list of any errors that might have occurred while attempting to create the association between the template and the repositories.</p>
    #[serde(rename = "errors")]
    pub errors: Vec<CodeCommitBatchAssociateApprovalRuleTemplateWithRepositoriesError>,
}

/// <p>Returns information about errors in a BatchDescribeMergeConflicts operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeCommitBatchDescribeMergeConflictsError {
    /// <p>The name of the exception.</p>
    #[serde(rename = "exceptionName")]
    pub exception_name: String,
    /// <p>The path to the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The message provided by the exception.</p>
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDescribeMergeConflictsInput {
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The path of the target files used to describe the conflicts. If not specified, the default is all conflict files.</p>
    #[serde(rename = "filePaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<Vec<String>>,
    /// <p>The maximum number of files to include in the output.</p>
    #[serde(rename = "maxConflictFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conflict_files: Option<i64>,
    /// <p>The maximum number of merge hunks to include in the output.</p>
    #[serde(rename = "maxMergeHunks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_merge_hunks: Option<i64>,
    /// <p>The merge option or strategy you want to use to merge the code.</p>
    #[serde(rename = "mergeOption")]
    pub merge_option: String,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the repository that contains the merge conflicts you want to review.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDescribeMergeConflictsOutput {
    /// <p>The commit ID of the merge base.</p>
    #[serde(rename = "baseCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    /// <p>A list of conflicts for each file, including the conflict metadata and the hunks of the differences between the files.</p>
    #[serde(rename = "conflicts")]
    pub conflicts: Vec<Conflict>,
    /// <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "destinationCommitId")]
    pub destination_commit_id: String,
    /// <p>A list of any errors returned while describing the merge conflicts for each file.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CodeCommitBatchDescribeMergeConflictsError>>,
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "sourceCommitId")]
    pub source_commit_id: String,
}

/// <p>Returns information about errors in a BatchDisassociateApprovalRuleTemplateFromRepositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeCommitBatchDisassociateApprovalRuleTemplateFromRepositoriesError {
    /// <p>An error code that specifies whether the repository name was not valid or not found.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error message that provides details about why the repository name was either not found or not valid.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The name of the repository where the association with the template was not able to be removed.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDisassociateApprovalRuleTemplateFromRepositoriesInput {
    /// <p>The name of the template that you want to disassociate from one or more repositories.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
    /// <p><p>The repository names that you want to disassociate from the approval rule template.</p> <note> <p>The length constraint limit is for each string in the array. The array itself can be empty.</p> </note></p>
    #[serde(rename = "repositoryNames")]
    pub repository_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput {
    /// <p>A list of repository names that have had their association with the template removed.</p>
    #[serde(rename = "disassociatedRepositoryNames")]
    pub disassociated_repository_names: Vec<String>,
    /// <p>A list of any errors that might have occurred while attempting to remove the association between the template and the repositories.</p>
    #[serde(rename = "errors")]
    pub errors: Vec<CodeCommitBatchDisassociateApprovalRuleTemplateFromRepositoriesError>,
}

/// <p>Returns information about errors in a BatchGetCommits operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeCommitBatchGetCommitsError {
    /// <p>A commit ID that either could not be found or was not in a valid format.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>An error code that specifies whether the commit ID was not valid or not found.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error message that provides detail about why the commit ID either was not found or was not valid.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetCommitsInput {
    /// <p><p>The full commit IDs of the commits to get information about.</p> <note> <p>You must supply the full SHA IDs of each commit. You cannot use shortened SHA IDs.</p> </note></p>
    #[serde(rename = "commitIds")]
    pub commit_ids: Vec<String>,
    /// <p>The name of the repository that contains the commits.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetCommitsOutput {
    /// <p>An array of commit data type objects, each of which contains information about a specified commit.</p>
    #[serde(rename = "commits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Commit>>,
    /// <p>Returns any commit IDs for which information could not be found. For example, if one of the commit IDs was a shortened SHA ID or that commit was not found in the specified repository, the ID returns an error object with more information.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CodeCommitBatchGetCommitsError>>,
}

/// <p>Represents the input of a batch get repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetRepositoriesInput {
    /// <p><p>The names of the repositories to get information about.</p> <note> <p>The length constraint limit is for each string in the array. The array itself can be empty.</p> </note></p>
    #[serde(rename = "repositoryNames")]
    pub repository_names: Vec<String>,
}

/// <p>Represents the output of a batch get repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BlobMetadata {
    /// <p>The full ID of the blob.</p>
    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    /// <p><p>The file mode permissions of the blob. File mode permission codes include:</p> <ul> <li> <p> <code>100644</code> indicates read/write</p> </li> <li> <p> <code>100755</code> indicates read/write/execute</p> </li> <li> <p> <code>160000</code> indicates a submodule</p> </li> <li> <p> <code>120000</code> indicates a symlink</p> </li> </ul></p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>The path to the blob and associated file name, if any.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>Returns information about a branch.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Comment {
    /// <p>The Amazon Resource Name (ARN) of the person who posted the comment.</p>
    #[serde(rename = "authorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    /// <p>A unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommentsForComparedCommit {
    /// <p>The full blob ID of the commit used to establish the after of the comparison.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>The full commit ID of the commit used to establish the after of the comparison.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full blob ID of the commit used to establish the before of the comparison.</p>
    #[serde(rename = "beforeBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    /// <p>The full commit ID of the commit used to establish the before of the comparison.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>An array of comment objects. Each comment object contains information about a comment on the comparison between commits.</p>
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    /// <p>Location information about the comment on the comparison, including the file name, line number, and whether the version of the file where the comment was made is BEFORE or AFTER.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommentsForPullRequest {
    /// <p>The full blob ID of the file on which you want to comment on the source commit.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>The full commit ID of the commit that was the tip of the source branch at the time the comment was made. </p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full blob ID of the file on which you want to comment on the destination commit.</p>
    #[serde(rename = "beforeBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    /// <p>The full commit ID of the commit that was the tip of the destination branch when the pull request was created. This commit is superceded by the after commit in the source branch when and if you merge the source branch into the destination branch.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>An array of comment objects. Each comment object contains information about a comment on the pull request.</p>
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    /// <p>Location information about the comment on the pull request, including the file name, line number, and whether the version of the file where the comment was made is BEFORE (destination branch) or AFTER (source branch).</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Commit {
    /// <p>Any other data associated with the specified commit.</p>
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    /// <p>Information about the author of the specified commit. Information includes the date in timestamp format with GMT offset, the name of the author, and the email address for the author, as configured in Git.</p>
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserInfo>,
    /// <p>The full SHA ID of the specified commit. </p>
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

/// <p>Information about conflicts in a merge operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Conflict {
    /// <p>Metadata about a conflict in a merge operation.</p>
    #[serde(rename = "conflictMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_metadata: Option<ConflictMetadata>,
    /// <p>A list of hunks that contain the differences between files or lines causing the conflict.</p>
    #[serde(rename = "mergeHunks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_hunks: Option<Vec<MergeHunk>>,
}

/// <p>Information about the metadata for a conflict in a merge operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConflictMetadata {
    /// <p>A boolean value indicating whether there are conflicts in the content of a file.</p>
    #[serde(rename = "contentConflict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_conflict: Option<bool>,
    /// <p>A boolean value indicating whether there are conflicts in the file mode of a file.</p>
    #[serde(rename = "fileModeConflict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode_conflict: Option<bool>,
    /// <p>The file modes of the file in the source, destination, and base of the merge.</p>
    #[serde(rename = "fileModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_modes: Option<FileModes>,
    /// <p>The path of the file that contains conflicts.</p>
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// <p>The file sizes of the file in the source, destination, and base of the merge.</p>
    #[serde(rename = "fileSizes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_sizes: Option<FileSizes>,
    /// <p>A boolean value (true or false) indicating whether the file is binary or textual in the source, destination, and base of the merge.</p>
    #[serde(rename = "isBinaryFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_binary_file: Option<IsBinaryFile>,
    /// <p>Whether an add, modify, or delete operation caused the conflict between the source and destination of the merge.</p>
    #[serde(rename = "mergeOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_operations: Option<MergeOperations>,
    /// <p>The number of conflicts, including both hunk conflicts and metadata conflicts.</p>
    #[serde(rename = "numberOfConflicts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_conflicts: Option<i64>,
    /// <p>A boolean value (true or false) indicating whether there are conflicts between the branches in the object type of a file, folder, or submodule.</p>
    #[serde(rename = "objectTypeConflict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type_conflict: Option<bool>,
    /// <p>Information about any object type conflicts in a merge operation.</p>
    #[serde(rename = "objectTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<ObjectTypes>,
}

/// <p>If AUTOMERGE is the conflict resolution strategy, a list of inputs to use when resolving conflicts during a merge.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConflictResolution {
    /// <p>Files to be deleted as part of the merge conflict resolution.</p>
    #[serde(rename = "deleteFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<Vec<DeleteFileEntry>>,
    /// <p>Files to have content replaced as part of the merge conflict resolution.</p>
    #[serde(rename = "replaceContents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_contents: Option<Vec<ReplaceContentEntry>>,
    /// <p>File modes that are set as part of the merge conflict resolution.</p>
    #[serde(rename = "setFileModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_file_modes: Option<Vec<SetFileModeEntry>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApprovalRuleTemplateInput {
    /// <p><p>The content of the approval rule that is created on pull requests in associated repositories. If you specify one or more destination references (branches), approval rules are created in an associated repository only if their destination references (branches) match those specified in the template.</p> <note> <p>When you create the content of the approval rule template, you can specify approvers in an approval pool in one of two ways:</p> <ul> <li> <p> <b>CodeCommitApprovers</b>: This option only requires an AWS account and a resource. It can be used for both IAM users and federated access users whose name matches the provided resource name. This is a very powerful option that offers a great deal of flexibility. For example, if you specify the AWS account <i>123456789012</i> and <i>Mary<em>Major</i>, all of the following are counted as approvals coming from that user:</p> <ul> <li> <p>An IAM user in the account (arn:aws:iam::<i>123456789012</i>:user/<i>Mary</em>Major</i>)</p> </li> <li> <p>A federated user identified in IAM as Mary<em>Major (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary</em>Major</i>)</p> </li> </ul> <p>This option does not recognize an active session of someone assuming the role of CodeCommitReview with a role session name of <i>Mary<em>Major</i> (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary</em>Major</i>) unless you include a wildcard (*Mary<em>Major).</p> </li> <li> <p> <b>Fully qualified ARN</b>: This option allows you to specify the fully qualified Amazon Resource Name (ARN) of the IAM user or role. </p> </li> </ul> <p>For more information about IAM ARNs, wildcards, and formats, see &lt;a href=&quot;https://docs.aws.amazon.com/iam/latest/UserGuide/reference</em>identifiers.html&quot;&gt;IAM Identifiers</a> in the <i>IAM User Guide</i>.</p> </note></p>
    #[serde(rename = "approvalRuleTemplateContent")]
    pub approval_rule_template_content: String,
    /// <p>The description of the approval rule template. Consider providing a description that explains what this template does and when it might be appropriate to associate it with repositories.</p>
    #[serde(rename = "approvalRuleTemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_description: Option<String>,
    /// <p>The name of the approval rule template. Provide descriptive names, because this name is applied to the approval rules created automatically in associated repositories.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApprovalRuleTemplateOutput {
    /// <p>The content and structure of the created approval rule template.</p>
    #[serde(rename = "approvalRuleTemplate")]
    pub approval_rule_template: ApprovalRuleTemplate,
}

/// <p>Represents the input of a create branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCommitInput {
    /// <p>The name of the author who created the commit. This information is used as both the author and committer for the commit.</p>
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// <p>The name of the branch where you create the commit.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>The commit message you want to include in the commit. Commit messages are limited to 256 KB. If no message is specified, a default message is used.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The files to delete in this commit. These files still exist in earlier commits.</p>
    #[serde(rename = "deleteFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<Vec<DeleteFileEntry>>,
    /// <p>The email address of the person who created the commit.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If true, a ..gitkeep file is created for empty folders. The default is false.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    /// <p>The ID of the commit that is the parent of the commit you create. Not required if this is an empty repository.</p>
    #[serde(rename = "parentCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_commit_id: Option<String>,
    /// <p>The files to add or update in this commit.</p>
    #[serde(rename = "putFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_files: Option<Vec<PutFileEntry>>,
    /// <p>The name of the repository where you create the commit.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The file modes to update for files in this commit.</p>
    #[serde(rename = "setFileModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_file_modes: Option<Vec<SetFileModeEntry>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePullRequestApprovalRuleInput {
    /// <p><p>The content of the approval rule, including the number of approvals needed and the structure of an approval pool defined for approvals, if any. For more information about approval pools, see the AWS CodeCommit User Guide.</p> <note> <p>When you create the content of the approval rule, you can specify approvers in an approval pool in one of two ways:</p> <ul> <li> <p> <b>CodeCommitApprovers</b>: This option only requires an AWS account and a resource. It can be used for both IAM users and federated access users whose name matches the provided resource name. This is a very powerful option that offers a great deal of flexibility. For example, if you specify the AWS account <i>123456789012</i> and <i>Mary<em>Major</i>, all of the following would be counted as approvals coming from that user:</p> <ul> <li> <p>An IAM user in the account (arn:aws:iam::<i>123456789012</i>:user/<i>Mary</em>Major</i>)</p> </li> <li> <p>A federated user identified in IAM as Mary<em>Major (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary</em>Major</i>)</p> </li> </ul> <p>This option does not recognize an active session of someone assuming the role of CodeCommitReview with a role session name of <i>Mary<em>Major</i> (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary</em>Major</i>) unless you include a wildcard (*Mary<em>Major).</p> </li> <li> <p> <b>Fully qualified ARN</b>: This option allows you to specify the fully qualified Amazon Resource Name (ARN) of the IAM user or role. </p> </li> </ul> <p>For more information about IAM ARNs, wildcards, and formats, see &lt;a href=&quot;https://docs.aws.amazon.com/iam/latest/UserGuide/reference</em>identifiers.html&quot;&gt;IAM Identifiers</a> in the <i>IAM User Guide</i>.</p> </note></p>
    #[serde(rename = "approvalRuleContent")]
    pub approval_rule_content: String,
    /// <p>The name for the approval rule.</p>
    #[serde(rename = "approvalRuleName")]
    pub approval_rule_name: String,
    /// <p>The system-generated ID of the pull request for which you want to create the approval rule.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePullRequestApprovalRuleOutput {
    /// <p>Information about the created approval rule.</p>
    #[serde(rename = "approvalRule")]
    pub approval_rule: ApprovalRule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePullRequestInput {
    /// <p><p>A unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p> <note> <p>The AWS SDKs prepopulate client request tokens. If you are using an AWS SDK, an idempotency token is created for you.</p> </note></p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>A description of the pull request.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The targets for the pull request, including the source of the code to be reviewed (the source branch) and the destination where the creator of the pull request intends the code to be merged after the pull request is closed (the destination branch).</p>
    #[serde(rename = "targets")]
    pub targets: Vec<Target>,
    /// <p>The title of the pull request. This title is used to identify the pull request to other users in the repository.</p>
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePullRequestOutput {
    /// <p>Information about the newly created pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

/// <p>Represents the input of a create repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRepositoryInput {
    /// <p><p>A comment or description about the new repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p> </note></p>
    #[serde(rename = "repositoryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    /// <p><p>The name of the new repository to be created.</p> <note> <p>The repository name must be unique across the calling AWS account. Repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. For more information about the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the <i>AWS CodeCommit User Guide</i>. The suffix .git is prohibited.</p> </note></p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>One or more tag key-value pairs to use when tagging this repository.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents the output of a create repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRepositoryOutput {
    /// <p>Information about the newly created repository.</p>
    #[serde(rename = "repositoryMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUnreferencedMergeCommitInput {
    /// <p>The name of the author who created the unreferenced commit. This information is used as both the author and committer for the commit.</p>
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// <p>The commit message for the unreferenced commit.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>If AUTOMERGE is the conflict resolution strategy, a list of inputs to use when resolving conflicts during a merge.</p>
    #[serde(rename = "conflictResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The email address for the person who created the unreferenced commit.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If this is specified as true, a .gitkeep file is created for empty folders. The default is false.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    /// <p>The merge option or strategy you want to use to merge the code.</p>
    #[serde(rename = "mergeOption")]
    pub merge_option: String,
    /// <p>The name of the repository where you want to create the unreferenced merge commit.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUnreferencedMergeCommitOutput {
    /// <p>The full commit ID of the commit that contains your merge results.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains the merge results.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApprovalRuleTemplateInput {
    /// <p>The name of the approval rule template to delete.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApprovalRuleTemplateOutput {
    /// <p>The system-generated ID of the deleted approval rule template. If the template has been previously deleted, the only response is a 200 OK.</p>
    #[serde(rename = "approvalRuleTemplateId")]
    pub approval_rule_template_id: String,
}

/// <p>Represents the input of a delete branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBranchOutput {
    /// <p>Information about the branch deleted by the operation, including the branch name and the commit ID that was the tip of the branch.</p>
    #[serde(rename = "deletedBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_branch: Option<BranchInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCommentContentInput {
    /// <p>The unique, system-generated ID of the comment. To get this ID, use <a>GetCommentsForComparedCommit</a> or <a>GetCommentsForPullRequest</a>.</p>
    #[serde(rename = "commentId")]
    pub comment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCommentContentOutput {
    /// <p>Information about the comment you just deleted.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>A file that is deleted as part of a commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileEntry {
    /// <p>The full path of the file to be deleted, including the name of the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileInput {
    /// <p>The name of the branch where the commit that deletes the file is made.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>The commit message you want to include as part of deleting the file. Commit messages are limited to 256 KB. If no message is specified, a default message is used.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The email address for the commit that deletes the file. If no email address is specified, the email address is left blank.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The fully qualified path to the file that to be deleted, including the full name and extension of that file. For example, /examples/file.md is a fully qualified path to a file named file.md in a folder named examples.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>If a file is the only object in the folder or directory, specifies whether to delete the folder or directory that contains the file. By default, empty folders are deleted. This includes empty folders that are part of the directory structure. For example, if the path to a file is dir1/dir2/dir3/dir4, and dir2 and dir3 are empty, deleting the last file in dir4 also deletes the empty folders dir4, dir3, and dir2.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    /// <p>The name of the author of the commit that deletes the file. If no name is specified, the user's ARN is used as the author name and committer name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the commit that is the tip of the branch where you want to create the commit that deletes the file. This must be the HEAD commit for the branch. The commit that deletes the file is created from this commit ID.</p>
    #[serde(rename = "parentCommitId")]
    pub parent_commit_id: String,
    /// <p>The name of the repository that contains the file to delete.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFileOutput {
    /// <p>The blob ID removed from the tree as part of deleting the file.</p>
    #[serde(rename = "blobId")]
    pub blob_id: String,
    /// <p>The full commit ID of the commit that contains the change that deletes the file.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The fully qualified path to the file to be deleted, including the full name and extension of that file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains the delete file change.</p>
    #[serde(rename = "treeId")]
    pub tree_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePullRequestApprovalRuleInput {
    /// <p>The name of the approval rule you want to delete.</p>
    #[serde(rename = "approvalRuleName")]
    pub approval_rule_name: String,
    /// <p>The system-generated ID of the pull request that contains the approval rule you want to delete.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePullRequestApprovalRuleOutput {
    /// <p><p>The ID of the deleted approval rule. </p> <note> <p>If the approval rule was deleted in an earlier API call, the response is 200 OK without content.</p> </note></p>
    #[serde(rename = "approvalRuleId")]
    pub approval_rule_id: String,
}

/// <p>Represents the input of a delete repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRepositoryInput {
    /// <p>The name of the repository to delete.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a delete repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRepositoryOutput {
    /// <p>The ID of the repository that was deleted.</p>
    #[serde(rename = "repositoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMergeConflictsInput {
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The path of the target files used to describe the conflicts. </p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The maximum number of merge hunks to include in the output.</p>
    #[serde(rename = "maxMergeHunks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_merge_hunks: Option<i64>,
    /// <p>The merge option or strategy you want to use to merge the code.</p>
    #[serde(rename = "mergeOption")]
    pub merge_option: String,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the repository where you want to get information about a merge conflict.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMergeConflictsOutput {
    /// <p>The commit ID of the merge base.</p>
    #[serde(rename = "baseCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    /// <p>Contains metadata about the conflicts found in the merge.</p>
    #[serde(rename = "conflictMetadata")]
    pub conflict_metadata: ConflictMetadata,
    /// <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "destinationCommitId")]
    pub destination_commit_id: String,
    /// <p>A list of merge hunks of the differences between the files or lines.</p>
    #[serde(rename = "mergeHunks")]
    pub merge_hunks: Vec<MergeHunk>,
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "sourceCommitId")]
    pub source_commit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePullRequestEventsInput {
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with more commits or changing the status of a pull request.</p>
    #[serde(rename = "actorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_arn: Option<String>,
    /// <p>A non-zero, non-negative integer used to limit the number of returned results. The default is 100 events, which is also the maximum number of events that can be returned in a result.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateApprovalRuleTemplateFromRepositoryInput {
    /// <p>The name of the approval rule template to disassociate from a specified repository.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
    /// <p>The name of the repository you want to disassociate from the template.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EvaluatePullRequestApprovalRulesInput {
    /// <p>The system-generated ID of the pull request you want to evaluate.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The system-generated ID for the pull request revision. To retrieve the most recent revision ID for a pull request, use <a>GetPullRequest</a>.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EvaluatePullRequestApprovalRulesOutput {
    /// <p>The result of the evaluation, including the names of the rules whose conditions have been met (if any), the names of the rules whose conditions have not been met (if any), whether the pull request is in the approved state, and whether the pull request approval rule has been set aside by an override. </p>
    #[serde(rename = "evaluation")]
    pub evaluation: Evaluation,
}

/// <p>Returns information about the approval rules applied to a pull request and whether conditions have been met.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Evaluation {
    /// <p>The names of the approval rules that have not had their conditions met.</p>
    #[serde(rename = "approvalRulesNotSatisfied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules_not_satisfied: Option<Vec<String>>,
    /// <p>The names of the approval rules that have had their conditions met.</p>
    #[serde(rename = "approvalRulesSatisfied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules_satisfied: Option<Vec<String>>,
    /// <p>Whether the state of the pull request is approved.</p>
    #[serde(rename = "approved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    /// <p>Whether the approval rule requirements for the pull request have been overridden and no longer need to be met.</p>
    #[serde(rename = "overridden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
}

/// <p>Returns information about a file in a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct File {
    /// <p>The fully qualified path to the file in the repository.</p>
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

/// <p>A file to be added, updated, or deleted as part of a commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileMetadata {
    /// <p>The full path to the file to be added or updated, including the name of the file.</p>
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

/// <p>Information about file modes in a merge or pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileModes {
    /// <p>The file mode of a file in the base of a merge or pull request.</p>
    #[serde(rename = "base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// <p>The file mode of a file in the destination of a merge or pull request.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The file mode of a file in the source of a merge or pull request.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// <p>Information about the size of files in a merge or pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSizes {
    /// <p>The size of a file in the base of a merge or pull request.</p>
    #[serde(rename = "base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i64>,
    /// <p>The size of a file in the destination of a merge or pull request.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<i64>,
    /// <p>The size of a file in the source of a merge or pull request.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i64>,
}

/// <p>Returns information about a folder in a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Folder {
    /// <p>The fully qualified path of the folder in the repository.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApprovalRuleTemplateInput {
    /// <p>The name of the approval rule template for which you want to get information.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApprovalRuleTemplateOutput {
    /// <p>The content and structure of the approval rule template.</p>
    #[serde(rename = "approvalRuleTemplate")]
    pub approval_rule_template: ApprovalRuleTemplate,
}

/// <p>Represents the input of a get blob operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBranchOutput {
    /// <p>The name of the branch.</p>
    #[serde(rename = "branch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCommentInput {
    /// <p>The unique, system-generated ID of the comment. To get this ID, use <a>GetCommentsForComparedCommit</a> or <a>GetCommentsForPullRequest</a>.</p>
    #[serde(rename = "commentId")]
    pub comment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCommentOutput {
    /// <p>The contents of the comment.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCommentsForComparedCommitInput {
    /// <p>To establish the directionality of the comparison, the full commit ID of the after commit.</p>
    #[serde(rename = "afterCommitId")]
    pub after_commit_id: String,
    /// <p>To establish the directionality of the comparison, the full commit ID of the before commit.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>A non-zero, non-negative integer used to limit the number of returned results. The default is 100 comments, but you can configure up to 500.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCommentsForPullRequestInput {
    /// <p>The full commit ID of the commit in the source branch that was the tip of the branch at the time the comment was made.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>The full commit ID of the commit in the destination branch that was the tip of the branch at the time the pull request was created.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>A non-zero, non-negative integer used to limit the number of returned results. The default is 100 comments. You can return up to 500 comments with a single request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCommitInput {
    /// <p>The commit ID. Commit IDs are the full SHA ID of the commit.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The name of the repository to which the commit was made.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a get commit operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCommitOutput {
    /// <p>A commit data type object that contains information about the specified commit.</p>
    #[serde(rename = "commit")]
    pub commit: Commit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDifferencesInput {
    /// <p>A non-zero, non-negative integer used to limit the number of returned results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit.</p>
    #[serde(rename = "afterCommitSpecifier")]
    pub after_commit_specifier: String,
    /// <p>The file path in which to check differences. Limits the results to this path. Can also be used to specify the changed name of a directory or folder, if it has changed. If not specified, differences are shown for all paths.</p>
    #[serde(rename = "afterPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_path: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, the full commit ID). Optional. If not specified, all changes before the <code>afterCommitSpecifier</code> value are shown. If you do not use <code>beforeCommitSpecifier</code> in your request, consider limiting the results with <code>maxResults</code>.</p>
    #[serde(rename = "beforeCommitSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_specifier: Option<String>,
    /// <p>The file path in which to check for differences. Limits the results to this path. Can also be used to specify the previous name of a directory or folder. If <code>beforePath</code> and <code>afterPath</code> are not specified, differences are shown for all paths.</p>
    #[serde(rename = "beforePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_path: Option<String>,
    /// <p>The name of the repository where you want to get differences.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDifferencesOutput {
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A data type object that contains information about the differences, including whether the difference is added, modified, or deleted (A, D, M).</p>
    #[serde(rename = "differences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differences: Option<Vec<Difference>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFileInput {
    /// <p>The fully quaified reference that identifies the commit that contains the file. For example, you can specify a full commit ID, a tag, a branch name, or a reference such as refs/heads/master. If none is provided, the head commit is used.</p>
    #[serde(rename = "commitSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_specifier: Option<String>,
    /// <p>The fully qualified path to the file, including the full name and extension of the file. For example, /examples/file.md is the fully qualified path to a file named file.md in a folder named examples.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The name of the repository that contains the file.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>The extrapolated file mode permissions of the blob. Valid values include strings such as EXECUTABLE and not numeric values.</p> <note> <p>The file mode permissions returned by this API are not the standard file mode permission values, such as 100644, but rather extrapolated values. See the supported return values.</p> </note></p>
    #[serde(rename = "fileMode")]
    pub file_mode: String,
    /// <p>The fully qualified path to the specified file. Returns the name and extension of the file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The size of the contents of the file, in bytes.</p>
    #[serde(rename = "fileSize")]
    pub file_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFolderInput {
    /// <p>A fully qualified reference used to identify a commit that contains the version of the folder's content to return. A fully qualified reference can be a commit ID, branch name, tag, or reference such as HEAD. If no specifier is provided, the folder content is returned as it exists in the HEAD commit.</p>
    #[serde(rename = "commitSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_specifier: Option<String>,
    /// <p>The fully qualified path to the folder whose contents are returned, including the folder name. For example, /examples is a fully-qualified path to a folder named examples that was created off of the root directory (/) of a repository. </p>
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    /// <p>The name of the repository.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFolderOutput {
    /// <p>The full commit ID used as a reference for the returned version of the folder content.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The list of files in the specified folder, if any.</p>
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    /// <p>The fully qualified path of the folder whose contents are returned.</p>
    #[serde(rename = "folderPath")]
    pub folder_path: String,
    /// <p>The list of folders that exist under the specified folder, if any.</p>
    #[serde(rename = "subFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_folders: Option<Vec<Folder>>,
    /// <p>The list of submodules in the specified folder, if any.</p>
    #[serde(rename = "subModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_modules: Option<Vec<SubModule>>,
    /// <p>The list of symbolic links to other files and folders in the specified folder, if any.</p>
    #[serde(rename = "symbolicLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbolic_links: Option<Vec<SymbolicLink>>,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains the folder.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMergeCommitInput {
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The name of the repository that contains the merge commit about which you want to get information.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMergeCommitOutput {
    /// <p>The commit ID of the merge base.</p>
    #[serde(rename = "baseCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    /// <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "destinationCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit_id: Option<String>,
    /// <p>The commit ID for the merge commit created when the source branch was merged into the destination branch. If the fast-forward merge strategy was used, there is no merge commit.</p>
    #[serde(rename = "mergedCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_commit_id: Option<String>,
    /// <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "sourceCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMergeConflictsInput {
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The maximum number of files to include in the output.</p>
    #[serde(rename = "maxConflictFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conflict_files: Option<i64>,
    /// <p>The merge option or strategy you want to use to merge the code. </p>
    #[serde(rename = "mergeOption")]
    pub merge_option: String,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the repository where the pull request was created.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMergeConflictsOutput {
    /// <p>The commit ID of the merge base.</p>
    #[serde(rename = "baseCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    /// <p>A list of metadata for any conflicting files. If the specified merge strategy is FAST_FORWARD_MERGE, this list is always empty.</p>
    #[serde(rename = "conflictMetadataList")]
    pub conflict_metadata_list: Vec<ConflictMetadata>,
    /// <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "destinationCommitId")]
    pub destination_commit_id: String,
    /// <p>A Boolean value that indicates whether the code is mergeable by the specified merge option.</p>
    #[serde(rename = "mergeable")]
    pub mergeable: bool,
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "sourceCommitId")]
    pub source_commit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMergeOptionsInput {
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The name of the repository that contains the commits about which you want to get merge options.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMergeOptionsOutput {
    /// <p>The commit ID of the merge base.</p>
    #[serde(rename = "baseCommitId")]
    pub base_commit_id: String,
    /// <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "destinationCommitId")]
    pub destination_commit_id: String,
    /// <p>The merge option or strategy used to merge the code.</p>
    #[serde(rename = "mergeOptions")]
    pub merge_options: Vec<String>,
    /// <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p>
    #[serde(rename = "sourceCommitId")]
    pub source_commit_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPullRequestApprovalStatesInput {
    /// <p>The system-generated ID for the pull request.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The system-generated ID for the pull request revision.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPullRequestApprovalStatesOutput {
    /// <p>Information about users who have approved the pull request.</p>
    #[serde(rename = "approvals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvals: Option<Vec<Approval>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPullRequestInput {
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPullRequestOutput {
    /// <p>Information about the specified pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPullRequestOverrideStateInput {
    /// <p>The ID of the pull request for which you want to get information about whether approval rules have been set aside (overridden).</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The system-generated ID of the revision for the pull request. To retrieve the most recent revision ID, use <a>GetPullRequest</a>.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPullRequestOverrideStateOutput {
    /// <p>A Boolean value that indicates whether a pull request has had its rules set aside (TRUE) or whether all approval rules still apply (FALSE).</p>
    #[serde(rename = "overridden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the user or identity that overrode the rules and their requirements for the pull request.</p>
    #[serde(rename = "overrider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrider: Option<String>,
}

/// <p>Represents the input of a get repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRepositoryInput {
    /// <p>The name of the repository to get information about.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a get repository operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRepositoryOutput {
    /// <p>Information about the repository.</p>
    #[serde(rename = "repositoryMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_metadata: Option<RepositoryMetadata>,
}

/// <p>Represents the input of a get repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRepositoryTriggersInput {
    /// <p>The name of the repository for which the trigger is configured.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

/// <p>Represents the output of a get repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Information about whether a file is binary or textual in a merge or pull request operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IsBinaryFile {
    /// <p>The binary or non-binary status of a file in the base of a merge or pull request.</p>
    #[serde(rename = "base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<bool>,
    /// <p>The binary or non-binary status of a file in the destination of a merge or pull request.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<bool>,
    /// <p>The binary or non-binary status of file in the source of a merge or pull request.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApprovalRuleTemplatesInput {
    /// <p>A non-zero, non-negative integer used to limit the number of returned results.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApprovalRuleTemplatesOutput {
    /// <p>The names of all the approval rule templates found in the AWS Region for your AWS account.</p>
    #[serde(rename = "approvalRuleTemplateNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_names: Option<Vec<String>>,
    /// <p>An enumeration token that allows the operation to batch the next results of the operation.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAssociatedApprovalRuleTemplatesForRepositoryInput {
    /// <p>A non-zero, non-negative integer used to limit the number of returned results.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the repository for which you want to list all associated approval rule templates.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssociatedApprovalRuleTemplatesForRepositoryOutput {
    /// <p>The names of all approval rule templates associated with the repository.</p>
    #[serde(rename = "approvalRuleTemplateNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_names: Option<Vec<String>>,
    /// <p>An enumeration token that allows the operation to batch the next results of the operation.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a list branches operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPullRequestsInput {
    /// <p>Optional. The Amazon Resource Name (ARN) of the user who created the pull request. If used, this filters the results to pull requests created by that user.</p>
    #[serde(rename = "authorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    /// <p>A non-zero, non-negative integer used to limit the number of returned results.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPullRequestsOutput {
    /// <p>An enumeration token that allows the operation to batch the next results of the operation.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The system-generated IDs of the pull requests.</p>
    #[serde(rename = "pullRequestIds")]
    pub pull_request_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRepositoriesForApprovalRuleTemplateInput {
    /// <p>The name of the approval rule template for which you want to list repositories that are associated with that template.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
    /// <p>A non-zero, non-negative integer used to limit the number of returned results.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRepositoriesForApprovalRuleTemplateOutput {
    /// <p>An enumeration token that allows the operation to batch the next results of the operation.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of repository names that are associated with the specified approval rule template.</p>
    #[serde(rename = "repositoryNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_names: Option<Vec<String>>,
}

/// <p>Represents the input of a list repositories operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to get information about tags, if any.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The position of a change in a compared file, in line number format.</p>
    #[serde(rename = "filePosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_position: Option<i64>,
    /// <p>In a comparison of commits or a pull request, whether the change is in the before or after of that comparison.</p>
    #[serde(rename = "relativeFileVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_file_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MergeBranchesByFastForwardInput {
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The name of the repository where you want to merge two branches.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
    /// <p>The branch where the merge is applied.</p>
    #[serde(rename = "targetBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergeBranchesByFastForwardOutput {
    /// <p>The commit ID of the merge in the destination or target branch.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>The tree ID of the merge in the destination or target branch.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MergeBranchesBySquashInput {
    /// <p>The name of the author who created the commit. This information is used as both the author and committer for the commit.</p>
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// <p>The commit message for the merge.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>If AUTOMERGE is the conflict resolution strategy, a list of inputs to use when resolving conflicts during a merge.</p>
    #[serde(rename = "conflictResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The email address of the person merging the branches. This information is used in the commit information for the merge.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If this is specified as true, a .gitkeep file is created for empty folders. The default is false.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    /// <p>The name of the repository where you want to merge two branches.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
    /// <p>The branch where the merge is applied. </p>
    #[serde(rename = "targetBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergeBranchesBySquashOutput {
    /// <p>The commit ID of the merge in the destination or target branch.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>The tree ID of the merge in the destination or target branch.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MergeBranchesByThreeWayInput {
    /// <p>The name of the author who created the commit. This information is used as both the author and committer for the commit.</p>
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// <p>The commit message to include in the commit information for the merge.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>If AUTOMERGE is the conflict resolution strategy, a list of inputs to use when resolving conflicts during a merge.</p>
    #[serde(rename = "conflictResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "destinationCommitSpecifier")]
    pub destination_commit_specifier: String,
    /// <p>The email address of the person merging the branches. This information is used in the commit information for the merge.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If true, a .gitkeep file is created for empty folders. The default is false.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
    /// <p>The name of the repository where you want to merge two branches.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    #[serde(rename = "sourceCommitSpecifier")]
    pub source_commit_specifier: String,
    /// <p>The branch where the merge is applied. </p>
    #[serde(rename = "targetBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergeBranchesByThreeWayOutput {
    /// <p>The commit ID of the merge in the destination or target branch.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>The tree ID of the merge in the destination or target branch.</p>
    #[serde(rename = "treeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
}

/// <p>Information about merge hunks in a merge or pull request operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergeHunk {
    /// <p>Information about the merge hunk in the base of a merge or pull request.</p>
    #[serde(rename = "base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<MergeHunkDetail>,
    /// <p>Information about the merge hunk in the destination of a merge or pull request.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<MergeHunkDetail>,
    /// <p>A Boolean value indicating whether a combination of hunks contains a conflict. Conflicts occur when the same file or the same lines in a file were modified in both the source and destination of a merge or pull request. Valid values include true, false, and null. True when the hunk represents a conflict and one or more files contains a line conflict. File mode conflicts in a merge do not set this to true.</p>
    #[serde(rename = "isConflict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_conflict: Option<bool>,
    /// <p>Information about the merge hunk in the source of a merge or pull request.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MergeHunkDetail>,
}

/// <p>Information about the details of a merge hunk that contains a conflict in a merge or pull request operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergeHunkDetail {
    /// <p>The end position of the hunk in the merge result.</p>
    #[serde(rename = "endLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    /// <p>The base-64 encoded content of the hunk merged region that might contain a conflict.</p>
    #[serde(rename = "hunkContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hunk_content: Option<String>,
    /// <p>The start position of the hunk in the merge result.</p>
    #[serde(rename = "startLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
}

/// <p>Returns information about a merge or potential merge between a source reference and a destination reference in a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergeMetadata {
    /// <p>A Boolean value indicating whether the merge has been made.</p>
    #[serde(rename = "isMerged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_merged: Option<bool>,
    /// <p>The commit ID for the merge commit, if any.</p>
    #[serde(rename = "mergeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_id: Option<String>,
    /// <p>The merge strategy used in the merge.</p>
    #[serde(rename = "mergeOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_option: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user who merged the branches.</p>
    #[serde(rename = "mergedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<String>,
}

/// <p>Information about the file operation conflicts in a merge operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergeOperations {
    /// <p>The operation on a file in the destination of a merge or pull request.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The operation (add, modify, or delete) on a file in the source of a merge or pull request.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergePullRequestByFastForwardOutput {
    /// <p>Information about the specified pull request, including the merge.</p>
    #[serde(rename = "pullRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MergePullRequestBySquashInput {
    /// <p>The name of the author who created the commit. This information is used as both the author and committer for the commit.</p>
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// <p>The commit message to include in the commit information for the merge.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>If AUTOMERGE is the conflict resolution strategy, a list of inputs to use when resolving conflicts during a merge.</p>
    #[serde(rename = "conflictResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The email address of the person merging the branches. This information is used in the commit information for the merge.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If true, a .gitkeep file is created for empty folders. The default is false.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergePullRequestBySquashOutput {
    #[serde(rename = "pullRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MergePullRequestByThreeWayInput {
    /// <p>The name of the author who created the commit. This information is used as both the author and committer for the commit.</p>
    #[serde(rename = "authorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// <p>The commit message to include in the commit information for the merge.</p>
    #[serde(rename = "commitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    #[serde(rename = "conflictDetailLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detail_level: Option<String>,
    /// <p>If AUTOMERGE is the conflict resolution strategy, a list of inputs to use when resolving conflicts during a merge.</p>
    #[serde(rename = "conflictResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<ConflictResolution>,
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    #[serde(rename = "conflictResolutionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_strategy: Option<String>,
    /// <p>The email address of the person merging the branches. This information is used in the commit information for the merge.</p>
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If true, a .gitkeep file is created for empty folders. The default is false.</p>
    #[serde(rename = "keepEmptyFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_folders: Option<bool>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MergePullRequestByThreeWayOutput {
    #[serde(rename = "pullRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

/// <p>Information about the type of an object in a merge operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ObjectTypes {
    /// <p>The type of the object in the base commit of the merge.</p>
    #[serde(rename = "base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// <p>The type of the object in the destination branch.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The type of the object in the source branch.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// <p>Returns information about the template that created the approval rule for a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OriginApprovalRuleTemplate {
    /// <p>The ID of the template that created the approval rule.</p>
    #[serde(rename = "approvalRuleTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_id: Option<String>,
    /// <p>The name of the template that created the approval rule.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_template_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OverridePullRequestApprovalRulesInput {
    /// <p>Whether you want to set aside approval rule requirements for the pull request (OVERRIDE) or revoke a previous override and apply approval rule requirements (REVOKE). REVOKE status is not stored.</p>
    #[serde(rename = "overrideStatus")]
    pub override_status: String,
    /// <p>The system-generated ID of the pull request for which you want to override all approval rule requirements. To get this information, use <a>GetPullRequest</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The system-generated ID of the most recent revision of the pull request. You cannot override approval rules for anything but the most recent revision of a pull request. To get the revision ID, use GetPullRequest.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostCommentForComparedCommitInput {
    /// <p>To establish the directionality of the comparison, the full commit ID of the after commit.</p>
    #[serde(rename = "afterCommitId")]
    pub after_commit_id: String,
    /// <p>To establish the directionality of the comparison, the full commit ID of the before commit. Required for commenting on any commit unless that commit is the initial commit.</p>
    #[serde(rename = "beforeCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_commit_id: Option<String>,
    /// <p>A unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PostCommentForComparedCommitOutput {
    /// <p>In the directionality you established, the blob ID of the after blob.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>In the directionality you established, the full commit ID of the after commit.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>In the directionality you established, the blob ID of the before blob.</p>
    #[serde(rename = "beforeBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_blob_id: Option<String>,
    /// <p>In the directionality you established, the full commit ID of the before commit.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostCommentForPullRequestInput {
    /// <p>The full commit ID of the commit in the source branch that is the current tip of the branch for the pull request when you post the comment.</p>
    #[serde(rename = "afterCommitId")]
    pub after_commit_id: String,
    /// <p>The full commit ID of the commit in the destination branch that was the tip of the branch at the time the pull request was created.</p>
    #[serde(rename = "beforeCommitId")]
    pub before_commit_id: String,
    /// <p>A unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The content of your comment on the change.</p>
    #[serde(rename = "content")]
    pub content: String,
    /// <p>The location of the change where you want to post your comment. If no location is provided, the comment is posted as a general comment on the pull request difference between the before commit ID and the after commit ID.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PostCommentForPullRequestOutput {
    /// <p>In the directionality of the pull request, the blob ID of the after blob.</p>
    #[serde(rename = "afterBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_blob_id: Option<String>,
    /// <p>The full commit ID of the commit in the destination branch where the pull request is merged.</p>
    #[serde(rename = "afterCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_commit_id: Option<String>,
    /// <p>In the directionality of the pull request, the blob ID of the before blob.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostCommentReplyInput {
    /// <p>A unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PostCommentReplyOutput {
    /// <p>Information about the reply to a comment.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>Returns information about a pull request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PullRequest {
    /// <p>The approval rules applied to the pull request.</p>
    #[serde(rename = "approvalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<Vec<ApprovalRule>>,
    /// <p>The Amazon Resource Name (ARN) of the user who created the pull request.</p>
    #[serde(rename = "authorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_arn: Option<String>,
    /// <p>A unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
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
    /// <p>The system-generated revision ID for the pull request.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The user-defined title of the pull request. This title is displayed in the list of pull requests to other repository users.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Metadata about the pull request that is used when comparing the pull request source with its destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PullRequestEvent {
    /// <p>The Amazon Resource Name (ARN) of the user whose actions resulted in the event. Examples include updating the pull request with more commits or changing the status of a pull request.</p>
    #[serde(rename = "actorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_arn: Option<String>,
    /// <p>Information about a pull request event.</p>
    #[serde(rename = "approvalRuleEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_event_metadata: Option<ApprovalRuleEventMetadata>,
    /// <p>Information about an approval rule override event for a pull request.</p>
    #[serde(rename = "approvalRuleOverriddenEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rule_overridden_event_metadata: Option<ApprovalRuleOverriddenEventMetadata>,
    /// <p>Information about an approval state change for a pull request.</p>
    #[serde(rename = "approvalStateChangedEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_state_changed_event_metadata: Option<ApprovalStateChangedEventMetadata>,
    /// <p>The day and time of the pull request event, in timestamp format.</p>
    #[serde(rename = "eventDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<f64>,
    /// <p>Information about the source and destination branches for the pull request.</p>
    #[serde(rename = "pullRequestCreatedEventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_created_event_metadata: Option<PullRequestCreatedEventMetadata>,
    /// <p>The type of the pull request event (for example, a status change event (PULL_REQUEST_STATUS_CHANGED) or update event (PULL_REQUEST_SOURCE_REFERENCE_UPDATED)).</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PullRequestMergedStateChangedEventMetadata {
    /// <p>The name of the branch that the pull request is merged into.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PullRequestStatusChangedEventMetadata {
    /// <p>The changed status of the pull request.</p>
    #[serde(rename = "pullRequestStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_status: Option<String>,
}

/// <p>Returns information about a pull request target.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PullRequestTarget {
    /// <p>The full commit ID that is the tip of the destination branch. This is the commit where the pull request was or will be merged.</p>
    #[serde(rename = "destinationCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit: Option<String>,
    /// <p>The branch of the repository where the pull request changes are merged. Also known as the destination branch. </p>
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
    /// <p>The full commit ID of the tip of the source branch used to create the pull request. If the pull request branch is updated by a push while the pull request is open, the commit ID changes to reflect the new tip of the branch.</p>
    #[serde(rename = "sourceCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
    /// <p>The branch of the repository that contains the changes for the pull request. Also known as the source branch.</p>
    #[serde(rename = "sourceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<String>,
}

/// <p>Information about a file added or updated as part of a commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutFileInput {
    /// <p>The name of the branch where you want to add or update the file. If this is an empty repository, this branch is created.</p>
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// <p>A message about why this file was added or updated. Although it is optional, a message makes the commit history for your repository more useful.</p>
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
    /// <p>The file mode permissions of the blob. Valid file mode permissions are listed here.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p><p>The name of the file you want to add or update, including the relative path to the file in the repository.</p> <note> <p>If the path does not currently exist in the repository, the path is created as part of adding the file.</p> </note></p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The name of the person adding or updating the file. Although it is optional, a name makes the commit history for your repository more useful.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The full commit ID of the head commit in the branch where you want to add or update the file. If this is an empty repository, no commit ID is required. If this is not an empty repository, a commit ID is required. </p> <p>The commit ID must match the ID of the head commit at the time of the operation. Otherwise, an error occurs, and the file is not added or updated.</p>
    #[serde(rename = "parentCommitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_commit_id: Option<String>,
    /// <p>The name of the repository where you want to add or update the file.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutFileOutput {
    /// <p>The ID of the blob, which is its SHA-1 pointer.</p>
    #[serde(rename = "blobId")]
    pub blob_id: String,
    /// <p>The full SHA ID of the commit that contains this file change.</p>
    #[serde(rename = "commitId")]
    pub commit_id: String,
    /// <p>The full SHA-1 pointer of the tree information for the commit that contains this file change.</p>
    #[serde(rename = "treeId")]
    pub tree_id: String,
}

/// <p>Represents the input of a put repository triggers operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRepositoryTriggersOutput {
    /// <p>The system-generated unique ID for the create or update operation.</p>
    #[serde(rename = "configurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

/// <p>Information about a replacement content entry in the conflict of a merge or pull request operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReplaceContentEntry {
    /// <p>The base-64 encoded content to use when the replacement type is USE_NEW_CONTENT.</p>
    #[serde(rename = "content")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<bytes::Bytes>,
    /// <p>The file mode to apply during conflict resoltion.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p>The path of the conflicting file.</p>
    #[serde(rename = "filePath")]
    pub file_path: String,
    /// <p>The replacement type to use when determining how to resolve the conflict.</p>
    #[serde(rename = "replacementType")]
    pub replacement_type: String,
}

/// <p>Information about a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>The branches to be included in the trigger configuration. If you specify an empty array, the trigger applies to all branches.</p> <note> <p>Although no content is required in the array, you must include the array itself.</p> </note></p>
    #[serde(rename = "branches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    /// <p>Any custom data associated with the trigger to be included in the information sent to the target of the trigger.</p>
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    /// <p>The ARN of the resource that is the target for a trigger (for example, the ARN of a topic in Amazon SNS).</p>
    #[serde(rename = "destinationArn")]
    pub destination_arn: String,
    /// <p><p>The repository events that cause the trigger to run actions in another service, such as sending a notification through Amazon SNS. </p> <note> <p>The valid value &quot;all&quot; cannot be used with any other values.</p> </note></p>
    #[serde(rename = "events")]
    pub events: Vec<String>,
    /// <p>The name of the trigger.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>A trigger failed to run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RepositoryTriggerExecutionFailure {
    /// <p>Message information about the trigger that did not run.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SymbolicLink {
    /// <p>The fully qualified path to the folder that contains the symbolic link.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Target {
    /// <p>The branch of the repository where the pull request changes are merged. Also known as the destination branch.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestRepositoryTriggersOutput {
    /// <p>The list of triggers that were not tested. This list provides the names of the triggers that could not be tested, separated by commas.</p>
    #[serde(rename = "failedExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_executions: Option<Vec<RepositoryTriggerExecutionFailure>>,
    /// <p>The list of triggers that were successfully tested. This list provides the names of the triggers that were successfully tested, separated by commas.</p>
    #[serde(rename = "successfulExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_executions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource to which you want to remove tags.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag key for each tag that you want to remove from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApprovalRuleTemplateContentInput {
    /// <p>The name of the approval rule template where you want to update the content of the rule. </p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
    /// <p>The SHA-256 hash signature for the content of the approval rule. You can retrieve this information by using <a>GetPullRequest</a>.</p>
    #[serde(rename = "existingRuleContentSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_rule_content_sha_256: Option<String>,
    /// <p>The content that replaces the existing content of the rule. Content statements must be complete. You cannot provide only the changes.</p>
    #[serde(rename = "newRuleContent")]
    pub new_rule_content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApprovalRuleTemplateContentOutput {
    #[serde(rename = "approvalRuleTemplate")]
    pub approval_rule_template: ApprovalRuleTemplate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApprovalRuleTemplateDescriptionInput {
    /// <p>The updated description of the approval rule template.</p>
    #[serde(rename = "approvalRuleTemplateDescription")]
    pub approval_rule_template_description: String,
    /// <p>The name of the template for which you want to update the description.</p>
    #[serde(rename = "approvalRuleTemplateName")]
    pub approval_rule_template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApprovalRuleTemplateDescriptionOutput {
    /// <p>The structure and content of the updated approval rule template.</p>
    #[serde(rename = "approvalRuleTemplate")]
    pub approval_rule_template: ApprovalRuleTemplate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApprovalRuleTemplateNameInput {
    /// <p>The new name you want to apply to the approval rule template.</p>
    #[serde(rename = "newApprovalRuleTemplateName")]
    pub new_approval_rule_template_name: String,
    /// <p>The current name of the approval rule template.</p>
    #[serde(rename = "oldApprovalRuleTemplateName")]
    pub old_approval_rule_template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApprovalRuleTemplateNameOutput {
    /// <p>The structure and content of the updated approval rule template.</p>
    #[serde(rename = "approvalRuleTemplate")]
    pub approval_rule_template: ApprovalRuleTemplate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCommentInput {
    /// <p>The system-generated ID of the comment you want to update. To get this ID, use <a>GetCommentsForComparedCommit</a> or <a>GetCommentsForPullRequest</a>.</p>
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// <p>The updated content to replace the existing content of the comment.</p>
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCommentOutput {
    /// <p>Information about the updated comment.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// <p>Represents the input of an update default branch operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDefaultBranchInput {
    /// <p>The name of the branch to set as the default.</p>
    #[serde(rename = "defaultBranchName")]
    pub default_branch_name: String,
    /// <p>The name of the repository to set or change the default branch for.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePullRequestApprovalRuleContentInput {
    /// <p>The name of the approval rule you want to update.</p>
    #[serde(rename = "approvalRuleName")]
    pub approval_rule_name: String,
    /// <p>The SHA-256 hash signature for the content of the approval rule. You can retrieve this information by using <a>GetPullRequest</a>.</p>
    #[serde(rename = "existingRuleContentSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_rule_content_sha_256: Option<String>,
    /// <p><p>The updated content for the approval rule.</p> <note> <p>When you update the content of the approval rule, you can specify approvers in an approval pool in one of two ways:</p> <ul> <li> <p> <b>CodeCommitApprovers</b>: This option only requires an AWS account and a resource. It can be used for both IAM users and federated access users whose name matches the provided resource name. This is a very powerful option that offers a great deal of flexibility. For example, if you specify the AWS account <i>123456789012</i> and <i>Mary<em>Major</i>, all of the following are counted as approvals coming from that user:</p> <ul> <li> <p>An IAM user in the account (arn:aws:iam::<i>123456789012</i>:user/<i>Mary</em>Major</i>)</p> </li> <li> <p>A federated user identified in IAM as Mary<em>Major (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary</em>Major</i>)</p> </li> </ul> <p>This option does not recognize an active session of someone assuming the role of CodeCommitReview with a role session name of <i>Mary<em>Major</i> (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary</em>Major</i>) unless you include a wildcard (*Mary<em>Major).</p> </li> <li> <p> <b>Fully qualified ARN</b>: This option allows you to specify the fully qualified Amazon Resource Name (ARN) of the IAM user or role. </p> </li> </ul> <p>For more information about IAM ARNs, wildcards, and formats, see &lt;a href=&quot;https://docs.aws.amazon.com/iam/latest/UserGuide/reference</em>identifiers.html&quot;&gt;IAM Identifiers</a> in the <i>IAM User Guide</i>.</p> </note></p>
    #[serde(rename = "newRuleContent")]
    pub new_rule_content: String,
    /// <p>The system-generated ID of the pull request.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePullRequestApprovalRuleContentOutput {
    /// <p>Information about the updated approval rule.</p>
    #[serde(rename = "approvalRule")]
    pub approval_rule: ApprovalRule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePullRequestApprovalStateInput {
    /// <p>The approval state to associate with the user on the pull request.</p>
    #[serde(rename = "approvalState")]
    pub approval_state: String,
    /// <p>The system-generated ID of the pull request.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The system-generated ID of the revision.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePullRequestDescriptionInput {
    /// <p>The updated content of the description for the pull request. This content replaces the existing description.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePullRequestDescriptionOutput {
    /// <p>Information about the updated pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePullRequestStatusInput {
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The status of the pull request. The only valid operations are to update the status from <code>OPEN</code> to <code>OPEN</code>, <code>OPEN</code> to <code>CLOSED</code> or from <code>CLOSED</code> to <code>CLOSED</code>.</p>
    #[serde(rename = "pullRequestStatus")]
    pub pull_request_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePullRequestStatusOutput {
    /// <p>Information about the pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePullRequestTitleInput {
    /// <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p>
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: String,
    /// <p>The updated title of the pull request. This replaces the existing title.</p>
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePullRequestTitleOutput {
    /// <p>Information about the updated pull request.</p>
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest,
}

/// <p>Represents the input of an update repository description operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRepositoryNameInput {
    /// <p>The new name for the repository.</p>
    #[serde(rename = "newName")]
    pub new_name: String,
    /// <p>The current name of the repository.</p>
    #[serde(rename = "oldName")]
    pub old_name: String,
}

/// <p>Information about the user who made a specified commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// Errors returned by AssociateApprovalRuleTemplateWithRepository
#[derive(Debug, PartialEq)]
pub enum AssociateApprovalRuleTemplateWithRepositoryError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
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
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The maximum number of approval rule templates for a repository has been exceeded. You cannot associate more than 25 approval rule templates with a repository.</p>
    MaximumRuleTemplatesAssociatedWithRepository(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
}

impl AssociateApprovalRuleTemplateWithRepositoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateApprovalRuleTemplateWithRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "ApprovalRuleTemplateDoesNotExistException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::ApprovalRuleTemplateDoesNotExist(err.msg)),
"ApprovalRuleTemplateNameRequiredException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::ApprovalRuleTemplateNameRequired(err.msg)),
"EncryptionIntegrityChecksFailedException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::EncryptionIntegrityChecksFailed(err.msg)),
"EncryptionKeyAccessDeniedException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyAccessDenied(err.msg)),
"EncryptionKeyDisabledException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyDisabled(err.msg)),
"EncryptionKeyNotFoundException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyNotFound(err.msg)),
"EncryptionKeyUnavailableException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyUnavailable(err.msg)),
"InvalidApprovalRuleTemplateNameException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::InvalidApprovalRuleTemplateName(err.msg)),
"InvalidRepositoryNameException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::InvalidRepositoryName(err.msg)),
"MaximumRuleTemplatesAssociatedWithRepositoryException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::MaximumRuleTemplatesAssociatedWithRepository(err.msg)),
"RepositoryDoesNotExistException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::RepositoryDoesNotExist(err.msg)),
"RepositoryNameRequiredException" => return RusotoError::Service(AssociateApprovalRuleTemplateWithRepositoryError::RepositoryNameRequired(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateApprovalRuleTemplateWithRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            AssociateApprovalRuleTemplateWithRepositoryError::ApprovalRuleTemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::ApprovalRuleTemplateNameRequired(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::InvalidApprovalRuleTemplateName(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::MaximumRuleTemplatesAssociatedWithRepository(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
AssociateApprovalRuleTemplateWithRepositoryError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for AssociateApprovalRuleTemplateWithRepositoryError {}
/// Errors returned by BatchAssociateApprovalRuleTemplateWithRepositories
#[derive(Debug, PartialEq)]
pub enum BatchAssociateApprovalRuleTemplateWithRepositoriesError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
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
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
    /// <p>The maximum number of allowed repository names was exceeded. Currently, this number is 100.</p>
    MaximumRepositoryNamesExceeded(String),
    /// <p>At least one repository name object is required, but was not specified.</p>
    RepositoryNamesRequired(String),
}

impl BatchAssociateApprovalRuleTemplateWithRepositoriesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchAssociateApprovalRuleTemplateWithRepositoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "ApprovalRuleTemplateDoesNotExistException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::ApprovalRuleTemplateDoesNotExist(err.msg)),
"ApprovalRuleTemplateNameRequiredException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::ApprovalRuleTemplateNameRequired(err.msg)),
"EncryptionIntegrityChecksFailedException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionIntegrityChecksFailed(err.msg)),
"EncryptionKeyAccessDeniedException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyAccessDenied(err.msg)),
"EncryptionKeyDisabledException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyDisabled(err.msg)),
"EncryptionKeyNotFoundException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyNotFound(err.msg)),
"EncryptionKeyUnavailableException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyUnavailable(err.msg)),
"InvalidApprovalRuleTemplateNameException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::InvalidApprovalRuleTemplateName(err.msg)),
"MaximumRepositoryNamesExceededException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::MaximumRepositoryNamesExceeded(err.msg)),
"RepositoryNamesRequiredException" => return RusotoError::Service(BatchAssociateApprovalRuleTemplateWithRepositoriesError::RepositoryNamesRequired(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchAssociateApprovalRuleTemplateWithRepositoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            BatchAssociateApprovalRuleTemplateWithRepositoriesError::ApprovalRuleTemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::ApprovalRuleTemplateNameRequired(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::InvalidApprovalRuleTemplateName(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::MaximumRepositoryNamesExceeded(ref cause) => write!(f, "{}", cause),
BatchAssociateApprovalRuleTemplateWithRepositoriesError::RepositoryNamesRequired(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for BatchAssociateApprovalRuleTemplateWithRepositoriesError {}
/// Errors returned by BatchDescribeMergeConflicts
#[derive(Debug, PartialEq)]
pub enum BatchDescribeMergeConflictsError {
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
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified value for the number of conflict files to return is not valid.</p>
    InvalidMaxConflictFiles(String),
    /// <p>The specified value for the number of merge hunks to return is not valid.</p>
    InvalidMaxMergeHunks(String),
    /// <p>The specified merge option is not valid for this operation. Not all merge strategies are supported for all operations.</p>
    InvalidMergeOption(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>A merge option or stategy is required, and none was provided.</p>
    MergeOptionRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl BatchDescribeMergeConflictsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDescribeMergeConflictsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::CommitDoesNotExist(err.msg),
                    )
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(BatchDescribeMergeConflictsError::CommitRequired(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(BatchDescribeMergeConflictsError::InvalidCommit(
                        err.msg,
                    ))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::InvalidConflictResolutionStrategy(
                            err.msg,
                        ),
                    )
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::InvalidContinuationToken(err.msg),
                    )
                }
                "InvalidMaxConflictFilesException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::InvalidMaxConflictFiles(err.msg),
                    )
                }
                "InvalidMaxMergeHunksException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::InvalidMaxMergeHunks(err.msg),
                    )
                }
                "InvalidMergeOptionException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::InvalidMergeOption(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::InvalidRepositoryName(err.msg),
                    )
                }
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "MergeOptionRequiredException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::MergeOptionRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::RepositoryNameRequired(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        BatchDescribeMergeConflictsError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDescribeMergeConflictsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDescribeMergeConflictsError::CommitDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::CommitRequired(ref cause) => write!(f, "{}", cause),
            BatchDescribeMergeConflictsError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            BatchDescribeMergeConflictsError::InvalidConflictDetailLevel(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::InvalidContinuationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::InvalidMaxConflictFiles(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::InvalidMaxMergeHunks(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::InvalidMergeOption(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::MergeOptionRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDescribeMergeConflictsError::TipsDivergenceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchDescribeMergeConflictsError {}
/// Errors returned by BatchDisassociateApprovalRuleTemplateFromRepositories
#[derive(Debug, PartialEq)]
pub enum BatchDisassociateApprovalRuleTemplateFromRepositoriesError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
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
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
    /// <p>The maximum number of allowed repository names was exceeded. Currently, this number is 100.</p>
    MaximumRepositoryNamesExceeded(String),
    /// <p>At least one repository name object is required, but was not specified.</p>
    RepositoryNamesRequired(String),
}

impl BatchDisassociateApprovalRuleTemplateFromRepositoriesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDisassociateApprovalRuleTemplateFromRepositoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "ApprovalRuleTemplateDoesNotExistException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::ApprovalRuleTemplateDoesNotExist(err.msg)),
"ApprovalRuleTemplateNameRequiredException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::ApprovalRuleTemplateNameRequired(err.msg)),
"EncryptionIntegrityChecksFailedException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionIntegrityChecksFailed(err.msg)),
"EncryptionKeyAccessDeniedException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyAccessDenied(err.msg)),
"EncryptionKeyDisabledException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyDisabled(err.msg)),
"EncryptionKeyNotFoundException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyNotFound(err.msg)),
"EncryptionKeyUnavailableException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyUnavailable(err.msg)),
"InvalidApprovalRuleTemplateNameException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::InvalidApprovalRuleTemplateName(err.msg)),
"MaximumRepositoryNamesExceededException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::MaximumRepositoryNamesExceeded(err.msg)),
"RepositoryNamesRequiredException" => return RusotoError::Service(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::RepositoryNamesRequired(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDisassociateApprovalRuleTemplateFromRepositoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            BatchDisassociateApprovalRuleTemplateFromRepositoriesError::ApprovalRuleTemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::ApprovalRuleTemplateNameRequired(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::InvalidApprovalRuleTemplateName(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::MaximumRepositoryNamesExceeded(ref cause) => write!(f, "{}", cause),
BatchDisassociateApprovalRuleTemplateFromRepositoriesError::RepositoryNamesRequired(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for BatchDisassociateApprovalRuleTemplateFromRepositoriesError {}
/// Errors returned by BatchGetCommits
#[derive(Debug, PartialEq)]
pub enum BatchGetCommitsError {
    /// <p>The maximum number of allowed commit IDs in a batch request is 100. Verify that your batch requests contains no more than 100 commit IDs, and then try again.</p>
    CommitIdsLimitExceeded(String),
    /// <p>A list of commit IDs is required, but was either not specified or the list was empty.</p>
    CommitIdsListRequired(String),
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
}

impl BatchGetCommitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetCommitsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitIdsLimitExceededException" => {
                    return RusotoError::Service(BatchGetCommitsError::CommitIdsLimitExceeded(
                        err.msg,
                    ))
                }
                "CommitIdsListRequiredException" => {
                    return RusotoError::Service(BatchGetCommitsError::CommitIdsListRequired(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        BatchGetCommitsError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(BatchGetCommitsError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(BatchGetCommitsError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(BatchGetCommitsError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(BatchGetCommitsError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(BatchGetCommitsError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(BatchGetCommitsError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(BatchGetCommitsError::RepositoryNameRequired(
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
impl fmt::Display for BatchGetCommitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetCommitsError::CommitIdsLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::CommitIdsListRequired(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetCommitsError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            BatchGetCommitsError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetCommitsError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The maximum number of allowed repository names was exceeded. Currently, this number is 100.</p>
    MaximumRepositoryNamesExceeded(String),
    /// <p>At least one repository name object is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetRepositoriesError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetRepositoriesError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetRepositoriesError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            BatchGetRepositoriesError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            BatchGetRepositoriesError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetRepositoriesError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            BatchGetRepositoriesError::MaximumRepositoryNamesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetRepositoriesError::RepositoryNamesRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetRepositoriesError {}
/// Errors returned by CreateApprovalRuleTemplate
#[derive(Debug, PartialEq)]
pub enum CreateApprovalRuleTemplateError {
    /// <p>The content for the approval rule template is empty. You must provide some content for an approval rule template. The content cannot be null.</p>
    ApprovalRuleTemplateContentRequired(String),
    /// <p>You cannot create an approval rule template with that name because a template with that name already exists in this AWS Region for your AWS account. Approval rule template names must be unique.</p>
    ApprovalRuleTemplateNameAlreadyExists(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
    /// <p>The content of the approval rule template is not valid.</p>
    InvalidApprovalRuleTemplateContent(String),
    /// <p>The description for the approval rule template is not valid because it exceeds the maximum characters allowed for a description. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateDescription(String),
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
    /// <p>The maximum number of approval rule templates has been exceeded for this AWS Region. </p>
    NumberOfRuleTemplatesExceeded(String),
}

impl CreateApprovalRuleTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateApprovalRuleTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleTemplateContentRequiredException" => {
                    return RusotoError::Service(
                        CreateApprovalRuleTemplateError::ApprovalRuleTemplateContentRequired(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleTemplateNameAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateApprovalRuleTemplateError::ApprovalRuleTemplateNameAlreadyExists(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleTemplateNameRequiredException" => {
                    return RusotoError::Service(
                        CreateApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(err.msg),
                    )
                }
                "InvalidApprovalRuleTemplateContentException" => {
                    return RusotoError::Service(
                        CreateApprovalRuleTemplateError::InvalidApprovalRuleTemplateContent(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleTemplateDescriptionException" => {
                    return RusotoError::Service(
                        CreateApprovalRuleTemplateError::InvalidApprovalRuleTemplateDescription(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleTemplateNameException" => {
                    return RusotoError::Service(
                        CreateApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(err.msg),
                    )
                }
                "NumberOfRuleTemplatesExceededException" => {
                    return RusotoError::Service(
                        CreateApprovalRuleTemplateError::NumberOfRuleTemplatesExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateApprovalRuleTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApprovalRuleTemplateError::ApprovalRuleTemplateContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApprovalRuleTemplateError::ApprovalRuleTemplateNameAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApprovalRuleTemplateError::InvalidApprovalRuleTemplateContent(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApprovalRuleTemplateError::InvalidApprovalRuleTemplateDescription(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApprovalRuleTemplateError::NumberOfRuleTemplatesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateApprovalRuleTemplateError {}
/// Errors returned by CreateBranch
#[derive(Debug, PartialEq)]
pub enum CreateBranchError {
    /// <p>The specified branch name already exists.</p>
    BranchNameExists(String),
    /// <p>A branch name is required, but was not specified.</p>
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBranchError::BranchNameExists(ref cause) => write!(f, "{}", cause),
            CreateBranchError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            CreateBranchError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateBranchError::CommitIdRequired(ref cause) => write!(f, "{}", cause),
            CreateBranchError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            CreateBranchError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            CreateBranchError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            CreateBranchError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            CreateBranchError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            CreateBranchError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            CreateBranchError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            CreateBranchError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            CreateBranchError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateBranchError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBranchError {}
/// Errors returned by CreateCommit
#[derive(Debug, PartialEq)]
pub enum CreateCommitError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Enter the name of a branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required, but was not specified.</p>
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
    /// <p>The commit cannot be created because both a source file and file content have been specified for the same file. You cannot provide both. Either specify a source file or provide the file content directly.</p>
    FileContentAndSourceFileSpecified(String),
    /// <p>The file cannot be added because it is too large. The maximum file size is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>The specified file does not exist. Verify that you have used the correct file name, full path, and extension.</p>
    FileDoesNotExist(String),
    /// <p>The commit cannot be created because no files have been specified as added, updated, or changed (PutFile or DeleteFile) for the commit.</p>
    FileEntryRequired(String),
    /// <p>The commit cannot be created because no file mode has been specified. A file mode is required to update mode permissions for a file.</p>
    FileModeRequired(String),
    /// <p>A file cannot be added to the repository because the specified file name has the same name as a directory in this repository. Either provide another name for the file, or add the file in a directory that does not match the file name.</p>
    FileNameConflictsWithDirectoryName(String),
    /// <p>The commit cannot be created because a specified file path points to a submodule. Verify that the destination files have valid file paths that do not point to a submodule.</p>
    FilePathConflictsWithSubmodulePath(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit results in a folder whose contents exceed the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The number of specified files to change as part of this commit exceeds the maximum number of files that can be changed in a single commit. Consider using a Git client for these changes.</p>
    MaximumFileEntriesExceeded(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
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
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCommitError::BranchDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateCommitError::BranchNameIsTagName(ref cause) => write!(f, "{}", cause),
            CreateCommitError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            CreateCommitError::CommitMessageLengthExceeded(ref cause) => write!(f, "{}", cause),
            CreateCommitError::DirectoryNameConflictsWithFileName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCommitError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            CreateCommitError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            CreateCommitError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            CreateCommitError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            CreateCommitError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            CreateCommitError::FileContentAndSourceFileSpecified(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCommitError::FileContentSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateCommitError::FileDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateCommitError::FileEntryRequired(ref cause) => write!(f, "{}", cause),
            CreateCommitError::FileModeRequired(ref cause) => write!(f, "{}", cause),
            CreateCommitError::FileNameConflictsWithDirectoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCommitError::FilePathConflictsWithSubmodulePath(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCommitError::FolderContentSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateCommitError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            CreateCommitError::InvalidDeletionParameter(ref cause) => write!(f, "{}", cause),
            CreateCommitError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            CreateCommitError::InvalidFileMode(ref cause) => write!(f, "{}", cause),
            CreateCommitError::InvalidParentCommitId(ref cause) => write!(f, "{}", cause),
            CreateCommitError::InvalidPath(ref cause) => write!(f, "{}", cause),
            CreateCommitError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            CreateCommitError::MaximumFileEntriesExceeded(ref cause) => write!(f, "{}", cause),
            CreateCommitError::NameLengthExceeded(ref cause) => write!(f, "{}", cause),
            CreateCommitError::NoChange(ref cause) => write!(f, "{}", cause),
            CreateCommitError::ParentCommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateCommitError::ParentCommitIdOutdated(ref cause) => write!(f, "{}", cause),
            CreateCommitError::ParentCommitIdRequired(ref cause) => write!(f, "{}", cause),
            CreateCommitError::PathRequired(ref cause) => write!(f, "{}", cause),
            CreateCommitError::PutFileEntryConflict(ref cause) => write!(f, "{}", cause),
            CreateCommitError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateCommitError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            CreateCommitError::RestrictedSourceFile(ref cause) => write!(f, "{}", cause),
            CreateCommitError::SamePathRequest(ref cause) => write!(f, "{}", cause),
            CreateCommitError::SourceFileOrContentRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCommitError {}
/// Errors returned by CreatePullRequest
#[derive(Debug, PartialEq)]
pub enum CreatePullRequestError {
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
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
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be reused.</p>
    IdempotencyParameterMismatch(String),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestToken(String),
    /// <p>The pull request description is not valid. Descriptions cannot be more than 1,000 characters.</p>
    InvalidDescription(String),
    /// <p>The specified reference name format is not valid. Reference names must conform to the Git references format (for example, refs/heads/master). For more information, see <a href="https://git-scm.com/book/en/v2/Git-Internals-Git-References">Git Internals - Git References</a> or consult your Git documentation.</p>
    InvalidReferenceName(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
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
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The source branch and destination branch for the pull request are the same. You must specify different branches for the source and destination.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePullRequestError::ClientRequestTokenRequired(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::IdempotencyParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestError::InvalidClientRequestToken(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::InvalidDescription(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::InvalidReferenceName(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::InvalidTarget(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::InvalidTargets(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::InvalidTitle(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::MaximumOpenPullRequestsExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestError::MultipleRepositoriesInPullRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestError::ReferenceDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::ReferenceNameRequired(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::ReferenceTypeNotSupported(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::SourceAndDestinationAreSame(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestError::TargetRequired(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::TargetsRequired(ref cause) => write!(f, "{}", cause),
            CreatePullRequestError::TitleRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePullRequestError {}
/// Errors returned by CreatePullRequestApprovalRule
#[derive(Debug, PartialEq)]
pub enum CreatePullRequestApprovalRuleError {
    /// <p>The content for the approval rule is empty. You must provide some content for an approval rule. The content cannot be null.</p>
    ApprovalRuleContentRequired(String),
    /// <p>An approval rule with that name already exists. Approval rule names must be unique within the scope of a pull request.</p>
    ApprovalRuleNameAlreadyExists(String),
    /// <p>An approval rule name is required, but was not specified.</p>
    ApprovalRuleNameRequired(String),
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
    /// <p>The content for the approval rule is not valid.</p>
    InvalidApprovalRuleContent(String),
    /// <p>The name for the approval rule is not valid.</p>
    InvalidApprovalRuleName(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The approval rule cannot be added. The pull request has the maximum number of approval rules associated with it.</p>
    NumberOfRulesExceeded(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
}

impl CreatePullRequestApprovalRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreatePullRequestApprovalRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleContentRequiredException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::ApprovalRuleContentRequired(err.msg),
                    )
                }
                "ApprovalRuleNameAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::ApprovalRuleNameAlreadyExists(err.msg),
                    )
                }
                "ApprovalRuleNameRequiredException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::ApprovalRuleNameRequired(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidApprovalRuleContentException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::InvalidApprovalRuleContent(err.msg),
                    )
                }
                "InvalidApprovalRuleNameException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::InvalidApprovalRuleName(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::InvalidPullRequestId(err.msg),
                    )
                }
                "NumberOfRulesExceededException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::NumberOfRulesExceeded(err.msg),
                    )
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        CreatePullRequestApprovalRuleError::PullRequestIdRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePullRequestApprovalRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePullRequestApprovalRuleError::ApprovalRuleContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::ApprovalRuleNameAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::ApprovalRuleNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::InvalidApprovalRuleContent(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::InvalidApprovalRuleName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::NumberOfRulesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePullRequestApprovalRuleError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreatePullRequestApprovalRuleError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified tag is not valid. Key names cannot be prefixed with aws:.</p>
    InvalidSystemTagUsage(String),
    /// <p>The map of tags is not valid.</p>
    InvalidTagsMap(String),
    /// <p>A repository resource limit was exceeded.</p>
    RepositoryLimitExceeded(String),
    /// <p>The specified repository name already exists.</p>
    RepositoryNameExists(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRepositoryError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::InvalidRepositoryDescription(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRepositoryError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::InvalidSystemTagUsage(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::InvalidTagsMap(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::RepositoryLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::RepositoryNameExists(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::TagPolicy(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRepositoryError {}
/// Errors returned by CreateUnreferencedMergeCommit
#[derive(Debug, PartialEq)]
pub enum CreateUnreferencedMergeCommitError {
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>The commit message is too long. Provide a shorter string. </p>
    CommitMessageLengthExceeded(String),
    /// <p>A commit was not specified.</p>
    CommitRequired(String),
    /// <p>The merge cannot be completed because the target branch has been modified. Another user might have modified the target branch while the merge was in progress. Wait a few minutes, and then try again.</p>
    ConcurrentReferenceUpdate(String),
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
    /// <p>The file cannot be added because it is too large. The maximum file size is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>The commit cannot be created because no file mode has been specified. A file mode is required to update mode permissions for a file.</p>
    FileModeRequired(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit results in a folder whose contents exceed the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
    FolderContentSizeLimitExceeded(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution list is not valid.</p>
    InvalidConflictResolution(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The specified file mode permission is not valid. For a list of valid file mode permissions, see <a>PutFile</a>. </p>
    InvalidFileMode(String),
    /// <p>The specified merge option is not valid for this operation. Not all merge strategies are supported for all operations.</p>
    InvalidMergeOption(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>Automerge was specified for resolving the conflict, but the replacement type is not valid or content is missing. </p>
    InvalidReplacementContent(String),
    /// <p>Automerge was specified for resolving the conflict, but the specified replacement type is not valid.</p>
    InvalidReplacementType(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The number of allowed conflict resolution entries was exceeded.</p>
    MaximumConflictResolutionEntriesExceeded(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>A merge option or stategy is required, and none was provided.</p>
    MergeOptionRequired(String),
    /// <p>More than one conflict resolution entries exists for the conflict. A conflict can have only one conflict resolution entry.</p>
    MultipleConflictResolutionEntries(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
    NameLengthExceeded(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>USE_NEW_CONTENT was specified, but no replacement content has been provided.</p>
    ReplacementContentRequired(String),
    /// <p>A replacement type is required.</p>
    ReplacementTypeRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl CreateUnreferencedMergeCommitError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateUnreferencedMergeCommitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::CommitDoesNotExist(err.msg),
                    )
                }
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::CommitMessageLengthExceeded(err.msg),
                    )
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::CommitRequired(err.msg),
                    )
                }
                "ConcurrentReferenceUpdateException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::ConcurrentReferenceUpdate(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "FileContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::FileContentSizeLimitExceeded(err.msg),
                    )
                }
                "FileModeRequiredException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::FileModeRequired(err.msg),
                    )
                }
                "FolderContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::FolderContentSizeLimitExceeded(err.msg),
                    )
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(CreateUnreferencedMergeCommitError::InvalidCommit(
                        err.msg,
                    ))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidConflictResolution(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidConflictResolutionStrategy(
                            err.msg,
                        ),
                    )
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(CreateUnreferencedMergeCommitError::InvalidEmail(
                        err.msg,
                    ))
                }
                "InvalidFileModeException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidFileMode(err.msg),
                    )
                }
                "InvalidMergeOptionException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidMergeOption(err.msg),
                    )
                }
                "InvalidPathException" => {
                    return RusotoError::Service(CreateUnreferencedMergeCommitError::InvalidPath(
                        err.msg,
                    ))
                }
                "InvalidReplacementContentException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidReplacementContent(err.msg),
                    )
                }
                "InvalidReplacementTypeException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidReplacementType(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::InvalidRepositoryName(err.msg),
                    )
                }
                "ManualMergeRequiredException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::ManualMergeRequired(err.msg),
                    )
                }
                "MaximumConflictResolutionEntriesExceededException" => return RusotoError::Service(
                    CreateUnreferencedMergeCommitError::MaximumConflictResolutionEntriesExceeded(
                        err.msg,
                    ),
                ),
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::MaximumFileContentToLoadExceeded(
                            err.msg,
                        ),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "MergeOptionRequiredException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::MergeOptionRequired(err.msg),
                    )
                }
                "MultipleConflictResolutionEntriesException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::MultipleConflictResolutionEntries(
                            err.msg,
                        ),
                    )
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::NameLengthExceeded(err.msg),
                    )
                }
                "PathRequiredException" => {
                    return RusotoError::Service(CreateUnreferencedMergeCommitError::PathRequired(
                        err.msg,
                    ))
                }
                "ReplacementContentRequiredException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::ReplacementContentRequired(err.msg),
                    )
                }
                "ReplacementTypeRequiredException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::ReplacementTypeRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::RepositoryNameRequired(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        CreateUnreferencedMergeCommitError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUnreferencedMergeCommitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUnreferencedMergeCommitError::CommitDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::CommitMessageLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::CommitRequired(ref cause) => write!(f, "{}", cause),
            CreateUnreferencedMergeCommitError::ConcurrentReferenceUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::FileContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::FileModeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::FolderContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            CreateUnreferencedMergeCommitError::InvalidConflictDetailLevel(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidConflictResolution(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            CreateUnreferencedMergeCommitError::InvalidFileMode(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidMergeOption(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidPath(ref cause) => write!(f, "{}", cause),
            CreateUnreferencedMergeCommitError::InvalidReplacementContent(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidReplacementType(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::ManualMergeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::MaximumConflictResolutionEntriesExceeded(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateUnreferencedMergeCommitError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::MergeOptionRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::MultipleConflictResolutionEntries(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::NameLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::PathRequired(ref cause) => write!(f, "{}", cause),
            CreateUnreferencedMergeCommitError::ReplacementContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::ReplacementTypeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUnreferencedMergeCommitError::TipsDivergenceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateUnreferencedMergeCommitError {}
/// Errors returned by DeleteApprovalRuleTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteApprovalRuleTemplateError {
    /// <p>The approval rule template is associated with one or more repositories. You cannot delete a template that is associated with a repository. Remove all associations, and then try again.</p>
    ApprovalRuleTemplateInUse(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
}

impl DeleteApprovalRuleTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApprovalRuleTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleTemplateInUseException" => {
                    return RusotoError::Service(
                        DeleteApprovalRuleTemplateError::ApprovalRuleTemplateInUse(err.msg),
                    )
                }
                "ApprovalRuleTemplateNameRequiredException" => {
                    return RusotoError::Service(
                        DeleteApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(err.msg),
                    )
                }
                "InvalidApprovalRuleTemplateNameException" => {
                    return RusotoError::Service(
                        DeleteApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApprovalRuleTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApprovalRuleTemplateError::ApprovalRuleTemplateInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteApprovalRuleTemplateError {}
/// Errors returned by DeleteBranch
#[derive(Debug, PartialEq)]
pub enum DeleteBranchError {
    /// <p>A branch name is required, but was not specified.</p>
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBranchError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::DefaultBranchCannotBeDeleted(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteBranchError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBranchError {}
/// Errors returned by DeleteCommentContent
#[derive(Debug, PartialEq)]
pub enum DeleteCommentContentError {
    /// <p>This comment has already been deleted. You cannot edit or delete a deleted comment.</p>
    CommentDeleted(String),
    /// <p>No comment exists with the provided ID. Verify that you have used the correct ID, and then try again.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCommentContentError::CommentDeleted(ref cause) => write!(f, "{}", cause),
            DeleteCommentContentError::CommentDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteCommentContentError::CommentIdRequired(ref cause) => write!(f, "{}", cause),
            DeleteCommentContentError::InvalidCommentId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCommentContentError {}
/// Errors returned by DeleteFile
#[derive(Debug, PartialEq)]
pub enum DeleteFileError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Enter the name of a branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required, but was not specified.</p>
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
    /// <p>The specified file does not exist. Verify that you have used the correct file name, full path, and extension.</p>
    FileDoesNotExist(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The parent commit ID is not valid. The commit ID cannot be empty, and must match the head commit ID for the branch of the repository where you want to add or update a file.</p>
    InvalidParentCommitId(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
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
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFileError::BranchDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteFileError::BranchNameIsTagName(ref cause) => write!(f, "{}", cause),
            DeleteFileError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            DeleteFileError::CommitMessageLengthExceeded(ref cause) => write!(f, "{}", cause),
            DeleteFileError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            DeleteFileError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteFileError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            DeleteFileError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFileError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteFileError::FileDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteFileError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            DeleteFileError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            DeleteFileError::InvalidParentCommitId(ref cause) => write!(f, "{}", cause),
            DeleteFileError::InvalidPath(ref cause) => write!(f, "{}", cause),
            DeleteFileError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            DeleteFileError::NameLengthExceeded(ref cause) => write!(f, "{}", cause),
            DeleteFileError::ParentCommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteFileError::ParentCommitIdOutdated(ref cause) => write!(f, "{}", cause),
            DeleteFileError::ParentCommitIdRequired(ref cause) => write!(f, "{}", cause),
            DeleteFileError::PathRequired(ref cause) => write!(f, "{}", cause),
            DeleteFileError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteFileError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFileError {}
/// Errors returned by DeletePullRequestApprovalRule
#[derive(Debug, PartialEq)]
pub enum DeletePullRequestApprovalRuleError {
    /// <p>An approval rule name is required, but was not specified.</p>
    ApprovalRuleNameRequired(String),
    /// <p>The approval rule cannot be deleted from the pull request because it was created by an approval rule template and applied to the pull request automatically.</p>
    CannotDeleteApprovalRuleFromTemplate(String),
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
    /// <p>The name for the approval rule is not valid.</p>
    InvalidApprovalRuleName(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
}

impl DeletePullRequestApprovalRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeletePullRequestApprovalRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleNameRequiredException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::ApprovalRuleNameRequired(err.msg),
                    )
                }
                "CannotDeleteApprovalRuleFromTemplateException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::CannotDeleteApprovalRuleFromTemplate(
                            err.msg,
                        ),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidApprovalRuleNameException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::InvalidApprovalRuleName(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::InvalidPullRequestId(err.msg),
                    )
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        DeletePullRequestApprovalRuleError::PullRequestIdRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePullRequestApprovalRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePullRequestApprovalRuleError::ApprovalRuleNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::CannotDeleteApprovalRuleFromTemplate(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::InvalidApprovalRuleName(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePullRequestApprovalRuleError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeletePullRequestApprovalRuleError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRepositoryError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRepositoryError {}
/// Errors returned by DescribeMergeConflicts
#[derive(Debug, PartialEq)]
pub enum DescribeMergeConflictsError {
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
    /// <p>The specified file does not exist. Verify that you have used the correct file name, full path, and extension.</p>
    FileDoesNotExist(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified value for the number of merge hunks to return is not valid.</p>
    InvalidMaxMergeHunks(String),
    /// <p>The specified merge option is not valid for this operation. Not all merge strategies are supported for all operations.</p>
    InvalidMergeOption(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>A merge option or stategy is required, and none was provided.</p>
    MergeOptionRequired(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl DescribeMergeConflictsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMergeConflictsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::CommitDoesNotExist(
                        err.msg,
                    ))
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::CommitRequired(
                        err.msg,
                    ))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "FileDoesNotExistException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::FileDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::InvalidCommit(
                        err.msg,
                    ))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::InvalidContinuationToken(err.msg),
                    )
                }
                "InvalidMaxMergeHunksException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::InvalidMaxMergeHunks(
                        err.msg,
                    ))
                }
                "InvalidMergeOptionException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::InvalidMergeOption(
                        err.msg,
                    ))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::InvalidPath(err.msg))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::InvalidRepositoryName(err.msg),
                    )
                }
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "MergeOptionRequiredException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::MergeOptionRequired(
                        err.msg,
                    ))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(DescribeMergeConflictsError::PathRequired(err.msg))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::RepositoryNameRequired(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        DescribeMergeConflictsError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMergeConflictsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMergeConflictsError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::CommitRequired(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::FileDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::InvalidConflictDetailLevel(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::InvalidContinuationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::InvalidMaxMergeHunks(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::InvalidMergeOption(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::InvalidPath(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::MergeOptionRequired(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::PathRequired(ref cause) => write!(f, "{}", cause),
            DescribeMergeConflictsError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMergeConflictsError::TipsDivergenceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeMergeConflictsError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePullRequestEventsError::ActorDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribePullRequestEventsError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::InvalidActorArn(ref cause) => write!(f, "{}", cause),
            DescribePullRequestEventsError::InvalidContinuationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::InvalidMaxResults(ref cause) => write!(f, "{}", cause),
            DescribePullRequestEventsError::InvalidPullRequestEventType(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePullRequestEventsError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribePullRequestEventsError {}
/// Errors returned by DisassociateApprovalRuleTemplateFromRepository
#[derive(Debug, PartialEq)]
pub enum DisassociateApprovalRuleTemplateFromRepositoryError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
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
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
}

impl DisassociateApprovalRuleTemplateFromRepositoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateApprovalRuleTemplateFromRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "ApprovalRuleTemplateDoesNotExistException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::ApprovalRuleTemplateDoesNotExist(err.msg)),
"ApprovalRuleTemplateNameRequiredException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::ApprovalRuleTemplateNameRequired(err.msg)),
"EncryptionIntegrityChecksFailedException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionIntegrityChecksFailed(err.msg)),
"EncryptionKeyAccessDeniedException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyAccessDenied(err.msg)),
"EncryptionKeyDisabledException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyDisabled(err.msg)),
"EncryptionKeyNotFoundException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyNotFound(err.msg)),
"EncryptionKeyUnavailableException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyUnavailable(err.msg)),
"InvalidApprovalRuleTemplateNameException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::InvalidApprovalRuleTemplateName(err.msg)),
"InvalidRepositoryNameException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::InvalidRepositoryName(err.msg)),
"RepositoryDoesNotExistException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::RepositoryDoesNotExist(err.msg)),
"RepositoryNameRequiredException" => return RusotoError::Service(DisassociateApprovalRuleTemplateFromRepositoryError::RepositoryNameRequired(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateApprovalRuleTemplateFromRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            DisassociateApprovalRuleTemplateFromRepositoryError::ApprovalRuleTemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::ApprovalRuleTemplateNameRequired(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::InvalidApprovalRuleTemplateName(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
DisassociateApprovalRuleTemplateFromRepositoryError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for DisassociateApprovalRuleTemplateFromRepositoryError {}
/// Errors returned by EvaluatePullRequestApprovalRules
#[derive(Debug, PartialEq)]
pub enum EvaluatePullRequestApprovalRulesError {
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
    /// <p>The revision ID is not valid. Use GetPullRequest to determine the value.</p>
    InvalidRevisionId(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>A revision ID is required, but was not provided.</p>
    RevisionIdRequired(String),
    /// <p>The revision ID provided in the request does not match the current revision ID. Use GetPullRequest to retrieve the current revision ID.</p>
    RevisionNotCurrent(String),
}

impl EvaluatePullRequestApprovalRulesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EvaluatePullRequestApprovalRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRevisionIdException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::InvalidRevisionId(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::PullRequestIdRequired(err.msg),
                    )
                }
                "RevisionIdRequiredException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::RevisionIdRequired(err.msg),
                    )
                }
                "RevisionNotCurrentException" => {
                    return RusotoError::Service(
                        EvaluatePullRequestApprovalRulesError::RevisionNotCurrent(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EvaluatePullRequestApprovalRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvaluatePullRequestApprovalRulesError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::InvalidRevisionId(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::RevisionIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            EvaluatePullRequestApprovalRulesError::RevisionNotCurrent(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EvaluatePullRequestApprovalRulesError {}
/// Errors returned by GetApprovalRuleTemplate
#[derive(Debug, PartialEq)]
pub enum GetApprovalRuleTemplateError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
}

impl GetApprovalRuleTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApprovalRuleTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleTemplateDoesNotExistException" => {
                    return RusotoError::Service(
                        GetApprovalRuleTemplateError::ApprovalRuleTemplateDoesNotExist(err.msg),
                    )
                }
                "ApprovalRuleTemplateNameRequiredException" => {
                    return RusotoError::Service(
                        GetApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(err.msg),
                    )
                }
                "InvalidApprovalRuleTemplateNameException" => {
                    return RusotoError::Service(
                        GetApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApprovalRuleTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApprovalRuleTemplateError::ApprovalRuleTemplateDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetApprovalRuleTemplateError {}
/// Errors returned by GetBlob
#[derive(Debug, PartialEq)]
pub enum GetBlobError {
    /// <p>The specified blob does not exist.</p>
    BlobIdDoesNotExist(String),
    /// <p>A blob ID is required, but was not specified.</p>
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBlobError::BlobIdDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetBlobError::BlobIdRequired(ref cause) => write!(f, "{}", cause),
            GetBlobError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            GetBlobError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetBlobError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetBlobError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetBlobError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetBlobError::FileTooLarge(ref cause) => write!(f, "{}", cause),
            GetBlobError::InvalidBlobId(ref cause) => write!(f, "{}", cause),
            GetBlobError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetBlobError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetBlobError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBlobError {}
/// Errors returned by GetBranch
#[derive(Debug, PartialEq)]
pub enum GetBranchError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>A branch name is required, but was not specified.</p>
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBranchError::BranchDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetBranchError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            GetBranchError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            GetBranchError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetBranchError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetBranchError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetBranchError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetBranchError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            GetBranchError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetBranchError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetBranchError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBranchError {}
/// Errors returned by GetComment
#[derive(Debug, PartialEq)]
pub enum GetCommentError {
    /// <p>This comment has already been deleted. You cannot edit or delete a deleted comment.</p>
    CommentDeleted(String),
    /// <p>No comment exists with the provided ID. Verify that you have used the correct ID, and then try again.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCommentError::CommentDeleted(ref cause) => write!(f, "{}", cause),
            GetCommentError::CommentDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetCommentError::CommentIdRequired(ref cause) => write!(f, "{}", cause),
            GetCommentError::InvalidCommentId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCommentError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCommentsForComparedCommitError::CommitDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::CommitIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            GetCommentsForComparedCommitError::InvalidContinuationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::InvalidMaxResults(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForComparedCommitError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetCommentsForComparedCommitError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCommentsForPullRequestError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetCommentsForPullRequestError::CommitIdRequired(ref cause) => write!(f, "{}", cause),
            GetCommentsForPullRequestError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            GetCommentsForPullRequestError::InvalidContinuationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::InvalidMaxResults(ref cause) => write!(f, "{}", cause),
            GetCommentsForPullRequestError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCommentsForPullRequestError::RepositoryNotAssociatedWithPullRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetCommentsForPullRequestError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCommitError::CommitIdDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetCommitError::CommitIdRequired(ref cause) => write!(f, "{}", cause),
            GetCommitError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            GetCommitError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetCommitError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetCommitError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetCommitError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetCommitError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            GetCommitError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetCommitError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetCommitError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCommitError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified path does not exist.</p>
    PathDoesNotExist(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDifferencesError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::CommitRequired(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDifferencesError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::InvalidContinuationToken(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::InvalidMaxResults(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::InvalidPath(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::PathDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDifferencesError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDifferencesError {}
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
    /// <p>The specified file does not exist. Verify that you have used the correct file name, full path, and extension.</p>
    FileDoesNotExist(String),
    /// <p>The specified file exceeds the file size limit for AWS CodeCommit. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    FileTooLarge(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFileError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetFileError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            GetFileError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetFileError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetFileError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetFileError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetFileError::FileDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetFileError::FileTooLarge(ref cause) => write!(f, "{}", cause),
            GetFileError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            GetFileError::InvalidPath(ref cause) => write!(f, "{}", cause),
            GetFileError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetFileError::PathRequired(ref cause) => write!(f, "{}", cause),
            GetFileError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetFileError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFileError {}
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
    /// <p>The specified folder does not exist. Either the folder name is not correct, or you did not enter the full path to the folder.</p>
    FolderDoesNotExist(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFolderError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetFolderError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            GetFolderError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetFolderError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetFolderError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetFolderError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetFolderError::FolderDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetFolderError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            GetFolderError::InvalidPath(ref cause) => write!(f, "{}", cause),
            GetFolderError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetFolderError::PathRequired(ref cause) => write!(f, "{}", cause),
            GetFolderError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetFolderError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFolderError {}
/// Errors returned by GetMergeCommit
#[derive(Debug, PartialEq)]
pub enum GetMergeCommitError {
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
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
}

impl GetMergeCommitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMergeCommitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(GetMergeCommitError::CommitDoesNotExist(err.msg))
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(GetMergeCommitError::CommitRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetMergeCommitError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetMergeCommitError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetMergeCommitError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetMergeCommitError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetMergeCommitError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(GetMergeCommitError::InvalidCommit(err.msg))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(GetMergeCommitError::InvalidConflictDetailLevel(
                        err.msg,
                    ))
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        GetMergeCommitError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetMergeCommitError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetMergeCommitError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetMergeCommitError::RepositoryNameRequired(
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
impl fmt::Display for GetMergeCommitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMergeCommitError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::CommitRequired(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeCommitError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::InvalidConflictDetailLevel(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeCommitError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetMergeCommitError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMergeCommitError {}
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
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The destination commit specifier is not valid. You must provide a valid branch name, tag, or full commit ID. </p>
    InvalidDestinationCommitSpecifier(String),
    /// <p>The specified value for the number of conflict files to return is not valid.</p>
    InvalidMaxConflictFiles(String),
    /// <p>The specified merge option is not valid for this operation. Not all merge strategies are supported for all operations.</p>
    InvalidMergeOption(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The source commit specifier is not valid. You must provide a valid branch name, tag, or full commit ID.</p>
    InvalidSourceCommitSpecifier(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>A merge option or stategy is required, and none was provided.</p>
    MergeOptionRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(GetMergeConflictsError::InvalidContinuationToken(
                        err.msg,
                    ))
                }
                "InvalidDestinationCommitSpecifierException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::InvalidDestinationCommitSpecifier(err.msg),
                    )
                }
                "InvalidMaxConflictFilesException" => {
                    return RusotoError::Service(GetMergeConflictsError::InvalidMaxConflictFiles(
                        err.msg,
                    ))
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
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        GetMergeConflictsError::MaximumItemsToCompareExceeded(err.msg),
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMergeConflictsError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::CommitRequired(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeConflictsError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::InvalidConflictDetailLevel(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeConflictsError::InvalidContinuationToken(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::InvalidDestinationCommitSpecifier(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeConflictsError::InvalidMaxConflictFiles(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::InvalidMergeOption(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::InvalidSourceCommitSpecifier(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeConflictsError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeConflictsError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeConflictsError::MergeOptionRequired(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            GetMergeConflictsError::TipsDivergenceExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMergeConflictsError {}
/// Errors returned by GetMergeOptions
#[derive(Debug, PartialEq)]
pub enum GetMergeOptionsError {
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
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl GetMergeOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMergeOptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(GetMergeOptionsError::CommitDoesNotExist(err.msg))
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(GetMergeOptionsError::CommitRequired(err.msg))
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetMergeOptionsError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(GetMergeOptionsError::EncryptionKeyAccessDenied(
                        err.msg,
                    ))
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(GetMergeOptionsError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(GetMergeOptionsError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(GetMergeOptionsError::EncryptionKeyUnavailable(
                        err.msg,
                    ))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(GetMergeOptionsError::InvalidCommit(err.msg))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(GetMergeOptionsError::InvalidConflictDetailLevel(
                        err.msg,
                    ))
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        GetMergeOptionsError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(GetMergeOptionsError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        GetMergeOptionsError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        GetMergeOptionsError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(GetMergeOptionsError::RepositoryDoesNotExist(
                        err.msg,
                    ))
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(GetMergeOptionsError::RepositoryNameRequired(
                        err.msg,
                    ))
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(GetMergeOptionsError::TipsDivergenceExceeded(
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
impl fmt::Display for GetMergeOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMergeOptionsError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::CommitRequired(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeOptionsError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::InvalidConflictDetailLevel(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeOptionsError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeOptionsError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetMergeOptionsError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            GetMergeOptionsError::TipsDivergenceExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMergeOptionsError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPullRequestError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetPullRequestError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetPullRequestError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetPullRequestError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetPullRequestError::InvalidPullRequestId(ref cause) => write!(f, "{}", cause),
            GetPullRequestError::PullRequestDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetPullRequestError::PullRequestIdRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPullRequestError {}
/// Errors returned by GetPullRequestApprovalStates
#[derive(Debug, PartialEq)]
pub enum GetPullRequestApprovalStatesError {
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
    /// <p>The revision ID is not valid. Use GetPullRequest to determine the value.</p>
    InvalidRevisionId(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>A revision ID is required, but was not provided.</p>
    RevisionIdRequired(String),
}

impl GetPullRequestApprovalStatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetPullRequestApprovalStatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRevisionIdException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::InvalidRevisionId(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::PullRequestIdRequired(err.msg),
                    )
                }
                "RevisionIdRequiredException" => {
                    return RusotoError::Service(
                        GetPullRequestApprovalStatesError::RevisionIdRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPullRequestApprovalStatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPullRequestApprovalStatesError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::InvalidRevisionId(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestApprovalStatesError::RevisionIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetPullRequestApprovalStatesError {}
/// Errors returned by GetPullRequestOverrideState
#[derive(Debug, PartialEq)]
pub enum GetPullRequestOverrideStateError {
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
    /// <p>The revision ID is not valid. Use GetPullRequest to determine the value.</p>
    InvalidRevisionId(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>A revision ID is required, but was not provided.</p>
    RevisionIdRequired(String),
}

impl GetPullRequestOverrideStateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetPullRequestOverrideStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRevisionIdException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::InvalidRevisionId(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::PullRequestIdRequired(err.msg),
                    )
                }
                "RevisionIdRequiredException" => {
                    return RusotoError::Service(
                        GetPullRequestOverrideStateError::RevisionIdRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPullRequestOverrideStateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPullRequestOverrideStateError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::InvalidRevisionId(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPullRequestOverrideStateError::RevisionIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetPullRequestOverrideStateError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRepositoryError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            GetRepositoryError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetRepositoryError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetRepositoryError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetRepositoryError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetRepositoryError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetRepositoryError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRepositoryError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRepositoryTriggersError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRepositoryTriggersError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRepositoryTriggersError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            GetRepositoryTriggersError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            GetRepositoryTriggersError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRepositoryTriggersError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            GetRepositoryTriggersError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetRepositoryTriggersError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRepositoryTriggersError {}
/// Errors returned by ListApprovalRuleTemplates
#[derive(Debug, PartialEq)]
pub enum ListApprovalRuleTemplatesError {
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
}

impl ListApprovalRuleTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApprovalRuleTemplatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(
                        ListApprovalRuleTemplatesError::InvalidContinuationToken(err.msg),
                    )
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(ListApprovalRuleTemplatesError::InvalidMaxResults(
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
impl fmt::Display for ListApprovalRuleTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApprovalRuleTemplatesError::InvalidContinuationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApprovalRuleTemplatesError::InvalidMaxResults(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApprovalRuleTemplatesError {}
/// Errors returned by ListAssociatedApprovalRuleTemplatesForRepository
#[derive(Debug, PartialEq)]
pub enum ListAssociatedApprovalRuleTemplatesForRepositoryError {
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
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
}

impl ListAssociatedApprovalRuleTemplatesForRepositoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAssociatedApprovalRuleTemplatesForRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "EncryptionIntegrityChecksFailedException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionIntegrityChecksFailed(err.msg)),
"EncryptionKeyAccessDeniedException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyAccessDenied(err.msg)),
"EncryptionKeyDisabledException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyDisabled(err.msg)),
"EncryptionKeyNotFoundException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyNotFound(err.msg)),
"EncryptionKeyUnavailableException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyUnavailable(err.msg)),
"InvalidContinuationTokenException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::InvalidContinuationToken(err.msg)),
"InvalidMaxResultsException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::InvalidMaxResults(err.msg)),
"InvalidRepositoryNameException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::InvalidRepositoryName(err.msg)),
"RepositoryDoesNotExistException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::RepositoryDoesNotExist(err.msg)),
"RepositoryNameRequiredException" => return RusotoError::Service(ListAssociatedApprovalRuleTemplatesForRepositoryError::RepositoryNameRequired(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssociatedApprovalRuleTemplatesForRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::InvalidContinuationToken(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::InvalidMaxResults(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
ListAssociatedApprovalRuleTemplatesForRepositoryError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for ListAssociatedApprovalRuleTemplatesForRepositoryError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBranchesError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            ListBranchesError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            ListBranchesError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            ListBranchesError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            ListBranchesError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            ListBranchesError::InvalidContinuationToken(ref cause) => write!(f, "{}", cause),
            ListBranchesError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            ListBranchesError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListBranchesError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBranchesError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPullRequestsError::AuthorDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPullRequestsError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::InvalidAuthorArn(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::InvalidContinuationToken(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::InvalidMaxResults(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::InvalidPullRequestStatus(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListPullRequestsError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPullRequestsError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRepositoriesError::InvalidContinuationToken(ref cause) => write!(f, "{}", cause),
            ListRepositoriesError::InvalidOrder(ref cause) => write!(f, "{}", cause),
            ListRepositoriesError::InvalidSortBy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRepositoriesError {}
/// Errors returned by ListRepositoriesForApprovalRuleTemplate
#[derive(Debug, PartialEq)]
pub enum ListRepositoriesForApprovalRuleTemplateError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
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
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
    /// <p>The specified continuation token is not valid.</p>
    InvalidContinuationToken(String),
    /// <p>The specified number of maximum results is not valid.</p>
    InvalidMaxResults(String),
}

impl ListRepositoriesForApprovalRuleTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRepositoriesForApprovalRuleTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleTemplateDoesNotExistException" => return RusotoError::Service(
                    ListRepositoriesForApprovalRuleTemplateError::ApprovalRuleTemplateDoesNotExist(
                        err.msg,
                    ),
                ),
                "ApprovalRuleTemplateNameRequiredException" => return RusotoError::Service(
                    ListRepositoriesForApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(
                        err.msg,
                    ),
                ),
                "EncryptionIntegrityChecksFailedException" => return RusotoError::Service(
                    ListRepositoriesForApprovalRuleTemplateError::EncryptionIntegrityChecksFailed(
                        err.msg,
                    ),
                ),
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyDisabled(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyNotFound(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyUnavailable(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleTemplateNameException" => return RusotoError::Service(
                    ListRepositoriesForApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(
                        err.msg,
                    ),
                ),
                "InvalidContinuationTokenException" => {
                    return RusotoError::Service(
                        ListRepositoriesForApprovalRuleTemplateError::InvalidContinuationToken(
                            err.msg,
                        ),
                    )
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(
                        ListRepositoriesForApprovalRuleTemplateError::InvalidMaxResults(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRepositoriesForApprovalRuleTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRepositoriesForApprovalRuleTemplateError::ApprovalRuleTemplateDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
            ListRepositoriesForApprovalRuleTemplateError::ApprovalRuleTemplateNameRequired(
                ref cause,
            ) => write!(f, "{}", cause),
            ListRepositoriesForApprovalRuleTemplateError::EncryptionIntegrityChecksFailed(
                ref cause,
            ) => write!(f, "{}", cause),
            ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRepositoriesForApprovalRuleTemplateError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRepositoriesForApprovalRuleTemplateError::InvalidApprovalRuleTemplateName(
                ref cause,
            ) => write!(f, "{}", cause),
            ListRepositoriesForApprovalRuleTemplateError::InvalidContinuationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRepositoriesForApprovalRuleTemplateError::InvalidMaxResults(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListRepositoriesForApprovalRuleTemplateError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidResourceArn(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceArnRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by MergeBranchesByFastForward
#[derive(Debug, PartialEq)]
pub enum MergeBranchesByFastForwardError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Enter the name of a branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required, but was not specified.</p>
    BranchNameRequired(String),
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>A commit was not specified.</p>
    CommitRequired(String),
    /// <p>The merge cannot be completed because the target branch has been modified. Another user might have modified the target branch while the merge was in progress. Wait a few minutes, and then try again.</p>
    ConcurrentReferenceUpdate(String),
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
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified target branch is not valid.</p>
    InvalidTargetBranch(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl MergeBranchesByFastForwardError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<MergeBranchesByFastForwardError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::BranchDoesNotExist(err.msg),
                    )
                }
                "BranchNameIsTagNameException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::BranchNameIsTagName(err.msg),
                    )
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::BranchNameRequired(err.msg),
                    )
                }
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::CommitDoesNotExist(err.msg),
                    )
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(MergeBranchesByFastForwardError::CommitRequired(
                        err.msg,
                    ))
                }
                "ConcurrentReferenceUpdateException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::ConcurrentReferenceUpdate(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::InvalidBranchName(err.msg),
                    )
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(MergeBranchesByFastForwardError::InvalidCommit(
                        err.msg,
                    ))
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::InvalidRepositoryName(err.msg),
                    )
                }
                "InvalidTargetBranchException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::InvalidTargetBranch(err.msg),
                    )
                }
                "ManualMergeRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::ManualMergeRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::RepositoryNameRequired(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByFastForwardError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MergeBranchesByFastForwardError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MergeBranchesByFastForwardError::BranchDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::BranchNameIsTagName(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::BranchNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::CommitDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::CommitRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesByFastForwardError::ConcurrentReferenceUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            MergeBranchesByFastForwardError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            MergeBranchesByFastForwardError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::InvalidTargetBranch(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::ManualMergeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByFastForwardError::TipsDivergenceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for MergeBranchesByFastForwardError {}
/// Errors returned by MergeBranchesBySquash
#[derive(Debug, PartialEq)]
pub enum MergeBranchesBySquashError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Enter the name of a branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required, but was not specified.</p>
    BranchNameRequired(String),
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>The commit message is too long. Provide a shorter string. </p>
    CommitMessageLengthExceeded(String),
    /// <p>A commit was not specified.</p>
    CommitRequired(String),
    /// <p>The merge cannot be completed because the target branch has been modified. Another user might have modified the target branch while the merge was in progress. Wait a few minutes, and then try again.</p>
    ConcurrentReferenceUpdate(String),
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
    /// <p>The file cannot be added because it is too large. The maximum file size is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>The commit cannot be created because no file mode has been specified. A file mode is required to update mode permissions for a file.</p>
    FileModeRequired(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit results in a folder whose contents exceed the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
    FolderContentSizeLimitExceeded(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution list is not valid.</p>
    InvalidConflictResolution(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The specified file mode permission is not valid. For a list of valid file mode permissions, see <a>PutFile</a>. </p>
    InvalidFileMode(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>Automerge was specified for resolving the conflict, but the replacement type is not valid or content is missing. </p>
    InvalidReplacementContent(String),
    /// <p>Automerge was specified for resolving the conflict, but the specified replacement type is not valid.</p>
    InvalidReplacementType(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified target branch is not valid.</p>
    InvalidTargetBranch(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The number of allowed conflict resolution entries was exceeded.</p>
    MaximumConflictResolutionEntriesExceeded(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>More than one conflict resolution entries exists for the conflict. A conflict can have only one conflict resolution entry.</p>
    MultipleConflictResolutionEntries(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
    NameLengthExceeded(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>USE_NEW_CONTENT was specified, but no replacement content has been provided.</p>
    ReplacementContentRequired(String),
    /// <p>A replacement type is required.</p>
    ReplacementTypeRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl MergeBranchesBySquashError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MergeBranchesBySquashError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::BranchDoesNotExist(
                        err.msg,
                    ))
                }
                "BranchNameIsTagNameException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::BranchNameIsTagName(
                        err.msg,
                    ))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::BranchNameRequired(
                        err.msg,
                    ))
                }
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::CommitDoesNotExist(
                        err.msg,
                    ))
                }
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::CommitMessageLengthExceeded(err.msg),
                    )
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::CommitRequired(
                        err.msg,
                    ))
                }
                "ConcurrentReferenceUpdateException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::ConcurrentReferenceUpdate(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::EncryptionKeyDisabled(
                        err.msg,
                    ))
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::EncryptionKeyNotFound(
                        err.msg,
                    ))
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "FileContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::FileContentSizeLimitExceeded(err.msg),
                    )
                }
                "FileModeRequiredException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::FileModeRequired(
                        err.msg,
                    ))
                }
                "FolderContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::FolderContentSizeLimitExceeded(err.msg),
                    )
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::InvalidBranchName(
                        err.msg,
                    ))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::InvalidCommit(err.msg))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::InvalidConflictResolution(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::InvalidEmail(err.msg))
                }
                "InvalidFileModeException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::InvalidFileMode(
                        err.msg,
                    ))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::InvalidPath(err.msg))
                }
                "InvalidReplacementContentException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::InvalidReplacementContent(err.msg),
                    )
                }
                "InvalidReplacementTypeException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::InvalidReplacementType(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::InvalidRepositoryName(
                        err.msg,
                    ))
                }
                "InvalidTargetBranchException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::InvalidTargetBranch(
                        err.msg,
                    ))
                }
                "ManualMergeRequiredException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::ManualMergeRequired(
                        err.msg,
                    ))
                }
                "MaximumConflictResolutionEntriesExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::MaximumConflictResolutionEntriesExceeded(
                            err.msg,
                        ),
                    )
                }
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "MultipleConflictResolutionEntriesException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::MultipleConflictResolutionEntries(err.msg),
                    )
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::NameLengthExceeded(
                        err.msg,
                    ))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(MergeBranchesBySquashError::PathRequired(err.msg))
                }
                "ReplacementContentRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::ReplacementContentRequired(err.msg),
                    )
                }
                "ReplacementTypeRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::ReplacementTypeRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::RepositoryNameRequired(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesBySquashError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MergeBranchesBySquashError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MergeBranchesBySquashError::BranchDoesNotExist(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::BranchNameIsTagName(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::CommitMessageLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::CommitRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::ConcurrentReferenceUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::FileContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::FileModeRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::FolderContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::InvalidConflictDetailLevel(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::InvalidConflictResolution(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::InvalidFileMode(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::InvalidPath(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::InvalidReplacementContent(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::InvalidReplacementType(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::InvalidTargetBranch(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::ManualMergeRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::MaximumConflictResolutionEntriesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::MultipleConflictResolutionEntries(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::NameLengthExceeded(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::PathRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::ReplacementContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::ReplacementTypeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesBySquashError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesBySquashError::TipsDivergenceExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for MergeBranchesBySquashError {}
/// Errors returned by MergeBranchesByThreeWay
#[derive(Debug, PartialEq)]
pub enum MergeBranchesByThreeWayError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Enter the name of a branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required, but was not specified.</p>
    BranchNameRequired(String),
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExist(String),
    /// <p>The commit message is too long. Provide a shorter string. </p>
    CommitMessageLengthExceeded(String),
    /// <p>A commit was not specified.</p>
    CommitRequired(String),
    /// <p>The merge cannot be completed because the target branch has been modified. Another user might have modified the target branch while the merge was in progress. Wait a few minutes, and then try again.</p>
    ConcurrentReferenceUpdate(String),
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
    /// <p>The file cannot be added because it is too large. The maximum file size is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>The commit cannot be created because no file mode has been specified. A file mode is required to update mode permissions for a file.</p>
    FileModeRequired(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit results in a folder whose contents exceed the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
    FolderContentSizeLimitExceeded(String),
    /// <p>The specified reference name is not valid.</p>
    InvalidBranchName(String),
    /// <p>The specified commit is not valid.</p>
    InvalidCommit(String),
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution list is not valid.</p>
    InvalidConflictResolution(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The specified file mode permission is not valid. For a list of valid file mode permissions, see <a>PutFile</a>. </p>
    InvalidFileMode(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>Automerge was specified for resolving the conflict, but the replacement type is not valid or content is missing. </p>
    InvalidReplacementContent(String),
    /// <p>Automerge was specified for resolving the conflict, but the specified replacement type is not valid.</p>
    InvalidReplacementType(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified target branch is not valid.</p>
    InvalidTargetBranch(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The number of allowed conflict resolution entries was exceeded.</p>
    MaximumConflictResolutionEntriesExceeded(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>More than one conflict resolution entries exists for the conflict. A conflict can have only one conflict resolution entry.</p>
    MultipleConflictResolutionEntries(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
    NameLengthExceeded(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>USE_NEW_CONTENT was specified, but no replacement content has been provided.</p>
    ReplacementContentRequired(String),
    /// <p>A replacement type is required.</p>
    ReplacementTypeRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl MergeBranchesByThreeWayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MergeBranchesByThreeWayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BranchDoesNotExistException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::BranchDoesNotExist(
                        err.msg,
                    ))
                }
                "BranchNameIsTagNameException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::BranchNameIsTagName(
                        err.msg,
                    ))
                }
                "BranchNameRequiredException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::BranchNameRequired(
                        err.msg,
                    ))
                }
                "CommitDoesNotExistException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::CommitDoesNotExist(
                        err.msg,
                    ))
                }
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::CommitMessageLengthExceeded(err.msg),
                    )
                }
                "CommitRequiredException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::CommitRequired(
                        err.msg,
                    ))
                }
                "ConcurrentReferenceUpdateException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::ConcurrentReferenceUpdate(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "FileContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::FileContentSizeLimitExceeded(err.msg),
                    )
                }
                "FileModeRequiredException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::FileModeRequired(
                        err.msg,
                    ))
                }
                "FolderContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::FolderContentSizeLimitExceeded(err.msg),
                    )
                }
                "InvalidBranchNameException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::InvalidBranchName(
                        err.msg,
                    ))
                }
                "InvalidCommitException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::InvalidCommit(
                        err.msg,
                    ))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::InvalidConflictResolution(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::InvalidEmail(
                        err.msg,
                    ))
                }
                "InvalidFileModeException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::InvalidFileMode(
                        err.msg,
                    ))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::InvalidPath(err.msg))
                }
                "InvalidReplacementContentException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::InvalidReplacementContent(err.msg),
                    )
                }
                "InvalidReplacementTypeException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::InvalidReplacementType(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::InvalidRepositoryName(err.msg),
                    )
                }
                "InvalidTargetBranchException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::InvalidTargetBranch(
                        err.msg,
                    ))
                }
                "ManualMergeRequiredException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::ManualMergeRequired(
                        err.msg,
                    ))
                }
                "MaximumConflictResolutionEntriesExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::MaximumConflictResolutionEntriesExceeded(
                            err.msg,
                        ),
                    )
                }
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "MultipleConflictResolutionEntriesException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::MultipleConflictResolutionEntries(err.msg),
                    )
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::NameLengthExceeded(
                        err.msg,
                    ))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(MergeBranchesByThreeWayError::PathRequired(
                        err.msg,
                    ))
                }
                "ReplacementContentRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::ReplacementContentRequired(err.msg),
                    )
                }
                "ReplacementTypeRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::ReplacementTypeRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::RepositoryNameRequired(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        MergeBranchesByThreeWayError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MergeBranchesByThreeWayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MergeBranchesByThreeWayError::BranchDoesNotExist(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::BranchNameIsTagName(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::CommitMessageLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::CommitRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::ConcurrentReferenceUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::FileContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::FileModeRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::FolderContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::InvalidCommit(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::InvalidConflictDetailLevel(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::InvalidConflictResolution(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::InvalidFileMode(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::InvalidPath(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::InvalidReplacementContent(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::InvalidReplacementType(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::InvalidTargetBranch(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::ManualMergeRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::MaximumConflictResolutionEntriesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::MultipleConflictResolutionEntries(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::NameLengthExceeded(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::PathRequired(ref cause) => write!(f, "{}", cause),
            MergeBranchesByThreeWayError::ReplacementContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::ReplacementTypeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergeBranchesByThreeWayError::TipsDivergenceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for MergeBranchesByThreeWayError {}
/// Errors returned by MergePullRequestByFastForward
#[derive(Debug, PartialEq)]
pub enum MergePullRequestByFastForwardError {
    /// <p>The merge cannot be completed because the target branch has been modified. Another user might have modified the target branch while the merge was in progress. Wait a few minutes, and then try again.</p>
    ConcurrentReferenceUpdate(String),
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request cannot be merged because one or more approval rules applied to the pull request have conditions that have not been met.</p>
    PullRequestApprovalRulesNotSatisfied(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>The specified reference does not exist. You must provide a full commit ID.</p>
    ReferenceDoesNotExist(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The repository does not contain any pull requests with that pull request ID. Use GetPullRequest to verify the correct repository name for the pull request ID.</p>
    RepositoryNotAssociatedWithPullRequest(String),
    /// <p>The tip of the source branch in the destination repository does not match the tip of the source branch specified in your request. The pull request might have been updated. Make sure that you have the latest changes.</p>
    TipOfSourceReferenceIsDifferent(String),
}

impl MergePullRequestByFastForwardError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<MergePullRequestByFastForwardError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentReferenceUpdateException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::ConcurrentReferenceUpdate(err.msg),
                    )
                }
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
                "PullRequestApprovalRulesNotSatisfiedException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::PullRequestApprovalRulesNotSatisfied(
                            err.msg,
                        ),
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
                "RepositoryNotAssociatedWithPullRequestException" => {
                    return RusotoError::Service(
                        MergePullRequestByFastForwardError::RepositoryNotAssociatedWithPullRequest(
                            err.msg,
                        ),
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MergePullRequestByFastForwardError::ConcurrentReferenceUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::InvalidCommitId(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::ManualMergeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::PullRequestApprovalRulesNotSatisfied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::ReferenceDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByFastForwardError::RepositoryNotAssociatedWithPullRequest(
                ref cause,
            ) => write!(f, "{}", cause),
            MergePullRequestByFastForwardError::TipOfSourceReferenceIsDifferent(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for MergePullRequestByFastForwardError {}
/// Errors returned by MergePullRequestBySquash
#[derive(Debug, PartialEq)]
pub enum MergePullRequestBySquashError {
    /// <p>The commit message is too long. Provide a shorter string. </p>
    CommitMessageLengthExceeded(String),
    /// <p>The merge cannot be completed because the target branch has been modified. Another user might have modified the target branch while the merge was in progress. Wait a few minutes, and then try again.</p>
    ConcurrentReferenceUpdate(String),
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
    /// <p>The file cannot be added because it is too large. The maximum file size is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit results in a folder whose contents exceed the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
    FolderContentSizeLimitExceeded(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution list is not valid.</p>
    InvalidConflictResolution(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The specified file mode permission is not valid. For a list of valid file mode permissions, see <a>PutFile</a>. </p>
    InvalidFileMode(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>Automerge was specified for resolving the conflict, but the replacement type is not valid or content is missing. </p>
    InvalidReplacementContent(String),
    /// <p>Automerge was specified for resolving the conflict, but the specified replacement type is not valid.</p>
    InvalidReplacementType(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The number of allowed conflict resolution entries was exceeded.</p>
    MaximumConflictResolutionEntriesExceeded(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>More than one conflict resolution entries exists for the conflict. A conflict can have only one conflict resolution entry.</p>
    MultipleConflictResolutionEntries(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
    NameLengthExceeded(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request cannot be merged because one or more approval rules applied to the pull request have conditions that have not been met.</p>
    PullRequestApprovalRulesNotSatisfied(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>USE_NEW_CONTENT was specified, but no replacement content has been provided.</p>
    ReplacementContentRequired(String),
    /// <p>A replacement type is required.</p>
    ReplacementTypeRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The repository does not contain any pull requests with that pull request ID. Use GetPullRequest to verify the correct repository name for the pull request ID.</p>
    RepositoryNotAssociatedWithPullRequest(String),
    /// <p>The tip of the source branch in the destination repository does not match the tip of the source branch specified in your request. The pull request might have been updated. Make sure that you have the latest changes.</p>
    TipOfSourceReferenceIsDifferent(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl MergePullRequestBySquashError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MergePullRequestBySquashError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::CommitMessageLengthExceeded(err.msg),
                    )
                }
                "ConcurrentReferenceUpdateException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::ConcurrentReferenceUpdate(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "FileContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::FileContentSizeLimitExceeded(err.msg),
                    )
                }
                "FolderContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::FolderContentSizeLimitExceeded(err.msg),
                    )
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(MergePullRequestBySquashError::InvalidCommitId(
                        err.msg,
                    ))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::InvalidConflictResolution(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(MergePullRequestBySquashError::InvalidEmail(
                        err.msg,
                    ))
                }
                "InvalidFileModeException" => {
                    return RusotoError::Service(MergePullRequestBySquashError::InvalidFileMode(
                        err.msg,
                    ))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(MergePullRequestBySquashError::InvalidPath(
                        err.msg,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidReplacementContentException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::InvalidReplacementContent(err.msg),
                    )
                }
                "InvalidReplacementTypeException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::InvalidReplacementType(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::InvalidRepositoryName(err.msg),
                    )
                }
                "ManualMergeRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::ManualMergeRequired(err.msg),
                    )
                }
                "MaximumConflictResolutionEntriesExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::MaximumConflictResolutionEntriesExceeded(
                            err.msg,
                        ),
                    )
                }
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "MultipleConflictResolutionEntriesException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::MultipleConflictResolutionEntries(err.msg),
                    )
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(MergePullRequestBySquashError::NameLengthExceeded(
                        err.msg,
                    ))
                }
                "PathRequiredException" => {
                    return RusotoError::Service(MergePullRequestBySquashError::PathRequired(
                        err.msg,
                    ))
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestApprovalRulesNotSatisfiedException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::PullRequestApprovalRulesNotSatisfied(
                            err.msg,
                        ),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::PullRequestIdRequired(err.msg),
                    )
                }
                "ReplacementContentRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::ReplacementContentRequired(err.msg),
                    )
                }
                "ReplacementTypeRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::ReplacementTypeRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::RepositoryNameRequired(err.msg),
                    )
                }
                "RepositoryNotAssociatedWithPullRequestException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::RepositoryNotAssociatedWithPullRequest(
                            err.msg,
                        ),
                    )
                }
                "TipOfSourceReferenceIsDifferentException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::TipOfSourceReferenceIsDifferent(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestBySquashError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MergePullRequestBySquashError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MergePullRequestBySquashError::CommitMessageLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::ConcurrentReferenceUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::FileContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::FolderContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            MergePullRequestBySquashError::InvalidConflictDetailLevel(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::InvalidConflictResolution(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            MergePullRequestBySquashError::InvalidFileMode(ref cause) => write!(f, "{}", cause),
            MergePullRequestBySquashError::InvalidPath(ref cause) => write!(f, "{}", cause),
            MergePullRequestBySquashError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::InvalidReplacementContent(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::InvalidReplacementType(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::ManualMergeRequired(ref cause) => write!(f, "{}", cause),
            MergePullRequestBySquashError::MaximumConflictResolutionEntriesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::MultipleConflictResolutionEntries(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::NameLengthExceeded(ref cause) => write!(f, "{}", cause),
            MergePullRequestBySquashError::PathRequired(ref cause) => write!(f, "{}", cause),
            MergePullRequestBySquashError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::PullRequestApprovalRulesNotSatisfied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::ReplacementContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::ReplacementTypeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::RepositoryNotAssociatedWithPullRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::TipOfSourceReferenceIsDifferent(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestBySquashError::TipsDivergenceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for MergePullRequestBySquashError {}
/// Errors returned by MergePullRequestByThreeWay
#[derive(Debug, PartialEq)]
pub enum MergePullRequestByThreeWayError {
    /// <p>The commit message is too long. Provide a shorter string. </p>
    CommitMessageLengthExceeded(String),
    /// <p>The merge cannot be completed because the target branch has been modified. Another user might have modified the target branch while the merge was in progress. Wait a few minutes, and then try again.</p>
    ConcurrentReferenceUpdate(String),
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
    /// <p>The file cannot be added because it is too large. The maximum file size is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit results in a folder whose contents exceed the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
    FolderContentSizeLimitExceeded(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The specified conflict detail level is not valid.</p>
    InvalidConflictDetailLevel(String),
    /// <p>The specified conflict resolution list is not valid.</p>
    InvalidConflictResolution(String),
    /// <p>The specified conflict resolution strategy is not valid.</p>
    InvalidConflictResolutionStrategy(String),
    /// <p>The specified email address either contains one or more characters that are not allowed, or it exceeds the maximum number of characters allowed for an email address.</p>
    InvalidEmail(String),
    /// <p>The specified file mode permission is not valid. For a list of valid file mode permissions, see <a>PutFile</a>. </p>
    InvalidFileMode(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>Automerge was specified for resolving the conflict, but the replacement type is not valid or content is missing. </p>
    InvalidReplacementContent(String),
    /// <p>Automerge was specified for resolving the conflict, but the specified replacement type is not valid.</p>
    InvalidReplacementType(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The pull request cannot be merged automatically into the destination branch. You must manually merge the branches and resolve any conflicts.</p>
    ManualMergeRequired(String),
    /// <p>The number of allowed conflict resolution entries was exceeded.</p>
    MaximumConflictResolutionEntriesExceeded(String),
    /// <p>The number of files to load exceeds the allowed limit.</p>
    MaximumFileContentToLoadExceeded(String),
    /// <p>The number of items to compare between the source or destination branches and the merge base has exceeded the maximum allowed.</p>
    MaximumItemsToCompareExceeded(String),
    /// <p>More than one conflict resolution entries exists for the conflict. A conflict can have only one conflict resolution entry.</p>
    MultipleConflictResolutionEntries(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
    NameLengthExceeded(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request cannot be merged because one or more approval rules applied to the pull request have conditions that have not been met.</p>
    PullRequestApprovalRulesNotSatisfied(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>USE_NEW_CONTENT was specified, but no replacement content has been provided.</p>
    ReplacementContentRequired(String),
    /// <p>A replacement type is required.</p>
    ReplacementTypeRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>The repository does not contain any pull requests with that pull request ID. Use GetPullRequest to verify the correct repository name for the pull request ID.</p>
    RepositoryNotAssociatedWithPullRequest(String),
    /// <p>The tip of the source branch in the destination repository does not match the tip of the source branch specified in your request. The pull request might have been updated. Make sure that you have the latest changes.</p>
    TipOfSourceReferenceIsDifferent(String),
    /// <p>The divergence between the tips of the provided commit specifiers is too great to determine whether there might be any merge conflicts. Locally compare the specifiers using <code>git diff</code> or a diff tool.</p>
    TipsDivergenceExceeded(String),
}

impl MergePullRequestByThreeWayError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<MergePullRequestByThreeWayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CommitMessageLengthExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::CommitMessageLengthExceeded(err.msg),
                    )
                }
                "ConcurrentReferenceUpdateException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::ConcurrentReferenceUpdate(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::EncryptionIntegrityChecksFailed(err.msg),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "FileContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::FileContentSizeLimitExceeded(err.msg),
                    )
                }
                "FolderContentSizeLimitExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::FolderContentSizeLimitExceeded(err.msg),
                    )
                }
                "InvalidCommitIdException" => {
                    return RusotoError::Service(MergePullRequestByThreeWayError::InvalidCommitId(
                        err.msg,
                    ))
                }
                "InvalidConflictDetailLevelException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::InvalidConflictDetailLevel(err.msg),
                    )
                }
                "InvalidConflictResolutionException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::InvalidConflictResolution(err.msg),
                    )
                }
                "InvalidConflictResolutionStrategyException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::InvalidConflictResolutionStrategy(err.msg),
                    )
                }
                "InvalidEmailException" => {
                    return RusotoError::Service(MergePullRequestByThreeWayError::InvalidEmail(
                        err.msg,
                    ))
                }
                "InvalidFileModeException" => {
                    return RusotoError::Service(MergePullRequestByThreeWayError::InvalidFileMode(
                        err.msg,
                    ))
                }
                "InvalidPathException" => {
                    return RusotoError::Service(MergePullRequestByThreeWayError::InvalidPath(
                        err.msg,
                    ))
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidReplacementContentException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::InvalidReplacementContent(err.msg),
                    )
                }
                "InvalidReplacementTypeException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::InvalidReplacementType(err.msg),
                    )
                }
                "InvalidRepositoryNameException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::InvalidRepositoryName(err.msg),
                    )
                }
                "ManualMergeRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::ManualMergeRequired(err.msg),
                    )
                }
                "MaximumConflictResolutionEntriesExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::MaximumConflictResolutionEntriesExceeded(
                            err.msg,
                        ),
                    )
                }
                "MaximumFileContentToLoadExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::MaximumFileContentToLoadExceeded(err.msg),
                    )
                }
                "MaximumItemsToCompareExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::MaximumItemsToCompareExceeded(err.msg),
                    )
                }
                "MultipleConflictResolutionEntriesException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::MultipleConflictResolutionEntries(err.msg),
                    )
                }
                "NameLengthExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::NameLengthExceeded(err.msg),
                    )
                }
                "PathRequiredException" => {
                    return RusotoError::Service(MergePullRequestByThreeWayError::PathRequired(
                        err.msg,
                    ))
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestApprovalRulesNotSatisfiedException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::PullRequestApprovalRulesNotSatisfied(
                            err.msg,
                        ),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::PullRequestIdRequired(err.msg),
                    )
                }
                "ReplacementContentRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::ReplacementContentRequired(err.msg),
                    )
                }
                "ReplacementTypeRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::ReplacementTypeRequired(err.msg),
                    )
                }
                "RepositoryDoesNotExistException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::RepositoryDoesNotExist(err.msg),
                    )
                }
                "RepositoryNameRequiredException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::RepositoryNameRequired(err.msg),
                    )
                }
                "RepositoryNotAssociatedWithPullRequestException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::RepositoryNotAssociatedWithPullRequest(
                            err.msg,
                        ),
                    )
                }
                "TipOfSourceReferenceIsDifferentException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::TipOfSourceReferenceIsDifferent(err.msg),
                    )
                }
                "TipsDivergenceExceededException" => {
                    return RusotoError::Service(
                        MergePullRequestByThreeWayError::TipsDivergenceExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MergePullRequestByThreeWayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MergePullRequestByThreeWayError::CommitMessageLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::ConcurrentReferenceUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::FileContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::FolderContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            MergePullRequestByThreeWayError::InvalidConflictDetailLevel(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::InvalidConflictResolution(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::InvalidConflictResolutionStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            MergePullRequestByThreeWayError::InvalidFileMode(ref cause) => write!(f, "{}", cause),
            MergePullRequestByThreeWayError::InvalidPath(ref cause) => write!(f, "{}", cause),
            MergePullRequestByThreeWayError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::InvalidReplacementContent(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::InvalidReplacementType(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::ManualMergeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::MaximumConflictResolutionEntriesExceeded(
                ref cause,
            ) => write!(f, "{}", cause),
            MergePullRequestByThreeWayError::MaximumFileContentToLoadExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::MaximumItemsToCompareExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::MultipleConflictResolutionEntries(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::NameLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::PathRequired(ref cause) => write!(f, "{}", cause),
            MergePullRequestByThreeWayError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::PullRequestApprovalRulesNotSatisfied(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::ReplacementContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::ReplacementTypeRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::RepositoryNotAssociatedWithPullRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::TipOfSourceReferenceIsDifferent(ref cause) => {
                write!(f, "{}", cause)
            }
            MergePullRequestByThreeWayError::TipsDivergenceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for MergePullRequestByThreeWayError {}
/// Errors returned by OverridePullRequestApprovalRules
#[derive(Debug, PartialEq)]
pub enum OverridePullRequestApprovalRulesError {
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
    /// <p>The override status is not valid. Valid statuses are OVERRIDE and REVOKE.</p>
    InvalidOverrideStatus(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The revision ID is not valid. Use GetPullRequest to determine the value.</p>
    InvalidRevisionId(String),
    /// <p>The pull request has already had its approval rules set to override.</p>
    OverrideAlreadySet(String),
    /// <p>An override status is required, but no value was provided. Valid values include OVERRIDE and REVOKE.</p>
    OverrideStatusRequired(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>A revision ID is required, but was not provided.</p>
    RevisionIdRequired(String),
    /// <p>The revision ID provided in the request does not match the current revision ID. Use GetPullRequest to retrieve the current revision ID.</p>
    RevisionNotCurrent(String),
}

impl OverridePullRequestApprovalRulesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<OverridePullRequestApprovalRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidOverrideStatusException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::InvalidOverrideStatus(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRevisionIdException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::InvalidRevisionId(err.msg),
                    )
                }
                "OverrideAlreadySetException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::OverrideAlreadySet(err.msg),
                    )
                }
                "OverrideStatusRequiredException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::OverrideStatusRequired(err.msg),
                    )
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::PullRequestIdRequired(err.msg),
                    )
                }
                "RevisionIdRequiredException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::RevisionIdRequired(err.msg),
                    )
                }
                "RevisionNotCurrentException" => {
                    return RusotoError::Service(
                        OverridePullRequestApprovalRulesError::RevisionNotCurrent(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for OverridePullRequestApprovalRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OverridePullRequestApprovalRulesError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::InvalidOverrideStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::InvalidRevisionId(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::OverrideAlreadySet(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::OverrideStatusRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::RevisionIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            OverridePullRequestApprovalRulesError::RevisionNotCurrent(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for OverridePullRequestApprovalRulesError {}
/// Errors returned by PostCommentForComparedCommit
#[derive(Debug, PartialEq)]
pub enum PostCommentForComparedCommitError {
    /// <p>The before commit ID and the after commit ID are the same, which is not valid. The before commit ID and the after commit ID must be different commit IDs.</p>
    BeforeCommitIdAndAfterCommitIdAreSame(String),
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
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
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be reused.</p>
    IdempotencyParameterMismatch(String),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestToken(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The location of the file is not valid. Make sure that you include the file name and extension.</p>
    InvalidFileLocation(String),
    /// <p>The position is not valid. Make sure that the line number exists in the version of the file you want to comment on.</p>
    InvalidFilePosition(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>Either the enum is not in a valid format, or the specified file version enum is not valid in respect to the current file version.</p>
    InvalidRelativeFileVersionEnum(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified path does not exist.</p>
    PathDoesNotExist(String),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequired(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostCommentForComparedCommitError::BeforeCommitIdAndAfterCommitIdAreSame(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::ClientRequestTokenRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::CommentContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::CommentContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::CommitDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::CommitIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::IdempotencyParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::InvalidClientRequestToken(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            PostCommentForComparedCommitError::InvalidFileLocation(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::InvalidFilePosition(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::InvalidPath(ref cause) => write!(f, "{}", cause),
            PostCommentForComparedCommitError::InvalidRelativeFileVersionEnum(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::PathDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::PathRequired(ref cause) => write!(f, "{}", cause),
            PostCommentForComparedCommitError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForComparedCommitError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PostCommentForComparedCommitError {}
/// Errors returned by PostCommentForPullRequest
#[derive(Debug, PartialEq)]
pub enum PostCommentForPullRequestError {
    /// <p>The before commit ID and the after commit ID are the same, which is not valid. The before commit ID and the after commit ID must be different commit IDs.</p>
    BeforeCommitIdAndAfterCommitIdAreSame(String),
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
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
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be reused.</p>
    IdempotencyParameterMismatch(String),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestToken(String),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitId(String),
    /// <p>The location of the file is not valid. Make sure that you include the file name and extension.</p>
    InvalidFileLocation(String),
    /// <p>The position is not valid. Make sure that the line number exists in the version of the file you want to comment on.</p>
    InvalidFilePosition(String),
    /// <p>The specified path is not valid.</p>
    InvalidPath(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>Either the enum is not in a valid format, or the specified file version enum is not valid in respect to the current file version.</p>
    InvalidRelativeFileVersionEnum(String),
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
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
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostCommentForPullRequestError::BeforeCommitIdAndAfterCommitIdAreSame(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::ClientRequestTokenRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::CommentContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::CommentContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::CommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            PostCommentForPullRequestError::CommitIdRequired(ref cause) => write!(f, "{}", cause),
            PostCommentForPullRequestError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::IdempotencyParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::InvalidClientRequestToken(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::InvalidCommitId(ref cause) => write!(f, "{}", cause),
            PostCommentForPullRequestError::InvalidFileLocation(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::InvalidFilePosition(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::InvalidPath(ref cause) => write!(f, "{}", cause),
            PostCommentForPullRequestError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::InvalidRelativeFileVersionEnum(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::PathDoesNotExist(ref cause) => write!(f, "{}", cause),
            PostCommentForPullRequestError::PathRequired(ref cause) => write!(f, "{}", cause),
            PostCommentForPullRequestError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentForPullRequestError::RepositoryNotAssociatedWithPullRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PostCommentForPullRequestError {}
/// Errors returned by PostCommentReply
#[derive(Debug, PartialEq)]
pub enum PostCommentReplyError {
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
    ClientRequestTokenRequired(String),
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequired(String),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceeded(String),
    /// <p>No comment exists with the provided ID. Verify that you have used the correct ID, and then try again.</p>
    CommentDoesNotExist(String),
    /// <p>The comment ID is missing or null. A comment ID is required.</p>
    CommentIdRequired(String),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be reused.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostCommentReplyError::ClientRequestTokenRequired(ref cause) => write!(f, "{}", cause),
            PostCommentReplyError::CommentContentRequired(ref cause) => write!(f, "{}", cause),
            PostCommentReplyError::CommentContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentReplyError::CommentDoesNotExist(ref cause) => write!(f, "{}", cause),
            PostCommentReplyError::CommentIdRequired(ref cause) => write!(f, "{}", cause),
            PostCommentReplyError::IdempotencyParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            PostCommentReplyError::InvalidClientRequestToken(ref cause) => write!(f, "{}", cause),
            PostCommentReplyError::InvalidCommentId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PostCommentReplyError {}
/// Errors returned by PutFile
#[derive(Debug, PartialEq)]
pub enum PutFileError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>The specified branch name is not valid because it is a tag name. Enter the name of a branch in the repository. For a list of valid branch names, use <a>ListBranches</a>.</p>
    BranchNameIsTagName(String),
    /// <p>A branch name is required, but was not specified.</p>
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
    /// <p>The file cannot be added because it is too large. The maximum file size is 6 MB, and the combined file content change size is 7 MB. Consider making these changes using a Git client.</p>
    FileContentSizeLimitExceeded(String),
    /// <p>A file cannot be added to the repository because the specified file name has the same name as a directory in this repository. Either provide another name for the file, or add the file in a directory that does not match the file name.</p>
    FileNameConflictsWithDirectoryName(String),
    /// <p>The commit cannot be created because a specified file path points to a submodule. Verify that the destination files have valid file paths that do not point to a submodule.</p>
    FilePathConflictsWithSubmodulePath(String),
    /// <p>The commit cannot be created because at least one of the overall changes in the commit results in a folder whose contents exceed the limit of 6 MB. Either reduce the number and size of your changes, or split the changes across multiple folders.</p>
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The user name is not valid because it has exceeded the character limit for author names. </p>
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
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutFileError::BranchDoesNotExist(ref cause) => write!(f, "{}", cause),
            PutFileError::BranchNameIsTagName(ref cause) => write!(f, "{}", cause),
            PutFileError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            PutFileError::CommitMessageLengthExceeded(ref cause) => write!(f, "{}", cause),
            PutFileError::DirectoryNameConflictsWithFileName(ref cause) => write!(f, "{}", cause),
            PutFileError::EncryptionIntegrityChecksFailed(ref cause) => write!(f, "{}", cause),
            PutFileError::EncryptionKeyAccessDenied(ref cause) => write!(f, "{}", cause),
            PutFileError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            PutFileError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            PutFileError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            PutFileError::FileContentRequired(ref cause) => write!(f, "{}", cause),
            PutFileError::FileContentSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            PutFileError::FileNameConflictsWithDirectoryName(ref cause) => write!(f, "{}", cause),
            PutFileError::FilePathConflictsWithSubmodulePath(ref cause) => write!(f, "{}", cause),
            PutFileError::FolderContentSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            PutFileError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            PutFileError::InvalidDeletionParameter(ref cause) => write!(f, "{}", cause),
            PutFileError::InvalidEmail(ref cause) => write!(f, "{}", cause),
            PutFileError::InvalidFileMode(ref cause) => write!(f, "{}", cause),
            PutFileError::InvalidParentCommitId(ref cause) => write!(f, "{}", cause),
            PutFileError::InvalidPath(ref cause) => write!(f, "{}", cause),
            PutFileError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            PutFileError::NameLengthExceeded(ref cause) => write!(f, "{}", cause),
            PutFileError::ParentCommitDoesNotExist(ref cause) => write!(f, "{}", cause),
            PutFileError::ParentCommitIdOutdated(ref cause) => write!(f, "{}", cause),
            PutFileError::ParentCommitIdRequired(ref cause) => write!(f, "{}", cause),
            PutFileError::PathRequired(ref cause) => write!(f, "{}", cause),
            PutFileError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            PutFileError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            PutFileError::SameFileContent(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutFileError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
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
    /// <p>The AWS Region for the trigger target does not match the AWS Region for the repository. Triggers must be created in the same Region as the target for the trigger.</p>
    InvalidRepositoryTriggerRegion(String),
    /// <p>The number of branches for the trigger was exceeded.</p>
    MaximumBranchesExceeded(String),
    /// <p>The number of triggers allowed for the repository was exceeded.</p>
    MaximumRepositoryTriggersExceeded(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>At least one branch name is required, but was not specified in the trigger configuration.</p>
    RepositoryTriggerBranchNameListRequired(String),
    /// <p>A destination ARN for the target service for the trigger is required, but was not specified.</p>
    RepositoryTriggerDestinationArnRequired(String),
    /// <p>At least one event for the trigger is required, but was not specified.</p>
    RepositoryTriggerEventsListRequired(String),
    /// <p>A name for the trigger is required, but was not specified.</p>
    RepositoryTriggerNameRequired(String),
    /// <p>The list of triggers for the repository is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRepositoryTriggersError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            PutRepositoryTriggersError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            PutRepositoryTriggersError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            PutRepositoryTriggersError::InvalidRepositoryTriggerBranchName(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::InvalidRepositoryTriggerCustomData(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::InvalidRepositoryTriggerEvents(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::InvalidRepositoryTriggerName(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::InvalidRepositoryTriggerRegion(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::MaximumBranchesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::MaximumRepositoryTriggersExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            PutRepositoryTriggersError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
            PutRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::RepositoryTriggerEventsListRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::RepositoryTriggerNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRepositoryTriggersError::RepositoryTriggersListRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutRepositoryTriggersError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidResourceArn(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidSystemTagUsage(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidTagsMap(ref cause) => write!(f, "{}", cause),
            TagResourceError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceArnRequired(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagPolicy(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagsMapRequired(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
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
    /// <p>The AWS Region for the trigger target does not match the AWS Region for the repository. Triggers must be created in the same Region as the target for the trigger.</p>
    InvalidRepositoryTriggerRegion(String),
    /// <p>The number of branches for the trigger was exceeded.</p>
    MaximumBranchesExceeded(String),
    /// <p>The number of triggers allowed for the repository was exceeded.</p>
    MaximumRepositoryTriggersExceeded(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequired(String),
    /// <p>At least one branch name is required, but was not specified in the trigger configuration.</p>
    RepositoryTriggerBranchNameListRequired(String),
    /// <p>A destination ARN for the target service for the trigger is required, but was not specified.</p>
    RepositoryTriggerDestinationArnRequired(String),
    /// <p>At least one event for the trigger is required, but was not specified.</p>
    RepositoryTriggerEventsListRequired(String),
    /// <p>A name for the trigger is required, but was not specified.</p>
    RepositoryTriggerNameRequired(String),
    /// <p>The list of triggers for the repository is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestRepositoryTriggersError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            TestRepositoryTriggersError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            TestRepositoryTriggersError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            TestRepositoryTriggersError::InvalidRepositoryTriggerBranchName(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::InvalidRepositoryTriggerCustomData(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::InvalidRepositoryTriggerDestinationArn(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::InvalidRepositoryTriggerEvents(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::InvalidRepositoryTriggerName(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::InvalidRepositoryTriggerRegion(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::MaximumBranchesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::MaximumRepositoryTriggersExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::RepositoryTriggerBranchNameListRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::RepositoryTriggerDestinationArnRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::RepositoryTriggerEventsListRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::RepositoryTriggerNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            TestRepositoryTriggersError::RepositoryTriggersListRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for TestRepositoryTriggersError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidResourceArn(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidSystemTagUsage(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidTagKeysList(ref cause) => write!(f, "{}", cause),
            UntagResourceError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceArnRequired(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TagKeysListRequired(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TagPolicy(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApprovalRuleTemplateContent
#[derive(Debug, PartialEq)]
pub enum UpdateApprovalRuleTemplateContentError {
    /// <p>The content for the approval rule template is empty. You must provide some content for an approval rule template. The content cannot be null.</p>
    ApprovalRuleTemplateContentRequired(String),
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
    /// <p>The content of the approval rule template is not valid.</p>
    InvalidApprovalRuleTemplateContent(String),
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
    /// <p>The SHA-256 hash signature for the rule content is not valid.</p>
    InvalidRuleContentSha256(String),
}

impl UpdateApprovalRuleTemplateContentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateApprovalRuleTemplateContentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleTemplateContentRequiredException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateContentRequired(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleTemplateDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateDoesNotExist(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleTemplateNameRequiredException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateNameRequired(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleTemplateContentException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateContentError::InvalidApprovalRuleTemplateContent(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleTemplateNameException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateContentError::InvalidApprovalRuleTemplateName(
                            err.msg,
                        ),
                    )
                }
                "InvalidRuleContentSha256Exception" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateContentError::InvalidRuleContentSha256(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApprovalRuleTemplateContentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateContentRequired(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApprovalRuleTemplateContentError::InvalidApprovalRuleTemplateContent(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateApprovalRuleTemplateContentError::InvalidApprovalRuleTemplateName(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApprovalRuleTemplateContentError::InvalidRuleContentSha256(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateApprovalRuleTemplateContentError {}
/// Errors returned by UpdateApprovalRuleTemplateDescription
#[derive(Debug, PartialEq)]
pub enum UpdateApprovalRuleTemplateDescriptionError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
    /// <p>The description for the approval rule template is not valid because it exceeds the maximum characters allowed for a description. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateDescription(String),
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
}

impl UpdateApprovalRuleTemplateDescriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateApprovalRuleTemplateDescriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "ApprovalRuleTemplateDoesNotExistException" => return RusotoError::Service(UpdateApprovalRuleTemplateDescriptionError::ApprovalRuleTemplateDoesNotExist(err.msg)),
"ApprovalRuleTemplateNameRequiredException" => return RusotoError::Service(UpdateApprovalRuleTemplateDescriptionError::ApprovalRuleTemplateNameRequired(err.msg)),
"InvalidApprovalRuleTemplateDescriptionException" => return RusotoError::Service(UpdateApprovalRuleTemplateDescriptionError::InvalidApprovalRuleTemplateDescription(err.msg)),
"InvalidApprovalRuleTemplateNameException" => return RusotoError::Service(UpdateApprovalRuleTemplateDescriptionError::InvalidApprovalRuleTemplateName(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApprovalRuleTemplateDescriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApprovalRuleTemplateDescriptionError::ApprovalRuleTemplateDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateApprovalRuleTemplateDescriptionError::ApprovalRuleTemplateNameRequired(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateApprovalRuleTemplateDescriptionError::InvalidApprovalRuleTemplateDescription(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateApprovalRuleTemplateDescriptionError::InvalidApprovalRuleTemplateName(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApprovalRuleTemplateDescriptionError {}
/// Errors returned by UpdateApprovalRuleTemplateName
#[derive(Debug, PartialEq)]
pub enum UpdateApprovalRuleTemplateNameError {
    /// <p>The specified approval rule template does not exist. Verify that the name is correct and that you are signed in to the AWS Region where the template was created, and then try again.</p>
    ApprovalRuleTemplateDoesNotExist(String),
    /// <p>You cannot create an approval rule template with that name because a template with that name already exists in this AWS Region for your AWS account. Approval rule template names must be unique.</p>
    ApprovalRuleTemplateNameAlreadyExists(String),
    /// <p>An approval rule template name is required, but was not specified.</p>
    ApprovalRuleTemplateNameRequired(String),
    /// <p>The name of the approval rule template is not valid. Template names must be between 1 and 100 valid characters in length. For more information about limits in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">AWS CodeCommit User Guide</a>.</p>
    InvalidApprovalRuleTemplateName(String),
}

impl UpdateApprovalRuleTemplateNameError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateApprovalRuleTemplateNameError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleTemplateDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateNameError::ApprovalRuleTemplateDoesNotExist(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleTemplateNameAlreadyExistsException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateNameError::ApprovalRuleTemplateNameAlreadyExists(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleTemplateNameRequiredException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateNameError::ApprovalRuleTemplateNameRequired(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleTemplateNameException" => {
                    return RusotoError::Service(
                        UpdateApprovalRuleTemplateNameError::InvalidApprovalRuleTemplateName(
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
impl fmt::Display for UpdateApprovalRuleTemplateNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApprovalRuleTemplateNameError::ApprovalRuleTemplateDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApprovalRuleTemplateNameError::ApprovalRuleTemplateNameAlreadyExists(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateApprovalRuleTemplateNameError::ApprovalRuleTemplateNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApprovalRuleTemplateNameError::InvalidApprovalRuleTemplateName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateApprovalRuleTemplateNameError {}
/// Errors returned by UpdateComment
#[derive(Debug, PartialEq)]
pub enum UpdateCommentError {
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequired(String),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceeded(String),
    /// <p>This comment has already been deleted. You cannot edit or delete a deleted comment.</p>
    CommentDeleted(String),
    /// <p>No comment exists with the provided ID. Verify that you have used the correct ID, and then try again.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCommentError::CommentContentRequired(ref cause) => write!(f, "{}", cause),
            UpdateCommentError::CommentContentSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCommentError::CommentDeleted(ref cause) => write!(f, "{}", cause),
            UpdateCommentError::CommentDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateCommentError::CommentIdRequired(ref cause) => write!(f, "{}", cause),
            UpdateCommentError::CommentNotCreatedByCaller(ref cause) => write!(f, "{}", cause),
            UpdateCommentError::InvalidCommentId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCommentError {}
/// Errors returned by UpdateDefaultBranch
#[derive(Debug, PartialEq)]
pub enum UpdateDefaultBranchError {
    /// <p>The specified branch does not exist.</p>
    BranchDoesNotExist(String),
    /// <p>A branch name is required, but was not specified.</p>
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDefaultBranchError::BranchDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::BranchNameRequired(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDefaultBranchError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDefaultBranchError::EncryptionKeyDisabled(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::EncryptionKeyNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::EncryptionKeyUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::InvalidBranchName(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateDefaultBranchError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDefaultBranchError {}
/// Errors returned by UpdatePullRequestApprovalRuleContent
#[derive(Debug, PartialEq)]
pub enum UpdatePullRequestApprovalRuleContentError {
    /// <p>The content for the approval rule is empty. You must provide some content for an approval rule. The content cannot be null.</p>
    ApprovalRuleContentRequired(String),
    /// <p>The specified approval rule does not exist.</p>
    ApprovalRuleDoesNotExist(String),
    /// <p>An approval rule name is required, but was not specified.</p>
    ApprovalRuleNameRequired(String),
    /// <p>The approval rule cannot be modified for the pull request because it was created by an approval rule template and applied to the pull request automatically.</p>
    CannotModifyApprovalRuleFromTemplate(String),
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
    /// <p>The content for the approval rule is not valid.</p>
    InvalidApprovalRuleContent(String),
    /// <p>The name for the approval rule is not valid.</p>
    InvalidApprovalRuleName(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The SHA-256 hash signature for the rule content is not valid.</p>
    InvalidRuleContentSha256(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
}

impl UpdatePullRequestApprovalRuleContentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdatePullRequestApprovalRuleContentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalRuleContentRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::ApprovalRuleContentRequired(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::ApprovalRuleDoesNotExist(
                            err.msg,
                        ),
                    )
                }
                "ApprovalRuleNameRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::ApprovalRuleNameRequired(
                            err.msg,
                        ),
                    )
                }
                "CannotModifyApprovalRuleFromTemplateException" => return RusotoError::Service(
                    UpdatePullRequestApprovalRuleContentError::CannotModifyApprovalRuleFromTemplate(
                        err.msg,
                    ),
                ),
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::EncryptionKeyAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::EncryptionKeyUnavailable(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleContentException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::InvalidApprovalRuleContent(
                            err.msg,
                        ),
                    )
                }
                "InvalidApprovalRuleNameException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::InvalidApprovalRuleName(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRuleContentSha256Exception" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::InvalidRuleContentSha256(
                            err.msg,
                        ),
                    )
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::PullRequestAlreadyClosed(
                            err.msg,
                        ),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalRuleContentError::PullRequestIdRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePullRequestApprovalRuleContentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePullRequestApprovalRuleContentError::ApprovalRuleContentRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::ApprovalRuleDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::ApprovalRuleNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::CannotModifyApprovalRuleFromTemplate(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdatePullRequestApprovalRuleContentError::EncryptionIntegrityChecksFailed(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdatePullRequestApprovalRuleContentError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::InvalidApprovalRuleContent(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::InvalidApprovalRuleName(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::InvalidRuleContentSha256(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalRuleContentError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdatePullRequestApprovalRuleContentError {}
/// Errors returned by UpdatePullRequestApprovalState
#[derive(Debug, PartialEq)]
pub enum UpdatePullRequestApprovalStateError {
    /// <p>An approval state is required, but was not specified.</p>
    ApprovalStateRequired(String),
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
    /// <p>The state for the approval is not valid. Valid values include APPROVE and REVOKE. </p>
    InvalidApprovalState(String),
    /// <p>The pull request ID is not valid. Make sure that you have provided the full ID and that the pull request is in the specified repository, and then try again.</p>
    InvalidPullRequestId(String),
    /// <p>The revision ID is not valid. Use GetPullRequest to determine the value.</p>
    InvalidRevisionId(String),
    /// <p>The number of approvals required for the approval rule exceeds the maximum number allowed.</p>
    MaximumNumberOfApprovalsExceeded(String),
    /// <p>The pull request status cannot be updated because it is already closed.</p>
    PullRequestAlreadyClosed(String),
    /// <p>The approval cannot be applied because the user approving the pull request matches the user who created the pull request. You cannot approve a pull request that you created.</p>
    PullRequestCannotBeApprovedByAuthor(String),
    /// <p>The pull request ID could not be found. Make sure that you have specified the correct repository name and pull request ID, and then try again.</p>
    PullRequestDoesNotExist(String),
    /// <p>A pull request ID is required, but none was provided.</p>
    PullRequestIdRequired(String),
    /// <p>A revision ID is required, but was not provided.</p>
    RevisionIdRequired(String),
    /// <p>The revision ID provided in the request does not match the current revision ID. Use GetPullRequest to retrieve the current revision ID.</p>
    RevisionNotCurrent(String),
}

impl UpdatePullRequestApprovalStateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdatePullRequestApprovalStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApprovalStateRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::ApprovalStateRequired(err.msg),
                    )
                }
                "EncryptionIntegrityChecksFailedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::EncryptionIntegrityChecksFailed(
                            err.msg,
                        ),
                    )
                }
                "EncryptionKeyAccessDeniedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::EncryptionKeyAccessDenied(err.msg),
                    )
                }
                "EncryptionKeyDisabledException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::EncryptionKeyDisabled(err.msg),
                    )
                }
                "EncryptionKeyNotFoundException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::EncryptionKeyNotFound(err.msg),
                    )
                }
                "EncryptionKeyUnavailableException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::EncryptionKeyUnavailable(err.msg),
                    )
                }
                "InvalidApprovalStateException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::InvalidApprovalState(err.msg),
                    )
                }
                "InvalidPullRequestIdException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::InvalidPullRequestId(err.msg),
                    )
                }
                "InvalidRevisionIdException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::InvalidRevisionId(err.msg),
                    )
                }
                "MaximumNumberOfApprovalsExceededException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::MaximumNumberOfApprovalsExceeded(
                            err.msg,
                        ),
                    )
                }
                "PullRequestAlreadyClosedException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::PullRequestAlreadyClosed(err.msg),
                    )
                }
                "PullRequestCannotBeApprovedByAuthorException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::PullRequestCannotBeApprovedByAuthor(
                            err.msg,
                        ),
                    )
                }
                "PullRequestDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::PullRequestDoesNotExist(err.msg),
                    )
                }
                "PullRequestIdRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::PullRequestIdRequired(err.msg),
                    )
                }
                "RevisionIdRequiredException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::RevisionIdRequired(err.msg),
                    )
                }
                "RevisionNotCurrentException" => {
                    return RusotoError::Service(
                        UpdatePullRequestApprovalStateError::RevisionNotCurrent(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePullRequestApprovalStateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePullRequestApprovalStateError::ApprovalStateRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::InvalidApprovalState(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::InvalidRevisionId(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::MaximumNumberOfApprovalsExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::PullRequestCannotBeApprovedByAuthor(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::RevisionIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestApprovalStateError::RevisionNotCurrent(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdatePullRequestApprovalStateError {}
/// Errors returned by UpdatePullRequestDescription
#[derive(Debug, PartialEq)]
pub enum UpdatePullRequestDescriptionError {
    /// <p>The pull request description is not valid. Descriptions cannot be more than 1,000 characters.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePullRequestDescriptionError::InvalidDescription(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestDescriptionError::InvalidPullRequestId(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestDescriptionError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestDescriptionError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestDescriptionError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdatePullRequestDescriptionError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePullRequestStatusError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::InvalidPullRequestId(ref cause) => write!(f, "{}", cause),
            UpdatePullRequestStatusError::InvalidPullRequestStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::InvalidPullRequestStatusUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::PullRequestIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestStatusError::PullRequestStatusRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdatePullRequestStatusError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePullRequestTitleError::InvalidPullRequestId(ref cause) => write!(f, "{}", cause),
            UpdatePullRequestTitleError::InvalidTitle(ref cause) => write!(f, "{}", cause),
            UpdatePullRequestTitleError::PullRequestAlreadyClosed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestTitleError::PullRequestDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePullRequestTitleError::PullRequestIdRequired(ref cause) => write!(f, "{}", cause),
            UpdatePullRequestTitleError::TitleRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePullRequestTitleError {}
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
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRepositoryDescriptionError::EncryptionIntegrityChecksFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::EncryptionKeyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::EncryptionKeyDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::EncryptionKeyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::EncryptionKeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::InvalidRepositoryDescription(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::InvalidRepositoryName(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::RepositoryDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRepositoryDescriptionError::RepositoryNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateRepositoryDescriptionError {}
/// Errors returned by UpdateRepositoryName
#[derive(Debug, PartialEq)]
pub enum UpdateRepositoryNameError {
    /// <p><p>A specified repository name is not valid.</p> <note> <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p> </note></p>
    InvalidRepositoryName(String),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExist(String),
    /// <p>The specified repository name already exists.</p>
    RepositoryNameExists(String),
    /// <p>A repository name is required, but was not specified.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRepositoryNameError::InvalidRepositoryName(ref cause) => write!(f, "{}", cause),
            UpdateRepositoryNameError::RepositoryDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateRepositoryNameError::RepositoryNameExists(ref cause) => write!(f, "{}", cause),
            UpdateRepositoryNameError::RepositoryNameRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRepositoryNameError {}
/// Trait representing the capabilities of the CodeCommit API. CodeCommit clients implement this trait.
#[async_trait]
pub trait CodeCommit {
    /// <p>Creates an association between an approval rule template and a specified repository. Then, the next time a pull request is created in the repository where the destination reference (if specified) matches the destination reference (branch) for the pull request, an approval rule that matches the template conditions is automatically created for that pull request. If no destination references are specified in the template, an approval rule that matches the template contents is created for all pull requests in that repository.</p>
    async fn associate_approval_rule_template_with_repository(
        &self,
        input: AssociateApprovalRuleTemplateWithRepositoryInput,
    ) -> Result<(), RusotoError<AssociateApprovalRuleTemplateWithRepositoryError>>;

    /// <p>Creates an association between an approval rule template and one or more specified repositories. </p>
    async fn batch_associate_approval_rule_template_with_repositories(
        &self,
        input: BatchAssociateApprovalRuleTemplateWithRepositoriesInput,
    ) -> Result<
        BatchAssociateApprovalRuleTemplateWithRepositoriesOutput,
        RusotoError<BatchAssociateApprovalRuleTemplateWithRepositoriesError>,
    >;

    /// <p>Returns information about one or more merge conflicts in the attempted merge of two commit specifiers using the squash or three-way merge strategy.</p>
    async fn batch_describe_merge_conflicts(
        &self,
        input: BatchDescribeMergeConflictsInput,
    ) -> Result<BatchDescribeMergeConflictsOutput, RusotoError<BatchDescribeMergeConflictsError>>;

    /// <p>Removes the association between an approval rule template and one or more specified repositories. </p>
    async fn batch_disassociate_approval_rule_template_from_repositories(
        &self,
        input: BatchDisassociateApprovalRuleTemplateFromRepositoriesInput,
    ) -> Result<
        BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput,
        RusotoError<BatchDisassociateApprovalRuleTemplateFromRepositoriesError>,
    >;

    /// <p>Returns information about the contents of one or more commits in a repository.</p>
    async fn batch_get_commits(
        &self,
        input: BatchGetCommitsInput,
    ) -> Result<BatchGetCommitsOutput, RusotoError<BatchGetCommitsError>>;

    /// <p><p>Returns information about one or more repositories.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p> </note></p>
    async fn batch_get_repositories(
        &self,
        input: BatchGetRepositoriesInput,
    ) -> Result<BatchGetRepositoriesOutput, RusotoError<BatchGetRepositoriesError>>;

    /// <p>Creates a template for approval rules that can then be associated with one or more repositories in your AWS account. When you associate a template with a repository, AWS CodeCommit creates an approval rule that matches the conditions of the template for all pull requests that meet the conditions of the template. For more information, see <a>AssociateApprovalRuleTemplateWithRepository</a>.</p>
    async fn create_approval_rule_template(
        &self,
        input: CreateApprovalRuleTemplateInput,
    ) -> Result<CreateApprovalRuleTemplateOutput, RusotoError<CreateApprovalRuleTemplateError>>;

    /// <p><p>Creates a branch in a repository and points the branch to a commit.</p> <note> <p>Calling the create branch operation does not set a repository&#39;s default branch. To do this, call the update default branch operation.</p> </note></p>
    async fn create_branch(
        &self,
        input: CreateBranchInput,
    ) -> Result<(), RusotoError<CreateBranchError>>;

    /// <p>Creates a commit for a repository on the tip of a specified branch.</p>
    async fn create_commit(
        &self,
        input: CreateCommitInput,
    ) -> Result<CreateCommitOutput, RusotoError<CreateCommitError>>;

    /// <p>Creates a pull request in the specified repository.</p>
    async fn create_pull_request(
        &self,
        input: CreatePullRequestInput,
    ) -> Result<CreatePullRequestOutput, RusotoError<CreatePullRequestError>>;

    /// <p>Creates an approval rule for a pull request.</p>
    async fn create_pull_request_approval_rule(
        &self,
        input: CreatePullRequestApprovalRuleInput,
    ) -> Result<CreatePullRequestApprovalRuleOutput, RusotoError<CreatePullRequestApprovalRuleError>>;

    /// <p>Creates a new, empty repository.</p>
    async fn create_repository(
        &self,
        input: CreateRepositoryInput,
    ) -> Result<CreateRepositoryOutput, RusotoError<CreateRepositoryError>>;

    /// <p><p>Creates an unreferenced commit that represents the result of merging two branches using a specified merge strategy. This can help you determine the outcome of a potential merge. This API cannot be used with the fast-forward merge strategy because that strategy does not create a merge commit.</p> <note> <p>This unreferenced merge commit can only be accessed using the GetCommit API or through git commands such as git fetch. To retrieve this commit, you must specify its commit ID or otherwise reference it.</p> </note></p>
    async fn create_unreferenced_merge_commit(
        &self,
        input: CreateUnreferencedMergeCommitInput,
    ) -> Result<CreateUnreferencedMergeCommitOutput, RusotoError<CreateUnreferencedMergeCommitError>>;

    /// <p>Deletes a specified approval rule template. Deleting a template does not remove approval rules on pull requests already created with the template.</p>
    async fn delete_approval_rule_template(
        &self,
        input: DeleteApprovalRuleTemplateInput,
    ) -> Result<DeleteApprovalRuleTemplateOutput, RusotoError<DeleteApprovalRuleTemplateError>>;

    /// <p>Deletes a branch from a repository, unless that branch is the default branch for the repository. </p>
    async fn delete_branch(
        &self,
        input: DeleteBranchInput,
    ) -> Result<DeleteBranchOutput, RusotoError<DeleteBranchError>>;

    /// <p>Deletes the content of a comment made on a change, file, or commit in a repository.</p>
    async fn delete_comment_content(
        &self,
        input: DeleteCommentContentInput,
    ) -> Result<DeleteCommentContentOutput, RusotoError<DeleteCommentContentError>>;

    /// <p>Deletes a specified file from a specified branch. A commit is created on the branch that contains the revision. The file still exists in the commits earlier to the commit that contains the deletion.</p>
    async fn delete_file(
        &self,
        input: DeleteFileInput,
    ) -> Result<DeleteFileOutput, RusotoError<DeleteFileError>>;

    /// <p>Deletes an approval rule from a specified pull request. Approval rules can be deleted from a pull request only if the pull request is open, and if the approval rule was created specifically for a pull request and not generated from an approval rule template associated with the repository where the pull request was created. You cannot delete an approval rule from a merged or closed pull request.</p>
    async fn delete_pull_request_approval_rule(
        &self,
        input: DeletePullRequestApprovalRuleInput,
    ) -> Result<DeletePullRequestApprovalRuleOutput, RusotoError<DeletePullRequestApprovalRuleError>>;

    /// <p><p>Deletes a repository. If a specified repository was already deleted, a null repository ID is returned.</p> <important> <p>Deleting a repository also deletes all associated objects and metadata. After a repository is deleted, all future push calls to the deleted repository fail.</p> </important></p>
    async fn delete_repository(
        &self,
        input: DeleteRepositoryInput,
    ) -> Result<DeleteRepositoryOutput, RusotoError<DeleteRepositoryError>>;

    /// <p>Returns information about one or more merge conflicts in the attempted merge of two commit specifiers using the squash or three-way merge strategy. If the merge option for the attempted merge is specified as FAST_FORWARD_MERGE, an exception is thrown.</p>
    async fn describe_merge_conflicts(
        &self,
        input: DescribeMergeConflictsInput,
    ) -> Result<DescribeMergeConflictsOutput, RusotoError<DescribeMergeConflictsError>>;

    /// <p>Returns information about one or more pull request events.</p>
    async fn describe_pull_request_events(
        &self,
        input: DescribePullRequestEventsInput,
    ) -> Result<DescribePullRequestEventsOutput, RusotoError<DescribePullRequestEventsError>>;

    /// <p>Removes the association between a template and a repository so that approval rules based on the template are not automatically created when pull requests are created in the specified repository. This does not delete any approval rules previously created for pull requests through the template association.</p>
    async fn disassociate_approval_rule_template_from_repository(
        &self,
        input: DisassociateApprovalRuleTemplateFromRepositoryInput,
    ) -> Result<(), RusotoError<DisassociateApprovalRuleTemplateFromRepositoryError>>;

    /// <p>Evaluates whether a pull request has met all the conditions specified in its associated approval rules.</p>
    async fn evaluate_pull_request_approval_rules(
        &self,
        input: EvaluatePullRequestApprovalRulesInput,
    ) -> Result<
        EvaluatePullRequestApprovalRulesOutput,
        RusotoError<EvaluatePullRequestApprovalRulesError>,
    >;

    /// <p>Returns information about a specified approval rule template.</p>
    async fn get_approval_rule_template(
        &self,
        input: GetApprovalRuleTemplateInput,
    ) -> Result<GetApprovalRuleTemplateOutput, RusotoError<GetApprovalRuleTemplateError>>;

    /// <p>Returns the base-64 encoded content of an individual blob in a repository.</p>
    async fn get_blob(
        &self,
        input: GetBlobInput,
    ) -> Result<GetBlobOutput, RusotoError<GetBlobError>>;

    /// <p>Returns information about a repository branch, including its name and the last commit ID.</p>
    async fn get_branch(
        &self,
        input: GetBranchInput,
    ) -> Result<GetBranchOutput, RusotoError<GetBranchError>>;

    /// <p>Returns the content of a comment made on a change, file, or commit in a repository.</p>
    async fn get_comment(
        &self,
        input: GetCommentInput,
    ) -> Result<GetCommentOutput, RusotoError<GetCommentError>>;

    /// <p>Returns information about comments made on the comparison between two commits.</p>
    async fn get_comments_for_compared_commit(
        &self,
        input: GetCommentsForComparedCommitInput,
    ) -> Result<GetCommentsForComparedCommitOutput, RusotoError<GetCommentsForComparedCommitError>>;

    /// <p>Returns comments made on a pull request.</p>
    async fn get_comments_for_pull_request(
        &self,
        input: GetCommentsForPullRequestInput,
    ) -> Result<GetCommentsForPullRequestOutput, RusotoError<GetCommentsForPullRequestError>>;

    /// <p>Returns information about a commit, including commit message and committer information.</p>
    async fn get_commit(
        &self,
        input: GetCommitInput,
    ) -> Result<GetCommitOutput, RusotoError<GetCommitError>>;

    /// <p>Returns information about the differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID, or other fully qualified reference). Results can be limited to a specified path.</p>
    async fn get_differences(
        &self,
        input: GetDifferencesInput,
    ) -> Result<GetDifferencesOutput, RusotoError<GetDifferencesError>>;

    /// <p>Returns the base-64 encoded contents of a specified file and its metadata.</p>
    async fn get_file(
        &self,
        input: GetFileInput,
    ) -> Result<GetFileOutput, RusotoError<GetFileError>>;

    /// <p>Returns the contents of a specified folder in a repository.</p>
    async fn get_folder(
        &self,
        input: GetFolderInput,
    ) -> Result<GetFolderOutput, RusotoError<GetFolderError>>;

    /// <p>Returns information about a specified merge commit.</p>
    async fn get_merge_commit(
        &self,
        input: GetMergeCommitInput,
    ) -> Result<GetMergeCommitOutput, RusotoError<GetMergeCommitError>>;

    /// <p>Returns information about merge conflicts between the before and after commit IDs for a pull request in a repository.</p>
    async fn get_merge_conflicts(
        &self,
        input: GetMergeConflictsInput,
    ) -> Result<GetMergeConflictsOutput, RusotoError<GetMergeConflictsError>>;

    /// <p>Returns information about the merge options available for merging two specified branches. For details about why a merge option is not available, use GetMergeConflicts or DescribeMergeConflicts.</p>
    async fn get_merge_options(
        &self,
        input: GetMergeOptionsInput,
    ) -> Result<GetMergeOptionsOutput, RusotoError<GetMergeOptionsError>>;

    /// <p>Gets information about a pull request in a specified repository.</p>
    async fn get_pull_request(
        &self,
        input: GetPullRequestInput,
    ) -> Result<GetPullRequestOutput, RusotoError<GetPullRequestError>>;

    /// <p>Gets information about the approval states for a specified pull request. Approval states only apply to pull requests that have one or more approval rules applied to them.</p>
    async fn get_pull_request_approval_states(
        &self,
        input: GetPullRequestApprovalStatesInput,
    ) -> Result<GetPullRequestApprovalStatesOutput, RusotoError<GetPullRequestApprovalStatesError>>;

    /// <p>Returns information about whether approval rules have been set aside (overridden) for a pull request, and if so, the Amazon Resource Name (ARN) of the user or identity that overrode the rules and their requirements for the pull request.</p>
    async fn get_pull_request_override_state(
        &self,
        input: GetPullRequestOverrideStateInput,
    ) -> Result<GetPullRequestOverrideStateOutput, RusotoError<GetPullRequestOverrideStateError>>;

    /// <p><p>Returns information about a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p> </note></p>
    async fn get_repository(
        &self,
        input: GetRepositoryInput,
    ) -> Result<GetRepositoryOutput, RusotoError<GetRepositoryError>>;

    /// <p>Gets information about triggers configured for a repository.</p>
    async fn get_repository_triggers(
        &self,
        input: GetRepositoryTriggersInput,
    ) -> Result<GetRepositoryTriggersOutput, RusotoError<GetRepositoryTriggersError>>;

    /// <p>Lists all approval rule templates in the specified AWS Region in your AWS account. If an AWS Region is not specified, the AWS Region where you are signed in is used.</p>
    async fn list_approval_rule_templates(
        &self,
        input: ListApprovalRuleTemplatesInput,
    ) -> Result<ListApprovalRuleTemplatesOutput, RusotoError<ListApprovalRuleTemplatesError>>;

    /// <p>Lists all approval rule templates that are associated with a specified repository.</p>
    async fn list_associated_approval_rule_templates_for_repository(
        &self,
        input: ListAssociatedApprovalRuleTemplatesForRepositoryInput,
    ) -> Result<
        ListAssociatedApprovalRuleTemplatesForRepositoryOutput,
        RusotoError<ListAssociatedApprovalRuleTemplatesForRepositoryError>,
    >;

    /// <p>Gets information about one or more branches in a repository.</p>
    async fn list_branches(
        &self,
        input: ListBranchesInput,
    ) -> Result<ListBranchesOutput, RusotoError<ListBranchesError>>;

    /// <p>Returns a list of pull requests for a specified repository. The return list can be refined by pull request status or pull request author ARN.</p>
    async fn list_pull_requests(
        &self,
        input: ListPullRequestsInput,
    ) -> Result<ListPullRequestsOutput, RusotoError<ListPullRequestsError>>;

    /// <p>Gets information about one or more repositories.</p>
    async fn list_repositories(
        &self,
        input: ListRepositoriesInput,
    ) -> Result<ListRepositoriesOutput, RusotoError<ListRepositoriesError>>;

    /// <p>Lists all repositories associated with the specified approval rule template.</p>
    async fn list_repositories_for_approval_rule_template(
        &self,
        input: ListRepositoriesForApprovalRuleTemplateInput,
    ) -> Result<
        ListRepositoriesForApprovalRuleTemplateOutput,
        RusotoError<ListRepositoriesForApprovalRuleTemplateError>,
    >;

    /// <p>Gets information about AWS tags for a specified Amazon Resource Name (ARN) in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the<i> AWS CodeCommit User Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Merges two branches using the fast-forward merge strategy.</p>
    async fn merge_branches_by_fast_forward(
        &self,
        input: MergeBranchesByFastForwardInput,
    ) -> Result<MergeBranchesByFastForwardOutput, RusotoError<MergeBranchesByFastForwardError>>;

    /// <p>Merges two branches using the squash merge strategy.</p>
    async fn merge_branches_by_squash(
        &self,
        input: MergeBranchesBySquashInput,
    ) -> Result<MergeBranchesBySquashOutput, RusotoError<MergeBranchesBySquashError>>;

    /// <p>Merges two specified branches using the three-way merge strategy.</p>
    async fn merge_branches_by_three_way(
        &self,
        input: MergeBranchesByThreeWayInput,
    ) -> Result<MergeBranchesByThreeWayOutput, RusotoError<MergeBranchesByThreeWayError>>;

    /// <p>Attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the fast-forward merge strategy. If the merge is successful, it closes the pull request.</p>
    async fn merge_pull_request_by_fast_forward(
        &self,
        input: MergePullRequestByFastForwardInput,
    ) -> Result<MergePullRequestByFastForwardOutput, RusotoError<MergePullRequestByFastForwardError>>;

    /// <p>Attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the squash merge strategy. If the merge is successful, it closes the pull request.</p>
    async fn merge_pull_request_by_squash(
        &self,
        input: MergePullRequestBySquashInput,
    ) -> Result<MergePullRequestBySquashOutput, RusotoError<MergePullRequestBySquashError>>;

    /// <p>Attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the three-way merge strategy. If the merge is successful, it closes the pull request.</p>
    async fn merge_pull_request_by_three_way(
        &self,
        input: MergePullRequestByThreeWayInput,
    ) -> Result<MergePullRequestByThreeWayOutput, RusotoError<MergePullRequestByThreeWayError>>;

    /// <p>Sets aside (overrides) all approval rule requirements for a specified pull request.</p>
    async fn override_pull_request_approval_rules(
        &self,
        input: OverridePullRequestApprovalRulesInput,
    ) -> Result<(), RusotoError<OverridePullRequestApprovalRulesError>>;

    /// <p>Posts a comment on the comparison between two commits.</p>
    async fn post_comment_for_compared_commit(
        &self,
        input: PostCommentForComparedCommitInput,
    ) -> Result<PostCommentForComparedCommitOutput, RusotoError<PostCommentForComparedCommitError>>;

    /// <p>Posts a comment on a pull request.</p>
    async fn post_comment_for_pull_request(
        &self,
        input: PostCommentForPullRequestInput,
    ) -> Result<PostCommentForPullRequestOutput, RusotoError<PostCommentForPullRequestError>>;

    /// <p>Posts a comment in reply to an existing comment on a comparison between commits or a pull request.</p>
    async fn post_comment_reply(
        &self,
        input: PostCommentReplyInput,
    ) -> Result<PostCommentReplyOutput, RusotoError<PostCommentReplyError>>;

    /// <p>Adds or updates a file in a branch in an AWS CodeCommit repository, and generates a commit for the addition in the specified branch.</p>
    async fn put_file(
        &self,
        input: PutFileInput,
    ) -> Result<PutFileOutput, RusotoError<PutFileError>>;

    /// <p>Replaces all triggers for a repository. Used to create or delete triggers.</p>
    async fn put_repository_triggers(
        &self,
        input: PutRepositoryTriggersInput,
    ) -> Result<PutRepositoryTriggersOutput, RusotoError<PutRepositoryTriggersError>>;

    /// <p>Adds or updates tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the <i>AWS CodeCommit User Guide</i>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test sends data from the last commit. If no data is available, sample data is generated.</p>
    async fn test_repository_triggers(
        &self,
        input: TestRepositoryTriggersInput,
    ) -> Result<TestRepositoryTriggersOutput, RusotoError<TestRepositoryTriggersError>>;

    /// <p>Removes tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the <i>AWS CodeCommit User Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates the content of an approval rule template. You can change the number of required approvals, the membership of the approval rule, and whether an approval pool is defined.</p>
    async fn update_approval_rule_template_content(
        &self,
        input: UpdateApprovalRuleTemplateContentInput,
    ) -> Result<
        UpdateApprovalRuleTemplateContentOutput,
        RusotoError<UpdateApprovalRuleTemplateContentError>,
    >;

    /// <p>Updates the description for a specified approval rule template.</p>
    async fn update_approval_rule_template_description(
        &self,
        input: UpdateApprovalRuleTemplateDescriptionInput,
    ) -> Result<
        UpdateApprovalRuleTemplateDescriptionOutput,
        RusotoError<UpdateApprovalRuleTemplateDescriptionError>,
    >;

    /// <p>Updates the name of a specified approval rule template.</p>
    async fn update_approval_rule_template_name(
        &self,
        input: UpdateApprovalRuleTemplateNameInput,
    ) -> Result<
        UpdateApprovalRuleTemplateNameOutput,
        RusotoError<UpdateApprovalRuleTemplateNameError>,
    >;

    /// <p>Replaces the contents of a comment.</p>
    async fn update_comment(
        &self,
        input: UpdateCommentInput,
    ) -> Result<UpdateCommentOutput, RusotoError<UpdateCommentError>>;

    /// <p><p>Sets or changes the default branch name for the specified repository.</p> <note> <p>If you use this operation to change the default branch name to the current default branch name, a success message is returned even though the default branch did not change.</p> </note></p>
    async fn update_default_branch(
        &self,
        input: UpdateDefaultBranchInput,
    ) -> Result<(), RusotoError<UpdateDefaultBranchError>>;

    /// <p>Updates the structure of an approval rule created specifically for a pull request. For example, you can change the number of required approvers and the approval pool for approvers. </p>
    async fn update_pull_request_approval_rule_content(
        &self,
        input: UpdatePullRequestApprovalRuleContentInput,
    ) -> Result<
        UpdatePullRequestApprovalRuleContentOutput,
        RusotoError<UpdatePullRequestApprovalRuleContentError>,
    >;

    /// <p>Updates the state of a user's approval on a pull request. The user is derived from the signed-in account when the request is made.</p>
    async fn update_pull_request_approval_state(
        &self,
        input: UpdatePullRequestApprovalStateInput,
    ) -> Result<(), RusotoError<UpdatePullRequestApprovalStateError>>;

    /// <p>Replaces the contents of the description of a pull request.</p>
    async fn update_pull_request_description(
        &self,
        input: UpdatePullRequestDescriptionInput,
    ) -> Result<UpdatePullRequestDescriptionOutput, RusotoError<UpdatePullRequestDescriptionError>>;

    /// <p>Updates the status of a pull request. </p>
    async fn update_pull_request_status(
        &self,
        input: UpdatePullRequestStatusInput,
    ) -> Result<UpdatePullRequestStatusOutput, RusotoError<UpdatePullRequestStatusError>>;

    /// <p>Replaces the title of a pull request.</p>
    async fn update_pull_request_title(
        &self,
        input: UpdatePullRequestTitleInput,
    ) -> Result<UpdatePullRequestTitleOutput, RusotoError<UpdatePullRequestTitleError>>;

    /// <p><p>Sets or changes the comment or description for a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p> </note></p>
    async fn update_repository_description(
        &self,
        input: UpdateRepositoryDescriptionInput,
    ) -> Result<(), RusotoError<UpdateRepositoryDescriptionError>>;

    /// <p>Renames a repository. The repository name must be unique across the calling AWS account. Repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix .git is prohibited. For more information about the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide.</p>
    async fn update_repository_name(
        &self,
        input: UpdateRepositoryNameInput,
    ) -> Result<(), RusotoError<UpdateRepositoryNameError>>;
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

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeCommitClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeCommitClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeCommitClient {
        CodeCommitClient { client, region }
    }
}

#[async_trait]
impl CodeCommit for CodeCommitClient {
    /// <p>Creates an association between an approval rule template and a specified repository. Then, the next time a pull request is created in the repository where the destination reference (if specified) matches the destination reference (branch) for the pull request, an approval rule that matches the template conditions is automatically created for that pull request. If no destination references are specified in the template, an approval rule that matches the template contents is created for all pull requests in that repository.</p>
    async fn associate_approval_rule_template_with_repository(
        &self,
        input: AssociateApprovalRuleTemplateWithRepositoryInput,
    ) -> Result<(), RusotoError<AssociateApprovalRuleTemplateWithRepositoryError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.AssociateApprovalRuleTemplateWithRepository",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateApprovalRuleTemplateWithRepositoryError::from_response(response))
        }
    }

    /// <p>Creates an association between an approval rule template and one or more specified repositories. </p>
    async fn batch_associate_approval_rule_template_with_repositories(
        &self,
        input: BatchAssociateApprovalRuleTemplateWithRepositoriesInput,
    ) -> Result<
        BatchAssociateApprovalRuleTemplateWithRepositoriesOutput,
        RusotoError<BatchAssociateApprovalRuleTemplateWithRepositoriesError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.BatchAssociateApprovalRuleTemplateWithRepositories",
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
                .deserialize::<BatchAssociateApprovalRuleTemplateWithRepositoriesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchAssociateApprovalRuleTemplateWithRepositoriesError::from_response(response))
        }
    }

    /// <p>Returns information about one or more merge conflicts in the attempted merge of two commit specifiers using the squash or three-way merge strategy.</p>
    async fn batch_describe_merge_conflicts(
        &self,
        input: BatchDescribeMergeConflictsInput,
    ) -> Result<BatchDescribeMergeConflictsOutput, RusotoError<BatchDescribeMergeConflictsError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.BatchDescribeMergeConflicts",
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
                .deserialize::<BatchDescribeMergeConflictsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDescribeMergeConflictsError::from_response(response))
        }
    }

    /// <p>Removes the association between an approval rule template and one or more specified repositories. </p>
    async fn batch_disassociate_approval_rule_template_from_repositories(
        &self,
        input: BatchDisassociateApprovalRuleTemplateFromRepositoriesInput,
    ) -> Result<
        BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput,
        RusotoError<BatchDisassociateApprovalRuleTemplateFromRepositoriesError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.BatchDisassociateApprovalRuleTemplateFromRepositories",
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
                .deserialize::<BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDisassociateApprovalRuleTemplateFromRepositoriesError::from_response(response))
        }
    }

    /// <p>Returns information about the contents of one or more commits in a repository.</p>
    async fn batch_get_commits(
        &self,
        input: BatchGetCommitsInput,
    ) -> Result<BatchGetCommitsOutput, RusotoError<BatchGetCommitsError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.BatchGetCommits");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchGetCommitsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetCommitsError::from_response(response))
        }
    }

    /// <p><p>Returns information about one or more repositories.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p> </note></p>
    async fn batch_get_repositories(
        &self,
        input: BatchGetRepositoriesInput,
    ) -> Result<BatchGetRepositoriesOutput, RusotoError<BatchGetRepositoriesError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.BatchGetRepositories");
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
                .deserialize::<BatchGetRepositoriesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetRepositoriesError::from_response(response))
        }
    }

    /// <p>Creates a template for approval rules that can then be associated with one or more repositories in your AWS account. When you associate a template with a repository, AWS CodeCommit creates an approval rule that matches the conditions of the template for all pull requests that meet the conditions of the template. For more information, see <a>AssociateApprovalRuleTemplateWithRepository</a>.</p>
    async fn create_approval_rule_template(
        &self,
        input: CreateApprovalRuleTemplateInput,
    ) -> Result<CreateApprovalRuleTemplateOutput, RusotoError<CreateApprovalRuleTemplateError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.CreateApprovalRuleTemplate",
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
                .deserialize::<CreateApprovalRuleTemplateOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApprovalRuleTemplateError::from_response(response))
        }
    }

    /// <p><p>Creates a branch in a repository and points the branch to a commit.</p> <note> <p>Calling the create branch operation does not set a repository&#39;s default branch. To do this, call the update default branch operation.</p> </note></p>
    async fn create_branch(
        &self,
        input: CreateBranchInput,
    ) -> Result<(), RusotoError<CreateBranchError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBranchError::from_response(response))
        }
    }

    /// <p>Creates a commit for a repository on the tip of a specified branch.</p>
    async fn create_commit(
        &self,
        input: CreateCommitInput,
    ) -> Result<CreateCommitOutput, RusotoError<CreateCommitError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateCommit");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateCommitOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCommitError::from_response(response))
        }
    }

    /// <p>Creates a pull request in the specified repository.</p>
    async fn create_pull_request(
        &self,
        input: CreatePullRequestInput,
    ) -> Result<CreatePullRequestOutput, RusotoError<CreatePullRequestError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreatePullRequest");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreatePullRequestOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePullRequestError::from_response(response))
        }
    }

    /// <p>Creates an approval rule for a pull request.</p>
    async fn create_pull_request_approval_rule(
        &self,
        input: CreatePullRequestApprovalRuleInput,
    ) -> Result<CreatePullRequestApprovalRuleOutput, RusotoError<CreatePullRequestApprovalRuleError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.CreatePullRequestApprovalRule",
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
                .deserialize::<CreatePullRequestApprovalRuleOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePullRequestApprovalRuleError::from_response(response))
        }
    }

    /// <p>Creates a new, empty repository.</p>
    async fn create_repository(
        &self,
        input: CreateRepositoryInput,
    ) -> Result<CreateRepositoryOutput, RusotoError<CreateRepositoryError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.CreateRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateRepositoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRepositoryError::from_response(response))
        }
    }

    /// <p><p>Creates an unreferenced commit that represents the result of merging two branches using a specified merge strategy. This can help you determine the outcome of a potential merge. This API cannot be used with the fast-forward merge strategy because that strategy does not create a merge commit.</p> <note> <p>This unreferenced merge commit can only be accessed using the GetCommit API or through git commands such as git fetch. To retrieve this commit, you must specify its commit ID or otherwise reference it.</p> </note></p>
    async fn create_unreferenced_merge_commit(
        &self,
        input: CreateUnreferencedMergeCommitInput,
    ) -> Result<CreateUnreferencedMergeCommitOutput, RusotoError<CreateUnreferencedMergeCommitError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.CreateUnreferencedMergeCommit",
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
                .deserialize::<CreateUnreferencedMergeCommitOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUnreferencedMergeCommitError::from_response(response))
        }
    }

    /// <p>Deletes a specified approval rule template. Deleting a template does not remove approval rules on pull requests already created with the template.</p>
    async fn delete_approval_rule_template(
        &self,
        input: DeleteApprovalRuleTemplateInput,
    ) -> Result<DeleteApprovalRuleTemplateOutput, RusotoError<DeleteApprovalRuleTemplateError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.DeleteApprovalRuleTemplate",
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
                .deserialize::<DeleteApprovalRuleTemplateOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApprovalRuleTemplateError::from_response(response))
        }
    }

    /// <p>Deletes a branch from a repository, unless that branch is the default branch for the repository. </p>
    async fn delete_branch(
        &self,
        input: DeleteBranchInput,
    ) -> Result<DeleteBranchOutput, RusotoError<DeleteBranchError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteBranchOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBranchError::from_response(response))
        }
    }

    /// <p>Deletes the content of a comment made on a change, file, or commit in a repository.</p>
    async fn delete_comment_content(
        &self,
        input: DeleteCommentContentInput,
    ) -> Result<DeleteCommentContentOutput, RusotoError<DeleteCommentContentError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteCommentContent");
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
                .deserialize::<DeleteCommentContentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCommentContentError::from_response(response))
        }
    }

    /// <p>Deletes a specified file from a specified branch. A commit is created on the branch that contains the revision. The file still exists in the commits earlier to the commit that contains the deletion.</p>
    async fn delete_file(
        &self,
        input: DeleteFileInput,
    ) -> Result<DeleteFileOutput, RusotoError<DeleteFileError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteFile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteFileOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFileError::from_response(response))
        }
    }

    /// <p>Deletes an approval rule from a specified pull request. Approval rules can be deleted from a pull request only if the pull request is open, and if the approval rule was created specifically for a pull request and not generated from an approval rule template associated with the repository where the pull request was created. You cannot delete an approval rule from a merged or closed pull request.</p>
    async fn delete_pull_request_approval_rule(
        &self,
        input: DeletePullRequestApprovalRuleInput,
    ) -> Result<DeletePullRequestApprovalRuleOutput, RusotoError<DeletePullRequestApprovalRuleError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.DeletePullRequestApprovalRule",
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
                .deserialize::<DeletePullRequestApprovalRuleOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePullRequestApprovalRuleError::from_response(response))
        }
    }

    /// <p><p>Deletes a repository. If a specified repository was already deleted, a null repository ID is returned.</p> <important> <p>Deleting a repository also deletes all associated objects and metadata. After a repository is deleted, all future push calls to the deleted repository fail.</p> </important></p>
    async fn delete_repository(
        &self,
        input: DeleteRepositoryInput,
    ) -> Result<DeleteRepositoryOutput, RusotoError<DeleteRepositoryError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DeleteRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteRepositoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRepositoryError::from_response(response))
        }
    }

    /// <p>Returns information about one or more merge conflicts in the attempted merge of two commit specifiers using the squash or three-way merge strategy. If the merge option for the attempted merge is specified as FAST_FORWARD_MERGE, an exception is thrown.</p>
    async fn describe_merge_conflicts(
        &self,
        input: DescribeMergeConflictsInput,
    ) -> Result<DescribeMergeConflictsOutput, RusotoError<DescribeMergeConflictsError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.DescribeMergeConflicts");
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
                .deserialize::<DescribeMergeConflictsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMergeConflictsError::from_response(response))
        }
    }

    /// <p>Returns information about one or more pull request events.</p>
    async fn describe_pull_request_events(
        &self,
        input: DescribePullRequestEventsInput,
    ) -> Result<DescribePullRequestEventsOutput, RusotoError<DescribePullRequestEventsError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.DescribePullRequestEvents",
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
                .deserialize::<DescribePullRequestEventsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePullRequestEventsError::from_response(response))
        }
    }

    /// <p>Removes the association between a template and a repository so that approval rules based on the template are not automatically created when pull requests are created in the specified repository. This does not delete any approval rules previously created for pull requests through the template association.</p>
    async fn disassociate_approval_rule_template_from_repository(
        &self,
        input: DisassociateApprovalRuleTemplateFromRepositoryInput,
    ) -> Result<(), RusotoError<DisassociateApprovalRuleTemplateFromRepositoryError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.DisassociateApprovalRuleTemplateFromRepository",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateApprovalRuleTemplateFromRepositoryError::from_response(response))
        }
    }

    /// <p>Evaluates whether a pull request has met all the conditions specified in its associated approval rules.</p>
    async fn evaluate_pull_request_approval_rules(
        &self,
        input: EvaluatePullRequestApprovalRulesInput,
    ) -> Result<
        EvaluatePullRequestApprovalRulesOutput,
        RusotoError<EvaluatePullRequestApprovalRulesError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.EvaluatePullRequestApprovalRules",
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
                .deserialize::<EvaluatePullRequestApprovalRulesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EvaluatePullRequestApprovalRulesError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns information about a specified approval rule template.</p>
    async fn get_approval_rule_template(
        &self,
        input: GetApprovalRuleTemplateInput,
    ) -> Result<GetApprovalRuleTemplateOutput, RusotoError<GetApprovalRuleTemplateError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.GetApprovalRuleTemplate",
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
                .deserialize::<GetApprovalRuleTemplateOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetApprovalRuleTemplateError::from_response(response))
        }
    }

    /// <p>Returns the base-64 encoded content of an individual blob in a repository.</p>
    async fn get_blob(
        &self,
        input: GetBlobInput,
    ) -> Result<GetBlobOutput, RusotoError<GetBlobError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetBlob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetBlobOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetBlobError::from_response(response))
        }
    }

    /// <p>Returns information about a repository branch, including its name and the last commit ID.</p>
    async fn get_branch(
        &self,
        input: GetBranchInput,
    ) -> Result<GetBranchOutput, RusotoError<GetBranchError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetBranchOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetBranchError::from_response(response))
        }
    }

    /// <p>Returns the content of a comment made on a change, file, or commit in a repository.</p>
    async fn get_comment(
        &self,
        input: GetCommentInput,
    ) -> Result<GetCommentOutput, RusotoError<GetCommentError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetComment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetCommentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCommentError::from_response(response))
        }
    }

    /// <p>Returns information about comments made on the comparison between two commits.</p>
    async fn get_comments_for_compared_commit(
        &self,
        input: GetCommentsForComparedCommitInput,
    ) -> Result<GetCommentsForComparedCommitOutput, RusotoError<GetCommentsForComparedCommitError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.GetCommentsForComparedCommit",
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
                .deserialize::<GetCommentsForComparedCommitOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCommentsForComparedCommitError::from_response(response))
        }
    }

    /// <p>Returns comments made on a pull request.</p>
    async fn get_comments_for_pull_request(
        &self,
        input: GetCommentsForPullRequestInput,
    ) -> Result<GetCommentsForPullRequestOutput, RusotoError<GetCommentsForPullRequestError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.GetCommentsForPullRequest",
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
                .deserialize::<GetCommentsForPullRequestOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCommentsForPullRequestError::from_response(response))
        }
    }

    /// <p>Returns information about a commit, including commit message and committer information.</p>
    async fn get_commit(
        &self,
        input: GetCommitInput,
    ) -> Result<GetCommitOutput, RusotoError<GetCommitError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetCommit");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetCommitOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCommitError::from_response(response))
        }
    }

    /// <p>Returns information about the differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID, or other fully qualified reference). Results can be limited to a specified path.</p>
    async fn get_differences(
        &self,
        input: GetDifferencesInput,
    ) -> Result<GetDifferencesOutput, RusotoError<GetDifferencesError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetDifferences");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDifferencesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDifferencesError::from_response(response))
        }
    }

    /// <p>Returns the base-64 encoded contents of a specified file and its metadata.</p>
    async fn get_file(
        &self,
        input: GetFileInput,
    ) -> Result<GetFileOutput, RusotoError<GetFileError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetFile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetFileOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetFileError::from_response(response))
        }
    }

    /// <p>Returns the contents of a specified folder in a repository.</p>
    async fn get_folder(
        &self,
        input: GetFolderInput,
    ) -> Result<GetFolderOutput, RusotoError<GetFolderError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetFolder");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetFolderOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetFolderError::from_response(response))
        }
    }

    /// <p>Returns information about a specified merge commit.</p>
    async fn get_merge_commit(
        &self,
        input: GetMergeCommitInput,
    ) -> Result<GetMergeCommitOutput, RusotoError<GetMergeCommitError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetMergeCommit");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetMergeCommitOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetMergeCommitError::from_response(response))
        }
    }

    /// <p>Returns information about merge conflicts between the before and after commit IDs for a pull request in a repository.</p>
    async fn get_merge_conflicts(
        &self,
        input: GetMergeConflictsInput,
    ) -> Result<GetMergeConflictsOutput, RusotoError<GetMergeConflictsError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetMergeConflicts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetMergeConflictsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetMergeConflictsError::from_response(response))
        }
    }

    /// <p>Returns information about the merge options available for merging two specified branches. For details about why a merge option is not available, use GetMergeConflicts or DescribeMergeConflicts.</p>
    async fn get_merge_options(
        &self,
        input: GetMergeOptionsInput,
    ) -> Result<GetMergeOptionsOutput, RusotoError<GetMergeOptionsError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetMergeOptions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetMergeOptionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetMergeOptionsError::from_response(response))
        }
    }

    /// <p>Gets information about a pull request in a specified repository.</p>
    async fn get_pull_request(
        &self,
        input: GetPullRequestInput,
    ) -> Result<GetPullRequestOutput, RusotoError<GetPullRequestError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetPullRequest");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetPullRequestOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetPullRequestError::from_response(response))
        }
    }

    /// <p>Gets information about the approval states for a specified pull request. Approval states only apply to pull requests that have one or more approval rules applied to them.</p>
    async fn get_pull_request_approval_states(
        &self,
        input: GetPullRequestApprovalStatesInput,
    ) -> Result<GetPullRequestApprovalStatesOutput, RusotoError<GetPullRequestApprovalStatesError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.GetPullRequestApprovalStates",
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
                .deserialize::<GetPullRequestApprovalStatesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetPullRequestApprovalStatesError::from_response(response))
        }
    }

    /// <p>Returns information about whether approval rules have been set aside (overridden) for a pull request, and if so, the Amazon Resource Name (ARN) of the user or identity that overrode the rules and their requirements for the pull request.</p>
    async fn get_pull_request_override_state(
        &self,
        input: GetPullRequestOverrideStateInput,
    ) -> Result<GetPullRequestOverrideStateOutput, RusotoError<GetPullRequestOverrideStateError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.GetPullRequestOverrideState",
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
                .deserialize::<GetPullRequestOverrideStateOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetPullRequestOverrideStateError::from_response(response))
        }
    }

    /// <p><p>Returns information about a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p> </note></p>
    async fn get_repository(
        &self,
        input: GetRepositoryInput,
    ) -> Result<GetRepositoryOutput, RusotoError<GetRepositoryError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetRepositoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRepositoryError::from_response(response))
        }
    }

    /// <p>Gets information about triggers configured for a repository.</p>
    async fn get_repository_triggers(
        &self,
        input: GetRepositoryTriggersInput,
    ) -> Result<GetRepositoryTriggersOutput, RusotoError<GetRepositoryTriggersError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.GetRepositoryTriggers");
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
                .deserialize::<GetRepositoryTriggersOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRepositoryTriggersError::from_response(response))
        }
    }

    /// <p>Lists all approval rule templates in the specified AWS Region in your AWS account. If an AWS Region is not specified, the AWS Region where you are signed in is used.</p>
    async fn list_approval_rule_templates(
        &self,
        input: ListApprovalRuleTemplatesInput,
    ) -> Result<ListApprovalRuleTemplatesOutput, RusotoError<ListApprovalRuleTemplatesError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.ListApprovalRuleTemplates",
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
                .deserialize::<ListApprovalRuleTemplatesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListApprovalRuleTemplatesError::from_response(response))
        }
    }

    /// <p>Lists all approval rule templates that are associated with a specified repository.</p>
    async fn list_associated_approval_rule_templates_for_repository(
        &self,
        input: ListAssociatedApprovalRuleTemplatesForRepositoryInput,
    ) -> Result<
        ListAssociatedApprovalRuleTemplatesForRepositoryOutput,
        RusotoError<ListAssociatedApprovalRuleTemplatesForRepositoryError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.ListAssociatedApprovalRuleTemplatesForRepository",
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
                .deserialize::<ListAssociatedApprovalRuleTemplatesForRepositoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAssociatedApprovalRuleTemplatesForRepositoryError::from_response(response))
        }
    }

    /// <p>Gets information about one or more branches in a repository.</p>
    async fn list_branches(
        &self,
        input: ListBranchesInput,
    ) -> Result<ListBranchesOutput, RusotoError<ListBranchesError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListBranches");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListBranchesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListBranchesError::from_response(response))
        }
    }

    /// <p>Returns a list of pull requests for a specified repository. The return list can be refined by pull request status or pull request author ARN.</p>
    async fn list_pull_requests(
        &self,
        input: ListPullRequestsInput,
    ) -> Result<ListPullRequestsOutput, RusotoError<ListPullRequestsError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListPullRequests");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListPullRequestsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListPullRequestsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more repositories.</p>
    async fn list_repositories(
        &self,
        input: ListRepositoriesInput,
    ) -> Result<ListRepositoriesOutput, RusotoError<ListRepositoriesError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListRepositories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListRepositoriesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListRepositoriesError::from_response(response))
        }
    }

    /// <p>Lists all repositories associated with the specified approval rule template.</p>
    async fn list_repositories_for_approval_rule_template(
        &self,
        input: ListRepositoriesForApprovalRuleTemplateInput,
    ) -> Result<
        ListRepositoriesForApprovalRuleTemplateOutput,
        RusotoError<ListRepositoriesForApprovalRuleTemplateError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.ListRepositoriesForApprovalRuleTemplate",
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
                .deserialize::<ListRepositoriesForApprovalRuleTemplateOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListRepositoriesForApprovalRuleTemplateError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets information about AWS tags for a specified Amazon Resource Name (ARN) in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the<i> AWS CodeCommit User Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Merges two branches using the fast-forward merge strategy.</p>
    async fn merge_branches_by_fast_forward(
        &self,
        input: MergeBranchesByFastForwardInput,
    ) -> Result<MergeBranchesByFastForwardOutput, RusotoError<MergeBranchesByFastForwardError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.MergeBranchesByFastForward",
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
                .deserialize::<MergeBranchesByFastForwardOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MergeBranchesByFastForwardError::from_response(response))
        }
    }

    /// <p>Merges two branches using the squash merge strategy.</p>
    async fn merge_branches_by_squash(
        &self,
        input: MergeBranchesBySquashInput,
    ) -> Result<MergeBranchesBySquashOutput, RusotoError<MergeBranchesBySquashError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.MergeBranchesBySquash");
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
                .deserialize::<MergeBranchesBySquashOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MergeBranchesBySquashError::from_response(response))
        }
    }

    /// <p>Merges two specified branches using the three-way merge strategy.</p>
    async fn merge_branches_by_three_way(
        &self,
        input: MergeBranchesByThreeWayInput,
    ) -> Result<MergeBranchesByThreeWayOutput, RusotoError<MergeBranchesByThreeWayError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.MergeBranchesByThreeWay",
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
                .deserialize::<MergeBranchesByThreeWayOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MergeBranchesByThreeWayError::from_response(response))
        }
    }

    /// <p>Attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the fast-forward merge strategy. If the merge is successful, it closes the pull request.</p>
    async fn merge_pull_request_by_fast_forward(
        &self,
        input: MergePullRequestByFastForwardInput,
    ) -> Result<MergePullRequestByFastForwardOutput, RusotoError<MergePullRequestByFastForwardError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.MergePullRequestByFastForward",
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
                .deserialize::<MergePullRequestByFastForwardOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MergePullRequestByFastForwardError::from_response(response))
        }
    }

    /// <p>Attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the squash merge strategy. If the merge is successful, it closes the pull request.</p>
    async fn merge_pull_request_by_squash(
        &self,
        input: MergePullRequestBySquashInput,
    ) -> Result<MergePullRequestBySquashOutput, RusotoError<MergePullRequestBySquashError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.MergePullRequestBySquash",
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
                .deserialize::<MergePullRequestBySquashOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MergePullRequestBySquashError::from_response(response))
        }
    }

    /// <p>Attempts to merge the source commit of a pull request into the specified destination branch for that pull request at the specified commit using the three-way merge strategy. If the merge is successful, it closes the pull request.</p>
    async fn merge_pull_request_by_three_way(
        &self,
        input: MergePullRequestByThreeWayInput,
    ) -> Result<MergePullRequestByThreeWayOutput, RusotoError<MergePullRequestByThreeWayError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.MergePullRequestByThreeWay",
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
                .deserialize::<MergePullRequestByThreeWayOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MergePullRequestByThreeWayError::from_response(response))
        }
    }

    /// <p>Sets aside (overrides) all approval rule requirements for a specified pull request.</p>
    async fn override_pull_request_approval_rules(
        &self,
        input: OverridePullRequestApprovalRulesInput,
    ) -> Result<(), RusotoError<OverridePullRequestApprovalRulesError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.OverridePullRequestApprovalRules",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(OverridePullRequestApprovalRulesError::from_response(
                response,
            ))
        }
    }

    /// <p>Posts a comment on the comparison between two commits.</p>
    async fn post_comment_for_compared_commit(
        &self,
        input: PostCommentForComparedCommitInput,
    ) -> Result<PostCommentForComparedCommitOutput, RusotoError<PostCommentForComparedCommitError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.PostCommentForComparedCommit",
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
                .deserialize::<PostCommentForComparedCommitOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PostCommentForComparedCommitError::from_response(response))
        }
    }

    /// <p>Posts a comment on a pull request.</p>
    async fn post_comment_for_pull_request(
        &self,
        input: PostCommentForPullRequestInput,
    ) -> Result<PostCommentForPullRequestOutput, RusotoError<PostCommentForPullRequestError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.PostCommentForPullRequest",
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
                .deserialize::<PostCommentForPullRequestOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PostCommentForPullRequestError::from_response(response))
        }
    }

    /// <p>Posts a comment in reply to an existing comment on a comparison between commits or a pull request.</p>
    async fn post_comment_reply(
        &self,
        input: PostCommentReplyInput,
    ) -> Result<PostCommentReplyOutput, RusotoError<PostCommentReplyError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PostCommentReply");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PostCommentReplyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PostCommentReplyError::from_response(response))
        }
    }

    /// <p>Adds or updates a file in a branch in an AWS CodeCommit repository, and generates a commit for the addition in the specified branch.</p>
    async fn put_file(
        &self,
        input: PutFileInput,
    ) -> Result<PutFileOutput, RusotoError<PutFileError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PutFile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutFileOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutFileError::from_response(response))
        }
    }

    /// <p>Replaces all triggers for a repository. Used to create or delete triggers.</p>
    async fn put_repository_triggers(
        &self,
        input: PutRepositoryTriggersInput,
    ) -> Result<PutRepositoryTriggersOutput, RusotoError<PutRepositoryTriggersError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.PutRepositoryTriggers");
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
                .deserialize::<PutRepositoryTriggersOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRepositoryTriggersError::from_response(response))
        }
    }

    /// <p>Adds or updates tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the <i>AWS CodeCommit User Guide</i>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test sends data from the last commit. If no data is available, sample data is generated.</p>
    async fn test_repository_triggers(
        &self,
        input: TestRepositoryTriggersInput,
    ) -> Result<TestRepositoryTriggersOutput, RusotoError<TestRepositoryTriggersError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.TestRepositoryTriggers");
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
                .deserialize::<TestRepositoryTriggersOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TestRepositoryTriggersError::from_response(response))
        }
    }

    /// <p>Removes tags for a resource in AWS CodeCommit. For a list of valid resources in AWS CodeCommit, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/auth-and-access-control-iam-access-control-identity-based.html#arn-formats">CodeCommit Resources and Operations</a> in the <i>AWS CodeCommit User Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the content of an approval rule template. You can change the number of required approvals, the membership of the approval rule, and whether an approval pool is defined.</p>
    async fn update_approval_rule_template_content(
        &self,
        input: UpdateApprovalRuleTemplateContentInput,
    ) -> Result<
        UpdateApprovalRuleTemplateContentOutput,
        RusotoError<UpdateApprovalRuleTemplateContentError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdateApprovalRuleTemplateContent",
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
                .deserialize::<UpdateApprovalRuleTemplateContentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApprovalRuleTemplateContentError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the description for a specified approval rule template.</p>
    async fn update_approval_rule_template_description(
        &self,
        input: UpdateApprovalRuleTemplateDescriptionInput,
    ) -> Result<
        UpdateApprovalRuleTemplateDescriptionOutput,
        RusotoError<UpdateApprovalRuleTemplateDescriptionError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdateApprovalRuleTemplateDescription",
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
                .deserialize::<UpdateApprovalRuleTemplateDescriptionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApprovalRuleTemplateDescriptionError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the name of a specified approval rule template.</p>
    async fn update_approval_rule_template_name(
        &self,
        input: UpdateApprovalRuleTemplateNameInput,
    ) -> Result<
        UpdateApprovalRuleTemplateNameOutput,
        RusotoError<UpdateApprovalRuleTemplateNameError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdateApprovalRuleTemplateName",
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
                .deserialize::<UpdateApprovalRuleTemplateNameOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApprovalRuleTemplateNameError::from_response(response))
        }
    }

    /// <p>Replaces the contents of a comment.</p>
    async fn update_comment(
        &self,
        input: UpdateCommentInput,
    ) -> Result<UpdateCommentOutput, RusotoError<UpdateCommentError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateComment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateCommentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateCommentError::from_response(response))
        }
    }

    /// <p><p>Sets or changes the default branch name for the specified repository.</p> <note> <p>If you use this operation to change the default branch name to the current default branch name, a success message is returned even though the default branch did not change.</p> </note></p>
    async fn update_default_branch(
        &self,
        input: UpdateDefaultBranchInput,
    ) -> Result<(), RusotoError<UpdateDefaultBranchError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateDefaultBranch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDefaultBranchError::from_response(response))
        }
    }

    /// <p>Updates the structure of an approval rule created specifically for a pull request. For example, you can change the number of required approvers and the approval pool for approvers. </p>
    async fn update_pull_request_approval_rule_content(
        &self,
        input: UpdatePullRequestApprovalRuleContentInput,
    ) -> Result<
        UpdatePullRequestApprovalRuleContentOutput,
        RusotoError<UpdatePullRequestApprovalRuleContentError>,
    > {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdatePullRequestApprovalRuleContent",
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
                .deserialize::<UpdatePullRequestApprovalRuleContentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePullRequestApprovalRuleContentError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the state of a user's approval on a pull request. The user is derived from the signed-in account when the request is made.</p>
    async fn update_pull_request_approval_state(
        &self,
        input: UpdatePullRequestApprovalStateInput,
    ) -> Result<(), RusotoError<UpdatePullRequestApprovalStateError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdatePullRequestApprovalState",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePullRequestApprovalStateError::from_response(response))
        }
    }

    /// <p>Replaces the contents of the description of a pull request.</p>
    async fn update_pull_request_description(
        &self,
        input: UpdatePullRequestDescriptionInput,
    ) -> Result<UpdatePullRequestDescriptionOutput, RusotoError<UpdatePullRequestDescriptionError>>
    {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdatePullRequestDescription",
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
                .deserialize::<UpdatePullRequestDescriptionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePullRequestDescriptionError::from_response(response))
        }
    }

    /// <p>Updates the status of a pull request. </p>
    async fn update_pull_request_status(
        &self,
        input: UpdatePullRequestStatusInput,
    ) -> Result<UpdatePullRequestStatusOutput, RusotoError<UpdatePullRequestStatusError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdatePullRequestStatus",
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
                .deserialize::<UpdatePullRequestStatusOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePullRequestStatusError::from_response(response))
        }
    }

    /// <p>Replaces the title of a pull request.</p>
    async fn update_pull_request_title(
        &self,
        input: UpdatePullRequestTitleInput,
    ) -> Result<UpdatePullRequestTitleOutput, RusotoError<UpdatePullRequestTitleError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdatePullRequestTitle");
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
                .deserialize::<UpdatePullRequestTitleOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePullRequestTitleError::from_response(response))
        }
    }

    /// <p><p>Sets or changes the comment or description for a repository.</p> <note> <p>The description field for a repository accepts all HTML characters and all valid Unicode characters. Applications that do not HTML-encode the description and display it in a webpage can expose users to potentially malicious code. Make sure that you HTML-encode the description field in any application that uses this API to display the repository description on a webpage.</p> </note></p>
    async fn update_repository_description(
        &self,
        input: UpdateRepositoryDescriptionInput,
    ) -> Result<(), RusotoError<UpdateRepositoryDescriptionError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeCommit_20150413.UpdateRepositoryDescription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRepositoryDescriptionError::from_response(response))
        }
    }

    /// <p>Renames a repository. The repository name must be unique across the calling AWS account. Repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix .git is prohibited. For more information about the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide.</p>
    async fn update_repository_name(
        &self,
        input: UpdateRepositoryNameInput,
    ) -> Result<(), RusotoError<UpdateRepositoryNameError>> {
        let mut request = SignedRequest::new("POST", "codecommit", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeCommit_20150413.UpdateRepositoryName");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRepositoryNameError::from_response(response))
        }
    }
}
