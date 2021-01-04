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
/// <p>A list of backup options for each resource type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AdvancedBackupSetting {
    /// <p>Specifies the backup option for a selected resource. This option is only available for Windows VSS backup jobs.</p> <p>Valid values: </p> <p>Set to <code>"WindowsVSS":"enabled"</code> to enable the WindowsVSS backup option and create a VSS Windows backup. </p> <p>Set to <code>"WindowsVSS":"disabled"</code> to create a regular backup. The WindowsVSS option is not enabled by default.</p> <p>If you specify an invalid option, you get an <code>InvalidParameterValueException</code> exception.</p> <p>For more information about Windows VSS backups, see <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/windows-backups.html">Creating a VSS-Enabled Windows Backup</a>.</p>
    #[serde(rename = "BackupOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of AWS resource to be backed up. For VSS Windows backups, the only supported resource type is Amazon EC2.</p> <p>Valid values: <code>EC2</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Contains detailed information about a backup job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupJob {
    /// <p>The account ID that owns the backup job.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Uniquely identifies a request to AWS Backup to back up a resource.</p>
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    /// <p>Specifies the backup option for a selected resource. This option is only available for Windows VSS backup jobs.</p> <p>Valid values: Set to <code>"WindowsVSS”:“enabled"</code> to enable WindowsVSS backup option and create a VSS Windows backup. Set to “WindowsVSS”:”disabled” to create a regular backup. If you specify an invalid option, you get an <code>InvalidParameterValueException</code> exception.</p>
    #[serde(rename = "BackupOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The size, in bytes, of a backup.</p>
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    /// <p>Represents the type of backup for a backup job.</p>
    #[serde(rename = "BackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>The size in bytes transferred to a backup vault at the time that the job status was queried.</p>
    #[serde(rename = "BytesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    /// <p>The date and time a job to create a backup job is completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>Contains identifying information about the creation of a backup job, including the <code>BackupPlanArn</code>, <code>BackupPlanId</code>, <code>BackupPlanVersion</code>, and <code>BackupRuleId</code> of the backup plan used to create it.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    /// <p>The date and time a backup job is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date and time a job to back up resources is expected to be completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>ExpectedCompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "ExpectedCompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<f64>,
    /// <p>Specifies the IAM role ARN used to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>Contains an estimated percentage complete of a job at the time the job status was queried.</p>
    #[serde(rename = "PercentDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of AWS resource to be backed up; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database. For VSS Windows backups, the only supported resource type is Amazon EC2.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Specifies the time in Unix format and Coordinated Universal Time (UTC) when a backup job must be started before it is canceled. The value is calculated by adding the start window to the scheduled time. So if the scheduled time were 6:00 PM and the start window is 2 hours, the <code>StartBy</code> time would be 8:00 PM on the date specified. The value of <code>StartBy</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "StartBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_by: Option<f64>,
    /// <p>The current state of a resource recovery point.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A detailed message explaining the status of the job to back up a resource.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>Contains an optional backup plan display name and an array of <code>BackupRule</code> objects, each of which specifies a backup rule. Each rule in a backup plan is a separate scheduled task and can back up a different selection of AWS resources.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupPlan {
    /// <p>Contains a list of <code>BackupOptions</code> for each resource type.</p>
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    /// <p>The display name of a backup plan.</p>
    #[serde(rename = "BackupPlanName")]
    pub backup_plan_name: String,
    /// <p>An array of <code>BackupRule</code> objects, each of which specifies a scheduled task that is used to back up a selection of resources. </p>
    #[serde(rename = "Rules")]
    pub rules: Vec<BackupRule>,
}

/// <p>Contains an optional backup plan display name and an array of <code>BackupRule</code> objects, each of which specifies a backup rule. Each rule in a backup plan is a separate scheduled task and can back up a different selection of AWS resources. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BackupPlanInput {
    /// <p>Specifies a list of <code>BackupOptions</code> for each resource type. These settings are only available for Windows VSS backup jobs.</p>
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    /// <p>The optional display name of a backup plan.</p>
    #[serde(rename = "BackupPlanName")]
    pub backup_plan_name: String,
    /// <p>An array of <code>BackupRule</code> objects, each of which specifies a scheduled task that is used to back up a selection of resources.</p>
    #[serde(rename = "Rules")]
    pub rules: Vec<BackupRuleInput>,
}

/// <p>An object specifying metadata associated with a backup plan template.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupPlanTemplatesListMember {
    /// <p>Uniquely identifies a stored backup plan template.</p>
    #[serde(rename = "BackupPlanTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_template_id: Option<String>,
    /// <p>The optional display name of a backup plan template.</p>
    #[serde(rename = "BackupPlanTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_template_name: Option<String>,
}

/// <p>Contains metadata about a backup plan.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupPlansListMember {
    /// <p>Contains a list of <code>BackupOptions</code> for a resource type.</p>
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup plan; for example, <code>arn:aws:backup:us-east-1:123456789012:plan:8F81F553-3A74-4A3F-B93D-B3360DC80C50</code>.</p>
    #[serde(rename = "BackupPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>The display name of a saved backup plan.</p>
    #[serde(rename = "BackupPlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_name: Option<String>,
    /// <p>The date and time a resource backup plan is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The date and time a backup plan is deleted, in Unix format and Coordinated Universal Time (UTC). The value of <code>DeletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The last time a job to back up resources was run with this rule. A date and time, in Unix format and Coordinated Universal Time (UTC). The value of <code>LastExecutionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>Unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes long. Version IDs cannot be edited.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>Specifies a scheduled task used to back up a selection of resources.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupRule {
    /// <p>A value in minutes after a backup job is successfully started before it must be completed or it will be canceled by AWS Backup. This value is optional.</p>
    #[serde(rename = "CompletionWindowMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window_minutes: Option<i64>,
    /// <p>An array of <code>CopyAction</code> objects, which contains the details of the copy operation.</p>
    #[serde(rename = "CopyActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_actions: Option<Vec<CopyAction>>,
    /// <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>An array of key-value pair strings that are assigned to resources that are associated with this rule when restored from backup.</p>
    #[serde(rename = "RecoveryPointTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Uniquely identifies a rule that is used to schedule the backup of a selection of resources.</p>
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// <p>An optional display name for a backup rule.</p>
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// <p>A CRON expression specifying when AWS Backup initiates a backup job. For more information about cron expressions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html">Schedule Expressions for Rules</a> in the <i>Amazon CloudWatch Events User Guide.</i>. Prior to specifying a value for this parameter, we recommend testing your cron expression using one of the many available cron generator and testing tools.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>A value in minutes after a backup is scheduled before a job will be canceled if it doesn't start successfully. This value is optional.</p>
    #[serde(rename = "StartWindowMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<i64>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "TargetBackupVaultName")]
    pub target_backup_vault_name: String,
}

/// <p>Specifies a scheduled task used to back up a selection of resources.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BackupRuleInput {
    /// <p>A value in minutes after a backup job is successfully started before it must be completed or it will be canceled by AWS Backup. This value is optional.</p>
    #[serde(rename = "CompletionWindowMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window_minutes: Option<i64>,
    /// <p>An array of <code>CopyAction</code> objects, which contains the details of the copy operation.</p>
    #[serde(rename = "CopyActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_actions: Option<Vec<CopyAction>>,
    /// <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup will transition and expire backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>To help organize your resources, you can assign your own metadata to the resources that you create. Each tag is a key-value pair.</p>
    #[serde(rename = "RecoveryPointTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>An optional display name for a backup rule.</p>
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// <p>A CRON expression specifying when AWS Backup initiates a backup job.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>A value in minutes after a backup is scheduled before a job will be canceled if it doesn't start successfully. This value is optional.</p>
    #[serde(rename = "StartWindowMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<i64>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "TargetBackupVaultName")]
    pub target_backup_vault_name: String,
}

/// <p>Used to specify a set of resources to a backup plan.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BackupSelection {
    /// <p>The ARN of the IAM role that AWS Backup uses to authenticate when backing up the target resource; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// <p>An array of conditions used to specify a set of resources to assign to a backup plan; for example, <code>"StringEquals": {"ec2:ResourceTag/Department": "accounting"</code>.</p>
    #[serde(rename = "ListOfTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_of_tags: Option<Vec<Condition>>,
    /// <p>An array of strings that contain Amazon Resource Names (ARNs) of resources to assign to a backup plan.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// <p>The display name of a resource selection document.</p>
    #[serde(rename = "SelectionName")]
    pub selection_name: String,
}

/// <p>Contains metadata about a <code>BackupSelection</code> object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupSelectionsListMember {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>The date and time a backup plan is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>Specifies the IAM role Amazon Resource Name (ARN) to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>Uniquely identifies a request to assign a set of resources to a backup plan.</p>
    #[serde(rename = "SelectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_id: Option<String>,
    /// <p>The display name of a resource selection document.</p>
    #[serde(rename = "SelectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_name: Option<String>,
}

/// <p>Contains metadata about a backup vault.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BackupVaultListMember {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>The date and time a resource backup is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The server-side encryption key that is used to protect your backups; for example, <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    /// <p>The number of recovery points that are stored in a backup vault.</p>
    #[serde(rename = "NumberOfRecoveryPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recovery_points: Option<i64>,
}

/// <p>Contains <code>DeleteAt</code> and <code>MoveToColdStorageAt</code> timestamps, which are used to specify a lifecycle for a recovery point.</p> <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define.</p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CalculatedLifecycle {
    /// <p>A timestamp that specifies when to delete a recovery point.</p>
    #[serde(rename = "DeleteAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<f64>,
    /// <p>A timestamp that specifies when to transition a recovery point to cold storage.</p>
    #[serde(rename = "MoveToColdStorageAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_at: Option<f64>,
}

/// <p>Contains an array of triplets made up of a condition type (such as <code>StringEquals</code>), a key, and a value. Conditions are used to filter resources in a selection that is assigned to a backup plan.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Condition {
    /// <p>The key in a key-value pair. For example, in <code>"ec2:ResourceTag/Department": "accounting"</code>, <code>"ec2:ResourceTag/Department"</code> is the key.</p>
    #[serde(rename = "ConditionKey")]
    pub condition_key: String,
    /// <p>An operation, such as <code>StringEquals</code>, that is applied to a key-value pair used to filter resources in a selection.</p>
    #[serde(rename = "ConditionType")]
    pub condition_type: String,
    /// <p>The value in a key-value pair. For example, in <code>"ec2:ResourceTag/Department": "accounting"</code>, <code>"accounting"</code> is the value.</p>
    #[serde(rename = "ConditionValue")]
    pub condition_value: String,
}

/// <p>The details of the copy operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CopyAction {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies the destination backup vault for the copied backup. For example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "DestinationBackupVaultArn")]
    pub destination_backup_vault_arn: String,
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
}

