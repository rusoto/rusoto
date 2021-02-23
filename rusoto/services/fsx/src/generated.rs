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
#[allow(unused_imports)]
use rusoto_core::pagination::{aws_stream, Paged, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};
#[allow(unused_imports)]
use std::borrow::Cow;

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl FsxClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "fsx", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>The Microsoft AD attributes of the Amazon FSx for Windows File Server file system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActiveDirectoryBackupAttributes {
    /// <p>The ID of the AWS Managed Microsoft Active Directory instance to which the file system is joined.</p>
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    /// <p>The fully qualified domain name of the self-managed AD directory.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// <p>Describes a specific Amazon FSx administrative action for the current Windows or Lustre file system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdministrativeAction {
    #[serde(rename = "AdministrativeActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_action_type: Option<String>,
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<AdministrativeActionFailureDetails>,
    /// <p>Provides the percent complete of a <code>STORAGE_OPTIMIZATION</code> administrative action. Does not apply to any other administrative action type.</p>
    #[serde(rename = "ProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>,
    /// <p>Time that the administrative action request was received.</p>
    #[serde(rename = "RequestTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_time: Option<f64>,
    /// <p><p>Describes the status of the administrative action, as follows:</p> <ul> <li> <p> <code>FAILED</code> - Amazon FSx failed to process the administrative action successfully.</p> </li> <li> <p> <code>IN<em>PROGRESS</code> - Amazon FSx is processing the administrative action.</p> </li> <li> <p> <code>PENDING</code> - Amazon FSx is waiting to process the administrative action.</p> </li> <li> <p> <code>COMPLETED</code> - Amazon FSx has finished processing the administrative task.</p> </li> <li> <p> <code>UPDATED</em>OPTIMIZING</code> - For a storage capacity increase update, Amazon FSx has updated the file system with the new storage capacity, and is now performing the storage optimization process. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-storage-capacity.html">Managing storage capacity</a> in the <i>Amazon FSx for Windows File Server User Guide</i> and <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/managing-storage-capacity.html">Managing storage and throughput capacity</a> in the <i>Amazon FSx for Lustre User Guide</i>.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Describes the target value for the administration action, provided in the <code>UpdateFileSystem</code> operation. Returned for <code>FILE_SYSTEM_UPDATE</code> administrative actions. </p>
    #[serde(rename = "TargetFileSystemValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_file_system_values: Option<FileSystem>,
}

/// <p>Provides information about a failed administrative action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdministrativeActionFailureDetails {
    /// <p>Error message providing details about the failed administrative action.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>A DNS alias that is associated with the file system. You can use a DNS alias to access a file system using user-defined DNS names, in addition to the default DNS name that Amazon FSx assigns to the file system. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html">DNS aliases</a> in the <i>FSx for Windows File Server User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Alias {
    /// <p><p>Describes the state of the DNS alias.</p> <ul> <li> <p>AVAILABLE - The DNS alias is associated with an Amazon FSx file system.</p> </li> <li> <p>CREATING - Amazon FSx is creating the DNS alias and associating it with the file system.</p> </li> <li> <p>CREATE<em>FAILED - Amazon FSx was unable to associate the DNS alias with the file system.</p> </li> <li> <p>DELETING - Amazon FSx is disassociating the DNS alias from the file system and deleting it.</p> </li> <li> <p>DELETE</em>FAILED - Amazon FSx was unable to disassocate the DNS alias from the file system.</p> </li> </ul></p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    /// <p>The name of the DNS alias. The alias name has to meet the following requirements:</p> <ul> <li> <p>Formatted as a fully-qualified domain name (FQDN), <code>hostname.domain</code>, for example, <code>accounting.example.com</code>.</p> </li> <li> <p>Can contain alphanumeric characters and the hyphen (-).</p> </li> <li> <p>Cannot start or end with a hyphen.</p> </li> <li> <p>Can start with a numeric.</p> </li> </ul> <p>For DNS names, Amazon FSx stores alphabetic characters as lowercase letters (a-z), regardless of how you specify them: as uppercase letters, lowercase letters, or the corresponding letters in escape codes.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The request object specifying one or more DNS alias names to associate with an Amazon FSx for Windows File Server file system.</p>
/// see [Fsx::associate_file_system_aliases]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateFileSystemAliasesRequest {
    /// <p>An array of one or more DNS alias names to associate with the file system. The alias name has to comply with the following formatting requirements:</p> <ul> <li> <p>Formatted as a fully-qualified domain name (FQDN), <i> <code>hostname.domain</code> </i>, for example, <code>accounting.corp.example.com</code>.</p> </li> <li> <p>Can contain alphanumeric characters and the hyphen (-).</p> </li> <li> <p>Cannot start or end with a hyphen.</p> </li> <li> <p>Can start with a numeric.</p> </li> </ul> <p>For DNS alias names, Amazon FSx stores alphabetic characters as lowercase letters (a-z), regardless of how you specify them: as uppercase letters, lowercase letters, or the corresponding letters in escape codes.</p>
    #[serde(rename = "Aliases")]
    pub aliases: Vec<String>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Specifies the file system with which you want to associate one or more DNS aliases.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

/// <p>The system generated response showing the DNS aliases that Amazon FSx is attempting to associate with the file system. Use the API operation to monitor the status of the aliases Amazon FSx is associating with the file system. It can take up to 2.5 minutes for the alias status to change from <code>CREATING</code> to <code>AVAILABLE</code>. </p>
/// see [Fsx::associate_file_system_aliases]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateFileSystemAliasesResponse {
    /// <p>An array of the DNS aliases that Amazon FSx is associating with the file system.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
}

/// <p><p>A backup of an Amazon FSx file system. For more information see:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/using-backups.html">Working with backups for Windows file systems</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/using-backups-fsx.html">Working with backups for Lustre file systems</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Backup {
    /// <p>The ID of the backup.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>The time when a particular backup was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The configuration of the self-managed Microsoft Active Directory (AD) to which the Windows File Server instance is joined.</p>
    #[serde(rename = "DirectoryInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_information: Option<ActiveDirectoryBackupAttributes>,
    /// <p>Details explaining any failures that occur when creating a backup.</p>
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<BackupFailureDetails>,
    /// <p>Metadata of the file system associated with the backup. This metadata is persisted even if the file system is deleted.</p>
    #[serde(rename = "FileSystem")]
    pub file_system: FileSystem,
    /// <p>The ID of the AWS Key Management Service (AWS KMS) key used to encrypt the backup of the Amazon FSx file system's data at rest. </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p><p>The lifecycle status of the backup.</p> <ul> <li> <p> <code>AVAILABLE</code> - The backup is fully available.</p> </li> <li> <p> <code>PENDING</code> - For user-initiated backups on Lustre file systems only; Amazon FSx has not started creating the backup.</p> </li> <li> <p> <code>CREATING</code> - Amazon FSx is creating the backup.</p> </li> <li> <p> <code>TRANSFERRING</code> - For user-initiated backups on Lustre file systems only; Amazon FSx is transferring the backup to S3.</p> </li> <li> <p> <code>DELETED</code> - Amazon FSx deleted the backup and it is no longer available.</p> </li> <li> <p> <code>FAILED</code> - Amazon FSx could not complete the backup.</p> </li> </ul></p>
    #[serde(rename = "Lifecycle")]
    pub lifecycle: String,
    #[serde(rename = "ProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) for the backup resource.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>Tags associated with a particular file system.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of the file system backup.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>If backup creation fails, this structure contains the details of that failure.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupFailureDetails {
    /// <p>A message describing the backup creation failure.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Cancels a data repository task.</p>
/// see [Fsx::cancel_data_repository_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelDataRepositoryTaskRequest {
    /// <p>Specifies the data repository task to cancel.</p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

/// see [Fsx::cancel_data_repository_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelDataRepositoryTaskResponse {
    /// <p><p>The lifecycle status of the data repository task, as follows:</p> <ul> <li> <p> <code>PENDING</code> - Amazon FSx has not started the task.</p> </li> <li> <p> <code>EXECUTING</code> - Amazon FSx is processing the task.</p> </li> <li> <p> <code>FAILED</code> - Amazon FSx was not able to complete the task. For example, there may be files the task failed to process. The <a>DataRepositoryTaskFailureDetails</a> property provides more information about task failures.</p> </li> <li> <p> <code>SUCCEEDED</code> - FSx completed the task successfully.</p> </li> <li> <p> <code>CANCELED</code> - Amazon FSx canceled the task and it did not complete.</p> </li> <li> <p> <code>CANCELING</code> - FSx is in process of canceling the task.</p> </li> </ul></p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    /// <p>The ID of the task being canceled.</p>
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// <p>Provides a report detailing the data repository task results of the files processed that match the criteria specified in the report <code>Scope</code> parameter. FSx delivers the report to the file system's linked data repository in Amazon S3, using the path specified in the report <code>Path</code> parameter. You can specify whether or not a report gets generated for a task using the <code>Enabled</code> parameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CompletionReport {
    /// <p>Set <code>Enabled</code> to <code>True</code> to generate a <code>CompletionReport</code> when the task completes. If set to <code>true</code>, then you need to provide a report <code>Scope</code>, <code>Path</code>, and <code>Format</code>. Set <code>Enabled</code> to <code>False</code> if you do not want a <code>CompletionReport</code> generated when the task completes.</p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    /// <p>Required if <code>Enabled</code> is set to <code>true</code>. Specifies the format of the <code>CompletionReport</code>. <code>REPORT_CSV_20191124</code> is the only format currently supported. When <code>Format</code> is set to <code>REPORT_CSV_20191124</code>, the <code>CompletionReport</code> is provided in CSV format, and is delivered to <code>{path}/task-{id}/failures.csv</code>. </p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Required if <code>Enabled</code> is set to <code>true</code>. Specifies the location of the report on the file system's linked S3 data repository. An absolute path that defines where the completion report will be stored in the destination location. The <code>Path</code> you provide must be located within the file system’s ExportPath. An example <code>Path</code> value is "s3://myBucket/myExportPath/optionalPrefix". The report provides the following information for each file in the report: FilePath, FileStatus, and ErrorCode. To learn more about a file system's <code>ExportPath</code>, see . </p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>Required if <code>Enabled</code> is set to <code>true</code>. Specifies the scope of the <code>CompletionReport</code>; <code>FAILED_FILES_ONLY</code> is the only scope currently supported. When <code>Scope</code> is set to <code>FAILED_FILES_ONLY</code>, the <code>CompletionReport</code> only contains information about files that the data repository task failed to process.</p>
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// <p>The request object for the <code>CreateBackup</code> operation.</p>
/// see [Fsx::create_backup]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBackupRequest {
    /// <p>(Optional) A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent creation. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The ID of the file system to back up.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>(Optional) The tags to apply to the backup at backup creation. The key value of the <code>Name</code> tag appears in the console as the backup name. If you have set <code>CopyTagsToBackups</code> to true, and you specify one or more tags using the <code>CreateBackup</code> action, no existing file system tags are copied from the file system to the backup.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The response object for the <code>CreateBackup</code> operation.</p>
/// see [Fsx::create_backup]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBackupResponse {
    /// <p>A description of the backup.</p>
    #[serde(rename = "Backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

/// see [Fsx::create_data_repository_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataRepositoryTaskRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>(Optional) The path or paths on the Amazon FSx file system to use when the data repository task is processed. The default path is the file system root directory. The paths you provide need to be relative to the mount point of the file system. If the mount point is <code>/mnt/fsx</code> and <code>/mnt/fsx/path1</code> is a directory or file on the file system you want to export, then the path to provide is <code>path1</code>. If a path that you provide isn't valid, the task fails.</p>
    #[serde(rename = "Paths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    /// <p>Defines whether or not Amazon FSx provides a CompletionReport once the task has completed. A CompletionReport provides a detailed report on the files that Amazon FSx processed that meet the criteria specified by the <code>Scope</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/task-completion-report.html">Working with Task Completion Reports</a>.</p>
    #[serde(rename = "Report")]
    pub report: CompletionReport,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specifies the type of data repository task to create.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// see [Fsx::create_data_repository_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataRepositoryTaskResponse {
    /// <p>The description of the data repository task that you just created.</p>
    #[serde(rename = "DataRepositoryTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_task: Option<DataRepositoryTask>,
}

/// <p>The request object for the <code>CreateFileSystemFromBackup</code> operation.</p>
/// see [Fsx::create_file_system_from_backup]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFileSystemFromBackupRequest {
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent creation. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<CreateFileSystemLustreConfiguration>,
    /// <p>A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups apply to all network interfaces. This value isn't returned in later DescribeFileSystem requests.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>Sets the storage type for the Windows file system you&#39;re creating from a backup. Valid values are <code>SSD</code> and <code>HDD</code>.</p> <ul> <li> <p>Set to <code>SSD</code> to use solid state drive storage. Supported on all Windows deployment types.</p> </li> <li> <p>Set to <code>HDD</code> to use hard disk drive storage. Supported on <code>SINGLE<em>AZ</em>2</code> and <code>MULTI<em>AZ</em>1</code> Windows file system deployment types. </p> </li> </ul> <p> Default value is <code>SSD</code>. </p> <note> <p>HDD and SSD storage types have different minimum storage capacity requirements. A restored file system&#39;s storage capacity is tied to the file system that was backed up. You can create a file system that uses HDD storage from a backup of a file system that used SSD storage only if the original SSD file system had a storage capacity of at least 2000 GiB. </p> </note></p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// <p>Specifies the IDs of the subnets that the file system will be accessible from. For Windows <code>MULTI_AZ_1</code> file system deployment types, provide exactly two subnet IDs, one for the preferred file server and one for the standby file server. You specify one of these subnets as the preferred subnet using the <code>WindowsConfiguration &gt; PreferredSubnetID</code> property.</p> <p>For Windows <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> deployment types and Lustre file systems, provide exactly one subnet ID. The file server is launched in that subnet's Availability Zone.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The tags to be applied to the file system at file system creation. The key value of the <code>Name</code> tag appears in the console as the file system name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The configuration for this Microsoft Windows file system.</p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<CreateFileSystemWindowsConfiguration>,
}

