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
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddRoleToDBClusterMessage {
    /// <p>The name of the DB cluster to associate the IAM role with.</p>
    pub db_cluster_identifier: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to associate with the Neptune DB cluster, for example <code>arn:aws:iam::123456789012:role/NeptuneAccessRole</code>.</p>
    pub role_arn: String,
}

/// Serialize `AddRoleToDBClusterMessage` contents to a `SignedRequest`.
struct AddRoleToDBClusterMessageSerializer;
impl AddRoleToDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddRoleToDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        params.put(&format!("{}{}", prefix, "RoleArn"), &obj.role_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddSourceIdentifierToSubscriptionMessage {
    /// <p><p>The identifier of the event source to be added.</p> <p>Constraints:</p> <ul> <li> <p>If the source type is a DB instance, then a <code>DBInstanceIdentifier</code> must be supplied.</p> </li> <li> <p>If the source type is a DB security group, a <code>DBSecurityGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB parameter group, a <code>DBParameterGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB snapshot, a <code>DBSnapshotIdentifier</code> must be supplied.</p> </li> </ul></p>
    pub source_identifier: String,
    /// <p>The name of the event notification subscription you want to add a source identifier to.</p>
    pub subscription_name: String,
}

/// Serialize `AddSourceIdentifierToSubscriptionMessage` contents to a `SignedRequest`.
struct AddSourceIdentifierToSubscriptionMessageSerializer;
impl AddSourceIdentifierToSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddSourceIdentifierToSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceIdentifier"),
            &obj.source_identifier,
        );
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AddSourceIdentifierToSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct AddSourceIdentifierToSubscriptionResultDeserializer;
impl AddSourceIdentifierToSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddSourceIdentifierToSubscriptionResult, XmlParseError> {
        deserialize_elements::<_, AddSourceIdentifierToSubscriptionResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EventSubscription" => {
                        obj.event_subscription = Some(EventSubscriptionDeserializer::deserialize(
                            "EventSubscription",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToResourceMessage {
    /// <p>The Amazon Neptune resource that the tags are added to. This value is an Amazon Resource Name (ARN). For information about creating an ARN, see <a href="https://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_name: String,
    /// <p>The tags to be assigned to the Amazon Neptune resource.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `AddTagsToResourceMessage` contents to a `SignedRequest`.
struct AddTagsToResourceMessageSerializer;
impl AddTagsToResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddTagsToResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceName"), &obj.resource_name);
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), &obj.tags);
    }
}

struct ApplyMethodDeserializer;
impl ApplyMethodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplyPendingMaintenanceActionMessage {
    /// <p>The pending maintenance action to apply to this resource.</p> <p>Valid values: <code>system-update</code>, <code>db-upgrade</code> </p>
    pub apply_action: String,
    /// <p><p>A value that specifies the type of opt-in request, or undoes an opt-in request. An opt-in request of type <code>immediate</code> can&#39;t be undone.</p> <p>Valid values:</p> <ul> <li> <p> <code>immediate</code> - Apply the maintenance action immediately.</p> </li> <li> <p> <code>next-maintenance</code> - Apply the maintenance action during the next maintenance window for the resource.</p> </li> <li> <p> <code>undo-opt-in</code> - Cancel any existing <code>next-maintenance</code> opt-in requests.</p> </li> </ul></p>
    pub opt_in_type: String,
    /// <p>The Amazon Resource Name (ARN) of the resource that the pending maintenance action applies to. For information about creating an ARN, see <a href="https://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_identifier: String,
}