/// <p>Contains detailed information about a copy job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CopyJob {
    /// <p>The account ID that owns the copy job.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The size, in bytes, of a copy job.</p>
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    /// <p>The date and time a copy job is completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>Uniquely identifies a copy job.</p>
    #[serde(rename = "CopyJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_job_id: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    /// <p>The date and time a copy job is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a destination copy vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "DestinationBackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_backup_vault_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a destination recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "DestinationRecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_recovery_point_arn: Option<String>,
    /// <p>Specifies the IAM role ARN used to copy the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>The AWS resource to be copied; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of AWS resource to be copied; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a source copy vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>. </p>
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a source recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "SourceRecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_recovery_point_arn: Option<String>,
    /// <p>The current state of a copy job.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A detailed message explaining the status of the job to copy a resource.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBackupPlanInput {
    /// <p>Specifies the body of a backup plan. Includes a <code>BackupPlanName</code> and one or more sets of <code>Rules</code>.</p>
    #[serde(rename = "BackupPlan")]
    pub backup_plan: BackupPlanInput,
    /// <p>To help organize your resources, you can assign your own metadata to the resources that you create. Each tag is a key-value pair. The specified tags are assigned to all backups created with this plan.</p>
    #[serde(rename = "BackupPlanTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Identifies the request and allows failed requests to be retried without the risk of running the operation twice. If the request includes a <code>CreatorRequestId</code> that matches an existing backup plan, that plan is returned. This parameter is optional.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBackupPlanOutput {
    /// <p>A list of <code>BackupOptions</code> settings for a resource type. This option is only available for Windows VSS backup jobs.</p>
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup plan; for example, <code>arn:aws:backup:us-east-1:123456789012:plan:8F81F553-3A74-4A3F-B93D-B3360DC80C50</code>.</p>
    #[serde(rename = "BackupPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>The date and time that a backup plan is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes long. They cannot be edited.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBackupSelectionInput {
    /// <p>Uniquely identifies the backup plan to be associated with the selection of resources.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
    /// <p>Specifies the body of a request to assign a set of resources to a backup plan.</p>
    #[serde(rename = "BackupSelection")]
    pub backup_selection: BackupSelection,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBackupSelectionOutput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>The date and time a backup selection is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Uniquely identifies the body of a request to assign a set of resources to a backup plan.</p>
    #[serde(rename = "SelectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBackupVaultInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>Metadata that you can assign to help organize the resources that you create. Each tag is a key-value pair.</p>
    #[serde(rename = "BackupVaultTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The server-side encryption key that is used to protect your backups; for example, <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBackupVaultOutput {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>The date and time a backup vault is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupPlanInput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBackupPlanOutput {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup plan; for example, <code>arn:aws:backup:us-east-1:123456789012:plan:8F81F553-3A74-4A3F-B93D-B3360DC80C50</code>.</p>
    #[serde(rename = "BackupPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>The date and time a backup plan is deleted, in Unix format and Coordinated Universal Time (UTC). The value of <code>DeletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>Unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes long. Version IDs cannot be edited.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupSelectionInput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
    /// <p>Uniquely identifies the body of a request to assign a set of resources to a backup plan.</p>
    #[serde(rename = "SelectionId")]
    pub selection_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupVaultAccessPolicyInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupVaultInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupVaultNotificationsInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRecoveryPointInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    pub recovery_point_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBackupJobInput {
    /// <p>Uniquely identifies a request to AWS Backup to back up a resource.</p>
    #[serde(rename = "BackupJobId")]
    pub backup_job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBackupJobOutput {
    /// <p>Returns the account ID that owns the backup job.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Uniquely identifies a request to AWS Backup to back up a resource.</p>
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    /// <p>Represents the options specified as part of backup plan or on-demand backup job.</p>
    #[serde(rename = "BackupOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The size, in bytes, of a backup.</p>
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    /// <p>Represents the actual backup type selected for a backup job. For example, if a successful WindowsVSS backup was taken, <code>BackupType</code> returns "WindowsVSS". If <code>BackupType</code> is empty, then the backup type that was is a regular backup.</p>
    #[serde(rename = "BackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>The size in bytes transferred to a backup vault at the time that the job status was queried.</p>
    #[serde(rename = "BytesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    /// <p>The date and time that a job to create a backup job is completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>Contains identifying information about the creation of a backup job, including the <code>BackupPlanArn</code>, <code>BackupPlanId</code>, <code>BackupPlanVersion</code>, and <code>BackupRuleId</code> of the backup plan that is used to create it.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    /// <p>The date and time that a backup job is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date and time that a job to back up resources is expected to be completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>ExpectedCompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "ExpectedCompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<f64>,
    /// <p>Specifies the IAM role ARN used to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>Contains an estimated percentage that is complete of a job at the time the job status was queried.</p>
    #[serde(rename = "PercentDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a saved resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of AWS resource to be backed up; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Specifies the time in Unix format and Coordinated Universal Time (UTC) when a backup job must be started before it is canceled. The value is calculated by adding the start window to the scheduled time. So if the scheduled time were 6:00 PM and the start window is 2 hours, the <code>StartBy</code> time would be 8:00 PM on the date specified. The value of <code>StartBy</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "StartBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_by: Option<f64>,
    /// <p>The current state of a resource recovery point.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A detailed message explaining the status of the job to back up a resource.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBackupVaultInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBackupVaultOutput {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>The date and time that a backup vault is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The server-side encryption key that is used to protect your backups; for example, <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    /// <p>The number of recovery points that are stored in a backup vault.</p>
    #[serde(rename = "NumberOfRecoveryPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recovery_points: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCopyJobInput {
    /// <p>Uniquely identifies a copy job.</p>
    #[serde(rename = "CopyJobId")]
    pub copy_job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCopyJobOutput {
    /// <p>Contains detailed information about a copy job.</p>
    #[serde(rename = "CopyJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_job: Option<CopyJob>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGlobalSettingsInput {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGlobalSettingsOutput {
    /// <p>A list of resources along with the opt-in preferences for the account.</p>
    #[serde(rename = "GlobalSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_settings: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date and time that the global settings was last updated. This update is in Unix format and Coordinated Universal Time (UTC). The value of <code>LastUpdateTime</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProtectedResourceInput {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProtectedResourceOutput {
    /// <p>The date and time that a resource was last backed up, in Unix format and Coordinated Universal Time (UTC). The value of <code>LastBackupTime</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "LastBackupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_time: Option<f64>,
    /// <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of AWS resource saved as a recovery point; for example, an EBS volume or an Amazon RDS database.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRecoveryPointInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    pub recovery_point_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRecoveryPointOutput {
    /// <p>The size, in bytes, of a backup.</p>
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    /// <p>An ARN that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>A <code>CalculatedLifecycle</code> object containing <code>DeleteAt</code> and <code>MoveToColdStorageAt</code> timestamps.</p>
    #[serde(rename = "CalculatedLifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_lifecycle: Option<CalculatedLifecycle>,
    /// <p>The date and time that a job to create a recovery point is completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>Contains identifying information about the creation of a recovery point, including the <code>BackupPlanArn</code>, <code>BackupPlanId</code>, <code>BackupPlanVersion</code>, and <code>BackupRuleId</code> of the backup plan used to create it.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    /// <p>The date and time that a recovery point is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The server-side encryption key used to protect your backups; for example, <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    /// <p>Specifies the IAM role ARN used to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>A Boolean value that is returned as <code>TRUE</code> if the specified recovery point is encrypted, or <code>FALSE</code> if the recovery point is not encrypted.</p>
    #[serde(rename = "IsEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    /// <p>The date and time that a recovery point was last restored, in Unix format and Coordinated Universal Time (UTC). The value of <code>LastRestoreTime</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "LastRestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_restore_time: Option<f64>,
    /// <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define. </p> <p>Backups that are transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a saved resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of AWS resource to save as a recovery point; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies the source vault where the resource was originally backed up in; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:BackupVault</code>. If the recovery is restored to the same AWS account or Region, this value will be <code>null</code>.</p>
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    /// <p><p>A status code specifying the state of the recovery point.</p> <note> <p>A partial status indicates that the recovery point was not successfully re-created and must be retried.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specifies the storage class of the recovery point. Valid values are <code>WARM</code> or <code>COLD</code>.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRegionSettingsInput {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRegionSettingsOutput {
    /// <p>Returns a list of all services along with the opt-in preferences in the Region.</p>
    #[serde(rename = "ResourceTypeOptInPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_opt_in_preference: Option<::std::collections::HashMap<String, bool>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRestoreJobInput {
    /// <p>Uniquely identifies the job that restores a recovery point.</p>
    #[serde(rename = "RestoreJobId")]
    pub restore_job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRestoreJobOutput {
    /// <p>Returns the account ID that owns the restore job.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The size, in bytes, of the restored resource.</p>
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    /// <p>The date and time that a job to restore a recovery point is completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource whose recovery point is being restored. The format of the ARN depends on the resource type of the backed-up resource.</p>
    #[serde(rename = "CreatedResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_resource_arn: Option<String>,
    /// <p>The date and time that a restore job is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The amount of time in minutes that a job restoring a recovery point is expected to take.</p>
    #[serde(rename = "ExpectedCompletionTimeMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_time_minutes: Option<i64>,
    /// <p>Specifies the IAM role ARN used to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>Contains an estimated percentage that is complete of a job at the time the job status was queried.</p>
    #[serde(rename = "PercentDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>Returns metadata associated with a restore job listed by resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Uniquely identifies the job that restores a recovery point.</p>
    #[serde(rename = "RestoreJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_id: Option<String>,
    /// <p>Status code specifying the state of the job that is initiated by AWS Backup to restore a recovery point.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message showing the status of a job to restore a recovery point.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportBackupPlanTemplateInput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportBackupPlanTemplateOutput {
    /// <p><p>The body of a backup plan template in JSON format.</p> <note> <p>This is a signed JSON document that cannot be modified before being passed to <code>GetBackupPlanFromJSON.</code> </p> </note></p>
    #[serde(rename = "BackupPlanTemplateJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_template_json: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBackupPlanFromJSONInput {
    /// <p>A customer-supplied backup plan document in JSON format.</p>
    #[serde(rename = "BackupPlanTemplateJson")]
    pub backup_plan_template_json: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBackupPlanFromJSONOutput {
    /// <p>Specifies the body of a backup plan. Includes a <code>BackupPlanName</code> and one or more sets of <code>Rules</code>.</p>
    #[serde(rename = "BackupPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan: Option<BackupPlan>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBackupPlanFromTemplateInput {
    /// <p>Uniquely identifies a stored backup plan template.</p>
    #[serde(rename = "BackupPlanTemplateId")]
    pub backup_plan_template_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBackupPlanFromTemplateOutput {
    /// <p>Returns the body of a backup plan based on the target template, including the name, rules, and backup vault of the plan.</p>
    #[serde(rename = "BackupPlanDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_document: Option<BackupPlan>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBackupPlanInput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
    /// <p>Unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes long. Version IDs cannot be edited.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBackupPlanOutput {
    /// <p>Contains a list of <code>BackupOptions</code> for each resource type. The list is populated only if the advanced option is set for the backup plan.</p>
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    /// <p>Specifies the body of a backup plan. Includes a <code>BackupPlanName</code> and one or more sets of <code>Rules</code>.</p>
    #[serde(rename = "BackupPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan: Option<BackupPlan>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup plan; for example, <code>arn:aws:backup:us-east-1:123456789012:plan:8F81F553-3A74-4A3F-B93D-B3360DC80C50</code>.</p>
    #[serde(rename = "BackupPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>The date and time that a backup plan is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The date and time that a backup plan is deleted, in Unix format and Coordinated Universal Time (UTC). The value of <code>DeletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The last time a job to back up resources was run with this backup plan. A date and time, in Unix format and Coordinated Universal Time (UTC). The value of <code>LastExecutionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>Unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes long. Version IDs cannot be edited.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBackupSelectionInput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
    /// <p>Uniquely identifies the body of a request to assign a set of resources to a backup plan.</p>
    #[serde(rename = "SelectionId")]
    pub selection_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBackupSelectionOutput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>Specifies the body of a request to assign a set of resources to a backup plan.</p>
    #[serde(rename = "BackupSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_selection: Option<BackupSelection>,
    /// <p>The date and time a backup selection is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique string that identifies the request and allows failed requests to be retried without the risk of running the operation twice.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>Uniquely identifies the body of a request to assign a set of resources to a backup plan.</p>
    #[serde(rename = "SelectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBackupVaultAccessPolicyInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBackupVaultAccessPolicyOutput {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>The backup vault access policy document in JSON format.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBackupVaultNotificationsInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBackupVaultNotificationsOutput {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>An array of events that indicate the status of jobs to back up resources to the backup vault.</p>
    #[serde(rename = "BackupVaultEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_events: Option<Vec<String>>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>An ARN that uniquely identifies an Amazon Simple Notification Service (Amazon SNS) topic; for example, <code>arn:aws:sns:us-west-2:111122223333:MyTopic</code>.</p>
    #[serde(rename = "SNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRecoveryPointRestoreMetadataInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    pub recovery_point_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecoveryPointRestoreMetadataOutput {
    /// <p>An ARN that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>The set of metadata key-value pairs that describe the original configuration of the backed-up resource. These values vary depending on the service that is being restored.</p>
    #[serde(rename = "RestoreMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_metadata: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSupportedResourceTypesOutput {
    /// <p><p>Contains a string with the supported AWS resource types:</p> <ul> <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li> <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li> <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li> <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li> <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li> <li> <p> <code>Storage Gateway</code> for AWS Storage Gateway</p> </li> </ul></p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

/// <p>Contains an array of <code>Transition</code> objects specifying how long in days before a recovery point transitions to cold storage or is deleted.</p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, on the console, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Lifecycle {
    /// <p>Specifies the number of days after creation that a recovery point is deleted. Must be greater than 90 days plus <code>MoveToColdStorageAfterDays</code>.</p>
    #[serde(rename = "DeleteAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_days: Option<i64>,
    /// <p>Specifies the number of days after creation that a recovery point is moved to cold storage.</p>
    #[serde(rename = "MoveToColdStorageAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_after_days: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBackupJobsInput {
    /// <p>The account ID to list the jobs from. Returns only backup jobs associated with the specified account ID.</p>
    #[serde(rename = "ByAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_account_id: Option<String>,
    /// <p>Returns only backup jobs that will be stored in the specified backup vault. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "ByBackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_backup_vault_name: Option<String>,
    /// <p>Returns only backup jobs that were created after the specified date.</p>
    #[serde(rename = "ByCreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    /// <p>Returns only backup jobs that were created before the specified date.</p>
    #[serde(rename = "ByCreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    /// <p>Returns only backup jobs that match the specified resource Amazon Resource Name (ARN).</p>
    #[serde(rename = "ByResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_arn: Option<String>,
    /// <p><p>Returns only backup jobs for the specified resources:</p> <ul> <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li> <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li> <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li> <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li> <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li> <li> <p> <code>Storage Gateway</code> for AWS Storage Gateway</p> </li> </ul></p>
    #[serde(rename = "ByResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
    /// <p>Returns only backup jobs that are in the specified state.</p>
    #[serde(rename = "ByState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_state: Option<String>,
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBackupJobsOutput {
    /// <p>An array of structures containing metadata about your backup jobs returned in JSON format.</p>
    #[serde(rename = "BackupJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_jobs: Option<Vec<BackupJob>>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBackupPlanTemplatesInput {
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBackupPlanTemplatesOutput {
    /// <p>An array of template list items containing metadata about your saved templates.</p>
    #[serde(rename = "BackupPlanTemplatesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_templates_list: Option<Vec<BackupPlanTemplatesListMember>>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBackupPlanVersionsInput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBackupPlanVersionsOutput {
    /// <p>An array of version list items containing metadata about your backup plans.</p>
    #[serde(rename = "BackupPlanVersionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_versions_list: Option<Vec<BackupPlansListMember>>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBackupPlansInput {
    /// <p>A Boolean value with a default value of <code>FALSE</code> that returns deleted backup plans when set to <code>TRUE</code>.</p>
    #[serde(rename = "IncludeDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBackupPlansOutput {
    /// <p>An array of backup plan list items containing metadata about your saved backup plans.</p>
    #[serde(rename = "BackupPlansList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plans_list: Option<Vec<BackupPlansListMember>>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBackupSelectionsInput {
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBackupSelectionsOutput {
    /// <p>An array of backup selection list items containing metadata about each resource in the list.</p>
    #[serde(rename = "BackupSelectionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_selections_list: Option<Vec<BackupSelectionsListMember>>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBackupVaultsInput {
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBackupVaultsOutput {
    /// <p>An array of backup vault list members containing vault metadata, including Amazon Resource Name (ARN), display name, creation date, number of saved recovery points, and encryption information if the resources saved in the backup vault are encrypted.</p>
    #[serde(rename = "BackupVaultList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_list: Option<Vec<BackupVaultListMember>>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCopyJobsInput {
    /// <p>The account ID to list the jobs from. Returns only copy jobs associated with the specified account ID.</p>
    #[serde(rename = "ByAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_account_id: Option<String>,
    /// <p>Returns only copy jobs that were created after the specified date.</p>
    #[serde(rename = "ByCreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    /// <p>Returns only copy jobs that were created before the specified date.</p>
    #[serde(rename = "ByCreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a source backup vault to copy from; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>. </p>
    #[serde(rename = "ByDestinationVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_destination_vault_arn: Option<String>,
    /// <p>Returns only copy jobs that match the specified resource Amazon Resource Name (ARN). </p>
    #[serde(rename = "ByResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_arn: Option<String>,
    /// <p><p>Returns only backup jobs for the specified resources:</p> <ul> <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li> <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li> <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li> <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li> <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li> <li> <p> <code>Storage Gateway</code> for AWS Storage Gateway</p> </li> </ul></p>
    #[serde(rename = "ByResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
    /// <p>Returns only copy jobs that are in the specified state.</p>
    #[serde(rename = "ByState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_state: Option<String>,
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return maxResults number of items, NextToken allows you to return more items in your list starting at the location pointed to by the next token. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCopyJobsOutput {
    /// <p>An array of structures containing metadata about your copy jobs returned in JSON format. </p>
    #[serde(rename = "CopyJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_jobs: Option<Vec<CopyJob>>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return maxResults number of items, NextToken allows you to return more items in your list starting at the location pointed to by the next token. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProtectedResourcesInput {
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProtectedResourcesOutput {
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of resources successfully backed up by AWS Backup including the time the resource was saved, an Amazon Resource Name (ARN) of the resource, and a resource type.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ProtectedResource>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRecoveryPointsByBackupVaultInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>Returns only recovery points that match the specified backup plan ID.</p>
    #[serde(rename = "ByBackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_backup_plan_id: Option<String>,
    /// <p>Returns only recovery points that were created after the specified timestamp.</p>
    #[serde(rename = "ByCreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    /// <p>Returns only recovery points that were created before the specified timestamp.</p>
    #[serde(rename = "ByCreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    /// <p>Returns only recovery points that match the specified resource Amazon Resource Name (ARN).</p>
    #[serde(rename = "ByResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_arn: Option<String>,
    /// <p>Returns only recovery points that match the specified resource type.</p>
    #[serde(rename = "ByResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_resource_type: Option<String>,
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRecoveryPointsByBackupVaultOutput {
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain detailed information about recovery points saved in a backup vault.</p>
    #[serde(rename = "RecoveryPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<Vec<RecoveryPointByBackupVault>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRecoveryPointsByResourceInput {
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRecoveryPointsByResourceOutput {
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain detailed information about recovery points of the specified resource type.</p>
    #[serde(rename = "RecoveryPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<Vec<RecoveryPointByResource>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRestoreJobsInput {
    /// <p>The account ID to list the jobs from. Returns only restore jobs associated with the specified account ID.</p>
    #[serde(rename = "ByAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_account_id: Option<String>,
    /// <p>Returns only restore jobs that were created after the specified date.</p>
    #[serde(rename = "ByCreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_after: Option<f64>,
    /// <p>Returns only restore jobs that were created before the specified date.</p>
    #[serde(rename = "ByCreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_created_before: Option<f64>,
    /// <p>Returns only restore jobs associated with the specified job status.</p>
    #[serde(rename = "ByStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_status: Option<String>,
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRestoreJobsOutput {
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that contain detailed information about jobs to restore saved resources.</p>
    #[serde(rename = "RestoreJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_jobs: Option<Vec<RestoreJobsListMember>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsInput {
    /// <p>The maximum number of items to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the type of resource. Valid targets for <code>ListTags</code> are recovery points, backup plans, and backup vaults.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsOutput {
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>To help organize your resources, you can assign your own metadata to the resources you create. Each tag is a key-value pair.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A structure that contains information about a backed-up resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProtectedResource {
    /// <p>The date and time a resource was last backed up, in Unix format and Coordinated Universal Time (UTC). The value of <code>LastBackupTime</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "LastBackupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_time: Option<f64>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of AWS resource; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database. For VSS Windows backups, the only supported resource type is Amazon EC2.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutBackupVaultAccessPolicyInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>The backup vault access policy document in JSON format.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutBackupVaultNotificationsInput {
    /// <p>An array of events that indicate the status of jobs to back up resources to the backup vault.</p>
    #[serde(rename = "BackupVaultEvents")]
    pub backup_vault_events: Vec<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>The Amazon Resource Name (ARN) that specifies the topic for a backup vault’s events; for example, <code>arn:aws:sns:us-west-2:111122223333:MyVaultTopic</code>.</p>
    #[serde(rename = "SNSTopicArn")]
    pub sns_topic_arn: String,
}

/// <p>Contains detailed information about the recovery points stored in a backup vault.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecoveryPointByBackupVault {
    /// <p>The size, in bytes, of a backup.</p>
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    /// <p>An ARN that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>A <code>CalculatedLifecycle</code> object containing <code>DeleteAt</code> and <code>MoveToColdStorageAt</code> timestamps.</p>
    #[serde(rename = "CalculatedLifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_lifecycle: Option<CalculatedLifecycle>,
    /// <p>The date and time a job to restore a recovery point is completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>Contains identifying information about the creation of a recovery point, including the <code>BackupPlanArn</code>, <code>BackupPlanId</code>, <code>BackupPlanVersion</code>, and <code>BackupRuleId</code> of the backup plan that is used to create it.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<RecoveryPointCreator>,
    /// <p>The date and time a recovery point is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The server-side encryption key that is used to protect your backups; for example, <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    /// <p>Specifies the IAM role ARN used to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>A Boolean value that is returned as <code>TRUE</code> if the specified recovery point is encrypted, or <code>FALSE</code> if the recovery point is not encrypted.</p>
    #[serde(rename = "IsEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    /// <p>The date and time a recovery point was last restored, in Unix format and Coordinated Universal Time (UTC). The value of <code>LastRestoreTime</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "LastRestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_restore_time: Option<f64>,
    /// <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The type of AWS resource saved as a recovery point; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database. For VSS Windows backups, the only supported resource type is Amazon EC2.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The backup vault where the recovery point was originally copied from. If the recovery point is restored to the same account this value will be <code>null</code>.</p>
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    /// <p>A status code specifying the state of the recovery point.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains detailed information about a saved recovery point.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecoveryPointByResource {
    /// <p>The size, in bytes, of a backup.</p>
    #[serde(rename = "BackupSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_bytes: Option<i64>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    /// <p>The date and time a recovery point is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The server-side encryption key that is used to protect your backups; for example, <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>A status code specifying the state of the recovery point.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains information about the backup plan and rule that AWS Backup used to initiate the recovery point backup.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecoveryPointCreator {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup plan; for example, <code>arn:aws:backup:us-east-1:123456789012:plan:8F81F553-3A74-4A3F-B93D-B3360DC80C50</code>.</p>
    #[serde(rename = "BackupPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>Version IDs are unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes long. They cannot be edited.</p>
    #[serde(rename = "BackupPlanVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_version: Option<String>,
    /// <p>Uniquely identifies a rule used to schedule the backup of a selection of resources.</p>
    #[serde(rename = "BackupRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule_id: Option<String>,
}

/// <p>Contains metadata about a restore job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestoreJobsListMember {
    /// <p>The account ID that owns the restore job.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The size, in bytes, of the restored resource.</p>
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    /// <p>The date and time a job to restore a recovery point is completed, in Unix format and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "CreatedResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_resource_arn: Option<String>,
    /// <p>The date and time a restore job is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The amount of time in minutes that a job restoring a recovery point is expected to take.</p>
    #[serde(rename = "ExpectedCompletionTimeMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_time_minutes: Option<i64>,
    /// <p>Specifies the IAM role ARN used to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>Contains an estimated percentage complete of a job at the time the job status was queried.</p>
    #[serde(rename = "PercentDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_done: Option<String>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    /// <p>The resource type of the listed restore jobs; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database. For VSS Windows backups, the only supported resource type is Amazon EC2.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Uniquely identifies the job that restores a recovery point.</p>
    #[serde(rename = "RestoreJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_id: Option<String>,
    /// <p>A status code specifying the state of the job initiated by AWS Backup to restore a recovery point.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A detailed message explaining the status of the job to restore a recovery point.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartBackupJobInput {
    /// <p>Specifies the backup option for a selected resource. This option is only available for Windows VSS backup jobs.</p> <p>Valid values: Set to <code>"WindowsVSS”:“enabled"</code> to enable WindowsVSS backup option and create a VSS Windows backup. Set to “WindowsVSS”:”disabled” to create a regular backup. The WindowsVSS option is not enabled by default.</p>
    #[serde(rename = "BackupOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>A value in minutes after a backup job is successfully started before it must be completed or it will be canceled by AWS Backup. This value is optional.</p>
    #[serde(rename = "CompleteWindowMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_window_minutes: Option<i64>,
    /// <p>Specifies the IAM role ARN used to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// <p>A customer chosen string that can be used to distinguish between calls to <code>StartBackupJob</code>.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup will transition and expire backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>To help organize your resources, you can assign your own metadata to the resources that you create. Each tag is a key-value pair.</p>
    #[serde(rename = "RecoveryPointTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A value in minutes after a backup is scheduled before a job will be canceled if it doesn't start successfully. This value is optional.</p>
    #[serde(rename = "StartWindowMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartBackupJobOutput {
    /// <p>Uniquely identifies a request to AWS Backup to back up a resource.</p>
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    /// <p>The date and time that a backup job is started, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartCopyJobInput {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a destination backup vault to copy to; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "DestinationBackupVaultArn")]
    pub destination_backup_vault_arn: String,
    /// <p>Specifies the IAM role ARN used to copy the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// <p>A customer chosen string that can be used to distinguish between calls to <code>StartCopyJob</code>.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>An ARN that uniquely identifies a recovery point to use for the copy job; for example, arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45. </p>
    #[serde(rename = "RecoveryPointArn")]
    pub recovery_point_arn: String,
    /// <p>The name of a logical source container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "SourceBackupVaultName")]
    pub source_backup_vault_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartCopyJobOutput {
    /// <p>Uniquely identifies a copy job.</p>
    #[serde(rename = "CopyJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_job_id: Option<String>,
    /// <p>The date and time that a copy job is started, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartRestoreJobInput {
    /// <p>The Amazon Resource Name (ARN) of the IAM role that AWS Backup uses to create the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// <p>A customer chosen string that can be used to distinguish between calls to <code>StartRestoreJob</code>.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p><p>A set of metadata key-value pairs. Contains information, such as a resource name, required to restore a recovery point.</p> <p> You can get configuration metadata about a resource at the time it was backed up by calling <code>GetRecoveryPointRestoreMetadata</code>. However, values in addition to those provided by <code>GetRecoveryPointRestoreMetadata</code> might be required to restore a resource. For example, you might need to provide a new resource name if the original already exists.</p> <p>You need to specify specific metadata to restore an Amazon Elastic File System (Amazon EFS) instance:</p> <ul> <li> <p> <code>file-system-id</code>: The ID of the Amazon EFS file system that is backed up by AWS Backup. Returned in <code>GetRecoveryPointRestoreMetadata</code>.</p> </li> <li> <p> <code>Encrypted</code>: A Boolean value that, if true, specifies that the file system is encrypted. If <code>KmsKeyId</code> is specified, <code>Encrypted</code> must be set to <code>true</code>.</p> </li> <li> <p> <code>KmsKeyId</code>: Specifies the AWS KMS key that is used to encrypt the restored file system. You can specify a key from another AWS account provided that key it is properly shared with your account via AWS KMS.</p> </li> <li> <p> <code>PerformanceMode</code>: Specifies the throughput mode of the file system.</p> </li> <li> <p> <code>CreationToken</code>: A user-supplied value that ensures the uniqueness (idempotency) of the request.</p> </li> <li> <p> <code>newFileSystem</code>: A Boolean value that, if true, specifies that the recovery point is restored to a new Amazon EFS file system.</p> </li> <li> <p> <code>ItemsToRestore </code>: A serialized list of up to five strings where each string is a file path. Use <code>ItemsToRestore</code> to restore specific files or directories rather than the entire file system. This parameter is optional.</p> </li> </ul></p>
    #[serde(rename = "Metadata")]
    pub metadata: ::std::collections::HashMap<String, String>,
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    pub recovery_point_arn: String,
    /// <p><p>Starts a job to restore a recovery point for one of the following resources:</p> <ul> <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li> <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li> <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li> <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li> <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li> <li> <p> <code>Storage Gateway</code> for AWS Storage Gateway</p> </li> </ul></p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartRestoreJobOutput {
    /// <p>Uniquely identifies the job that restores a recovery point.</p>
    #[serde(rename = "RestoreJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopBackupJobInput {
    /// <p>Uniquely identifies a request to AWS Backup to back up a resource.</p>
    #[serde(rename = "BackupJobId")]
    pub backup_job_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the type of the tagged resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Key-value pairs that are used to help organize your resources. You can assign your own metadata to the resources you create. </p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the type of the tagged resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A list of keys to identify which key-value tags to remove from a resource.</p>
    #[serde(rename = "TagKeyList")]
    pub tag_key_list: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBackupPlanInput {
    /// <p>Specifies the body of a backup plan. Includes a <code>BackupPlanName</code> and one or more sets of <code>Rules</code>.</p>
    #[serde(rename = "BackupPlan")]
    pub backup_plan: BackupPlanInput,
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBackupPlanOutput {
    /// <p>Contains a list of <code>BackupOptions</code> for each resource type.</p>
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSetting>>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a backup plan; for example, <code>arn:aws:backup:us-east-1:123456789012:plan:8F81F553-3A74-4A3F-B93D-B3360DC80C50</code>.</p>
    #[serde(rename = "BackupPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    /// <p>Uniquely identifies a backup plan.</p>
    #[serde(rename = "BackupPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    /// <p>The date and time a backup plan is updated, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes long. Version Ids cannot be edited.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGlobalSettingsInput {
    /// <p>A list of resources along with the opt-in preferences for the account.</p>
    #[serde(rename = "GlobalSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_settings: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRecoveryPointLifecycleInput {
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the AWS Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: String,
    /// <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    pub recovery_point_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRecoveryPointLifecycleOutput {
    /// <p>An ARN that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    #[serde(rename = "BackupVaultArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    /// <p>A <code>CalculatedLifecycle</code> object containing <code>DeleteAt</code> and <code>MoveToColdStorageAt</code> timestamps.</p>
    #[serde(rename = "CalculatedLifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_lifecycle: Option<CalculatedLifecycle>,
    /// <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    #[serde(rename = "RecoveryPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRegionSettingsInput {
    /// <p>Updates the list of services along with the opt-in preferences for the Region.</p>
    #[serde(rename = "ResourceTypeOptInPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_opt_in_preference: Option<::std::collections::HashMap<String, bool>>,
}

/// Errors returned by CreateBackupPlan
#[derive(Debug, PartialEq)]
pub enum CreateBackupPlanError {
    /// <p>The required resource already exists.</p>
    AlreadyExists(String),
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>A limit in the request has been exceeded; for example, a maximum number of items allowed in a request.</p>
    LimitExceeded(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl CreateBackupPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBackupPlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateBackupPlanError::AlreadyExists(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateBackupPlanError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBackupPlanError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(CreateBackupPlanError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateBackupPlanError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBackupPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBackupPlanError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateBackupPlanError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateBackupPlanError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBackupPlanError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            CreateBackupPlanError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBackupPlanError {}
/// Errors returned by CreateBackupSelection
#[derive(Debug, PartialEq)]
pub enum CreateBackupSelectionError {
    /// <p>The required resource already exists.</p>
    AlreadyExists(String),
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>A limit in the request has been exceeded; for example, a maximum number of items allowed in a request.</p>
    LimitExceeded(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl CreateBackupSelectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBackupSelectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateBackupSelectionError::AlreadyExists(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateBackupSelectionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBackupSelectionError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(CreateBackupSelectionError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateBackupSelectionError::ServiceUnavailable(
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
impl fmt::Display for CreateBackupSelectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBackupSelectionError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateBackupSelectionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateBackupSelectionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBackupSelectionError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            CreateBackupSelectionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBackupSelectionError {}
/// Errors returned by CreateBackupVault
#[derive(Debug, PartialEq)]
pub enum CreateBackupVaultError {
    /// <p>The required resource already exists.</p>
    AlreadyExists(String),
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>A limit in the request has been exceeded; for example, a maximum number of items allowed in a request.</p>
    LimitExceeded(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl CreateBackupVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBackupVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateBackupVaultError::AlreadyExists(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateBackupVaultError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBackupVaultError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(CreateBackupVaultError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateBackupVaultError::ServiceUnavailable(
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
impl fmt::Display for CreateBackupVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBackupVaultError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateBackupVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateBackupVaultError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBackupVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            CreateBackupVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBackupVaultError {}
/// Errors returned by DeleteBackupPlan
#[derive(Debug, PartialEq)]
pub enum DeleteBackupPlanError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a parameter is of the wrong type.</p>
    InvalidRequest(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DeleteBackupPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupPlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteBackupPlanError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteBackupPlanError::InvalidRequest(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DeleteBackupPlanError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteBackupPlanError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteBackupPlanError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBackupPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupPlanError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteBackupPlanError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteBackupPlanError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteBackupPlanError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteBackupPlanError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBackupPlanError {}
/// Errors returned by DeleteBackupSelection
#[derive(Debug, PartialEq)]
pub enum DeleteBackupSelectionError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DeleteBackupSelectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupSelectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteBackupSelectionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DeleteBackupSelectionError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteBackupSelectionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteBackupSelectionError::ServiceUnavailable(
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
impl fmt::Display for DeleteBackupSelectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupSelectionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteBackupSelectionError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteBackupSelectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteBackupSelectionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBackupSelectionError {}
/// Errors returned by DeleteBackupVault
#[derive(Debug, PartialEq)]
pub enum DeleteBackupVaultError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a parameter is of the wrong type.</p>
    InvalidRequest(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DeleteBackupVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteBackupVaultError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteBackupVaultError::InvalidRequest(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DeleteBackupVaultError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteBackupVaultError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteBackupVaultError::ServiceUnavailable(
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
impl fmt::Display for DeleteBackupVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteBackupVaultError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteBackupVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteBackupVaultError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteBackupVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBackupVaultError {}
/// Errors returned by DeleteBackupVaultAccessPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteBackupVaultAccessPolicyError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DeleteBackupVaultAccessPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteBackupVaultAccessPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultAccessPolicyError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultAccessPolicyError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultAccessPolicyError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultAccessPolicyError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBackupVaultAccessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupVaultAccessPolicyError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBackupVaultAccessPolicyError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBackupVaultAccessPolicyError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBackupVaultAccessPolicyError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteBackupVaultAccessPolicyError {}
/// Errors returned by DeleteBackupVaultNotifications
#[derive(Debug, PartialEq)]
pub enum DeleteBackupVaultNotificationsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DeleteBackupVaultNotificationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteBackupVaultNotificationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultNotificationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultNotificationsError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultNotificationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteBackupVaultNotificationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBackupVaultNotificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupVaultNotificationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBackupVaultNotificationsError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBackupVaultNotificationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBackupVaultNotificationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteBackupVaultNotificationsError {}
/// Errors returned by DeleteRecoveryPoint
#[derive(Debug, PartialEq)]
pub enum DeleteRecoveryPointError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a parameter is of the wrong type.</p>
    InvalidRequest(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DeleteRecoveryPointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRecoveryPointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteRecoveryPointError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteRecoveryPointError::InvalidRequest(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DeleteRecoveryPointError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteRecoveryPointError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteRecoveryPointError::ServiceUnavailable(
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
impl fmt::Display for DeleteRecoveryPointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRecoveryPointError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteRecoveryPointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteRecoveryPointError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteRecoveryPointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteRecoveryPointError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRecoveryPointError {}
/// Errors returned by DescribeBackupJob
#[derive(Debug, PartialEq)]
pub enum DescribeBackupJobError {
    /// <p>A dependent AWS service or resource returned an error to the AWS Backup service, and the action cannot be completed.</p>
    DependencyFailure(String),
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeBackupJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBackupJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DependencyFailureException" => {
                    return RusotoError::Service(DescribeBackupJobError::DependencyFailure(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeBackupJobError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DescribeBackupJobError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeBackupJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeBackupJobError::ServiceUnavailable(
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
impl fmt::Display for DescribeBackupJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBackupJobError::DependencyFailure(ref cause) => write!(f, "{}", cause),
            DescribeBackupJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeBackupJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeBackupJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeBackupJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBackupJobError {}
/// Errors returned by DescribeBackupVault
#[derive(Debug, PartialEq)]
pub enum DescribeBackupVaultError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeBackupVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBackupVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeBackupVaultError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DescribeBackupVaultError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeBackupVaultError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeBackupVaultError::ServiceUnavailable(
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
impl fmt::Display for DescribeBackupVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBackupVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeBackupVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeBackupVaultError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeBackupVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBackupVaultError {}
/// Errors returned by DescribeCopyJob
#[derive(Debug, PartialEq)]
pub enum DescribeCopyJobError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeCopyJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCopyJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeCopyJobError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DescribeCopyJobError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCopyJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeCopyJobError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCopyJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCopyJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeCopyJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeCopyJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeCopyJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCopyJobError {}
/// Errors returned by DescribeGlobalSettings
#[derive(Debug, PartialEq)]
pub enum DescribeGlobalSettingsError {
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeGlobalSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGlobalSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeGlobalSettingsError::ServiceUnavailable(
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
impl fmt::Display for DescribeGlobalSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGlobalSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGlobalSettingsError {}
/// Errors returned by DescribeProtectedResource
#[derive(Debug, PartialEq)]
pub enum DescribeProtectedResourceError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeProtectedResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProtectedResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeProtectedResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        DescribeProtectedResourceError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProtectedResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeProtectedResourceError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProtectedResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProtectedResourceError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProtectedResourceError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProtectedResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeProtectedResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProtectedResourceError {}
/// Errors returned by DescribeRecoveryPoint
#[derive(Debug, PartialEq)]
pub enum DescribeRecoveryPointError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeRecoveryPointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRecoveryPointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeRecoveryPointError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DescribeRecoveryPointError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRecoveryPointError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeRecoveryPointError::ServiceUnavailable(
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
impl fmt::Display for DescribeRecoveryPointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRecoveryPointError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeRecoveryPointError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeRecoveryPointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRecoveryPointError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRecoveryPointError {}
/// Errors returned by DescribeRegionSettings
#[derive(Debug, PartialEq)]
pub enum DescribeRegionSettingsError {
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeRegionSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRegionSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeRegionSettingsError::ServiceUnavailable(
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
impl fmt::Display for DescribeRegionSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRegionSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRegionSettingsError {}
/// Errors returned by DescribeRestoreJob
#[derive(Debug, PartialEq)]
pub enum DescribeRestoreJobError {
    /// <p>A dependent AWS service or resource returned an error to the AWS Backup service, and the action cannot be completed.</p>
    DependencyFailure(String),
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl DescribeRestoreJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRestoreJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "DependencyFailureException" => {
                    return RusotoError::Service(DescribeRestoreJobError::DependencyFailure(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeRestoreJobError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DescribeRestoreJobError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRestoreJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeRestoreJobError::ServiceUnavailable(
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
impl fmt::Display for DescribeRestoreJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRestoreJobError::DependencyFailure(ref cause) => write!(f, "{}", cause),
            DescribeRestoreJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeRestoreJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeRestoreJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRestoreJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRestoreJobError {}
/// Errors returned by ExportBackupPlanTemplate
#[derive(Debug, PartialEq)]
pub enum ExportBackupPlanTemplateError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ExportBackupPlanTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportBackupPlanTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ExportBackupPlanTemplateError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        ExportBackupPlanTemplateError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ExportBackupPlanTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ExportBackupPlanTemplateError::ServiceUnavailable(
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
impl fmt::Display for ExportBackupPlanTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportBackupPlanTemplateError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportBackupPlanTemplateError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportBackupPlanTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ExportBackupPlanTemplateError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportBackupPlanTemplateError {}
/// Errors returned by GetBackupPlan
#[derive(Debug, PartialEq)]
pub enum GetBackupPlanError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetBackupPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBackupPlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetBackupPlanError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(GetBackupPlanError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetBackupPlanError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetBackupPlanError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBackupPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBackupPlanError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetBackupPlanError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetBackupPlanError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetBackupPlanError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBackupPlanError {}
/// Errors returned by GetBackupPlanFromJSON
#[derive(Debug, PartialEq)]
pub enum GetBackupPlanFromJSONError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a parameter is of the wrong type.</p>
    InvalidRequest(String),
    /// <p>A limit in the request has been exceeded; for example, a maximum number of items allowed in a request.</p>
    LimitExceeded(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetBackupPlanFromJSONError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBackupPlanFromJSONError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetBackupPlanFromJSONError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetBackupPlanFromJSONError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBackupPlanFromJSONError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(GetBackupPlanFromJSONError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetBackupPlanFromJSONError::ServiceUnavailable(
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
impl fmt::Display for GetBackupPlanFromJSONError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBackupPlanFromJSONError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetBackupPlanFromJSONError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetBackupPlanFromJSONError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetBackupPlanFromJSONError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetBackupPlanFromJSONError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBackupPlanFromJSONError {}
/// Errors returned by GetBackupPlanFromTemplate
#[derive(Debug, PartialEq)]
pub enum GetBackupPlanFromTemplateError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetBackupPlanFromTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBackupPlanFromTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetBackupPlanFromTemplateError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        GetBackupPlanFromTemplateError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetBackupPlanFromTemplateError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetBackupPlanFromTemplateError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBackupPlanFromTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBackupPlanFromTemplateError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetBackupPlanFromTemplateError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetBackupPlanFromTemplateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetBackupPlanFromTemplateError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBackupPlanFromTemplateError {}
/// Errors returned by GetBackupSelection
#[derive(Debug, PartialEq)]
pub enum GetBackupSelectionError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetBackupSelectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBackupSelectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetBackupSelectionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(GetBackupSelectionError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetBackupSelectionError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetBackupSelectionError::ServiceUnavailable(
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
impl fmt::Display for GetBackupSelectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBackupSelectionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetBackupSelectionError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetBackupSelectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetBackupSelectionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBackupSelectionError {}
/// Errors returned by GetBackupVaultAccessPolicy
#[derive(Debug, PartialEq)]
pub enum GetBackupVaultAccessPolicyError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetBackupVaultAccessPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBackupVaultAccessPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetBackupVaultAccessPolicyError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        GetBackupVaultAccessPolicyError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetBackupVaultAccessPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetBackupVaultAccessPolicyError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBackupVaultAccessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBackupVaultAccessPolicyError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetBackupVaultAccessPolicyError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetBackupVaultAccessPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetBackupVaultAccessPolicyError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetBackupVaultAccessPolicyError {}
/// Errors returned by GetBackupVaultNotifications
#[derive(Debug, PartialEq)]
pub enum GetBackupVaultNotificationsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetBackupVaultNotificationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBackupVaultNotificationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetBackupVaultNotificationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        GetBackupVaultNotificationsError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetBackupVaultNotificationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetBackupVaultNotificationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBackupVaultNotificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBackupVaultNotificationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetBackupVaultNotificationsError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetBackupVaultNotificationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetBackupVaultNotificationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetBackupVaultNotificationsError {}
/// Errors returned by GetRecoveryPointRestoreMetadata
#[derive(Debug, PartialEq)]
pub enum GetRecoveryPointRestoreMetadataError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetRecoveryPointRestoreMetadataError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRecoveryPointRestoreMetadataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetRecoveryPointRestoreMetadataError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        GetRecoveryPointRestoreMetadataError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetRecoveryPointRestoreMetadataError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetRecoveryPointRestoreMetadataError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRecoveryPointRestoreMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRecoveryPointRestoreMetadataError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecoveryPointRestoreMetadataError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecoveryPointRestoreMetadataError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecoveryPointRestoreMetadataError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRecoveryPointRestoreMetadataError {}
/// Errors returned by GetSupportedResourceTypes
#[derive(Debug, PartialEq)]
pub enum GetSupportedResourceTypesError {
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl GetSupportedResourceTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSupportedResourceTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetSupportedResourceTypesError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSupportedResourceTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSupportedResourceTypesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSupportedResourceTypesError {}
/// Errors returned by ListBackupJobs
#[derive(Debug, PartialEq)]
pub enum ListBackupJobsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListBackupJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBackupJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListBackupJobsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBackupJobsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBackupJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBackupJobsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupJobsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBackupJobsError {}
/// Errors returned by ListBackupPlanTemplates
#[derive(Debug, PartialEq)]
pub enum ListBackupPlanTemplatesError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListBackupPlanTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBackupPlanTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListBackupPlanTemplatesError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        ListBackupPlanTemplatesError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBackupPlanTemplatesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBackupPlanTemplatesError::ServiceUnavailable(
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
impl fmt::Display for ListBackupPlanTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBackupPlanTemplatesError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListBackupPlanTemplatesError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListBackupPlanTemplatesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListBackupPlanTemplatesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBackupPlanTemplatesError {}
/// Errors returned by ListBackupPlanVersions
#[derive(Debug, PartialEq)]
pub enum ListBackupPlanVersionsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListBackupPlanVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBackupPlanVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListBackupPlanVersionsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        ListBackupPlanVersionsError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBackupPlanVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBackupPlanVersionsError::ServiceUnavailable(
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
impl fmt::Display for ListBackupPlanVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBackupPlanVersionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupPlanVersionsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupPlanVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListBackupPlanVersionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBackupPlanVersionsError {}
/// Errors returned by ListBackupPlans
#[derive(Debug, PartialEq)]
pub enum ListBackupPlansError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListBackupPlansError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBackupPlansError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListBackupPlansError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListBackupPlansError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBackupPlansError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBackupPlansError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBackupPlansError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBackupPlansError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupPlansError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupPlansError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListBackupPlansError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBackupPlansError {}
/// Errors returned by ListBackupSelections
#[derive(Debug, PartialEq)]
pub enum ListBackupSelectionsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListBackupSelectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBackupSelectionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListBackupSelectionsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListBackupSelectionsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBackupSelectionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBackupSelectionsError::ServiceUnavailable(
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
impl fmt::Display for ListBackupSelectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBackupSelectionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupSelectionsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupSelectionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListBackupSelectionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBackupSelectionsError {}
/// Errors returned by ListBackupVaults
#[derive(Debug, PartialEq)]
pub enum ListBackupVaultsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListBackupVaultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBackupVaultsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListBackupVaultsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListBackupVaultsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListBackupVaultsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBackupVaultsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBackupVaultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBackupVaultsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupVaultsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListBackupVaultsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListBackupVaultsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBackupVaultsError {}
/// Errors returned by ListCopyJobs
#[derive(Debug, PartialEq)]
pub enum ListCopyJobsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListCopyJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCopyJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListCopyJobsError::InvalidParameterValue(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListCopyJobsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCopyJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCopyJobsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListCopyJobsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCopyJobsError {}
/// Errors returned by ListProtectedResources
#[derive(Debug, PartialEq)]
pub enum ListProtectedResourcesError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListProtectedResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProtectedResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListProtectedResourcesError::InvalidParameterValue(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListProtectedResourcesError::ServiceUnavailable(
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
impl fmt::Display for ListProtectedResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProtectedResourcesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListProtectedResourcesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProtectedResourcesError {}
/// Errors returned by ListRecoveryPointsByBackupVault
#[derive(Debug, PartialEq)]
pub enum ListRecoveryPointsByBackupVaultError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListRecoveryPointsByBackupVaultError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRecoveryPointsByBackupVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByBackupVaultError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByBackupVaultError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByBackupVaultError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByBackupVaultError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRecoveryPointsByBackupVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRecoveryPointsByBackupVaultError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRecoveryPointsByBackupVaultError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRecoveryPointsByBackupVaultError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRecoveryPointsByBackupVaultError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListRecoveryPointsByBackupVaultError {}
/// Errors returned by ListRecoveryPointsByResource
#[derive(Debug, PartialEq)]
pub enum ListRecoveryPointsByResourceError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListRecoveryPointsByResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRecoveryPointsByResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByResourceError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByResourceError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListRecoveryPointsByResourceError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRecoveryPointsByResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRecoveryPointsByResourceError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRecoveryPointsByResourceError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRecoveryPointsByResourceError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRecoveryPointsByResourceError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListRecoveryPointsByResourceError {}
/// Errors returned by ListRestoreJobs
#[derive(Debug, PartialEq)]
pub enum ListRestoreJobsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListRestoreJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRestoreJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListRestoreJobsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListRestoreJobsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRestoreJobsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListRestoreJobsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRestoreJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRestoreJobsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListRestoreJobsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListRestoreJobsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRestoreJobsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRestoreJobsError {}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTagsError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListTagsError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListTagsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTagsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsError {}
/// Errors returned by PutBackupVaultAccessPolicy
#[derive(Debug, PartialEq)]
pub enum PutBackupVaultAccessPolicyError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl PutBackupVaultAccessPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBackupVaultAccessPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutBackupVaultAccessPolicyError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        PutBackupVaultAccessPolicyError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutBackupVaultAccessPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PutBackupVaultAccessPolicyError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutBackupVaultAccessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutBackupVaultAccessPolicyError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutBackupVaultAccessPolicyError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutBackupVaultAccessPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutBackupVaultAccessPolicyError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutBackupVaultAccessPolicyError {}
/// Errors returned by PutBackupVaultNotifications
#[derive(Debug, PartialEq)]
pub enum PutBackupVaultNotificationsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl PutBackupVaultNotificationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBackupVaultNotificationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutBackupVaultNotificationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        PutBackupVaultNotificationsError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        PutBackupVaultNotificationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PutBackupVaultNotificationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutBackupVaultNotificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutBackupVaultNotificationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutBackupVaultNotificationsError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutBackupVaultNotificationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutBackupVaultNotificationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutBackupVaultNotificationsError {}
/// Errors returned by StartBackupJob
#[derive(Debug, PartialEq)]
pub enum StartBackupJobError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a parameter is of the wrong type.</p>
    InvalidRequest(String),
    /// <p>A limit in the request has been exceeded; for example, a maximum number of items allowed in a request.</p>
    LimitExceeded(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl StartBackupJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartBackupJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StartBackupJobError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartBackupJobError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartBackupJobError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(StartBackupJobError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartBackupJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartBackupJobError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartBackupJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartBackupJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StartBackupJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartBackupJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartBackupJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            StartBackupJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartBackupJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartBackupJobError {}
/// Errors returned by StartCopyJob
#[derive(Debug, PartialEq)]
pub enum StartCopyJobError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>A limit in the request has been exceeded; for example, a maximum number of items allowed in a request.</p>
    LimitExceeded(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl StartCopyJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartCopyJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StartCopyJobError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartCopyJobError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(StartCopyJobError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartCopyJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartCopyJobError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartCopyJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartCopyJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StartCopyJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartCopyJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            StartCopyJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartCopyJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartCopyJobError {}
/// Errors returned by StartRestoreJob
#[derive(Debug, PartialEq)]
pub enum StartRestoreJobError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl StartRestoreJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartRestoreJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StartRestoreJobError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(StartRestoreJobError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartRestoreJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartRestoreJobError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartRestoreJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartRestoreJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StartRestoreJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            StartRestoreJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartRestoreJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartRestoreJobError {}
/// Errors returned by StopBackupJob
#[derive(Debug, PartialEq)]
pub enum StopBackupJobError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a parameter is of the wrong type.</p>
    InvalidRequest(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl StopBackupJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopBackupJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StopBackupJobError::InvalidParameterValue(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopBackupJobError::InvalidRequest(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(StopBackupJobError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopBackupJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StopBackupJobError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopBackupJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopBackupJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StopBackupJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopBackupJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            StopBackupJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopBackupJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopBackupJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>A limit in the request has been exceeded; for example, a maximum number of items allowed in a request.</p>
    LimitExceeded(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(TagResourceError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TagResourceError::ServiceUnavailable(err.msg))
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
            TagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(UntagResourceError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UntagResourceError::ServiceUnavailable(err.msg))
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
            UntagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UntagResourceError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateBackupPlan
#[derive(Debug, PartialEq)]
pub enum UpdateBackupPlanError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl UpdateBackupPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBackupPlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateBackupPlanError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(UpdateBackupPlanError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateBackupPlanError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateBackupPlanError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBackupPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBackupPlanError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateBackupPlanError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateBackupPlanError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateBackupPlanError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBackupPlanError {}
/// Errors returned by UpdateGlobalSettings
#[derive(Debug, PartialEq)]
pub enum UpdateGlobalSettingsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a parameter is of the wrong type.</p>
    InvalidRequest(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl UpdateGlobalSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGlobalSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::InvalidRequest(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::ServiceUnavailable(
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
impl fmt::Display for UpdateGlobalSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGlobalSettingsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGlobalSettingsError {}
/// Errors returned by UpdateRecoveryPointLifecycle
#[derive(Debug, PartialEq)]
pub enum UpdateRecoveryPointLifecycleError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl UpdateRecoveryPointLifecycleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRecoveryPointLifecycleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateRecoveryPointLifecycleError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        UpdateRecoveryPointLifecycleError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateRecoveryPointLifecycleError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        UpdateRecoveryPointLifecycleError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRecoveryPointLifecycleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRecoveryPointLifecycleError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRecoveryPointLifecycleError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRecoveryPointLifecycleError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRecoveryPointLifecycleError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateRecoveryPointLifecycleError {}
/// Errors returned by UpdateRegionSettings
#[derive(Debug, PartialEq)]
pub enum UpdateRegionSettingsError {
    /// <p>Indicates that something is wrong with a parameter's value. For example, the value is out of range.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that a required parameter is missing.</p>
    MissingParameterValue(String),
    /// <p>The request failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
}

impl UpdateRegionSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRegionSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateRegionSettingsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(UpdateRegionSettingsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateRegionSettingsError::ServiceUnavailable(
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
impl fmt::Display for UpdateRegionSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRegionSettingsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateRegionSettingsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateRegionSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRegionSettingsError {}
/// Trait representing the capabilities of the AWS Backup API. AWS Backup clients implement this trait.
#[async_trait]
pub trait Backup {
    /// <p>Creates a backup plan using a backup plan name and backup rules. A backup plan is a document that contains information that AWS Backup uses to schedule tasks that create recovery points for resources.</p> <p>If you call <code>CreateBackupPlan</code> with a plan that already exists, an <code>AlreadyExistsException</code> is returned.</p>
    async fn create_backup_plan(
        &self,
        input: CreateBackupPlanInput,
    ) -> Result<CreateBackupPlanOutput, RusotoError<CreateBackupPlanError>>;

    /// <p>Creates a JSON document that specifies a set of resources to assign to a backup plan. Resources can be included by specifying patterns for a <code>ListOfTags</code> and selected <code>Resources</code>. </p> <p>For example, consider the following patterns:</p> <ul> <li> <p> <code>Resources: "arn:aws:ec2:region:account-id:volume/volume-id"</code> </p> </li> <li> <p> <code>ConditionKey:"department"</code> </p> <p> <code>ConditionValue:"finance"</code> </p> <p> <code>ConditionType:"StringEquals"</code> </p> </li> <li> <p> <code>ConditionKey:"importance"</code> </p> <p> <code>ConditionValue:"critical"</code> </p> <p> <code>ConditionType:"StringEquals"</code> </p> </li> </ul> <p>Using these patterns would back up all Amazon Elastic Block Store (Amazon EBS) volumes that are tagged as <code>"department=finance"</code>, <code>"importance=critical"</code>, in addition to an EBS volume with the specified volume ID.</p> <p>Resources and conditions are additive in that all resources that match the pattern are selected. This shouldn't be confused with a logical AND, where all conditions must match. The matching patterns are logically put together using the OR operator. In other words, all patterns that match are selected for backup.</p>
    async fn create_backup_selection(
        &self,
        input: CreateBackupSelectionInput,
    ) -> Result<CreateBackupSelectionOutput, RusotoError<CreateBackupSelectionError>>;

    /// <p><p>Creates a logical container where backups are stored. A <code>CreateBackupVault</code> request includes a name, optionally one or more resource tags, an encryption key, and a request ID.</p> <note> <p>Sensitive data, such as passport numbers, should not be included the name of a backup vault.</p> </note></p>
    async fn create_backup_vault(
        &self,
        input: CreateBackupVaultInput,
    ) -> Result<CreateBackupVaultOutput, RusotoError<CreateBackupVaultError>>;

    /// <p>Deletes a backup plan. A backup plan can only be deleted after all associated selections of resources have been deleted. Deleting a backup plan deletes the current version of a backup plan. Previous versions, if any, will still exist.</p>
    async fn delete_backup_plan(
        &self,
        input: DeleteBackupPlanInput,
    ) -> Result<DeleteBackupPlanOutput, RusotoError<DeleteBackupPlanError>>;

    /// <p>Deletes the resource selection associated with a backup plan that is specified by the <code>SelectionId</code>.</p>
    async fn delete_backup_selection(
        &self,
        input: DeleteBackupSelectionInput,
    ) -> Result<(), RusotoError<DeleteBackupSelectionError>>;

    /// <p>Deletes the backup vault identified by its name. A vault can be deleted only if it is empty.</p>
    async fn delete_backup_vault(
        &self,
        input: DeleteBackupVaultInput,
    ) -> Result<(), RusotoError<DeleteBackupVaultError>>;

    /// <p>Deletes the policy document that manages permissions on a backup vault.</p>
    async fn delete_backup_vault_access_policy(
        &self,
        input: DeleteBackupVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<DeleteBackupVaultAccessPolicyError>>;

    /// <p>Deletes event notifications for the specified backup vault.</p>
    async fn delete_backup_vault_notifications(
        &self,
        input: DeleteBackupVaultNotificationsInput,
    ) -> Result<(), RusotoError<DeleteBackupVaultNotificationsError>>;

    /// <p>Deletes the recovery point specified by a recovery point ID.</p>
    async fn delete_recovery_point(
        &self,
        input: DeleteRecoveryPointInput,
    ) -> Result<(), RusotoError<DeleteRecoveryPointError>>;

    /// <p>Returns backup job details for the specified <code>BackupJobId</code>.</p>
    async fn describe_backup_job(
        &self,
        input: DescribeBackupJobInput,
    ) -> Result<DescribeBackupJobOutput, RusotoError<DescribeBackupJobError>>;

    /// <p>Returns metadata about a backup vault specified by its name.</p>
    async fn describe_backup_vault(
        &self,
        input: DescribeBackupVaultInput,
    ) -> Result<DescribeBackupVaultOutput, RusotoError<DescribeBackupVaultError>>;

    /// <p>Returns metadata associated with creating a copy of a resource.</p>
    async fn describe_copy_job(
        &self,
        input: DescribeCopyJobInput,
    ) -> Result<DescribeCopyJobOutput, RusotoError<DescribeCopyJobError>>;

    /// <p>The current feature settings for the AWS Account.</p>
    async fn describe_global_settings(
        &self,
    ) -> Result<DescribeGlobalSettingsOutput, RusotoError<DescribeGlobalSettingsError>>;

    /// <p>Returns information about a saved resource, including the last time it was backed up, its Amazon Resource Name (ARN), and the AWS service type of the saved resource.</p>
    async fn describe_protected_resource(
        &self,
        input: DescribeProtectedResourceInput,
    ) -> Result<DescribeProtectedResourceOutput, RusotoError<DescribeProtectedResourceError>>;

    /// <p>Returns metadata associated with a recovery point, including ID, status, encryption, and lifecycle.</p>
    async fn describe_recovery_point(
        &self,
        input: DescribeRecoveryPointInput,
    ) -> Result<DescribeRecoveryPointOutput, RusotoError<DescribeRecoveryPointError>>;

    /// <p>Returns the current service opt-in settings for the Region. If service-opt-in is enabled for a service, AWS Backup tries to protect that service's resources in this Region, when the resource is included in an on-demand backup or scheduled backup plan. Otherwise, AWS Backup does not try to protect that service's resources in this Region, AWS Backup does not try to protect that service's resources in this Region.</p>
    async fn describe_region_settings(
        &self,
    ) -> Result<DescribeRegionSettingsOutput, RusotoError<DescribeRegionSettingsError>>;

    /// <p>Returns metadata associated with a restore job that is specified by a job ID.</p>
    async fn describe_restore_job(
        &self,
        input: DescribeRestoreJobInput,
    ) -> Result<DescribeRestoreJobOutput, RusotoError<DescribeRestoreJobError>>;

    /// <p>Returns the backup plan that is specified by the plan ID as a backup template.</p>
    async fn export_backup_plan_template(
        &self,
        input: ExportBackupPlanTemplateInput,
    ) -> Result<ExportBackupPlanTemplateOutput, RusotoError<ExportBackupPlanTemplateError>>;

    /// <p>Returns <code>BackupPlan</code> details for the specified <code>BackupPlanId</code>. Returns the body of a backup plan in JSON format, in addition to plan metadata.</p>
    async fn get_backup_plan(
        &self,
        input: GetBackupPlanInput,
    ) -> Result<GetBackupPlanOutput, RusotoError<GetBackupPlanError>>;

    /// <p>Returns a valid JSON document specifying a backup plan or an error.</p>
    async fn get_backup_plan_from_json(
        &self,
        input: GetBackupPlanFromJSONInput,
    ) -> Result<GetBackupPlanFromJSONOutput, RusotoError<GetBackupPlanFromJSONError>>;

    /// <p>Returns the template specified by its <code>templateId</code> as a backup plan.</p>
    async fn get_backup_plan_from_template(
        &self,
        input: GetBackupPlanFromTemplateInput,
    ) -> Result<GetBackupPlanFromTemplateOutput, RusotoError<GetBackupPlanFromTemplateError>>;

    /// <p>Returns selection metadata and a document in JSON format that specifies a list of resources that are associated with a backup plan.</p>
    async fn get_backup_selection(
        &self,
        input: GetBackupSelectionInput,
    ) -> Result<GetBackupSelectionOutput, RusotoError<GetBackupSelectionError>>;

    /// <p>Returns the access policy document that is associated with the named backup vault.</p>
    async fn get_backup_vault_access_policy(
        &self,
        input: GetBackupVaultAccessPolicyInput,
    ) -> Result<GetBackupVaultAccessPolicyOutput, RusotoError<GetBackupVaultAccessPolicyError>>;

    /// <p>Returns event notifications for the specified backup vault.</p>
    async fn get_backup_vault_notifications(
        &self,
        input: GetBackupVaultNotificationsInput,
    ) -> Result<GetBackupVaultNotificationsOutput, RusotoError<GetBackupVaultNotificationsError>>;

    /// <p>Returns a set of metadata key-value pairs that were used to create the backup.</p>
    async fn get_recovery_point_restore_metadata(
        &self,
        input: GetRecoveryPointRestoreMetadataInput,
    ) -> Result<
        GetRecoveryPointRestoreMetadataOutput,
        RusotoError<GetRecoveryPointRestoreMetadataError>,
    >;

    /// <p>Returns the AWS resource types supported by AWS Backup.</p>
    async fn get_supported_resource_types(
        &self,
    ) -> Result<GetSupportedResourceTypesOutput, RusotoError<GetSupportedResourceTypesError>>;

    /// <p>Returns a list of existing backup jobs for an authenticated account.</p>
    async fn list_backup_jobs(
        &self,
        input: ListBackupJobsInput,
    ) -> Result<ListBackupJobsOutput, RusotoError<ListBackupJobsError>>;

    /// <p>Returns metadata of your saved backup plan templates, including the template ID, name, and the creation and deletion dates.</p>
    async fn list_backup_plan_templates(
        &self,
        input: ListBackupPlanTemplatesInput,
    ) -> Result<ListBackupPlanTemplatesOutput, RusotoError<ListBackupPlanTemplatesError>>;

    /// <p>Returns version metadata of your backup plans, including Amazon Resource Names (ARNs), backup plan IDs, creation and deletion dates, plan names, and version IDs.</p>
    async fn list_backup_plan_versions(
        &self,
        input: ListBackupPlanVersionsInput,
    ) -> Result<ListBackupPlanVersionsOutput, RusotoError<ListBackupPlanVersionsError>>;

    /// <p>Returns a list of existing backup plans for an authenticated account. The list is populated only if the advanced option is set for the backup plan. The list contains information such as Amazon Resource Names (ARNs), plan IDs, creation and deletion dates, version IDs, plan names, and creator request IDs.</p>
    async fn list_backup_plans(
        &self,
        input: ListBackupPlansInput,
    ) -> Result<ListBackupPlansOutput, RusotoError<ListBackupPlansError>>;

    /// <p>Returns an array containing metadata of the resources associated with the target backup plan.</p>
    async fn list_backup_selections(
        &self,
        input: ListBackupSelectionsInput,
    ) -> Result<ListBackupSelectionsOutput, RusotoError<ListBackupSelectionsError>>;

    /// <p>Returns a list of recovery point storage containers along with information about them.</p>
    async fn list_backup_vaults(
        &self,
        input: ListBackupVaultsInput,
    ) -> Result<ListBackupVaultsOutput, RusotoError<ListBackupVaultsError>>;

    /// <p>Returns metadata about your copy jobs.</p>
    async fn list_copy_jobs(
        &self,
        input: ListCopyJobsInput,
    ) -> Result<ListCopyJobsOutput, RusotoError<ListCopyJobsError>>;

    /// <p>Returns an array of resources successfully backed up by AWS Backup, including the time the resource was saved, an Amazon Resource Name (ARN) of the resource, and a resource type.</p>
    async fn list_protected_resources(
        &self,
        input: ListProtectedResourcesInput,
    ) -> Result<ListProtectedResourcesOutput, RusotoError<ListProtectedResourcesError>>;

    /// <p>Returns detailed information about the recovery points stored in a backup vault.</p>
    async fn list_recovery_points_by_backup_vault(
        &self,
        input: ListRecoveryPointsByBackupVaultInput,
    ) -> Result<
        ListRecoveryPointsByBackupVaultOutput,
        RusotoError<ListRecoveryPointsByBackupVaultError>,
    >;

    /// <p>Returns detailed information about recovery points of the type specified by a resource Amazon Resource Name (ARN).</p>
    async fn list_recovery_points_by_resource(
        &self,
        input: ListRecoveryPointsByResourceInput,
    ) -> Result<ListRecoveryPointsByResourceOutput, RusotoError<ListRecoveryPointsByResourceError>>;

    /// <p>Returns a list of jobs that AWS Backup initiated to restore a saved resource, including metadata about the recovery process.</p>
    async fn list_restore_jobs(
        &self,
        input: ListRestoreJobsInput,
    ) -> Result<ListRestoreJobsOutput, RusotoError<ListRestoreJobsError>>;

    /// <p><p>Returns a list of key-value pairs assigned to a target recovery point, backup plan, or backup vault.</p> <note> <p> <code>ListTags</code> are currently only supported with Amazon EFS backups.</p> </note></p>
    async fn list_tags(
        &self,
        input: ListTagsInput,
    ) -> Result<ListTagsOutput, RusotoError<ListTagsError>>;

    /// <p>Sets a resource-based policy that is used to manage access permissions on the target backup vault. Requires a backup vault name and an access policy document in JSON format.</p>
    async fn put_backup_vault_access_policy(
        &self,
        input: PutBackupVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<PutBackupVaultAccessPolicyError>>;

    /// <p>Turns on notifications on a backup vault for the specified topic and events.</p>
    async fn put_backup_vault_notifications(
        &self,
        input: PutBackupVaultNotificationsInput,
    ) -> Result<(), RusotoError<PutBackupVaultNotificationsError>>;

    /// <p>Starts an on-demand backup job for the specified resource.</p>
    async fn start_backup_job(
        &self,
        input: StartBackupJobInput,
    ) -> Result<StartBackupJobOutput, RusotoError<StartBackupJobError>>;

    /// <p>Starts a job to create a one-time copy of the specified resource.</p>
    async fn start_copy_job(
        &self,
        input: StartCopyJobInput,
    ) -> Result<StartCopyJobOutput, RusotoError<StartCopyJobError>>;

    /// <p>Recovers the saved resource identified by an Amazon Resource Name (ARN). </p>
    async fn start_restore_job(
        &self,
        input: StartRestoreJobInput,
    ) -> Result<StartRestoreJobOutput, RusotoError<StartRestoreJobError>>;

    /// <p>Attempts to cancel a job to create a one-time backup of a resource.</p>
    async fn stop_backup_job(
        &self,
        input: StopBackupJobInput,
    ) -> Result<(), RusotoError<StopBackupJobError>>;

    /// <p>Assigns a set of key-value pairs to a recovery point, backup plan, or backup vault identified by an Amazon Resource Name (ARN).</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes a set of key-value pairs from a recovery point, backup plan, or backup vault identified by an Amazon Resource Name (ARN)</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates an existing backup plan identified by its <code>backupPlanId</code> with the input document in JSON format. The new version is uniquely identified by a <code>VersionId</code>.</p>
    async fn update_backup_plan(
        &self,
        input: UpdateBackupPlanInput,
    ) -> Result<UpdateBackupPlanOutput, RusotoError<UpdateBackupPlanError>>;

    /// <p>Updates the current global settings for the AWS Account. Use the <code>DescribeGlobalSettings</code> API to determine the current settings.</p>
    async fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsInput,
    ) -> Result<(), RusotoError<UpdateGlobalSettingsError>>;

    /// <p>Sets the transition lifecycle of a recovery point.</p> <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    async fn update_recovery_point_lifecycle(
        &self,
        input: UpdateRecoveryPointLifecycleInput,
    ) -> Result<UpdateRecoveryPointLifecycleOutput, RusotoError<UpdateRecoveryPointLifecycleError>>;

    /// <p>Updates the current service opt-in settings for the Region. If service-opt-in is enabled for a service, AWS Backup tries to protect that service's resources in this Region, when the resource is included in an on-demand backup or scheduled backup plan. Otherwise, AWS Backup does not try to protect that service's resources in this Region. Use the <code>DescribeRegionSettings</code> API to determine the resource types that are supported.</p>
    async fn update_region_settings(
        &self,
        input: UpdateRegionSettingsInput,
    ) -> Result<(), RusotoError<UpdateRegionSettingsError>>;
}
/// A client for the AWS Backup API.
#[derive(Clone)]
pub struct BackupClient {
    client: Client,
    region: region::Region,
}

impl BackupClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> BackupClient {
        BackupClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> BackupClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        BackupClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> BackupClient {
        BackupClient { client, region }
    }
}

#[async_trait]
impl Backup for BackupClient {
    /// <p>Creates a backup plan using a backup plan name and backup rules. A backup plan is a document that contains information that AWS Backup uses to schedule tasks that create recovery points for resources.</p> <p>If you call <code>CreateBackupPlan</code> with a plan that already exists, an <code>AlreadyExistsException</code> is returned.</p>
    #[allow(unused_mut)]
    async fn create_backup_plan(
        &self,
        input: CreateBackupPlanInput,
    ) -> Result<CreateBackupPlanOutput, RusotoError<CreateBackupPlanError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/backup/plans/";

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateBackupPlanOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBackupPlanError::from_response(response))
        }
    }

    /// <p>Creates a JSON document that specifies a set of resources to assign to a backup plan. Resources can be included by specifying patterns for a <code>ListOfTags</code> and selected <code>Resources</code>. </p> <p>For example, consider the following patterns:</p> <ul> <li> <p> <code>Resources: "arn:aws:ec2:region:account-id:volume/volume-id"</code> </p> </li> <li> <p> <code>ConditionKey:"department"</code> </p> <p> <code>ConditionValue:"finance"</code> </p> <p> <code>ConditionType:"StringEquals"</code> </p> </li> <li> <p> <code>ConditionKey:"importance"</code> </p> <p> <code>ConditionValue:"critical"</code> </p> <p> <code>ConditionType:"StringEquals"</code> </p> </li> </ul> <p>Using these patterns would back up all Amazon Elastic Block Store (Amazon EBS) volumes that are tagged as <code>"department=finance"</code>, <code>"importance=critical"</code>, in addition to an EBS volume with the specified volume ID.</p> <p>Resources and conditions are additive in that all resources that match the pattern are selected. This shouldn't be confused with a logical AND, where all conditions must match. The matching patterns are logically put together using the OR operator. In other words, all patterns that match are selected for backup.</p>
    #[allow(unused_mut)]
    async fn create_backup_selection(
        &self,
        input: CreateBackupSelectionInput,
    ) -> Result<CreateBackupSelectionOutput, RusotoError<CreateBackupSelectionError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}/selections/",
            backup_plan_id = input.backup_plan_id
        );

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateBackupSelectionOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBackupSelectionError::from_response(response))
        }
    }

    /// <p><p>Creates a logical container where backups are stored. A <code>CreateBackupVault</code> request includes a name, optionally one or more resource tags, an encryption key, and a request ID.</p> <note> <p>Sensitive data, such as passport numbers, should not be included the name of a backup vault.</p> </note></p>
    #[allow(unused_mut)]
    async fn create_backup_vault(
        &self,
        input: CreateBackupVaultInput,
    ) -> Result<CreateBackupVaultOutput, RusotoError<CreateBackupVaultError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateBackupVaultOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBackupVaultError::from_response(response))
        }
    }

    /// <p>Deletes a backup plan. A backup plan can only be deleted after all associated selections of resources have been deleted. Deleting a backup plan deletes the current version of a backup plan. Previous versions, if any, will still exist.</p>
    #[allow(unused_mut)]
    async fn delete_backup_plan(
        &self,
        input: DeleteBackupPlanInput,
    ) -> Result<DeleteBackupPlanOutput, RusotoError<DeleteBackupPlanError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}",
            backup_plan_id = input.backup_plan_id
        );

        let mut request = SignedRequest::new("DELETE", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteBackupPlanOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBackupPlanError::from_response(response))
        }
    }

    /// <p>Deletes the resource selection associated with a backup plan that is specified by the <code>SelectionId</code>.</p>
    #[allow(unused_mut)]
    async fn delete_backup_selection(
        &self,
        input: DeleteBackupSelectionInput,
    ) -> Result<(), RusotoError<DeleteBackupSelectionError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}/selections/{selection_id}",
            backup_plan_id = input.backup_plan_id,
            selection_id = input.selection_id
        );

        let mut request = SignedRequest::new("DELETE", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBackupSelectionError::from_response(response))
        }
    }

    /// <p>Deletes the backup vault identified by its name. A vault can be deleted only if it is empty.</p>
    #[allow(unused_mut)]
    async fn delete_backup_vault(
        &self,
        input: DeleteBackupVaultInput,
    ) -> Result<(), RusotoError<DeleteBackupVaultError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("DELETE", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBackupVaultError::from_response(response))
        }
    }

    /// <p>Deletes the policy document that manages permissions on a backup vault.</p>
    #[allow(unused_mut)]
    async fn delete_backup_vault_access_policy(
        &self,
        input: DeleteBackupVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<DeleteBackupVaultAccessPolicyError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/access-policy",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("DELETE", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBackupVaultAccessPolicyError::from_response(response))
        }
    }

    /// <p>Deletes event notifications for the specified backup vault.</p>
    #[allow(unused_mut)]
    async fn delete_backup_vault_notifications(
        &self,
        input: DeleteBackupVaultNotificationsInput,
    ) -> Result<(), RusotoError<DeleteBackupVaultNotificationsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/notification-configuration",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("DELETE", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBackupVaultNotificationsError::from_response(response))
        }
    }

    /// <p>Deletes the recovery point specified by a recovery point ID.</p>
    #[allow(unused_mut)]
    async fn delete_recovery_point(
        &self,
        input: DeleteRecoveryPointInput,
    ) -> Result<(), RusotoError<DeleteRecoveryPointError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/recovery-points/{recovery_point_arn}",
            backup_vault_name = input.backup_vault_name,
            recovery_point_arn = input.recovery_point_arn
        );

        let mut request = SignedRequest::new("DELETE", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRecoveryPointError::from_response(response))
        }
    }

    /// <p>Returns backup job details for the specified <code>BackupJobId</code>.</p>
    #[allow(unused_mut)]
    async fn describe_backup_job(
        &self,
        input: DescribeBackupJobInput,
    ) -> Result<DescribeBackupJobOutput, RusotoError<DescribeBackupJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-jobs/{backup_job_id}",
            backup_job_id = input.backup_job_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeBackupJobOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBackupJobError::from_response(response))
        }
    }

    /// <p>Returns metadata about a backup vault specified by its name.</p>
    #[allow(unused_mut)]
    async fn describe_backup_vault(
        &self,
        input: DescribeBackupVaultInput,
    ) -> Result<DescribeBackupVaultOutput, RusotoError<DescribeBackupVaultError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeBackupVaultOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBackupVaultError::from_response(response))
        }
    }

    /// <p>Returns metadata associated with creating a copy of a resource.</p>
    #[allow(unused_mut)]
    async fn describe_copy_job(
        &self,
        input: DescribeCopyJobInput,
    ) -> Result<DescribeCopyJobOutput, RusotoError<DescribeCopyJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/copy-jobs/{copy_job_id}", copy_job_id = input.copy_job_id);

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeCopyJobOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCopyJobError::from_response(response))
        }
    }

    /// <p>The current feature settings for the AWS Account.</p>
    #[allow(unused_mut)]
    async fn describe_global_settings(
        &self,
    ) -> Result<DescribeGlobalSettingsOutput, RusotoError<DescribeGlobalSettingsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/global-settings";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeGlobalSettingsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeGlobalSettingsError::from_response(response))
        }
    }

    /// <p>Returns information about a saved resource, including the last time it was backed up, its Amazon Resource Name (ARN), and the AWS service type of the saved resource.</p>
    #[allow(unused_mut)]
    async fn describe_protected_resource(
        &self,
        input: DescribeProtectedResourceInput,
    ) -> Result<DescribeProtectedResourceOutput, RusotoError<DescribeProtectedResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/resources/{resource_arn}",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeProtectedResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProtectedResourceError::from_response(response))
        }
    }

    /// <p>Returns metadata associated with a recovery point, including ID, status, encryption, and lifecycle.</p>
    #[allow(unused_mut)]
    async fn describe_recovery_point(
        &self,
        input: DescribeRecoveryPointInput,
    ) -> Result<DescribeRecoveryPointOutput, RusotoError<DescribeRecoveryPointError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/recovery-points/{recovery_point_arn}",
            backup_vault_name = input.backup_vault_name,
            recovery_point_arn = input.recovery_point_arn
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRecoveryPointOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRecoveryPointError::from_response(response))
        }
    }

    /// <p>Returns the current service opt-in settings for the Region. If service-opt-in is enabled for a service, AWS Backup tries to protect that service's resources in this Region, when the resource is included in an on-demand backup or scheduled backup plan. Otherwise, AWS Backup does not try to protect that service's resources in this Region, AWS Backup does not try to protect that service's resources in this Region.</p>
    #[allow(unused_mut)]
    async fn describe_region_settings(
        &self,
    ) -> Result<DescribeRegionSettingsOutput, RusotoError<DescribeRegionSettingsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/account-settings";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRegionSettingsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRegionSettingsError::from_response(response))
        }
    }

    /// <p>Returns metadata associated with a restore job that is specified by a job ID.</p>
    #[allow(unused_mut)]
    async fn describe_restore_job(
        &self,
        input: DescribeRestoreJobInput,
    ) -> Result<DescribeRestoreJobOutput, RusotoError<DescribeRestoreJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/restore-jobs/{restore_job_id}",
            restore_job_id = input.restore_job_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRestoreJobOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRestoreJobError::from_response(response))
        }
    }

    /// <p>Returns the backup plan that is specified by the plan ID as a backup template.</p>
    #[allow(unused_mut)]
    async fn export_backup_plan_template(
        &self,
        input: ExportBackupPlanTemplateInput,
    ) -> Result<ExportBackupPlanTemplateOutput, RusotoError<ExportBackupPlanTemplateError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}/toTemplate/",
            backup_plan_id = input.backup_plan_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ExportBackupPlanTemplateOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ExportBackupPlanTemplateError::from_response(response))
        }
    }

    /// <p>Returns <code>BackupPlan</code> details for the specified <code>BackupPlanId</code>. Returns the body of a backup plan in JSON format, in addition to plan metadata.</p>
    #[allow(unused_mut)]
    async fn get_backup_plan(
        &self,
        input: GetBackupPlanInput,
    ) -> Result<GetBackupPlanOutput, RusotoError<GetBackupPlanError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}/",
            backup_plan_id = input.backup_plan_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBackupPlanOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBackupPlanError::from_response(response))
        }
    }

    /// <p>Returns a valid JSON document specifying a backup plan or an error.</p>
    #[allow(unused_mut)]
    async fn get_backup_plan_from_json(
        &self,
        input: GetBackupPlanFromJSONInput,
    ) -> Result<GetBackupPlanFromJSONOutput, RusotoError<GetBackupPlanFromJSONError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/backup/template/json/toPlan";

        let mut request = SignedRequest::new("POST", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBackupPlanFromJSONOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBackupPlanFromJSONError::from_response(response))
        }
    }

    /// <p>Returns the template specified by its <code>templateId</code> as a backup plan.</p>
    #[allow(unused_mut)]
    async fn get_backup_plan_from_template(
        &self,
        input: GetBackupPlanFromTemplateInput,
    ) -> Result<GetBackupPlanFromTemplateOutput, RusotoError<GetBackupPlanFromTemplateError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/template/plans/{template_id}/toPlan",
            template_id = input.backup_plan_template_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBackupPlanFromTemplateOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBackupPlanFromTemplateError::from_response(response))
        }
    }

    /// <p>Returns selection metadata and a document in JSON format that specifies a list of resources that are associated with a backup plan.</p>
    #[allow(unused_mut)]
    async fn get_backup_selection(
        &self,
        input: GetBackupSelectionInput,
    ) -> Result<GetBackupSelectionOutput, RusotoError<GetBackupSelectionError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}/selections/{selection_id}",
            backup_plan_id = input.backup_plan_id,
            selection_id = input.selection_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBackupSelectionOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBackupSelectionError::from_response(response))
        }
    }

    /// <p>Returns the access policy document that is associated with the named backup vault.</p>
    #[allow(unused_mut)]
    async fn get_backup_vault_access_policy(
        &self,
        input: GetBackupVaultAccessPolicyInput,
    ) -> Result<GetBackupVaultAccessPolicyOutput, RusotoError<GetBackupVaultAccessPolicyError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/access-policy",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBackupVaultAccessPolicyOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBackupVaultAccessPolicyError::from_response(response))
        }
    }

    /// <p>Returns event notifications for the specified backup vault.</p>
    #[allow(unused_mut)]
    async fn get_backup_vault_notifications(
        &self,
        input: GetBackupVaultNotificationsInput,
    ) -> Result<GetBackupVaultNotificationsOutput, RusotoError<GetBackupVaultNotificationsError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/notification-configuration",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBackupVaultNotificationsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBackupVaultNotificationsError::from_response(response))
        }
    }

    /// <p>Returns a set of metadata key-value pairs that were used to create the backup.</p>
    #[allow(unused_mut)]
    async fn get_recovery_point_restore_metadata(
        &self,
        input: GetRecoveryPointRestoreMetadataInput,
    ) -> Result<
        GetRecoveryPointRestoreMetadataOutput,
        RusotoError<GetRecoveryPointRestoreMetadataError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/backup-vaults/{backup_vault_name}/recovery-points/{recovery_point_arn}/restore-metadata", backup_vault_name = input.backup_vault_name, recovery_point_arn = input.recovery_point_arn);

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRecoveryPointRestoreMetadataOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRecoveryPointRestoreMetadataError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the AWS resource types supported by AWS Backup.</p>
    #[allow(unused_mut)]
    async fn get_supported_resource_types(
        &self,
    ) -> Result<GetSupportedResourceTypesOutput, RusotoError<GetSupportedResourceTypesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/supported-resource-types";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSupportedResourceTypesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSupportedResourceTypesError::from_response(response))
        }
    }

    /// <p>Returns a list of existing backup jobs for an authenticated account.</p>
    #[allow(unused_mut)]
    async fn list_backup_jobs(
        &self,
        input: ListBackupJobsInput,
    ) -> Result<ListBackupJobsOutput, RusotoError<ListBackupJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/backup-jobs/";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.by_account_id {
            params.put("accountId", x);
        }
        if let Some(ref x) = input.by_backup_vault_name {
            params.put("backupVaultName", x);
        }
        if let Some(ref x) = input.by_created_after {
            params.put("createdAfter", x);
        }
        if let Some(ref x) = input.by_created_before {
            params.put("createdBefore", x);
        }
        if let Some(ref x) = input.by_resource_arn {
            params.put("resourceArn", x);
        }
        if let Some(ref x) = input.by_resource_type {
            params.put("resourceType", x);
        }
        if let Some(ref x) = input.by_state {
            params.put("state", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBackupJobsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBackupJobsError::from_response(response))
        }
    }

    /// <p>Returns metadata of your saved backup plan templates, including the template ID, name, and the creation and deletion dates.</p>
    #[allow(unused_mut)]
    async fn list_backup_plan_templates(
        &self,
        input: ListBackupPlanTemplatesInput,
    ) -> Result<ListBackupPlanTemplatesOutput, RusotoError<ListBackupPlanTemplatesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/backup/template/plans";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBackupPlanTemplatesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBackupPlanTemplatesError::from_response(response))
        }
    }

    /// <p>Returns version metadata of your backup plans, including Amazon Resource Names (ARNs), backup plan IDs, creation and deletion dates, plan names, and version IDs.</p>
    #[allow(unused_mut)]
    async fn list_backup_plan_versions(
        &self,
        input: ListBackupPlanVersionsInput,
    ) -> Result<ListBackupPlanVersionsOutput, RusotoError<ListBackupPlanVersionsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}/versions/",
            backup_plan_id = input.backup_plan_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBackupPlanVersionsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBackupPlanVersionsError::from_response(response))
        }
    }

    /// <p>Returns a list of existing backup plans for an authenticated account. The list is populated only if the advanced option is set for the backup plan. The list contains information such as Amazon Resource Names (ARNs), plan IDs, creation and deletion dates, version IDs, plan names, and creator request IDs.</p>
    #[allow(unused_mut)]
    async fn list_backup_plans(
        &self,
        input: ListBackupPlansInput,
    ) -> Result<ListBackupPlansOutput, RusotoError<ListBackupPlansError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/backup/plans/";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_deleted {
            params.put("includeDeleted", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBackupPlansOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBackupPlansError::from_response(response))
        }
    }

    /// <p>Returns an array containing metadata of the resources associated with the target backup plan.</p>
    #[allow(unused_mut)]
    async fn list_backup_selections(
        &self,
        input: ListBackupSelectionsInput,
    ) -> Result<ListBackupSelectionsOutput, RusotoError<ListBackupSelectionsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}/selections/",
            backup_plan_id = input.backup_plan_id
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBackupSelectionsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBackupSelectionsError::from_response(response))
        }
    }

    /// <p>Returns a list of recovery point storage containers along with information about them.</p>
    #[allow(unused_mut)]
    async fn list_backup_vaults(
        &self,
        input: ListBackupVaultsInput,
    ) -> Result<ListBackupVaultsOutput, RusotoError<ListBackupVaultsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/backup-vaults/";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBackupVaultsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBackupVaultsError::from_response(response))
        }
    }

    /// <p>Returns metadata about your copy jobs.</p>
    #[allow(unused_mut)]
    async fn list_copy_jobs(
        &self,
        input: ListCopyJobsInput,
    ) -> Result<ListCopyJobsOutput, RusotoError<ListCopyJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/copy-jobs/";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.by_account_id {
            params.put("accountId", x);
        }
        if let Some(ref x) = input.by_created_after {
            params.put("createdAfter", x);
        }
        if let Some(ref x) = input.by_created_before {
            params.put("createdBefore", x);
        }
        if let Some(ref x) = input.by_destination_vault_arn {
            params.put("destinationVaultArn", x);
        }
        if let Some(ref x) = input.by_resource_arn {
            params.put("resourceArn", x);
        }
        if let Some(ref x) = input.by_resource_type {
            params.put("resourceType", x);
        }
        if let Some(ref x) = input.by_state {
            params.put("state", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListCopyJobsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListCopyJobsError::from_response(response))
        }
    }

    /// <p>Returns an array of resources successfully backed up by AWS Backup, including the time the resource was saved, an Amazon Resource Name (ARN) of the resource, and a resource type.</p>
    #[allow(unused_mut)]
    async fn list_protected_resources(
        &self,
        input: ListProtectedResourcesInput,
    ) -> Result<ListProtectedResourcesOutput, RusotoError<ListProtectedResourcesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/resources/";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProtectedResourcesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProtectedResourcesError::from_response(response))
        }
    }

    /// <p>Returns detailed information about the recovery points stored in a backup vault.</p>
    #[allow(unused_mut)]
    async fn list_recovery_points_by_backup_vault(
        &self,
        input: ListRecoveryPointsByBackupVaultInput,
    ) -> Result<
        ListRecoveryPointsByBackupVaultOutput,
        RusotoError<ListRecoveryPointsByBackupVaultError>,
    > {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/recovery-points/",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.by_backup_plan_id {
            params.put("backupPlanId", x);
        }
        if let Some(ref x) = input.by_created_after {
            params.put("createdAfter", x);
        }
        if let Some(ref x) = input.by_created_before {
            params.put("createdBefore", x);
        }
        if let Some(ref x) = input.by_resource_arn {
            params.put("resourceArn", x);
        }
        if let Some(ref x) = input.by_resource_type {
            params.put("resourceType", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRecoveryPointsByBackupVaultOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRecoveryPointsByBackupVaultError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns detailed information about recovery points of the type specified by a resource Amazon Resource Name (ARN).</p>
    #[allow(unused_mut)]
    async fn list_recovery_points_by_resource(
        &self,
        input: ListRecoveryPointsByResourceInput,
    ) -> Result<ListRecoveryPointsByResourceOutput, RusotoError<ListRecoveryPointsByResourceError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/resources/{resource_arn}/recovery-points/",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRecoveryPointsByResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRecoveryPointsByResourceError::from_response(response))
        }
    }

    /// <p>Returns a list of jobs that AWS Backup initiated to restore a saved resource, including metadata about the recovery process.</p>
    #[allow(unused_mut)]
    async fn list_restore_jobs(
        &self,
        input: ListRestoreJobsInput,
    ) -> Result<ListRestoreJobsOutput, RusotoError<ListRestoreJobsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/restore-jobs/";

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.by_account_id {
            params.put("accountId", x);
        }
        if let Some(ref x) = input.by_created_after {
            params.put("createdAfter", x);
        }
        if let Some(ref x) = input.by_created_before {
            params.put("createdBefore", x);
        }
        if let Some(ref x) = input.by_status {
            params.put("status", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRestoreJobsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRestoreJobsError::from_response(response))
        }
    }

    /// <p><p>Returns a list of key-value pairs assigned to a target recovery point, backup plan, or backup vault.</p> <note> <p> <code>ListTags</code> are currently only supported with Amazon EFS backups.</p> </note></p>
    #[allow(unused_mut)]
    async fn list_tags(
        &self,
        input: ListTagsInput,
    ) -> Result<ListTagsOutput, RusotoError<ListTagsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}/", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListTagsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsError::from_response(response))
        }
    }

    /// <p>Sets a resource-based policy that is used to manage access permissions on the target backup vault. Requires a backup vault name and an access policy document in JSON format.</p>
    #[allow(unused_mut)]
    async fn put_backup_vault_access_policy(
        &self,
        input: PutBackupVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<PutBackupVaultAccessPolicyError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/access-policy",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutBackupVaultAccessPolicyError::from_response(response))
        }
    }

    /// <p>Turns on notifications on a backup vault for the specified topic and events.</p>
    #[allow(unused_mut)]
    async fn put_backup_vault_notifications(
        &self,
        input: PutBackupVaultNotificationsInput,
    ) -> Result<(), RusotoError<PutBackupVaultNotificationsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/notification-configuration",
            backup_vault_name = input.backup_vault_name
        );

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutBackupVaultNotificationsError::from_response(response))
        }
    }

    /// <p>Starts an on-demand backup job for the specified resource.</p>
    #[allow(unused_mut)]
    async fn start_backup_job(
        &self,
        input: StartBackupJobInput,
    ) -> Result<StartBackupJobOutput, RusotoError<StartBackupJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/backup-jobs";

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartBackupJobOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartBackupJobError::from_response(response))
        }
    }

    /// <p>Starts a job to create a one-time copy of the specified resource.</p>
    #[allow(unused_mut)]
    async fn start_copy_job(
        &self,
        input: StartCopyJobInput,
    ) -> Result<StartCopyJobOutput, RusotoError<StartCopyJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/copy-jobs";

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartCopyJobOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartCopyJobError::from_response(response))
        }
    }

    /// <p>Recovers the saved resource identified by an Amazon Resource Name (ARN). </p>
    #[allow(unused_mut)]
    async fn start_restore_job(
        &self,
        input: StartRestoreJobInput,
    ) -> Result<StartRestoreJobOutput, RusotoError<StartRestoreJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/restore-jobs";

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartRestoreJobOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartRestoreJobError::from_response(response))
        }
    }

    /// <p>Attempts to cancel a job to create a one-time backup of a resource.</p>
    #[allow(unused_mut)]
    async fn stop_backup_job(
        &self,
        input: StopBackupJobInput,
    ) -> Result<(), RusotoError<StopBackupJobError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-jobs/{backup_job_id}",
            backup_job_id = input.backup_job_id
        );

        let mut request = SignedRequest::new("POST", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopBackupJobError::from_response(response))
        }
    }

    /// <p>Assigns a set of key-value pairs to a recovery point, backup plan, or backup vault identified by an Amazon Resource Name (ARN).</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<(), RusotoError<TagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes a set of key-value pairs from a recovery point, backup plan, or backup vault identified by an Amazon Resource Name (ARN)</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/untag/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an existing backup plan identified by its <code>backupPlanId</code> with the input document in JSON format. The new version is uniquely identified by a <code>VersionId</code>.</p>
    #[allow(unused_mut)]
    async fn update_backup_plan(
        &self,
        input: UpdateBackupPlanInput,
    ) -> Result<UpdateBackupPlanOutput, RusotoError<UpdateBackupPlanError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup/plans/{backup_plan_id}",
            backup_plan_id = input.backup_plan_id
        );

        let mut request = SignedRequest::new("POST", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateBackupPlanOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBackupPlanError::from_response(response))
        }
    }

    /// <p>Updates the current global settings for the AWS Account. Use the <code>DescribeGlobalSettings</code> API to determine the current settings.</p>
    #[allow(unused_mut)]
    async fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsInput,
    ) -> Result<(), RusotoError<UpdateGlobalSettingsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/global-settings";

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGlobalSettingsError::from_response(response))
        }
    }

    /// <p>Sets the transition lifecycle of a recovery point.</p> <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. AWS Backup transitions and expires backups automatically according to the lifecycle that you define. </p> <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “expire after days” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
    #[allow(unused_mut)]
    async fn update_recovery_point_lifecycle(
        &self,
        input: UpdateRecoveryPointLifecycleInput,
    ) -> Result<UpdateRecoveryPointLifecycleOutput, RusotoError<UpdateRecoveryPointLifecycleError>>
    {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/backup-vaults/{backup_vault_name}/recovery-points/{recovery_point_arn}",
            backup_vault_name = input.backup_vault_name,
            recovery_point_arn = input.recovery_point_arn
        );

        let mut request = SignedRequest::new("POST", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRecoveryPointLifecycleOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRecoveryPointLifecycleError::from_response(response))
        }
    }

    /// <p>Updates the current service opt-in settings for the Region. If service-opt-in is enabled for a service, AWS Backup tries to protect that service's resources in this Region, when the resource is included in an on-demand backup or scheduled backup plan. Otherwise, AWS Backup does not try to protect that service's resources in this Region. Use the <code>DescribeRegionSettings</code> API to determine the resource types that are supported.</p>
    #[allow(unused_mut)]
    async fn update_region_settings(
        &self,
        input: UpdateRegionSettingsInput,
    ) -> Result<(), RusotoError<UpdateRegionSettingsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/account-settings";

        let mut request = SignedRequest::new("PUT", "backup", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let _result = ::std::mem::drop(response);

            Ok(())
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRegionSettingsError::from_response(response))
        }
    }
}