/// <p>The response object for the <code>CreateFileSystemFromBackup</code> operation.</p>
/// see [Fsx::create_file_system_from_backup]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFileSystemFromBackupResponse {
    /// <p>A description of the file system.</p>
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

/// <p>The Lustre configuration for the file system being created. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFileSystemLustreConfiguration {
    /// <p> (Optional) When you create your file system, your existing S3 objects appear as file and directory listings. Use this property to choose how Amazon FSx keeps your file and directory listings up to date as you add or modify objects in your linked S3 bucket. <code>AutoImportPolicy</code> can have the following values:</p> <ul> <li> <p> <code>NONE</code> - (Default) AutoImport is off. Amazon FSx only updates file and directory listings from the linked S3 bucket when the file system is created. FSx does not update file and directory listings for any new or changed objects after choosing this option.</p> </li> <li> <p> <code>NEW</code> - AutoImport is on. Amazon FSx automatically imports directory listings of any new objects added to the linked S3 bucket that do not currently exist in the FSx file system. </p> </li> <li> <p> <code>NEW_CHANGED</code> - AutoImport is on. Amazon FSx automatically imports file and directory listings of any new objects added to the S3 bucket and any existing objects that are changed in the S3 bucket after you choose this option. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/autoimport-data-repo.html">Automatically import updates from your S3 bucket</a>.</p>
    #[serde(rename = "AutoImportPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_import_policy: Option<String>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>(Optional) Not available to use with file systems that are linked to a data repository. A boolean flag indicating whether tags for the file system should be copied to backups. The default value is false. If it's set to true, all file system tags are copied to all automatic and user-initiated backups when the user doesn't specify any backup-specific tags. If this value is true, and you specify one or more backup tags, only the specified tags are copied to backups. If you specify one or more tags when creating a user-initiated backup, no tags are copied from the file system, regardless of this value.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/using-backups-fsx.html">Working with backups</a>.</p>
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    /// <p> Choose <code>SCRATCH_1</code> and <code>SCRATCH_2</code> deployment types when you need temporary storage and shorter-term processing of data. The <code>SCRATCH_2</code> deployment type provides in-transit encryption of data and higher burst throughput capacity than <code>SCRATCH_1</code>.</p> <p>Choose <code>PERSISTENT_1</code> deployment type for longer-term storage and workloads and encryption of data in transit. To learn more about deployment types, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/lustre-deployment-types.html"> FSx for Lustre Deployment Options</a>.</p> <p>Encryption of data in-transit is automatically enabled when you access a <code>SCRATCH_2</code> or <code>PERSISTENT_1</code> file system from Amazon EC2 instances that <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/data- protection.html">support this feature</a>. (Default = <code>SCRATCH_1</code>) </p> <p>Encryption of data in-transit for <code>SCRATCH_2</code> and <code>PERSISTENT_1</code> deployment types is supported when accessed from supported instance types in supported AWS Regions. To learn more, <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/encryption-in-transit-fsxl.html">Encrypting Data in Transit</a>.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>The type of drive cache used by PERSISTENT_1 file systems that are provisioned with HDD storage devices. This parameter is required when storage type is HDD. Set to <code>READ</code>, improve the performance for frequently accessed files and allows 20% of the total storage capacity of the file system to be cached. </p> <p>This parameter is required when <code>StorageType</code> is set to HDD.</p>
    #[serde(rename = "DriveCacheType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_cache_type: Option<String>,
    /// <p>(Optional) The path in Amazon S3 where the root of your Amazon FSx file system is exported. The path must use the same Amazon S3 bucket as specified in ImportPath. You can provide an optional prefix to which new and changed data is to be exported from your Amazon FSx for Lustre file system. If an <code>ExportPath</code> value is not provided, Amazon FSx sets a default export path, <code>s3://import-bucket/FSxLustre[creation-timestamp]</code>. The timestamp is in UTC format, for example <code>s3://import-bucket/FSxLustre20181105T222312Z</code>.</p> <p>The Amazon S3 export bucket must be the same as the import bucket specified by <code>ImportPath</code>. If you only specify a bucket name, such as <code>s3://import-bucket</code>, you get a 1:1 mapping of file system objects to S3 bucket objects. This mapping means that the input data in S3 is overwritten on export. If you provide a custom prefix in the export path, such as <code>s3://import-bucket/[custom-optional-prefix]</code>, Amazon FSx exports the contents of your file system to that export prefix in the Amazon S3 bucket.</p>
    #[serde(rename = "ExportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_path: Option<String>,
    /// <p>(Optional) The path to the Amazon S3 bucket (including the optional prefix) that you're using as the data repository for your Amazon FSx for Lustre file system. The root of your FSx for Lustre file system will be mapped to the root of the Amazon S3 bucket you select. An example is <code>s3://import-bucket/optional-prefix</code>. If you specify a prefix after the Amazon S3 bucket name, only object keys with that prefix are loaded into the file system.</p>
    #[serde(rename = "ImportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_path: Option<String>,
    /// <p>(Optional) For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. The maximum number of disks that a single file can be striped across is limited by the total number of disks that make up the file system.</p> <p>The default chunk size is 1,024 MiB (1 GiB) and can go as high as 512,000 MiB (500 GiB). Amazon S3 objects have a maximum size of 5 TB.</p>
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i64>,
    /// <p> Required for the <code>PERSISTENT_1</code> deployment type, describes the amount of read and write throughput for each 1 tebibyte of storage, in MB/s/TiB. File system throughput capacity is calculated by multiplying ﬁle system storage capacity (TiB) by the PerUnitStorageThroughput (MB/s/TiB). For a 2.4 TiB ﬁle system, provisioning 50 MB/s/TiB of PerUnitStorageThroughput yields 120 MB/s of ﬁle system throughput. You pay for the amount of throughput that you provision. </p> <p>Valid values for SSD storage: 50, 100, 200. Valid values for HDD storage: 12, 40.</p>
    #[serde(rename = "PerUnitStorageThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_storage_throughput: Option<i64>,
    /// <p>(Optional) The preferred start time to perform weekly maintenance, formatted d:HH:MM in the UTC time zone, where d is the weekday number, from 1 through 7, beginning with Monday and ending with Sunday.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The request object used to create a new Amazon FSx file system.</p>
/// see [Fsx::create_file_system]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFileSystemRequest {
    /// <p>A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent creation. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The type of Amazon FSx file system to create, either <code>WINDOWS</code> or <code>LUSTRE</code>.</p>
    #[serde(rename = "FileSystemType")]
    pub file_system_type: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<CreateFileSystemLustreConfiguration>,
    /// <p>A list of IDs specifying the security groups to apply to all network interfaces created for file system access. This list isn't returned in later requests to describe the file system.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>Sets the storage capacity of the file system that you&#39;re creating.</p> <p>For Lustre file systems:</p> <ul> <li> <p>For <code>SCRATCH<em>2</code> and <code>PERSISTENT</em>1 SSD</code> deployment types, valid values are 1200 GiB, 2400 GiB, and increments of 2400 GiB.</p> </li> <li> <p>For <code>PERSISTENT HDD</code> file systems, valid values are increments of 6000 GiB for 12 MB/s/TiB file systems and increments of 1800 GiB for 40 MB/s/TiB file systems.</p> </li> <li> <p>For <code>SCRATCH_1</code> deployment type, valid values are 1200 GiB, 2400 GiB, and increments of 3600 GiB.</p> </li> </ul> <p>For Windows file systems:</p> <ul> <li> <p>If <code>StorageType=SSD</code>, valid values are 32 GiB - 65,536 GiB (64 TiB).</p> </li> <li> <p>If <code>StorageType=HDD</code>, valid values are 2000 GiB - 65,536 GiB (64 TiB).</p> </li> </ul></p>
    #[serde(rename = "StorageCapacity")]
    pub storage_capacity: i64,
    /// <p>Sets the storage type for the file system you're creating. Valid values are <code>SSD</code> and <code>HDD</code>.</p> <ul> <li> <p>Set to <code>SSD</code> to use solid state drive storage. SSD is supported on all Windows and Lustre deployment types.</p> </li> <li> <p>Set to <code>HDD</code> to use hard disk drive storage. HDD is supported on <code>SINGLE_AZ_2</code> and <code>MULTI_AZ_1</code> Windows file system deployment types, and on <code>PERSISTENT</code> Lustre file system deployment types. </p> </li> </ul> <p> Default value is <code>SSD</code>. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/optimize-fsx-costs.html#storage-type-options"> Storage Type Options</a> in the <i>Amazon FSx for Windows User Guide</i> and <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/what-is.html#storage-options">Multiple Storage Options</a> in the <i>Amazon FSx for Lustre User Guide</i>. </p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// <p>Specifies the IDs of the subnets that the file system will be accessible from. For Windows <code>MULTI_AZ_1</code> file system deployment types, provide exactly two subnet IDs, one for the preferred file server and one for the standby file server. You specify one of these subnets as the preferred subnet using the <code>WindowsConfiguration &gt; PreferredSubnetID</code> property.</p> <p>For Windows <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> file system deployment types and Lustre file systems, provide exactly one subnet ID. The file server is launched in that subnet's Availability Zone.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The tags to apply to the file system being created. The key value of the <code>Name</code> tag appears in the console as the file system name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Microsoft Windows configuration for the file system being created. </p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<CreateFileSystemWindowsConfiguration>,
}

/// <p>The response object returned after the file system is created.</p>
/// see [Fsx::create_file_system]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFileSystemResponse {
    /// <p>The configuration of the file system that was created.</p>
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

/// <p>The configuration object for the Microsoft Windows file system used in <code>CreateFileSystem</code> and <code>CreateFileSystemFromBackup</code> operations.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFileSystemWindowsConfiguration {
    /// <p>The ID for an existing AWS Managed Microsoft Active Directory (AD) instance that the file system should join when it's created.</p>
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    /// <p>An array of one or more DNS alias names that you want to associate with the Amazon FSx file system. Aliases allow you to use existing DNS names to access the data in your Amazon FSx file system. You can associate up to 50 aliases with a file system at any time. You can associate additional DNS aliases after you create the file system using the AssociateFileSystemAliases operation. You can remove DNS aliases from the file system after it is created using the DisassociateFileSystemAliases operation. You only need to specify the alias name in the request payload.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html">Working with DNS Aliases</a> and <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/walkthrough05-file-system-custom-CNAME.html">Walkthrough 5: Using DNS aliases to access your file system</a>, including additional steps you must take to be able to access your file system using a DNS alias.</p> <p>An alias name has to meet the following requirements:</p> <ul> <li> <p>Formatted as a fully-qualified domain name (FQDN), <code>hostname.domain</code>, for example, <code>accounting.example.com</code>.</p> </li> <li> <p>Can contain alphanumeric characters and the hyphen (-).</p> </li> <li> <p>Cannot start or end with a hyphen.</p> </li> <li> <p>Can start with a numeric.</p> </li> </ul> <p>For DNS alias names, Amazon FSx stores alphabetic characters as lowercase letters (a-z), regardless of how you specify them: as uppercase letters, lowercase letters, or the corresponding letters in escape codes.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// <p>The number of days to retain automatic backups. The default is to retain backups for 7 days. Setting this value to 0 disables the creation of automatic backups. The maximum retention period for backups is 90 days.</p>
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>A boolean flag indicating whether tags for the file system should be copied to backups. This value defaults to false. If it's set to true, all tags for the file system are copied to all automatic and user-initiated backups where the user doesn't specify tags. If this value is true, and you specify one or more tags, only the specified tags are copied to backups. If you specify one or more tags when creating a user-initiated backup, no tags are copied from the file system, regardless of this value.</p>
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    /// <p>The preferred time to take daily automatic backups, formatted HH:MM in the UTC time zone.</p>
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    /// <p>Specifies the file system deployment type, valid values are the following:</p> <ul> <li> <p> <code>MULTI_AZ_1</code> - Deploys a high availability file system that is configured for Multi-AZ redundancy to tolerate temporary Availability Zone (AZ) unavailability. You can only deploy a Multi-AZ file system in AWS Regions that have a minimum of three Availability Zones. Also supports HDD storage type</p> </li> <li> <p> <code>SINGLE_AZ_1</code> - (Default) Choose to deploy a file system that is configured for single AZ redundancy.</p> </li> <li> <p> <code>SINGLE_AZ_2</code> - The latest generation Single AZ file system. Specifies a file system that is configured for single AZ redundancy and supports HDD storage type.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/high-availability-multiAZ.html"> Availability and Durability: Single-AZ and Multi-AZ File Systems</a>.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>Required when <code>DeploymentType</code> is set to <code>MULTI_AZ_1</code>. This specifies the subnet in which you want the preferred file server to be located. For in-AWS applications, we recommend that you launch your clients in the same Availability Zone (AZ) as your preferred file server to reduce cross-AZ data transfer costs and minimize latency. </p>
    #[serde(rename = "PreferredSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfiguration>,
    /// <p>The throughput of an Amazon FSx file system, measured in megabytes per second, in 2 to the <i>n</i>th increments, between 2^3 (8) and 2^11 (2048).</p>
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: i64,
    /// <p>The preferred start time to perform weekly maintenance, formatted d:HH:MM in the UTC time zone, where d is the weekday number, from 1 through 7, beginning with Monday and ending with Sunday.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The data repository configuration object for Lustre file systems returned in the response of the <code>CreateFileSystem</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataRepositoryConfiguration {
    /// <p>Describes the file system's linked S3 data repository's <code>AutoImportPolicy</code>. The AutoImportPolicy configures how Amazon FSx keeps your file and directory listings up to date as you add or modify objects in your linked S3 bucket. <code>AutoImportPolicy</code> can have the following values:</p> <ul> <li> <p> <code>NONE</code> - (Default) AutoImport is off. Amazon FSx only updates file and directory listings from the linked S3 bucket when the file system is created. FSx does not update file and directory listings for any new or changed objects after choosing this option.</p> </li> <li> <p> <code>NEW</code> - AutoImport is on. Amazon FSx automatically imports directory listings of any new objects added to the linked S3 bucket that do not currently exist in the FSx file system. </p> </li> <li> <p> <code>NEW_CHANGED</code> - AutoImport is on. Amazon FSx automatically imports file and directory listings of any new objects added to the S3 bucket and any existing objects that are changed in the S3 bucket after you choose this option. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/autoimport-data-repo.html">Automatically import updates from your S3 bucket</a>.</p>
    #[serde(rename = "AutoImportPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_import_policy: Option<String>,
    /// <p>The export path to the Amazon S3 bucket (and prefix) that you are using to store new and changed Lustre file system files in S3.</p>
    #[serde(rename = "ExportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_path: Option<String>,
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<DataRepositoryFailureDetails>,
    /// <p>The import path to the Amazon S3 bucket (and optional prefix) that you're using as the data repository for your FSx for Lustre file system, for example <code>s3://import-bucket/optional-prefix</code>. If a prefix is specified after the Amazon S3 bucket name, only object keys with that prefix are loaded into the file system.</p>
    #[serde(rename = "ImportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_path: Option<String>,
    /// <p>For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. The maximum number of disks that a single file can be striped across is limited by the total number of disks that make up the file system.</p> <p>The default chunk size is 1,024 MiB (1 GiB) and can go as high as 512,000 MiB (500 GiB). Amazon S3 objects have a maximum size of 5 TB.</p>
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i64>,
    /// <p><p>Describes the state of the file system&#39;s S3 durable data repository, if it is configured with an S3 repository. The lifecycle can have the following values:</p> <ul> <li> <p> <code>CREATING</code> - The data repository configuration between the FSx file system and the linked S3 data repository is being created. The data repository is unavailable.</p> </li> <li> <p> <code>AVAILABLE</code> - The data repository is available for use.</p> </li> <li> <p> <code>MISCONFIGURED</code> - Amazon FSx cannot automatically import updates from the S3 bucket until the data repository configuration is corrected. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/troubleshooting.html#troubleshooting-misconfigured-data-repository">Troubleshooting a Misconfigured linked S3 bucket</a>. </p> </li> <li> <p> <code>UPDATING</code> - The data repository is undergoing a customer initiated update and availability may be impacted.</p> </li> </ul></p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
}