/// Serialize `ApplyPendingMaintenanceActionMessage` contents to a `SignedRequest`.
struct ApplyPendingMaintenanceActionMessageSerializer;
impl ApplyPendingMaintenanceActionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ApplyPendingMaintenanceActionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ApplyAction"), &obj.apply_action);
        params.put(&format!("{}{}", prefix, "OptInType"), &obj.opt_in_type);
        params.put(
            &format!("{}{}", prefix, "ResourceIdentifier"),
            &obj.resource_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ApplyPendingMaintenanceActionResult {
    pub resource_pending_maintenance_actions: Option<ResourcePendingMaintenanceActions>,
}

struct ApplyPendingMaintenanceActionResultDeserializer;
impl ApplyPendingMaintenanceActionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplyPendingMaintenanceActionResult, XmlParseError> {
        deserialize_elements::<_, ApplyPendingMaintenanceActionResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ResourcePendingMaintenanceActions" => {
                        obj.resource_pending_maintenance_actions =
                            Some(ResourcePendingMaintenanceActionsDeserializer::deserialize(
                                "ResourcePendingMaintenanceActions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct AttributeValueListDeserializer;
impl AttributeValueListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AttributeValue" {
                obj.push(StringDeserializer::deserialize("AttributeValue", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `AttributeValueList` contents to a `SignedRequest`.
struct AttributeValueListSerializer;
impl AttributeValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Specifies an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>The name of the availability zone.</p>
    pub name: Option<String>,
}

struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AvailabilityZone, XmlParseError> {
        deserialize_elements::<_, AvailabilityZone, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = Some(StringDeserializer::deserialize("Name", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AvailabilityZoneListDeserializer;
impl AvailabilityZoneListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AvailabilityZone>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AvailabilityZone" {
                obj.push(AvailabilityZoneDeserializer::deserialize(
                    "AvailabilityZone",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct AvailabilityZonesDeserializer;
impl AvailabilityZonesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AvailabilityZone" {
                obj.push(StringDeserializer::deserialize("AvailabilityZone", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `AvailabilityZones` contents to a `SignedRequest`.
struct AvailabilityZonesSerializer;
impl AvailabilityZonesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BooleanOptionalDeserializer;
impl BooleanOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Specifies a character set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CharacterSet {
    /// <p>The description of the character set.</p>
    pub character_set_description: Option<String>,
    /// <p>The name of the character set.</p>
    pub character_set_name: Option<String>,
}

struct CharacterSetDeserializer;
impl CharacterSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CharacterSet, XmlParseError> {
        deserialize_elements::<_, CharacterSet, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CharacterSetDescription" => {
                    obj.character_set_description = Some(StringDeserializer::deserialize(
                        "CharacterSetDescription",
                        stack,
                    )?);
                }
                "CharacterSetName" => {
                    obj.character_set_name =
                        Some(StringDeserializer::deserialize("CharacterSetName", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The configuration setting for the log types to be enabled for export to CloudWatch Logs for a specific DB instance or DB cluster.</p> <p>The <code>EnableLogTypes</code> and <code>DisableLogTypes</code> arrays determine which logs will be exported (or not exported) to CloudWatch Logs.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloudwatchLogsExportConfiguration {
    /// <p>The list of log types to disable.</p>
    pub disable_log_types: Option<Vec<String>>,
    /// <p>The list of log types to enable.</p>
    pub enable_log_types: Option<Vec<String>>,
}

/// Serialize `CloudwatchLogsExportConfiguration` contents to a `SignedRequest`.
struct CloudwatchLogsExportConfigurationSerializer;
impl CloudwatchLogsExportConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CloudwatchLogsExportConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.disable_log_types {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DisableLogTypes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_log_types {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnableLogTypes"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyDBClusterParameterGroupMessage {
    /// <p><p>The identifier or Amazon Resource Name (ARN) for the source DB cluster parameter group. For information about creating an ARN, see <a href="https://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid DB cluster parameter group.</p> </li> <li> <p>If the source DB cluster parameter group is in the same AWS Region as the copy, specify a valid DB parameter group identifier, for example <code>my-db-cluster-param-group</code>, or a valid ARN.</p> </li> <li> <p>If the source DB parameter group is in a different AWS Region than the copy, specify a valid DB cluster parameter group ARN, for example <code>arn:aws:rds:us-east-1:123456789012:cluster-pg:custom-cluster-group1</code>.</p> </li> </ul></p>
    pub source_db_cluster_parameter_group_identifier: String,
    /// <p>The tags to be assigned to the copied DB cluster parameter group.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A description for the copied DB cluster parameter group.</p>
    pub target_db_cluster_parameter_group_description: String,
    /// <p>The identifier for the copied DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank</p> </li> <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-cluster-param-group1</code> </p>
    pub target_db_cluster_parameter_group_identifier: String,
}

/// Serialize `CopyDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct CopyDBClusterParameterGroupMessageSerializer;
impl CopyDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopyDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceDBClusterParameterGroupIdentifier"),
            &obj.source_db_cluster_parameter_group_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetDBClusterParameterGroupDescription"),
            &obj.target_db_cluster_parameter_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "TargetDBClusterParameterGroupIdentifier"),
            &obj.target_db_cluster_parameter_group_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CopyDBClusterParameterGroupResult {
    pub db_cluster_parameter_group: Option<DBClusterParameterGroup>,
}

struct CopyDBClusterParameterGroupResultDeserializer;
impl CopyDBClusterParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyDBClusterParameterGroupResult, XmlParseError> {
        deserialize_elements::<_, CopyDBClusterParameterGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterParameterGroup" => {
                        obj.db_cluster_parameter_group =
                            Some(DBClusterParameterGroupDeserializer::deserialize(
                                "DBClusterParameterGroup",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyDBClusterSnapshotMessage {
    /// <p>True to copy all tags from the source DB cluster snapshot to the target DB cluster snapshot, and otherwise false. The default is false.</p>
    pub copy_tags: Option<bool>,
    /// <p>The AWS AWS KMS key ID for an encrypted DB cluster snapshot. The KMS key ID is the Amazon Resource Name (ARN), KMS key identifier, or the KMS key alias for the KMS encryption key.</p> <p>If you copy an encrypted DB cluster snapshot from your AWS account, you can specify a value for <code>KmsKeyId</code> to encrypt the copy with a new KMS encryption key. If you don't specify a value for <code>KmsKeyId</code>, then the copy of the DB cluster snapshot is encrypted with the same KMS key as the source DB cluster snapshot.</p> <p>If you copy an encrypted DB cluster snapshot that is shared from another AWS account, then you must specify a value for <code>KmsKeyId</code>.</p> <p> KMS encryption keys are specific to the AWS Region that they are created in, and you can't use encryption keys from one AWS Region in another AWS Region.</p> <p>You cannot encrypt an unencrypted DB cluster snapshot when you copy it. If you try to copy an unencrypted DB cluster snapshot and specify a value for the KmsKeyId parameter, an error is returned.</p>
    pub kms_key_id: Option<String>,
    /// <p>Not currently supported.</p>
    pub pre_signed_url: Option<String>,
    /// <p>The identifier of the DB cluster snapshot to copy. This parameter is not case-sensitive.</p> <p>You can't copy from one AWS Region to another.</p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid system snapshot in the "available" state.</p> </li> <li> <p>Specify a valid DB snapshot identifier.</p> </li> </ul> <p>Example: <code>my-cluster-snapshot1</code> </p>
    pub source_db_cluster_snapshot_identifier: String,
    /// <p>The tags to assign to the new DB cluster snapshot copy.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The identifier of the new DB cluster snapshot to create from the source DB cluster snapshot. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster-snapshot2</code> </p>
    pub target_db_cluster_snapshot_identifier: String,
}

/// Serialize `CopyDBClusterSnapshotMessage` contents to a `SignedRequest`.
struct CopyDBClusterSnapshotMessageSerializer;
impl CopyDBClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopyDBClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.copy_tags {
            params.put(&format!("{}{}", prefix, "CopyTags"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.pre_signed_url {
            params.put(&format!("{}{}", prefix, "PreSignedUrl"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SourceDBClusterSnapshotIdentifier"),
            &obj.source_db_cluster_snapshot_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetDBClusterSnapshotIdentifier"),
            &obj.target_db_cluster_snapshot_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CopyDBClusterSnapshotResult {
    pub db_cluster_snapshot: Option<DBClusterSnapshot>,
}

struct CopyDBClusterSnapshotResultDeserializer;
impl CopyDBClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyDBClusterSnapshotResult, XmlParseError> {
        deserialize_elements::<_, CopyDBClusterSnapshotResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterSnapshot" => {
                        obj.db_cluster_snapshot = Some(DBClusterSnapshotDeserializer::deserialize(
                            "DBClusterSnapshot",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyDBParameterGroupMessage {
    /// <p><p>The identifier or ARN for the source DB parameter group. For information about creating an ARN, see <a href="https://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid DB parameter group.</p> </li> <li> <p>Must specify a valid DB parameter group identifier, for example <code>my-db-param-group</code>, or a valid ARN.</p> </li> </ul></p>
    pub source_db_parameter_group_identifier: String,
    /// <p>The tags to be assigned to the copied DB parameter group.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A description for the copied DB parameter group.</p>
    pub target_db_parameter_group_description: String,
    /// <p>The identifier for the copied DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank.</p> </li> <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-db-parameter-group</code> </p>
    pub target_db_parameter_group_identifier: String,
}

/// Serialize `CopyDBParameterGroupMessage` contents to a `SignedRequest`.
struct CopyDBParameterGroupMessageSerializer;
impl CopyDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopyDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceDBParameterGroupIdentifier"),
            &obj.source_db_parameter_group_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetDBParameterGroupDescription"),
            &obj.target_db_parameter_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "TargetDBParameterGroupIdentifier"),
            &obj.target_db_parameter_group_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CopyDBParameterGroupResult {
    pub db_parameter_group: Option<DBParameterGroup>,
}

struct CopyDBParameterGroupResultDeserializer;
impl CopyDBParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyDBParameterGroupResult, XmlParseError> {
        deserialize_elements::<_, CopyDBParameterGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBParameterGroup" => {
                        obj.db_parameter_group = Some(DBParameterGroupDeserializer::deserialize(
                            "DBParameterGroup",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBClusterMessage {
    /// <p>A list of EC2 Availability Zones that instances in the DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p><p>The number of days for which automated backups are retained. You must specify a minimum value of 1.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 1 to 35</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub character_set_name: Option<String>,
    /// <p>The DB cluster identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster1</code> </p>
    pub db_cluster_identifier: String,
    /// <p><p> The name of the DB cluster parameter group to associate with this DB cluster. If this argument is omitted, the default is used.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>A DB subnet group to associate with this DB cluster.</p> <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>The name for your database of up to 64 alpha-numeric characters. If you do not provide a name, Amazon Neptune will not create a database in the DB cluster you are creating.</p>
    pub database_name: Option<String>,
    /// <p>A value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. </p>
    pub deletion_protection: Option<bool>,
    /// <p>The list of log types that need to be enabled for exporting to CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The name of the database engine to be used for this DB cluster.</p> <p>Valid Values: <code>neptune</code> </p>
    pub engine: String,
    /// <p>The version number of the database engine to use. Currently, setting this parameter has no effect.</p> <p>Example: <code>1.0.1</code> </p>
    pub engine_version: Option<String>,
    /// <p>The AWS KMS key identifier for an encrypted DB cluster.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a DB cluster with the same AWS account that owns the KMS encryption key used to encrypt the new DB cluster, then you can use the KMS key alias instead of the ARN for the KMS encryption key.</p> <p>If an encryption key is not specified in <code>KmsKeyId</code>:</p> <ul> <li> <p>If <code>ReplicationSourceIdentifier</code> identifies an encrypted source, then Amazon Neptune will use the encryption key used to encrypt the source. Otherwise, Amazon Neptune will use your default encryption key.</p> </li> <li> <p>If the <code>StorageEncrypted</code> parameter is true and <code>ReplicationSourceIdentifier</code> is not specified, then Amazon Neptune will use your default encryption key.</p> </li> </ul> <p>AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p> <p>If you create a Read Replica of an encrypted DB cluster in another AWS Region, you must set <code>KmsKeyId</code> to a KMS key ID that is valid in the destination AWS Region. This key is used to encrypt the Read Replica in that AWS Region.</p>
    pub kms_key_id: Option<String>,
    /// <p>The password for the master database user. This password can contain any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain from 8 to 41 characters.</p>
    pub master_user_password: Option<String>,
    /// <p><p>The name of the master user for the DB cluster.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 16 letters or numbers.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot be a reserved word for the chosen database engine.</p> </li> </ul></p>
    pub master_username: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the instances in the DB cluster accept connections.</p> <p> Default: <code>8182</code> </p>
    pub port: Option<i64>,
    /// <p>This parameter is not currently supported.</p>
    pub pre_signed_url: Option<String>,
    /// <p><p>The daily time range during which automated backups are created if automated backups are enabled using the <code>BackupRetentionPeriod</code> parameter.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. To see the time blocks available, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/AdjustingTheMaintenanceWindow.html"> Adjusting the Preferred Maintenance Window</a> in the <i>Amazon Neptune User Guide.</i> </p> <p>Constraints:</p> <ul> <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li> <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week. To see the time blocks available, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/AdjustingTheMaintenanceWindow.html"> Adjusting the Preferred Maintenance Window</a> in the <i>Amazon Neptune User Guide.</i> </p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the source DB instance or DB cluster if this DB cluster is created as a Read Replica.</p>
    pub replication_source_identifier: Option<String>,
    /// <p>Specifies whether the DB cluster is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>The tags to assign to the new DB cluster.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of EC2 VPC security groups to associate with this DB cluster.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `CreateDBClusterMessage` contents to a `SignedRequest`.
struct CreateDBClusterMessageSerializer;
impl CreateDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZone"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.character_set_name {
            params.put(&format!("{}{}", prefix, "CharacterSetName"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.database_name {
            params.put(&format!("{}{}", prefix, "DatabaseName"), &field_value);
        }
        if let Some(ref field_value) = obj.deletion_protection {
            params.put(&format!("{}{}", prefix, "DeletionProtection"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_cloudwatch_logs_exports {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnableCloudwatchLogsExports"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.master_username {
            params.put(&format!("{}{}", prefix, "MasterUsername"), &field_value);
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.pre_signed_url {
            params.put(&format!("{}{}", prefix, "PreSignedUrl"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.replication_source_identifier {
            params.put(
                &format!("{}{}", prefix, "ReplicationSourceIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.storage_encrypted {
            params.put(&format!("{}{}", prefix, "StorageEncrypted"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBClusterParameterGroupMessage {
    /// <p><p>The name of the DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing DBClusterParameterGroup.</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
    pub db_cluster_parameter_group_name: String,
    /// <p>The DB cluster parameter group family name. A DB cluster parameter group can be associated with one and only one DB cluster parameter group family, and can be applied only to a DB cluster running a database engine and engine version compatible with that DB cluster parameter group family.</p>
    pub db_parameter_group_family: String,
    /// <p>The description for the DB cluster parameter group.</p>
    pub description: String,
    /// <p>The tags to be assigned to the new DB cluster parameter group.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct CreateDBClusterParameterGroupMessageSerializer;
impl CreateDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDBClusterParameterGroupResult {
    pub db_cluster_parameter_group: Option<DBClusterParameterGroup>,
}

struct CreateDBClusterParameterGroupResultDeserializer;
impl CreateDBClusterParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBClusterParameterGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateDBClusterParameterGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterParameterGroup" => {
                        obj.db_cluster_parameter_group =
                            Some(DBClusterParameterGroupDeserializer::deserialize(
                                "DBClusterParameterGroup",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct CreateDBClusterResultDeserializer;
impl CreateDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBClusterResult, XmlParseError> {
        deserialize_elements::<_, CreateDBClusterResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBCluster" => {
                    obj.db_cluster = Some(DBClusterDeserializer::deserialize("DBCluster", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBClusterSnapshotMessage {
    /// <p>The identifier of the DB cluster to create a snapshot for. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul> <p>Example: <code>my-cluster1</code> </p>
    pub db_cluster_identifier: String,
    /// <p>The identifier of the DB cluster snapshot. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster1-snapshot1</code> </p>
    pub db_cluster_snapshot_identifier: String,
    /// <p>The tags to be assigned to the DB cluster snapshot.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBClusterSnapshotMessage` contents to a `SignedRequest`.
struct CreateDBClusterSnapshotMessageSerializer;
impl CreateDBClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDBClusterSnapshotResult {
    pub db_cluster_snapshot: Option<DBClusterSnapshot>,
}

struct CreateDBClusterSnapshotResultDeserializer;
impl CreateDBClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBClusterSnapshotResult, XmlParseError> {
        deserialize_elements::<_, CreateDBClusterSnapshotResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterSnapshot" => {
                        obj.db_cluster_snapshot = Some(DBClusterSnapshotDeserializer::deserialize(
                            "DBClusterSnapshot",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBInstanceMessage {
    /// <p>The amount of storage (in gibibytes) to allocate for the DB instance.</p> <p>Type: Integer</p> <p>Not applicable. Neptune cluster volumes automatically grow as the amount of data in your database increases, though you are only charged for the space that you use in a Neptune cluster volume.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that minor engine upgrades are applied automatically to the DB instance during the maintenance window.</p> <p>Default: <code>true</code> </p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p> The EC2 Availability Zone that the DB instance is created in</p> <p>Default: A random, system-chosen Availability Zone in the endpoint's AWS Region.</p> <p> Example: <code>us-east-1d</code> </p> <p> Constraint: The AvailabilityZone parameter can't be specified if the MultiAZ parameter is set to <code>true</code>. The specified Availability Zone must be in the same AWS Region as the current endpoint.</p>
    pub availability_zone: Option<String>,
    /// <p><p>The number of days for which automated backups are retained.</p> <p>Not applicable. The retention period for automated backups is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 0 to 35</p> </li> <li> <p>Cannot be set to 0 if the DB instance is a source to Read Replicas</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub character_set_name: Option<String>,
    /// <p>True to copy all tags from the DB instance to snapshots of the DB instance, and otherwise false. The default is false.</p>
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>The identifier of the DB cluster that the instance will belong to.</p> <p>For information on creating a DB cluster, see <a>CreateDBCluster</a>.</p> <p>Type: String</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>. Not all DB instance classes are available in all AWS Regions.</p>
    pub db_instance_class: String,
    /// <p>The DB instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>mydbinstance</code> </p>
    pub db_instance_identifier: String,
    /// <p>Not supported.</p>
    pub db_name: Option<String>,
    /// <p><p>The name of the DB parameter group to associate with this DB instance. If this argument is omitted, the default DBParameterGroup for the specified engine is used.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub db_parameter_group_name: Option<String>,
    /// <p>A list of DB security groups to associate with this DB instance.</p> <p>Default: The default DB security group for the database engine.</p>
    pub db_security_groups: Option<Vec<String>>,
    /// <p>A DB subnet group to associate with this DB instance.</p> <p>If there is no DB subnet group, then it is a non-VPC DB instance.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>A value that indicates whether the DB instance has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. </p> <p>You can enable or disable deletion protection for the DB cluster. For more information, see <a>CreateDBCluster</a>. DB instances in a DB cluster can be deleted even when deletion protection is enabled for the DB cluster. </p>
    pub deletion_protection: Option<bool>,
    /// <p>Specify the Active Directory Domain to create the instance in.</p>
    pub domain: Option<String>,
    /// <p>Specify the name of the IAM role to be used when making API calls to the Directory Service.</p>
    pub domain_iam_role_name: Option<String>,
    /// <p>The list of log types that need to be enabled for exporting to CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>True to enable AWS Identity and Access Management (IAM) authentication for Neptune.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub enable_performance_insights: Option<bool>,
    /// <p>The name of the database engine to be used for this instance.</p> <p>Valid Values: <code>neptune</code> </p>
    pub engine: String,
    /// <p>The version number of the database engine to use. Currently, setting this parameter has no effect.</p>
    pub engine_version: Option<String>,
    /// <p>The amount of Provisioned IOPS (input/output operations per second) to be initially allocated for the DB instance.</p>
    pub iops: Option<i64>,
    /// <p>The AWS KMS key identifier for an encrypted DB instance.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a DB instance with the same AWS account that owns the KMS encryption key used to encrypt the new DB instance, then you can use the KMS key alias instead of the ARN for the KM encryption key.</p> <p>Not applicable. The KMS key identifier is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>If the <code>StorageEncrypted</code> parameter is true, and you do not specify a value for the <code>KmsKeyId</code> parameter, then Amazon Neptune will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    pub kms_key_id: Option<String>,
    /// <p>License model information for this DB instance.</p> <p> Valid values: <code>license-included</code> | <code>bring-your-own-license</code> | <code>general-public-license</code> </p>
    pub license_model: Option<String>,
    /// <p>The password for the master user. The password can include any printable ASCII character except "/", """, or "@".</p> <p> Not used.</p>
    pub master_user_password: Option<String>,
    /// <p>The name for the master user. Not used.</p>
    pub master_username: Option<String>,
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.</p> <p>If <code>MonitoringRoleArn</code> is specified, then you must also set <code>MonitoringInterval</code> to a value other than 0.</p> <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code> </p>
    pub monitoring_interval: Option<i64>,
    /// <p>The ARN for the IAM role that permits Neptune to send enhanced monitoring metrics to Amazon CloudWatch Logs. For example, <code>arn:aws:iam:123456789012:role/emaccess</code>.</p> <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a <code>MonitoringRoleArn</code> value.</p>
    pub monitoring_role_arn: Option<String>,
    /// <p>Specifies if the DB instance is a Multi-AZ deployment. You can't set the AvailabilityZone parameter if the MultiAZ parameter is set to true.</p>
    pub multi_az: Option<bool>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub option_group_name: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub performance_insights_kms_key_id: Option<String>,
    /// <p>The port number on which the database accepts connections.</p> <p>Not applicable. The port is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p> Default: <code>8182</code> </p> <p>Type: Integer</p>
    pub port: Option<i64>,
    /// <p> The daily time range during which automated backups are created.</p> <p>Not applicable. The daily time range for creating automated backups is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p>
    pub preferred_backup_window: Option<String>,
    /// <p>The time range each week during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which an Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p> <p>Default: 1</p> <p>Valid Values: 0 - 15</p>
    pub promotion_tier: Option<i64>,
    /// <p>Specifies whether the DB instance is encrypted.</p> <p>Not applicable. The encryption for DB instances is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>Default: false</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Specifies the storage type to be associated with the DB instance.</p> <p>Not applicable. Storage is managed by the DB Cluster.</p>
    pub storage_type: Option<String>,
    /// <p>The tags to assign to the new instance.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub tde_credential_arn: Option<String>,
    /// <p>The password for the given ARN from the key store in order to access the device.</p>
    pub tde_credential_password: Option<String>,
    /// <p>The time zone of the DB instance.</p>
    pub timezone: Option<String>,
    /// <p>A list of EC2 VPC security groups to associate with this DB instance.</p> <p>Not applicable. The associated list of EC2 VPC security groups is managed by the DB cluster. For more information, see <a>CreateDBCluster</a>.</p> <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `CreateDBInstanceMessage` contents to a `SignedRequest`.
struct CreateDBInstanceMessageSerializer;
impl CreateDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allocated_storage {
            params.put(&format!("{}{}", prefix, "AllocatedStorage"), &field_value);
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.availability_zone {
            params.put(&format!("{}{}", prefix, "AvailabilityZone"), &field_value);
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.character_set_name {
            params.put(&format!("{}{}", prefix, "CharacterSetName"), &field_value);
        }
        if let Some(ref field_value) = obj.copy_tags_to_snapshot {
            params.put(&format!("{}{}", prefix, "CopyTagsToSnapshot"), &field_value);
        }
        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBInstanceClass"),
            &obj.db_instance_class,
        );
        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.db_name {
            params.put(&format!("{}{}", prefix, "DBName"), &field_value);
        }
        if let Some(ref field_value) = obj.db_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_security_groups {
            DBSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DBSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.deletion_protection {
            params.put(&format!("{}{}", prefix, "DeletionProtection"), &field_value);
        }
        if let Some(ref field_value) = obj.domain {
            params.put(&format!("{}{}", prefix, "Domain"), &field_value);
        }
        if let Some(ref field_value) = obj.domain_iam_role_name {
            params.put(&format!("{}{}", prefix, "DomainIAMRoleName"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_cloudwatch_logs_exports {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnableCloudwatchLogsExports"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_performance_insights {
            params.put(
                &format!("{}{}", prefix, "EnablePerformanceInsights"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.iops {
            params.put(&format!("{}{}", prefix, "Iops"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.license_model {
            params.put(&format!("{}{}", prefix, "LicenseModel"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.master_username {
            params.put(&format!("{}{}", prefix, "MasterUsername"), &field_value);
        }
        if let Some(ref field_value) = obj.monitoring_interval {
            params.put(&format!("{}{}", prefix, "MonitoringInterval"), &field_value);
        }
        if let Some(ref field_value) = obj.monitoring_role_arn {
            params.put(&format!("{}{}", prefix, "MonitoringRoleArn"), &field_value);
        }
        if let Some(ref field_value) = obj.multi_az {
            params.put(&format!("{}{}", prefix, "MultiAZ"), &field_value);
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.performance_insights_kms_key_id {
            params.put(
                &format!("{}{}", prefix, "PerformanceInsightsKMSKeyId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.promotion_tier {
            params.put(&format!("{}{}", prefix, "PromotionTier"), &field_value);
        }

        if let Some(ref field_value) = obj.storage_encrypted {
            params.put(&format!("{}{}", prefix, "StorageEncrypted"), &field_value);
        }
        if let Some(ref field_value) = obj.storage_type {
            params.put(&format!("{}{}", prefix, "StorageType"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_arn {
            params.put(&format!("{}{}", prefix, "TdeCredentialArn"), &field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_password {
            params.put(
                &format!("{}{}", prefix, "TdeCredentialPassword"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.timezone {
            params.put(&format!("{}{}", prefix, "Timezone"), &field_value);
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct CreateDBInstanceResultDeserializer;
impl CreateDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBInstanceResult, XmlParseError> {
        deserialize_elements::<_, CreateDBInstanceResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBInstance" => {
                    obj.db_instance =
                        Some(DBInstanceDeserializer::deserialize("DBInstance", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBParameterGroupMessage {
    /// <p>The DB parameter group family name. A DB parameter group can be associated with one and only one DB parameter group family, and can be applied only to a DB instance running a database engine and engine version compatible with that DB parameter group family.</p>
    pub db_parameter_group_family: String,
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
    pub db_parameter_group_name: String,
    /// <p>The description for the DB parameter group.</p>
    pub description: String,
    /// <p>The tags to be assigned to the new DB parameter group.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBParameterGroupMessage` contents to a `SignedRequest`.
struct CreateDBParameterGroupMessageSerializer;
impl CreateDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDBParameterGroupResult {
    pub db_parameter_group: Option<DBParameterGroup>,
}

struct CreateDBParameterGroupResultDeserializer;
impl CreateDBParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBParameterGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateDBParameterGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBParameterGroup" => {
                        obj.db_parameter_group = Some(DBParameterGroupDeserializer::deserialize(
                            "DBParameterGroup",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBSubnetGroupMessage {
    /// <p>The description for the DB subnet group.</p>
    pub db_subnet_group_description: String,
    /// <p>The name for the DB subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 letters, numbers, periods, underscores, spaces, or hyphens. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
    /// <p>The EC2 Subnet IDs for the DB subnet group.</p>
    pub subnet_ids: Vec<String>,
    /// <p>The tags to be assigned to the new DB subnet group.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBSubnetGroupMessage` contents to a `SignedRequest`.
struct CreateDBSubnetGroupMessageSerializer;
impl CreateDBSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupDescription"),
            &obj.db_subnet_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupName"),
            &obj.db_subnet_group_name,
        );
        SubnetIdentifierListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SubnetIdentifier"),
            &obj.subnet_ids,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDBSubnetGroupResult {
    pub db_subnet_group: Option<DBSubnetGroup>,
}

struct CreateDBSubnetGroupResultDeserializer;
impl CreateDBSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDBSubnetGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateDBSubnetGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBSubnetGroup" => {
                        obj.db_subnet_group = Some(DBSubnetGroupDeserializer::deserialize(
                            "DBSubnetGroup",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEventSubscriptionMessage {
    /// <p> A Boolean value; set to <b>true</b> to activate the subscription, set to <b>false</b> to create the subscription but not active it.</p>
    pub enabled: Option<bool>,
    /// <p> A list of event categories for a SourceType that you want to subscribe to. You can see a list of the categories for a given SourceType by using the <b>DescribeEventCategories</b> action.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>
    pub sns_topic_arn: String,
    /// <p><p>The list of identifiers of the event sources for which events are returned. If not specified, then all sources are included in the response. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can&#39;t end with a hyphen or contain two consecutive hyphens.</p> <p>Constraints:</p> <ul> <li> <p>If SourceIds are supplied, SourceType must also be provided.</p> </li> <li> <p>If the source type is a DB instance, then a <code>DBInstanceIdentifier</code> must be supplied.</p> </li> <li> <p>If the source type is a DB security group, a <code>DBSecurityGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB parameter group, a <code>DBParameterGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is a DB snapshot, a <code>DBSnapshotIdentifier</code> must be supplied.</p> </li> </ul></p>
    pub source_ids: Option<Vec<String>>,
    /// <p>The type of source that is generating the events. For example, if you want to be notified of events generated by a DB instance, you would set this parameter to db-instance. if this value is not specified, all events are returned.</p> <p>Valid values: <code>db-instance</code> | <code>db-cluster</code> | <code>db-parameter-group</code> | <code>db-security-group</code> | <code>db-snapshot</code> | <code>db-cluster-snapshot</code> </p>
    pub source_type: Option<String>,
    /// <p>The name of the subscription.</p> <p>Constraints: The name must be less than 255 characters.</p>
    pub subscription_name: String,
    /// <p>The tags to be applied to the new event subscription.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateEventSubscriptionMessage` contents to a `SignedRequest`.
struct CreateEventSubscriptionMessageSerializer;
impl CreateEventSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateEventSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.enabled {
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "SnsTopicArn"), &obj.sns_topic_arn);
        if let Some(ref field_value) = obj.source_ids {
            SourceIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct CreateEventSubscriptionResultDeserializer;
impl CreateEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateEventSubscriptionResult, XmlParseError> {
        deserialize_elements::<_, CreateEventSubscriptionResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EventSubscription" => {
                        obj.event_subscription = Some(EventSubscriptionDeserializer::deserialize(
                            "EventSubscription",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the details of an Amazon Neptune DB cluster.</p> <p>This data type is used as a response element in the <a>DescribeDBClusters</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBCluster {
    /// <p> <code>AllocatedStorage</code> always returns 1, because Neptune DB cluster storage size is not fixed, but instead automatically adjusts as needed.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Provides a list of the AWS Identity and Access Management (IAM) roles that are associated with the DB cluster. IAM roles that are associated with a DB cluster grant permission for the DB cluster to access other AWS services on your behalf.</p>
    pub associated_roles: Option<Vec<DBClusterRole>>,
    /// <p>Provides the list of EC2 Availability Zones that instances in the DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Specifies the number of days for which automatic DB snapshots are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub character_set_name: Option<String>,
    /// <p>Identifies the clone group to which the DB cluster is associated.</p>
    pub clone_group_id: Option<String>,
    /// <p>Specifies the time when the DB cluster was created, in Universal Coordinated Time (UTC).</p>
    pub cluster_create_time: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB cluster.</p>
    pub db_cluster_arn: Option<String>,
    /// <p>Contains a user-supplied DB cluster identifier. This identifier is the unique key that identifies a DB cluster.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>Provides the list of instances that make up the DB cluster.</p>
    pub db_cluster_members: Option<Vec<DBClusterMember>>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub db_cluster_option_group_memberships: Option<Vec<DBClusterOptionGroupStatus>>,
    /// <p>Specifies the name of the DB cluster parameter group for the DB cluster.</p>
    pub db_cluster_parameter_group: Option<String>,
    /// <p>Specifies information on the subnet group associated with the DB cluster, including the name, description, and subnets in the subnet group.</p>
    pub db_subnet_group: Option<String>,
    /// <p>Contains the name of the initial database of this DB cluster that was provided at create time, if one was specified when the DB cluster was created. This same name is returned for the life of the DB cluster.</p>
    pub database_name: Option<String>,
    /// <p>The AWS Region-unique, immutable identifier for the DB cluster. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB cluster is accessed.</p>
    pub db_cluster_resource_id: Option<String>,
    /// <p>Indicates if the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. </p>
    pub deletion_protection: Option<bool>,
    /// <p>Specifies the earliest time to which a database can be restored with point-in-time restore.</p>
    pub earliest_restorable_time: Option<String>,
    /// <p>A list of log types that this DB cluster is configured to export to CloudWatch Logs.</p>
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>Specifies the connection endpoint for the primary instance of the DB cluster.</p>
    pub endpoint: Option<String>,
    /// <p>Provides the name of the database engine to be used for this DB cluster.</p>
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>True if mapping of AWS Identity and Access Management (IAM) accounts to database accounts is enabled, and otherwise false.</p>
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>If <code>StorageEncrypted</code> is true, the AWS KMS key identifier for the encrypted DB cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p>Specifies the latest time to which a database can be restored with point-in-time restore.</p>
    pub latest_restorable_time: Option<String>,
    /// <p>Contains the master username for the DB cluster.</p>
    pub master_username: Option<String>,
    /// <p>Specifies whether the DB cluster has instances in multiple Availability Zones.</p>
    pub multi_az: Option<bool>,
    /// <p>Specifies the progress of the operation as a percentage.</p>
    pub percent_progress: Option<String>,
    /// <p>Specifies the port that the database engine is listening on.</p>
    pub port: Option<i64>,
    /// <p>Specifies the daily time range during which automated backups are created if automated backups are enabled, as determined by the <code>BackupRetentionPeriod</code>.</p>
    pub preferred_backup_window: Option<String>,
    /// <p>Specifies the weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>Contains one or more identifiers of the Read Replicas associated with this DB cluster.</p>
    pub read_replica_identifiers: Option<Vec<String>>,
    /// <p>The reader endpoint for the DB cluster. The reader endpoint for a DB cluster load-balances connections across the Read Replicas that are available in a DB cluster. As clients request new connections to the reader endpoint, Neptune distributes the connection requests among the Read Replicas in the DB cluster. This functionality can help balance your read workload across multiple Read Replicas in your DB cluster.</p> <p>If a failover occurs, and the Read Replica that you are connected to is promoted to be the primary instance, your connection is dropped. To continue sending your read workload to other Read Replicas in the cluster, you can then reconnect to the reader endpoint.</p>
    pub reader_endpoint: Option<String>,
    /// <p>Not supported by Neptune.</p>
    pub replication_source_identifier: Option<String>,
    /// <p>Specifies the current state of this DB cluster.</p>
    pub status: Option<String>,
    /// <p>Specifies whether the DB cluster is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Provides a list of VPC security groups that the DB cluster belongs to.</p>
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

struct DBClusterDeserializer;
impl DBClusterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBCluster, XmlParseError> {
        deserialize_elements::<_, DBCluster, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllocatedStorage" => {
                    obj.allocated_storage = Some(IntegerOptionalDeserializer::deserialize(
                        "AllocatedStorage",
                        stack,
                    )?);
                }
                "AssociatedRoles" => {
                    obj.associated_roles.get_or_insert(vec![]).extend(
                        DBClusterRolesDeserializer::deserialize("AssociatedRoles", stack)?,
                    );
                }
                "AvailabilityZones" => {
                    obj.availability_zones.get_or_insert(vec![]).extend(
                        AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)?,
                    );
                }
                "BackupRetentionPeriod" => {
                    obj.backup_retention_period = Some(IntegerOptionalDeserializer::deserialize(
                        "BackupRetentionPeriod",
                        stack,
                    )?);
                }
                "CharacterSetName" => {
                    obj.character_set_name =
                        Some(StringDeserializer::deserialize("CharacterSetName", stack)?);
                }
                "CloneGroupId" => {
                    obj.clone_group_id =
                        Some(StringDeserializer::deserialize("CloneGroupId", stack)?);
                }
                "ClusterCreateTime" => {
                    obj.cluster_create_time =
                        Some(TStampDeserializer::deserialize("ClusterCreateTime", stack)?);
                }
                "DBClusterArn" => {
                    obj.db_cluster_arn =
                        Some(StringDeserializer::deserialize("DBClusterArn", stack)?);
                }
                "DBClusterIdentifier" => {
                    obj.db_cluster_identifier = Some(StringDeserializer::deserialize(
                        "DBClusterIdentifier",
                        stack,
                    )?);
                }
                "DBClusterMembers" => {
                    obj.db_cluster_members.get_or_insert(vec![]).extend(
                        DBClusterMemberListDeserializer::deserialize("DBClusterMembers", stack)?,
                    );
                }
                "DBClusterOptionGroupMemberships" => {
                    obj.db_cluster_option_group_memberships
                        .get_or_insert(vec![])
                        .extend(DBClusterOptionGroupMembershipsDeserializer::deserialize(
                            "DBClusterOptionGroupMemberships",
                            stack,
                        )?);
                }
                "DBClusterParameterGroup" => {
                    obj.db_cluster_parameter_group = Some(StringDeserializer::deserialize(
                        "DBClusterParameterGroup",
                        stack,
                    )?);
                }
                "DBSubnetGroup" => {
                    obj.db_subnet_group =
                        Some(StringDeserializer::deserialize("DBSubnetGroup", stack)?);
                }
                "DatabaseName" => {
                    obj.database_name =
                        Some(StringDeserializer::deserialize("DatabaseName", stack)?);
                }
                "DbClusterResourceId" => {
                    obj.db_cluster_resource_id = Some(StringDeserializer::deserialize(
                        "DbClusterResourceId",
                        stack,
                    )?);
                }
                "DeletionProtection" => {
                    obj.deletion_protection = Some(BooleanOptionalDeserializer::deserialize(
                        "DeletionProtection",
                        stack,
                    )?);
                }
                "EarliestRestorableTime" => {
                    obj.earliest_restorable_time = Some(TStampDeserializer::deserialize(
                        "EarliestRestorableTime",
                        stack,
                    )?);
                }
                "EnabledCloudwatchLogsExports" => {
                    obj.enabled_cloudwatch_logs_exports
                        .get_or_insert(vec![])
                        .extend(LogTypeListDeserializer::deserialize(
                            "EnabledCloudwatchLogsExports",
                            stack,
                        )?);
                }
                "Endpoint" => {
                    obj.endpoint = Some(StringDeserializer::deserialize("Endpoint", stack)?);
                }
                "Engine" => {
                    obj.engine = Some(StringDeserializer::deserialize("Engine", stack)?);
                }
                "EngineVersion" => {
                    obj.engine_version =
                        Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                }
                "HostedZoneId" => {
                    obj.hosted_zone_id =
                        Some(StringDeserializer::deserialize("HostedZoneId", stack)?);
                }
                "IAMDatabaseAuthenticationEnabled" => {
                    obj.iam_database_authentication_enabled =
                        Some(BooleanDeserializer::deserialize(
                            "IAMDatabaseAuthenticationEnabled",
                            stack,
                        )?);
                }
                "KmsKeyId" => {
                    obj.kms_key_id = Some(StringDeserializer::deserialize("KmsKeyId", stack)?);
                }
                "LatestRestorableTime" => {
                    obj.latest_restorable_time = Some(TStampDeserializer::deserialize(
                        "LatestRestorableTime",
                        stack,
                    )?);
                }
                "MasterUsername" => {
                    obj.master_username =
                        Some(StringDeserializer::deserialize("MasterUsername", stack)?);
                }
                "MultiAZ" => {
                    obj.multi_az = Some(BooleanDeserializer::deserialize("MultiAZ", stack)?);
                }
                "PercentProgress" => {
                    obj.percent_progress =
                        Some(StringDeserializer::deserialize("PercentProgress", stack)?);
                }
                "Port" => {
                    obj.port = Some(IntegerOptionalDeserializer::deserialize("Port", stack)?);
                }
                "PreferredBackupWindow" => {
                    obj.preferred_backup_window = Some(StringDeserializer::deserialize(
                        "PreferredBackupWindow",
                        stack,
                    )?);
                }
                "PreferredMaintenanceWindow" => {
                    obj.preferred_maintenance_window = Some(StringDeserializer::deserialize(
                        "PreferredMaintenanceWindow",
                        stack,
                    )?);
                }
                "ReadReplicaIdentifiers" => {
                    obj.read_replica_identifiers.get_or_insert(vec![]).extend(
                        ReadReplicaIdentifierListDeserializer::deserialize(
                            "ReadReplicaIdentifiers",
                            stack,
                        )?,
                    );
                }
                "ReaderEndpoint" => {
                    obj.reader_endpoint =
                        Some(StringDeserializer::deserialize("ReaderEndpoint", stack)?);
                }
                "ReplicationSourceIdentifier" => {
                    obj.replication_source_identifier = Some(StringDeserializer::deserialize(
                        "ReplicationSourceIdentifier",
                        stack,
                    )?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "StorageEncrypted" => {
                    obj.storage_encrypted =
                        Some(BooleanDeserializer::deserialize("StorageEncrypted", stack)?);
                }
                "VpcSecurityGroups" => {
                    obj.vpc_security_groups.get_or_insert(vec![]).extend(
                        VpcSecurityGroupMembershipListDeserializer::deserialize(
                            "VpcSecurityGroups",
                            stack,
                        )?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBClusterListDeserializer;
impl DBClusterListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBCluster>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBCluster" {
                obj.push(DBClusterDeserializer::deserialize("DBCluster", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains information about an instance that is part of a DB cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterMember {
    /// <p>Specifies the status of the DB cluster parameter group for this member of the DB cluster.</p>
    pub db_cluster_parameter_group_status: Option<String>,
    /// <p>Specifies the instance identifier for this member of the DB cluster.</p>
    pub db_instance_identifier: Option<String>,
    /// <p>Value that is <code>true</code> if the cluster member is the primary instance for the DB cluster and <code>false</code> otherwise.</p>
    pub is_cluster_writer: Option<bool>,
    /// <p>A value that specifies the order in which a Read Replica is promoted to the primary instance after a failure of the existing primary instance.</p>
    pub promotion_tier: Option<i64>,
}

struct DBClusterMemberDeserializer;
impl DBClusterMemberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterMember, XmlParseError> {
        deserialize_elements::<_, DBClusterMember, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBClusterParameterGroupStatus" => {
                    obj.db_cluster_parameter_group_status = Some(StringDeserializer::deserialize(
                        "DBClusterParameterGroupStatus",
                        stack,
                    )?);
                }
                "DBInstanceIdentifier" => {
                    obj.db_instance_identifier = Some(StringDeserializer::deserialize(
                        "DBInstanceIdentifier",
                        stack,
                    )?);
                }
                "IsClusterWriter" => {
                    obj.is_cluster_writer =
                        Some(BooleanDeserializer::deserialize("IsClusterWriter", stack)?);
                }
                "PromotionTier" => {
                    obj.promotion_tier = Some(IntegerOptionalDeserializer::deserialize(
                        "PromotionTier",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBClusterMemberListDeserializer;
impl DBClusterMemberListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterMember>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBClusterMember" {
                obj.push(DBClusterMemberDeserializer::deserialize(
                    "DBClusterMember",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterMessage {
    /// <p>Contains a list of DB clusters for the user.</p>
    pub db_clusters: Option<Vec<DBCluster>>,
    /// <p>A pagination token that can be used in a subsequent DescribeDBClusters request.</p>
    pub marker: Option<String>,
}

struct DBClusterMessageDeserializer;
impl DBClusterMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterMessage, XmlParseError> {
        deserialize_elements::<_, DBClusterMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBClusters" => {
                    obj.db_clusters
                        .get_or_insert(vec![])
                        .extend(DBClusterListDeserializer::deserialize("DBClusters", stack)?);
                }
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBClusterOptionGroupMembershipsDeserializer;
impl DBClusterOptionGroupMembershipsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterOptionGroupStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBClusterOptionGroup" {
                obj.push(DBClusterOptionGroupStatusDeserializer::deserialize(
                    "DBClusterOptionGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains status information for a DB cluster option group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterOptionGroupStatus {
    /// <p>Specifies the name of the DB cluster option group.</p>
    pub db_cluster_option_group_name: Option<String>,
    /// <p>Specifies the status of the DB cluster option group.</p>
    pub status: Option<String>,
}

struct DBClusterOptionGroupStatusDeserializer;
impl DBClusterOptionGroupStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterOptionGroupStatus, XmlParseError> {
        deserialize_elements::<_, DBClusterOptionGroupStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterOptionGroupName" => {
                        obj.db_cluster_option_group_name = Some(StringDeserializer::deserialize(
                            "DBClusterOptionGroupName",
                            stack,
                        )?);
                    }
                    "Status" => {
                        obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the details of an Amazon Neptune DB cluster parameter group.</p> <p>This data type is used as a response element in the <a>DescribeDBClusterParameterGroups</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterParameterGroup {
    /// <p>The Amazon Resource Name (ARN) for the DB cluster parameter group.</p>
    pub db_cluster_parameter_group_arn: Option<String>,
    /// <p>Provides the name of the DB cluster parameter group.</p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>Provides the name of the DB parameter group family that this DB cluster parameter group is compatible with.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p>Provides the customer-specified description for this DB cluster parameter group.</p>
    pub description: Option<String>,
}

struct DBClusterParameterGroupDeserializer;
impl DBClusterParameterGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroup, XmlParseError> {
        deserialize_elements::<_, DBClusterParameterGroup, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterParameterGroupArn" => {
                        obj.db_cluster_parameter_group_arn = Some(StringDeserializer::deserialize(
                            "DBClusterParameterGroupArn",
                            stack,
                        )?);
                    }
                    "DBClusterParameterGroupName" => {
                        obj.db_cluster_parameter_group_name = Some(
                            StringDeserializer::deserialize("DBClusterParameterGroupName", stack)?,
                        );
                    }
                    "DBParameterGroupFamily" => {
                        obj.db_parameter_group_family = Some(StringDeserializer::deserialize(
                            "DBParameterGroupFamily",
                            stack,
                        )?);
                    }
                    "Description" => {
                        obj.description =
                            Some(StringDeserializer::deserialize("Description", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterParameterGroupDetails {
    /// <p> An optional pagination token provided by a previous DescribeDBClusterParameters request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
    pub marker: Option<String>,
    /// <p>Provides a list of parameters for the DB cluster parameter group.</p>
    pub parameters: Option<Vec<Parameter>>,
}

struct DBClusterParameterGroupDetailsDeserializer;
impl DBClusterParameterGroupDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroupDetails, XmlParseError> {
        deserialize_elements::<_, DBClusterParameterGroupDetails, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "Parameters" => {
                        obj.parameters.get_or_insert(vec![]).extend(
                            ParametersListDeserializer::deserialize("Parameters", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DBClusterParameterGroupListDeserializer;
impl DBClusterParameterGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterParameterGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBClusterParameterGroup" {
                obj.push(DBClusterParameterGroupDeserializer::deserialize(
                    "DBClusterParameterGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterParameterGroupNameMessage {
    /// <p><p>The name of the DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters or numbers.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
    pub db_cluster_parameter_group_name: Option<String>,
}

struct DBClusterParameterGroupNameMessageDeserializer;
impl DBClusterParameterGroupNameMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroupNameMessage, XmlParseError> {
        deserialize_elements::<_, DBClusterParameterGroupNameMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterParameterGroupName" => {
                        obj.db_cluster_parameter_group_name = Some(
                            StringDeserializer::deserialize("DBClusterParameterGroupName", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterParameterGroupsMessage {
    /// <p>A list of DB cluster parameter groups.</p>
    pub db_cluster_parameter_groups: Option<Vec<DBClusterParameterGroup>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterParameterGroups</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
}

struct DBClusterParameterGroupsMessageDeserializer;
impl DBClusterParameterGroupsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterParameterGroupsMessage, XmlParseError> {
        deserialize_elements::<_, DBClusterParameterGroupsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterParameterGroups" => {
                        obj.db_cluster_parameter_groups
                            .get_or_insert(vec![])
                            .extend(DBClusterParameterGroupListDeserializer::deserialize(
                                "DBClusterParameterGroups",
                                stack,
                            )?);
                    }
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Describes an AWS Identity and Access Management (IAM) role that is associated with a DB cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterRole {
    /// <p>The Amazon Resource Name (ARN) of the IAM role that is associated with the DB cluster.</p>
    pub role_arn: Option<String>,
    /// <p><p>Describes the state of association between the IAM role and the DB cluster. The Status property returns one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - the IAM role ARN is associated with the DB cluster and can be used to access other AWS services on your behalf.</p> </li> <li> <p> <code>PENDING</code> - the IAM role ARN is being associated with the DB cluster.</p> </li> <li> <p> <code>INVALID</code> - the IAM role ARN is associated with the DB cluster, but the DB cluster is unable to assume the IAM role in order to access other AWS services on your behalf.</p> </li> </ul></p>
    pub status: Option<String>,
}

struct DBClusterRoleDeserializer;
impl DBClusterRoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterRole, XmlParseError> {
        deserialize_elements::<_, DBClusterRole, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "RoleArn" => {
                    obj.role_arn = Some(StringDeserializer::deserialize("RoleArn", stack)?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBClusterRolesDeserializer;
impl DBClusterRolesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterRole>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBClusterRole" {
                obj.push(DBClusterRoleDeserializer::deserialize(
                    "DBClusterRole",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains the details for an Amazon Neptune DB cluster snapshot</p> <p>This data type is used as a response element in the <a>DescribeDBClusterSnapshots</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshot {
    /// <p>Specifies the allocated storage size in gibibytes (GiB).</p>
    pub allocated_storage: Option<i64>,
    /// <p>Provides the list of EC2 Availability Zones that instances in the DB cluster snapshot can be restored in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Specifies the time when the DB cluster was created, in Universal Coordinated Time (UTC).</p>
    pub cluster_create_time: Option<String>,
    /// <p>Specifies the DB cluster identifier of the DB cluster that this DB cluster snapshot was created from.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB cluster snapshot.</p>
    pub db_cluster_snapshot_arn: Option<String>,
    /// <p>Specifies the identifier for the DB cluster snapshot.</p>
    pub db_cluster_snapshot_identifier: Option<String>,
    /// <p>Specifies the name of the database engine.</p>
    pub engine: Option<String>,
    /// <p>Provides the version of the database engine for this DB cluster snapshot.</p>
    pub engine_version: Option<String>,
    /// <p>True if mapping of AWS Identity and Access Management (IAM) accounts to database accounts is enabled, and otherwise false.</p>
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>If <code>StorageEncrypted</code> is true, the AWS KMS key identifier for the encrypted DB cluster snapshot.</p>
    pub kms_key_id: Option<String>,
    /// <p>Provides the license model information for this DB cluster snapshot.</p>
    pub license_model: Option<String>,
    /// <p>Provides the master username for the DB cluster snapshot.</p>
    pub master_username: Option<String>,
    /// <p>Specifies the percentage of the estimated data that has been transferred.</p>
    pub percent_progress: Option<i64>,
    /// <p>Specifies the port that the DB cluster was listening on at the time of the snapshot.</p>
    pub port: Option<i64>,
    /// <p>Provides the time when the snapshot was taken, in Universal Coordinated Time (UTC).</p>
    pub snapshot_create_time: Option<String>,
    /// <p>Provides the type of the DB cluster snapshot.</p>
    pub snapshot_type: Option<String>,
    /// <p>If the DB cluster snapshot was copied from a source DB cluster snapshot, the Amazon Resource Name (ARN) for the source DB cluster snapshot, otherwise, a null value.</p>
    pub source_db_cluster_snapshot_arn: Option<String>,
    /// <p>Specifies the status of this DB cluster snapshot.</p>
    pub status: Option<String>,
    /// <p>Specifies whether the DB cluster snapshot is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Provides the VPC ID associated with the DB cluster snapshot.</p>
    pub vpc_id: Option<String>,
}

struct DBClusterSnapshotDeserializer;
impl DBClusterSnapshotDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshot, XmlParseError> {
        deserialize_elements::<_, DBClusterSnapshot, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllocatedStorage" => {
                    obj.allocated_storage =
                        Some(IntegerDeserializer::deserialize("AllocatedStorage", stack)?);
                }
                "AvailabilityZones" => {
                    obj.availability_zones.get_or_insert(vec![]).extend(
                        AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)?,
                    );
                }
                "ClusterCreateTime" => {
                    obj.cluster_create_time =
                        Some(TStampDeserializer::deserialize("ClusterCreateTime", stack)?);
                }
                "DBClusterIdentifier" => {
                    obj.db_cluster_identifier = Some(StringDeserializer::deserialize(
                        "DBClusterIdentifier",
                        stack,
                    )?);
                }
                "DBClusterSnapshotArn" => {
                    obj.db_cluster_snapshot_arn = Some(StringDeserializer::deserialize(
                        "DBClusterSnapshotArn",
                        stack,
                    )?);
                }
                "DBClusterSnapshotIdentifier" => {
                    obj.db_cluster_snapshot_identifier = Some(StringDeserializer::deserialize(
                        "DBClusterSnapshotIdentifier",
                        stack,
                    )?);
                }
                "Engine" => {
                    obj.engine = Some(StringDeserializer::deserialize("Engine", stack)?);
                }
                "EngineVersion" => {
                    obj.engine_version =
                        Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                }
                "IAMDatabaseAuthenticationEnabled" => {
                    obj.iam_database_authentication_enabled =
                        Some(BooleanDeserializer::deserialize(
                            "IAMDatabaseAuthenticationEnabled",
                            stack,
                        )?);
                }
                "KmsKeyId" => {
                    obj.kms_key_id = Some(StringDeserializer::deserialize("KmsKeyId", stack)?);
                }
                "LicenseModel" => {
                    obj.license_model =
                        Some(StringDeserializer::deserialize("LicenseModel", stack)?);
                }
                "MasterUsername" => {
                    obj.master_username =
                        Some(StringDeserializer::deserialize("MasterUsername", stack)?);
                }
                "PercentProgress" => {
                    obj.percent_progress =
                        Some(IntegerDeserializer::deserialize("PercentProgress", stack)?);
                }
                "Port" => {
                    obj.port = Some(IntegerDeserializer::deserialize("Port", stack)?);
                }
                "SnapshotCreateTime" => {
                    obj.snapshot_create_time = Some(TStampDeserializer::deserialize(
                        "SnapshotCreateTime",
                        stack,
                    )?);
                }
                "SnapshotType" => {
                    obj.snapshot_type =
                        Some(StringDeserializer::deserialize("SnapshotType", stack)?);
                }
                "SourceDBClusterSnapshotArn" => {
                    obj.source_db_cluster_snapshot_arn = Some(StringDeserializer::deserialize(
                        "SourceDBClusterSnapshotArn",
                        stack,
                    )?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "StorageEncrypted" => {
                    obj.storage_encrypted =
                        Some(BooleanDeserializer::deserialize("StorageEncrypted", stack)?);
                }
                "VpcId" => {
                    obj.vpc_id = Some(StringDeserializer::deserialize("VpcId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains the name and values of a manual DB cluster snapshot attribute.</p> <p>Manual DB cluster snapshot attributes are used to authorize other AWS accounts to restore a manual DB cluster snapshot. For more information, see the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshotAttribute {
    /// <p>The name of the manual DB cluster snapshot attribute.</p> <p>The attribute named <code>restore</code> refers to the list of AWS accounts that have permission to copy or restore the manual DB cluster snapshot. For more information, see the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    pub attribute_name: Option<String>,
    /// <p>The value(s) for the manual DB cluster snapshot attribute.</p> <p>If the <code>AttributeName</code> field is set to <code>restore</code>, then this element returns a list of IDs of the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If a value of <code>all</code> is in the list, then the manual DB cluster snapshot is public and available for any AWS account to copy or restore.</p>
    pub attribute_values: Option<Vec<String>>,
}

struct DBClusterSnapshotAttributeDeserializer;
impl DBClusterSnapshotAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshotAttribute, XmlParseError> {
        deserialize_elements::<_, DBClusterSnapshotAttribute, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AttributeName" => {
                        obj.attribute_name =
                            Some(StringDeserializer::deserialize("AttributeName", stack)?);
                    }
                    "AttributeValues" => {
                        obj.attribute_values.get_or_insert(vec![]).extend(
                            AttributeValueListDeserializer::deserialize("AttributeValues", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DBClusterSnapshotAttributeListDeserializer;
impl DBClusterSnapshotAttributeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterSnapshotAttribute>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBClusterSnapshotAttribute" {
                obj.push(DBClusterSnapshotAttributeDeserializer::deserialize(
                    "DBClusterSnapshotAttribute",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains the results of a successful call to the <a>DescribeDBClusterSnapshotAttributes</a> API action.</p> <p>Manual DB cluster snapshot attributes are used to authorize other AWS accounts to copy or restore a manual DB cluster snapshot. For more information, see the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshotAttributesResult {
    /// <p>The list of attributes and values for the manual DB cluster snapshot.</p>
    pub db_cluster_snapshot_attributes: Option<Vec<DBClusterSnapshotAttribute>>,
    /// <p>The identifier of the manual DB cluster snapshot that the attributes apply to.</p>
    pub db_cluster_snapshot_identifier: Option<String>,
}

struct DBClusterSnapshotAttributesResultDeserializer;
impl DBClusterSnapshotAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshotAttributesResult, XmlParseError> {
        deserialize_elements::<_, DBClusterSnapshotAttributesResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterSnapshotAttributes" => {
                        obj.db_cluster_snapshot_attributes
                            .get_or_insert(vec![])
                            .extend(DBClusterSnapshotAttributeListDeserializer::deserialize(
                                "DBClusterSnapshotAttributes",
                                stack,
                            )?);
                    }
                    "DBClusterSnapshotIdentifier" => {
                        obj.db_cluster_snapshot_identifier = Some(StringDeserializer::deserialize(
                            "DBClusterSnapshotIdentifier",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DBClusterSnapshotListDeserializer;
impl DBClusterSnapshotListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBClusterSnapshot>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBClusterSnapshot" {
                obj.push(DBClusterSnapshotDeserializer::deserialize(
                    "DBClusterSnapshot",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshotMessage {
    /// <p>Provides a list of DB cluster snapshots for the user.</p>
    pub db_cluster_snapshots: Option<Vec<DBClusterSnapshot>>,
    /// <p> An optional pagination token provided by a previous <a>DescribeDBClusterSnapshots</a> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
}

struct DBClusterSnapshotMessageDeserializer;
impl DBClusterSnapshotMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBClusterSnapshotMessage, XmlParseError> {
        deserialize_elements::<_, DBClusterSnapshotMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterSnapshots" => {
                        obj.db_cluster_snapshots.get_or_insert(vec![]).extend(
                            DBClusterSnapshotListDeserializer::deserialize(
                                "DBClusterSnapshots",
                                stack,
                            )?,
                        );
                    }
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p> This data type is used as a response element in the action <a>DescribeDBEngineVersions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBEngineVersion {
    /// <p>The description of the database engine.</p>
    pub db_engine_description: Option<String>,
    /// <p>The description of the database engine version.</p>
    pub db_engine_version_description: Option<String>,
    /// <p>The name of the DB parameter group family for the database engine.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub default_character_set: Option<CharacterSet>,
    /// <p>The name of the database engine.</p>
    pub engine: Option<String>,
    /// <p>The version number of the database engine.</p>
    pub engine_version: Option<String>,
    /// <p>The types of logs that the database engine has available for export to CloudWatch Logs.</p>
    pub exportable_log_types: Option<Vec<String>>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub supported_character_sets: Option<Vec<CharacterSet>>,
    /// <p>A list of the time zones supported by this engine for the <code>Timezone</code> parameter of the <code>CreateDBInstance</code> action.</p>
    pub supported_timezones: Option<Vec<Timezone>>,
    /// <p>A value that indicates whether the engine version supports exporting the log types specified by ExportableLogTypes to CloudWatch Logs.</p>
    pub supports_log_exports_to_cloudwatch_logs: Option<bool>,
    /// <p>Indicates whether the database engine version supports read replicas.</p>
    pub supports_read_replica: Option<bool>,
    /// <p>A list of engine versions that this database engine version can be upgraded to.</p>
    pub valid_upgrade_target: Option<Vec<UpgradeTarget>>,
}

struct DBEngineVersionDeserializer;
impl DBEngineVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBEngineVersion, XmlParseError> {
        deserialize_elements::<_, DBEngineVersion, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBEngineDescription" => {
                    obj.db_engine_description = Some(StringDeserializer::deserialize(
                        "DBEngineDescription",
                        stack,
                    )?);
                }
                "DBEngineVersionDescription" => {
                    obj.db_engine_version_description = Some(StringDeserializer::deserialize(
                        "DBEngineVersionDescription",
                        stack,
                    )?);
                }
                "DBParameterGroupFamily" => {
                    obj.db_parameter_group_family = Some(StringDeserializer::deserialize(
                        "DBParameterGroupFamily",
                        stack,
                    )?);
                }
                "DefaultCharacterSet" => {
                    obj.default_character_set = Some(CharacterSetDeserializer::deserialize(
                        "DefaultCharacterSet",
                        stack,
                    )?);
                }
                "Engine" => {
                    obj.engine = Some(StringDeserializer::deserialize("Engine", stack)?);
                }
                "EngineVersion" => {
                    obj.engine_version =
                        Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                }
                "ExportableLogTypes" => {
                    obj.exportable_log_types.get_or_insert(vec![]).extend(
                        LogTypeListDeserializer::deserialize("ExportableLogTypes", stack)?,
                    );
                }
                "SupportedCharacterSets" => {
                    obj.supported_character_sets.get_or_insert(vec![]).extend(
                        SupportedCharacterSetsListDeserializer::deserialize(
                            "SupportedCharacterSets",
                            stack,
                        )?,
                    );
                }
                "SupportedTimezones" => {
                    obj.supported_timezones.get_or_insert(vec![]).extend(
                        SupportedTimezonesListDeserializer::deserialize(
                            "SupportedTimezones",
                            stack,
                        )?,
                    );
                }
                "SupportsLogExportsToCloudwatchLogs" => {
                    obj.supports_log_exports_to_cloudwatch_logs =
                        Some(BooleanDeserializer::deserialize(
                            "SupportsLogExportsToCloudwatchLogs",
                            stack,
                        )?);
                }
                "SupportsReadReplica" => {
                    obj.supports_read_replica = Some(BooleanDeserializer::deserialize(
                        "SupportsReadReplica",
                        stack,
                    )?);
                }
                "ValidUpgradeTarget" => {
                    obj.valid_upgrade_target.get_or_insert(vec![]).extend(
                        ValidUpgradeTargetListDeserializer::deserialize(
                            "ValidUpgradeTarget",
                            stack,
                        )?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBEngineVersionListDeserializer;
impl DBEngineVersionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBEngineVersion>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBEngineVersion" {
                obj.push(DBEngineVersionDeserializer::deserialize(
                    "DBEngineVersion",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBEngineVersionMessage {
    /// <p> A list of <code>DBEngineVersion</code> elements.</p>
    pub db_engine_versions: Option<Vec<DBEngineVersion>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
}

struct DBEngineVersionMessageDeserializer;
impl DBEngineVersionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBEngineVersionMessage, XmlParseError> {
        deserialize_elements::<_, DBEngineVersionMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBEngineVersions" => {
                    obj.db_engine_versions.get_or_insert(vec![]).extend(
                        DBEngineVersionListDeserializer::deserialize("DBEngineVersions", stack)?,
                    );
                }
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains the details of an Amazon Neptune DB instance.</p> <p>This data type is used as a response element in the <a>DescribeDBInstances</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBInstance {
    /// <p>Specifies the allocated storage size specified in gibibytes.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that minor version patches are applied automatically.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Specifies the name of the Availability Zone the DB instance is located in.</p>
    pub availability_zone: Option<String>,
    /// <p>Specifies the number of days for which automatic DB snapshots are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>The identifier of the CA certificate for this DB instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub character_set_name: Option<String>,
    /// <p>Specifies whether tags are copied from the DB instance to snapshots of the DB instance.</p>
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>If the DB instance is a member of a DB cluster, contains the name of the DB cluster that the DB instance is a member of.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB instance.</p>
    pub db_instance_arn: Option<String>,
    /// <p>Contains the name of the compute and memory capacity class of the DB instance.</p>
    pub db_instance_class: Option<String>,
    /// <p>Contains a user-supplied database identifier. This identifier is the unique key that identifies a DB instance.</p>
    pub db_instance_identifier: Option<String>,
    /// <p>Specifies the current state of this database.</p>
    pub db_instance_status: Option<String>,
    /// <p>The database name.</p>
    pub db_name: Option<String>,
    /// <p>Provides the list of DB parameter groups applied to this DB instance.</p>
    pub db_parameter_groups: Option<Vec<DBParameterGroupStatus>>,
    /// <p> Provides List of DB security group elements containing only <code>DBSecurityGroup.Name</code> and <code>DBSecurityGroup.Status</code> subelements.</p>
    pub db_security_groups: Option<Vec<DBSecurityGroupMembership>>,
    /// <p>Specifies information on the subnet group associated with the DB instance, including the name, description, and subnets in the subnet group.</p>
    pub db_subnet_group: Option<DBSubnetGroup>,
    /// <p>Specifies the port that the DB instance listens on. If the DB instance is part of a DB cluster, this can be a different port than the DB cluster port.</p>
    pub db_instance_port: Option<i64>,
    /// <p>The AWS Region-unique, immutable identifier for the DB instance. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB instance is accessed.</p>
    pub dbi_resource_id: Option<String>,
    /// <p>Indicates if the DB instance has deletion protection enabled. The database can't be deleted when deletion protection is enabled. </p>
    pub deletion_protection: Option<bool>,
    /// <p>Not supported</p>
    pub domain_memberships: Option<Vec<DomainMembership>>,
    /// <p>A list of log types that this DB instance is configured to export to CloudWatch Logs.</p>
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>Specifies the connection endpoint.</p>
    pub endpoint: Option<Endpoint>,
    /// <p>Provides the name of the database engine to be used for this DB instance.</p>
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch Logs log stream that receives the Enhanced Monitoring metrics data for the DB instance.</p>
    pub enhanced_monitoring_resource_arn: Option<String>,
    /// <p>True if AWS Identity and Access Management (IAM) authentication is enabled, and otherwise false.</p>
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>Provides the date and time the DB instance was created.</p>
    pub instance_create_time: Option<String>,
    /// <p>Specifies the Provisioned IOPS (I/O operations per second) value.</p>
    pub iops: Option<i64>,
    /// <p> Not supported: The encryption for DB instances is managed by the DB cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p>Specifies the latest time to which a database can be restored with point-in-time restore.</p>
    pub latest_restorable_time: Option<String>,
    /// <p>License model information for this DB instance.</p>
    pub license_model: Option<String>,
    /// <p>Contains the master username for the DB instance.</p>
    pub master_username: Option<String>,
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance.</p>
    pub monitoring_interval: Option<i64>,
    /// <p>The ARN for the IAM role that permits Neptune to send Enhanced Monitoring metrics to Amazon CloudWatch Logs.</p>
    pub monitoring_role_arn: Option<String>,
    /// <p>Specifies if the DB instance is a Multi-AZ deployment.</p>
    pub multi_az: Option<bool>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub option_group_memberships: Option<Vec<OptionGroupMembership>>,
    /// <p>Specifies that changes to the DB instance are pending. This element is only included when changes are pending. Specific changes are identified by subelements.</p>
    pub pending_modified_values: Option<PendingModifiedValues>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub performance_insights_enabled: Option<bool>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub performance_insights_kms_key_id: Option<String>,
    /// <p> Specifies the daily time range during which automated backups are created if automated backups are enabled, as determined by the <code>BackupRetentionPeriod</code>.</p>
    pub preferred_backup_window: Option<String>,
    /// <p>Specifies the weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which a Read Replica is promoted to the primary instance after a failure of the existing primary instance. </p>
    pub promotion_tier: Option<i64>,
    /// <p>Contains one or more identifiers of DB clusters that are Read Replicas of this DB instance.</p>
    pub read_replica_db_cluster_identifiers: Option<Vec<String>>,
    /// <p>Contains one or more identifiers of the Read Replicas associated with this DB instance.</p>
    pub read_replica_db_instance_identifiers: Option<Vec<String>>,
    /// <p>Contains the identifier of the source DB instance if this DB instance is a Read Replica.</p>
    pub read_replica_source_db_instance_identifier: Option<String>,
    /// <p>If present, specifies the name of the secondary Availability Zone for a DB instance with multi-AZ support.</p>
    pub secondary_availability_zone: Option<String>,
    /// <p>The status of a Read Replica. If the instance is not a Read Replica, this is blank.</p>
    pub status_infos: Option<Vec<DBInstanceStatusInfo>>,
    /// <p>Not supported: The encryption for DB instances is managed by the DB cluster.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Specifies the storage type associated with DB instance.</p>
    pub storage_type: Option<String>,
    /// <p>The ARN from the key store with which the instance is associated for TDE encryption.</p>
    pub tde_credential_arn: Option<String>,
    /// <p>Not supported.</p>
    pub timezone: Option<String>,
    /// <p>Provides a list of VPC security group elements that the DB instance belongs to.</p>
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

struct DBInstanceDeserializer;
impl DBInstanceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBInstance, XmlParseError> {
        deserialize_elements::<_, DBInstance, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllocatedStorage" => {
                    obj.allocated_storage =
                        Some(IntegerDeserializer::deserialize("AllocatedStorage", stack)?);
                }
                "AutoMinorVersionUpgrade" => {
                    obj.auto_minor_version_upgrade = Some(BooleanDeserializer::deserialize(
                        "AutoMinorVersionUpgrade",
                        stack,
                    )?);
                }
                "AvailabilityZone" => {
                    obj.availability_zone =
                        Some(StringDeserializer::deserialize("AvailabilityZone", stack)?);
                }
                "BackupRetentionPeriod" => {
                    obj.backup_retention_period = Some(IntegerDeserializer::deserialize(
                        "BackupRetentionPeriod",
                        stack,
                    )?);
                }
                "CACertificateIdentifier" => {
                    obj.ca_certificate_identifier = Some(StringDeserializer::deserialize(
                        "CACertificateIdentifier",
                        stack,
                    )?);
                }
                "CharacterSetName" => {
                    obj.character_set_name =
                        Some(StringDeserializer::deserialize("CharacterSetName", stack)?);
                }
                "CopyTagsToSnapshot" => {
                    obj.copy_tags_to_snapshot = Some(BooleanDeserializer::deserialize(
                        "CopyTagsToSnapshot",
                        stack,
                    )?);
                }
                "DBClusterIdentifier" => {
                    obj.db_cluster_identifier = Some(StringDeserializer::deserialize(
                        "DBClusterIdentifier",
                        stack,
                    )?);
                }
                "DBInstanceArn" => {
                    obj.db_instance_arn =
                        Some(StringDeserializer::deserialize("DBInstanceArn", stack)?);
                }
                "DBInstanceClass" => {
                    obj.db_instance_class =
                        Some(StringDeserializer::deserialize("DBInstanceClass", stack)?);
                }
                "DBInstanceIdentifier" => {
                    obj.db_instance_identifier = Some(StringDeserializer::deserialize(
                        "DBInstanceIdentifier",
                        stack,
                    )?);
                }
                "DBInstanceStatus" => {
                    obj.db_instance_status =
                        Some(StringDeserializer::deserialize("DBInstanceStatus", stack)?);
                }
                "DBName" => {
                    obj.db_name = Some(StringDeserializer::deserialize("DBName", stack)?);
                }
                "DBParameterGroups" => {
                    obj.db_parameter_groups.get_or_insert(vec![]).extend(
                        DBParameterGroupStatusListDeserializer::deserialize(
                            "DBParameterGroups",
                            stack,
                        )?,
                    );
                }
                "DBSecurityGroups" => {
                    obj.db_security_groups.get_or_insert(vec![]).extend(
                        DBSecurityGroupMembershipListDeserializer::deserialize(
                            "DBSecurityGroups",
                            stack,
                        )?,
                    );
                }
                "DBSubnetGroup" => {
                    obj.db_subnet_group = Some(DBSubnetGroupDeserializer::deserialize(
                        "DBSubnetGroup",
                        stack,
                    )?);
                }
                "DbInstancePort" => {
                    obj.db_instance_port =
                        Some(IntegerDeserializer::deserialize("DbInstancePort", stack)?);
                }
                "DbiResourceId" => {
                    obj.dbi_resource_id =
                        Some(StringDeserializer::deserialize("DbiResourceId", stack)?);
                }
                "DeletionProtection" => {
                    obj.deletion_protection = Some(BooleanOptionalDeserializer::deserialize(
                        "DeletionProtection",
                        stack,
                    )?);
                }
                "DomainMemberships" => {
                    obj.domain_memberships.get_or_insert(vec![]).extend(
                        DomainMembershipListDeserializer::deserialize("DomainMemberships", stack)?,
                    );
                }
                "EnabledCloudwatchLogsExports" => {
                    obj.enabled_cloudwatch_logs_exports
                        .get_or_insert(vec![])
                        .extend(LogTypeListDeserializer::deserialize(
                            "EnabledCloudwatchLogsExports",
                            stack,
                        )?);
                }
                "Endpoint" => {
                    obj.endpoint = Some(EndpointDeserializer::deserialize("Endpoint", stack)?);
                }
                "Engine" => {
                    obj.engine = Some(StringDeserializer::deserialize("Engine", stack)?);
                }
                "EngineVersion" => {
                    obj.engine_version =
                        Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                }
                "EnhancedMonitoringResourceArn" => {
                    obj.enhanced_monitoring_resource_arn = Some(StringDeserializer::deserialize(
                        "EnhancedMonitoringResourceArn",
                        stack,
                    )?);
                }
                "IAMDatabaseAuthenticationEnabled" => {
                    obj.iam_database_authentication_enabled =
                        Some(BooleanDeserializer::deserialize(
                            "IAMDatabaseAuthenticationEnabled",
                            stack,
                        )?);
                }
                "InstanceCreateTime" => {
                    obj.instance_create_time = Some(TStampDeserializer::deserialize(
                        "InstanceCreateTime",
                        stack,
                    )?);
                }
                "Iops" => {
                    obj.iops = Some(IntegerOptionalDeserializer::deserialize("Iops", stack)?);
                }
                "KmsKeyId" => {
                    obj.kms_key_id = Some(StringDeserializer::deserialize("KmsKeyId", stack)?);
                }
                "LatestRestorableTime" => {
                    obj.latest_restorable_time = Some(TStampDeserializer::deserialize(
                        "LatestRestorableTime",
                        stack,
                    )?);
                }
                "LicenseModel" => {
                    obj.license_model =
                        Some(StringDeserializer::deserialize("LicenseModel", stack)?);
                }
                "MasterUsername" => {
                    obj.master_username =
                        Some(StringDeserializer::deserialize("MasterUsername", stack)?);
                }
                "MonitoringInterval" => {
                    obj.monitoring_interval = Some(IntegerOptionalDeserializer::deserialize(
                        "MonitoringInterval",
                        stack,
                    )?);
                }
                "MonitoringRoleArn" => {
                    obj.monitoring_role_arn =
                        Some(StringDeserializer::deserialize("MonitoringRoleArn", stack)?);
                }
                "MultiAZ" => {
                    obj.multi_az = Some(BooleanDeserializer::deserialize("MultiAZ", stack)?);
                }
                "OptionGroupMemberships" => {
                    obj.option_group_memberships.get_or_insert(vec![]).extend(
                        OptionGroupMembershipListDeserializer::deserialize(
                            "OptionGroupMemberships",
                            stack,
                        )?,
                    );
                }
                "PendingModifiedValues" => {
                    obj.pending_modified_values =
                        Some(PendingModifiedValuesDeserializer::deserialize(
                            "PendingModifiedValues",
                            stack,
                        )?);
                }
                "PerformanceInsightsEnabled" => {
                    obj.performance_insights_enabled =
                        Some(BooleanOptionalDeserializer::deserialize(
                            "PerformanceInsightsEnabled",
                            stack,
                        )?);
                }
                "PerformanceInsightsKMSKeyId" => {
                    obj.performance_insights_kms_key_id = Some(StringDeserializer::deserialize(
                        "PerformanceInsightsKMSKeyId",
                        stack,
                    )?);
                }
                "PreferredBackupWindow" => {
                    obj.preferred_backup_window = Some(StringDeserializer::deserialize(
                        "PreferredBackupWindow",
                        stack,
                    )?);
                }
                "PreferredMaintenanceWindow" => {
                    obj.preferred_maintenance_window = Some(StringDeserializer::deserialize(
                        "PreferredMaintenanceWindow",
                        stack,
                    )?);
                }
                "PromotionTier" => {
                    obj.promotion_tier = Some(IntegerOptionalDeserializer::deserialize(
                        "PromotionTier",
                        stack,
                    )?);
                }
                "ReadReplicaDBClusterIdentifiers" => {
                    obj.read_replica_db_cluster_identifiers
                        .get_or_insert(vec![])
                        .extend(ReadReplicaDBClusterIdentifierListDeserializer::deserialize(
                            "ReadReplicaDBClusterIdentifiers",
                            stack,
                        )?);
                }
                "ReadReplicaDBInstanceIdentifiers" => {
                    obj.read_replica_db_instance_identifiers
                        .get_or_insert(vec![])
                        .extend(
                            ReadReplicaDBInstanceIdentifierListDeserializer::deserialize(
                                "ReadReplicaDBInstanceIdentifiers",
                                stack,
                            )?,
                        );
                }
                "ReadReplicaSourceDBInstanceIdentifier" => {
                    obj.read_replica_source_db_instance_identifier =
                        Some(StringDeserializer::deserialize(
                            "ReadReplicaSourceDBInstanceIdentifier",
                            stack,
                        )?);
                }
                "SecondaryAvailabilityZone" => {
                    obj.secondary_availability_zone = Some(StringDeserializer::deserialize(
                        "SecondaryAvailabilityZone",
                        stack,
                    )?);
                }
                "StatusInfos" => {
                    obj.status_infos.get_or_insert(vec![]).extend(
                        DBInstanceStatusInfoListDeserializer::deserialize("StatusInfos", stack)?,
                    );
                }
                "StorageEncrypted" => {
                    obj.storage_encrypted =
                        Some(BooleanDeserializer::deserialize("StorageEncrypted", stack)?);
                }
                "StorageType" => {
                    obj.storage_type = Some(StringDeserializer::deserialize("StorageType", stack)?);
                }
                "TdeCredentialArn" => {
                    obj.tde_credential_arn =
                        Some(StringDeserializer::deserialize("TdeCredentialArn", stack)?);
                }
                "Timezone" => {
                    obj.timezone = Some(StringDeserializer::deserialize("Timezone", stack)?);
                }
                "VpcSecurityGroups" => {
                    obj.vpc_security_groups.get_or_insert(vec![]).extend(
                        VpcSecurityGroupMembershipListDeserializer::deserialize(
                            "VpcSecurityGroups",
                            stack,
                        )?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBInstanceListDeserializer;
impl DBInstanceListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBInstance>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBInstance" {
                obj.push(DBInstanceDeserializer::deserialize("DBInstance", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBInstanceMessage {
    /// <p> A list of <a>DBInstance</a> instances.</p>
    pub db_instances: Option<Vec<DBInstance>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
    pub marker: Option<String>,
}

struct DBInstanceMessageDeserializer;
impl DBInstanceMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBInstanceMessage, XmlParseError> {
        deserialize_elements::<_, DBInstanceMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBInstances" => {
                    obj.db_instances.get_or_insert(vec![]).extend(
                        DBInstanceListDeserializer::deserialize("DBInstances", stack)?,
                    );
                }
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Provides a list of status information for a DB instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBInstanceStatusInfo {
    /// <p>Details of the error if there is an error for the instance. If the instance is not in an error state, this value is blank.</p>
    pub message: Option<String>,
    /// <p>Boolean value that is true if the instance is operating normally, or false if the instance is in an error state.</p>
    pub normal: Option<bool>,
    /// <p>Status of the DB instance. For a StatusType of read replica, the values can be replicating, error, stopped, or terminated.</p>
    pub status: Option<String>,
    /// <p>This value is currently "read replication."</p>
    pub status_type: Option<String>,
}

struct DBInstanceStatusInfoDeserializer;
impl DBInstanceStatusInfoDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBInstanceStatusInfo, XmlParseError> {
        deserialize_elements::<_, DBInstanceStatusInfo, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Message" => {
                    obj.message = Some(StringDeserializer::deserialize("Message", stack)?);
                }
                "Normal" => {
                    obj.normal = Some(BooleanDeserializer::deserialize("Normal", stack)?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "StatusType" => {
                    obj.status_type = Some(StringDeserializer::deserialize("StatusType", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBInstanceStatusInfoListDeserializer;
impl DBInstanceStatusInfoListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBInstanceStatusInfo>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBInstanceStatusInfo" {
                obj.push(DBInstanceStatusInfoDeserializer::deserialize(
                    "DBInstanceStatusInfo",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains the details of an Amazon Neptune DB parameter group.</p> <p>This data type is used as a response element in the <a>DescribeDBParameterGroups</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBParameterGroup {
    /// <p>The Amazon Resource Name (ARN) for the DB parameter group.</p>
    pub db_parameter_group_arn: Option<String>,
    /// <p>Provides the name of the DB parameter group family that this DB parameter group is compatible with.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p>Provides the name of the DB parameter group.</p>
    pub db_parameter_group_name: Option<String>,
    /// <p>Provides the customer-specified description for this DB parameter group.</p>
    pub description: Option<String>,
}

struct DBParameterGroupDeserializer;
impl DBParameterGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroup, XmlParseError> {
        deserialize_elements::<_, DBParameterGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBParameterGroupArn" => {
                    obj.db_parameter_group_arn = Some(StringDeserializer::deserialize(
                        "DBParameterGroupArn",
                        stack,
                    )?);
                }
                "DBParameterGroupFamily" => {
                    obj.db_parameter_group_family = Some(StringDeserializer::deserialize(
                        "DBParameterGroupFamily",
                        stack,
                    )?);
                }
                "DBParameterGroupName" => {
                    obj.db_parameter_group_name = Some(StringDeserializer::deserialize(
                        "DBParameterGroupName",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description = Some(StringDeserializer::deserialize("Description", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBParameterGroupDetails {
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>A list of <a>Parameter</a> values.</p>
    pub parameters: Option<Vec<Parameter>>,
}

struct DBParameterGroupDetailsDeserializer;
impl DBParameterGroupDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupDetails, XmlParseError> {
        deserialize_elements::<_, DBParameterGroupDetails, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "Parameters" => {
                        obj.parameters.get_or_insert(vec![]).extend(
                            ParametersListDeserializer::deserialize("Parameters", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DBParameterGroupListDeserializer;
impl DBParameterGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBParameterGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBParameterGroup" {
                obj.push(DBParameterGroupDeserializer::deserialize(
                    "DBParameterGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBParameterGroupNameMessage {
    /// <p>Provides the name of the DB parameter group.</p>
    pub db_parameter_group_name: Option<String>,
}

struct DBParameterGroupNameMessageDeserializer;
impl DBParameterGroupNameMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupNameMessage, XmlParseError> {
        deserialize_elements::<_, DBParameterGroupNameMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBParameterGroupName" => {
                        obj.db_parameter_group_name = Some(StringDeserializer::deserialize(
                            "DBParameterGroupName",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p><p>The status of the DB parameter group.</p> <p>This data type is used as a response element in the following actions:</p> <ul> <li> <p> <a>CreateDBInstance</a> </p> </li> <li> <p> <a>DeleteDBInstance</a> </p> </li> <li> <p> <a>ModifyDBInstance</a> </p> </li> <li> <p> <a>RebootDBInstance</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBParameterGroupStatus {
    /// <p>The name of the DP parameter group.</p>
    pub db_parameter_group_name: Option<String>,
    /// <p>The status of parameter updates.</p>
    pub parameter_apply_status: Option<String>,
}

struct DBParameterGroupStatusDeserializer;
impl DBParameterGroupStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupStatus, XmlParseError> {
        deserialize_elements::<_, DBParameterGroupStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBParameterGroupName" => {
                    obj.db_parameter_group_name = Some(StringDeserializer::deserialize(
                        "DBParameterGroupName",
                        stack,
                    )?);
                }
                "ParameterApplyStatus" => {
                    obj.parameter_apply_status = Some(StringDeserializer::deserialize(
                        "ParameterApplyStatus",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBParameterGroupStatusListDeserializer;
impl DBParameterGroupStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBParameterGroupStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBParameterGroup" {
                obj.push(DBParameterGroupStatusDeserializer::deserialize(
                    "DBParameterGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBParameterGroupsMessage {
    /// <p>A list of <a>DBParameterGroup</a> instances.</p>
    pub db_parameter_groups: Option<Vec<DBParameterGroup>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
}

struct DBParameterGroupsMessageDeserializer;
impl DBParameterGroupsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBParameterGroupsMessage, XmlParseError> {
        deserialize_elements::<_, DBParameterGroupsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBParameterGroups" => {
                        obj.db_parameter_groups.get_or_insert(vec![]).extend(
                            DBParameterGroupListDeserializer::deserialize(
                                "DBParameterGroups",
                                stack,
                            )?,
                        );
                    }
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Specifies membership in a designated DB security group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBSecurityGroupMembership {
    /// <p>The name of the DB security group.</p>
    pub db_security_group_name: Option<String>,
    /// <p>The status of the DB security group.</p>
    pub status: Option<String>,
}

struct DBSecurityGroupMembershipDeserializer;
impl DBSecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBSecurityGroupMembership, XmlParseError> {
        deserialize_elements::<_, DBSecurityGroupMembership, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBSecurityGroupName" => {
                        obj.db_security_group_name = Some(StringDeserializer::deserialize(
                            "DBSecurityGroupName",
                            stack,
                        )?);
                    }
                    "Status" => {
                        obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DBSecurityGroupMembershipListDeserializer;
impl DBSecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBSecurityGroupMembership>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBSecurityGroup" {
                obj.push(DBSecurityGroupMembershipDeserializer::deserialize(
                    "DBSecurityGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `DBSecurityGroupNameList` contents to a `SignedRequest`.
struct DBSecurityGroupNameListSerializer;
impl DBSecurityGroupNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Contains the details of an Amazon Neptune DB subnet group.</p> <p>This data type is used as a response element in the <a>DescribeDBSubnetGroups</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBSubnetGroup {
    /// <p>The Amazon Resource Name (ARN) for the DB subnet group.</p>
    pub db_subnet_group_arn: Option<String>,
    /// <p>Provides the description of the DB subnet group.</p>
    pub db_subnet_group_description: Option<String>,
    /// <p>The name of the DB subnet group.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Provides the status of the DB subnet group.</p>
    pub subnet_group_status: Option<String>,
    /// <p> Contains a list of <a>Subnet</a> elements.</p>
    pub subnets: Option<Vec<Subnet>>,
    /// <p>Provides the VpcId of the DB subnet group.</p>
    pub vpc_id: Option<String>,
}

struct DBSubnetGroupDeserializer;
impl DBSubnetGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBSubnetGroup, XmlParseError> {
        deserialize_elements::<_, DBSubnetGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBSubnetGroupArn" => {
                    obj.db_subnet_group_arn =
                        Some(StringDeserializer::deserialize("DBSubnetGroupArn", stack)?);
                }
                "DBSubnetGroupDescription" => {
                    obj.db_subnet_group_description = Some(StringDeserializer::deserialize(
                        "DBSubnetGroupDescription",
                        stack,
                    )?);
                }
                "DBSubnetGroupName" => {
                    obj.db_subnet_group_name =
                        Some(StringDeserializer::deserialize("DBSubnetGroupName", stack)?);
                }
                "SubnetGroupStatus" => {
                    obj.subnet_group_status =
                        Some(StringDeserializer::deserialize("SubnetGroupStatus", stack)?);
                }
                "Subnets" => {
                    obj.subnets
                        .get_or_insert(vec![])
                        .extend(SubnetListDeserializer::deserialize("Subnets", stack)?);
                }
                "VpcId" => {
                    obj.vpc_id = Some(StringDeserializer::deserialize("VpcId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBSubnetGroupMessage {
    /// <p> A list of <a>DBSubnetGroup</a> instances.</p>
    pub db_subnet_groups: Option<Vec<DBSubnetGroup>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
}

struct DBSubnetGroupMessageDeserializer;
impl DBSubnetGroupMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DBSubnetGroupMessage, XmlParseError> {
        deserialize_elements::<_, DBSubnetGroupMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBSubnetGroups" => {
                    obj.db_subnet_groups.get_or_insert(vec![]).extend(
                        DBSubnetGroupsDeserializer::deserialize("DBSubnetGroups", stack)?,
                    );
                }
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DBSubnetGroupsDeserializer;
impl DBSubnetGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DBSubnetGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DBSubnetGroup" {
                obj.push(DBSubnetGroupDeserializer::deserialize(
                    "DBSubnetGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBClusterMessage {
    /// <p><p>The DB cluster identifier for the DB cluster to be deleted. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match an existing DBClusterIdentifier.</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p><p> The DB cluster snapshot identifier of the new DB cluster snapshot created when <code>SkipFinalSnapshot</code> is set to <code>false</code>.</p> <note> <p> Specifying this parameter and also setting the <code>SkipFinalShapshot</code> parameter to true results in an error.</p> </note> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub final_db_snapshot_identifier: Option<String>,
    /// <p> Determines whether a final DB cluster snapshot is created before the DB cluster is deleted. If <code>true</code> is specified, no DB cluster snapshot is created. If <code>false</code> is specified, a DB cluster snapshot is created before the DB cluster is deleted.</p> <note> <p>You must specify a <code>FinalDBSnapshotIdentifier</code> parameter if <code>SkipFinalSnapshot</code> is <code>false</code>.</p> </note> <p>Default: <code>false</code> </p>
    pub skip_final_snapshot: Option<bool>,
}

/// Serialize `DeleteDBClusterMessage` contents to a `SignedRequest`.
struct DeleteDBClusterMessageSerializer;
impl DeleteDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.final_db_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "FinalDBSnapshotIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.skip_final_snapshot {
            params.put(&format!("{}{}", prefix, "SkipFinalSnapshot"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBClusterParameterGroupMessage {
    /// <p><p>The name of the DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be the name of an existing DB cluster parameter group.</p> </li> <li> <p>You can&#39;t delete a default DB cluster parameter group.</p> </li> <li> <p>Cannot be associated with any DB clusters.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: String,
}

/// Serialize `DeleteDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct DeleteDBClusterParameterGroupMessageSerializer;
impl DeleteDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct DeleteDBClusterResultDeserializer;
impl DeleteDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDBClusterResult, XmlParseError> {
        deserialize_elements::<_, DeleteDBClusterResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBCluster" => {
                    obj.db_cluster = Some(DBClusterDeserializer::deserialize("DBCluster", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBClusterSnapshotMessage {
    /// <p>The identifier of the DB cluster snapshot to delete.</p> <p>Constraints: Must be the name of an existing DB cluster snapshot in the <code>available</code> state.</p>
    pub db_cluster_snapshot_identifier: String,
}

/// Serialize `DeleteDBClusterSnapshotMessage` contents to a `SignedRequest`.
struct DeleteDBClusterSnapshotMessageSerializer;
impl DeleteDBClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteDBClusterSnapshotResult {
    pub db_cluster_snapshot: Option<DBClusterSnapshot>,
}

struct DeleteDBClusterSnapshotResultDeserializer;
impl DeleteDBClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDBClusterSnapshotResult, XmlParseError> {
        deserialize_elements::<_, DeleteDBClusterSnapshotResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterSnapshot" => {
                        obj.db_cluster_snapshot = Some(DBClusterSnapshotDeserializer::deserialize(
                            "DBClusterSnapshot",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBInstanceMessage {
    /// <p><p>The DB instance identifier for the DB instance to be deleted. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing DB instance.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p><p> The DBSnapshotIdentifier of the new DBSnapshot created when SkipFinalSnapshot is set to <code>false</code>.</p> <note> <p>Specifying this parameter and also setting the SkipFinalShapshot parameter to true results in an error.</p> </note> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 letters or numbers.</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> <li> <p>Cannot be specified when deleting a Read Replica.</p> </li> </ul></p>
    pub final_db_snapshot_identifier: Option<String>,
    /// <p> Determines whether a final DB snapshot is created before the DB instance is deleted. If <code>true</code> is specified, no DBSnapshot is created. If <code>false</code> is specified, a DB snapshot is created before the DB instance is deleted.</p> <p>Note that when a DB instance is in a failure state and has a status of 'failed', 'incompatible-restore', or 'incompatible-network', it can only be deleted when the SkipFinalSnapshot parameter is set to "true".</p> <p>Specify <code>true</code> when deleting a Read Replica.</p> <note> <p>The FinalDBSnapshotIdentifier parameter must be specified if SkipFinalSnapshot is <code>false</code>.</p> </note> <p>Default: <code>false</code> </p>
    pub skip_final_snapshot: Option<bool>,
}

/// Serialize `DeleteDBInstanceMessage` contents to a `SignedRequest`.
struct DeleteDBInstanceMessageSerializer;
impl DeleteDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.final_db_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "FinalDBSnapshotIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.skip_final_snapshot {
            params.put(&format!("{}{}", prefix, "SkipFinalSnapshot"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct DeleteDBInstanceResultDeserializer;
impl DeleteDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDBInstanceResult, XmlParseError> {
        deserialize_elements::<_, DeleteDBInstanceResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBInstance" => {
                    obj.db_instance =
                        Some(DBInstanceDeserializer::deserialize("DBInstance", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBParameterGroupMessage {
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be the name of an existing DB parameter group</p> </li> <li> <p>You can&#39;t delete a default DB parameter group</p> </li> <li> <p>Cannot be associated with any DB instances</p> </li> </ul></p>
    pub db_parameter_group_name: String,
}

/// Serialize `DeleteDBParameterGroupMessage` contents to a `SignedRequest`.
struct DeleteDBParameterGroupMessageSerializer;
impl DeleteDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBSubnetGroupMessage {
    /// <p>The name of the database subnet group to delete.</p> <note> <p>You can't delete the default subnet group.</p> </note> <p>Constraints:</p> <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
}

/// Serialize `DeleteDBSubnetGroupMessage` contents to a `SignedRequest`.
struct DeleteDBSubnetGroupMessageSerializer;
impl DeleteDBSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDBSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupName"),
            &obj.db_subnet_group_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventSubscriptionMessage {
    /// <p>The name of the event notification subscription you want to delete.</p>
    pub subscription_name: String,
}

/// Serialize `DeleteEventSubscriptionMessage` contents to a `SignedRequest`.
struct DeleteEventSubscriptionMessageSerializer;
impl DeleteEventSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteEventSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct DeleteEventSubscriptionResultDeserializer;
impl DeleteEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteEventSubscriptionResult, XmlParseError> {
        deserialize_elements::<_, DeleteEventSubscriptionResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EventSubscription" => {
                        obj.event_subscription = Some(EventSubscriptionDeserializer::deserialize(
                            "EventSubscription",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClusterParameterGroupsMessage {
    /// <p><p>The name of a specific DB cluster parameter group to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterParameterGroups</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBClusterParameterGroupsMessage` contents to a `SignedRequest`.
struct DescribeDBClusterParameterGroupsMessageSerializer;
impl DescribeDBClusterParameterGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClusterParameterGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClusterParametersMessage {
    /// <p><p>The name of a specific DB cluster parameter group to return parameter details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBClusterParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p> A value that indicates to return only parameters for a specific source. Parameter sources can be <code>engine</code>, <code>service</code>, or <code>customer</code>.</p>
    pub source: Option<String>,
}

/// Serialize `DescribeDBClusterParametersMessage` contents to a `SignedRequest`.
struct DescribeDBClusterParametersMessageSerializer;
impl DescribeDBClusterParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClusterParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClusterSnapshotAttributesMessage {
    /// <p>The identifier for the DB cluster snapshot to describe the attributes for.</p>
    pub db_cluster_snapshot_identifier: String,
}

/// Serialize `DescribeDBClusterSnapshotAttributesMessage` contents to a `SignedRequest`.
struct DescribeDBClusterSnapshotAttributesMessageSerializer;
impl DescribeDBClusterSnapshotAttributesMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeDBClusterSnapshotAttributesMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeDBClusterSnapshotAttributesResult {
    pub db_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

struct DescribeDBClusterSnapshotAttributesResultDeserializer;
impl DescribeDBClusterSnapshotAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeDBClusterSnapshotAttributesResult, XmlParseError> {
        deserialize_elements::<_, DescribeDBClusterSnapshotAttributesResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterSnapshotAttributesResult" => {
                        obj.db_cluster_snapshot_attributes_result =
                            Some(DBClusterSnapshotAttributesResultDeserializer::deserialize(
                                "DBClusterSnapshotAttributesResult",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClusterSnapshotsMessage {
    /// <p><p>The ID of the DB cluster to retrieve the list of DB cluster snapshots for. This parameter can&#39;t be used in conjunction with the <code>DBClusterSnapshotIdentifier</code> parameter. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p><p>A specific DB cluster snapshot identifier to describe. This parameter can&#39;t be used in conjunction with the <code>DBClusterIdentifier</code> parameter. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the identifier of an existing DBClusterSnapshot.</p> </li> <li> <p>If this identifier is for an automated snapshot, the <code>SnapshotType</code> parameter must also be specified.</p> </li> </ul></p>
    pub db_cluster_snapshot_identifier: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>True to include manual DB cluster snapshots that are public and can be copied or restored by any AWS account, and otherwise false. The default is <code>false</code>. The default is false.</p> <p>You can share a manual DB cluster snapshot as public by using the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    pub include_public: Option<bool>,
    /// <p>True to include shared manual DB cluster snapshots from other AWS accounts that this AWS account has been given permission to copy or restore, and otherwise false. The default is <code>false</code>.</p> <p>You can give an AWS account permission to restore a manual DB cluster snapshot from another AWS account by the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    pub include_shared: Option<bool>,
    /// <p>An optional pagination token provided by a previous <code>DescribeDBClusterSnapshots</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The type of DB cluster snapshots to be returned. You can specify one of the following values:</p> <ul> <li> <p> <code>automated</code> - Return all DB cluster snapshots that have been automatically taken by Amazon Neptune for my AWS account.</p> </li> <li> <p> <code>manual</code> - Return all DB cluster snapshots that have been taken by my AWS account.</p> </li> <li> <p> <code>shared</code> - Return all manual DB cluster snapshots that have been shared to my AWS account.</p> </li> <li> <p> <code>public</code> - Return all DB cluster snapshots that have been marked as public.</p> </li> </ul> <p>If you don't specify a <code>SnapshotType</code> value, then both automated and manual DB cluster snapshots are returned. You can include shared DB cluster snapshots with these results by setting the <code>IncludeShared</code> parameter to <code>true</code>. You can include public DB cluster snapshots with these results by setting the <code>IncludePublic</code> parameter to <code>true</code>.</p> <p>The <code>IncludeShared</code> and <code>IncludePublic</code> parameters don't apply for <code>SnapshotType</code> values of <code>manual</code> or <code>automated</code>. The <code>IncludePublic</code> parameter doesn't apply when <code>SnapshotType</code> is set to <code>shared</code>. The <code>IncludeShared</code> parameter doesn't apply when <code>SnapshotType</code> is set to <code>public</code>.</p>
    pub snapshot_type: Option<String>,
}

/// Serialize `DescribeDBClusterSnapshotsMessage` contents to a `SignedRequest`.
struct DescribeDBClusterSnapshotsMessageSerializer;
impl DescribeDBClusterSnapshotsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClusterSnapshotsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_cluster_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.include_public {
            params.put(&format!("{}{}", prefix, "IncludePublic"), &field_value);
        }
        if let Some(ref field_value) = obj.include_shared {
            params.put(&format!("{}{}", prefix, "IncludeShared"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_type {
            params.put(&format!("{}{}", prefix, "SnapshotType"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClustersMessage {
    /// <p><p>The user-supplied DB cluster identifier. If this parameter is specified, information from only the specific DB cluster is returned. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match an existing DBClusterIdentifier.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p><p>A filter that specifies one or more DB clusters to describe.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list will only include information about the DB clusters identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous <a>DescribeDBClusters</a> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBClustersMessage` contents to a `SignedRequest`.
struct DescribeDBClustersMessageSerializer;
impl DescribeDBClustersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBClustersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBEngineVersionsMessage {
    /// <p><p>The name of a specific DB parameter group family to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match an existing DBParameterGroupFamily.</p> </li> </ul></p>
    pub db_parameter_group_family: Option<String>,
    /// <p>Indicates that only the default version of the specified engine or engine and major version combination is returned.</p>
    pub default_only: Option<bool>,
    /// <p>The database engine to return.</p>
    pub engine: Option<String>,
    /// <p>The database engine version to return.</p> <p>Example: <code>5.1.49</code> </p>
    pub engine_version: Option<String>,
    /// <p>Not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>If this parameter is specified and the requested engine supports the <code>CharacterSetName</code> parameter for <code>CreateDBInstance</code>, the response includes a list of supported character sets for each engine version.</p>
    pub list_supported_character_sets: Option<bool>,
    /// <p>If this parameter is specified and the requested engine supports the <code>TimeZone</code> parameter for <code>CreateDBInstance</code>, the response includes a list of supported time zones for each engine version.</p>
    pub list_supported_timezones: Option<bool>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more than the <code>MaxRecords</code> value is available, a pagination token called a marker is included in the response so that the following results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBEngineVersionsMessage` contents to a `SignedRequest`.
struct DescribeDBEngineVersionsMessageSerializer;
impl DescribeDBEngineVersionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBEngineVersionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_parameter_group_family {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupFamily"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.default_only {
            params.put(&format!("{}{}", prefix, "DefaultOnly"), &field_value);
        }
        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.list_supported_character_sets {
            params.put(
                &format!("{}{}", prefix, "ListSupportedCharacterSets"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.list_supported_timezones {
            params.put(
                &format!("{}{}", prefix, "ListSupportedTimezones"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBInstancesMessage {
    /// <p><p>The user-supplied instance identifier. If this parameter is specified, information from only the specific DB instance is returned. This parameter isn&#39;t case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the identifier of an existing DBInstance.</p> </li> </ul></p>
    pub db_instance_identifier: Option<String>,
    /// <p><p>A filter that specifies one or more DB instances to describe.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list will only include information about the DB instances associated with the DB clusters identified by these ARNs.</p> </li> <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and DB instance Amazon Resource Names (ARNs). The results list will only include information about the DB instances identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeDBInstances</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBInstancesMessage` contents to a `SignedRequest`.
struct DescribeDBInstancesMessageSerializer;
impl DescribeDBInstancesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBInstancesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_instance_identifier {
            params.put(
                &format!("{}{}", prefix, "DBInstanceIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBParameterGroupsMessage {
    /// <p><p>The name of a specific DB parameter group to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous <code>DescribeDBParameterGroups</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBParameterGroupsMessage` contents to a `SignedRequest`.
struct DescribeDBParameterGroupsMessageSerializer;
impl DescribeDBParameterGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBParameterGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBParametersMessage {
    /// <p><p>The name of a specific DB parameter group to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous <code>DescribeDBParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The parameter types to return.</p> <p>Default: All parameter types returned</p> <p>Valid Values: <code>user | system | engine-default</code> </p>
    pub source: Option<String>,
}

/// Serialize `DescribeDBParametersMessage` contents to a `SignedRequest`.
struct DescribeDBParametersMessageSerializer;
impl DescribeDBParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBSubnetGroupsMessage {
    /// <p>The name of the DB subnet group to return details for.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous DescribeDBSubnetGroups request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeDBSubnetGroupsMessage` contents to a `SignedRequest`.
struct DescribeDBSubnetGroupsMessageSerializer;
impl DescribeDBSubnetGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDBSubnetGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEngineDefaultClusterParametersMessage {
    /// <p>The name of the DB cluster parameter group family to return engine parameter information for.</p>
    pub db_parameter_group_family: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeEngineDefaultClusterParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeEngineDefaultClusterParametersMessage` contents to a `SignedRequest`.
struct DescribeEngineDefaultClusterParametersMessageSerializer;
impl DescribeEngineDefaultClusterParametersMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeEngineDefaultClusterParametersMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeEngineDefaultClusterParametersResult {
    pub engine_defaults: Option<EngineDefaults>,
}

struct DescribeEngineDefaultClusterParametersResultDeserializer;
impl DescribeEngineDefaultClusterParametersResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEngineDefaultClusterParametersResult, XmlParseError> {
        deserialize_elements::<_, DescribeEngineDefaultClusterParametersResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EngineDefaults" => {
                        obj.engine_defaults = Some(EngineDefaultsDeserializer::deserialize(
                            "EngineDefaults",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEngineDefaultParametersMessage {
    /// <p>The name of the DB parameter group family.</p>
    pub db_parameter_group_family: String,
    /// <p>Not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribeEngineDefaultParameters</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeEngineDefaultParametersMessage` contents to a `SignedRequest`.
struct DescribeEngineDefaultParametersMessageSerializer;
impl DescribeEngineDefaultParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEngineDefaultParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupFamily"),
            &obj.db_parameter_group_family,
        );
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeEngineDefaultParametersResult {
    pub engine_defaults: Option<EngineDefaults>,
}

struct DescribeEngineDefaultParametersResultDeserializer;
impl DescribeEngineDefaultParametersResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEngineDefaultParametersResult, XmlParseError> {
        deserialize_elements::<_, DescribeEngineDefaultParametersResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EngineDefaults" => {
                        obj.engine_defaults = Some(EngineDefaultsDeserializer::deserialize(
                            "EngineDefaults",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventCategoriesMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The type of source that is generating the events.</p> <p>Valid values: db-instance | db-parameter-group | db-security-group | db-snapshot</p>
    pub source_type: Option<String>,
}

/// Serialize `DescribeEventCategoriesMessage` contents to a `SignedRequest`.
struct DescribeEventCategoriesMessageSerializer;
impl DescribeEventCategoriesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventCategoriesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventSubscriptionsMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The name of the event notification subscription you want to describe.</p>
    pub subscription_name: Option<String>,
}

/// Serialize `DescribeEventSubscriptionsMessage` contents to a `SignedRequest`.
struct DescribeEventSubscriptionsMessageSerializer;
impl DescribeEventSubscriptionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventSubscriptionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.subscription_name {
            params.put(&format!("{}{}", prefix, "SubscriptionName"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventsMessage {
    /// <p>The number of minutes to retrieve events for.</p> <p>Default: 60</p>
    pub duration: Option<i64>,
    /// <p> The end of the time interval for which to retrieve events, specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: 2009-07-08T18:00Z</p>
    pub end_time: Option<String>,
    /// <p>A list of event categories that trigger notifications for a event notification subscription.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous DescribeEvents request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p><p>The identifier of the event source for which events are returned. If not specified, then all sources are included in the response.</p> <p>Constraints:</p> <ul> <li> <p>If SourceIdentifier is supplied, SourceType must also be provided.</p> </li> <li> <p>If the source type is <code>DBInstance</code>, then a <code>DBInstanceIdentifier</code> must be supplied.</p> </li> <li> <p>If the source type is <code>DBSecurityGroup</code>, a <code>DBSecurityGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is <code>DBParameterGroup</code>, a <code>DBParameterGroupName</code> must be supplied.</p> </li> <li> <p>If the source type is <code>DBSnapshot</code>, a <code>DBSnapshotIdentifier</code> must be supplied.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub source_identifier: Option<String>,
    /// <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    pub source_type: Option<String>,
    /// <p> The beginning of the time interval to retrieve events for, specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: 2009-07-08T18:00Z</p>
    pub start_time: Option<String>,
}

/// Serialize `DescribeEventsMessage` contents to a `SignedRequest`.
struct DescribeEventsMessageSerializer;
impl DescribeEventsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.duration {
            params.put(&format!("{}{}", prefix, "Duration"), &field_value);
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(&format!("{}{}", prefix, "EndTime"), &field_value);
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.source_identifier {
            params.put(&format!("{}{}", prefix, "SourceIdentifier"), &field_value);
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(&format!("{}{}", prefix, "StartTime"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrderableDBInstanceOptionsMessage {
    /// <p>The DB instance class filter value. Specify this parameter to show only the available offerings matching the specified DB instance class.</p>
    pub db_instance_class: Option<String>,
    /// <p>The name of the engine to retrieve DB instance options for.</p>
    pub engine: String,
    /// <p>The engine version filter value. Specify this parameter to show only the available offerings matching the specified engine version.</p>
    pub engine_version: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The license model filter value. Specify this parameter to show only the available offerings matching the specified license model.</p>
    pub license_model: Option<String>,
    /// <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The VPC filter value. Specify this parameter to show only the available VPC or non-VPC offerings.</p>
    pub vpc: Option<bool>,
}

/// Serialize `DescribeOrderableDBInstanceOptionsMessage` contents to a `SignedRequest`.
struct DescribeOrderableDBInstanceOptionsMessageSerializer;
impl DescribeOrderableDBInstanceOptionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeOrderableDBInstanceOptionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_instance_class {
            params.put(&format!("{}{}", prefix, "DBInstanceClass"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.license_model {
            params.put(&format!("{}{}", prefix, "LicenseModel"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.vpc {
            params.put(&format!("{}{}", prefix, "Vpc"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePendingMaintenanceActionsMessage {
    /// <p><p>A filter that specifies one or more resources to return pending maintenance actions for.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list will only include pending maintenance actions for the DB clusters identified by these ARNs.</p> </li> <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and DB instance ARNs. The results list will only include pending maintenance actions for the DB instances identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous <code>DescribePendingMaintenanceActions</code> request. If this parameter is specified, the response includes only records beyond the marker, up to a number of records specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The ARN of a resource to return pending maintenance actions for.</p>
    pub resource_identifier: Option<String>,
}

/// Serialize `DescribePendingMaintenanceActionsMessage` contents to a `SignedRequest`.
struct DescribePendingMaintenanceActionsMessageSerializer;
impl DescribePendingMaintenanceActionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribePendingMaintenanceActionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.resource_identifier {
            params.put(&format!("{}{}", prefix, "ResourceIdentifier"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeValidDBInstanceModificationsMessage {
    /// <p>The customer identifier or the ARN of your DB instance.</p>
    pub db_instance_identifier: String,
}

/// Serialize `DescribeValidDBInstanceModificationsMessage` contents to a `SignedRequest`.
struct DescribeValidDBInstanceModificationsMessageSerializer;
impl DescribeValidDBInstanceModificationsMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeValidDBInstanceModificationsMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeValidDBInstanceModificationsResult {
    pub valid_db_instance_modifications_message: Option<ValidDBInstanceModificationsMessage>,
}

struct DescribeValidDBInstanceModificationsResultDeserializer;
impl DescribeValidDBInstanceModificationsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeValidDBInstanceModificationsResult, XmlParseError> {
        deserialize_elements::<_, DescribeValidDBInstanceModificationsResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ValidDBInstanceModificationsMessage" => {
                        obj.valid_db_instance_modifications_message = Some(
                            ValidDBInstanceModificationsMessageDeserializer::deserialize(
                                "ValidDBInstanceModificationsMessage",
                                stack,
                            )?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>An Active Directory Domain membership record associated with a DB instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DomainMembership {
    /// <p>The identifier of the Active Directory Domain.</p>
    pub domain: Option<String>,
    /// <p>The fully qualified domain name of the Active Directory Domain.</p>
    pub fqdn: Option<String>,
    /// <p>The name of the IAM role to be used when making API calls to the Directory Service.</p>
    pub iam_role_name: Option<String>,
    /// <p>The status of the DB instance's Active Directory Domain membership, such as joined, pending-join, failed etc).</p>
    pub status: Option<String>,
}

struct DomainMembershipDeserializer;
impl DomainMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainMembership, XmlParseError> {
        deserialize_elements::<_, DomainMembership, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Domain" => {
                    obj.domain = Some(StringDeserializer::deserialize("Domain", stack)?);
                }
                "FQDN" => {
                    obj.fqdn = Some(StringDeserializer::deserialize("FQDN", stack)?);
                }
                "IAMRoleName" => {
                    obj.iam_role_name =
                        Some(StringDeserializer::deserialize("IAMRoleName", stack)?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DomainMembershipListDeserializer;
impl DomainMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DomainMembership>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DomainMembership" {
                obj.push(DomainMembershipDeserializer::deserialize(
                    "DomainMembership",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DoubleDeserializer;
impl DoubleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DoubleOptionalDeserializer;
impl DoubleOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A range of double values.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DoubleRange {
    /// <p>The minimum value in the range.</p>
    pub from: Option<f64>,
    /// <p>The maximum value in the range.</p>
    pub to: Option<f64>,
}

struct DoubleRangeDeserializer;
impl DoubleRangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DoubleRange, XmlParseError> {
        deserialize_elements::<_, DoubleRange, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "From" => {
                    obj.from = Some(DoubleDeserializer::deserialize("From", stack)?);
                }
                "To" => {
                    obj.to = Some(DoubleDeserializer::deserialize("To", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DoubleRangeListDeserializer;
impl DoubleRangeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DoubleRange>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DoubleRange" {
                obj.push(DoubleRangeDeserializer::deserialize("DoubleRange", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Specifies a connection endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Endpoint {
    /// <p>Specifies the DNS address of the DB instance.</p>
    pub address: Option<String>,
    /// <p>Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>Specifies the port that the database engine is listening on.</p>
    pub port: Option<i64>,
}

struct EndpointDeserializer;
impl EndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Endpoint, XmlParseError> {
        deserialize_elements::<_, Endpoint, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Address" => {
                    obj.address = Some(StringDeserializer::deserialize("Address", stack)?);
                }
                "HostedZoneId" => {
                    obj.hosted_zone_id =
                        Some(StringDeserializer::deserialize("HostedZoneId", stack)?);
                }
                "Port" => {
                    obj.port = Some(IntegerDeserializer::deserialize("Port", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p> Contains the result of a successful invocation of the <a>DescribeEngineDefaultParameters</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EngineDefaults {
    /// <p>Specifies the name of the DB parameter group family that the engine default parameters apply to.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p> An optional pagination token provided by a previous EngineDefaults request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
    pub marker: Option<String>,
    /// <p>Contains a list of engine default parameters.</p>
    pub parameters: Option<Vec<Parameter>>,
}

struct EngineDefaultsDeserializer;
impl EngineDefaultsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EngineDefaults, XmlParseError> {
        deserialize_elements::<_, EngineDefaults, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBParameterGroupFamily" => {
                    obj.db_parameter_group_family = Some(StringDeserializer::deserialize(
                        "DBParameterGroupFamily",
                        stack,
                    )?);
                }
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                "Parameters" => {
                    obj.parameters.get_or_insert(vec![]).extend(
                        ParametersListDeserializer::deserialize("Parameters", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p> This data type is used as a response element in the <a>DescribeEvents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Event {
    /// <p>Specifies the date and time of the event.</p>
    pub date: Option<String>,
    /// <p>Specifies the category for the event.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>Provides the text of this event.</p>
    pub message: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the event.</p>
    pub source_arn: Option<String>,
    /// <p>Provides the identifier for the source of the event.</p>
    pub source_identifier: Option<String>,
    /// <p>Specifies the source type for this event.</p>
    pub source_type: Option<String>,
}

struct EventDeserializer;
impl EventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Event, XmlParseError> {
        deserialize_elements::<_, Event, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Date" => {
                    obj.date = Some(TStampDeserializer::deserialize("Date", stack)?);
                }
                "EventCategories" => {
                    obj.event_categories.get_or_insert(vec![]).extend(
                        EventCategoriesListDeserializer::deserialize("EventCategories", stack)?,
                    );
                }
                "Message" => {
                    obj.message = Some(StringDeserializer::deserialize("Message", stack)?);
                }
                "SourceArn" => {
                    obj.source_arn = Some(StringDeserializer::deserialize("SourceArn", stack)?);
                }
                "SourceIdentifier" => {
                    obj.source_identifier =
                        Some(StringDeserializer::deserialize("SourceIdentifier", stack)?);
                }
                "SourceType" => {
                    obj.source_type =
                        Some(SourceTypeDeserializer::deserialize("SourceType", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct EventCategoriesListDeserializer;
impl EventCategoriesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "EventCategory" {
                obj.push(StringDeserializer::deserialize("EventCategory", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `EventCategoriesList` contents to a `SignedRequest`.
struct EventCategoriesListSerializer;
impl EventCategoriesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Contains the results of a successful invocation of the <a>DescribeEventCategories</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventCategoriesMap {
    /// <p>The event categories for the specified source type</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The source type that the returned categories belong to</p>
    pub source_type: Option<String>,
}

struct EventCategoriesMapDeserializer;
impl EventCategoriesMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventCategoriesMap, XmlParseError> {
        deserialize_elements::<_, EventCategoriesMap, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EventCategories" => {
                    obj.event_categories.get_or_insert(vec![]).extend(
                        EventCategoriesListDeserializer::deserialize("EventCategories", stack)?,
                    );
                }
                "SourceType" => {
                    obj.source_type = Some(StringDeserializer::deserialize("SourceType", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct EventCategoriesMapListDeserializer;
impl EventCategoriesMapListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventCategoriesMap>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "EventCategoriesMap" {
                obj.push(EventCategoriesMapDeserializer::deserialize(
                    "EventCategoriesMap",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventCategoriesMessage {
    /// <p>A list of EventCategoriesMap data types.</p>
    pub event_categories_map_list: Option<Vec<EventCategoriesMap>>,
}

struct EventCategoriesMessageDeserializer;
impl EventCategoriesMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventCategoriesMessage, XmlParseError> {
        deserialize_elements::<_, EventCategoriesMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EventCategoriesMapList" => {
                    obj.event_categories_map_list.get_or_insert(vec![]).extend(
                        EventCategoriesMapListDeserializer::deserialize(
                            "EventCategoriesMapList",
                            stack,
                        )?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Event>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Event" {
                obj.push(EventDeserializer::deserialize("Event", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains the results of a successful invocation of the <a>DescribeEventSubscriptions</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventSubscription {
    /// <p>The event notification subscription Id.</p>
    pub cust_subscription_id: Option<String>,
    /// <p>The AWS customer account associated with the event notification subscription.</p>
    pub customer_aws_id: Option<String>,
    /// <p>A Boolean value indicating if the subscription is enabled. True indicates the subscription is enabled.</p>
    pub enabled: Option<bool>,
    /// <p>A list of event categories for the event notification subscription.</p>
    pub event_categories_list: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) for the event subscription.</p>
    pub event_subscription_arn: Option<String>,
    /// <p>The topic ARN of the event notification subscription.</p>
    pub sns_topic_arn: Option<String>,
    /// <p>A list of source IDs for the event notification subscription.</p>
    pub source_ids_list: Option<Vec<String>>,
    /// <p>The source type for the event notification subscription.</p>
    pub source_type: Option<String>,
    /// <p>The status of the event notification subscription.</p> <p>Constraints:</p> <p>Can be one of the following: creating | modifying | deleting | active | no-permission | topic-not-exist</p> <p>The status "no-permission" indicates that Neptune no longer has permission to post to the SNS topic. The status "topic-not-exist" indicates that the topic was deleted after the subscription was created.</p>
    pub status: Option<String>,
    /// <p>The time the event notification subscription was created.</p>
    pub subscription_creation_time: Option<String>,
}

struct EventSubscriptionDeserializer;
impl EventSubscriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventSubscription, XmlParseError> {
        deserialize_elements::<_, EventSubscription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CustSubscriptionId" => {
                    obj.cust_subscription_id = Some(StringDeserializer::deserialize(
                        "CustSubscriptionId",
                        stack,
                    )?);
                }
                "CustomerAwsId" => {
                    obj.customer_aws_id =
                        Some(StringDeserializer::deserialize("CustomerAwsId", stack)?);
                }
                "Enabled" => {
                    obj.enabled = Some(BooleanDeserializer::deserialize("Enabled", stack)?);
                }
                "EventCategoriesList" => {
                    obj.event_categories_list.get_or_insert(vec![]).extend(
                        EventCategoriesListDeserializer::deserialize("EventCategoriesList", stack)?,
                    );
                }
                "EventSubscriptionArn" => {
                    obj.event_subscription_arn = Some(StringDeserializer::deserialize(
                        "EventSubscriptionArn",
                        stack,
                    )?);
                }
                "SnsTopicArn" => {
                    obj.sns_topic_arn =
                        Some(StringDeserializer::deserialize("SnsTopicArn", stack)?);
                }
                "SourceIdsList" => {
                    obj.source_ids_list.get_or_insert(vec![]).extend(
                        SourceIdsListDeserializer::deserialize("SourceIdsList", stack)?,
                    );
                }
                "SourceType" => {
                    obj.source_type = Some(StringDeserializer::deserialize("SourceType", stack)?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "SubscriptionCreationTime" => {
                    obj.subscription_creation_time = Some(StringDeserializer::deserialize(
                        "SubscriptionCreationTime",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct EventSubscriptionsListDeserializer;
impl EventSubscriptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventSubscription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "EventSubscription" {
                obj.push(EventSubscriptionDeserializer::deserialize(
                    "EventSubscription",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventSubscriptionsMessage {
    /// <p>A list of EventSubscriptions data types.</p>
    pub event_subscriptions_list: Option<Vec<EventSubscription>>,
    /// <p> An optional pagination token provided by a previous DescribeOrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
}

struct EventSubscriptionsMessageDeserializer;
impl EventSubscriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventSubscriptionsMessage, XmlParseError> {
        deserialize_elements::<_, EventSubscriptionsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EventSubscriptionsList" => {
                        obj.event_subscriptions_list.get_or_insert(vec![]).extend(
                            EventSubscriptionsListDeserializer::deserialize(
                                "EventSubscriptionsList",
                                stack,
                            )?,
                        );
                    }
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventsMessage {
    /// <p> A list of <a>Event</a> instances.</p>
    pub events: Option<Vec<Event>>,
    /// <p> An optional pagination token provided by a previous Events request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
    pub marker: Option<String>,
}

struct EventsMessageDeserializer;
impl EventsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventsMessage, XmlParseError> {
        deserialize_elements::<_, EventsMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Events" => {
                    obj.events
                        .get_or_insert(vec![])
                        .extend(EventListDeserializer::deserialize("Events", stack)?);
                }
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FailoverDBClusterMessage {
    /// <p><p>A DB cluster identifier to force a failover for. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The name of the instance to promote to the primary instance.</p> <p>You must specify the instance identifier for an Read Replica in the DB cluster. For example, <code>mydbcluster-replica1</code>.</p>
    pub target_db_instance_identifier: Option<String>,
}

/// Serialize `FailoverDBClusterMessage` contents to a `SignedRequest`.
struct FailoverDBClusterMessageSerializer;
impl FailoverDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &FailoverDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "DBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.target_db_instance_identifier {
            params.put(
                &format!("{}{}", prefix, "TargetDBInstanceIdentifier"),
                &field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FailoverDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct FailoverDBClusterResultDeserializer;
impl FailoverDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FailoverDBClusterResult, XmlParseError> {
        deserialize_elements::<_, FailoverDBClusterResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(DBClusterDeserializer::deserialize("DBCluster", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>This type is not currently supported.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>This parameter is not currently supported.</p>
    pub name: String,
    /// <p>This parameter is not currently supported.</p>
    pub values: Vec<String>,
}

/// Serialize `Filter` contents to a `SignedRequest`.
struct FilterSerializer;
impl FilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Filter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        FilterValueListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Value"),
            &obj.values,
        );
    }
}

/// Serialize `FilterList` contents to a `SignedRequest`.
struct FilterListSerializer;
impl FilterListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Filter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            FilterSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `FilterValueList` contents to a `SignedRequest`.
struct FilterValueListSerializer;
impl FilterValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct IntegerOptionalDeserializer;
impl IntegerOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `KeyList` contents to a `SignedRequest`.
struct KeyListSerializer;
impl KeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The Amazon Neptune resource with tags to be listed. This value is an Amazon Resource Name (ARN). For information about creating an ARN, see <a href="https://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_name: String,
}

/// Serialize `ListTagsForResourceMessage` contents to a `SignedRequest`.
struct ListTagsForResourceMessageSerializer;
impl ListTagsForResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListTagsForResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filter"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "ResourceName"), &obj.resource_name);
    }
}

struct LogTypeListDeserializer;
impl LogTypeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StringDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `LogTypeList` contents to a `SignedRequest`.
struct LogTypeListSerializer;
impl LogTypeListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBClusterMessage {
    /// <p>A value that specifies whether the modifications in this request and any pending modifications are asynchronously applied as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the DB cluster. If this parameter is set to <code>false</code>, changes to the DB cluster are applied during the next maintenance window.</p> <p>The <code>ApplyImmediately</code> parameter only affects the <code>NewDBClusterIdentifier</code> and <code>MasterUserPassword</code> values. If you set the <code>ApplyImmediately</code> parameter value to false, then changes to the <code>NewDBClusterIdentifier</code> and <code>MasterUserPassword</code> values are applied during the next maintenance window. All other changes are applied immediately, regardless of the value of the <code>ApplyImmediately</code> parameter.</p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p><p>The number of days for which automated backups are retained. You must specify a minimum value of 1.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 1 to 35</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p>The configuration setting for the log types to be enabled for export to CloudWatch Logs for a specific DB cluster.</p>
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    /// <p><p>The DB cluster identifier for the cluster being modified. This parameter is not case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p>The name of the DB cluster parameter group to use for the DB cluster.</p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>A value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. </p>
    pub deletion_protection: Option<bool>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The version number of the database engine. Currently, setting this parameter has no effect. To upgrade your database engine to the most recent release, use the <a>ApplyPendingMaintenanceAction</a> API.</p> <p>For a list of valid engine versions, see <a>CreateDBInstance</a>, or call <a>DescribeDBEngineVersions</a>.</p>
    pub engine_version: Option<String>,
    /// <p>The new password for the master database user. This password can contain any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain from 8 to 41 characters.</p>
    pub master_user_password: Option<String>,
    /// <p>The new DB cluster identifier for the DB cluster when renaming a DB cluster. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens</p> </li> <li> <p>The first character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-cluster2</code> </p>
    pub new_db_cluster_identifier: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the DB cluster accepts connections.</p> <p>Constraints: Value must be <code>1150-65535</code> </p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p><p>The daily time range during which automated backups are created if automated backups are enabled, using the <code>BackupRetentionPeriod</code> parameter.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li> <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A list of VPC security groups that the DB cluster will belong to.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `ModifyDBClusterMessage` contents to a `SignedRequest`.
struct ModifyDBClusterMessageSerializer;
impl ModifyDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.apply_immediately {
            params.put(&format!("{}{}", prefix, "ApplyImmediately"), &field_value);
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cloudwatch_logs_export_configuration {
            CloudwatchLogsExportConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CloudwatchLogsExportConfiguration"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.deletion_protection {
            params.put(&format!("{}{}", prefix, "DeletionProtection"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.new_db_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "NewDBClusterIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBClusterParameterGroupMessage {
    /// <p>The name of the DB cluster parameter group to modify.</p>
    pub db_cluster_parameter_group_name: String,
    /// <p>A list of parameters in the DB cluster parameter group to modify.</p>
    pub parameters: Vec<Parameter>,
}

/// Serialize `ModifyDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct ModifyDBClusterParameterGroupMessageSerializer;
impl ModifyDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        ParametersListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Parameter"),
            &obj.parameters,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct ModifyDBClusterResultDeserializer;
impl ModifyDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBClusterResult, XmlParseError> {
        deserialize_elements::<_, ModifyDBClusterResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBCluster" => {
                    obj.db_cluster = Some(DBClusterDeserializer::deserialize("DBCluster", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBClusterSnapshotAttributeMessage {
    /// <p>The name of the DB cluster snapshot attribute to modify.</p> <p>To manage authorization for other AWS accounts to copy or restore a manual DB cluster snapshot, set this value to <code>restore</code>.</p>
    pub attribute_name: String,
    /// <p>The identifier for the DB cluster snapshot to modify the attributes for.</p>
    pub db_cluster_snapshot_identifier: String,
    /// <p>A list of DB cluster snapshot attributes to add to the attribute specified by <code>AttributeName</code>.</p> <p>To authorize other AWS accounts to copy or restore a manual DB cluster snapshot, set this list to include one or more AWS account IDs, or <code>all</code> to make the manual DB cluster snapshot restorable by any AWS account. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts.</p>
    pub values_to_add: Option<Vec<String>>,
    /// <p>A list of DB cluster snapshot attributes to remove from the attribute specified by <code>AttributeName</code>.</p> <p>To remove authorization for other AWS accounts to copy or restore a manual DB cluster snapshot, set this list to include one or more AWS account identifiers, or <code>all</code> to remove authorization for any AWS account to copy or restore the DB cluster snapshot. If you specify <code>all</code>, an AWS account whose account ID is explicitly added to the <code>restore</code> attribute can still copy or restore a manual DB cluster snapshot.</p>
    pub values_to_remove: Option<Vec<String>>,
}

/// Serialize `ModifyDBClusterSnapshotAttributeMessage` contents to a `SignedRequest`.
struct ModifyDBClusterSnapshotAttributeMessageSerializer;
impl ModifyDBClusterSnapshotAttributeMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBClusterSnapshotAttributeMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AttributeName"),
            &obj.attribute_name,
        );
        params.put(
            &format!("{}{}", prefix, "DBClusterSnapshotIdentifier"),
            &obj.db_cluster_snapshot_identifier,
        );
        if let Some(ref field_value) = obj.values_to_add {
            AttributeValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeValue"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.values_to_remove {
            AttributeValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeValue"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyDBClusterSnapshotAttributeResult {
    pub db_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

struct ModifyDBClusterSnapshotAttributeResultDeserializer;
impl ModifyDBClusterSnapshotAttributeResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBClusterSnapshotAttributeResult, XmlParseError> {
        deserialize_elements::<_, ModifyDBClusterSnapshotAttributeResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBClusterSnapshotAttributesResult" => {
                        obj.db_cluster_snapshot_attributes_result =
                            Some(DBClusterSnapshotAttributesResultDeserializer::deserialize(
                                "DBClusterSnapshotAttributesResult",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBInstanceMessage {
    /// <p>The new amount of storage (in gibibytes) to allocate for the DB instance.</p> <p>Not applicable. Storage is managed by the DB Cluster.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that major version upgrades are allowed. Changing this parameter doesn't result in an outage and the change is asynchronously applied as soon as possible.</p>
    pub allow_major_version_upgrade: Option<bool>,
    /// <p>Specifies whether the modifications in this request and any pending modifications are asynchronously applied as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the DB instance.</p> <p> If this parameter is set to <code>false</code>, changes to the DB instance are applied during the next maintenance window. Some parameter changes can cause an outage and are applied on the next call to <a>RebootDBInstance</a>, or the next failure reboot.</p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p> Indicates that minor version upgrades are applied automatically to the DB instance during the maintenance window. Changing this parameter doesn't result in an outage except in the following case and the change is asynchronously applied as soon as possible. An outage will result if this parameter is set to <code>true</code> during the maintenance window, and a newer minor version is available, and Neptune has enabled auto patching for that engine version.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Not applicable. The retention period for automated backups is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Default: Uses existing setting</p>
    pub backup_retention_period: Option<i64>,
    /// <p>Indicates the certificate that needs to be associated with the instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p>The configuration setting for the log types to be enabled for export to CloudWatch Logs for a specific DB instance or DB cluster.</p>
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    /// <p>True to copy all tags from the DB instance to snapshots of the DB instance, and otherwise false. The default is false.</p>
    pub copy_tags_to_snapshot: Option<bool>,
    /// <p>The new compute and memory capacity of the DB instance, for example, <code>db.m4.large</code>. Not all DB instance classes are available in all AWS Regions.</p> <p>If you modify the DB instance class, an outage occurs during the change. The change is applied during the next maintenance window, unless <code>ApplyImmediately</code> is specified as <code>true</code> for this request.</p> <p>Default: Uses existing setting</p>
    pub db_instance_class: Option<String>,
    /// <p><p>The DB instance identifier. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBInstance.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p>The name of the DB parameter group to apply to the DB instance. Changing this setting doesn't result in an outage. The parameter group name itself is changed immediately, but the actual parameter changes are not applied until you reboot the instance without failover. The db instance will NOT be rebooted automatically and the parameter changes will NOT be applied during the next maintenance window.</p> <p>Default: Uses existing setting</p> <p>Constraints: The DB parameter group must be in the same DB parameter group family as this DB instance.</p>
    pub db_parameter_group_name: Option<String>,
    /// <p>The port number on which the database accepts connections.</p> <p>The value of the <code>DBPortNumber</code> parameter must not match any of the port values specified for options in the option group for the DB instance.</p> <p>Your database will restart when you change the <code>DBPortNumber</code> value regardless of the value of the <code>ApplyImmediately</code> parameter.</p> <p> Default: <code>8182</code> </p>
    pub db_port_number: Option<i64>,
    /// <p><p>A list of DB security groups to authorize on this DB instance. Changing this setting doesn&#39;t result in an outage and the change is asynchronously applied as soon as possible.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match existing DBSecurityGroups.</p> </li> </ul></p>
    pub db_security_groups: Option<Vec<String>>,
    /// <p>The new DB subnet group for the DB instance. You can use this parameter to move your DB instance to a different VPC.</p> <p>Changing the subnet group causes an outage during the change. The change is applied during the next maintenance window, unless you specify <code>true</code> for the <code>ApplyImmediately</code> parameter.</p> <p>Constraints: If supplied, must match the name of an existing DBSubnetGroup.</p> <p>Example: <code>mySubnetGroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>A value that indicates whether the DB instance has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. </p>
    pub deletion_protection: Option<bool>,
    /// <p>Not supported.</p>
    pub domain: Option<String>,
    /// <p>Not supported</p>
    pub domain_iam_role_name: Option<String>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>You can enable IAM database authentication for the following database engines</p> <p>Not applicable. Mapping AWS IAM accounts to database accounts is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub enable_performance_insights: Option<bool>,
    /// <p>The version number of the database engine to upgrade to. Currently, setting this parameter has no effect. To upgrade your database engine to the most recent release, use the <a>ApplyPendingMaintenanceAction</a> API.</p>
    pub engine_version: Option<String>,
    /// <p>The new Provisioned IOPS (I/O operations per second) value for the instance.</p> <p>Changing this setting doesn't result in an outage and the change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code> for this request.</p> <p>Default: Uses existing setting</p>
    pub iops: Option<i64>,
    /// <p>Not supported.</p>
    pub license_model: Option<String>,
    /// <p>Not applicable.</p>
    pub master_user_password: Option<String>,
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.</p> <p>If <code>MonitoringRoleArn</code> is specified, then you must also set <code>MonitoringInterval</code> to a value other than 0.</p> <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code> </p>
    pub monitoring_interval: Option<i64>,
    /// <p>The ARN for the IAM role that permits Neptune to send enhanced monitoring metrics to Amazon CloudWatch Logs. For example, <code>arn:aws:iam:123456789012:role/emaccess</code>.</p> <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a <code>MonitoringRoleArn</code> value.</p>
    pub monitoring_role_arn: Option<String>,
    /// <p>Specifies if the DB instance is a Multi-AZ deployment. Changing this parameter doesn't result in an outage and the change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code> for this request.</p>
    pub multi_az: Option<bool>,
    /// <p> The new DB instance identifier for the DB instance when renaming a DB instance. When you change the DB instance identifier, an instance reboot will occur immediately if you set <code>Apply Immediately</code> to true, or will occur during the next maintenance window if <code>Apply Immediately</code> to false. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>mydbinstance</code> </p>
    pub new_db_instance_identifier: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub option_group_name: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub performance_insights_kms_key_id: Option<String>,
    /// <p><p> The daily time range during which automated backups are created if automated backups are enabled.</p> <p>Not applicable. The daily time range for creating automated backups is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the format hh24:mi-hh24:mi</p> </li> <li> <p>Must be in Universal Time Coordinated (UTC)</p> </li> <li> <p>Must not conflict with the preferred maintenance window</p> </li> <li> <p>Must be at least 30 minutes</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range (in UTC) during which system maintenance can occur, which might result in an outage. Changing this parameter doesn't result in an outage, except in the following situation, and the change is asynchronously applied as soon as possible. If there are pending actions that cause a reboot, and the maintenance window is changed to include the current time, then changing this parameter will cause a reboot of the DB instance. If moving this window to the current time, there must be at least 30 minutes between the current time and end of the window to ensure pending changes are applied.</p> <p>Default: Uses existing setting</p> <p>Format: ddd:hh24:mi-ddd:hh24:mi</p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Must be at least 30 minutes</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which a Read Replica is promoted to the primary instance after a failure of the existing primary instance.</p> <p>Default: 1</p> <p>Valid Values: 0 - 15</p>
    pub promotion_tier: Option<i64>,
    /// <p>Not supported.</p>
    pub storage_type: Option<String>,
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    pub tde_credential_arn: Option<String>,
    /// <p>The password for the given ARN from the key store in order to access the device.</p>
    pub tde_credential_password: Option<String>,
    /// <p><p>A list of EC2 VPC security groups to authorize on this DB instance. This change is asynchronously applied as soon as possible.</p> <p>Not applicable. The associated list of EC2 VPC security groups is managed by the DB cluster. For more information, see <a>ModifyDBCluster</a>.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match existing VpcSecurityGroupIds.</p> </li> </ul></p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `ModifyDBInstanceMessage` contents to a `SignedRequest`.
struct ModifyDBInstanceMessageSerializer;
impl ModifyDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allocated_storage {
            params.put(&format!("{}{}", prefix, "AllocatedStorage"), &field_value);
        }
        if let Some(ref field_value) = obj.allow_major_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AllowMajorVersionUpgrade"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.apply_immediately {
            params.put(&format!("{}{}", prefix, "ApplyImmediately"), &field_value);
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.backup_retention_period {
            params.put(
                &format!("{}{}", prefix, "BackupRetentionPeriod"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.ca_certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "CACertificateIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cloudwatch_logs_export_configuration {
            CloudwatchLogsExportConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CloudwatchLogsExportConfiguration"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.copy_tags_to_snapshot {
            params.put(&format!("{}{}", prefix, "CopyTagsToSnapshot"), &field_value);
        }
        if let Some(ref field_value) = obj.db_instance_class {
            params.put(&format!("{}{}", prefix, "DBInstanceClass"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.db_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_port_number {
            params.put(&format!("{}{}", prefix, "DBPortNumber"), &field_value);
        }
        if let Some(ref field_value) = obj.db_security_groups {
            DBSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DBSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.deletion_protection {
            params.put(&format!("{}{}", prefix, "DeletionProtection"), &field_value);
        }
        if let Some(ref field_value) = obj.domain {
            params.put(&format!("{}{}", prefix, "Domain"), &field_value);
        }
        if let Some(ref field_value) = obj.domain_iam_role_name {
            params.put(&format!("{}{}", prefix, "DomainIAMRoleName"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_performance_insights {
            params.put(
                &format!("{}{}", prefix, "EnablePerformanceInsights"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.iops {
            params.put(&format!("{}{}", prefix, "Iops"), &field_value);
        }
        if let Some(ref field_value) = obj.license_model {
            params.put(&format!("{}{}", prefix, "LicenseModel"), &field_value);
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(&format!("{}{}", prefix, "MasterUserPassword"), &field_value);
        }
        if let Some(ref field_value) = obj.monitoring_interval {
            params.put(&format!("{}{}", prefix, "MonitoringInterval"), &field_value);
        }
        if let Some(ref field_value) = obj.monitoring_role_arn {
            params.put(&format!("{}{}", prefix, "MonitoringRoleArn"), &field_value);
        }
        if let Some(ref field_value) = obj.multi_az {
            params.put(&format!("{}{}", prefix, "MultiAZ"), &field_value);
        }
        if let Some(ref field_value) = obj.new_db_instance_identifier {
            params.put(
                &format!("{}{}", prefix, "NewDBInstanceIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.performance_insights_kms_key_id {
            params.put(
                &format!("{}{}", prefix, "PerformanceInsightsKMSKeyId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_backup_window {
            params.put(
                &format!("{}{}", prefix, "PreferredBackupWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.promotion_tier {
            params.put(&format!("{}{}", prefix, "PromotionTier"), &field_value);
        }

        if let Some(ref field_value) = obj.storage_type {
            params.put(&format!("{}{}", prefix, "StorageType"), &field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_arn {
            params.put(&format!("{}{}", prefix, "TdeCredentialArn"), &field_value);
        }
        if let Some(ref field_value) = obj.tde_credential_password {
            params.put(
                &format!("{}{}", prefix, "TdeCredentialPassword"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct ModifyDBInstanceResultDeserializer;
impl ModifyDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBInstanceResult, XmlParseError> {
        deserialize_elements::<_, ModifyDBInstanceResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBInstance" => {
                    obj.db_instance =
                        Some(DBInstanceDeserializer::deserialize("DBInstance", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBParameterGroupMessage {
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: String,
    /// <p><p>An array of parameter names, values, and the apply method for the parameter update. At least one parameter name, value, and apply method must be supplied; subsequent arguments are optional. A maximum of 20 parameters can be modified in a single request.</p> <p>Valid Values (for the application method): <code>immediate | pending-reboot</code> </p> <note> <p>You can use the immediate value with dynamic parameters only. You can use the pending-reboot value for both dynamic and static parameters, and changes are applied when you reboot the DB instance without failover.</p> </note></p>
    pub parameters: Vec<Parameter>,
}

/// Serialize `ModifyDBParameterGroupMessage` contents to a `SignedRequest`.
struct ModifyDBParameterGroupMessageSerializer;
impl ModifyDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        ParametersListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Parameter"),
            &obj.parameters,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBSubnetGroupMessage {
    /// <p>The description for the DB subnet group.</p>
    pub db_subnet_group_description: Option<String>,
    /// <p>The name for the DB subnet group. This value is stored as a lowercase string. You can't modify the default subnet group.</p> <p>Constraints: Must match the name of an existing DBSubnetGroup. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
    /// <p>The EC2 subnet IDs for the DB subnet group.</p>
    pub subnet_ids: Vec<String>,
}

/// Serialize `ModifyDBSubnetGroupMessage` contents to a `SignedRequest`.
struct ModifyDBSubnetGroupMessageSerializer;
impl ModifyDBSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.db_subnet_group_description {
            params.put(
                &format!("{}{}", prefix, "DBSubnetGroupDescription"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBSubnetGroupName"),
            &obj.db_subnet_group_name,
        );
        SubnetIdentifierListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SubnetIdentifier"),
            &obj.subnet_ids,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyDBSubnetGroupResult {
    pub db_subnet_group: Option<DBSubnetGroup>,
}

struct ModifyDBSubnetGroupResultDeserializer;
impl ModifyDBSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyDBSubnetGroupResult, XmlParseError> {
        deserialize_elements::<_, ModifyDBSubnetGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBSubnetGroup" => {
                        obj.db_subnet_group = Some(DBSubnetGroupDeserializer::deserialize(
                            "DBSubnetGroup",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyEventSubscriptionMessage {
    /// <p> A Boolean value; set to <b>true</b> to activate the subscription.</p>
    pub enabled: Option<bool>,
    /// <p> A list of event categories for a SourceType that you want to subscribe to. You can see a list of the categories for a given SourceType by using the <b>DescribeEventCategories</b> action.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>
    pub sns_topic_arn: Option<String>,
    /// <p>The type of source that is generating the events. For example, if you want to be notified of events generated by a DB instance, you would set this parameter to db-instance. if this value is not specified, all events are returned.</p> <p>Valid values: db-instance | db-parameter-group | db-security-group | db-snapshot</p>
    pub source_type: Option<String>,
    /// <p>The name of the event notification subscription.</p>
    pub subscription_name: String,
}

/// Serialize `ModifyEventSubscriptionMessage` contents to a `SignedRequest`.
struct ModifyEventSubscriptionMessageSerializer;
impl ModifyEventSubscriptionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyEventSubscriptionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.enabled {
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.sns_topic_arn {
            params.put(&format!("{}{}", prefix, "SnsTopicArn"), &field_value);
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(&format!("{}{}", prefix, "SourceType"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct ModifyEventSubscriptionResultDeserializer;
impl ModifyEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyEventSubscriptionResult, XmlParseError> {
        deserialize_elements::<_, ModifyEventSubscriptionResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EventSubscription" => {
                        obj.event_subscription = Some(EventSubscriptionDeserializer::deserialize(
                            "EventSubscription",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Provides information on the option groups the DB instance is a member of.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OptionGroupMembership {
    /// <p>The name of the option group that the instance belongs to.</p>
    pub option_group_name: Option<String>,
    /// <p>The status of the DB instance's option group membership. Valid values are: <code>in-sync</code>, <code>pending-apply</code>, <code>pending-removal</code>, <code>pending-maintenance-apply</code>, <code>pending-maintenance-removal</code>, <code>applying</code>, <code>removing</code>, and <code>failed</code>.</p>
    pub status: Option<String>,
}

struct OptionGroupMembershipDeserializer;
impl OptionGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OptionGroupMembership, XmlParseError> {
        deserialize_elements::<_, OptionGroupMembership, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "OptionGroupName" => {
                    obj.option_group_name =
                        Some(StringDeserializer::deserialize("OptionGroupName", stack)?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct OptionGroupMembershipListDeserializer;
impl OptionGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OptionGroupMembership>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OptionGroupMembership" {
                obj.push(OptionGroupMembershipDeserializer::deserialize(
                    "OptionGroupMembership",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains a list of available options for a DB instance.</p> <p> This data type is used as a response element in the <a>DescribeOrderableDBInstanceOptions</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OrderableDBInstanceOption {
    /// <p>A list of Availability Zones for a DB instance.</p>
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The DB instance class for a DB instance.</p>
    pub db_instance_class: Option<String>,
    /// <p>The engine type of a DB instance.</p>
    pub engine: Option<String>,
    /// <p>The engine version of a DB instance.</p>
    pub engine_version: Option<String>,
    /// <p>The license model for a DB instance.</p>
    pub license_model: Option<String>,
    /// <p>Maximum total provisioned IOPS for a DB instance.</p>
    pub max_iops_per_db_instance: Option<i64>,
    /// <p>Maximum provisioned IOPS per GiB for a DB instance.</p>
    pub max_iops_per_gib: Option<f64>,
    /// <p>Maximum storage size for a DB instance.</p>
    pub max_storage_size: Option<i64>,
    /// <p>Minimum total provisioned IOPS for a DB instance.</p>
    pub min_iops_per_db_instance: Option<i64>,
    /// <p>Minimum provisioned IOPS per GiB for a DB instance.</p>
    pub min_iops_per_gib: Option<f64>,
    /// <p>Minimum storage size for a DB instance.</p>
    pub min_storage_size: Option<i64>,
    /// <p>Indicates whether a DB instance is Multi-AZ capable.</p>
    pub multi_az_capable: Option<bool>,
    /// <p>Indicates whether a DB instance can have a Read Replica.</p>
    pub read_replica_capable: Option<bool>,
    /// <p>Indicates the storage type for a DB instance.</p>
    pub storage_type: Option<String>,
    /// <p>Indicates whether a DB instance supports Enhanced Monitoring at intervals from 1 to 60 seconds.</p>
    pub supports_enhanced_monitoring: Option<bool>,
    /// <p>Indicates whether a DB instance supports IAM database authentication.</p>
    pub supports_iam_database_authentication: Option<bool>,
    /// <p>Indicates whether a DB instance supports provisioned IOPS.</p>
    pub supports_iops: Option<bool>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub supports_performance_insights: Option<bool>,
    /// <p>Indicates whether a DB instance supports encrypted storage.</p>
    pub supports_storage_encryption: Option<bool>,
    /// <p>Indicates whether a DB instance is in a VPC.</p>
    pub vpc: Option<bool>,
}

struct OrderableDBInstanceOptionDeserializer;
impl OrderableDBInstanceOptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OrderableDBInstanceOption, XmlParseError> {
        deserialize_elements::<_, OrderableDBInstanceOption, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AvailabilityZones" => {
                        obj.availability_zones.get_or_insert(vec![]).extend(
                            AvailabilityZoneListDeserializer::deserialize(
                                "AvailabilityZones",
                                stack,
                            )?,
                        );
                    }
                    "DBInstanceClass" => {
                        obj.db_instance_class =
                            Some(StringDeserializer::deserialize("DBInstanceClass", stack)?);
                    }
                    "Engine" => {
                        obj.engine = Some(StringDeserializer::deserialize("Engine", stack)?);
                    }
                    "EngineVersion" => {
                        obj.engine_version =
                            Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                    }
                    "LicenseModel" => {
                        obj.license_model =
                            Some(StringDeserializer::deserialize("LicenseModel", stack)?);
                    }
                    "MaxIopsPerDbInstance" => {
                        obj.max_iops_per_db_instance =
                            Some(IntegerOptionalDeserializer::deserialize(
                                "MaxIopsPerDbInstance",
                                stack,
                            )?);
                    }
                    "MaxIopsPerGib" => {
                        obj.max_iops_per_gib = Some(DoubleOptionalDeserializer::deserialize(
                            "MaxIopsPerGib",
                            stack,
                        )?);
                    }
                    "MaxStorageSize" => {
                        obj.max_storage_size = Some(IntegerOptionalDeserializer::deserialize(
                            "MaxStorageSize",
                            stack,
                        )?);
                    }
                    "MinIopsPerDbInstance" => {
                        obj.min_iops_per_db_instance =
                            Some(IntegerOptionalDeserializer::deserialize(
                                "MinIopsPerDbInstance",
                                stack,
                            )?);
                    }
                    "MinIopsPerGib" => {
                        obj.min_iops_per_gib = Some(DoubleOptionalDeserializer::deserialize(
                            "MinIopsPerGib",
                            stack,
                        )?);
                    }
                    "MinStorageSize" => {
                        obj.min_storage_size = Some(IntegerOptionalDeserializer::deserialize(
                            "MinStorageSize",
                            stack,
                        )?);
                    }
                    "MultiAZCapable" => {
                        obj.multi_az_capable =
                            Some(BooleanDeserializer::deserialize("MultiAZCapable", stack)?);
                    }
                    "ReadReplicaCapable" => {
                        obj.read_replica_capable = Some(BooleanDeserializer::deserialize(
                            "ReadReplicaCapable",
                            stack,
                        )?);
                    }
                    "StorageType" => {
                        obj.storage_type =
                            Some(StringDeserializer::deserialize("StorageType", stack)?);
                    }
                    "SupportsEnhancedMonitoring" => {
                        obj.supports_enhanced_monitoring = Some(BooleanDeserializer::deserialize(
                            "SupportsEnhancedMonitoring",
                            stack,
                        )?);
                    }
                    "SupportsIAMDatabaseAuthentication" => {
                        obj.supports_iam_database_authentication =
                            Some(BooleanDeserializer::deserialize(
                                "SupportsIAMDatabaseAuthentication",
                                stack,
                            )?);
                    }
                    "SupportsIops" => {
                        obj.supports_iops =
                            Some(BooleanDeserializer::deserialize("SupportsIops", stack)?);
                    }
                    "SupportsPerformanceInsights" => {
                        obj.supports_performance_insights = Some(BooleanDeserializer::deserialize(
                            "SupportsPerformanceInsights",
                            stack,
                        )?);
                    }
                    "SupportsStorageEncryption" => {
                        obj.supports_storage_encryption = Some(BooleanDeserializer::deserialize(
                            "SupportsStorageEncryption",
                            stack,
                        )?);
                    }
                    "Vpc" => {
                        obj.vpc = Some(BooleanDeserializer::deserialize("Vpc", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct OrderableDBInstanceOptionsListDeserializer;
impl OrderableDBInstanceOptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OrderableDBInstanceOption>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OrderableDBInstanceOption" {
                obj.push(OrderableDBInstanceOptionDeserializer::deserialize(
                    "OrderableDBInstanceOption",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OrderableDBInstanceOptionsMessage {
    /// <p> An optional pagination token provided by a previous OrderableDBInstanceOptions request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
    pub marker: Option<String>,
    /// <p>An <a>OrderableDBInstanceOption</a> structure containing information about orderable options for the DB instance.</p>
    pub orderable_db_instance_options: Option<Vec<OrderableDBInstanceOption>>,
}

struct OrderableDBInstanceOptionsMessageDeserializer;
impl OrderableDBInstanceOptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OrderableDBInstanceOptionsMessage, XmlParseError> {
        deserialize_elements::<_, OrderableDBInstanceOptionsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "OrderableDBInstanceOptions" => {
                        obj.orderable_db_instance_options
                            .get_or_insert(vec![])
                            .extend(OrderableDBInstanceOptionsListDeserializer::deserialize(
                                "OrderableDBInstanceOptions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Specifies a parameter.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Parameter {
    /// <p>Specifies the valid range of values for the parameter.</p>
    pub allowed_values: Option<String>,
    /// <p>Indicates when to apply parameter updates.</p>
    pub apply_method: Option<String>,
    /// <p>Specifies the engine specific parameters type.</p>
    pub apply_type: Option<String>,
    /// <p>Specifies the valid data type for the parameter.</p>
    pub data_type: Option<String>,
    /// <p>Provides a description of the parameter.</p>
    pub description: Option<String>,
    /// <p> Indicates whether (<code>true</code>) or not (<code>false</code>) the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed.</p>
    pub is_modifiable: Option<bool>,
    /// <p>The earliest engine version to which the parameter can apply.</p>
    pub minimum_engine_version: Option<String>,
    /// <p>Specifies the name of the parameter.</p>
    pub parameter_name: Option<String>,
    /// <p>Specifies the value of the parameter.</p>
    pub parameter_value: Option<String>,
    /// <p>Indicates the source of the parameter value.</p>
    pub source: Option<String>,
}

struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Parameter, XmlParseError> {
        deserialize_elements::<_, Parameter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllowedValues" => {
                    obj.allowed_values =
                        Some(StringDeserializer::deserialize("AllowedValues", stack)?);
                }
                "ApplyMethod" => {
                    obj.apply_method =
                        Some(ApplyMethodDeserializer::deserialize("ApplyMethod", stack)?);
                }
                "ApplyType" => {
                    obj.apply_type = Some(StringDeserializer::deserialize("ApplyType", stack)?);
                }
                "DataType" => {
                    obj.data_type = Some(StringDeserializer::deserialize("DataType", stack)?);
                }
                "Description" => {
                    obj.description = Some(StringDeserializer::deserialize("Description", stack)?);
                }
                "IsModifiable" => {
                    obj.is_modifiable =
                        Some(BooleanDeserializer::deserialize("IsModifiable", stack)?);
                }
                "MinimumEngineVersion" => {
                    obj.minimum_engine_version = Some(StringDeserializer::deserialize(
                        "MinimumEngineVersion",
                        stack,
                    )?);
                }
                "ParameterName" => {
                    obj.parameter_name =
                        Some(StringDeserializer::deserialize("ParameterName", stack)?);
                }
                "ParameterValue" => {
                    obj.parameter_value =
                        Some(StringDeserializer::deserialize("ParameterValue", stack)?);
                }
                "Source" => {
                    obj.source = Some(StringDeserializer::deserialize("Source", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Parameter` contents to a `SignedRequest`.
struct ParameterSerializer;
impl ParameterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Parameter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allowed_values {
            params.put(&format!("{}{}", prefix, "AllowedValues"), &field_value);
        }
        if let Some(ref field_value) = obj.apply_method {
            params.put(&format!("{}{}", prefix, "ApplyMethod"), &field_value);
        }
        if let Some(ref field_value) = obj.apply_type {
            params.put(&format!("{}{}", prefix, "ApplyType"), &field_value);
        }
        if let Some(ref field_value) = obj.data_type {
            params.put(&format!("{}{}", prefix, "DataType"), &field_value);
        }
        if let Some(ref field_value) = obj.description {
            params.put(&format!("{}{}", prefix, "Description"), &field_value);
        }
        if let Some(ref field_value) = obj.is_modifiable {
            params.put(&format!("{}{}", prefix, "IsModifiable"), &field_value);
        }
        if let Some(ref field_value) = obj.minimum_engine_version {
            params.put(
                &format!("{}{}", prefix, "MinimumEngineVersion"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.parameter_name {
            params.put(&format!("{}{}", prefix, "ParameterName"), &field_value);
        }
        if let Some(ref field_value) = obj.parameter_value {
            params.put(&format!("{}{}", prefix, "ParameterValue"), &field_value);
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }
    }
}

struct ParametersListDeserializer;
impl ParametersListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Parameter>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Parameter" {
                obj.push(ParameterDeserializer::deserialize("Parameter", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `ParametersList` contents to a `SignedRequest`.
struct ParametersListSerializer;
impl ParametersListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Parameter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ParameterSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>A list of the log types whose configuration is still pending. In other words, these log types are in the process of being activated or deactivated.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingCloudwatchLogsExports {
    /// <p>Log types that are in the process of being enabled. After they are enabled, these log types are exported to CloudWatch Logs.</p>
    pub log_types_to_disable: Option<Vec<String>>,
    /// <p>Log types that are in the process of being deactivated. After they are deactivated, these log types aren't exported to CloudWatch Logs.</p>
    pub log_types_to_enable: Option<Vec<String>>,
}

struct PendingCloudwatchLogsExportsDeserializer;
impl PendingCloudwatchLogsExportsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingCloudwatchLogsExports, XmlParseError> {
        deserialize_elements::<_, PendingCloudwatchLogsExports, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LogTypesToDisable" => {
                        obj.log_types_to_disable.get_or_insert(vec![]).extend(
                            LogTypeListDeserializer::deserialize("LogTypesToDisable", stack)?,
                        );
                    }
                    "LogTypesToEnable" => {
                        obj.log_types_to_enable.get_or_insert(vec![]).extend(
                            LogTypeListDeserializer::deserialize("LogTypesToEnable", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Provides information about a pending maintenance action for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingMaintenanceAction {
    /// <p>The type of pending maintenance action that is available for the resource.</p>
    pub action: Option<String>,
    /// <p>The date of the maintenance window when the action is applied. The maintenance action is applied to the resource during its first maintenance window after this date. If this date is specified, any <code>next-maintenance</code> opt-in requests are ignored.</p>
    pub auto_applied_after_date: Option<String>,
    /// <p>The effective date when the pending maintenance action is applied to the resource. This date takes into account opt-in requests received from the <a>ApplyPendingMaintenanceAction</a> API, the <code>AutoAppliedAfterDate</code>, and the <code>ForcedApplyDate</code>. This value is blank if an opt-in request has not been received and nothing has been specified as <code>AutoAppliedAfterDate</code> or <code>ForcedApplyDate</code>.</p>
    pub current_apply_date: Option<String>,
    /// <p>A description providing more detail about the maintenance action.</p>
    pub description: Option<String>,
    /// <p>The date when the maintenance action is automatically applied. The maintenance action is applied to the resource on this date regardless of the maintenance window for the resource. If this date is specified, any <code>immediate</code> opt-in requests are ignored.</p>
    pub forced_apply_date: Option<String>,
    /// <p>Indicates the type of opt-in request that has been received for the resource.</p>
    pub opt_in_status: Option<String>,
}

struct PendingMaintenanceActionDeserializer;
impl PendingMaintenanceActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingMaintenanceAction, XmlParseError> {
        deserialize_elements::<_, PendingMaintenanceAction, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Action" => {
                        obj.action = Some(StringDeserializer::deserialize("Action", stack)?);
                    }
                    "AutoAppliedAfterDate" => {
                        obj.auto_applied_after_date = Some(TStampDeserializer::deserialize(
                            "AutoAppliedAfterDate",
                            stack,
                        )?);
                    }
                    "CurrentApplyDate" => {
                        obj.current_apply_date =
                            Some(TStampDeserializer::deserialize("CurrentApplyDate", stack)?);
                    }
                    "Description" => {
                        obj.description =
                            Some(StringDeserializer::deserialize("Description", stack)?);
                    }
                    "ForcedApplyDate" => {
                        obj.forced_apply_date =
                            Some(TStampDeserializer::deserialize("ForcedApplyDate", stack)?);
                    }
                    "OptInStatus" => {
                        obj.opt_in_status =
                            Some(StringDeserializer::deserialize("OptInStatus", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct PendingMaintenanceActionDetailsDeserializer;
impl PendingMaintenanceActionDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PendingMaintenanceAction>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "PendingMaintenanceAction" {
                obj.push(PendingMaintenanceActionDeserializer::deserialize(
                    "PendingMaintenanceAction",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct PendingMaintenanceActionsDeserializer;
impl PendingMaintenanceActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourcePendingMaintenanceActions>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ResourcePendingMaintenanceActions" {
                obj.push(ResourcePendingMaintenanceActionsDeserializer::deserialize(
                    "ResourcePendingMaintenanceActions",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingMaintenanceActionsMessage {
    /// <p> An optional pagination token provided by a previous <code>DescribePendingMaintenanceActions</code> request. If this parameter is specified, the response includes only records beyond the marker, up to a number of records specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>A list of the pending maintenance actions for the resource.</p>
    pub pending_maintenance_actions: Option<Vec<ResourcePendingMaintenanceActions>>,
}

struct PendingMaintenanceActionsMessageDeserializer;
impl PendingMaintenanceActionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingMaintenanceActionsMessage, XmlParseError> {
        deserialize_elements::<_, PendingMaintenanceActionsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "PendingMaintenanceActions" => {
                        obj.pending_maintenance_actions
                            .get_or_insert(vec![])
                            .extend(PendingMaintenanceActionsDeserializer::deserialize(
                                "PendingMaintenanceActions",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p> This data type is used as a response element in the <a>ModifyDBInstance</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingModifiedValues {
    /// <p> Contains the new <code>AllocatedStorage</code> size for the DB instance that will be applied or is currently being applied.</p>
    pub allocated_storage: Option<i64>,
    /// <p>Specifies the pending number of days for which automated backups are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>Specifies the identifier of the CA certificate for the DB instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p> Contains the new <code>DBInstanceClass</code> for the DB instance that will be applied or is currently being applied.</p>
    pub db_instance_class: Option<String>,
    /// <p> Contains the new <code>DBInstanceIdentifier</code> for the DB instance that will be applied or is currently being applied.</p>
    pub db_instance_identifier: Option<String>,
    /// <p>The new DB subnet group for the DB instance.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Specifies the new Provisioned IOPS value for the DB instance that will be applied or is currently being applied.</p>
    pub iops: Option<i64>,
    /// <p>The license model for the DB instance.</p> <p>Valid values: <code>license-included</code> | <code>bring-your-own-license</code> | <code>general-public-license</code> </p>
    pub license_model: Option<String>,
    /// <p>Contains the pending or currently-in-progress change of the master credentials for the DB instance.</p>
    pub master_user_password: Option<String>,
    /// <p>Indicates that the Single-AZ DB instance is to change to a Multi-AZ deployment.</p>
    pub multi_az: Option<bool>,
    /// <p>This <code>PendingCloudwatchLogsExports</code> structure specifies pending changes to which CloudWatch logs are enabled and which are disabled.</p>
    pub pending_cloudwatch_logs_exports: Option<PendingCloudwatchLogsExports>,
    /// <p>Specifies the pending port for the DB instance.</p>
    pub port: Option<i64>,
    /// <p>Specifies the storage type to be associated with the DB instance.</p>
    pub storage_type: Option<String>,
}

struct PendingModifiedValuesDeserializer;
impl PendingModifiedValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingModifiedValues, XmlParseError> {
        deserialize_elements::<_, PendingModifiedValues, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllocatedStorage" => {
                    obj.allocated_storage = Some(IntegerOptionalDeserializer::deserialize(
                        "AllocatedStorage",
                        stack,
                    )?);
                }
                "BackupRetentionPeriod" => {
                    obj.backup_retention_period = Some(IntegerOptionalDeserializer::deserialize(
                        "BackupRetentionPeriod",
                        stack,
                    )?);
                }
                "CACertificateIdentifier" => {
                    obj.ca_certificate_identifier = Some(StringDeserializer::deserialize(
                        "CACertificateIdentifier",
                        stack,
                    )?);
                }
                "DBInstanceClass" => {
                    obj.db_instance_class =
                        Some(StringDeserializer::deserialize("DBInstanceClass", stack)?);
                }
                "DBInstanceIdentifier" => {
                    obj.db_instance_identifier = Some(StringDeserializer::deserialize(
                        "DBInstanceIdentifier",
                        stack,
                    )?);
                }
                "DBSubnetGroupName" => {
                    obj.db_subnet_group_name =
                        Some(StringDeserializer::deserialize("DBSubnetGroupName", stack)?);
                }
                "EngineVersion" => {
                    obj.engine_version =
                        Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                }
                "Iops" => {
                    obj.iops = Some(IntegerOptionalDeserializer::deserialize("Iops", stack)?);
                }
                "LicenseModel" => {
                    obj.license_model =
                        Some(StringDeserializer::deserialize("LicenseModel", stack)?);
                }
                "MasterUserPassword" => {
                    obj.master_user_password = Some(StringDeserializer::deserialize(
                        "MasterUserPassword",
                        stack,
                    )?);
                }
                "MultiAZ" => {
                    obj.multi_az =
                        Some(BooleanOptionalDeserializer::deserialize("MultiAZ", stack)?);
                }
                "PendingCloudwatchLogsExports" => {
                    obj.pending_cloudwatch_logs_exports =
                        Some(PendingCloudwatchLogsExportsDeserializer::deserialize(
                            "PendingCloudwatchLogsExports",
                            stack,
                        )?);
                }
                "Port" => {
                    obj.port = Some(IntegerOptionalDeserializer::deserialize("Port", stack)?);
                }
                "StorageType" => {
                    obj.storage_type = Some(StringDeserializer::deserialize("StorageType", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PromoteReadReplicaDBClusterMessage {
    /// <p>Not supported.</p>
    pub db_cluster_identifier: String,
}

/// Serialize `PromoteReadReplicaDBClusterMessage` contents to a `SignedRequest`.
struct PromoteReadReplicaDBClusterMessageSerializer;
impl PromoteReadReplicaDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PromoteReadReplicaDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PromoteReadReplicaDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct PromoteReadReplicaDBClusterResultDeserializer;
impl PromoteReadReplicaDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PromoteReadReplicaDBClusterResult, XmlParseError> {
        deserialize_elements::<_, PromoteReadReplicaDBClusterResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(DBClusterDeserializer::deserialize("DBCluster", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A range of integer values.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Range {
    /// <p>The minimum value in the range.</p>
    pub from: Option<i64>,
    /// <p>The step value for the range. For example, if you have a range of 5,000 to 10,000, with a step value of 1,000, the valid values start at 5,000 and step up by 1,000. Even though 7,500 is within the range, it isn't a valid value for the range. The valid values are 5,000, 6,000, 7,000, 8,000...</p>
    pub step: Option<i64>,
    /// <p>The maximum value in the range.</p>
    pub to: Option<i64>,
}

struct RangeDeserializer;
impl RangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Range, XmlParseError> {
        deserialize_elements::<_, Range, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "From" => {
                    obj.from = Some(IntegerDeserializer::deserialize("From", stack)?);
                }
                "Step" => {
                    obj.step = Some(IntegerOptionalDeserializer::deserialize("Step", stack)?);
                }
                "To" => {
                    obj.to = Some(IntegerDeserializer::deserialize("To", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct RangeListDeserializer;
impl RangeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Range>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Range" {
                obj.push(RangeDeserializer::deserialize("Range", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ReadReplicaDBClusterIdentifierListDeserializer;
impl ReadReplicaDBClusterIdentifierListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReadReplicaDBClusterIdentifier" {
                obj.push(StringDeserializer::deserialize(
                    "ReadReplicaDBClusterIdentifier",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ReadReplicaDBInstanceIdentifierListDeserializer;
impl ReadReplicaDBInstanceIdentifierListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReadReplicaDBInstanceIdentifier" {
                obj.push(StringDeserializer::deserialize(
                    "ReadReplicaDBInstanceIdentifier",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ReadReplicaIdentifierListDeserializer;
impl ReadReplicaIdentifierListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReadReplicaIdentifier" {
                obj.push(StringDeserializer::deserialize(
                    "ReadReplicaIdentifier",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootDBInstanceMessage {
    /// <p><p>The DB instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBInstance.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p> When <code>true</code>, the reboot is conducted through a MultiAZ failover.</p> <p>Constraint: You can't specify <code>true</code> if the instance is not configured for MultiAZ.</p>
    pub force_failover: Option<bool>,
}

/// Serialize `RebootDBInstanceMessage` contents to a `SignedRequest`.
struct RebootDBInstanceMessageSerializer;
impl RebootDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RebootDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.force_failover {
            params.put(&format!("{}{}", prefix, "ForceFailover"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RebootDBInstanceResult {
    pub db_instance: Option<DBInstance>,
}

struct RebootDBInstanceResultDeserializer;
impl RebootDBInstanceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RebootDBInstanceResult, XmlParseError> {
        deserialize_elements::<_, RebootDBInstanceResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DBInstance" => {
                    obj.db_instance =
                        Some(DBInstanceDeserializer::deserialize("DBInstance", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveRoleFromDBClusterMessage {
    /// <p>The name of the DB cluster to disassociate the IAM role from.</p>
    pub db_cluster_identifier: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to disassociate from the DB cluster, for example <code>arn:aws:iam::123456789012:role/NeptuneAccessRole</code>.</p>
    pub role_arn: String,
}

/// Serialize `RemoveRoleFromDBClusterMessage` contents to a `SignedRequest`.
struct RemoveRoleFromDBClusterMessageSerializer;
impl RemoveRoleFromDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveRoleFromDBClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        params.put(&format!("{}{}", prefix, "RoleArn"), &obj.role_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveSourceIdentifierFromSubscriptionMessage {
    /// <p> The source identifier to be removed from the subscription, such as the <b>DB instance identifier</b> for a DB instance or the name of a security group.</p>
    pub source_identifier: String,
    /// <p>The name of the event notification subscription you want to remove a source identifier from.</p>
    pub subscription_name: String,
}

/// Serialize `RemoveSourceIdentifierFromSubscriptionMessage` contents to a `SignedRequest`.
struct RemoveSourceIdentifierFromSubscriptionMessageSerializer;
impl RemoveSourceIdentifierFromSubscriptionMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &RemoveSourceIdentifierFromSubscriptionMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceIdentifier"),
            &obj.source_identifier,
        );
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RemoveSourceIdentifierFromSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct RemoveSourceIdentifierFromSubscriptionResultDeserializer;
impl RemoveSourceIdentifierFromSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveSourceIdentifierFromSubscriptionResult, XmlParseError> {
        deserialize_elements::<_, RemoveSourceIdentifierFromSubscriptionResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EventSubscription" => {
                        obj.event_subscription = Some(EventSubscriptionDeserializer::deserialize(
                            "EventSubscription",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromResourceMessage {
    /// <p>The Amazon Neptune resource that the tags are removed from. This value is an Amazon Resource Name (ARN). For information about creating an ARN, see <a href="https://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>
    pub resource_name: String,
    /// <p>The tag key (name) of the tag to be removed.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `RemoveTagsFromResourceMessage` contents to a `SignedRequest`.
struct RemoveTagsFromResourceMessageSerializer;
impl RemoveTagsFromResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveTagsFromResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceName"), &obj.resource_name);
        KeyListSerializer::serialize(params, &format!("{}{}", prefix, "TagKeys"), &obj.tag_keys);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetDBClusterParameterGroupMessage {
    /// <p>The name of the DB cluster parameter group to reset.</p>
    pub db_cluster_parameter_group_name: String,
    /// <p>A list of parameter names in the DB cluster parameter group to reset to the default values. You can't use this parameter if the <code>ResetAllParameters</code> parameter is set to <code>true</code>.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>A value that is set to <code>true</code> to reset all parameters in the DB cluster parameter group to their default values, and <code>false</code> otherwise. You can't use this parameter if there is a list of parameter names specified for the <code>Parameters</code> parameter.</p>
    pub reset_all_parameters: Option<bool>,
}

/// Serialize `ResetDBClusterParameterGroupMessage` contents to a `SignedRequest`.
struct ResetDBClusterParameterGroupMessageSerializer;
impl ResetDBClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ResetDBClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterParameterGroupName"),
            &obj.db_cluster_parameter_group_name,
        );
        if let Some(ref field_value) = obj.parameters {
            ParametersListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.reset_all_parameters {
            params.put(&format!("{}{}", prefix, "ResetAllParameters"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetDBParameterGroupMessage {
    /// <p><p>The name of the DB parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing DBParameterGroup.</p> </li> </ul></p>
    pub db_parameter_group_name: String,
    /// <p>To reset the entire DB parameter group, specify the <code>DBParameterGroup</code> name and <code>ResetAllParameters</code> parameters. To reset specific parameters, provide a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request.</p> <p>Valid Values (for Apply method): <code>pending-reboot</code> </p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>Specifies whether (<code>true</code>) or not (<code>false</code>) to reset all parameters in the DB parameter group to default values.</p> <p>Default: <code>true</code> </p>
    pub reset_all_parameters: Option<bool>,
}

/// Serialize `ResetDBParameterGroupMessage` contents to a `SignedRequest`.
struct ResetDBParameterGroupMessageSerializer;
impl ResetDBParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ResetDBParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBParameterGroupName"),
            &obj.db_parameter_group_name,
        );
        if let Some(ref field_value) = obj.parameters {
            ParametersListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.reset_all_parameters {
            params.put(&format!("{}{}", prefix, "ResetAllParameters"), &field_value);
        }
    }
}

/// <p>Describes the pending maintenance actions for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ResourcePendingMaintenanceActions {
    /// <p>A list that provides details about the pending maintenance actions for the resource.</p>
    pub pending_maintenance_action_details: Option<Vec<PendingMaintenanceAction>>,
    /// <p>The ARN of the resource that has pending maintenance actions.</p>
    pub resource_identifier: Option<String>,
}

struct ResourcePendingMaintenanceActionsDeserializer;
impl ResourcePendingMaintenanceActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourcePendingMaintenanceActions, XmlParseError> {
        deserialize_elements::<_, ResourcePendingMaintenanceActions, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PendingMaintenanceActionDetails" => {
                        obj.pending_maintenance_action_details
                            .get_or_insert(vec![])
                            .extend(PendingMaintenanceActionDetailsDeserializer::deserialize(
                                "PendingMaintenanceActionDetails",
                                stack,
                            )?);
                    }
                    "ResourceIdentifier" => {
                        obj.resource_identifier = Some(StringDeserializer::deserialize(
                            "ResourceIdentifier",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreDBClusterFromSnapshotMessage {
    /// <p>Provides the list of EC2 Availability Zones that instances in the restored DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The name of the DB cluster to create from the DB snapshot or DB cluster snapshot. This parameter isn't case-sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-snapshot-id</code> </p>
    pub db_cluster_identifier: String,
    /// <p><p>The name of the DB cluster parameter group to associate with the new DB cluster.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>The name of the DB subnet group to use for the new DB cluster.</p> <p>Constraints: If supplied, must match the name of an existing DBSubnetGroup.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Not supported.</p>
    pub database_name: Option<String>,
    /// <p>A value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. </p>
    pub deletion_protection: Option<bool>,
    /// <p>The list of logs that the restored DB cluster is to export to Amazon CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The database engine to use for the new DB cluster.</p> <p>Default: The same as source</p> <p>Constraint: Must be compatible with the engine of the source</p>
    pub engine: String,
    /// <p>The version of the database engine to use for the new DB cluster.</p>
    pub engine_version: Option<String>,
    /// <p><p>The AWS KMS key identifier to use when restoring an encrypted DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are restoring a DB cluster with the same AWS account that owns the KMS encryption key used to encrypt the new DB cluster, then you can use the KMS key alias instead of the ARN for the KMS encryption key.</p> <p>If you do not specify a value for the <code>KmsKeyId</code> parameter, then the following will occur:</p> <ul> <li> <p>If the DB snapshot or DB cluster snapshot in <code>SnapshotIdentifier</code> is encrypted, then the restored DB cluster is encrypted using the KMS key that was used to encrypt the DB snapshot or DB cluster snapshot.</p> </li> <li> <p>If the DB snapshot or DB cluster snapshot in <code>SnapshotIdentifier</code> is not encrypted, then the restored DB cluster is not encrypted.</p> </li> </ul></p>
    pub kms_key_id: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the new DB cluster accepts connections.</p> <p>Constraints: Value must be <code>1150-65535</code> </p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p><p>The identifier for the DB snapshot or DB cluster snapshot to restore from.</p> <p>You can use either the name or the Amazon Resource Name (ARN) to specify a DB cluster snapshot. However, you can use only the ARN to specify a DB snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing Snapshot.</p> </li> </ul></p>
    pub snapshot_identifier: String,
    /// <p>The tags to be assigned to the restored DB cluster.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of VPC security groups that the new DB cluster will belong to.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `RestoreDBClusterFromSnapshotMessage` contents to a `SignedRequest`.
struct RestoreDBClusterFromSnapshotMessageSerializer;
impl RestoreDBClusterFromSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RestoreDBClusterFromSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZone"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.database_name {
            params.put(&format!("{}{}", prefix, "DatabaseName"), &field_value);
        }
        if let Some(ref field_value) = obj.deletion_protection {
            params.put(&format!("{}{}", prefix, "DeletionProtection"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_cloudwatch_logs_exports {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnableCloudwatchLogsExports"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RestoreDBClusterFromSnapshotResult {
    pub db_cluster: Option<DBCluster>,
}

struct RestoreDBClusterFromSnapshotResultDeserializer;
impl RestoreDBClusterFromSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreDBClusterFromSnapshotResult, XmlParseError> {
        deserialize_elements::<_, RestoreDBClusterFromSnapshotResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(DBClusterDeserializer::deserialize("DBCluster", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreDBClusterToPointInTimeMessage {
    /// <p><p>The name of the new DB cluster to be created.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p><p>The name of the DB cluster parameter group to associate with the new DB cluster.</p> <p>Constraints:</p> <ul> <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>The DB subnet group name to use for the new DB cluster.</p> <p>Constraints: If supplied, must match the name of an existing DBSubnetGroup.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>A value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled. </p>
    pub deletion_protection: Option<bool>,
    /// <p>The list of logs that the restored DB cluster is to export to CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>True to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts, and otherwise false.</p> <p>Default: <code>false</code> </p>
    pub enable_iam_database_authentication: Option<bool>,
    /// <p>The AWS KMS key identifier to use when restoring an encrypted DB cluster from an encrypted DB cluster.</p> <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are restoring a DB cluster with the same AWS account that owns the KMS encryption key used to encrypt the new DB cluster, then you can use the KMS key alias instead of the ARN for the KMS encryption key.</p> <p>You can restore to a new DB cluster and encrypt the new DB cluster with a KMS key that is different than the KMS key used to encrypt the source DB cluster. The new DB cluster is encrypted with the KMS key identified by the <code>KmsKeyId</code> parameter.</p> <p>If you do not specify a value for the <code>KmsKeyId</code> parameter, then the following will occur:</p> <ul> <li> <p>If the DB cluster is encrypted, then the restored DB cluster is encrypted using the KMS key that was used to encrypt the source DB cluster.</p> </li> <li> <p>If the DB cluster is not encrypted, then the restored DB cluster is not encrypted.</p> </li> </ul> <p>If <code>DBClusterIdentifier</code> refers to a DB cluster that is not encrypted, then the restore request is rejected.</p>
    pub kms_key_id: Option<String>,
    /// <p> <i>(Not supported by Neptune)</i> </p>
    pub option_group_name: Option<String>,
    /// <p>The port number on which the new DB cluster accepts connections.</p> <p>Constraints: Value must be <code>1150-65535</code> </p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p>The date and time to restore the DB cluster to.</p> <p>Valid Values: Value must be a time in Universal Coordinated Time (UTC) format</p> <p>Constraints:</p> <ul> <li> <p>Must be before the latest restorable time for the DB instance</p> </li> <li> <p>Must be specified if <code>UseLatestRestorableTime</code> parameter is not provided</p> </li> <li> <p>Cannot be specified if <code>UseLatestRestorableTime</code> parameter is true</p> </li> <li> <p>Cannot be specified if <code>RestoreType</code> parameter is <code>copy-on-write</code> </p> </li> </ul> <p>Example: <code>2015-03-07T23:45:00Z</code> </p>
    pub restore_to_time: Option<String>,
    /// <p>The type of restore to be performed. You can specify one of the following values:</p> <ul> <li> <p> <code>full-copy</code> - The new DB cluster is restored as a full copy of the source DB cluster.</p> </li> <li> <p> <code>copy-on-write</code> - The new DB cluster is restored as a clone of the source DB cluster.</p> </li> </ul> <p>If you don't specify a <code>RestoreType</code> value, then the new DB cluster is restored as a full copy of the source DB cluster.</p>
    pub restore_type: Option<String>,
    /// <p><p>The identifier of the source DB cluster from which to restore.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing DBCluster.</p> </li> </ul></p>
    pub source_db_cluster_identifier: String,
    /// <p>The tags to be applied to the restored DB cluster.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A value that is set to <code>true</code> to restore the DB cluster to the latest restorable backup time, and <code>false</code> otherwise.</p> <p>Default: <code>false</code> </p> <p>Constraints: Cannot be specified if <code>RestoreToTime</code> parameter is provided.</p>
    pub use_latest_restorable_time: Option<bool>,
    /// <p>A list of VPC security groups that the new DB cluster belongs to.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `RestoreDBClusterToPointInTimeMessage` contents to a `SignedRequest`.
struct RestoreDBClusterToPointInTimeMessageSerializer;
impl RestoreDBClusterToPointInTimeMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RestoreDBClusterToPointInTimeMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.db_cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "DBClusterParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_subnet_group_name {
            params.put(&format!("{}{}", prefix, "DBSubnetGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.deletion_protection {
            params.put(&format!("{}{}", prefix, "DeletionProtection"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_cloudwatch_logs_exports {
            LogTypeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnableCloudwatchLogsExports"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enable_iam_database_authentication {
            params.put(
                &format!("{}{}", prefix, "EnableIAMDatabaseAuthentication"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.option_group_name {
            params.put(&format!("{}{}", prefix, "OptionGroupName"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.restore_to_time {
            params.put(&format!("{}{}", prefix, "RestoreToTime"), &field_value);
        }
        if let Some(ref field_value) = obj.restore_type {
            params.put(&format!("{}{}", prefix, "RestoreType"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SourceDBClusterIdentifier"),
            &obj.source_db_cluster_identifier,
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.use_latest_restorable_time {
            params.put(
                &format!("{}{}", prefix, "UseLatestRestorableTime"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_security_group_ids {
            VpcSecurityGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VpcSecurityGroupId"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RestoreDBClusterToPointInTimeResult {
    pub db_cluster: Option<DBCluster>,
}

struct RestoreDBClusterToPointInTimeResultDeserializer;
impl RestoreDBClusterToPointInTimeResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreDBClusterToPointInTimeResult, XmlParseError> {
        deserialize_elements::<_, RestoreDBClusterToPointInTimeResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DBCluster" => {
                        obj.db_cluster =
                            Some(DBClusterDeserializer::deserialize("DBCluster", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct SourceIdsListDeserializer;
impl SourceIdsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "SourceId" {
                obj.push(StringDeserializer::deserialize("SourceId", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `SourceIdsList` contents to a `SignedRequest`.
struct SourceIdsListSerializer;
impl SourceIdsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct SourceTypeDeserializer;
impl SourceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Specifies a subnet.</p> <p> This data type is used as a response element in the <a>DescribeDBSubnetGroups</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Subnet {
    /// <p>Specifies the EC2 Availability Zone that the subnet is in.</p>
    pub subnet_availability_zone: Option<AvailabilityZone>,
    /// <p>Specifies the identifier of the subnet.</p>
    pub subnet_identifier: Option<String>,
    /// <p>Specifies the status of the subnet.</p>
    pub subnet_status: Option<String>,
}

struct SubnetDeserializer;
impl SubnetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Subnet, XmlParseError> {
        deserialize_elements::<_, Subnet, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "SubnetAvailabilityZone" => {
                    obj.subnet_availability_zone = Some(AvailabilityZoneDeserializer::deserialize(
                        "SubnetAvailabilityZone",
                        stack,
                    )?);
                }
                "SubnetIdentifier" => {
                    obj.subnet_identifier =
                        Some(StringDeserializer::deserialize("SubnetIdentifier", stack)?);
                }
                "SubnetStatus" => {
                    obj.subnet_status =
                        Some(StringDeserializer::deserialize("SubnetStatus", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `SubnetIdentifierList` contents to a `SignedRequest`.
struct SubnetIdentifierListSerializer;
impl SubnetIdentifierListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct SubnetListDeserializer;
impl SubnetListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Subnet>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Subnet" {
                obj.push(SubnetDeserializer::deserialize("Subnet", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct SupportedCharacterSetsListDeserializer;
impl SupportedCharacterSetsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CharacterSet>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CharacterSet" {
                obj.push(CharacterSetDeserializer::deserialize(
                    "CharacterSet",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct SupportedTimezonesListDeserializer;
impl SupportedTimezonesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Timezone>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Timezone" {
                obj.push(TimezoneDeserializer::deserialize("Timezone", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TStampDeserializer;
impl TStampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Metadata assigned to an Amazon Neptune resource consisting of a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>A key is the required name of the tag. The string value can be from 1 to 128 Unicode characters in length and can't be prefixed with "aws:" or "rds:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    pub key: Option<String>,
    /// <p>A value is the optional value of the tag. The string value can be from 1 to 256 Unicode characters in length and can't be prefixed with "aws:" or "rds:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = Some(StringDeserializer::deserialize("Key", stack)?);
                }
                "Value" => {
                    obj.value = Some(StringDeserializer::deserialize("Value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Tag" {
                obj.push(TagDeserializer::deserialize("Tag", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `TagList` contents to a `SignedRequest`.
struct TagListSerializer;
impl TagListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagListMessage {
    /// <p>List of tags returned by the ListTagsForResource operation.</p>
    pub tag_list: Option<Vec<Tag>>,
}

struct TagListMessageDeserializer;
impl TagListMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagListMessage, XmlParseError> {
        deserialize_elements::<_, TagListMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TagList" => {
                    obj.tag_list
                        .get_or_insert(vec![])
                        .extend(TagListDeserializer::deserialize("TagList", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A time zone associated with a <a>DBInstance</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Timezone {
    /// <p>The name of the time zone.</p>
    pub timezone_name: Option<String>,
}

struct TimezoneDeserializer;
impl TimezoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Timezone, XmlParseError> {
        deserialize_elements::<_, Timezone, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TimezoneName" => {
                    obj.timezone_name =
                        Some(StringDeserializer::deserialize("TimezoneName", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The version of the database engine that a DB instance can be upgraded to.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpgradeTarget {
    /// <p>A value that indicates whether the target version is applied to any source DB instances that have AutoMinorVersionUpgrade set to true.</p>
    pub auto_upgrade: Option<bool>,
    /// <p>The version of the database engine that a DB instance can be upgraded to.</p>
    pub description: Option<String>,
    /// <p>The name of the upgrade target database engine.</p>
    pub engine: Option<String>,
    /// <p>The version number of the upgrade target database engine.</p>
    pub engine_version: Option<String>,
    /// <p>A value that indicates whether a database engine is upgraded to a major version.</p>
    pub is_major_version_upgrade: Option<bool>,
}

struct UpgradeTargetDeserializer;
impl UpgradeTargetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpgradeTarget, XmlParseError> {
        deserialize_elements::<_, UpgradeTarget, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AutoUpgrade" => {
                    obj.auto_upgrade =
                        Some(BooleanDeserializer::deserialize("AutoUpgrade", stack)?);
                }
                "Description" => {
                    obj.description = Some(StringDeserializer::deserialize("Description", stack)?);
                }
                "Engine" => {
                    obj.engine = Some(StringDeserializer::deserialize("Engine", stack)?);
                }
                "EngineVersion" => {
                    obj.engine_version =
                        Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                }
                "IsMajorVersionUpgrade" => {
                    obj.is_major_version_upgrade = Some(BooleanDeserializer::deserialize(
                        "IsMajorVersionUpgrade",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Information about valid modifications that you can make to your DB instance. Contains the result of a successful call to the <a>DescribeValidDBInstanceModifications</a> action. You can use this information when you call <a>ModifyDBInstance</a>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ValidDBInstanceModificationsMessage {
    /// <p>Valid storage options for your DB instance.</p>
    pub storage: Option<Vec<ValidStorageOptions>>,
}

struct ValidDBInstanceModificationsMessageDeserializer;
impl ValidDBInstanceModificationsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ValidDBInstanceModificationsMessage, XmlParseError> {
        deserialize_elements::<_, ValidDBInstanceModificationsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Storage" => {
                        obj.storage.get_or_insert(vec![]).extend(
                            ValidStorageOptionsListDeserializer::deserialize("Storage", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Information about valid modifications that you can make to your DB instance.</p> <p>Contains the result of a successful call to the <a>DescribeValidDBInstanceModifications</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ValidStorageOptions {
    /// <p>The valid range of Provisioned IOPS to gibibytes of storage multiplier. For example, 3-10, which means that provisioned IOPS can be between 3 and 10 times storage.</p>
    pub iops_to_storage_ratio: Option<Vec<DoubleRange>>,
    /// <p>The valid range of provisioned IOPS. For example, 1000-20000.</p>
    pub provisioned_iops: Option<Vec<Range>>,
    /// <p>The valid range of storage in gibibytes. For example, 100 to 16384.</p>
    pub storage_size: Option<Vec<Range>>,
    /// <p>The valid storage types for your DB instance. For example, gp2, io1.</p>
    pub storage_type: Option<String>,
}

struct ValidStorageOptionsDeserializer;
impl ValidStorageOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ValidStorageOptions, XmlParseError> {
        deserialize_elements::<_, ValidStorageOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IopsToStorageRatio" => {
                    obj.iops_to_storage_ratio.get_or_insert(vec![]).extend(
                        DoubleRangeListDeserializer::deserialize("IopsToStorageRatio", stack)?,
                    );
                }
                "ProvisionedIops" => {
                    obj.provisioned_iops.get_or_insert(vec![]).extend(
                        RangeListDeserializer::deserialize("ProvisionedIops", stack)?,
                    );
                }
                "StorageSize" => {
                    obj.storage_size
                        .get_or_insert(vec![])
                        .extend(RangeListDeserializer::deserialize("StorageSize", stack)?);
                }
                "StorageType" => {
                    obj.storage_type = Some(StringDeserializer::deserialize("StorageType", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ValidStorageOptionsListDeserializer;
impl ValidStorageOptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ValidStorageOptions>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ValidStorageOptions" {
                obj.push(ValidStorageOptionsDeserializer::deserialize(
                    "ValidStorageOptions",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ValidUpgradeTargetListDeserializer;
impl ValidUpgradeTargetListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<UpgradeTarget>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "UpgradeTarget" {
                obj.push(UpgradeTargetDeserializer::deserialize(
                    "UpgradeTarget",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `VpcSecurityGroupIdList` contents to a `SignedRequest`.
struct VpcSecurityGroupIdListSerializer;
impl VpcSecurityGroupIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>This data type is used as a response element for queries on VPC security group membership.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct VpcSecurityGroupMembership {
    /// <p>The status of the VPC security group.</p>
    pub status: Option<String>,
    /// <p>The name of the VPC security group.</p>
    pub vpc_security_group_id: Option<String>,
}

struct VpcSecurityGroupMembershipDeserializer;
impl VpcSecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VpcSecurityGroupMembership, XmlParseError> {
        deserialize_elements::<_, VpcSecurityGroupMembership, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                    }
                    "VpcSecurityGroupId" => {
                        obj.vpc_security_group_id = Some(StringDeserializer::deserialize(
                            "VpcSecurityGroupId",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct VpcSecurityGroupMembershipListDeserializer;
impl VpcSecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<VpcSecurityGroupMembership>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "VpcSecurityGroupMembership" {
                obj.push(VpcSecurityGroupMembershipDeserializer::deserialize(
                    "VpcSecurityGroupMembership",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// Errors returned by AddRoleToDBCluster
#[derive(Debug, PartialEq)]
pub enum AddRoleToDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p>The specified IAM role Amazon Resource Name (ARN) is already associated with the specified DB cluster.</p>
    DBClusterRoleAlreadyExistsFault(String),
    /// <p>You have exceeded the maximum number of IAM roles that can be associated with the specified DB cluster.</p>
    DBClusterRoleQuotaExceededFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
}

impl AddRoleToDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddRoleToDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            AddRoleToDBClusterError::DBClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBClusterRoleAlreadyExists" => {
                        return RusotoError::Service(
                            AddRoleToDBClusterError::DBClusterRoleAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterRoleQuotaExceeded" => {
                        return RusotoError::Service(
                            AddRoleToDBClusterError::DBClusterRoleQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            AddRoleToDBClusterError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AddRoleToDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddRoleToDBClusterError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            AddRoleToDBClusterError::DBClusterRoleAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            AddRoleToDBClusterError::DBClusterRoleQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            AddRoleToDBClusterError::InvalidDBClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddRoleToDBClusterError {}
/// Errors returned by AddSourceIdentifierToSubscription
#[derive(Debug, PartialEq)]
pub enum AddSourceIdentifierToSubscriptionError {
    /// <p>The source could not be found.</p>
    SourceNotFoundFault(String),
    /// <p>The designated subscription could not be found.</p>
    SubscriptionNotFoundFault(String),
}

impl AddSourceIdentifierToSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddSourceIdentifierToSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "SourceNotFound" => {
                        return RusotoError::Service(
                            AddSourceIdentifierToSubscriptionError::SourceNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SubscriptionNotFound" => {
                        return RusotoError::Service(
                            AddSourceIdentifierToSubscriptionError::SubscriptionNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AddSourceIdentifierToSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddSourceIdentifierToSubscriptionError::SourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            AddSourceIdentifierToSubscriptionError::SubscriptionNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddSourceIdentifierToSubscriptionError {}
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot.</p>
    DBSnapshotNotFoundFault(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            AddTagsToResourceError::DBClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            AddTagsToResourceError::DBInstanceNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBSnapshotNotFound" => {
                        return RusotoError::Service(
                            AddTagsToResourceError::DBSnapshotNotFoundFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AddTagsToResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToResourceError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::DBInstanceNotFoundFault(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::DBSnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToResourceError {}
/// Errors returned by ApplyPendingMaintenanceAction
#[derive(Debug, PartialEq)]
pub enum ApplyPendingMaintenanceActionError {
    /// <p>The specified resource ID was not found.</p>
    ResourceNotFoundFault(String),
}

impl ApplyPendingMaintenanceActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ApplyPendingMaintenanceActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceNotFoundFault" => {
                        return RusotoError::Service(
                            ApplyPendingMaintenanceActionError::ResourceNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ApplyPendingMaintenanceActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplyPendingMaintenanceActionError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ApplyPendingMaintenanceActionError {}
/// Errors returned by CopyDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum CopyDBClusterParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
}

impl CopyDBClusterParameterGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CopyDBClusterParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CopyDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            CopyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBParameterGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CopyDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CopyDBClusterParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CopyDBClusterParameterGroupError {}
/// Errors returned by CopyDBClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum CopyDBClusterSnapshotError {
    /// <p>User already has a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot.</p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl CopyDBClusterSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyDBClusterSnapshotError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotAlreadyExistsFault" => {
                        return RusotoError::Service(
                            CopyDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterSnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            CopyDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return RusotoError::Service(
                            CopyDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            CopyDBClusterSnapshotError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return RusotoError::Service(
                            CopyDBClusterSnapshotError::KMSKeyNotAccessibleFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotQuotaExceeded" => {
                        return RusotoError::Service(
                            CopyDBClusterSnapshotError::SnapshotQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CopyDBClusterSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBClusterSnapshotError::InvalidDBClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBClusterSnapshotError::KMSKeyNotAccessibleFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBClusterSnapshotError::SnapshotQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CopyDBClusterSnapshotError {}
/// Errors returned by CopyDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum CopyDBParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
}

impl CopyDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyDBParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CopyDBParameterGroupError::DBParameterGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            CopyDBParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBParameterGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CopyDBParameterGroupError::DBParameterGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CopyDBParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyDBParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CopyDBParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CopyDBParameterGroupError {}
/// Errors returned by CreateDBCluster
#[derive(Debug, PartialEq)]
pub enum CreateDBClusterError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBClusterParameterGroupName</i> does not refer to an existing DB Cluster parameter group.</p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p>User attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>There is insufficient storage available for the current action. You may be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available.</p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The specified DB instance is not in the <i>available</i> state.</p>
    InvalidDBInstanceStateFault(String),
    /// <p>The DB subnet group cannot be deleted because it is in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
}

impl CreateDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterAlreadyExistsFault" => {
                        return RusotoError::Service(
                            CreateDBClusterError::DBClusterAlreadyExistsFault(parsed_error.message),
                        )
                    }
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(CreateDBClusterError::DBClusterNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterParameterGroupNotFound" => {
                        return RusotoError::Service(
                            CreateDBClusterError::DBClusterParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterQuotaExceededFault" => {
                        return RusotoError::Service(
                            CreateDBClusterError::DBClusterQuotaExceededFault(parsed_error.message),
                        )
                    }
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(CreateDBClusterError::DBInstanceNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return RusotoError::Service(
                            CreateDBClusterError::DBSubnetGroupDoesNotCoverEnoughAZs(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CreateDBClusterError::DBSubnetGroupNotFoundFault(parsed_error.message),
                        )
                    }
                    "InsufficientStorageClusterCapacity" => {
                        return RusotoError::Service(
                            CreateDBClusterError::InsufficientStorageClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            CreateDBClusterError::InvalidDBClusterStateFault(parsed_error.message),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            CreateDBClusterError::InvalidDBInstanceStateFault(parsed_error.message),
                        )
                    }
                    "InvalidDBSubnetGroupStateFault" => {
                        return RusotoError::Service(
                            CreateDBClusterError::InvalidDBSubnetGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(CreateDBClusterError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            CreateDBClusterError::InvalidVPCNetworkStateFault(parsed_error.message),
                        )
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return RusotoError::Service(
                            CreateDBClusterError::KMSKeyNotAccessibleFault(parsed_error.message),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateDBClusterError::StorageQuotaExceededFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDBClusterError::DBClusterAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::DBClusterParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterError::DBClusterQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::DBInstanceNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterError::DBSubnetGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::InsufficientStorageClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::InvalidDBSubnetGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::InvalidVPCNetworkStateFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::KMSKeyNotAccessibleFault(ref cause) => write!(f, "{}", cause),
            CreateDBClusterError::StorageQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDBClusterError {}
/// Errors returned by CreateDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateDBClusterParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
}

impl CreateDBClusterParameterGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDBClusterParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CreateDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBParameterGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDBClusterParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDBClusterParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDBClusterParameterGroupError {}
/// Errors returned by CreateDBClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateDBClusterSnapshotError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p>User already has a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl CreateDBClusterSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDBClusterSnapshotError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            CreateDBClusterSnapshotError::DBClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterSnapshotAlreadyExistsFault" => {
                        return RusotoError::Service(
                            CreateDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return RusotoError::Service(
                            CreateDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            CreateDBClusterSnapshotError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateDBClusterSnapshotError::SnapshotQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDBClusterSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDBClusterSnapshotError::DBClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterSnapshotError::DBClusterSnapshotAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterSnapshotError::InvalidDBClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBClusterSnapshotError::SnapshotQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDBClusterSnapshotError {}
/// Errors returned by CreateDBInstance
#[derive(Debug, PartialEq)]
pub enum CreateDBInstanceError {
    /// <p>Specified CIDRIP or EC2 security group is not authorized for the specified DB security group.</p> <p>Neptune may not also be authorized via IAM to perform necessary actions on your behalf.</p>
    AuthorizationNotFoundFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p>User already has a DB instance with the given identifier.</p>
    DBInstanceAlreadyExistsFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p> <i>DBSecurityGroupName</i> does not refer to an existing DB security group.</p>
    DBSecurityGroupNotFoundFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
    /// <p> <i>Domain</i> does not refer to an existing Active Directory Domain.</p>
    DomainNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB instances.</p>
    InstanceQuotaExceededFault(String),
    /// <p>Specified DB instance class is not available in the specified Availability Zone.</p>
    InsufficientDBInstanceCapacityFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The designated option group could not be found.</p>
    OptionGroupNotFoundFault(String),
    /// <p>Provisioned IOPS not available in the specified Availability Zone.</p>
    ProvisionedIopsNotAvailableInAZFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// <p> <i>StorageType</i> specified cannot be associated with the DB Instance.</p>
    StorageTypeNotSupportedFault(String),
}

impl CreateDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDBInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationNotFound" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::AuthorizationNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(CreateDBInstanceError::DBClusterNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "DBInstanceAlreadyExists" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::DBInstanceAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::DBSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::DBSubnetGroupDoesNotCoverEnoughAZs(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::DBSubnetGroupNotFoundFault(parsed_error.message),
                        )
                    }
                    "DomainNotFoundFault" => {
                        return RusotoError::Service(CreateDBInstanceError::DomainNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "InstanceQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::InstanceQuotaExceededFault(parsed_error.message),
                        )
                    }
                    "InsufficientDBInstanceCapacity" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::InsufficientDBInstanceCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::InvalidDBClusterStateFault(parsed_error.message),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(CreateDBInstanceError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::KMSKeyNotAccessibleFault(parsed_error.message),
                        )
                    }
                    "OptionGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::OptionGroupNotFoundFault(parsed_error.message),
                        )
                    }
                    "ProvisionedIopsNotAvailableInAZFault" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::ProvisionedIopsNotAvailableInAZFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::StorageQuotaExceededFault(parsed_error.message),
                        )
                    }
                    "StorageTypeNotSupported" => {
                        return RusotoError::Service(
                            CreateDBInstanceError::StorageTypeNotSupportedFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDBInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDBInstanceError::AuthorizationNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::DBInstanceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBInstanceError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBInstanceError::DBSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBInstanceError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBInstanceError::DBSubnetGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::DomainNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::InstanceQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::InsufficientDBInstanceCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBInstanceError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::InvalidVPCNetworkStateFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::KMSKeyNotAccessibleFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::OptionGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::ProvisionedIopsNotAvailableInAZFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBInstanceError::StorageQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::StorageTypeNotSupportedFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDBInstanceError {}
/// Errors returned by CreateDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateDBParameterGroupError {
    /// <p>A DB parameter group with the same name exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB parameter groups.</p>
    DBParameterGroupQuotaExceededFault(String),
}

impl CreateDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDBParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CreateDBParameterGroupError::DBParameterGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBParameterGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateDBParameterGroupError::DBParameterGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDBParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDBParameterGroupError::DBParameterGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBParameterGroupError::DBParameterGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDBParameterGroupError {}
/// Errors returned by CreateDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateDBSubnetGroupError {
    /// <p> <i>DBSubnetGroupName</i> is already used by an existing DB subnet group.</p>
    DBSubnetGroupAlreadyExistsFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>Request would result in user exceeding the allowed number of DB subnet groups.</p>
    DBSubnetGroupQuotaExceededFault(String),
    /// <p>Request would result in user exceeding the allowed number of subnets in a DB subnet groups.</p>
    DBSubnetQuotaExceededFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
}

impl CreateDBSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDBSubnetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CreateDBSubnetGroupError::DBSubnetGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return RusotoError::Service(
                            CreateDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateDBSubnetGroupError::DBSubnetGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetQuotaExceededFault" => {
                        return RusotoError::Service(
                            CreateDBSubnetGroupError::DBSubnetQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(CreateDBSubnetGroupError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDBSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDBSubnetGroupError::DBSubnetGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBSubnetGroupError::DBSubnetGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBSubnetGroupError::DBSubnetQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBSubnetGroupError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDBSubnetGroupError {}
/// Errors returned by CreateEventSubscription
#[derive(Debug, PartialEq)]
pub enum CreateEventSubscriptionError {
    /// <p>You have exceeded the number of events you can subscribe to.</p>
    EventSubscriptionQuotaExceededFault(String),
    /// <p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    /// <p>There is no SNS authorization.</p>
    SNSNoAuthorizationFault(String),
    /// <p>The ARN of the SNS topic could not be found.</p>
    SNSTopicArnNotFoundFault(String),
    /// <p>The source could not be found.</p>
    SourceNotFoundFault(String),
    /// <p>This subscription already exists.</p>
    SubscriptionAlreadyExistFault(String),
    /// <p>The designated subscription category could not be found.</p>
    SubscriptionCategoryNotFoundFault(String),
}

impl CreateEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEventSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "EventSubscriptionQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateEventSubscriptionError::EventSubscriptionQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SNSInvalidTopic" => {
                        return RusotoError::Service(
                            CreateEventSubscriptionError::SNSInvalidTopicFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SNSNoAuthorization" => {
                        return RusotoError::Service(
                            CreateEventSubscriptionError::SNSNoAuthorizationFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SNSTopicArnNotFound" => {
                        return RusotoError::Service(
                            CreateEventSubscriptionError::SNSTopicArnNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SourceNotFound" => {
                        return RusotoError::Service(
                            CreateEventSubscriptionError::SourceNotFoundFault(parsed_error.message),
                        )
                    }
                    "SubscriptionAlreadyExist" => {
                        return RusotoError::Service(
                            CreateEventSubscriptionError::SubscriptionAlreadyExistFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SubscriptionCategoryNotFound" => {
                        return RusotoError::Service(
                            CreateEventSubscriptionError::SubscriptionCategoryNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateEventSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEventSubscriptionError::EventSubscriptionQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSubscriptionError::SNSInvalidTopicFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSubscriptionError::SNSTopicArnNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSubscriptionError::SourceNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::SubscriptionAlreadyExistFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSubscriptionError::SubscriptionCategoryNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateEventSubscriptionError {}
/// Errors returned by DeleteDBCluster
#[derive(Debug, PartialEq)]
pub enum DeleteDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p>User already has a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl DeleteDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(DeleteDBClusterError::DBClusterNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterSnapshotAlreadyExistsFault" => {
                        return RusotoError::Service(
                            DeleteDBClusterError::DBClusterSnapshotAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return RusotoError::Service(
                            DeleteDBClusterError::InvalidDBClusterSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            DeleteDBClusterError::InvalidDBClusterStateFault(parsed_error.message),
                        )
                    }
                    "SnapshotQuotaExceeded" => {
                        return RusotoError::Service(
                            DeleteDBClusterError::SnapshotQuotaExceededFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDBClusterError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            DeleteDBClusterError::DBClusterSnapshotAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBClusterError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBClusterError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            DeleteDBClusterError::SnapshotQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDBClusterError {}
/// Errors returned by DeleteDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDBClusterParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
}

impl DeleteDBClusterParameterGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteDBClusterParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DeleteDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return RusotoError::Service(
                            DeleteDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDBClusterParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDBClusterParameterGroupError {}
/// Errors returned by DeleteDBClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteDBClusterSnapshotError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot.</p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
}

impl DeleteDBClusterSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDBClusterSnapshotError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            DeleteDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => {
                        return RusotoError::Service(
                            DeleteDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDBClusterSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDBClusterSnapshotError::DBClusterSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBClusterSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDBClusterSnapshotError {}
/// Errors returned by DeleteDBInstance
#[derive(Debug, PartialEq)]
pub enum DeleteDBInstanceError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> is already used by an existing snapshot.</p>
    DBSnapshotAlreadyExistsFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The specified DB instance is not in the <i>available</i> state.</p>
    InvalidDBInstanceStateFault(String),
    /// <p>Request would result in user exceeding the allowed number of DB snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl DeleteDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDBInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            DeleteDBInstanceError::DBInstanceNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBSnapshotAlreadyExists" => {
                        return RusotoError::Service(
                            DeleteDBInstanceError::DBSnapshotAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            DeleteDBInstanceError::InvalidDBClusterStateFault(parsed_error.message),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            DeleteDBInstanceError::InvalidDBInstanceStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotQuotaExceeded" => {
                        return RusotoError::Service(
                            DeleteDBInstanceError::SnapshotQuotaExceededFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDBInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDBInstanceError::DBInstanceNotFoundFault(ref cause) => write!(f, "{}", cause),
            DeleteDBInstanceError::DBSnapshotAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBInstanceError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            DeleteDBInstanceError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
            DeleteDBInstanceError::SnapshotQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDBInstanceError {}
/// Errors returned by DeleteDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDBParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
}

impl DeleteDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDBParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DeleteDBParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return RusotoError::Service(
                            DeleteDBParameterGroupError::InvalidDBParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDBParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDBParameterGroupError {}
/// Errors returned by DeleteDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDBSubnetGroupError {
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB subnet group cannot be deleted because it is in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p>The DB subnet is not in the <i>available</i> state.</p>
    InvalidDBSubnetStateFault(String),
}

impl DeleteDBSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDBSubnetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DeleteDBSubnetGroupError::DBSubnetGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBSubnetGroupStateFault" => {
                        return RusotoError::Service(
                            DeleteDBSubnetGroupError::InvalidDBSubnetGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBSubnetStateFault" => {
                        return RusotoError::Service(
                            DeleteDBSubnetGroupError::InvalidDBSubnetStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDBSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDBSubnetGroupError::DBSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBSubnetGroupError::InvalidDBSubnetGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDBSubnetGroupError::InvalidDBSubnetStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteDBSubnetGroupError {}
/// Errors returned by DeleteEventSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteEventSubscriptionError {
    /// <p>The event subscription is in an invalid state.</p>
    InvalidEventSubscriptionStateFault(String),
    /// <p>The designated subscription could not be found.</p>
    SubscriptionNotFoundFault(String),
}

impl DeleteEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidEventSubscriptionState" => {
                        return RusotoError::Service(
                            DeleteEventSubscriptionError::InvalidEventSubscriptionStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SubscriptionNotFound" => {
                        return RusotoError::Service(
                            DeleteEventSubscriptionError::SubscriptionNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteEventSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventSubscriptionError::InvalidEventSubscriptionStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteEventSubscriptionError::SubscriptionNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteEventSubscriptionError {}
/// Errors returned by DescribeDBClusterParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterParameterGroupsError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
}

impl DescribeDBClusterParameterGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDBClusterParameterGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeDBClusterParameterGroupsError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBClusterParameterGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBClusterParameterGroupsError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDBClusterParameterGroupsError {}
/// Errors returned by DescribeDBClusterParameters
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterParametersError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
}

impl DescribeDBClusterParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDBClusterParametersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeDBClusterParametersError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBClusterParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBClusterParametersError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDBClusterParametersError {}
/// Errors returned by DescribeDBClusterSnapshotAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterSnapshotAttributesError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot.</p>
    DBClusterSnapshotNotFoundFault(String),
}

impl DescribeDBClusterSnapshotAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDBClusterSnapshotAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotNotFoundFault" => return RusotoError::Service(
                        DescribeDBClusterSnapshotAttributesError::DBClusterSnapshotNotFoundFault(
                            parsed_error.message,
                        ),
                    ),
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBClusterSnapshotAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBClusterSnapshotAttributesError::DBClusterSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDBClusterSnapshotAttributesError {}
/// Errors returned by DescribeDBClusterSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterSnapshotsError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot.</p>
    DBClusterSnapshotNotFoundFault(String),
}

impl DescribeDBClusterSnapshotsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDBClusterSnapshotsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            DescribeDBClusterSnapshotsError::DBClusterSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBClusterSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBClusterSnapshotsError::DBClusterSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDBClusterSnapshotsError {}
/// Errors returned by DescribeDBClusters
#[derive(Debug, PartialEq)]
pub enum DescribeDBClustersError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
}

impl DescribeDBClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDBClustersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            DescribeDBClustersError::DBClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBClustersError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDBClustersError {}
/// Errors returned by DescribeDBEngineVersions
#[derive(Debug, PartialEq)]
pub enum DescribeDBEngineVersionsError {}

impl DescribeDBEngineVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDBEngineVersionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBEngineVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeDBEngineVersionsError {}
/// Errors returned by DescribeDBInstances
#[derive(Debug, PartialEq)]
pub enum DescribeDBInstancesError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
}

impl DescribeDBInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDBInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            DescribeDBInstancesError::DBInstanceNotFoundFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBInstancesError::DBInstanceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDBInstancesError {}
/// Errors returned by DescribeDBParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBParameterGroupsError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
}

impl DescribeDBParameterGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDBParameterGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeDBParameterGroupsError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBParameterGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBParameterGroupsError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDBParameterGroupsError {}
/// Errors returned by DescribeDBParameters
#[derive(Debug, PartialEq)]
pub enum DescribeDBParametersError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
}

impl DescribeDBParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDBParametersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeDBParametersError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBParametersError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDBParametersError {}
/// Errors returned by DescribeDBSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBSubnetGroupsError {
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
}

impl DescribeDBSubnetGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDBSubnetGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DescribeDBSubnetGroupsError::DBSubnetGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeDBSubnetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDBSubnetGroupsError::DBSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDBSubnetGroupsError {}
/// Errors returned by DescribeEngineDefaultClusterParameters
#[derive(Debug, PartialEq)]
pub enum DescribeEngineDefaultClusterParametersError {}

impl DescribeEngineDefaultClusterParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEngineDefaultClusterParametersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeEngineDefaultClusterParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEngineDefaultClusterParametersError {}
/// Errors returned by DescribeEngineDefaultParameters
#[derive(Debug, PartialEq)]
pub enum DescribeEngineDefaultParametersError {}

impl DescribeEngineDefaultParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEngineDefaultParametersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeEngineDefaultParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEngineDefaultParametersError {}
/// Errors returned by DescribeEventCategories
#[derive(Debug, PartialEq)]
pub enum DescribeEventCategoriesError {}

impl DescribeEventCategoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventCategoriesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeEventCategoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEventCategoriesError {}
/// Errors returned by DescribeEventSubscriptions
#[derive(Debug, PartialEq)]
pub enum DescribeEventSubscriptionsError {
    /// <p>The designated subscription could not be found.</p>
    SubscriptionNotFoundFault(String),
}

impl DescribeEventSubscriptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEventSubscriptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "SubscriptionNotFound" => {
                        return RusotoError::Service(
                            DescribeEventSubscriptionsError::SubscriptionNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeEventSubscriptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventSubscriptionsError::SubscriptionNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEventSubscriptionsError {}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {}

impl DescribeEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEventsError {}
/// Errors returned by DescribeOrderableDBInstanceOptions
#[derive(Debug, PartialEq)]
pub enum DescribeOrderableDBInstanceOptionsError {}

impl DescribeOrderableDBInstanceOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrderableDBInstanceOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeOrderableDBInstanceOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeOrderableDBInstanceOptionsError {}
/// Errors returned by DescribePendingMaintenanceActions
#[derive(Debug, PartialEq)]
pub enum DescribePendingMaintenanceActionsError {
    /// <p>The specified resource ID was not found.</p>
    ResourceNotFoundFault(String),
}

impl DescribePendingMaintenanceActionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePendingMaintenanceActionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceNotFoundFault" => {
                        return RusotoError::Service(
                            DescribePendingMaintenanceActionsError::ResourceNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribePendingMaintenanceActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePendingMaintenanceActionsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribePendingMaintenanceActionsError {}
/// Errors returned by DescribeValidDBInstanceModifications
#[derive(Debug, PartialEq)]
pub enum DescribeValidDBInstanceModificationsError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p>The specified DB instance is not in the <i>available</i> state.</p>
    InvalidDBInstanceStateFault(String),
}

impl DescribeValidDBInstanceModificationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeValidDBInstanceModificationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            DescribeValidDBInstanceModificationsError::DBInstanceNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            DescribeValidDBInstanceModificationsError::InvalidDBInstanceStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeValidDBInstanceModificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeValidDBInstanceModificationsError::DBInstanceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeValidDBInstanceModificationsError::InvalidDBInstanceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeValidDBInstanceModificationsError {}
/// Errors returned by FailoverDBCluster
#[derive(Debug, PartialEq)]
pub enum FailoverDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The specified DB instance is not in the <i>available</i> state.</p>
    InvalidDBInstanceStateFault(String),
}

impl FailoverDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<FailoverDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            FailoverDBClusterError::DBClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            FailoverDBClusterError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            FailoverDBClusterError::InvalidDBInstanceStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for FailoverDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FailoverDBClusterError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            FailoverDBClusterError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            FailoverDBClusterError::InvalidDBInstanceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for FailoverDBClusterError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot.</p>
    DBSnapshotNotFoundFault(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::DBClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::DBInstanceNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBSnapshotNotFound" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::DBSnapshotNotFoundFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::DBInstanceNotFoundFault(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::DBSnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ModifyDBCluster
#[derive(Debug, PartialEq)]
pub enum ModifyDBClusterError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBClusterParameterGroupName</i> does not refer to an existing DB Cluster parameter group.</p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The specified DB instance is not in the <i>available</i> state.</p>
    InvalidDBInstanceStateFault(String),
    /// <p>The state of the DB security group does not allow deletion.</p>
    InvalidDBSecurityGroupStateFault(String),
    /// <p>The DB subnet group cannot be deleted because it is in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
}

impl ModifyDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterAlreadyExistsFault" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::DBClusterAlreadyExistsFault(parsed_error.message),
                        )
                    }
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(ModifyDBClusterError::DBClusterNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "DBClusterParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::DBClusterParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::DBSubnetGroupNotFoundFault(parsed_error.message),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::InvalidDBClusterStateFault(parsed_error.message),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::InvalidDBInstanceStateFault(parsed_error.message),
                        )
                    }
                    "InvalidDBSecurityGroupState" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::InvalidDBSecurityGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBSubnetGroupStateFault" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::InvalidDBSubnetGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(ModifyDBClusterError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::InvalidVPCNetworkStateFault(parsed_error.message),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return RusotoError::Service(
                            ModifyDBClusterError::StorageQuotaExceededFault(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyDBClusterError::DBClusterAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            ModifyDBClusterError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyDBClusterError::DBClusterParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBClusterError::DBSubnetGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyDBClusterError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            ModifyDBClusterError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
            ModifyDBClusterError::InvalidDBSecurityGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBClusterError::InvalidDBSubnetGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBClusterError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            ModifyDBClusterError::InvalidVPCNetworkStateFault(ref cause) => write!(f, "{}", cause),
            ModifyDBClusterError::StorageQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyDBClusterError {}
/// Errors returned by ModifyDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum ModifyDBClusterParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
}

impl ModifyDBClusterParameterGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyDBClusterParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return RusotoError::Service(
                            ModifyDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyDBClusterParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyDBClusterParameterGroupError {}
/// Errors returned by ModifyDBClusterSnapshotAttribute
#[derive(Debug, PartialEq)]
pub enum ModifyDBClusterSnapshotAttributeError {
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot.</p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>You have exceeded the maximum number of accounts that you can share a manual DB snapshot with.</p>
    SharedSnapshotQuotaExceededFault(String),
}

impl ModifyDBClusterSnapshotAttributeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyDBClusterSnapshotAttributeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterSnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            ModifyDBClusterSnapshotAttributeError::DBClusterSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterSnapshotStateFault" => return RusotoError::Service(
                        ModifyDBClusterSnapshotAttributeError::InvalidDBClusterSnapshotStateFault(
                            parsed_error.message,
                        ),
                    ),
                    "SharedSnapshotQuotaExceeded" => {
                        return RusotoError::Service(
                            ModifyDBClusterSnapshotAttributeError::SharedSnapshotQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyDBClusterSnapshotAttributeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyDBClusterSnapshotAttributeError::DBClusterSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBClusterSnapshotAttributeError::InvalidDBClusterSnapshotStateFault(
                ref cause,
            ) => write!(f, "{}", cause),
            ModifyDBClusterSnapshotAttributeError::SharedSnapshotQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyDBClusterSnapshotAttributeError {}
/// Errors returned by ModifyDBInstance
#[derive(Debug, PartialEq)]
pub enum ModifyDBInstanceError {
    /// <p>Specified CIDRIP or EC2 security group is not authorized for the specified DB security group.</p> <p>Neptune may not also be authorized via IAM to perform necessary actions on your behalf.</p>
    AuthorizationNotFoundFault(String),
    /// <p> <i>CertificateIdentifier</i> does not refer to an existing certificate.</p>
    CertificateNotFoundFault(String),
    /// <p>User already has a DB instance with the given identifier.</p>
    DBInstanceAlreadyExistsFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p> <i>DBSecurityGroupName</i> does not refer to an existing DB security group.</p>
    DBSecurityGroupNotFoundFault(String),
    /// <p>The DB upgrade failed because a resource the DB depends on could not be modified.</p>
    DBUpgradeDependencyFailureFault(String),
    /// <p> <i>Domain</i> does not refer to an existing Active Directory Domain.</p>
    DomainNotFoundFault(String),
    /// <p>Specified DB instance class is not available in the specified Availability Zone.</p>
    InsufficientDBInstanceCapacityFault(String),
    /// <p>The specified DB instance is not in the <i>available</i> state.</p>
    InvalidDBInstanceStateFault(String),
    /// <p>The state of the DB security group does not allow deletion.</p>
    InvalidDBSecurityGroupStateFault(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The designated option group could not be found.</p>
    OptionGroupNotFoundFault(String),
    /// <p>Provisioned IOPS not available in the specified Availability Zone.</p>
    ProvisionedIopsNotAvailableInAZFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// <p> <i>StorageType</i> specified cannot be associated with the DB Instance.</p>
    StorageTypeNotSupportedFault(String),
}

impl ModifyDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyDBInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationNotFound" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::AuthorizationNotFoundFault(parsed_error.message),
                        )
                    }
                    "CertificateNotFound" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::CertificateNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBInstanceAlreadyExists" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::DBInstanceAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::DBInstanceNotFoundFault(parsed_error.message),
                        )
                    }
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::DBSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBUpgradeDependencyFailure" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::DBUpgradeDependencyFailureFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DomainNotFoundFault" => {
                        return RusotoError::Service(ModifyDBInstanceError::DomainNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "InsufficientDBInstanceCapacity" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::InsufficientDBInstanceCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::InvalidDBInstanceStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBSecurityGroupState" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::InvalidDBSecurityGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OptionGroupNotFoundFault" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::OptionGroupNotFoundFault(parsed_error.message),
                        )
                    }
                    "ProvisionedIopsNotAvailableInAZFault" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::ProvisionedIopsNotAvailableInAZFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::StorageQuotaExceededFault(parsed_error.message),
                        )
                    }
                    "StorageTypeNotSupported" => {
                        return RusotoError::Service(
                            ModifyDBInstanceError::StorageTypeNotSupportedFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyDBInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyDBInstanceError::AuthorizationNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::CertificateNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::DBInstanceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::DBInstanceNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::DBSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::DBUpgradeDependencyFailureFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::DomainNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::InsufficientDBInstanceCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::InvalidDBSecurityGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::InvalidVPCNetworkStateFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::OptionGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::ProvisionedIopsNotAvailableInAZFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::StorageQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::StorageTypeNotSupportedFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyDBInstanceError {}
/// Errors returned by ModifyDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum ModifyDBParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
}

impl ModifyDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyDBParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyDBParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return RusotoError::Service(
                            ModifyDBParameterGroupError::InvalidDBParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyDBParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyDBParameterGroupError {}
/// Errors returned by ModifyDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyDBSubnetGroupError {
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed number of subnets in a DB subnet groups.</p>
    DBSubnetQuotaExceededFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>The DB subnet is already in use in the Availability Zone.</p>
    SubnetAlreadyInUse(String),
}

impl ModifyDBSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyDBSubnetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBSubnetGroupDoesNotCoverEnoughAZs" => {
                        return RusotoError::Service(
                            ModifyDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            ModifyDBSubnetGroupError::DBSubnetGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetQuotaExceededFault" => {
                        return RusotoError::Service(
                            ModifyDBSubnetGroupError::DBSubnetQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(ModifyDBSubnetGroupError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "SubnetAlreadyInUse" => {
                        return RusotoError::Service(ModifyDBSubnetGroupError::SubnetAlreadyInUse(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyDBSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyDBSubnetGroupError::DBSubnetGroupDoesNotCoverEnoughAZs(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBSubnetGroupError::DBSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBSubnetGroupError::DBSubnetQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBSubnetGroupError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            ModifyDBSubnetGroupError::SubnetAlreadyInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyDBSubnetGroupError {}
/// Errors returned by ModifyEventSubscription
#[derive(Debug, PartialEq)]
pub enum ModifyEventSubscriptionError {
    /// <p>You have exceeded the number of events you can subscribe to.</p>
    EventSubscriptionQuotaExceededFault(String),
    /// <p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    /// <p>There is no SNS authorization.</p>
    SNSNoAuthorizationFault(String),
    /// <p>The ARN of the SNS topic could not be found.</p>
    SNSTopicArnNotFoundFault(String),
    /// <p>The designated subscription category could not be found.</p>
    SubscriptionCategoryNotFoundFault(String),
    /// <p>The designated subscription could not be found.</p>
    SubscriptionNotFoundFault(String),
}

impl ModifyEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyEventSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "EventSubscriptionQuotaExceeded" => {
                        return RusotoError::Service(
                            ModifyEventSubscriptionError::EventSubscriptionQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SNSInvalidTopic" => {
                        return RusotoError::Service(
                            ModifyEventSubscriptionError::SNSInvalidTopicFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SNSNoAuthorization" => {
                        return RusotoError::Service(
                            ModifyEventSubscriptionError::SNSNoAuthorizationFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SNSTopicArnNotFound" => {
                        return RusotoError::Service(
                            ModifyEventSubscriptionError::SNSTopicArnNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SubscriptionCategoryNotFound" => {
                        return RusotoError::Service(
                            ModifyEventSubscriptionError::SubscriptionCategoryNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SubscriptionNotFound" => {
                        return RusotoError::Service(
                            ModifyEventSubscriptionError::SubscriptionNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyEventSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyEventSubscriptionError::EventSubscriptionQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyEventSubscriptionError::SNSInvalidTopicFault(ref cause) => write!(f, "{}", cause),
            ModifyEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyEventSubscriptionError::SNSTopicArnNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyEventSubscriptionError::SubscriptionCategoryNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyEventSubscriptionError::SubscriptionNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyEventSubscriptionError {}
/// Errors returned by PromoteReadReplicaDBCluster
#[derive(Debug, PartialEq)]
pub enum PromoteReadReplicaDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
}

impl PromoteReadReplicaDBClusterError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PromoteReadReplicaDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            PromoteReadReplicaDBClusterError::DBClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            PromoteReadReplicaDBClusterError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PromoteReadReplicaDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PromoteReadReplicaDBClusterError::DBClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            PromoteReadReplicaDBClusterError::InvalidDBClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PromoteReadReplicaDBClusterError {}
/// Errors returned by RebootDBInstance
#[derive(Debug, PartialEq)]
pub enum RebootDBInstanceError {
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p>The specified DB instance is not in the <i>available</i> state.</p>
    InvalidDBInstanceStateFault(String),
}

impl RebootDBInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootDBInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            RebootDBInstanceError::DBInstanceNotFoundFault(parsed_error.message),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            RebootDBInstanceError::InvalidDBInstanceStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RebootDBInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RebootDBInstanceError::DBInstanceNotFoundFault(ref cause) => write!(f, "{}", cause),
            RebootDBInstanceError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RebootDBInstanceError {}
/// Errors returned by RemoveRoleFromDBCluster
#[derive(Debug, PartialEq)]
pub enum RemoveRoleFromDBClusterError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p>The specified IAM role Amazon Resource Name (ARN) is not associated with the specified DB cluster.</p>
    DBClusterRoleNotFoundFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
}

impl RemoveRoleFromDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveRoleFromDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            RemoveRoleFromDBClusterError::DBClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterRoleNotFound" => {
                        return RusotoError::Service(
                            RemoveRoleFromDBClusterError::DBClusterRoleNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            RemoveRoleFromDBClusterError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RemoveRoleFromDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveRoleFromDBClusterError::DBClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveRoleFromDBClusterError::DBClusterRoleNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveRoleFromDBClusterError::InvalidDBClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveRoleFromDBClusterError {}
/// Errors returned by RemoveSourceIdentifierFromSubscription
#[derive(Debug, PartialEq)]
pub enum RemoveSourceIdentifierFromSubscriptionError {
    /// <p>The source could not be found.</p>
    SourceNotFoundFault(String),
    /// <p>The designated subscription could not be found.</p>
    SubscriptionNotFoundFault(String),
}

impl RemoveSourceIdentifierFromSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveSourceIdentifierFromSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "SourceNotFound" => {
                        return RusotoError::Service(
                            RemoveSourceIdentifierFromSubscriptionError::SourceNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SubscriptionNotFound" => {
                        return RusotoError::Service(
                            RemoveSourceIdentifierFromSubscriptionError::SubscriptionNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RemoveSourceIdentifierFromSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveSourceIdentifierFromSubscriptionError::SourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveSourceIdentifierFromSubscriptionError::SubscriptionNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveSourceIdentifierFromSubscriptionError {}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBInstanceIdentifier</i> does not refer to an existing DB instance.</p>
    DBInstanceNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot.</p>
    DBSnapshotNotFoundFault(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            RemoveTagsFromResourceError::DBClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBInstanceNotFound" => {
                        return RusotoError::Service(
                            RemoveTagsFromResourceError::DBInstanceNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSnapshotNotFound" => {
                        return RusotoError::Service(
                            RemoveTagsFromResourceError::DBSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromResourceError::DBClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromResourceError::DBInstanceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromResourceError::DBSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveTagsFromResourceError {}
/// Errors returned by ResetDBClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum ResetDBClusterParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
}

impl ResetDBClusterParameterGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ResetDBClusterParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ResetDBClusterParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return RusotoError::Service(
                            ResetDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ResetDBClusterParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResetDBClusterParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ResetDBClusterParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ResetDBClusterParameterGroupError {}
/// Errors returned by ResetDBParameterGroup
#[derive(Debug, PartialEq)]
pub enum ResetDBParameterGroupError {
    /// <p> <i>DBParameterGroupName</i> does not refer to an existing DB parameter group.</p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you cannot delete it when the parameter group is in this state.</p>
    InvalidDBParameterGroupStateFault(String),
}

impl ResetDBParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetDBParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ResetDBParameterGroupError::DBParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBParameterGroupState" => {
                        return RusotoError::Service(
                            ResetDBParameterGroupError::InvalidDBParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ResetDBParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResetDBParameterGroupError::DBParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ResetDBParameterGroupError::InvalidDBParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ResetDBParameterGroupError {}
/// Errors returned by RestoreDBClusterFromSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreDBClusterFromSnapshotError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <i>DBClusterParameterGroupName</i> does not refer to an existing DB Cluster parameter group.</p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p>User attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot.</p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p> <i>DBSnapshotIdentifier</i> does not refer to an existing DB snapshot.</p>
    DBSnapshotNotFoundFault(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster does not have enough capacity for the current operation.</p>
    InsufficientDBClusterCapacityFault(String),
    /// <p>There is insufficient storage available for the current action. You may be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available.</p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The state of the DB snapshot does not allow deletion.</p>
    InvalidDBSnapshotStateFault(String),
    /// <p>Cannot restore from vpc backup to non-vpc DB instance.</p>
    InvalidRestoreFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The designated option group could not be found.</p>
    OptionGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
}

impl RestoreDBClusterFromSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RestoreDBClusterFromSnapshotError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterAlreadyExistsFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::DBClusterAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterParameterGroupNotFound" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::DBClusterParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterQuotaExceededFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::DBClusterQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterSnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::DBClusterSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSnapshotNotFound" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::DBSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::DBSubnetGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientDBClusterCapacityFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::InsufficientDBClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientStorageClusterCapacity" => return RusotoError::Service(
                        RestoreDBClusterFromSnapshotError::InsufficientStorageClusterCapacityFault(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidDBClusterSnapshotStateFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::InvalidDBClusterSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBSnapshotState" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::InvalidDBSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidRestoreFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::InvalidRestoreFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::InvalidSubnet(parsed_error.message),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::KMSKeyNotAccessibleFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OptionGroupNotFoundFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::OptionGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return RusotoError::Service(
                            RestoreDBClusterFromSnapshotError::StorageQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RestoreDBClusterFromSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreDBClusterFromSnapshotError::DBClusterAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::DBClusterParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::DBClusterQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::DBClusterSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::DBSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::DBSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::InsufficientDBClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::InsufficientStorageClusterCapacityFault(
                ref cause,
            ) => write!(f, "{}", cause),
            RestoreDBClusterFromSnapshotError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::InvalidDBSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::InvalidRestoreFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            RestoreDBClusterFromSnapshotError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::KMSKeyNotAccessibleFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::OptionGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterFromSnapshotError::StorageQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RestoreDBClusterFromSnapshotError {}
/// Errors returned by RestoreDBClusterToPointInTime
#[derive(Debug, PartialEq)]
pub enum RestoreDBClusterToPointInTimeError {
    /// <p>User already has a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <i>DBClusterIdentifier</i> does not refer to an existing DB cluster.</p>
    DBClusterNotFoundFault(String),
    /// <p> <i>DBClusterParameterGroupName</i> does not refer to an existing DB Cluster parameter group.</p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p>User attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <i>DBClusterSnapshotIdentifier</i> does not refer to an existing DB cluster snapshot.</p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p> <i>DBSubnetGroupName</i> does not refer to an existing DB subnet group.</p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster does not have enough capacity for the current operation.</p>
    InsufficientDBClusterCapacityFault(String),
    /// <p>There is insufficient storage available for the current action. You may be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available.</p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The supplied value is not a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster is not in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The state of the DB snapshot does not allow deletion.</p>
    InvalidDBSnapshotStateFault(String),
    /// <p>Cannot restore from vpc backup to non-vpc DB instance.</p>
    InvalidRestoreFault(String),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(String),
    /// <p>DB subnet group does not cover all Availability Zones after it is created because users' change.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>Error accessing KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The designated option group could not be found.</p>
    OptionGroupNotFoundFault(String),
    /// <p>Request would result in user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
}

impl RestoreDBClusterToPointInTimeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RestoreDBClusterToPointInTimeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterAlreadyExistsFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::DBClusterAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::DBClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterParameterGroupNotFound" => return RusotoError::Service(
                        RestoreDBClusterToPointInTimeError::DBClusterParameterGroupNotFoundFault(
                            parsed_error.message,
                        ),
                    ),
                    "DBClusterQuotaExceededFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::DBClusterQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBClusterSnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::DBClusterSnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DBSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::DBSubnetGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientDBClusterCapacityFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::InsufficientDBClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientStorageClusterCapacity" => return RusotoError::Service(
                        RestoreDBClusterToPointInTimeError::InsufficientStorageClusterCapacityFault(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidDBClusterSnapshotStateFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::InvalidDBClusterSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBSnapshotState" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::InvalidDBSnapshotStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidRestoreFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::InvalidRestoreFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::InvalidSubnet(parsed_error.message),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "KMSKeyNotAccessibleFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::KMSKeyNotAccessibleFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OptionGroupNotFoundFault" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::OptionGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "StorageQuotaExceeded" => {
                        return RusotoError::Service(
                            RestoreDBClusterToPointInTimeError::StorageQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RestoreDBClusterToPointInTimeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreDBClusterToPointInTimeError::DBClusterAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::DBClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::DBClusterParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::DBClusterQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::DBClusterSnapshotNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::DBSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::InsufficientDBClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::InsufficientStorageClusterCapacityFault(
                ref cause,
            ) => write!(f, "{}", cause),
            RestoreDBClusterToPointInTimeError::InvalidDBClusterSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::InvalidDBClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::InvalidDBSnapshotStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::InvalidRestoreFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            RestoreDBClusterToPointInTimeError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::KMSKeyNotAccessibleFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::OptionGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RestoreDBClusterToPointInTimeError::StorageQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RestoreDBClusterToPointInTimeError {}
/// Trait representing the capabilities of the Amazon Neptune API. Amazon Neptune clients implement this trait.
#[async_trait]
pub trait Neptune {
    /// <p>Associates an Identity and Access Management (IAM) role from an Neptune DB cluster.</p>
    async fn add_role_to_db_cluster(
        &self,
        input: AddRoleToDBClusterMessage,
    ) -> Result<(), RusotoError<AddRoleToDBClusterError>>;

    /// <p>Adds a source identifier to an existing event notification subscription.</p>
    async fn add_source_identifier_to_subscription(
        &self,
        input: AddSourceIdentifierToSubscriptionMessage,
    ) -> Result<
        AddSourceIdentifierToSubscriptionResult,
        RusotoError<AddSourceIdentifierToSubscriptionError>,
    >;

    /// <p>Adds metadata tags to an Amazon Neptune resource. These tags can also be used with cost allocation reporting to track cost associated with Amazon Neptune resources, or used in a Condition statement in an IAM policy for Amazon Neptune.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> Result<(), RusotoError<AddTagsToResourceError>>;

    /// <p>Applies a pending maintenance action to a resource (for example, to a DB instance).</p>
    async fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> Result<ApplyPendingMaintenanceActionResult, RusotoError<ApplyPendingMaintenanceActionError>>;

    /// <p>Copies the specified DB cluster parameter group.</p>
    async fn copy_db_cluster_parameter_group(
        &self,
        input: CopyDBClusterParameterGroupMessage,
    ) -> Result<CopyDBClusterParameterGroupResult, RusotoError<CopyDBClusterParameterGroupError>>;

    /// <p>Copies a snapshot of a DB cluster.</p> <p>To copy a DB cluster snapshot from a shared manual DB cluster snapshot, <code>SourceDBClusterSnapshotIdentifier</code> must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot.</p> <p>You can't copy from one AWS Region to another.</p>
    async fn copy_db_cluster_snapshot(
        &self,
        input: CopyDBClusterSnapshotMessage,
    ) -> Result<CopyDBClusterSnapshotResult, RusotoError<CopyDBClusterSnapshotError>>;

    /// <p>Copies the specified DB parameter group.</p>
    async fn copy_db_parameter_group(
        &self,
        input: CopyDBParameterGroupMessage,
    ) -> Result<CopyDBParameterGroupResult, RusotoError<CopyDBParameterGroupError>>;

    /// <p>Creates a new Amazon Neptune DB cluster.</p> <p>You can use the <code>ReplicationSourceIdentifier</code> parameter to create the DB cluster as a Read Replica of another DB cluster or Amazon Neptune DB instance.</p>
    async fn create_db_cluster(
        &self,
        input: CreateDBClusterMessage,
    ) -> Result<CreateDBClusterResult, RusotoError<CreateDBClusterError>>;

    /// <p><p>Creates a new DB cluster parameter group.</p> <p>Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster.</p> <p> A DB cluster parameter group is initially created with the default parameters for the database engine used by instances in the DB cluster. To provide custom values for any of the parameters, you must modify the group after creating it using <a>ModifyDBClusterParameterGroup</a>. Once you&#39;ve created a DB cluster parameter group, you need to associate it with your DB cluster using <a>ModifyDBCluster</a>. When you associate a new DB cluster parameter group with a running DB cluster, you need to reboot the DB instances in the DB cluster without failover for the new DB cluster parameter group and associated settings to take effect.</p> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the DB cluster parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the <a href="https://console.aws.amazon.com/rds/">Amazon Neptune console</a> or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    async fn create_db_cluster_parameter_group(
        &self,
        input: CreateDBClusterParameterGroupMessage,
    ) -> Result<CreateDBClusterParameterGroupResult, RusotoError<CreateDBClusterParameterGroupError>>;

    /// <p>Creates a snapshot of a DB cluster.</p>
    async fn create_db_cluster_snapshot(
        &self,
        input: CreateDBClusterSnapshotMessage,
    ) -> Result<CreateDBClusterSnapshotResult, RusotoError<CreateDBClusterSnapshotError>>;

    /// <p>Creates a new DB instance.</p>
    async fn create_db_instance(
        &self,
        input: CreateDBInstanceMessage,
    ) -> Result<CreateDBInstanceResult, RusotoError<CreateDBInstanceError>>;

    /// <p><p>Creates a new DB parameter group.</p> <p>A DB parameter group is initially created with the default parameters for the database engine used by the DB instance. To provide custom values for any of the parameters, you must modify the group after creating it using <i>ModifyDBParameterGroup</i>. Once you&#39;ve created a DB parameter group, you need to associate it with your DB instance using <i>ModifyDBInstance</i>. When you associate a new DB parameter group with a running DB instance, you need to reboot the DB instance without failover for the new DB parameter group and associated settings to take effect.</p> <important> <p>After you create a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    async fn create_db_parameter_group(
        &self,
        input: CreateDBParameterGroupMessage,
    ) -> Result<CreateDBParameterGroupResult, RusotoError<CreateDBParameterGroupError>>;

    /// <p>Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    async fn create_db_subnet_group(
        &self,
        input: CreateDBSubnetGroupMessage,
    ) -> Result<CreateDBSubnetGroupResult, RusotoError<CreateDBSubnetGroupError>>;

    /// <p>Creates an event notification subscription. This action requires a topic ARN (Amazon Resource Name) created by either the Neptune console, the SNS console, or the SNS API. To obtain an ARN with SNS, you must create a topic in Amazon SNS and subscribe to the topic. The ARN is displayed in the SNS console.</p> <p>You can specify the type of source (SourceType) you want to be notified of, provide a list of Neptune sources (SourceIds) that triggers the events, and provide a list of event categories (EventCategories) for events you want to be notified of. For example, you can specify SourceType = db-instance, SourceIds = mydbinstance1, mydbinstance2 and EventCategories = Availability, Backup.</p> <p>If you specify both the SourceType and SourceIds, such as SourceType = db-instance and SourceIdentifier = myDBInstance1, you are notified of all the db-instance events for the specified source. If you specify a SourceType but do not specify a SourceIdentifier, you receive notice of the events for that source type for all your Neptune sources. If you do not specify either the SourceType nor the SourceIdentifier, you are notified of events generated from all Neptune sources belonging to your customer account.</p>
    async fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> Result<CreateEventSubscriptionResult, RusotoError<CreateEventSubscriptionError>>;

    /// <p>The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can't be recovered. Manual DB cluster snapshots of the specified DB cluster are not deleted.</p>
    async fn delete_db_cluster(
        &self,
        input: DeleteDBClusterMessage,
    ) -> Result<DeleteDBClusterResult, RusotoError<DeleteDBClusterError>>;

    /// <p>Deletes a specified DB cluster parameter group. The DB cluster parameter group to be deleted can't be associated with any DB clusters.</p>
    async fn delete_db_cluster_parameter_group(
        &self,
        input: DeleteDBClusterParameterGroupMessage,
    ) -> Result<(), RusotoError<DeleteDBClusterParameterGroupError>>;

    /// <p><p>Deletes a DB cluster snapshot. If the snapshot is being copied, the copy operation is terminated.</p> <note> <p>The DB cluster snapshot must be in the <code>available</code> state to be deleted.</p> </note></p>
    async fn delete_db_cluster_snapshot(
        &self,
        input: DeleteDBClusterSnapshotMessage,
    ) -> Result<DeleteDBClusterSnapshotResult, RusotoError<DeleteDBClusterSnapshotError>>;

    /// <p>The DeleteDBInstance action deletes a previously provisioned DB instance. When you delete a DB instance, all automated backups for that instance are deleted and can't be recovered. Manual DB snapshots of the DB instance to be deleted by <code>DeleteDBInstance</code> are not deleted.</p> <p> If you request a final DB snapshot the status of the Amazon Neptune DB instance is <code>deleting</code> until the DB snapshot is created. The API action <code>DescribeDBInstance</code> is used to monitor the status of this operation. The action can't be canceled or reverted once submitted.</p> <p>Note that when a DB instance is in a failure state and has a status of <code>failed</code>, <code>incompatible-restore</code>, or <code>incompatible-network</code>, you can only delete it when the <code>SkipFinalSnapshot</code> parameter is set to <code>true</code>.</p> <p>You can't delete a DB instance if it is the only instance in the DB cluster.</p>
    async fn delete_db_instance(
        &self,
        input: DeleteDBInstanceMessage,
    ) -> Result<DeleteDBInstanceResult, RusotoError<DeleteDBInstanceError>>;

    /// <p>Deletes a specified DBParameterGroup. The DBParameterGroup to be deleted can't be associated with any DB instances.</p>
    async fn delete_db_parameter_group(
        &self,
        input: DeleteDBParameterGroupMessage,
    ) -> Result<(), RusotoError<DeleteDBParameterGroupError>>;

    /// <p><p>Deletes a DB subnet group.</p> <note> <p>The specified database subnet group must not be associated with any DB instances.</p> </note></p>
    async fn delete_db_subnet_group(
        &self,
        input: DeleteDBSubnetGroupMessage,
    ) -> Result<(), RusotoError<DeleteDBSubnetGroupError>>;

    /// <p>Deletes an event notification subscription.</p>
    async fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> Result<DeleteEventSubscriptionResult, RusotoError<DeleteEventSubscriptionError>>;

    /// <p> Returns a list of <code>DBClusterParameterGroup</code> descriptions. If a <code>DBClusterParameterGroupName</code> parameter is specified, the list will contain only the description of the specified DB cluster parameter group.</p>
    async fn describe_db_cluster_parameter_groups(
        &self,
        input: DescribeDBClusterParameterGroupsMessage,
    ) -> Result<DBClusterParameterGroupsMessage, RusotoError<DescribeDBClusterParameterGroupsError>>;

    /// <p>Returns the detailed parameter list for a particular DB cluster parameter group.</p>
    async fn describe_db_cluster_parameters(
        &self,
        input: DescribeDBClusterParametersMessage,
    ) -> Result<DBClusterParameterGroupDetails, RusotoError<DescribeDBClusterParametersError>>;

    /// <p>Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot.</p> <p>When sharing snapshots with other AWS accounts, <code>DescribeDBClusterSnapshotAttributes</code> returns the <code>restore</code> attribute and a list of IDs for the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If <code>all</code> is included in the list of values for the <code>restore</code> attribute, then the manual DB cluster snapshot is public and can be copied or restored by all AWS accounts.</p> <p>To add or remove access for an AWS account to copy or restore a manual DB cluster snapshot, or to make the manual DB cluster snapshot public or private, use the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    async fn describe_db_cluster_snapshot_attributes(
        &self,
        input: DescribeDBClusterSnapshotAttributesMessage,
    ) -> Result<
        DescribeDBClusterSnapshotAttributesResult,
        RusotoError<DescribeDBClusterSnapshotAttributesError>,
    >;

    /// <p>Returns information about DB cluster snapshots. This API action supports pagination.</p>
    async fn describe_db_cluster_snapshots(
        &self,
        input: DescribeDBClusterSnapshotsMessage,
    ) -> Result<DBClusterSnapshotMessage, RusotoError<DescribeDBClusterSnapshotsError>>;

    /// <p>Returns information about provisioned DB clusters. This API supports pagination.</p>
    async fn describe_db_clusters(
        &self,
        input: DescribeDBClustersMessage,
    ) -> Result<DBClusterMessage, RusotoError<DescribeDBClustersError>>;

    /// <p>Returns a list of the available DB engines.</p>
    async fn describe_db_engine_versions(
        &self,
        input: DescribeDBEngineVersionsMessage,
    ) -> Result<DBEngineVersionMessage, RusotoError<DescribeDBEngineVersionsError>>;

    /// <p>Returns information about provisioned instances. This API supports pagination.</p>
    async fn describe_db_instances(
        &self,
        input: DescribeDBInstancesMessage,
    ) -> Result<DBInstanceMessage, RusotoError<DescribeDBInstancesError>>;

    /// <p>Returns a list of <code>DBParameterGroup</code> descriptions. If a <code>DBParameterGroupName</code> is specified, the list will contain only the description of the specified DB parameter group.</p>
    async fn describe_db_parameter_groups(
        &self,
        input: DescribeDBParameterGroupsMessage,
    ) -> Result<DBParameterGroupsMessage, RusotoError<DescribeDBParameterGroupsError>>;

    /// <p>Returns the detailed parameter list for a particular DB parameter group.</p>
    async fn describe_db_parameters(
        &self,
        input: DescribeDBParametersMessage,
    ) -> Result<DBParameterGroupDetails, RusotoError<DescribeDBParametersError>>;

    /// <p>Returns a list of DBSubnetGroup descriptions. If a DBSubnetGroupName is specified, the list will contain only the descriptions of the specified DBSubnetGroup.</p> <p>For an overview of CIDR ranges, go to the <a href="http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Wikipedia Tutorial</a>.</p>
    async fn describe_db_subnet_groups(
        &self,
        input: DescribeDBSubnetGroupsMessage,
    ) -> Result<DBSubnetGroupMessage, RusotoError<DescribeDBSubnetGroupsError>>;

    /// <p>Returns the default engine and system parameter information for the cluster database engine.</p>
    async fn describe_engine_default_cluster_parameters(
        &self,
        input: DescribeEngineDefaultClusterParametersMessage,
    ) -> Result<
        DescribeEngineDefaultClusterParametersResult,
        RusotoError<DescribeEngineDefaultClusterParametersError>,
    >;

    /// <p>Returns the default engine and system parameter information for the specified database engine.</p>
    async fn describe_engine_default_parameters(
        &self,
        input: DescribeEngineDefaultParametersMessage,
    ) -> Result<
        DescribeEngineDefaultParametersResult,
        RusotoError<DescribeEngineDefaultParametersError>,
    >;

    /// <p>Displays a list of categories for all event source types, or, if specified, for a specified source type.</p>
    async fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> Result<EventCategoriesMessage, RusotoError<DescribeEventCategoriesError>>;

    /// <p>Lists all the subscription descriptions for a customer account. The description for a subscription includes SubscriptionName, SNSTopicARN, CustomerID, SourceType, SourceID, CreationTime, and Status.</p> <p>If you specify a SubscriptionName, lists the description for that subscription.</p>
    async fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> Result<EventSubscriptionsMessage, RusotoError<DescribeEventSubscriptionsError>>;

    /// <p>Returns events related to DB instances, DB security groups, DB snapshots, and DB parameter groups for the past 14 days. Events specific to a particular DB instance, DB security group, database snapshot, or DB parameter group can be obtained by providing the name as a parameter. By default, the past hour of events are returned.</p>
    async fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> Result<EventsMessage, RusotoError<DescribeEventsError>>;

    /// <p>Returns a list of orderable DB instance options for the specified engine.</p>
    async fn describe_orderable_db_instance_options(
        &self,
        input: DescribeOrderableDBInstanceOptionsMessage,
    ) -> Result<
        OrderableDBInstanceOptionsMessage,
        RusotoError<DescribeOrderableDBInstanceOptionsError>,
    >;

    /// <p>Returns a list of resources (for example, DB instances) that have at least one pending maintenance action.</p>
    async fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> Result<PendingMaintenanceActionsMessage, RusotoError<DescribePendingMaintenanceActionsError>>;

    /// <p>You can call <a>DescribeValidDBInstanceModifications</a> to learn what modifications you can make to your DB instance. You can use this information when you call <a>ModifyDBInstance</a>.</p>
    async fn describe_valid_db_instance_modifications(
        &self,
        input: DescribeValidDBInstanceModificationsMessage,
    ) -> Result<
        DescribeValidDBInstanceModificationsResult,
        RusotoError<DescribeValidDBInstanceModificationsError>,
    >;

    /// <p>Forces a failover for a DB cluster.</p> <p>A failover for a DB cluster promotes one of the Read Replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer).</p> <p>Amazon Neptune will automatically fail over to a Read Replica, if one exists, when the primary instance fails. You can force a failover when you want to simulate a failure of a primary instance for testing. Because each instance in a DB cluster has its own endpoint address, you will need to clean up and re-establish any existing connections that use those endpoint addresses when the failover is complete.</p>
    async fn failover_db_cluster(
        &self,
        input: FailoverDBClusterMessage,
    ) -> Result<FailoverDBClusterResult, RusotoError<FailoverDBClusterError>>;

    /// <p>Lists all tags on an Amazon Neptune resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> Result<TagListMessage, RusotoError<ListTagsForResourceError>>;

    /// <p>Modify a setting for a DB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request.</p>
    async fn modify_db_cluster(
        &self,
        input: ModifyDBClusterMessage,
    ) -> Result<ModifyDBClusterResult, RusotoError<ModifyDBClusterError>>;

    /// <p><p> Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request.</p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB cluster associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    async fn modify_db_cluster_parameter_group(
        &self,
        input: ModifyDBClusterParameterGroupMessage,
    ) -> Result<DBClusterParameterGroupNameMessage, RusotoError<ModifyDBClusterParameterGroupError>>;

    /// <p>Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot.</p> <p>To share a manual DB cluster snapshot with other AWS accounts, specify <code>restore</code> as the <code>AttributeName</code> and use the <code>ValuesToAdd</code> parameter to add a list of IDs of the AWS accounts that are authorized to restore the manual DB cluster snapshot. Use the value <code>all</code> to make the manual DB cluster snapshot public, which means that it can be copied or restored by all AWS accounts. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts. If a manual DB cluster snapshot is encrypted, it can be shared, but only by specifying a list of authorized AWS account IDs for the <code>ValuesToAdd</code> parameter. You can't use <code>all</code> as a value for that parameter in this case.</p> <p>To view which AWS accounts have access to copy or restore a manual DB cluster snapshot, or whether a manual DB cluster snapshot public or private, use the <a>DescribeDBClusterSnapshotAttributes</a> API action.</p>
    async fn modify_db_cluster_snapshot_attribute(
        &self,
        input: ModifyDBClusterSnapshotAttributeMessage,
    ) -> Result<
        ModifyDBClusterSnapshotAttributeResult,
        RusotoError<ModifyDBClusterSnapshotAttributeError>,
    >;

    /// <p>Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. To learn what modifications you can make to your DB instance, call <a>DescribeValidDBInstanceModifications</a> before you call <a>ModifyDBInstance</a>.</p>
    async fn modify_db_instance(
        &self,
        input: ModifyDBInstanceMessage,
    ) -> Result<ModifyDBInstanceResult, RusotoError<ModifyDBInstanceError>>;

    /// <p><p>Modifies the parameters of a DB parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request.</p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB instance associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you modify a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the modify action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    async fn modify_db_parameter_group(
        &self,
        input: ModifyDBParameterGroupMessage,
    ) -> Result<DBParameterGroupNameMessage, RusotoError<ModifyDBParameterGroupError>>;

    /// <p>Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    async fn modify_db_subnet_group(
        &self,
        input: ModifyDBSubnetGroupMessage,
    ) -> Result<ModifyDBSubnetGroupResult, RusotoError<ModifyDBSubnetGroupError>>;

    /// <p>Modifies an existing event notification subscription. Note that you can't modify the source identifiers using this call; to change source identifiers for a subscription, use the <a>AddSourceIdentifierToSubscription</a> and <a>RemoveSourceIdentifierFromSubscription</a> calls.</p> <p>You can see a list of the event categories for a given SourceType by using the <b>DescribeEventCategories</b> action.</p>
    async fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> Result<ModifyEventSubscriptionResult, RusotoError<ModifyEventSubscriptionError>>;

    /// <p>Not supported.</p>
    async fn promote_read_replica_db_cluster(
        &self,
        input: PromoteReadReplicaDBClusterMessage,
    ) -> Result<PromoteReadReplicaDBClusterResult, RusotoError<PromoteReadReplicaDBClusterError>>;

    /// <p>You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB parameter group associated with the DB instance, you must reboot the instance for the changes to take effect.</p> <p>Rebooting a DB instance restarts the database engine service. Rebooting a DB instance results in a momentary outage, during which the DB instance status is set to rebooting.</p>
    async fn reboot_db_instance(
        &self,
        input: RebootDBInstanceMessage,
    ) -> Result<RebootDBInstanceResult, RusotoError<RebootDBInstanceError>>;

    /// <p>Disassociates an Identity and Access Management (IAM) role from a DB cluster.</p>
    async fn remove_role_from_db_cluster(
        &self,
        input: RemoveRoleFromDBClusterMessage,
    ) -> Result<(), RusotoError<RemoveRoleFromDBClusterError>>;

    /// <p>Removes a source identifier from an existing event notification subscription.</p>
    async fn remove_source_identifier_from_subscription(
        &self,
        input: RemoveSourceIdentifierFromSubscriptionMessage,
    ) -> Result<
        RemoveSourceIdentifierFromSubscriptionResult,
        RusotoError<RemoveSourceIdentifierFromSubscriptionError>,
    >;

    /// <p>Removes metadata tags from an Amazon Neptune resource.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> Result<(), RusotoError<RemoveTagsFromResourceError>>;

    /// <p> Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters submit a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB cluster parameter group, specify the <code>DBClusterParameterGroupName</code> and <code>ResetAllParameters</code> parameters.</p> <p> When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <a>RebootDBInstance</a> request. You must call <a>RebootDBInstance</a> for every DB instance in your DB cluster that you want the updated static parameter to apply to.</p>
    async fn reset_db_cluster_parameter_group(
        &self,
        input: ResetDBClusterParameterGroupMessage,
    ) -> Result<DBClusterParameterGroupNameMessage, RusotoError<ResetDBClusterParameterGroupError>>;

    /// <p>Modifies the parameters of a DB parameter group to the engine/system default value. To reset specific parameters, provide a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB parameter group, specify the <code>DBParameterGroup</code> name and <code>ResetAllParameters</code> parameters. When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <code>RebootDBInstance</code> request.</p>
    async fn reset_db_parameter_group(
        &self,
        input: ResetDBParameterGroupMessage,
    ) -> Result<DBParameterGroupNameMessage, RusotoError<ResetDBParameterGroupError>>;

    /// <p>Creates a new DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.</p> <p>If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.</p>
    async fn restore_db_cluster_from_snapshot(
        &self,
        input: RestoreDBClusterFromSnapshotMessage,
    ) -> Result<RestoreDBClusterFromSnapshotResult, RusotoError<RestoreDBClusterFromSnapshotError>>;

    /// <p><p>Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before <code>LatestRestorableTime</code> for up to <code>BackupRetentionPeriod</code> days. The target DB cluster is created from the source DB cluster with the same configuration as the original DB cluster, except that the new DB cluster is created with the default DB security group.</p> <note> <p>This action only restores the DB cluster, not the DB instances for that DB cluster. You must invoke the <a>CreateDBInstance</a> action to create DB instances for the restored DB cluster, specifying the identifier of the restored DB cluster in <code>DBClusterIdentifier</code>. You can create DB instances only after the <code>RestoreDBClusterToPointInTime</code> action has completed and the DB cluster is available.</p> </note></p>
    async fn restore_db_cluster_to_point_in_time(
        &self,
        input: RestoreDBClusterToPointInTimeMessage,
    ) -> Result<RestoreDBClusterToPointInTimeResult, RusotoError<RestoreDBClusterToPointInTimeError>>;
}
/// A client for the Amazon Neptune API.
#[derive(Clone)]
pub struct NeptuneClient {
    client: Client,
    region: region::Region,
}

impl NeptuneClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> NeptuneClient {
        NeptuneClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> NeptuneClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        NeptuneClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> NeptuneClient {
        NeptuneClient { client, region }
    }
}

#[async_trait]
impl Neptune for NeptuneClient {
    /// <p>Associates an Identity and Access Management (IAM) role from an Neptune DB cluster.</p>
    async fn add_role_to_db_cluster(
        &self,
        input: AddRoleToDBClusterMessage,
    ) -> Result<(), RusotoError<AddRoleToDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddRoleToDBCluster");
        params.put("Version", "2014-10-31");
        AddRoleToDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AddRoleToDBClusterError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Adds a source identifier to an existing event notification subscription.</p>
    async fn add_source_identifier_to_subscription(
        &self,
        input: AddSourceIdentifierToSubscriptionMessage,
    ) -> Result<
        AddSourceIdentifierToSubscriptionResult,
        RusotoError<AddSourceIdentifierToSubscriptionError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddSourceIdentifierToSubscription");
        params.put("Version", "2014-10-31");
        AddSourceIdentifierToSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AddSourceIdentifierToSubscriptionError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = AddSourceIdentifierToSubscriptionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = AddSourceIdentifierToSubscriptionResultDeserializer::deserialize(
                "AddSourceIdentifierToSubscriptionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds metadata tags to an Amazon Neptune resource. These tags can also be used with cost allocation reporting to track cost associated with Amazon Neptune resources, or used in a Condition statement in an IAM policy for Amazon Neptune.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> Result<(), RusotoError<AddTagsToResourceError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddTagsToResource");
        params.put("Version", "2014-10-31");
        AddTagsToResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AddTagsToResourceError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Applies a pending maintenance action to a resource (for example, to a DB instance).</p>
    async fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> Result<ApplyPendingMaintenanceActionResult, RusotoError<ApplyPendingMaintenanceActionError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ApplyPendingMaintenanceAction");
        params.put("Version", "2014-10-31");
        ApplyPendingMaintenanceActionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ApplyPendingMaintenanceActionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ApplyPendingMaintenanceActionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ApplyPendingMaintenanceActionResultDeserializer::deserialize(
                "ApplyPendingMaintenanceActionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Copies the specified DB cluster parameter group.</p>
    async fn copy_db_cluster_parameter_group(
        &self,
        input: CopyDBClusterParameterGroupMessage,
    ) -> Result<CopyDBClusterParameterGroupResult, RusotoError<CopyDBClusterParameterGroupError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopyDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        CopyDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CopyDBClusterParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CopyDBClusterParameterGroupResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CopyDBClusterParameterGroupResultDeserializer::deserialize(
                "CopyDBClusterParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Copies a snapshot of a DB cluster.</p> <p>To copy a DB cluster snapshot from a shared manual DB cluster snapshot, <code>SourceDBClusterSnapshotIdentifier</code> must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot.</p> <p>You can't copy from one AWS Region to another.</p>
    async fn copy_db_cluster_snapshot(
        &self,
        input: CopyDBClusterSnapshotMessage,
    ) -> Result<CopyDBClusterSnapshotResult, RusotoError<CopyDBClusterSnapshotError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopyDBClusterSnapshot");
        params.put("Version", "2014-10-31");
        CopyDBClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CopyDBClusterSnapshotError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CopyDBClusterSnapshotResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CopyDBClusterSnapshotResultDeserializer::deserialize(
                "CopyDBClusterSnapshotResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Copies the specified DB parameter group.</p>
    async fn copy_db_parameter_group(
        &self,
        input: CopyDBParameterGroupMessage,
    ) -> Result<CopyDBParameterGroupResult, RusotoError<CopyDBParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopyDBParameterGroup");
        params.put("Version", "2014-10-31");
        CopyDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CopyDBParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CopyDBParameterGroupResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CopyDBParameterGroupResultDeserializer::deserialize(
                "CopyDBParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new Amazon Neptune DB cluster.</p> <p>You can use the <code>ReplicationSourceIdentifier</code> parameter to create the DB cluster as a Read Replica of another DB cluster or Amazon Neptune DB instance.</p>
    async fn create_db_cluster(
        &self,
        input: CreateDBClusterMessage,
    ) -> Result<CreateDBClusterResult, RusotoError<CreateDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBCluster");
        params.put("Version", "2014-10-31");
        CreateDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDBClusterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateDBClusterResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateDBClusterResultDeserializer::deserialize(
                "CreateDBClusterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a new DB cluster parameter group.</p> <p>Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster.</p> <p> A DB cluster parameter group is initially created with the default parameters for the database engine used by instances in the DB cluster. To provide custom values for any of the parameters, you must modify the group after creating it using <a>ModifyDBClusterParameterGroup</a>. Once you&#39;ve created a DB cluster parameter group, you need to associate it with your DB cluster using <a>ModifyDBCluster</a>. When you associate a new DB cluster parameter group with a running DB cluster, you need to reboot the DB instances in the DB cluster without failover for the new DB cluster parameter group and associated settings to take effect.</p> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the DB cluster parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the <a href="https://console.aws.amazon.com/rds/">Amazon Neptune console</a> or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    async fn create_db_cluster_parameter_group(
        &self,
        input: CreateDBClusterParameterGroupMessage,
    ) -> Result<CreateDBClusterParameterGroupResult, RusotoError<CreateDBClusterParameterGroupError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        CreateDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDBClusterParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateDBClusterParameterGroupResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateDBClusterParameterGroupResultDeserializer::deserialize(
                "CreateDBClusterParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a snapshot of a DB cluster.</p>
    async fn create_db_cluster_snapshot(
        &self,
        input: CreateDBClusterSnapshotMessage,
    ) -> Result<CreateDBClusterSnapshotResult, RusotoError<CreateDBClusterSnapshotError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBClusterSnapshot");
        params.put("Version", "2014-10-31");
        CreateDBClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDBClusterSnapshotError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateDBClusterSnapshotResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateDBClusterSnapshotResultDeserializer::deserialize(
                "CreateDBClusterSnapshotResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new DB instance.</p>
    async fn create_db_instance(
        &self,
        input: CreateDBInstanceMessage,
    ) -> Result<CreateDBInstanceResult, RusotoError<CreateDBInstanceError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBInstance");
        params.put("Version", "2014-10-31");
        CreateDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDBInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateDBInstanceResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateDBInstanceResultDeserializer::deserialize(
                "CreateDBInstanceResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a new DB parameter group.</p> <p>A DB parameter group is initially created with the default parameters for the database engine used by the DB instance. To provide custom values for any of the parameters, you must modify the group after creating it using <i>ModifyDBParameterGroup</i>. Once you&#39;ve created a DB parameter group, you need to associate it with your DB instance using <i>ModifyDBInstance</i>. When you associate a new DB parameter group with a running DB instance, you need to reboot the DB instance without failover for the new DB parameter group and associated settings to take effect.</p> <important> <p>After you create a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    async fn create_db_parameter_group(
        &self,
        input: CreateDBParameterGroupMessage,
    ) -> Result<CreateDBParameterGroupResult, RusotoError<CreateDBParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBParameterGroup");
        params.put("Version", "2014-10-31");
        CreateDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDBParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateDBParameterGroupResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateDBParameterGroupResultDeserializer::deserialize(
                "CreateDBParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    async fn create_db_subnet_group(
        &self,
        input: CreateDBSubnetGroupMessage,
    ) -> Result<CreateDBSubnetGroupResult, RusotoError<CreateDBSubnetGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDBSubnetGroup");
        params.put("Version", "2014-10-31");
        CreateDBSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDBSubnetGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateDBSubnetGroupResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateDBSubnetGroupResultDeserializer::deserialize(
                "CreateDBSubnetGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an event notification subscription. This action requires a topic ARN (Amazon Resource Name) created by either the Neptune console, the SNS console, or the SNS API. To obtain an ARN with SNS, you must create a topic in Amazon SNS and subscribe to the topic. The ARN is displayed in the SNS console.</p> <p>You can specify the type of source (SourceType) you want to be notified of, provide a list of Neptune sources (SourceIds) that triggers the events, and provide a list of event categories (EventCategories) for events you want to be notified of. For example, you can specify SourceType = db-instance, SourceIds = mydbinstance1, mydbinstance2 and EventCategories = Availability, Backup.</p> <p>If you specify both the SourceType and SourceIds, such as SourceType = db-instance and SourceIdentifier = myDBInstance1, you are notified of all the db-instance events for the specified source. If you specify a SourceType but do not specify a SourceIdentifier, you receive notice of the events for that source type for all your Neptune sources. If you do not specify either the SourceType nor the SourceIdentifier, you are notified of events generated from all Neptune sources belonging to your customer account.</p>
    async fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> Result<CreateEventSubscriptionResult, RusotoError<CreateEventSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateEventSubscription");
        params.put("Version", "2014-10-31");
        CreateEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateEventSubscriptionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateEventSubscriptionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateEventSubscriptionResultDeserializer::deserialize(
                "CreateEventSubscriptionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can't be recovered. Manual DB cluster snapshots of the specified DB cluster are not deleted.</p>
    async fn delete_db_cluster(
        &self,
        input: DeleteDBClusterMessage,
    ) -> Result<DeleteDBClusterResult, RusotoError<DeleteDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBCluster");
        params.put("Version", "2014-10-31");
        DeleteDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDBClusterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteDBClusterResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteDBClusterResultDeserializer::deserialize(
                "DeleteDBClusterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a specified DB cluster parameter group. The DB cluster parameter group to be deleted can't be associated with any DB clusters.</p>
    async fn delete_db_cluster_parameter_group(
        &self,
        input: DeleteDBClusterParameterGroupMessage,
    ) -> Result<(), RusotoError<DeleteDBClusterParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        DeleteDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDBClusterParameterGroupError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a DB cluster snapshot. If the snapshot is being copied, the copy operation is terminated.</p> <note> <p>The DB cluster snapshot must be in the <code>available</code> state to be deleted.</p> </note></p>
    async fn delete_db_cluster_snapshot(
        &self,
        input: DeleteDBClusterSnapshotMessage,
    ) -> Result<DeleteDBClusterSnapshotResult, RusotoError<DeleteDBClusterSnapshotError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBClusterSnapshot");
        params.put("Version", "2014-10-31");
        DeleteDBClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDBClusterSnapshotError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteDBClusterSnapshotResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteDBClusterSnapshotResultDeserializer::deserialize(
                "DeleteDBClusterSnapshotResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>The DeleteDBInstance action deletes a previously provisioned DB instance. When you delete a DB instance, all automated backups for that instance are deleted and can't be recovered. Manual DB snapshots of the DB instance to be deleted by <code>DeleteDBInstance</code> are not deleted.</p> <p> If you request a final DB snapshot the status of the Amazon Neptune DB instance is <code>deleting</code> until the DB snapshot is created. The API action <code>DescribeDBInstance</code> is used to monitor the status of this operation. The action can't be canceled or reverted once submitted.</p> <p>Note that when a DB instance is in a failure state and has a status of <code>failed</code>, <code>incompatible-restore</code>, or <code>incompatible-network</code>, you can only delete it when the <code>SkipFinalSnapshot</code> parameter is set to <code>true</code>.</p> <p>You can't delete a DB instance if it is the only instance in the DB cluster.</p>
    async fn delete_db_instance(
        &self,
        input: DeleteDBInstanceMessage,
    ) -> Result<DeleteDBInstanceResult, RusotoError<DeleteDBInstanceError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBInstance");
        params.put("Version", "2014-10-31");
        DeleteDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDBInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteDBInstanceResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteDBInstanceResultDeserializer::deserialize(
                "DeleteDBInstanceResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a specified DBParameterGroup. The DBParameterGroup to be deleted can't be associated with any DB instances.</p>
    async fn delete_db_parameter_group(
        &self,
        input: DeleteDBParameterGroupMessage,
    ) -> Result<(), RusotoError<DeleteDBParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBParameterGroup");
        params.put("Version", "2014-10-31");
        DeleteDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDBParameterGroupError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a DB subnet group.</p> <note> <p>The specified database subnet group must not be associated with any DB instances.</p> </note></p>
    async fn delete_db_subnet_group(
        &self,
        input: DeleteDBSubnetGroupMessage,
    ) -> Result<(), RusotoError<DeleteDBSubnetGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDBSubnetGroup");
        params.put("Version", "2014-10-31");
        DeleteDBSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDBSubnetGroupError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes an event notification subscription.</p>
    async fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> Result<DeleteEventSubscriptionResult, RusotoError<DeleteEventSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteEventSubscription");
        params.put("Version", "2014-10-31");
        DeleteEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteEventSubscriptionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteEventSubscriptionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteEventSubscriptionResultDeserializer::deserialize(
                "DeleteEventSubscriptionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p> Returns a list of <code>DBClusterParameterGroup</code> descriptions. If a <code>DBClusterParameterGroupName</code> parameter is specified, the list will contain only the description of the specified DB cluster parameter group.</p>
    async fn describe_db_cluster_parameter_groups(
        &self,
        input: DescribeDBClusterParameterGroupsMessage,
    ) -> Result<DBClusterParameterGroupsMessage, RusotoError<DescribeDBClusterParameterGroupsError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterParameterGroups");
        params.put("Version", "2014-10-31");
        DescribeDBClusterParameterGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBClusterParameterGroupsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBClusterParameterGroupsMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBClusterParameterGroupsMessageDeserializer::deserialize(
                "DescribeDBClusterParameterGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the detailed parameter list for a particular DB cluster parameter group.</p>
    async fn describe_db_cluster_parameters(
        &self,
        input: DescribeDBClusterParametersMessage,
    ) -> Result<DBClusterParameterGroupDetails, RusotoError<DescribeDBClusterParametersError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterParameters");
        params.put("Version", "2014-10-31");
        DescribeDBClusterParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBClusterParametersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBClusterParameterGroupDetails::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBClusterParameterGroupDetailsDeserializer::deserialize(
                "DescribeDBClusterParametersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot.</p> <p>When sharing snapshots with other AWS accounts, <code>DescribeDBClusterSnapshotAttributes</code> returns the <code>restore</code> attribute and a list of IDs for the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If <code>all</code> is included in the list of values for the <code>restore</code> attribute, then the manual DB cluster snapshot is public and can be copied or restored by all AWS accounts.</p> <p>To add or remove access for an AWS account to copy or restore a manual DB cluster snapshot, or to make the manual DB cluster snapshot public or private, use the <a>ModifyDBClusterSnapshotAttribute</a> API action.</p>
    async fn describe_db_cluster_snapshot_attributes(
        &self,
        input: DescribeDBClusterSnapshotAttributesMessage,
    ) -> Result<
        DescribeDBClusterSnapshotAttributesResult,
        RusotoError<DescribeDBClusterSnapshotAttributesError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterSnapshotAttributes");
        params.put("Version", "2014-10-31");
        DescribeDBClusterSnapshotAttributesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBClusterSnapshotAttributesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeDBClusterSnapshotAttributesResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeDBClusterSnapshotAttributesResultDeserializer::deserialize(
                "DescribeDBClusterSnapshotAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns information about DB cluster snapshots. This API action supports pagination.</p>
    async fn describe_db_cluster_snapshots(
        &self,
        input: DescribeDBClusterSnapshotsMessage,
    ) -> Result<DBClusterSnapshotMessage, RusotoError<DescribeDBClusterSnapshotsError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusterSnapshots");
        params.put("Version", "2014-10-31");
        DescribeDBClusterSnapshotsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBClusterSnapshotsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBClusterSnapshotMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBClusterSnapshotMessageDeserializer::deserialize(
                "DescribeDBClusterSnapshotsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns information about provisioned DB clusters. This API supports pagination.</p>
    async fn describe_db_clusters(
        &self,
        input: DescribeDBClustersMessage,
    ) -> Result<DBClusterMessage, RusotoError<DescribeDBClustersError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBClusters");
        params.put("Version", "2014-10-31");
        DescribeDBClustersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBClustersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBClusterMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                DBClusterMessageDeserializer::deserialize("DescribeDBClustersResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of the available DB engines.</p>
    async fn describe_db_engine_versions(
        &self,
        input: DescribeDBEngineVersionsMessage,
    ) -> Result<DBEngineVersionMessage, RusotoError<DescribeDBEngineVersionsError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBEngineVersions");
        params.put("Version", "2014-10-31");
        DescribeDBEngineVersionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBEngineVersionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBEngineVersionMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBEngineVersionMessageDeserializer::deserialize(
                "DescribeDBEngineVersionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns information about provisioned instances. This API supports pagination.</p>
    async fn describe_db_instances(
        &self,
        input: DescribeDBInstancesMessage,
    ) -> Result<DBInstanceMessage, RusotoError<DescribeDBInstancesError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBInstances");
        params.put("Version", "2014-10-31");
        DescribeDBInstancesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBInstancesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBInstanceMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBInstanceMessageDeserializer::deserialize(
                "DescribeDBInstancesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of <code>DBParameterGroup</code> descriptions. If a <code>DBParameterGroupName</code> is specified, the list will contain only the description of the specified DB parameter group.</p>
    async fn describe_db_parameter_groups(
        &self,
        input: DescribeDBParameterGroupsMessage,
    ) -> Result<DBParameterGroupsMessage, RusotoError<DescribeDBParameterGroupsError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBParameterGroups");
        params.put("Version", "2014-10-31");
        DescribeDBParameterGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBParameterGroupsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBParameterGroupsMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBParameterGroupsMessageDeserializer::deserialize(
                "DescribeDBParameterGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the detailed parameter list for a particular DB parameter group.</p>
    async fn describe_db_parameters(
        &self,
        input: DescribeDBParametersMessage,
    ) -> Result<DBParameterGroupDetails, RusotoError<DescribeDBParametersError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBParameters");
        params.put("Version", "2014-10-31");
        DescribeDBParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBParametersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBParameterGroupDetails::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBParameterGroupDetailsDeserializer::deserialize(
                "DescribeDBParametersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of DBSubnetGroup descriptions. If a DBSubnetGroupName is specified, the list will contain only the descriptions of the specified DBSubnetGroup.</p> <p>For an overview of CIDR ranges, go to the <a href="http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Wikipedia Tutorial</a>.</p>
    async fn describe_db_subnet_groups(
        &self,
        input: DescribeDBSubnetGroupsMessage,
    ) -> Result<DBSubnetGroupMessage, RusotoError<DescribeDBSubnetGroupsError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDBSubnetGroups");
        params.put("Version", "2014-10-31");
        DescribeDBSubnetGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeDBSubnetGroupsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBSubnetGroupMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBSubnetGroupMessageDeserializer::deserialize(
                "DescribeDBSubnetGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the default engine and system parameter information for the cluster database engine.</p>
    async fn describe_engine_default_cluster_parameters(
        &self,
        input: DescribeEngineDefaultClusterParametersMessage,
    ) -> Result<
        DescribeEngineDefaultClusterParametersResult,
        RusotoError<DescribeEngineDefaultClusterParametersError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEngineDefaultClusterParameters");
        params.put("Version", "2014-10-31");
        DescribeEngineDefaultClusterParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeEngineDefaultClusterParametersError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeEngineDefaultClusterParametersResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeEngineDefaultClusterParametersResultDeserializer::deserialize(
                "DescribeEngineDefaultClusterParametersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the default engine and system parameter information for the specified database engine.</p>
    async fn describe_engine_default_parameters(
        &self,
        input: DescribeEngineDefaultParametersMessage,
    ) -> Result<
        DescribeEngineDefaultParametersResult,
        RusotoError<DescribeEngineDefaultParametersError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEngineDefaultParameters");
        params.put("Version", "2014-10-31");
        DescribeEngineDefaultParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeEngineDefaultParametersError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeEngineDefaultParametersResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeEngineDefaultParametersResultDeserializer::deserialize(
                "DescribeEngineDefaultParametersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Displays a list of categories for all event source types, or, if specified, for a specified source type.</p>
    async fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> Result<EventCategoriesMessage, RusotoError<DescribeEventCategoriesError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEventCategories");
        params.put("Version", "2014-10-31");
        DescribeEventCategoriesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeEventCategoriesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = EventCategoriesMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = EventCategoriesMessageDeserializer::deserialize(
                "DescribeEventCategoriesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists all the subscription descriptions for a customer account. The description for a subscription includes SubscriptionName, SNSTopicARN, CustomerID, SourceType, SourceID, CreationTime, and Status.</p> <p>If you specify a SubscriptionName, lists the description for that subscription.</p>
    async fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> Result<EventSubscriptionsMessage, RusotoError<DescribeEventSubscriptionsError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEventSubscriptions");
        params.put("Version", "2014-10-31");
        DescribeEventSubscriptionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeEventSubscriptionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = EventSubscriptionsMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = EventSubscriptionsMessageDeserializer::deserialize(
                "DescribeEventSubscriptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns events related to DB instances, DB security groups, DB snapshots, and DB parameter groups for the past 14 days. Events specific to a particular DB instance, DB security group, database snapshot, or DB parameter group can be obtained by providing the name as a parameter. By default, the past hour of events are returned.</p>
    async fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> Result<EventsMessage, RusotoError<DescribeEventsError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEvents");
        params.put("Version", "2014-10-31");
        DescribeEventsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeEventsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = EventsMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = EventsMessageDeserializer::deserialize("DescribeEventsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of orderable DB instance options for the specified engine.</p>
    async fn describe_orderable_db_instance_options(
        &self,
        input: DescribeOrderableDBInstanceOptionsMessage,
    ) -> Result<
        OrderableDBInstanceOptionsMessage,
        RusotoError<DescribeOrderableDBInstanceOptionsError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeOrderableDBInstanceOptions");
        params.put("Version", "2014-10-31");
        DescribeOrderableDBInstanceOptionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeOrderableDBInstanceOptionsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = OrderableDBInstanceOptionsMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = OrderableDBInstanceOptionsMessageDeserializer::deserialize(
                "DescribeOrderableDBInstanceOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of resources (for example, DB instances) that have at least one pending maintenance action.</p>
    async fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> Result<PendingMaintenanceActionsMessage, RusotoError<DescribePendingMaintenanceActionsError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribePendingMaintenanceActions");
        params.put("Version", "2014-10-31");
        DescribePendingMaintenanceActionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribePendingMaintenanceActionsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PendingMaintenanceActionsMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PendingMaintenanceActionsMessageDeserializer::deserialize(
                "DescribePendingMaintenanceActionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>You can call <a>DescribeValidDBInstanceModifications</a> to learn what modifications you can make to your DB instance. You can use this information when you call <a>ModifyDBInstance</a>.</p>
    async fn describe_valid_db_instance_modifications(
        &self,
        input: DescribeValidDBInstanceModificationsMessage,
    ) -> Result<
        DescribeValidDBInstanceModificationsResult,
        RusotoError<DescribeValidDBInstanceModificationsError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeValidDBInstanceModifications");
        params.put("Version", "2014-10-31");
        DescribeValidDBInstanceModificationsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeValidDBInstanceModificationsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeValidDBInstanceModificationsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeValidDBInstanceModificationsResultDeserializer::deserialize(
                "DescribeValidDBInstanceModificationsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Forces a failover for a DB cluster.</p> <p>A failover for a DB cluster promotes one of the Read Replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer).</p> <p>Amazon Neptune will automatically fail over to a Read Replica, if one exists, when the primary instance fails. You can force a failover when you want to simulate a failure of a primary instance for testing. Because each instance in a DB cluster has its own endpoint address, you will need to clean up and re-establish any existing connections that use those endpoint addresses when the failover is complete.</p>
    async fn failover_db_cluster(
        &self,
        input: FailoverDBClusterMessage,
    ) -> Result<FailoverDBClusterResult, RusotoError<FailoverDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "FailoverDBCluster");
        params.put("Version", "2014-10-31");
        FailoverDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(FailoverDBClusterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = FailoverDBClusterResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = FailoverDBClusterResultDeserializer::deserialize(
                "FailoverDBClusterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists all tags on an Amazon Neptune resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> Result<TagListMessage, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTagsForResource");
        params.put("Version", "2014-10-31");
        ListTagsForResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTagsForResourceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = TagListMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                TagListMessageDeserializer::deserialize("ListTagsForResourceResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modify a setting for a DB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request.</p>
    async fn modify_db_cluster(
        &self,
        input: ModifyDBClusterMessage,
    ) -> Result<ModifyDBClusterResult, RusotoError<ModifyDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBCluster");
        params.put("Version", "2014-10-31");
        ModifyDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyDBClusterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyDBClusterResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyDBClusterResultDeserializer::deserialize(
                "ModifyDBClusterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p> Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request.</p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB cluster associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon Neptune to fully complete the create action before the parameter group is used as the default for a new DB cluster. This is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <a>DescribeDBClusterParameters</a> command to verify that your DB cluster parameter group has been created or modified.</p> </important></p>
    async fn modify_db_cluster_parameter_group(
        &self,
        input: ModifyDBClusterParameterGroupMessage,
    ) -> Result<DBClusterParameterGroupNameMessage, RusotoError<ModifyDBClusterParameterGroupError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        ModifyDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyDBClusterParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBClusterParameterGroupNameMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBClusterParameterGroupNameMessageDeserializer::deserialize(
                "ModifyDBClusterParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot.</p> <p>To share a manual DB cluster snapshot with other AWS accounts, specify <code>restore</code> as the <code>AttributeName</code> and use the <code>ValuesToAdd</code> parameter to add a list of IDs of the AWS accounts that are authorized to restore the manual DB cluster snapshot. Use the value <code>all</code> to make the manual DB cluster snapshot public, which means that it can be copied or restored by all AWS accounts. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts. If a manual DB cluster snapshot is encrypted, it can be shared, but only by specifying a list of authorized AWS account IDs for the <code>ValuesToAdd</code> parameter. You can't use <code>all</code> as a value for that parameter in this case.</p> <p>To view which AWS accounts have access to copy or restore a manual DB cluster snapshot, or whether a manual DB cluster snapshot public or private, use the <a>DescribeDBClusterSnapshotAttributes</a> API action.</p>
    async fn modify_db_cluster_snapshot_attribute(
        &self,
        input: ModifyDBClusterSnapshotAttributeMessage,
    ) -> Result<
        ModifyDBClusterSnapshotAttributeResult,
        RusotoError<ModifyDBClusterSnapshotAttributeError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBClusterSnapshotAttribute");
        params.put("Version", "2014-10-31");
        ModifyDBClusterSnapshotAttributeMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyDBClusterSnapshotAttributeError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyDBClusterSnapshotAttributeResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyDBClusterSnapshotAttributeResultDeserializer::deserialize(
                "ModifyDBClusterSnapshotAttributeResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. To learn what modifications you can make to your DB instance, call <a>DescribeValidDBInstanceModifications</a> before you call <a>ModifyDBInstance</a>.</p>
    async fn modify_db_instance(
        &self,
        input: ModifyDBInstanceMessage,
    ) -> Result<ModifyDBInstanceResult, RusotoError<ModifyDBInstanceError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBInstance");
        params.put("Version", "2014-10-31");
        ModifyDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyDBInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyDBInstanceResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyDBInstanceResultDeserializer::deserialize(
                "ModifyDBInstanceResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Modifies the parameters of a DB parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request.</p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot without failover to the DB instance associated with the parameter group before the change can take effect.</p> </note> <important> <p>After you modify a DB parameter group, you should wait at least 5 minutes before creating your first DB instance that uses that DB parameter group as the default parameter group. This allows Amazon Neptune to fully complete the modify action before the parameter group is used as the default for a new DB instance. This is especially important for parameters that are critical when creating the default database for a DB instance, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter. You can use the <i>Parameter Groups</i> option of the Amazon Neptune console or the <i>DescribeDBParameters</i> command to verify that your DB parameter group has been created or modified.</p> </important></p>
    async fn modify_db_parameter_group(
        &self,
        input: ModifyDBParameterGroupMessage,
    ) -> Result<DBParameterGroupNameMessage, RusotoError<ModifyDBParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBParameterGroup");
        params.put("Version", "2014-10-31");
        ModifyDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyDBParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBParameterGroupNameMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBParameterGroupNameMessageDeserializer::deserialize(
                "ModifyDBParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the AWS Region.</p>
    async fn modify_db_subnet_group(
        &self,
        input: ModifyDBSubnetGroupMessage,
    ) -> Result<ModifyDBSubnetGroupResult, RusotoError<ModifyDBSubnetGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyDBSubnetGroup");
        params.put("Version", "2014-10-31");
        ModifyDBSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyDBSubnetGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyDBSubnetGroupResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyDBSubnetGroupResultDeserializer::deserialize(
                "ModifyDBSubnetGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modifies an existing event notification subscription. Note that you can't modify the source identifiers using this call; to change source identifiers for a subscription, use the <a>AddSourceIdentifierToSubscription</a> and <a>RemoveSourceIdentifierFromSubscription</a> calls.</p> <p>You can see a list of the event categories for a given SourceType by using the <b>DescribeEventCategories</b> action.</p>
    async fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> Result<ModifyEventSubscriptionResult, RusotoError<ModifyEventSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyEventSubscription");
        params.put("Version", "2014-10-31");
        ModifyEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyEventSubscriptionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyEventSubscriptionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyEventSubscriptionResultDeserializer::deserialize(
                "ModifyEventSubscriptionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Not supported.</p>
    async fn promote_read_replica_db_cluster(
        &self,
        input: PromoteReadReplicaDBClusterMessage,
    ) -> Result<PromoteReadReplicaDBClusterResult, RusotoError<PromoteReadReplicaDBClusterError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PromoteReadReplicaDBCluster");
        params.put("Version", "2014-10-31");
        PromoteReadReplicaDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PromoteReadReplicaDBClusterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PromoteReadReplicaDBClusterResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PromoteReadReplicaDBClusterResultDeserializer::deserialize(
                "PromoteReadReplicaDBClusterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB parameter group associated with the DB instance, you must reboot the instance for the changes to take effect.</p> <p>Rebooting a DB instance restarts the database engine service. Rebooting a DB instance results in a momentary outage, during which the DB instance status is set to rebooting.</p>
    async fn reboot_db_instance(
        &self,
        input: RebootDBInstanceMessage,
    ) -> Result<RebootDBInstanceResult, RusotoError<RebootDBInstanceError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RebootDBInstance");
        params.put("Version", "2014-10-31");
        RebootDBInstanceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RebootDBInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RebootDBInstanceResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RebootDBInstanceResultDeserializer::deserialize(
                "RebootDBInstanceResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Disassociates an Identity and Access Management (IAM) role from a DB cluster.</p>
    async fn remove_role_from_db_cluster(
        &self,
        input: RemoveRoleFromDBClusterMessage,
    ) -> Result<(), RusotoError<RemoveRoleFromDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveRoleFromDBCluster");
        params.put("Version", "2014-10-31");
        RemoveRoleFromDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RemoveRoleFromDBClusterError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Removes a source identifier from an existing event notification subscription.</p>
    async fn remove_source_identifier_from_subscription(
        &self,
        input: RemoveSourceIdentifierFromSubscriptionMessage,
    ) -> Result<
        RemoveSourceIdentifierFromSubscriptionResult,
        RusotoError<RemoveSourceIdentifierFromSubscriptionError>,
    > {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveSourceIdentifierFromSubscription");
        params.put("Version", "2014-10-31");
        RemoveSourceIdentifierFromSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RemoveSourceIdentifierFromSubscriptionError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RemoveSourceIdentifierFromSubscriptionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RemoveSourceIdentifierFromSubscriptionResultDeserializer::deserialize(
                "RemoveSourceIdentifierFromSubscriptionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Removes metadata tags from an Amazon Neptune resource.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> Result<(), RusotoError<RemoveTagsFromResourceError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveTagsFromResource");
        params.put("Version", "2014-10-31");
        RemoveTagsFromResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RemoveTagsFromResourceError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p> Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters submit a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB cluster parameter group, specify the <code>DBClusterParameterGroupName</code> and <code>ResetAllParameters</code> parameters.</p> <p> When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <a>RebootDBInstance</a> request. You must call <a>RebootDBInstance</a> for every DB instance in your DB cluster that you want the updated static parameter to apply to.</p>
    async fn reset_db_cluster_parameter_group(
        &self,
        input: ResetDBClusterParameterGroupMessage,
    ) -> Result<DBClusterParameterGroupNameMessage, RusotoError<ResetDBClusterParameterGroupError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResetDBClusterParameterGroup");
        params.put("Version", "2014-10-31");
        ResetDBClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ResetDBClusterParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBClusterParameterGroupNameMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBClusterParameterGroupNameMessageDeserializer::deserialize(
                "ResetDBClusterParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the parameters of a DB parameter group to the engine/system default value. To reset specific parameters, provide a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB parameter group, specify the <code>DBParameterGroup</code> name and <code>ResetAllParameters</code> parameters. When resetting the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance restart or <code>RebootDBInstance</code> request.</p>
    async fn reset_db_parameter_group(
        &self,
        input: ResetDBParameterGroupMessage,
    ) -> Result<DBParameterGroupNameMessage, RusotoError<ResetDBParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResetDBParameterGroup");
        params.put("Version", "2014-10-31");
        ResetDBParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ResetDBParameterGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DBParameterGroupNameMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DBParameterGroupNameMessageDeserializer::deserialize(
                "ResetDBParameterGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.</p> <p>If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.</p>
    async fn restore_db_cluster_from_snapshot(
        &self,
        input: RestoreDBClusterFromSnapshotMessage,
    ) -> Result<RestoreDBClusterFromSnapshotResult, RusotoError<RestoreDBClusterFromSnapshotError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RestoreDBClusterFromSnapshot");
        params.put("Version", "2014-10-31");
        RestoreDBClusterFromSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RestoreDBClusterFromSnapshotError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RestoreDBClusterFromSnapshotResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RestoreDBClusterFromSnapshotResultDeserializer::deserialize(
                "RestoreDBClusterFromSnapshotResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before <code>LatestRestorableTime</code> for up to <code>BackupRetentionPeriod</code> days. The target DB cluster is created from the source DB cluster with the same configuration as the original DB cluster, except that the new DB cluster is created with the default DB security group.</p> <note> <p>This action only restores the DB cluster, not the DB instances for that DB cluster. You must invoke the <a>CreateDBInstance</a> action to create DB instances for the restored DB cluster, specifying the identifier of the restored DB cluster in <code>DBClusterIdentifier</code>. You can create DB instances only after the <code>RestoreDBClusterToPointInTime</code> action has completed and the DB cluster is available.</p> </note></p>
    async fn restore_db_cluster_to_point_in_time(
        &self,
        input: RestoreDBClusterToPointInTimeMessage,
    ) -> Result<RestoreDBClusterToPointInTimeResult, RusotoError<RestoreDBClusterToPointInTimeError>>
    {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RestoreDBClusterToPointInTime");
        params.put("Version", "2014-10-31");
        RestoreDBClusterToPointInTimeMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RestoreDBClusterToPointInTimeError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RestoreDBClusterToPointInTimeResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RestoreDBClusterToPointInTimeResultDeserializer::deserialize(
                "RestoreDBClusterToPointInTimeResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}