/// <p>Provides detailed information about the data respository if its <code>Lifecycle</code> is set to <code>MISCONFIGURED</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataRepositoryFailureDetails {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>A description of the data repository task. You use data repository tasks to perform bulk transfer operations between your Amazon FSx file system and its linked data repository.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataRepositoryTask {
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The time that Amazon FSx completed processing the task, populated after the task is complete.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Failure message describing why the task failed, it is populated only when <code>Lifecycle</code> is set to <code>FAILED</code>.</p>
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<DataRepositoryTaskFailureDetails>,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p><p>The lifecycle status of the data repository task, as follows:</p> <ul> <li> <p> <code>PENDING</code> - Amazon FSx has not started the task.</p> </li> <li> <p> <code>EXECUTING</code> - Amazon FSx is processing the task.</p> </li> <li> <p> <code>FAILED</code> - Amazon FSx was not able to complete the task. For example, there may be files the task failed to process. The <a>DataRepositoryTaskFailureDetails</a> property provides more information about task failures.</p> </li> <li> <p> <code>SUCCEEDED</code> - FSx completed the task successfully.</p> </li> <li> <p> <code>CANCELED</code> - Amazon FSx canceled the task and it did not complete.</p> </li> <li> <p> <code>CANCELING</code> - FSx is in process of canceling the task.</p> </li> </ul> <note> <p>You cannot delete an FSx for Lustre file system if there are data repository tasks for the file system in the <code>PENDING</code> or <code>EXECUTING</code> states. Please retry when the data repository task is finished (with a status of <code>CANCELED</code>, <code>SUCCEEDED</code>, or <code>FAILED</code>). You can use the DescribeDataRepositoryTask action to monitor the task status. Contact the FSx team if you need to delete your file system immediately.</p> </note></p>
    #[serde(rename = "Lifecycle")]
    pub lifecycle: String,
    /// <p>An array of paths on the Amazon FSx for Lustre file system that specify the data for the data repository task to process. For example, in an EXPORT_TO_REPOSITORY task, the paths specify which data to export to the linked data repository.</p> <p>(Default) If <code>Paths</code> is not specified, Amazon FSx uses the file system root directory.</p>
    #[serde(rename = "Paths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(rename = "Report")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<CompletionReport>,
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The time that Amazon FSx began processing the task.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Provides the status of the number of files that the task has processed successfully and failed to process.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DataRepositoryTaskStatus>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The system-generated, unique 17-digit ID of the data repository task.</p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// <p>The type of data repository task; EXPORT_TO_REPOSITORY is the only type currently supported.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Provides information about why a data repository task failed. Only populated when the task <code>Lifecycle</code> is set to <code>FAILED</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataRepositoryTaskFailureDetails {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>(Optional) An array of filter objects you can use to filter the response of data repository tasks you will see in the the response. You can filter the tasks returned in the response by one or more file system IDs, task lifecycles, and by task type. A filter object consists of a filter <code>Name</code>, and one or more <code>Values</code> for the filter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DataRepositoryTaskFilter {
    /// <p><p>Name of the task property to use in filtering the tasks returned in the response.</p> <ul> <li> <p>Use <code>file-system-id</code> to retrieve data repository tasks for specific file systems.</p> </li> <li> <p>Use <code>task-lifecycle</code> to retrieve data repository tasks with one or more specific lifecycle states, as follows: CANCELED, EXECUTING, FAILED, PENDING, and SUCCEEDED.</p> </li> </ul></p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Use Values to include the specific file system IDs and task lifecycle states for the filters you are using.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Provides the task status showing a running total of the total number of files to be processed, the number successfully processed, and the number of files the task failed to process.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataRepositoryTaskStatus {
    /// <p>A running total of the number of files that the task failed to process.</p>
    #[serde(rename = "FailedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    /// <p>The time at which the task status was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A running total of the number of files that the task has successfully processed.</p>
    #[serde(rename = "SucceededCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i64>,
    /// <p>The total number of files that the task will process. While a task is executing, the sum of <code>SucceededCount</code> plus <code>FailedCount</code> may not equal <code>TotalCount</code>. When the task is complete, <code>TotalCount</code> equals the sum of <code>SucceededCount</code> plus <code>FailedCount</code>.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>The request object for <code>DeleteBackup</code> operation.</p>
/// see [Fsx::delete_backup]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupRequest {
    /// <p>The ID of the backup you want to delete.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent deletion. This is automatically filled on your behalf when using the AWS CLI or SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
}

/// <p>The response object for <code>DeleteBackup</code> operation.</p>
/// see [Fsx::delete_backup]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBackupResponse {
    /// <p>The ID of the backup deleted.</p>
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// <p>The lifecycle of the backup. Should be <code>DELETED</code>.</p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
}

/// <p>The configuration object for the Amazon FSx for Lustre file system being deleted in the <code>DeleteFileSystem</code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileSystemLustreConfiguration {
    /// <p>Use if <code>SkipFinalBackup</code> is set to <code>false</code>, and you want to apply an array of tags to the final backup. If you have set the file system property <code>CopyTagsToBackups</code> to true, and you specify one or more <code>FinalBackupTags</code> when deleting a file system, Amazon FSx will not copy any existing file system tags to the backup.</p>
    #[serde(rename = "FinalBackupTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
    /// <p>Set <code>SkipFinalBackup</code> to false if you want to take a final backup of the file system you are deleting. By default, Amazon FSx will not take a final backup on your behalf when the <code>DeleteFileSystem</code> operation is invoked. (Default = true)</p>
    #[serde(rename = "SkipFinalBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_backup: Option<bool>,
}

/// <p>The response object for the Amazon FSx for Lustre file system being deleted in the <code>DeleteFileSystem</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFileSystemLustreResponse {
    /// <p>The ID of the final backup for this file system.</p>
    #[serde(rename = "FinalBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_id: Option<String>,
    /// <p>The set of tags applied to the final backup.</p>
    #[serde(rename = "FinalBackupTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
}

/// <p>The request object for <code>DeleteFileSystem</code> operation.</p>
/// see [Fsx::delete_file_system]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileSystemRequest {
    /// <p>A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent deletion. This is automatically filled on your behalf when using the AWS CLI or SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The ID of the file system you want to delete.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<DeleteFileSystemLustreConfiguration>,
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<DeleteFileSystemWindowsConfiguration>,
}

/// <p>The response object for the <code>DeleteFileSystem</code> operation.</p>
/// see [Fsx::delete_file_system]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFileSystemResponse {
    /// <p>The ID of the file system being deleted.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>The file system lifecycle for the deletion request. Should be <code>DELETING</code>.</p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LustreResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_response: Option<DeleteFileSystemLustreResponse>,
    #[serde(rename = "WindowsResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_response: Option<DeleteFileSystemWindowsResponse>,
}

/// <p>The configuration object for the Microsoft Windows file system used in the <code>DeleteFileSystem</code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileSystemWindowsConfiguration {
    /// <p>A set of tags for your final backup.</p>
    #[serde(rename = "FinalBackupTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
    /// <p>By default, Amazon FSx for Windows takes a final backup on your behalf when the <code>DeleteFileSystem</code> operation is invoked. Doing this helps protect you from data loss, and we highly recommend taking the final backup. If you want to skip this backup, use this flag to do so.</p>
    #[serde(rename = "SkipFinalBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_backup: Option<bool>,
}

/// <p>The response object for the Microsoft Windows file system used in the <code>DeleteFileSystem</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFileSystemWindowsResponse {
    /// <p>The ID of the final backup for this file system.</p>
    #[serde(rename = "FinalBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_id: Option<String>,
    /// <p>The set of tags applied to the final backup.</p>
    #[serde(rename = "FinalBackupTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
}

/// <p>The request object for <code>DescribeBackups</code> operation.</p>
/// see [Fsx::describe_backups]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBackupsRequest {
    /// <p>IDs of the backups you want to retrieve (String). This overrides any filters. If any IDs are not found, BackupNotFound will be thrown.</p>
    #[serde(rename = "BackupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_ids: Option<Vec<String>>,
    /// <p>Filters structure. Supported names are file-system-id and backup-type.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Maximum number of backups to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Opaque pagination token returned from a previous <code>DescribeBackups</code> operation (String). If a token present, the action continues the list from where the returning call left off.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl Paged for DescribeBackupsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedRequest for DescribeBackupsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.next_token = key;
    }
}

/// <p>Response object for <code>DescribeBackups</code> operation.</p>
/// see [Fsx::describe_backups]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBackupsResponse {
    /// <p>Any array of backups.</p>
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    /// <p>This is present if there are more backups than returned in the response (String). You can use the <code>NextToken</code> value in the later request to fetch the backups. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl Paged for DescribeBackupsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedOutput for DescribeBackupsResponse {
    type Item = Backup;

    fn into_pagination_page(self) -> Vec<Backup> {
        self.backups.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Fsx::describe_data_repository_tasks]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataRepositoryTasksRequest {
    /// <p>(Optional) You can use filters to narrow the <code>DescribeDataRepositoryTasks</code> response to include just tasks for specific file systems, or tasks in a specific lifecycle state.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DataRepositoryTaskFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>(Optional) IDs of the tasks whose descriptions you want to retrieve (String).</p>
    #[serde(rename = "TaskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
}

/// see [Fsx::describe_data_repository_tasks]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDataRepositoryTasksResponse {
    /// <p>The collection of data repository task descriptions returned.</p>
    #[serde(rename = "DataRepositoryTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_tasks: Option<Vec<DataRepositoryTask>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The request object for <code>DescribeFileSystemAliases</code> operation.</p>
/// see [Fsx::describe_file_system_aliases]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFileSystemAliasesRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The ID of the file system to return the associated DNS aliases for (String).</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>Maximum number of DNS aliases to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Opaque pagination token returned from a previous <code>DescribeFileSystemAliases</code> operation (String). If a token is included in the request, the action continues the list from where the previous returning call left off.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The response object for <code>DescribeFileSystemAliases</code> operation.</p>
/// see [Fsx::describe_file_system_aliases]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFileSystemAliasesResponse {
    /// <p>An array of one or more DNS aliases currently associated with the specified file system.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    /// <p>Present if there are more DNS aliases than returned in the response (String). You can use the <code>NextToken</code> value in a later request to fetch additional descriptions. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The request object for <code>DescribeFileSystems</code> operation.</p>
/// see [Fsx::describe_file_systems]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFileSystemsRequest {
    /// <p>IDs of the file systems whose descriptions you want to retrieve (String).</p>
    #[serde(rename = "FileSystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_ids: Option<Vec<String>>,
    /// <p>Maximum number of file systems to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Opaque pagination token returned from a previous <code>DescribeFileSystems</code> operation (String). If a token present, the action continues the list from where the returning call left off.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl Paged for DescribeFileSystemsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedRequest for DescribeFileSystemsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.next_token = key;
    }
}

/// <p>The response object for <code>DescribeFileSystems</code> operation.</p>
/// see [Fsx::describe_file_systems]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFileSystemsResponse {
    /// <p>An array of file system descriptions.</p>
    #[serde(rename = "FileSystems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<Vec<FileSystem>>,
    /// <p>Present if there are more file systems than returned in the response (String). You can use the <code>NextToken</code> value in the later request to fetch the descriptions. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl Paged for DescribeFileSystemsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedOutput for DescribeFileSystemsResponse {
    type Item = FileSystem;

    fn into_pagination_page(self) -> Vec<FileSystem> {
        self.file_systems.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// <p>The request object of DNS aliases to disassociate from an Amazon FSx for Windows File Server file system.</p>
/// see [Fsx::disassociate_file_system_aliases]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateFileSystemAliasesRequest {
    /// <p>An array of one or more DNS alias names to disassociate, or remove, from the file system.</p>
    #[serde(rename = "Aliases")]
    pub aliases: Vec<String>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Specifies the file system from which to disassociate the DNS aliases.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

/// <p>The system generated response showing the DNS aliases that Amazon FSx is attempting to disassociate from the file system. Use the API operation to monitor the status of the aliases Amazon FSx is removing from the file system.</p>
/// see [Fsx::disassociate_file_system_aliases]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateFileSystemAliasesResponse {
    /// <p>An array of one or more DNS aliases that Amazon FSx is attempting to disassociate from the file system.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
}

/// <p>A description of a specific Amazon FSx file system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystem {
    /// <p>A list of administrative actions for the file system that are in process or waiting to be processed. Administrative actions describe changes to the Windows file system that you have initiated using the <code>UpdateFileSystem</code> action. </p>
    #[serde(rename = "AdministrativeActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_actions: Option<Vec<AdministrativeAction>>,
    /// <p>The time that the file system was created, in seconds (since 1970-01-01T00:00:00Z), also known as Unix time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The DNS name for the file system.</p>
    #[serde(rename = "DNSName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FileSystemFailureDetails>,
    /// <p>The system-generated, unique 17-digit ID of the file system.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>The type of Amazon FSx file system, either <code>LUSTRE</code> or <code>WINDOWS</code>.</p>
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// <p>The ID of the AWS Key Management Service (AWS KMS) key used to encrypt the file system's data for Amazon FSx for Windows File Server file systems and persistent Amazon FSx for Lustre file systems at rest. In either case, if not specified, the Amazon FSx managed key is used. The scratch Amazon FSx for Lustre file systems are always encrypted at rest using Amazon FSx managed keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html">Encrypt</a> in the <i>AWS Key Management Service API Reference</i>.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p><p>The lifecycle status of the file system, following are the possible values and what they mean:</p> <ul> <li> <p> <code>AVAILABLE</code> - The file system is in a healthy state, and is reachable and available for use.</p> </li> <li> <p> <code>CREATING</code> - Amazon FSx is creating the new file system.</p> </li> <li> <p> <code>DELETING</code> - Amazon FSx is deleting an existing file system.</p> </li> <li> <p> <code>FAILED</code> - An existing file system has experienced an unrecoverable failure. When creating a new file system, Amazon FSx was unable to create the file system.</p> </li> <li> <p> <code>MISCONFIGURED</code> indicates that the file system is in a failed but recoverable state.</p> </li> <li> <p> <code>UPDATING</code> indicates that the file system is undergoing a customer initiated update.</p> </li> </ul></p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<LustreFileSystemConfiguration>,
    /// <p>The IDs of the elastic network interface from which a specific file system is accessible. The elastic network interface is automatically created in the same VPC that the Amazon FSx file system was created in. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html">Elastic Network Interfaces</a> in the <i>Amazon EC2 User Guide.</i> </p> <p>For an Amazon FSx for Windows File Server file system, you can have one network interface ID. For an Amazon FSx for Lustre file system, you can have more than one.</p>
    #[serde(rename = "NetworkInterfaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    /// <p>The AWS account that created the file system. If the file system was created by an AWS Identity and Access Management (IAM) user, the AWS account to which the IAM user belongs is the owner.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the file system resource.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The storage capacity of the file system in gibibytes (GiB).</p>
    #[serde(rename = "StorageCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i64>,
    /// <p>The storage type of the file system. Valid values are <code>SSD</code> and <code>HDD</code>. If set to <code>SSD</code>, the file system uses solid state drive storage. If set to <code>HDD</code>, the file system uses hard disk drive storage. </p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// <p>Specifies the IDs of the subnets that the file system is accessible from. For Windows <code>MULTI_AZ_1</code> file system deployment type, there are two subnet IDs, one for the preferred file server and one for the standby file server. The preferred file server subnet identified in the <code>PreferredSubnetID</code> property. All other file systems have only one subnet ID.</p> <p>For Lustre file systems, and Single-AZ Windows file systems, this is the ID of the subnet that contains the endpoint for the file system. For <code>MULTI_AZ_1</code> Windows file systems, the endpoint for the file system is available in the <code>PreferredSubnetID</code>.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The tags to associate with the file system. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tagging Your Amazon EC2 Resources</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The ID of the primary VPC for the file system.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The configuration for this Microsoft Windows file system.</p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsFileSystemConfiguration>,
}

/// <p>A structure providing details of any failures that occur when creating the file system has failed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystemFailureDetails {
    /// <p>A message describing any failures that occurred during file system creation.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>A filter used to restrict the results of describe calls. You can use multiple filters to return results that meet all applied filter requirements.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name for this filter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The values of the filter. These are all the values for any of the applied filters.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The request object for <code>ListTagsForResource</code> operation.</p>
/// see [Fsx::list_tags_for_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>Maximum number of tags to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon FSx returns is the minimum of the <code>MaxResults</code> parameter specified in the request and the service's internal maximum number of items per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Opaque pagination token returned from a previous <code>ListTagsForResource</code> operation (String). If a token present, the action continues the list from where the returning call left off.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the Amazon FSx resource that will have its tags listed.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

impl Paged for ListTagsForResourceRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedRequest for ListTagsForResourceRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.next_token = key;
    }
}

/// <p>The response object for <code>ListTagsForResource</code> operation.</p>
/// see [Fsx::list_tags_for_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>This is present if there are more tags than returned in the response (String). You can use the <code>NextToken</code> value in the later request to fetch the tags. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags on the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl Paged for ListTagsForResourceResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedOutput for ListTagsForResourceResponse {
    type Item = Tag;

    fn into_pagination_page(self) -> Vec<Tag> {
        self.tags.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// <p>The configuration for the Amazon FSx for Lustre file system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LustreFileSystemConfiguration {
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>A boolean flag indicating whether tags on the file system should be copied to backups. If it's set to true, all tags on the file system are copied to all automatic backups and any user-initiated backups where the user doesn't specify any tags. If this value is true, and you specify one or more tags, only the specified tags are copied to backups. If you specify one or more tags when creating a user-initiated backup, no tags are copied from the file system, regardless of this value. (Default = false)</p>
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DataRepositoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_configuration: Option<DataRepositoryConfiguration>,
    /// <p>The deployment type of the FSX for Lustre file system. <i>Scratch deployment type</i> is designed for temporary storage and shorter-term processing of data.</p> <p> <code>SCRATCH_1</code> and <code>SCRATCH_2</code> deployment types are best suited for when you need temporary storage and shorter-term processing of data. The <code>SCRATCH_2</code> deployment type provides in-transit encryption of data and higher burst throughput capacity than <code>SCRATCH_1</code>.</p> <p>The <code>PERSISTENT_1</code> deployment type is used for longer-term storage and workloads and encryption of data in transit. To learn more about deployment types, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/lustre-deployment-types.html"> FSx for Lustre Deployment Options</a>. (Default = <code>SCRATCH_1</code>)</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>The type of drive cache used by PERSISTENT_1 file systems that are provisioned with HDD storage devices. This parameter is required when storage type is HDD. Set to <code>READ</code>, improve the performance for frequently accessed files and allows 20% of the total storage capacity of the file system to be cached. </p> <p>This parameter is required when <code>StorageType</code> is set to HDD.</p>
    #[serde(rename = "DriveCacheType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_cache_type: Option<String>,
    /// <p>You use the <code>MountName</code> value when mounting the file system.</p> <p>For the <code>SCRATCH_1</code> deployment type, this value is always "<code>fsx</code>". For <code>SCRATCH_2</code> and <code>PERSISTENT_1</code> deployment types, this value is a string that is unique within an AWS Region. </p>
    #[serde(rename = "MountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_name: Option<String>,
    /// <p> Per unit storage throughput represents the megabytes per second of read or write throughput per 1 tebibyte of storage provisioned. File system throughput capacity is equal to Storage capacity (TiB) * PerUnitStorageThroughput (MB/s/TiB). This option is only valid for <code>PERSISTENT_1</code> deployment types. </p> <p>Valid values for SSD storage: 50, 100, 200. Valid values for HDD storage: 12, 40. </p>
    #[serde(rename = "PerUnitStorageThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_storage_throughput: Option<i64>,
    /// <p>The preferred start time to perform weekly maintenance, formatted d:HH:MM in the UTC time zone. d is the weekday number, from 1 through 7, beginning with Monday and ending with Sunday.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The configuration of the self-managed Microsoft Active Directory (AD) directory to which the Windows File Server instance is joined.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SelfManagedActiveDirectoryAttributes {
    /// <p>A list of up to two IP addresses of DNS servers or domain controllers in the self-managed AD directory.</p>
    #[serde(rename = "DnsIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    /// <p>The fully qualified domain name of the self-managed AD directory.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The name of the domain group whose members have administrative privileges for the FSx file system.</p>
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<String>,
    /// <p>The fully qualified distinguished name of the organizational unit within the self-managed AD directory to which the Windows File Server instance is joined.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    /// <p>The user name for the service account on your self-managed AD domain that FSx uses to join to your AD domain.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>The configuration that Amazon FSx uses to join the Windows File Server instance to your self-managed (including on-premises) Microsoft Active Directory (AD) directory.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SelfManagedActiveDirectoryConfiguration {
    /// <p><p>A list of up to two IP addresses of DNS servers or domain controllers in the self-managed AD directory. The IP addresses need to be either in the same VPC CIDR range as the one in which your Amazon FSx file system is being created, or in the private IP version 4 (IPv4) address ranges, as specified in <a href="http://www.faqs.org/rfcs/rfc1918.html">RFC 1918</a>:</p> <ul> <li> <p>10.0.0.0 - 10.255.255.255 (10/8 prefix)</p> </li> <li> <p>172.16.0.0 - 172.31.255.255 (172.16/12 prefix)</p> </li> <li> <p>192.168.0.0 - 192.168.255.255 (192.168/16 prefix)</p> </li> </ul></p>
    #[serde(rename = "DnsIps")]
    pub dns_ips: Vec<String>,
    /// <p>The fully qualified domain name of the self-managed AD directory, such as <code>corp.example.com</code>.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>(Optional) The name of the domain group whose members are granted administrative privileges for the file system. Administrative privileges include taking ownership of files and folders, setting audit controls (audit ACLs) on files and folders, and administering the file system remotely by using the FSx Remote PowerShell. The group that you specify must already exist in your domain. If you don't provide one, your AD domain's Domain Admins group is used.</p>
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<String>,
    /// <p><p>(Optional) The fully qualified distinguished name of the organizational unit within your self-managed AD directory that the Windows File Server instance will join. Amazon FSx only accepts OU as the direct parent of the file system. An example is <code>OU=FSx,DC=yourdomain,DC=corp,DC=com</code>. To learn more, see <a href="https://tools.ietf.org/html/rfc2253">RFC 2253</a>. If none is provided, the FSx file system is created in the default location of your self-managed AD directory. </p> <important> <p>Only Organizational Unit (OU) objects can be the direct parent of the file system that you&#39;re creating.</p> </important></p>
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    /// <p>The password for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The user name for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain. This account must have the permission to join computers to the domain in the organizational unit provided in <code>OrganizationalUnitDistinguishedName</code>, or in the default location of your AD domain.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

/// <p>The configuration that Amazon FSx uses to join the Windows File Server instance to a self-managed Microsoft Active Directory (AD) directory.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SelfManagedActiveDirectoryConfigurationUpdates {
    /// <p>A list of up to two IP addresses of DNS servers or domain controllers in the self-managed AD directory.</p>
    #[serde(rename = "DnsIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    /// <p>The password for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The user name for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain. This account must have the permission to join computers to the domain in the organizational unit provided in <code>OrganizationalUnitDistinguishedName</code>.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Specifies a key-value pair for a resource tag.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A value that specifies the <code>TagKey</code>, the name of the tag. Tag keys must be unique for the resource to which they are attached.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>A value that specifies the <code>TagValue</code>, the value assigned to the corresponding tag key. Tag values can be null and don't have to be unique in a tag set. For example, you can have a key-value pair in a tag set of <code>finances : April</code> and also of <code>payroll : April</code>.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>The request object for the <code>TagResource</code> operation.</p>
/// see [Fsx::tag_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the Amazon FSx resource that you want to tag.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of tags for the resource. If a tag with a given key already exists, the value is replaced by the one specified in this parameter.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>The response object for the <code>TagResource</code> operation.</p>
/// see [Fsx::tag_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>The request object for <code>UntagResource</code> action.</p>
/// see [Fsx::untag_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the Amazon FSx resource to untag.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of keys of tags on the resource to untag. In case the tag key doesn't exist, the call will still succeed to be idempotent.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The response object for <code>UntagResource</code> action.</p>
/// see [Fsx::untag_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>The configuration object for Amazon FSx for Lustre file systems used in the <code>UpdateFileSystem</code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFileSystemLustreConfiguration {
    /// <p> (Optional) When you create your file system, your existing S3 objects appear as file and directory listings. Use this property to choose how Amazon FSx keeps your file and directory listing up to date as you add or modify objects in your linked S3 bucket. <code>AutoImportPolicy</code> can have the following values:</p> <ul> <li> <p> <code>NONE</code> - (Default) AutoImport is off. Amazon FSx only updates file and directory listings from the linked S3 bucket when the file system is created. FSx does not update the file and directory listing for any new or changed objects after choosing this option.</p> </li> <li> <p> <code>NEW</code> - AutoImport is on. Amazon FSx automatically imports directory listings of any new objects added to the linked S3 bucket that do not currently exist in the FSx file system. </p> </li> <li> <p> <code>NEW_CHANGED</code> - AutoImport is on. Amazon FSx automatically imports file and directory listings of any new objects added to the S3 bucket and any existing objects that are changed in the S3 bucket after you choose this option. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/autoimport-data-repo.html">Automatically import updates from your S3 bucket</a>.</p>
    #[serde(rename = "AutoImportPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_import_policy: Option<String>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    /// <p>(Optional) The preferred start time to perform weekly maintenance, formatted d:HH:MM in the UTC time zone. d is the weekday number, from 1 through 7, beginning with Monday and ending with Sunday.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The request object for the <code>UpdateFileSystem</code> operation.</p>
/// see [Fsx::update_file_system]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFileSystemRequest {
    /// <p>A string of up to 64 ASCII characters that Amazon FSx uses to ensure idempotent updates. This string is automatically filled on your behalf when you use the AWS Command Line Interface (AWS CLI) or an AWS SDK.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Identifies the file system that you are updating.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "LustreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<UpdateFileSystemLustreConfiguration>,
    /// <p>Use this parameter to increase the storage capacity of an Amazon FSx file system. Specifies the storage capacity target value, GiB, to increase the storage capacity for the file system that you're updating. You cannot make a storage capacity increase request if there is an existing storage capacity increase request in progress.</p> <p>For Windows file systems, the storage capacity target value must be at least 10 percent (%) greater than the current storage capacity value. In order to increase storage capacity, the file system must have at least 16 MB/s of throughput capacity.</p> <p>For Lustre file systems, the storage capacity target value can be the following:</p> <ul> <li> <p>For <code>SCRATCH_2</code> and <code>PERSISTENT_1 SSD</code> deployment types, valid values are in multiples of 2400 GiB. The value must be greater than the current storage capacity.</p> </li> <li> <p>For <code>PERSISTENT HDD</code> file systems, valid values are multiples of 6000 GiB for 12 MB/s/TiB file systems and multiples of 1800 GiB for 40 MB/s/TiB file systems. The values must be greater than the current storage capacity.</p> </li> <li> <p>For <code>SCRATCH_1</code> file systems, you cannot increase the storage capacity.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-storage-capacity.html">Managing storage capacity</a> in the <i>Amazon FSx for Windows File Server User Guide</i> and <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/managing-storage-capacity.html">Managing storage and throughput capacity</a> in the <i>Amazon FSx for Lustre User Guide</i>.</p>
    #[serde(rename = "StorageCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i64>,
    /// <p>The configuration updates for an Amazon FSx for Windows File Server file system.</p>
    #[serde(rename = "WindowsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<UpdateFileSystemWindowsConfiguration>,
}

/// <p>The response object for the <code>UpdateFileSystem</code> operation.</p>
/// see [Fsx::update_file_system]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFileSystemResponse {
    /// <p>A description of the file system that was updated.</p>
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

/// <p>Updates the configuration for an existing Amazon FSx for Windows File Server file system. Amazon FSx only overwrites existing properties with non-null values provided in the request.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFileSystemWindowsConfiguration {
    /// <p>The number of days to retain automatic daily backups. Setting this to zero (0) disables automatic daily backups. You can retain automatic daily backups for a maximum of 90 days. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/using-backups.html#automatic-backups">Working with Automatic Daily Backups</a>.</p>
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>The preferred time to start the daily automatic backup, in the UTC time zone, for example, <code>02:00</code> </p>
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    /// <p>The configuration Amazon FSx uses to join the Windows File Server instance to the self-managed Microsoft AD directory. You cannot make a self-managed Microsoft AD update request if there is an existing self-managed Microsoft AD update request in progress.</p>
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfigurationUpdates>,
    /// <p>Sets the target value for a file system's throughput capacity, in MB/s, that you are updating the file system to. Valid values are 8, 16, 32, 64, 128, 256, 512, 1024, 2048. You cannot make a throughput capacity update request if there is an existing throughput capacity update request in progress. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-throughput-capacity.html">Managing Throughput Capacity</a>.</p>
    #[serde(rename = "ThroughputCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i64>,
    /// <p>The preferred start time to perform weekly maintenance, formatted d:HH:MM in the UTC time zone. Where d is the weekday number, from 1 through 7, with 1 = Monday and 7 = Sunday.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// <p>The configuration for this Microsoft Windows file system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WindowsFileSystemConfiguration {
    /// <p>The ID for an existing Microsoft Active Directory instance that the file system should join when it's created.</p>
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    /// <p>The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days.</p>
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i64>,
    /// <p>A boolean flag indicating whether tags on the file system should be copied to backups. This value defaults to false. If it's set to true, all tags on the file system are copied to all automatic backups and any user-initiated backups where the user doesn't specify any tags. If this value is true, and you specify one or more tags, only the specified tags are copied to backups. If you specify one or more tags when creating a user-initiated backup, no tags are copied from the file system, regardless of this value.</p>
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    /// <p>The preferred time to take daily automatic backups, in the UTC time zone.</p>
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    /// <p>Specifies the file system deployment type, valid values are the following:</p> <ul> <li> <p> <code>MULTI_AZ_1</code> - Specifies a high availability file system that is configured for Multi-AZ redundancy to tolerate temporary Availability Zone (AZ) unavailability, and supports SSD and HDD storage.</p> </li> <li> <p> <code>SINGLE_AZ_1</code> - (Default) Specifies a file system that is configured for single AZ redundancy, only supports SSD storage.</p> </li> <li> <p> <code>SINGLE_AZ_2</code> - Latest generation Single AZ file system. Specifies a file system that is configured for single AZ redundancy and supports SSD and HDD storage.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/high-availability-multiAZ.html">Single-AZ and Multi-AZ File Systems</a>.</p>
    #[serde(rename = "DeploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    /// <p>The list of maintenance operations in progress for this file system.</p>
    #[serde(rename = "MaintenanceOperationsInProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_operations_in_progress: Option<Vec<String>>,
    /// <p>For <code>MULTI_AZ_1</code> deployment types, the IP address of the primary, or preferred, file server.</p> <p>Use this IP address when mounting the file system on Linux SMB clients or Windows SMB clients that are not joined to a Microsoft Active Directory. Applicable for all Windows file system deployment types. This IP address is temporarily unavailable when the file system is undergoing maintenance. For Linux and Windows SMB clients that are joined to an Active Directory, use the file system's DNSName instead. For more information on mapping and mounting file shares, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/accessing-file-shares.html">Accessing File Shares</a>.</p>
    #[serde(rename = "PreferredFileServerIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_file_server_ip: Option<String>,
    /// <p>For <code>MULTI_AZ_1</code> deployment types, it specifies the ID of the subnet where the preferred file server is located. Must be one of the two subnet IDs specified in <code>SubnetIds</code> property. Amazon FSx serves traffic from this subnet except in the event of a failover to the secondary file server.</p> <p>For <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> deployment types, this value is the same as that for <code>SubnetIDs</code>. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/high-availability-multiAZ.html#single-multi-az-resources">Availability and Durability: Single-AZ and Multi-AZ File Systems</a> </p>
    #[serde(rename = "PreferredSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    /// <p>For <code>MULTI_AZ_1</code> deployment types, use this endpoint when performing administrative tasks on the file system using Amazon FSx Remote PowerShell.</p> <p>For <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> deployment types, this is the DNS name of the file system.</p> <p>This endpoint is temporarily unavailable when the file system is undergoing maintenance.</p>
    #[serde(rename = "RemoteAdministrationEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_administration_endpoint: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryAttributes>,
    /// <p>The throughput of an Amazon FSx file system, measured in megabytes per second.</p>
    #[serde(rename = "ThroughputCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i64>,
    /// <p>The preferred start time to perform weekly maintenance, formatted d:HH:MM in the UTC time zone. d is the weekday number, from 1 through 7, beginning with Monday and ending with Sunday.</p>
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

/// Errors returned by AssociateFileSystemAliases
#[derive(Debug, PartialEq)]
pub enum AssociateFileSystemAliasesError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl AssociateFileSystemAliasesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateFileSystemAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(AssociateFileSystemAliasesError::BadRequest(
                        err.msg,
                    ))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(
                        AssociateFileSystemAliasesError::FileSystemNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        AssociateFileSystemAliasesError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateFileSystemAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateFileSystemAliasesError::BadRequest(ref cause) => write!(f, "{}", cause),
            AssociateFileSystemAliasesError::FileSystemNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateFileSystemAliasesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateFileSystemAliasesError {}
/// Errors returned by CancelDataRepositoryTask
#[derive(Debug, PartialEq)]
pub enum CancelDataRepositoryTaskError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The data repository task could not be canceled because the task has already ended.</p>
    DataRepositoryTaskEnded(String),
    /// <p>The data repository task or tasks you specified could not be found.</p>
    DataRepositoryTaskNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The requested operation is not supported for this resource or API.</p>
    UnsupportedOperation(String),
}

impl CancelDataRepositoryTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelDataRepositoryTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(CancelDataRepositoryTaskError::BadRequest(err.msg))
                }
                "DataRepositoryTaskEnded" => {
                    return RusotoError::Service(
                        CancelDataRepositoryTaskError::DataRepositoryTaskEnded(err.msg),
                    )
                }
                "DataRepositoryTaskNotFound" => {
                    return RusotoError::Service(
                        CancelDataRepositoryTaskError::DataRepositoryTaskNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        CancelDataRepositoryTaskError::InternalServerError(err.msg),
                    )
                }
                "UnsupportedOperation" => {
                    return RusotoError::Service(
                        CancelDataRepositoryTaskError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelDataRepositoryTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelDataRepositoryTaskError::BadRequest(ref cause) => write!(f, "{}", cause),
            CancelDataRepositoryTaskError::DataRepositoryTaskEnded(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelDataRepositoryTaskError::DataRepositoryTaskNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelDataRepositoryTaskError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CancelDataRepositoryTaskError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CancelDataRepositoryTaskError {}
/// Errors returned by CreateBackup
#[derive(Debug, PartialEq)]
pub enum CreateBackupError {
    /// <p>Another backup is already under way. Wait for completion before initiating additional backups of this file system.</p>
    BackupInProgress(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
    /// <p>The requested operation is not supported for this resource or API.</p>
    UnsupportedOperation(String),
}

impl CreateBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BackupInProgress" => {
                    return RusotoError::Service(CreateBackupError::BackupInProgress(err.msg))
                }
                "BadRequest" => {
                    return RusotoError::Service(CreateBackupError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(CreateBackupError::FileSystemNotFound(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(CreateBackupError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateBackupError::InternalServerError(err.msg))
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(CreateBackupError::ServiceLimitExceeded(err.msg))
                }
                "UnsupportedOperation" => {
                    return RusotoError::Service(CreateBackupError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBackupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBackupError::BackupInProgress(ref cause) => write!(f, "{}", cause),
            CreateBackupError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateBackupError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            CreateBackupError::IncompatibleParameterError(ref cause) => write!(f, "{}", cause),
            CreateBackupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateBackupError::ServiceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBackupError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBackupError {}
/// Errors returned by CreateDataRepositoryTask
#[derive(Debug, PartialEq)]
pub enum CreateDataRepositoryTaskError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>An existing data repository task is currently executing on the file system. Wait until the existing task has completed, then create the new task.</p>
    DataRepositoryTaskExecuting(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
    /// <p>The requested operation is not supported for this resource or API.</p>
    UnsupportedOperation(String),
}

impl CreateDataRepositoryTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataRepositoryTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(CreateDataRepositoryTaskError::BadRequest(err.msg))
                }
                "DataRepositoryTaskExecuting" => {
                    return RusotoError::Service(
                        CreateDataRepositoryTaskError::DataRepositoryTaskExecuting(err.msg),
                    )
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(CreateDataRepositoryTaskError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(
                        CreateDataRepositoryTaskError::IncompatibleParameterError(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        CreateDataRepositoryTaskError::InternalServerError(err.msg),
                    )
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateDataRepositoryTaskError::ServiceLimitExceeded(err.msg),
                    )
                }
                "UnsupportedOperation" => {
                    return RusotoError::Service(
                        CreateDataRepositoryTaskError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDataRepositoryTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataRepositoryTaskError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDataRepositoryTaskError::DataRepositoryTaskExecuting(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDataRepositoryTaskError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            CreateDataRepositoryTaskError::IncompatibleParameterError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDataRepositoryTaskError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateDataRepositoryTaskError::ServiceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDataRepositoryTaskError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDataRepositoryTaskError {}
/// Errors returned by CreateFileSystem
#[derive(Debug, PartialEq)]
pub enum CreateFileSystemError {
    /// <p>An Active Directory error.</p>
    ActiveDirectoryError(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The path provided for data repository export isn't valid.</p>
    InvalidExportPath(String),
    /// <p>The path provided for data repository import isn't valid.</p>
    InvalidImportPath(String),
    /// <p>One or more network settings specified in the request are invalid. <code>InvalidVpcId</code> means that the ID passed for the virtual private cloud (VPC) is invalid. <code>InvalidSubnetIds</code> returns the list of IDs for subnets that are either invalid or not part of the VPC specified. <code>InvalidSecurityGroupIds</code> returns the list of IDs for security groups that are either invalid or not part of the VPC specified.</p>
    InvalidNetworkSettings(String),
    /// <p>An invalid value for <code>PerUnitStorageThroughput</code> was provided. Please create your file system again, using a valid value.</p>
    InvalidPerUnitStorageThroughput(String),
    /// <p>A file system configuration is required for this operation.</p>
    MissingFileSystemConfiguration(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
}

impl CreateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActiveDirectoryError" => {
                    return RusotoError::Service(CreateFileSystemError::ActiveDirectoryError(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(CreateFileSystemError::BadRequest(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(CreateFileSystemError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidExportPath" => {
                    return RusotoError::Service(CreateFileSystemError::InvalidExportPath(err.msg))
                }
                "InvalidImportPath" => {
                    return RusotoError::Service(CreateFileSystemError::InvalidImportPath(err.msg))
                }
                "InvalidNetworkSettings" => {
                    return RusotoError::Service(CreateFileSystemError::InvalidNetworkSettings(
                        err.msg,
                    ))
                }
                "InvalidPerUnitStorageThroughput" => {
                    return RusotoError::Service(
                        CreateFileSystemError::InvalidPerUnitStorageThroughput(err.msg),
                    )
                }
                "MissingFileSystemConfiguration" => {
                    return RusotoError::Service(
                        CreateFileSystemError::MissingFileSystemConfiguration(err.msg),
                    )
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(CreateFileSystemError::ServiceLimitExceeded(
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
impl fmt::Display for CreateFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFileSystemError::ActiveDirectoryError(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::IncompatibleParameterError(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::InvalidExportPath(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::InvalidImportPath(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::InvalidNetworkSettings(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::InvalidPerUnitStorageThroughput(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemError::MissingFileSystemConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemError::ServiceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFileSystemError {}
/// Errors returned by CreateFileSystemFromBackup
#[derive(Debug, PartialEq)]
pub enum CreateFileSystemFromBackupError {
    /// <p>An Active Directory error.</p>
    ActiveDirectoryError(String),
    /// <p>No Amazon FSx backups were found based upon the supplied parameters.</p>
    BackupNotFound(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>One or more network settings specified in the request are invalid. <code>InvalidVpcId</code> means that the ID passed for the virtual private cloud (VPC) is invalid. <code>InvalidSubnetIds</code> returns the list of IDs for subnets that are either invalid or not part of the VPC specified. <code>InvalidSecurityGroupIds</code> returns the list of IDs for security groups that are either invalid or not part of the VPC specified.</p>
    InvalidNetworkSettings(String),
    /// <p>An invalid value for <code>PerUnitStorageThroughput</code> was provided. Please create your file system again, using a valid value.</p>
    InvalidPerUnitStorageThroughput(String),
    /// <p>A file system configuration is required for this operation.</p>
    MissingFileSystemConfiguration(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
}

impl CreateFileSystemFromBackupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateFileSystemFromBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActiveDirectoryError" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::ActiveDirectoryError(err.msg),
                    )
                }
                "BackupNotFound" => {
                    return RusotoError::Service(CreateFileSystemFromBackupError::BackupNotFound(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(CreateFileSystemFromBackupError::BadRequest(
                        err.msg,
                    ))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::IncompatibleParameterError(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::InternalServerError(err.msg),
                    )
                }
                "InvalidNetworkSettings" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::InvalidNetworkSettings(err.msg),
                    )
                }
                "InvalidPerUnitStorageThroughput" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::InvalidPerUnitStorageThroughput(err.msg),
                    )
                }
                "MissingFileSystemConfiguration" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::MissingFileSystemConfiguration(err.msg),
                    )
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateFileSystemFromBackupError::ServiceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFileSystemFromBackupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFileSystemFromBackupError::ActiveDirectoryError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemFromBackupError::BackupNotFound(ref cause) => write!(f, "{}", cause),
            CreateFileSystemFromBackupError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateFileSystemFromBackupError::IncompatibleParameterError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemFromBackupError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemFromBackupError::InvalidNetworkSettings(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemFromBackupError::InvalidPerUnitStorageThroughput(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemFromBackupError::MissingFileSystemConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemFromBackupError::ServiceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateFileSystemFromBackupError {}
/// Errors returned by DeleteBackup
#[derive(Debug, PartialEq)]
pub enum DeleteBackupError {
    /// <p>Another backup is already under way. Wait for completion before initiating additional backups of this file system.</p>
    BackupInProgress(String),
    /// <p>No Amazon FSx backups were found based upon the supplied parameters.</p>
    BackupNotFound(String),
    /// <p>You can't delete a backup while it's being used to restore a file system.</p>
    BackupRestoring(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DeleteBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BackupInProgress" => {
                    return RusotoError::Service(DeleteBackupError::BackupInProgress(err.msg))
                }
                "BackupNotFound" => {
                    return RusotoError::Service(DeleteBackupError::BackupNotFound(err.msg))
                }
                "BackupRestoring" => {
                    return RusotoError::Service(DeleteBackupError::BackupRestoring(err.msg))
                }
                "BadRequest" => {
                    return RusotoError::Service(DeleteBackupError::BadRequest(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(DeleteBackupError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteBackupError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBackupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupError::BackupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::BackupNotFound(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::BackupRestoring(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::IncompatibleParameterError(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBackupError {}
/// Errors returned by DeleteFileSystem
#[derive(Debug, PartialEq)]
pub enum DeleteFileSystemError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
}

impl DeleteFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DeleteFileSystemError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DeleteFileSystemError::FileSystemNotFound(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(DeleteFileSystemError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(DeleteFileSystemError::ServiceLimitExceeded(
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
impl fmt::Display for DeleteFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFileSystemError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemError::IncompatibleParameterError(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemError::ServiceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFileSystemError {}
/// Errors returned by DescribeBackups
#[derive(Debug, PartialEq)]
pub enum DescribeBackupsError {
    /// <p>No Amazon FSx backups were found based upon the supplied parameters.</p>
    BackupNotFound(String),
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DescribeBackupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBackupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BackupNotFound" => {
                    return RusotoError::Service(DescribeBackupsError::BackupNotFound(err.msg))
                }
                "BadRequest" => {
                    return RusotoError::Service(DescribeBackupsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeBackupsError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeBackupsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBackupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBackupsError::BackupNotFound(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBackupsError {}
/// Errors returned by DescribeDataRepositoryTasks
#[derive(Debug, PartialEq)]
pub enum DescribeDataRepositoryTasksError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>The data repository task or tasks you specified could not be found.</p>
    DataRepositoryTaskNotFound(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DescribeDataRepositoryTasksError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDataRepositoryTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DescribeDataRepositoryTasksError::BadRequest(
                        err.msg,
                    ))
                }
                "DataRepositoryTaskNotFound" => {
                    return RusotoError::Service(
                        DescribeDataRepositoryTasksError::DataRepositoryTaskNotFound(err.msg),
                    )
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(
                        DescribeDataRepositoryTasksError::FileSystemNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeDataRepositoryTasksError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDataRepositoryTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDataRepositoryTasksError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeDataRepositoryTasksError::DataRepositoryTaskNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDataRepositoryTasksError::FileSystemNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDataRepositoryTasksError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDataRepositoryTasksError {}
/// Errors returned by DescribeFileSystemAliases
#[derive(Debug, PartialEq)]
pub enum DescribeFileSystemAliasesError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DescribeFileSystemAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFileSystemAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DescribeFileSystemAliasesError::BadRequest(
                        err.msg,
                    ))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(
                        DescribeFileSystemAliasesError::FileSystemNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeFileSystemAliasesError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFileSystemAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFileSystemAliasesError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemAliasesError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemAliasesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeFileSystemAliasesError {}
/// Errors returned by DescribeFileSystems
#[derive(Debug, PartialEq)]
pub enum DescribeFileSystemsError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DescribeFileSystemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFileSystemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DescribeFileSystemsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeFileSystemsError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeFileSystemsError::InternalServerError(
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
impl fmt::Display for DescribeFileSystemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFileSystemsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFileSystemsError {}
/// Errors returned by DisassociateFileSystemAliases
#[derive(Debug, PartialEq)]
pub enum DisassociateFileSystemAliasesError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
}

impl DisassociateFileSystemAliasesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateFileSystemAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DisassociateFileSystemAliasesError::BadRequest(
                        err.msg,
                    ))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(
                        DisassociateFileSystemAliasesError::FileSystemNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DisassociateFileSystemAliasesError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateFileSystemAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateFileSystemAliasesError::BadRequest(ref cause) => write!(f, "{}", cause),
            DisassociateFileSystemAliasesError::FileSystemNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateFileSystemAliasesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateFileSystemAliasesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The resource specified for the tagging operation is not a resource type owned by Amazon FSx. Use the API of the relevant service to perform the operation. </p>
    NotServiceResourceError(String),
    /// <p>The resource specified does not support tagging. </p>
    ResourceDoesNotSupportTagging(String),
    /// <p>The resource specified by the Amazon Resource Name (ARN) can't be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotServiceResourceError" => {
                    return RusotoError::Service(ListTagsForResourceError::NotServiceResourceError(
                        err.msg,
                    ))
                }
                "ResourceDoesNotSupportTagging" => {
                    return RusotoError::Service(
                        ListTagsForResourceError::ResourceDoesNotSupportTagging(err.msg),
                    )
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotServiceResourceError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceDoesNotSupportTagging(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The resource specified for the tagging operation is not a resource type owned by Amazon FSx. Use the API of the relevant service to perform the operation. </p>
    NotServiceResourceError(String),
    /// <p>The resource specified does not support tagging. </p>
    ResourceDoesNotSupportTagging(String),
    /// <p>The resource specified by the Amazon Resource Name (ARN) can't be found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => return RusotoError::Service(TagResourceError::BadRequest(err.msg)),
                "InternalServerError" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotServiceResourceError" => {
                    return RusotoError::Service(TagResourceError::NotServiceResourceError(err.msg))
                }
                "ResourceDoesNotSupportTagging" => {
                    return RusotoError::Service(TagResourceError::ResourceDoesNotSupportTagging(
                        err.msg,
                    ))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotServiceResourceError(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceDoesNotSupportTagging(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>The resource specified for the tagging operation is not a resource type owned by Amazon FSx. Use the API of the relevant service to perform the operation. </p>
    NotServiceResourceError(String),
    /// <p>The resource specified does not support tagging. </p>
    ResourceDoesNotSupportTagging(String),
    /// <p>The resource specified by the Amazon Resource Name (ARN) can't be found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotServiceResourceError" => {
                    return RusotoError::Service(UntagResourceError::NotServiceResourceError(
                        err.msg,
                    ))
                }
                "ResourceDoesNotSupportTagging" => {
                    return RusotoError::Service(UntagResourceError::ResourceDoesNotSupportTagging(
                        err.msg,
                    ))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotServiceResourceError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceDoesNotSupportTagging(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateFileSystem
#[derive(Debug, PartialEq)]
pub enum UpdateFileSystemError {
    /// <p>A generic error indicating a failure with a client request.</p>
    BadRequest(String),
    /// <p>No Amazon FSx file systems were found based upon supplied parameters.</p>
    FileSystemNotFound(String),
    /// <p>The error returned when a second request is received with the same client request token but different parameters settings. A client request token should always uniquely identify a single request.</p>
    IncompatibleParameterError(String),
    /// <p>A generic error indicating a server-side failure.</p>
    InternalServerError(String),
    /// <p>A file system configuration is required for this operation.</p>
    MissingFileSystemConfiguration(String),
    /// <p>An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting AWS Support. </p>
    ServiceLimitExceeded(String),
    /// <p>The requested operation is not supported for this resource or API.</p>
    UnsupportedOperation(String),
}

impl UpdateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(UpdateFileSystemError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(UpdateFileSystemError::FileSystemNotFound(err.msg))
                }
                "IncompatibleParameterError" => {
                    return RusotoError::Service(UpdateFileSystemError::IncompatibleParameterError(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "MissingFileSystemConfiguration" => {
                    return RusotoError::Service(
                        UpdateFileSystemError::MissingFileSystemConfiguration(err.msg),
                    )
                }
                "ServiceLimitExceeded" => {
                    return RusotoError::Service(UpdateFileSystemError::ServiceLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedOperation" => {
                    return RusotoError::Service(UpdateFileSystemError::UnsupportedOperation(
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
impl fmt::Display for UpdateFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFileSystemError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::IncompatibleParameterError(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::MissingFileSystemConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFileSystemError::ServiceLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFileSystemError {}
/// Trait representing the capabilities of the Amazon FSx API. Amazon FSx clients implement this trait.
#[async_trait]
pub trait Fsx: Clone + Sync + Send + 'static {
    /// <p>Use this action to associate one or more Domain Name Server (DNS) aliases with an existing Amazon FSx for Windows File Server file system. A file systen can have a maximum of 50 DNS aliases associated with it at any one time. If you try to associate a DNS alias that is already associated with the file system, FSx takes no action on that alias in the request. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html">Working with DNS Aliases</a> and <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/walkthrough05-file-system-custom-CNAME.html">Walkthrough 5: Using DNS aliases to access your file system</a>, including additional steps you must take to be able to access your file system using a DNS alias.</p> <p>The system response shows the DNS aliases that Amazon FSx is attempting to associate with the file system. Use the API operation to monitor the status of the aliases Amazon FSx is associating with the file system.</p>
    async fn associate_file_system_aliases(
        &self,
        input: AssociateFileSystemAliasesRequest,
    ) -> Result<AssociateFileSystemAliasesResponse, RusotoError<AssociateFileSystemAliasesError>>;

    /// <p><p>Cancels an existing Amazon FSx for Lustre data repository task if that task is in either the <code>PENDING</code> or <code>EXECUTING</code> state. When you cancel a task, Amazon FSx does the following.</p> <ul> <li> <p>Any files that FSx has already exported are not reverted.</p> </li> <li> <p>FSx continues to export any files that are &quot;in-flight&quot; when the cancel operation is received.</p> </li> <li> <p>FSx does not export any files that have not yet been exported.</p> </li> </ul></p>
    async fn cancel_data_repository_task(
        &self,
        input: CancelDataRepositoryTaskRequest,
    ) -> Result<CancelDataRepositoryTaskResponse, RusotoError<CancelDataRepositoryTaskError>>;

    /// <p>Creates a backup of an existing Amazon FSx file system. Creating regular backups for your file system is a best practice, enabling you to restore a file system from a backup if an issue arises with the original file system.</p> <p>For Amazon FSx for Lustre file systems, you can create a backup only for file systems with the following configuration:</p> <ul> <li> <p>a Persistent deployment type</p> </li> <li> <p>is <i>not</i> linked to a data respository.</p> </li> </ul> <p>For more information about backing up Amazon FSx for Lustre file systems, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/using-backups-fsx.html">Working with FSx for Lustre backups</a>.</p> <p>For more information about backing up Amazon FSx for Windows file systems, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/using-backups.html">Working with FSx for Windows backups</a>.</p> <p>If a backup with the specified client request token exists, and the parameters match, this operation returns the description of the existing backup. If a backup specified client request token exists, and the parameters don't match, this operation returns <code>IncompatibleParameterError</code>. If a backup with the specified client request token doesn't exist, <code>CreateBackup</code> does the following: </p> <ul> <li> <p>Creates a new Amazon FSx backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the backup.</p> </li> </ul> <p>By using the idempotent operation, you can retry a <code>CreateBackup</code> operation without the risk of creating an extra backup. This approach can be useful when an initial call fails in a way that makes it unclear whether a backup was created. If you use the same client request token and the initial call created a backup, the operation returns a successful result because all the parameters are the same.</p> <p>The <code>CreateBackup</code> operation returns while the backup's lifecycle state is still <code>CREATING</code>. You can check the backup creation status by calling the <a>DescribeBackups</a> operation, which returns the backup state along with other information.</p>
    async fn create_backup(
        &self,
        input: CreateBackupRequest,
    ) -> Result<CreateBackupResponse, RusotoError<CreateBackupError>>;

    /// <p>Creates an Amazon FSx for Lustre data repository task. You use data repository tasks to perform bulk operations between your Amazon FSx file system and its linked data repository. An example of a data repository task is exporting any data and metadata changes, including POSIX metadata, to files, directories, and symbolic links (symlinks) from your FSx file system to its linked data repository. A <code>CreateDataRepositoryTask</code> operation will fail if a data repository is not linked to the FSx file system. To learn more about data repository tasks, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/data-repository-tasks.html">Data Repository Tasks</a>. To learn more about linking a data repository to your file system, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/create-fs-linked-data-repo.html">Linking your file system to an S3 bucket</a>.</p>
    async fn create_data_repository_task(
        &self,
        input: CreateDataRepositoryTaskRequest,
    ) -> Result<CreateDataRepositoryTaskResponse, RusotoError<CreateDataRepositoryTaskError>>;

    /// <p><p>Creates a new, empty Amazon FSx file system.</p> <p>If a file system with the specified client request token exists and the parameters match, <code>CreateFileSystem</code> returns the description of the existing file system. If a file system specified client request token exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, <code>CreateFileSystem</code> does the following: </p> <ul> <li> <p>Creates a new, empty Amazon FSx file system with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>This operation requires a client request token in the request that Amazon FSx uses to ensure idempotent creation. This means that calling the operation multiple times with the same client request token has no effect. By using the idempotent operation, you can retry a <code>CreateFileSystem</code> operation without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    async fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> Result<CreateFileSystemResponse, RusotoError<CreateFileSystemError>>;

    /// <p><p>Creates a new Amazon FSx file system from an existing Amazon FSx backup.</p> <p>If a file system with the specified client request token exists and the parameters match, this operation returns the description of the file system. If a client request token specified by the file system exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, this operation does the following:</p> <ul> <li> <p>Creates a new Amazon FSx file system from backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>Parameters like Active Directory, default share name, automatic backup, and backup settings default to the parameters of the file system that was backed up, unless overridden. You can explicitly supply other settings.</p> <p>By using the idempotent operation, you can retry a <code>CreateFileSystemFromBackup</code> call without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystemFromBackup</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    async fn create_file_system_from_backup(
        &self,
        input: CreateFileSystemFromBackupRequest,
    ) -> Result<CreateFileSystemFromBackupResponse, RusotoError<CreateFileSystemFromBackupError>>;

    /// <p><p>Deletes an Amazon FSx backup, deleting its contents. After deletion, the backup no longer exists, and its data is gone.</p> <p>The <code>DeleteBackup</code> call returns instantly. The backup will not show up in later <code>DescribeBackups</code> calls.</p> <important> <p>The data in a deleted backup is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    async fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> Result<DeleteBackupResponse, RusotoError<DeleteBackupError>>;

    /// <p><p>Deletes a file system, deleting its contents. After deletion, the file system no longer exists, and its data is gone. Any existing automatic backups will also be deleted.</p> <p>By default, when you delete an Amazon FSx for Windows File Server file system, a final backup is created upon deletion. This final backup is not subject to the file system&#39;s retention policy, and must be manually deleted.</p> <p>The <code>DeleteFileSystem</code> action returns while the file system has the <code>DELETING</code> status. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> action, which returns a list of file systems in your account. If you pass the file system ID for a deleted file system, the <a>DescribeFileSystems</a> returns a <code>FileSystemNotFound</code> error.</p> <note> <p>Deleting an Amazon FSx for Lustre file system will fail with a 400 BadRequest if a data repository task is in a <code>PENDING</code> or <code>EXECUTING</code> state.</p> </note> <important> <p>The data in a deleted file system is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    async fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> Result<DeleteFileSystemResponse, RusotoError<DeleteFileSystemError>>;

    /// <p><p>Returns the description of specific Amazon FSx backups, if a <code>BackupIds</code> value is provided for that backup. Otherwise, it returns all backups owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all backups, you can optionally specify the <code>MaxResults</code> parameter to limit the number of backups in a response. If more backups remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your backups. <code>DescribeBackups</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of backups returned in the response of one <code>DescribeBackups</code> call and the order of backups returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    async fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> Result<DescribeBackupsResponse, RusotoError<DescribeBackupsError>>;

    /// Auto-paginating version of `describe_backups`
    fn describe_backups_pages<'a>(
        &'a self,
        mut input: DescribeBackupsRequest,
    ) -> RusotoStream<'a, Backup, DescribeBackupsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.describe_backups(input.clone())
        }))
    }

    /// <p>Returns the description of specific Amazon FSx for Lustre data repository tasks, if one or more <code>TaskIds</code> values are provided in the request, or if filters are used in the request. You can use filters to narrow the response to include just tasks for specific file systems, or tasks in a specific lifecycle state. Otherwise, it returns all data repository tasks owned by your AWS account in the AWS Region of the endpoint that you're calling.</p> <p>When retrieving all tasks, you can paginate the response by using the optional <code>MaxResults</code> parameter to limit the number of tasks returned in a response. If more tasks remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p>
    async fn describe_data_repository_tasks(
        &self,
        input: DescribeDataRepositoryTasksRequest,
    ) -> Result<DescribeDataRepositoryTasksResponse, RusotoError<DescribeDataRepositoryTasksError>>;

    /// <p>Returns the DNS aliases that are associated with the specified Amazon FSx for Windows File Server file system. A history of all DNS aliases that have been associated with and disassociated from the file system is available in the list of <a>AdministrativeAction</a> provided in the <a>DescribeFileSystems</a> operation response.</p>
    async fn describe_file_system_aliases(
        &self,
        input: DescribeFileSystemAliasesRequest,
    ) -> Result<DescribeFileSystemAliasesResponse, RusotoError<DescribeFileSystemAliasesError>>;

    /// <p><p>Returns the description of specific Amazon FSx file systems, if a <code>FileSystemIds</code> value is provided for that file system. Otherwise, it returns descriptions of all file systems owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all file system descriptions, you can optionally specify the <code>MaxResults</code> parameter to limit the number of descriptions in a response. If more file system descriptions remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your file system descriptions. <code>DescribeFileSystems</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multicall iteration is unspecified.</p> </li> </ul></p>
    async fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> Result<DescribeFileSystemsResponse, RusotoError<DescribeFileSystemsError>>;

    /// Auto-paginating version of `describe_file_systems`
    fn describe_file_systems_pages<'a>(
        &'a self,
        mut input: DescribeFileSystemsRequest,
    ) -> RusotoStream<'a, FileSystem, DescribeFileSystemsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.describe_file_systems(input.clone())
        }))
    }

    /// <p>Use this action to disassociate, or remove, one or more Domain Name Service (DNS) aliases from an Amazon FSx for Windows File Server file system. If you attempt to disassociate a DNS alias that is not associated with the file system, Amazon FSx responds with a 400 Bad Request. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html">Working with DNS Aliases</a>.</p> <p>The system generated response showing the DNS aliases that Amazon FSx is attempting to disassociate from the file system. Use the API operation to monitor the status of the aliases Amazon FSx is disassociating with the file system.</p>
    async fn disassociate_file_system_aliases(
        &self,
        input: DisassociateFileSystemAliasesRequest,
    ) -> Result<
        DisassociateFileSystemAliasesResponse,
        RusotoError<DisassociateFileSystemAliasesError>,
    >;

    /// <p><p>Lists tags for an Amazon FSx file systems and backups in the case of Amazon FSx for Windows File Server.</p> <p>When retrieving all tags, you can optionally specify the <code>MaxResults</code> parameter to limit the number of tags in a response. If more tags remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your tags. <code>ListTagsForResource</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of tags returned in the response of one <code>ListTagsForResource</code> call and the order of tags returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// Auto-paginating version of `list_tags_for_resource`
    fn list_tags_for_resource_pages<'a>(
        &'a self,
        mut input: ListTagsForResourceRequest,
    ) -> RusotoStream<'a, Tag, ListTagsForResourceError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_tags_for_resource(input.clone())
        }))
    }

    /// <p>Tags an Amazon FSx resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>This action removes a tag from an Amazon FSx resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><p>Use this operation to update the configuration of an existing Amazon FSx file system. You can update multiple properties in a single request.</p> <p>For Amazon FSx for Windows File Server file systems, you can update the following properties:</p> <ul> <li> <p>AutomaticBackupRetentionDays</p> </li> <li> <p>DailyAutomaticBackupStartTime</p> </li> <li> <p>SelfManagedActiveDirectoryConfiguration</p> </li> <li> <p>StorageCapacity</p> </li> <li> <p>ThroughputCapacity</p> </li> <li> <p>WeeklyMaintenanceStartTime</p> </li> </ul> <p>For Amazon FSx for Lustre file systems, you can update the following properties:</p> <ul> <li> <p>AutoImportPolicy</p> </li> <li> <p>AutomaticBackupRetentionDays</p> </li> <li> <p>DailyAutomaticBackupStartTime</p> </li> <li> <p>StorageCapacity</p> </li> <li> <p>WeeklyMaintenanceStartTime</p> </li> </ul></p>
    async fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> Result<UpdateFileSystemResponse, RusotoError<UpdateFileSystemError>>;
}
/// A client for the Amazon FSx API.
#[derive(Clone)]
pub struct FsxClient {
    client: Client,
    region: region::Region,
}

impl FsxClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> FsxClient {
        FsxClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> FsxClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        FsxClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> FsxClient {
        FsxClient { client, region }
    }
}

#[async_trait]
impl Fsx for FsxClient {
    /// <p>Use this action to associate one or more Domain Name Server (DNS) aliases with an existing Amazon FSx for Windows File Server file system. A file systen can have a maximum of 50 DNS aliases associated with it at any one time. If you try to associate a DNS alias that is already associated with the file system, FSx takes no action on that alias in the request. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html">Working with DNS Aliases</a> and <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/walkthrough05-file-system-custom-CNAME.html">Walkthrough 5: Using DNS aliases to access your file system</a>, including additional steps you must take to be able to access your file system using a DNS alias.</p> <p>The system response shows the DNS aliases that Amazon FSx is attempting to associate with the file system. Use the API operation to monitor the status of the aliases Amazon FSx is associating with the file system.</p>
    async fn associate_file_system_aliases(
        &self,
        input: AssociateFileSystemAliasesRequest,
    ) -> Result<AssociateFileSystemAliasesResponse, RusotoError<AssociateFileSystemAliasesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.AssociateFileSystemAliases",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateFileSystemAliasesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateFileSystemAliasesResponse, _>()
    }

    /// <p><p>Cancels an existing Amazon FSx for Lustre data repository task if that task is in either the <code>PENDING</code> or <code>EXECUTING</code> state. When you cancel a task, Amazon FSx does the following.</p> <ul> <li> <p>Any files that FSx has already exported are not reverted.</p> </li> <li> <p>FSx continues to export any files that are &quot;in-flight&quot; when the cancel operation is received.</p> </li> <li> <p>FSx does not export any files that have not yet been exported.</p> </li> </ul></p>
    async fn cancel_data_repository_task(
        &self,
        input: CancelDataRepositoryTaskRequest,
    ) -> Result<CancelDataRepositoryTaskResponse, RusotoError<CancelDataRepositoryTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.CancelDataRepositoryTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelDataRepositoryTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CancelDataRepositoryTaskResponse, _>()
    }

    /// <p>Creates a backup of an existing Amazon FSx file system. Creating regular backups for your file system is a best practice, enabling you to restore a file system from a backup if an issue arises with the original file system.</p> <p>For Amazon FSx for Lustre file systems, you can create a backup only for file systems with the following configuration:</p> <ul> <li> <p>a Persistent deployment type</p> </li> <li> <p>is <i>not</i> linked to a data respository.</p> </li> </ul> <p>For more information about backing up Amazon FSx for Lustre file systems, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/using-backups-fsx.html">Working with FSx for Lustre backups</a>.</p> <p>For more information about backing up Amazon FSx for Windows file systems, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/using-backups.html">Working with FSx for Windows backups</a>.</p> <p>If a backup with the specified client request token exists, and the parameters match, this operation returns the description of the existing backup. If a backup specified client request token exists, and the parameters don't match, this operation returns <code>IncompatibleParameterError</code>. If a backup with the specified client request token doesn't exist, <code>CreateBackup</code> does the following: </p> <ul> <li> <p>Creates a new Amazon FSx backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the backup.</p> </li> </ul> <p>By using the idempotent operation, you can retry a <code>CreateBackup</code> operation without the risk of creating an extra backup. This approach can be useful when an initial call fails in a way that makes it unclear whether a backup was created. If you use the same client request token and the initial call created a backup, the operation returns a successful result because all the parameters are the same.</p> <p>The <code>CreateBackup</code> operation returns while the backup's lifecycle state is still <code>CREATING</code>. You can check the backup creation status by calling the <a>DescribeBackups</a> operation, which returns the backup state along with other information.</p>
    async fn create_backup(
        &self,
        input: CreateBackupRequest,
    ) -> Result<CreateBackupResponse, RusotoError<CreateBackupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.CreateBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateBackupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateBackupResponse, _>()
    }

    /// <p>Creates an Amazon FSx for Lustre data repository task. You use data repository tasks to perform bulk operations between your Amazon FSx file system and its linked data repository. An example of a data repository task is exporting any data and metadata changes, including POSIX metadata, to files, directories, and symbolic links (symlinks) from your FSx file system to its linked data repository. A <code>CreateDataRepositoryTask</code> operation will fail if a data repository is not linked to the FSx file system. To learn more about data repository tasks, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/data-repository-tasks.html">Data Repository Tasks</a>. To learn more about linking a data repository to your file system, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/create-fs-linked-data-repo.html">Linking your file system to an S3 bucket</a>.</p>
    async fn create_data_repository_task(
        &self,
        input: CreateDataRepositoryTaskRequest,
    ) -> Result<CreateDataRepositoryTaskResponse, RusotoError<CreateDataRepositoryTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.CreateDataRepositoryTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDataRepositoryTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateDataRepositoryTaskResponse, _>()
    }

    /// <p><p>Creates a new, empty Amazon FSx file system.</p> <p>If a file system with the specified client request token exists and the parameters match, <code>CreateFileSystem</code> returns the description of the existing file system. If a file system specified client request token exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, <code>CreateFileSystem</code> does the following: </p> <ul> <li> <p>Creates a new, empty Amazon FSx file system with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>This operation requires a client request token in the request that Amazon FSx uses to ensure idempotent creation. This means that calling the operation multiple times with the same client request token has no effect. By using the idempotent operation, you can retry a <code>CreateFileSystem</code> operation without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    async fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> Result<CreateFileSystemResponse, RusotoError<CreateFileSystemError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.CreateFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateFileSystemError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateFileSystemResponse, _>()
    }

    /// <p><p>Creates a new Amazon FSx file system from an existing Amazon FSx backup.</p> <p>If a file system with the specified client request token exists and the parameters match, this operation returns the description of the file system. If a client request token specified by the file system exists and the parameters don&#39;t match, this call returns <code>IncompatibleParameterError</code>. If a file system with the specified client request token doesn&#39;t exist, this operation does the following:</p> <ul> <li> <p>Creates a new Amazon FSx file system from backup with an assigned ID, and an initial lifecycle state of <code>CREATING</code>.</p> </li> <li> <p>Returns the description of the file system.</p> </li> </ul> <p>Parameters like Active Directory, default share name, automatic backup, and backup settings default to the parameters of the file system that was backed up, unless overridden. You can explicitly supply other settings.</p> <p>By using the idempotent operation, you can retry a <code>CreateFileSystemFromBackup</code> call without the risk of creating an extra file system. This approach can be useful when an initial call fails in a way that makes it unclear whether a file system was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a file system, the client receives success as long as the parameters are the same.</p> <note> <p>The <code>CreateFileSystemFromBackup</code> call returns while the file system&#39;s lifecycle state is still <code>CREATING</code>. You can check the file-system creation status by calling the <a>DescribeFileSystems</a> operation, which returns the file system state along with other information.</p> </note></p>
    async fn create_file_system_from_backup(
        &self,
        input: CreateFileSystemFromBackupRequest,
    ) -> Result<CreateFileSystemFromBackupResponse, RusotoError<CreateFileSystemFromBackupError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.CreateFileSystemFromBackup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateFileSystemFromBackupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateFileSystemFromBackupResponse, _>()
    }

    /// <p><p>Deletes an Amazon FSx backup, deleting its contents. After deletion, the backup no longer exists, and its data is gone.</p> <p>The <code>DeleteBackup</code> call returns instantly. The backup will not show up in later <code>DescribeBackups</code> calls.</p> <important> <p>The data in a deleted backup is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    async fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> Result<DeleteBackupResponse, RusotoError<DeleteBackupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.DeleteBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteBackupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteBackupResponse, _>()
    }

    /// <p><p>Deletes a file system, deleting its contents. After deletion, the file system no longer exists, and its data is gone. Any existing automatic backups will also be deleted.</p> <p>By default, when you delete an Amazon FSx for Windows File Server file system, a final backup is created upon deletion. This final backup is not subject to the file system&#39;s retention policy, and must be manually deleted.</p> <p>The <code>DeleteFileSystem</code> action returns while the file system has the <code>DELETING</code> status. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> action, which returns a list of file systems in your account. If you pass the file system ID for a deleted file system, the <a>DescribeFileSystems</a> returns a <code>FileSystemNotFound</code> error.</p> <note> <p>Deleting an Amazon FSx for Lustre file system will fail with a 400 BadRequest if a data repository task is in a <code>PENDING</code> or <code>EXECUTING</code> state.</p> </note> <important> <p>The data in a deleted file system is also deleted and can&#39;t be recovered by any means.</p> </important></p>
    async fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> Result<DeleteFileSystemResponse, RusotoError<DeleteFileSystemError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DeleteFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteFileSystemError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteFileSystemResponse, _>()
    }

    /// <p><p>Returns the description of specific Amazon FSx backups, if a <code>BackupIds</code> value is provided for that backup. Otherwise, it returns all backups owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all backups, you can optionally specify the <code>MaxResults</code> parameter to limit the number of backups in a response. If more backups remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your backups. <code>DescribeBackups</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of backups returned in the response of one <code>DescribeBackups</code> call and the order of backups returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    async fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> Result<DescribeBackupsResponse, RusotoError<DescribeBackupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DescribeBackups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeBackupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeBackupsResponse, _>()
    }

    /// <p>Returns the description of specific Amazon FSx for Lustre data repository tasks, if one or more <code>TaskIds</code> values are provided in the request, or if filters are used in the request. You can use filters to narrow the response to include just tasks for specific file systems, or tasks in a specific lifecycle state. Otherwise, it returns all data repository tasks owned by your AWS account in the AWS Region of the endpoint that you're calling.</p> <p>When retrieving all tasks, you can paginate the response by using the optional <code>MaxResults</code> parameter to limit the number of tasks returned in a response. If more tasks remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p>
    async fn describe_data_repository_tasks(
        &self,
        input: DescribeDataRepositoryTasksRequest,
    ) -> Result<DescribeDataRepositoryTasksResponse, RusotoError<DescribeDataRepositoryTasksError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DescribeDataRepositoryTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDataRepositoryTasksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeDataRepositoryTasksResponse, _>()
    }

    /// <p>Returns the DNS aliases that are associated with the specified Amazon FSx for Windows File Server file system. A history of all DNS aliases that have been associated with and disassociated from the file system is available in the list of <a>AdministrativeAction</a> provided in the <a>DescribeFileSystems</a> operation response.</p>
    async fn describe_file_system_aliases(
        &self,
        input: DescribeFileSystemAliasesRequest,
    ) -> Result<DescribeFileSystemAliasesResponse, RusotoError<DescribeFileSystemAliasesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DescribeFileSystemAliases",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFileSystemAliasesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeFileSystemAliasesResponse, _>()
    }

    /// <p><p>Returns the description of specific Amazon FSx file systems, if a <code>FileSystemIds</code> value is provided for that file system. Otherwise, it returns descriptions of all file systems owned by your AWS account in the AWS Region of the endpoint that you&#39;re calling.</p> <p>When retrieving all file system descriptions, you can optionally specify the <code>MaxResults</code> parameter to limit the number of descriptions in a response. If more file system descriptions remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your file system descriptions. <code>DescribeFileSystems</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multicall iteration is unspecified.</p> </li> </ul></p>
    async fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> Result<DescribeFileSystemsResponse, RusotoError<DescribeFileSystemsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DescribeFileSystems",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFileSystemsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeFileSystemsResponse, _>()
    }

    /// <p>Use this action to disassociate, or remove, one or more Domain Name Service (DNS) aliases from an Amazon FSx for Windows File Server file system. If you attempt to disassociate a DNS alias that is not associated with the file system, Amazon FSx responds with a 400 Bad Request. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html">Working with DNS Aliases</a>.</p> <p>The system generated response showing the DNS aliases that Amazon FSx is attempting to disassociate from the file system. Use the API operation to monitor the status of the aliases Amazon FSx is disassociating with the file system.</p>
    async fn disassociate_file_system_aliases(
        &self,
        input: DisassociateFileSystemAliasesRequest,
    ) -> Result<
        DisassociateFileSystemAliasesResponse,
        RusotoError<DisassociateFileSystemAliasesError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.DisassociateFileSystemAliases",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateFileSystemAliasesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateFileSystemAliasesResponse, _>()
    }

    /// <p><p>Lists tags for an Amazon FSx file systems and backups in the case of Amazon FSx for Windows File Server.</p> <p>When retrieving all tags, you can optionally specify the <code>MaxResults</code> parameter to limit the number of tags in a response. If more tags remain, Amazon FSx returns a <code>NextToken</code> value in the response. In this case, send a later request with the <code>NextToken</code> request parameter set to the value of <code>NextToken</code> from the last response.</p> <p>This action is used in an iterative process to retrieve a list of your tags. <code>ListTagsForResource</code> is called first without a <code>NextToken</code>value. Then the action continues to be called with the <code>NextToken</code> parameter set to the value of the last <code>NextToken</code> value until a response has no <code>NextToken</code>.</p> <p>When using this action, keep the following in mind:</p> <ul> <li> <p>The implementation might return fewer than <code>MaxResults</code> file system descriptions while still including a <code>NextToken</code> value.</p> </li> <li> <p>The order of tags returned in the response of one <code>ListTagsForResource</code> call and the order of tags returned across the responses of a multi-call iteration is unspecified.</p> </li> </ul></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Tags an Amazon FSx resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>This action removes a tag from an Amazon FSx resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSSimbaAPIService_v20180301.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p><p>Use this operation to update the configuration of an existing Amazon FSx file system. You can update multiple properties in a single request.</p> <p>For Amazon FSx for Windows File Server file systems, you can update the following properties:</p> <ul> <li> <p>AutomaticBackupRetentionDays</p> </li> <li> <p>DailyAutomaticBackupStartTime</p> </li> <li> <p>SelfManagedActiveDirectoryConfiguration</p> </li> <li> <p>StorageCapacity</p> </li> <li> <p>ThroughputCapacity</p> </li> <li> <p>WeeklyMaintenanceStartTime</p> </li> </ul> <p>For Amazon FSx for Lustre file systems, you can update the following properties:</p> <ul> <li> <p>AutoImportPolicy</p> </li> <li> <p>AutomaticBackupRetentionDays</p> </li> <li> <p>DailyAutomaticBackupStartTime</p> </li> <li> <p>StorageCapacity</p> </li> <li> <p>WeeklyMaintenanceStartTime</p> </li> </ul></p>
    async fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> Result<UpdateFileSystemResponse, RusotoError<UpdateFileSystemError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSSimbaAPIService_v20180301.UpdateFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateFileSystemError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateFileSystemResponse, _>()
    }
}
