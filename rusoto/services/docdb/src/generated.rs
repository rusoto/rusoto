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

/// <p>Represents the input to <a>AddTagsToResource</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToResourceMessage {
    /// <p>The Amazon DocumentDB resource that the tags are added to. This value is an Amazon Resource Name (ARN).</p>
    pub resource_name: String,
    /// <p>The tags to be assigned to the Amazon DocumentDB resource. </p>
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
/// <p>Represents the input to <a>ApplyPendingMaintenanceAction</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplyPendingMaintenanceActionMessage {
    /// <p>The pending maintenance action to apply to this resource.</p> <p>Valid values: <code>system-update</code>, <code>db-upgrade</code> </p>
    pub apply_action: String,
    /// <p><p>A value that specifies the type of opt-in request or undoes an opt-in request. An opt-in request of type <code>immediate</code> can&#39;t be undone.</p> <p>Valid values:</p> <ul> <li> <p> <code>immediate</code> - Apply the maintenance action immediately.</p> </li> <li> <p> <code>next-maintenance</code> - Apply the maintenance action during the next maintenance window for the resource.</p> </li> <li> <p> <code>undo-opt-in</code> - Cancel any existing <code>next-maintenance</code> opt-in requests.</p> </li> </ul></p>
    pub opt_in_type: String,
    /// <p>The Amazon Resource Name (ARN) of the resource that the pending maintenance action applies to.</p>
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

/// <p>Information about an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>The name of the Availability Zone.</p>
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
/// <p>A certificate authority (CA) certificate for an AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Certificate {
    /// <p>The Amazon Resource Name (ARN) for the certificate.</p> <p>Example: <code>arn:aws:rds:us-east-1::cert:rds-ca-2019</code> </p>
    pub certificate_arn: Option<String>,
    /// <p>The unique key that identifies a certificate.</p> <p>Example: <code>rds-ca-2019</code> </p>
    pub certificate_identifier: Option<String>,
    /// <p>The type of the certificate.</p> <p>Example: <code>CA</code> </p>
    pub certificate_type: Option<String>,
    /// <p>The thumbprint of the certificate.</p>
    pub thumbprint: Option<String>,
    /// <p>The starting date-time from which the certificate is valid.</p> <p>Example: <code>2019-07-31T17:57:09Z</code> </p>
    pub valid_from: Option<String>,
    /// <p>The date-time after which the certificate is no longer valid.</p> <p>Example: <code>2024-07-31T17:57:09Z</code> </p>
    pub valid_till: Option<String>,
}

struct CertificateDeserializer;
impl CertificateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Certificate, XmlParseError> {
        deserialize_elements::<_, Certificate, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CertificateArn" => {
                    obj.certificate_arn =
                        Some(StringDeserializer::deserialize("CertificateArn", stack)?);
                }
                "CertificateIdentifier" => {
                    obj.certificate_identifier = Some(StringDeserializer::deserialize(
                        "CertificateIdentifier",
                        stack,
                    )?);
                }
                "CertificateType" => {
                    obj.certificate_type =
                        Some(StringDeserializer::deserialize("CertificateType", stack)?);
                }
                "Thumbprint" => {
                    obj.thumbprint = Some(StringDeserializer::deserialize("Thumbprint", stack)?);
                }
                "ValidFrom" => {
                    obj.valid_from = Some(TStampDeserializer::deserialize("ValidFrom", stack)?);
                }
                "ValidTill" => {
                    obj.valid_till = Some(TStampDeserializer::deserialize("ValidTill", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct CertificateListDeserializer;
impl CertificateListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Certificate>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Certificate" {
                obj.push(CertificateDeserializer::deserialize("Certificate", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CertificateMessage {
    /// <p>A list of certificates for this AWS account.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>An optional pagination token provided if the number of records retrieved is greater than <code>MaxRecords</code>. If this parameter is specified, the marker specifies the next record in the list. Including the value of <code>Marker</code> in the next call to <code>DescribeCertificates</code> results in the next page of certificates.</p>
    pub marker: Option<String>,
}

struct CertificateMessageDeserializer;
impl CertificateMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CertificateMessage, XmlParseError> {
        deserialize_elements::<_, CertificateMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Certificates" => {
                    obj.certificates.get_or_insert(vec![]).extend(
                        CertificateListDeserializer::deserialize("Certificates", stack)?,
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
/// <p>The configuration setting for the log types to be enabled for export to Amazon CloudWatch Logs for a specific DB instance or DB cluster.</p> <p>The <code>EnableLogTypes</code> and <code>DisableLogTypes</code> arrays determine which logs are exported (or not exported) to CloudWatch Logs. The values within these arrays depend on the DB engine that is being used.</p>
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

/// <p>Represents the input to <a>CopyDBClusterParameterGroup</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyDBClusterParameterGroupMessage {
    /// <p><p>The identifier or Amazon Resource Name (ARN) for the source DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid DB cluster parameter group.</p> </li> <li> <p>If the source DB cluster parameter group is in the same AWS Region as the copy, specify a valid DB parameter group identifier; for example, <code>my-db-cluster-param-group</code>, or a valid ARN.</p> </li> <li> <p>If the source DB parameter group is in a different AWS Region than the copy, specify a valid DB cluster parameter group ARN; for example, <code>arn:aws:rds:us-east-1:123456789012:cluster-pg:custom-cluster-group1</code>.</p> </li> </ul></p>
    pub source_db_cluster_parameter_group_identifier: String,
    /// <p>The tags that are to be assigned to the parameter group.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A description for the copied DB cluster parameter group.</p>
    pub target_db_cluster_parameter_group_description: String,
    /// <p>The identifier for the copied DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank.</p> </li> <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster-param-group1</code> </p>
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
/// <p>Represents the input to <a>CopyDBClusterSnapshot</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopyDBClusterSnapshotMessage {
    /// <p>Set to <code>true</code> to copy all tags from the source DB cluster snapshot to the target DB cluster snapshot, and otherwise <code>false</code>. The default is <code>false</code>.</p>
    pub copy_tags: Option<bool>,
    /// <p>The AWS KMS key ID for an encrypted DB cluster snapshot. The AWS KMS key ID is the Amazon Resource Name (ARN), AWS KMS key identifier, or the AWS KMS key alias for the AWS KMS encryption key. </p> <p>If you copy an encrypted DB cluster snapshot from your AWS account, you can specify a value for <code>KmsKeyId</code> to encrypt the copy with a new AWS KMS encryption key. If you don't specify a value for <code>KmsKeyId</code>, then the copy of the DB cluster snapshot is encrypted with the same AWS KMS key as the source DB cluster snapshot. </p> <p>If you copy an encrypted DB cluster snapshot that is shared from another AWS account, then you must specify a value for <code>KmsKeyId</code>. </p> <p>To copy an encrypted DB cluster snapshot to another AWS Region, set <code>KmsKeyId</code> to the AWS KMS key ID that you want to use to encrypt the copy of the DB cluster snapshot in the destination Region. AWS KMS encryption keys are specific to the AWS Region that they are created in, and you can't use encryption keys from one Region in another Region.</p> <p>If you copy an unencrypted DB cluster snapshot and specify a value for the <code>KmsKeyId</code> parameter, an error is returned.</p>
    pub kms_key_id: Option<String>,
    /// <p><p>The URL that contains a Signature Version 4 signed request for the <code>CopyDBClusterSnapshot</code> API action in the AWS Region that contains the source DB cluster snapshot to copy. You must use the <code>PreSignedUrl</code> parameter when copying an encrypted DB cluster snapshot from another AWS Region.</p> <p>The presigned URL must be a valid request for the <code>CopyDBSClusterSnapshot</code> API action that can be executed in the source AWS Region that contains the encrypted DB cluster snapshot to be copied. The presigned URL request must contain the following parameter values:</p> <ul> <li> <p> <code>KmsKeyId</code> - The AWS KMS key identifier for the key to use to encrypt the copy of the DB cluster snapshot in the destination AWS Region. This is the same identifier for both the <code>CopyDBClusterSnapshot</code> action that is called in the destination AWS Region, and the action contained in the presigned URL.</p> </li> <li> <p> <code>DestinationRegion</code> - The name of the AWS Region that the DB cluster snapshot will be created in.</p> </li> <li> <p> <code>SourceDBClusterSnapshotIdentifier</code> - The DB cluster snapshot identifier for the encrypted DB cluster snapshot to be copied. This identifier must be in the Amazon Resource Name (ARN) format for the source AWS Region. For example, if you are copying an encrypted DB cluster snapshot from the us-west-2 AWS Region, then your <code>SourceDBClusterSnapshotIdentifier</code> looks like the following example: <code>arn:aws:rds:us-west-2:123456789012:cluster-snapshot:my-cluster-snapshot-20161115</code>.</p> </li> </ul></p>
    pub pre_signed_url: Option<String>,
    /// <p>The identifier of the DB cluster snapshot to copy. This parameter is not case sensitive.</p> <p>You can't copy an encrypted, shared DB cluster snapshot from one AWS Region to another.</p> <p>Constraints:</p> <ul> <li> <p>Must specify a valid system snapshot in the "available" state.</p> </li> <li> <p>If the source snapshot is in the same AWS Region as the copy, specify a valid DB snapshot identifier.</p> </li> <li> <p>If the source snapshot is in a different AWS Region than the copy, specify a valid DB cluster snapshot ARN.</p> </li> </ul> <p>Example: <code>my-cluster-snapshot1</code> </p>
    pub source_db_cluster_snapshot_identifier: String,
    /// <p>The tags to be assigned to the DB cluster snapshot.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The identifier of the new DB cluster snapshot to create from the source DB cluster snapshot. This parameter is not case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster-snapshot2</code> </p>
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
/// <p>Represents the input to <a>CreateDBCluster</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBClusterMessage {
    /// <p>A list of Amazon EC2 Availability Zones that instances in the DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p><p>The number of days for which automated backups are retained. You must specify a minimum value of 1.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 1 to 35.</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p>The DB cluster identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster</code> </p>
    pub db_cluster_identifier: String,
    /// <p> The name of the DB cluster parameter group to associate with this DB cluster.</p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>A DB subnet group to associate with this DB cluster.</p> <p>Constraints: Must match the name of an existing <code>DBSubnetGroup</code>. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Specifies whether this cluster can be deleted. If <code>DeletionProtection</code> is enabled, the cluster cannot be deleted unless it is modified and <code>DeletionProtection</code> is disabled. <code>DeletionProtection</code> protects clusters from being accidentally deleted.</p>
    pub deletion_protection: Option<bool>,
    /// <p>A list of log types that need to be enabled for exporting to Amazon CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>The name of the database engine to be used for this DB cluster.</p> <p>Valid values: <code>docdb</code> </p>
    pub engine: String,
    /// <p>The version number of the database engine to use.</p>
    pub engine_version: Option<String>,
    /// <p>The AWS KMS key identifier for an encrypted DB cluster.</p> <p>The AWS KMS key identifier is the Amazon Resource Name (ARN) for the AWS KMS encryption key. If you are creating a DB cluster using the same AWS account that owns the AWS KMS encryption key that is used to encrypt the new DB cluster, you can use the AWS KMS key alias instead of the ARN for the AWS KMS encryption key.</p> <p>If an encryption key is not specified in <code>KmsKeyId</code>:</p> <ul> <li> <p>If <code>ReplicationSourceIdentifier</code> identifies an encrypted source, then Amazon DocumentDB uses the encryption key that is used to encrypt the source. Otherwise, Amazon DocumentDB uses your default encryption key. </p> </li> <li> <p>If the <code>StorageEncrypted</code> parameter is <code>true</code> and <code>ReplicationSourceIdentifier</code> is not specified, Amazon DocumentDB uses your default encryption key.</p> </li> </ul> <p>AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p> <p>If you create a replica of an encrypted DB cluster in another AWS Region, you must set <code>KmsKeyId</code> to a KMS key ID that is valid in the destination AWS Region. This key is used to encrypt the replica in that AWS Region.</p>
    pub kms_key_id: Option<String>,
    /// <p>The password for the master database user. This password can contain any printable ASCII character except forward slash (/), double quote ("), or the "at" symbol (@).</p> <p>Constraints: Must contain from 8 to 100 characters.</p>
    pub master_user_password: String,
    /// <p><p>The name of the master user for the DB cluster.</p> <p>Constraints:</p> <ul> <li> <p>Must be from 1 to 63 letters or numbers.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot be a reserved word for the chosen database engine.</p> </li> </ul></p>
    pub master_username: String,
    /// <p>The port number on which the instances in the DB cluster accept connections.</p>
    pub port: Option<i64>,
    /// <p><p>The daily time range during which automated backups are created if automated backups are enabled using the <code>BackupRetentionPeriod</code> parameter. </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. </p> <p>Constraints:</p> <ul> <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li> <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>Specifies whether the DB cluster is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>The tags to be assigned to the DB cluster.</p>
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
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "MasterUserPassword"),
            &obj.master_user_password,
        );
        params.put(
            &format!("{}{}", prefix, "MasterUsername"),
            &obj.master_username,
        );
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

/// <p>Represents the input of <a>CreateDBClusterParameterGroup</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBClusterParameterGroupMessage {
    /// <p><p>The name of the DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing <code>DBClusterParameterGroup</code>.</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
    pub db_cluster_parameter_group_name: String,
    /// <p>The DB cluster parameter group family name.</p>
    pub db_parameter_group_family: String,
    /// <p>The description for the DB cluster parameter group.</p>
    pub description: String,
    /// <p>The tags to be assigned to the DB cluster parameter group.</p>
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
/// <p>Represents the input of <a>CreateDBClusterSnapshot</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBClusterSnapshotMessage {
    /// <p>The identifier of the DB cluster to create a snapshot for. This parameter is not case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing <code>DBCluster</code>.</p> </li> </ul> <p>Example: <code>my-cluster</code> </p>
    pub db_cluster_identifier: String,
    /// <p>The identifier of the DB cluster snapshot. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster-snapshot1</code> </p>
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
/// <p>Represents the input to <a>CreateDBInstance</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBInstanceMessage {
    /// <p>Indicates that minor engine upgrades are applied automatically to the DB instance during the maintenance window.</p> <p>Default: <code>true</code> </p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p> The Amazon EC2 Availability Zone that the DB instance is created in.</p> <p>Default: A random, system-chosen Availability Zone in the endpoint's AWS Region.</p> <p> Example: <code>us-east-1d</code> </p> <p> Constraint: The <code>AvailabilityZone</code> parameter can't be specified if the <code>MultiAZ</code> parameter is set to <code>true</code>. The specified Availability Zone must be in the same AWS Region as the current endpoint. </p>
    pub availability_zone: Option<String>,
    /// <p>The identifier of the DB cluster that the instance will belong to.</p>
    pub db_cluster_identifier: String,
    /// <p>The compute and memory capacity of the DB instance; for example, <code>db.r5.large</code>. </p>
    pub db_instance_class: String,
    /// <p>The DB instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>mydbinstance</code> </p>
    pub db_instance_identifier: String,
    /// <p>The name of the database engine to be used for this instance.</p> <p>Valid value: <code>docdb</code> </p>
    pub engine: String,
    /// <p>The time range each week during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week. </p> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which an Amazon DocumentDB replica is promoted to the primary instance after a failure of the existing primary instance.</p> <p>Default: 1</p> <p>Valid values: 0-15</p>
    pub promotion_tier: Option<i64>,
    /// <p>The tags to be assigned to the DB instance. You can assign up to 10 tags to an instance.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateDBInstanceMessage` contents to a `SignedRequest`.
struct CreateDBInstanceMessageSerializer;
impl CreateDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
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
        params.put(
            &format!("{}{}", prefix, "DBClusterIdentifier"),
            &obj.db_cluster_identifier,
        );
        params.put(
            &format!("{}{}", prefix, "DBInstanceClass"),
            &obj.db_instance_class,
        );
        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.promotion_tier {
            params.put(&format!("{}{}", prefix, "PromotionTier"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
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
/// <p>Represents the input to <a>CreateDBSubnetGroup</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDBSubnetGroupMessage {
    /// <p>The description for the DB subnet group.</p>
    pub db_subnet_group_description: String,
    /// <p>The name for the DB subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 letters, numbers, periods, underscores, spaces, or hyphens. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
    /// <p>The Amazon EC2 subnet IDs for the DB subnet group.</p>
    pub subnet_ids: Vec<String>,
    /// <p>The tags to be assigned to the DB subnet group.</p>
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
/// <p>Detailed information about a DB cluster. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBCluster {
    /// <p>Provides a list of the AWS Identity and Access Management (IAM) roles that are associated with the DB cluster. IAM roles that are associated with a DB cluster grant permission for the DB cluster to access other AWS services on your behalf.</p>
    pub associated_roles: Option<Vec<DBClusterRole>>,
    /// <p>Provides the list of Amazon EC2 Availability Zones that instances in the DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Specifies the number of days for which automatic DB snapshots are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>Specifies the time when the DB cluster was created, in Universal Coordinated Time (UTC).</p>
    pub cluster_create_time: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB cluster.</p>
    pub db_cluster_arn: Option<String>,
    /// <p>Contains a user-supplied DB cluster identifier. This identifier is the unique key that identifies a DB cluster.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>Provides the list of instances that make up the DB cluster.</p>
    pub db_cluster_members: Option<Vec<DBClusterMember>>,
    /// <p>Specifies the name of the DB cluster parameter group for the DB cluster.</p>
    pub db_cluster_parameter_group: Option<String>,
    /// <p>Specifies information on the subnet group that is associated with the DB cluster, including the name, description, and subnets in the subnet group.</p>
    pub db_subnet_group: Option<String>,
    /// <p>The AWS Region-unique, immutable identifier for the DB cluster. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB cluster is accessed.</p>
    pub db_cluster_resource_id: Option<String>,
    /// <p>Specifies whether this cluster can be deleted. If <code>DeletionProtection</code> is enabled, the cluster cannot be deleted unless it is modified and <code>DeletionProtection</code> is disabled. <code>DeletionProtection</code> protects clusters from being accidentally deleted.</p>
    pub deletion_protection: Option<bool>,
    /// <p>The earliest time to which a database can be restored with point-in-time restore.</p>
    pub earliest_restorable_time: Option<String>,
    /// <p>A list of log types that this DB cluster is configured to export to Amazon CloudWatch Logs.</p>
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>Specifies the connection endpoint for the primary instance of the DB cluster.</p>
    pub endpoint: Option<String>,
    /// <p>Provides the name of the database engine to be used for this DB cluster.</p>
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>If <code>StorageEncrypted</code> is <code>true</code>, the AWS KMS key identifier for the encrypted DB cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p>Specifies the latest time to which a database can be restored with point-in-time restore.</p>
    pub latest_restorable_time: Option<String>,
    /// <p>Contains the master user name for the DB cluster.</p>
    pub master_username: Option<String>,
    /// <p>Specifies whether the DB cluster has instances in multiple Availability Zones.</p>
    pub multi_az: Option<bool>,
    /// <p>Specifies the progress of the operation as a percentage.</p>
    pub percent_progress: Option<String>,
    /// <p>Specifies the port that the database engine is listening on.</p>
    pub port: Option<i64>,
    /// <p>Specifies the daily time range during which automated backups are created if automated backups are enabled, as determined by the <code>BackupRetentionPeriod</code>. </p>
    pub preferred_backup_window: Option<String>,
    /// <p>Specifies the weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>The reader endpoint for the DB cluster. The reader endpoint for a DB cluster load balances connections across the Amazon DocumentDB replicas that are available in a DB cluster. As clients request new connections to the reader endpoint, Amazon DocumentDB distributes the connection requests among the Amazon DocumentDB replicas in the DB cluster. This functionality can help balance your read workload across multiple Amazon DocumentDB replicas in your DB cluster. </p> <p>If a failover occurs, and the Amazon DocumentDB replica that you are connected to is promoted to be the primary instance, your connection is dropped. To continue sending your read workload to other Amazon DocumentDB replicas in the cluster, you can then reconnect to the reader endpoint.</p>
    pub reader_endpoint: Option<String>,
    /// <p>Specifies the current state of this DB cluster.</p>
    pub status: Option<String>,
    /// <p>Specifies whether the DB cluster is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Provides a list of virtual private cloud (VPC) security groups that the DB cluster belongs to.</p>
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
                "DbClusterResourceId" => {
                    obj.db_cluster_resource_id = Some(StringDeserializer::deserialize(
                        "DbClusterResourceId",
                        stack,
                    )?);
                }
                "DeletionProtection" => {
                    obj.deletion_protection = Some(BooleanDeserializer::deserialize(
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
                "ReaderEndpoint" => {
                    obj.reader_endpoint =
                        Some(StringDeserializer::deserialize("ReaderEndpoint", stack)?);
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
    /// <p>A value that is <code>true</code> if the cluster member is the primary instance for the DB cluster and <code>false</code> otherwise.</p>
    pub is_cluster_writer: Option<bool>,
    /// <p>A value that specifies the order in which an Amazon DocumentDB replica is promoted to the primary instance after a failure of the existing primary instance. </p>
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
/// <p>Represents the output of <a>DescribeDBClusters</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterMessage {
    /// <p>A list of DB clusters.</p>
    pub db_clusters: Option<Vec<DBCluster>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
/// <p>Detailed information about a DB cluster parameter group. </p>
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
/// <p>Represents the output of <a>DBClusterParameterGroup</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterParameterGroupDetails {
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
/// <p>Contains the name of a DB cluster parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterParameterGroupNameMessage {
    /// <p><p>The name of a DB cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be from 1 to 255 letters or numbers.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <note> <p>This value is stored as a lowercase string.</p> </note></p>
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
/// <p>Represents the output of <a>DBClusterParameterGroups</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterParameterGroupsMessage {
    /// <p>A list of DB cluster parameter groups.</p>
    pub db_cluster_parameter_groups: Option<Vec<DBClusterParameterGroup>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
    /// <p><p>Describes the state of association between the IAM role and the DB cluster. The <code>Status</code> property returns one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - The IAM role ARN is associated with the DB cluster and can be used to access other AWS services on your behalf.</p> </li> <li> <p> <code>PENDING</code> - The IAM role ARN is being associated with the DB cluster.</p> </li> <li> <p> <code>INVALID</code> - The IAM role ARN is associated with the DB cluster, but the DB cluster cannot assume the IAM role to access other AWS services on your behalf.</p> </li> </ul></p>
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
/// <p>Detailed information about a DB cluster snapshot. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshot {
    /// <p>Provides the list of Amazon EC2 Availability Zones that instances in the DB cluster snapshot can be restored in.</p>
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
    /// <p>If <code>StorageEncrypted</code> is <code>true</code>, the AWS KMS key identifier for the encrypted DB cluster snapshot.</p>
    pub kms_key_id: Option<String>,
    /// <p>Provides the master user name for the DB cluster snapshot.</p>
    pub master_username: Option<String>,
    /// <p>Specifies the percentage of the estimated data that has been transferred.</p>
    pub percent_progress: Option<i64>,
    /// <p>Specifies the port that the DB cluster was listening on at the time of the snapshot.</p>
    pub port: Option<i64>,
    /// <p>Provides the time when the snapshot was taken, in UTC.</p>
    pub snapshot_create_time: Option<String>,
    /// <p>Provides the type of the DB cluster snapshot.</p>
    pub snapshot_type: Option<String>,
    /// <p>If the DB cluster snapshot was copied from a source DB cluster snapshot, the ARN for the source DB cluster snapshot; otherwise, a null value.</p>
    pub source_db_cluster_snapshot_arn: Option<String>,
    /// <p>Specifies the status of this DB cluster snapshot.</p>
    pub status: Option<String>,
    /// <p>Specifies whether the DB cluster snapshot is encrypted.</p>
    pub storage_encrypted: Option<bool>,
    /// <p>Provides the virtual private cloud (VPC) ID that is associated with the DB cluster snapshot.</p>
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
                "KmsKeyId" => {
                    obj.kms_key_id = Some(StringDeserializer::deserialize("KmsKeyId", stack)?);
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
/// <p>Contains the name and values of a manual DB cluster snapshot attribute.</p> <p>Manual DB cluster snapshot attributes are used to authorize other AWS accounts to restore a manual DB cluster snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshotAttribute {
    /// <p>The name of the manual DB cluster snapshot attribute.</p> <p>The attribute named <code>restore</code> refers to the list of AWS accounts that have permission to copy or restore the manual DB cluster snapshot.</p>
    pub attribute_name: Option<String>,
    /// <p>The values for the manual DB cluster snapshot attribute.</p> <p>If the <code>AttributeName</code> field is set to <code>restore</code>, then this element returns a list of IDs of the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If a value of <code>all</code> is in the list, then the manual DB cluster snapshot is public and available for any AWS account to copy or restore.</p>
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
/// <p>Detailed information about the attributes that are associated with a DB cluster snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshotAttributesResult {
    /// <p>The list of attributes and values for the DB cluster snapshot.</p>
    pub db_cluster_snapshot_attributes: Option<Vec<DBClusterSnapshotAttribute>>,
    /// <p>The identifier of the DB cluster snapshot that the attributes apply to.</p>
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
/// <p>Represents the output of <a>DescribeDBClusterSnapshots</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBClusterSnapshotMessage {
    /// <p>Provides a list of DB cluster snapshots.</p>
    pub db_cluster_snapshots: Option<Vec<DBClusterSnapshot>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
/// <p> Detailed information about a DB engine version. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBEngineVersion {
    /// <p>The description of the database engine.</p>
    pub db_engine_description: Option<String>,
    /// <p>The description of the database engine version.</p>
    pub db_engine_version_description: Option<String>,
    /// <p>The name of the DB parameter group family for the database engine.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p>The name of the database engine.</p>
    pub engine: Option<String>,
    /// <p>The version number of the database engine.</p>
    pub engine_version: Option<String>,
    /// <p>The types of logs that the database engine has available for export to Amazon CloudWatch Logs.</p>
    pub exportable_log_types: Option<Vec<String>>,
    /// <p>A value that indicates whether the engine version supports exporting the log types specified by <code>ExportableLogTypes</code> to CloudWatch Logs.</p>
    pub supports_log_exports_to_cloudwatch_logs: Option<bool>,
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
                "SupportsLogExportsToCloudwatchLogs" => {
                    obj.supports_log_exports_to_cloudwatch_logs =
                        Some(BooleanDeserializer::deserialize(
                            "SupportsLogExportsToCloudwatchLogs",
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
/// <p>Represents the output of <a>DescribeDBEngineVersions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBEngineVersionMessage {
    /// <p>Detailed information about one or more DB engine versions.</p>
    pub db_engine_versions: Option<Vec<DBEngineVersion>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
/// <p>Detailed information about a DB instance. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBInstance {
    /// <p>Indicates that minor version patches are applied automatically.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Specifies the name of the Availability Zone that the DB instance is located in.</p>
    pub availability_zone: Option<String>,
    /// <p>Specifies the number of days for which automatic DB snapshots are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>The identifier of the CA certificate for this DB instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p>Contains the name of the DB cluster that the DB instance is a member of if the DB instance is a member of a DB cluster.</p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the DB instance.</p>
    pub db_instance_arn: Option<String>,
    /// <p>Contains the name of the compute and memory capacity class of the DB instance.</p>
    pub db_instance_class: Option<String>,
    /// <p>Contains a user-provided database identifier. This identifier is the unique key that identifies a DB instance.</p>
    pub db_instance_identifier: Option<String>,
    /// <p>Specifies the current state of this database.</p>
    pub db_instance_status: Option<String>,
    /// <p>Specifies information on the subnet group that is associated with the DB instance, including the name, description, and subnets in the subnet group.</p>
    pub db_subnet_group: Option<DBSubnetGroup>,
    /// <p>The AWS Region-unique, immutable identifier for the DB instance. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB instance is accessed.</p>
    pub dbi_resource_id: Option<String>,
    /// <p>A list of log types that this DB instance is configured to export to Amazon CloudWatch Logs.</p>
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>Specifies the connection endpoint.</p>
    pub endpoint: Option<Endpoint>,
    /// <p>Provides the name of the database engine to be used for this DB instance.</p>
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Provides the date and time that the DB instance was created.</p>
    pub instance_create_time: Option<String>,
    /// <p> If <code>StorageEncrypted</code> is <code>true</code>, the AWS KMS key identifier for the encrypted DB instance. </p>
    pub kms_key_id: Option<String>,
    /// <p>Specifies the latest time to which a database can be restored with point-in-time restore.</p>
    pub latest_restorable_time: Option<String>,
    /// <p>Specifies that changes to the DB instance are pending. This element is included only when changes are pending. Specific changes are identified by subelements.</p>
    pub pending_modified_values: Option<PendingModifiedValues>,
    /// <p> Specifies the daily time range during which automated backups are created if automated backups are enabled, as determined by the <code>BackupRetentionPeriod</code>. </p>
    pub preferred_backup_window: Option<String>,
    /// <p>Specifies the weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which an Amazon DocumentDB replica is promoted to the primary instance after a failure of the existing primary instance.</p>
    pub promotion_tier: Option<i64>,
    /// <p>Not supported. Amazon DocumentDB does not currently support public endpoints. The value of <code>PubliclyAccessible</code> is always <code>false</code>.</p>
    pub publicly_accessible: Option<bool>,
    /// <p>The status of a read replica. If the instance is not a read replica, this is blank.</p>
    pub status_infos: Option<Vec<DBInstanceStatusInfo>>,
    /// <p>Specifies whether or not the DB instance is encrypted.</p>
    pub storage_encrypted: Option<bool>,
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
                "DBSubnetGroup" => {
                    obj.db_subnet_group = Some(DBSubnetGroupDeserializer::deserialize(
                        "DBSubnetGroup",
                        stack,
                    )?);
                }
                "DbiResourceId" => {
                    obj.dbi_resource_id =
                        Some(StringDeserializer::deserialize("DbiResourceId", stack)?);
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
                "InstanceCreateTime" => {
                    obj.instance_create_time = Some(TStampDeserializer::deserialize(
                        "InstanceCreateTime",
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
                "PendingModifiedValues" => {
                    obj.pending_modified_values =
                        Some(PendingModifiedValuesDeserializer::deserialize(
                            "PendingModifiedValues",
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
                "PubliclyAccessible" => {
                    obj.publicly_accessible = Some(BooleanDeserializer::deserialize(
                        "PubliclyAccessible",
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
/// <p>Represents the output of <a>DescribeDBInstances</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBInstanceMessage {
    /// <p>Detailed information about one or more DB instances. </p>
    pub db_instances: Option<Vec<DBInstance>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
    /// <p>A Boolean value that is <code>true</code> if the instance is operating normally, or <code>false</code> if the instance is in an error state.</p>
    pub normal: Option<bool>,
    /// <p>Status of the DB instance. For a <code>StatusType</code> of read replica, the values can be <code>replicating</code>, error, <code>stopped</code>, or <code>terminated</code>.</p>
    pub status: Option<String>,
    /// <p>This value is currently "<code>read replication</code>."</p>
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
/// <p>Detailed information about a DB subnet group. </p>
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
    /// <p>Detailed information about one or more subnets within a DB subnet group.</p>
    pub subnets: Option<Vec<Subnet>>,
    /// <p>Provides the virtual private cloud (VPC) ID of the DB subnet group.</p>
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
/// <p>Represents the output of <a>DescribeDBSubnetGroups</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DBSubnetGroupMessage {
    /// <p>Detailed information about one or more DB subnet groups.</p>
    pub db_subnet_groups: Option<Vec<DBSubnetGroup>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
/// <p>Represents the input to <a>DeleteDBCluster</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBClusterMessage {
    /// <p><p>The DB cluster identifier for the DB cluster to be deleted. This parameter isn&#39;t case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match an existing <code>DBClusterIdentifier</code>.</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p><p> The DB cluster snapshot identifier of the new DB cluster snapshot created when <code>SkipFinalSnapshot</code> is set to <code>false</code>. </p> <note> <p> Specifying this parameter and also setting the <code>SkipFinalShapshot</code> parameter to <code>true</code> results in an error. </p> </note> <p>Constraints:</p> <ul> <li> <p>Must be from 1 to 255 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub final_db_snapshot_identifier: Option<String>,
    /// <p> Determines whether a final DB cluster snapshot is created before the DB cluster is deleted. If <code>true</code> is specified, no DB cluster snapshot is created. If <code>false</code> is specified, a DB cluster snapshot is created before the DB cluster is deleted. </p> <note> <p>If <code>SkipFinalSnapshot</code> is <code>false</code>, you must specify a <code>FinalDBSnapshotIdentifier</code> parameter.</p> </note> <p>Default: <code>false</code> </p>
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

/// <p>Represents the input to <a>DeleteDBClusterParameterGroup</a>.</p>
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
/// <p>Represents the input to <a>DeleteDBClusterSnapshot</a>.</p>
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
/// <p>Represents the input to <a>DeleteDBInstance</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBInstanceMessage {
    /// <p><p>The DB instance identifier for the DB instance to be deleted. This parameter isn&#39;t case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the name of an existing DB instance.</p> </li> </ul></p>
    pub db_instance_identifier: String,
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
/// <p>Represents the input to <a>DeleteDBSubnetGroup</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDBSubnetGroupMessage {
    /// <p>The name of the database subnet group to delete.</p> <note> <p>You can't delete the default subnet group.</p> </note> <p>Constraints:</p> <p>Must match the name of an existing <code>DBSubnetGroup</code>. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
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
pub struct DescribeCertificatesMessage {
    /// <p><p>The user-supplied certificate identifier. If this parameter is specified, information for only the specified certificate is returned. If this parameter is omitted, a list of up to <code>MaxRecords</code> certificates is returned. This parameter is not case sensitive.</p> <p>Constraints</p> <ul> <li> <p>Must match an existing <code>CertificateIdentifier</code>.</p> </li> </ul></p>
    pub certificate_identifier: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous <code>DescribeCertificates</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p><p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints:</p> <ul> <li> <p>Minimum: 20</p> </li> <li> <p>Maximum: 100</p> </li> </ul></p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeCertificatesMessage` contents to a `SignedRequest`.
struct DescribeCertificatesMessageSerializer;
impl DescribeCertificatesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCertificatesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "CertificateIdentifier"),
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

/// <p>Represents the input to <a>DescribeDBClusterParameterGroups</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClusterParameterGroupsMessage {
    /// <p><p>The name of a specific DB cluster parameter group to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If provided, must match the name of an existing <code>DBClusterParameterGroup</code>.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
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

/// <p>Represents the input to <a>DescribeDBClusterParameters</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClusterParametersMessage {
    /// <p><p>The name of a specific DB cluster parameter group to return parameter details for.</p> <p>Constraints:</p> <ul> <li> <p>If provided, must match the name of an existing <code>DBClusterParameterGroup</code>.</p> </li> </ul></p>
    pub db_cluster_parameter_group_name: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p> A value that indicates to return only parameters for a specific source. Parameter sources can be <code>engine</code>, <code>service</code>, or <code>customer</code>. </p>
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

/// <p>Represents the input to <a>DescribeDBClusterSnapshotAttributes</a>.</p>
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
/// <p>Represents the input to <a>DescribeDBClusterSnapshots</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClusterSnapshotsMessage {
    /// <p><p>The ID of the DB cluster to retrieve the list of DB cluster snapshots for. This parameter can&#39;t be used with the <code>DBClusterSnapshotIdentifier</code> parameter. This parameter is not case sensitive. </p> <p>Constraints:</p> <ul> <li> <p>If provided, must match the identifier of an existing <code>DBCluster</code>.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p><p>A specific DB cluster snapshot identifier to describe. This parameter can&#39;t be used with the <code>DBClusterIdentifier</code> parameter. This value is stored as a lowercase string. </p> <p>Constraints:</p> <ul> <li> <p>If provided, must match the identifier of an existing <code>DBClusterSnapshot</code>.</p> </li> <li> <p>If this identifier is for an automated snapshot, the <code>SnapshotType</code> parameter must also be specified.</p> </li> </ul></p>
    pub db_cluster_snapshot_identifier: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>Set to <code>true</code> to include manual DB cluster snapshots that are public and can be copied or restored by any AWS account, and otherwise <code>false</code>. The default is <code>false</code>.</p>
    pub include_public: Option<bool>,
    /// <p>Set to <code>true</code> to include shared manual DB cluster snapshots from other AWS accounts that this AWS account has been given permission to copy or restore, and otherwise <code>false</code>. The default is <code>false</code>.</p>
    pub include_shared: Option<bool>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The type of DB cluster snapshots to be returned. You can specify one of the following values:</p> <ul> <li> <p> <code>automated</code> - Return all DB cluster snapshots that Amazon DocumentDB has automatically created for your AWS account.</p> </li> <li> <p> <code>manual</code> - Return all DB cluster snapshots that you have manually created for your AWS account.</p> </li> <li> <p> <code>shared</code> - Return all manual DB cluster snapshots that have been shared to your AWS account.</p> </li> <li> <p> <code>public</code> - Return all DB cluster snapshots that have been marked as public.</p> </li> </ul> <p>If you don't specify a <code>SnapshotType</code> value, then both automated and manual DB cluster snapshots are returned. You can include shared DB cluster snapshots with these results by setting the <code>IncludeShared</code> parameter to <code>true</code>. You can include public DB cluster snapshots with these results by setting the <code>IncludePublic</code> parameter to <code>true</code>.</p> <p>The <code>IncludeShared</code> and <code>IncludePublic</code> parameters don't apply for <code>SnapshotType</code> values of <code>manual</code> or <code>automated</code>. The <code>IncludePublic</code> parameter doesn't apply when <code>SnapshotType</code> is set to <code>shared</code>. The <code>IncludeShared</code> parameter doesn't apply when <code>SnapshotType</code> is set to <code>public</code>.</p>
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

/// <p>Represents the input to <a>DescribeDBClusters</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBClustersMessage {
    /// <p><p>The user-provided DB cluster identifier. If this parameter is specified, information from only the specific DB cluster is returned. This parameter isn&#39;t case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>If provided, must match an existing <code>DBClusterIdentifier</code>.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p><p>A filter that specifies one or more DB clusters to describe.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list only includes information about the DB clusters identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
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

/// <p>Represents the input to <a>DescribeDBEngineVersions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBEngineVersionsMessage {
    /// <p><p>The name of a specific DB parameter group family to return details for.</p> <p>Constraints:</p> <ul> <li> <p>If provided, must match an existing <code>DBParameterGroupFamily</code>.</p> </li> </ul></p>
    pub db_parameter_group_family: Option<String>,
    /// <p>Indicates that only the default version of the specified engine or engine and major version combination is returned.</p>
    pub default_only: Option<bool>,
    /// <p>The database engine to return.</p>
    pub engine: Option<String>,
    /// <p>The database engine version to return.</p> <p>Example: <code>5.1.49</code> </p>
    pub engine_version: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>If this parameter is specified and the requested engine supports the <code>CharacterSetName</code> parameter for <code>CreateDBInstance</code>, the response includes a list of supported character sets for each engine version. </p>
    pub list_supported_character_sets: Option<bool>,
    /// <p>If this parameter is specified and the requested engine supports the <code>TimeZone</code> parameter for <code>CreateDBInstance</code>, the response includes a list of supported time zones for each engine version. </p>
    pub list_supported_timezones: Option<bool>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
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

/// <p>Represents the input to <a>DescribeDBInstances</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBInstancesMessage {
    /// <p><p>The user-provided instance identifier. If this parameter is specified, information from only the specific DB instance is returned. This parameter isn&#39;t case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>If provided, must match the identifier of an existing <code>DBInstance</code>.</p> </li> </ul></p>
    pub db_instance_identifier: Option<String>,
    /// <p><p>A filter that specifies one or more DB instances to describe.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list includes only the information about the DB instances that are associated with the DB clusters that are identified by these ARNs.</p> </li> <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and DB instance ARNs. The results list includes only the information about the DB instances that are identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
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

/// <p>Represents the input to <a>DescribeDBSubnetGroups</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDBSubnetGroupsMessage {
    /// <p>The name of the DB subnet group to return details for.</p>
    pub db_subnet_group_name: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
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

/// <p>Represents the input to <a>DescribeEngineDefaultClusterParameters</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEngineDefaultClusterParametersMessage {
    /// <p>The name of the DB cluster parameter group family to return the engine parameter information for.</p>
    pub db_parameter_group_family: String,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
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
/// <p>Represents the input to <a>DescribeEventCategories</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventCategoriesMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The type of source that is generating the events.</p> <p>Valid values: <code>db-instance</code>, <code>db-parameter-group</code>, <code>db-security-group</code>, <code>db-snapshot</code> </p>
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

/// <p>Represents the input to <a>DescribeEvents</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventsMessage {
    /// <p>The number of minutes to retrieve events for.</p> <p>Default: 60</p>
    pub duration: Option<i64>,
    /// <p> The end of the time interval for which to retrieve events, specified in ISO 8601 format. </p> <p>Example: 2009-07-08T18:00Z</p>
    pub end_time: Option<String>,
    /// <p>A list of event categories that trigger notifications for an event notification subscription.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p><p>The identifier of the event source for which events are returned. If not specified, then all sources are included in the response.</p> <p>Constraints:</p> <ul> <li> <p>If <code>SourceIdentifier</code> is provided, <code>SourceType</code> must also be provided.</p> </li> <li> <p>If the source type is <code>DBInstance</code>, a <code>DBInstanceIdentifier</code> must be provided.</p> </li> <li> <p>If the source type is <code>DBSecurityGroup</code>, a <code>DBSecurityGroupName</code> must be provided.</p> </li> <li> <p>If the source type is <code>DBParameterGroup</code>, a <code>DBParameterGroupName</code> must be provided.</p> </li> <li> <p>If the source type is <code>DBSnapshot</code>, a <code>DBSnapshotIdentifier</code> must be provided.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub source_identifier: Option<String>,
    /// <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    pub source_type: Option<String>,
    /// <p> The beginning of the time interval to retrieve events for, specified in ISO 8601 format. </p> <p>Example: 2009-07-08T18:00Z</p>
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

/// <p>Represents the input to <a>DescribeOrderableDBInstanceOptions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrderableDBInstanceOptionsMessage {
    /// <p>The DB instance class filter value. Specify this parameter to show only the available offerings that match the specified DB instance class.</p>
    pub db_instance_class: Option<String>,
    /// <p>The name of the engine to retrieve DB instance options for.</p>
    pub engine: String,
    /// <p>The engine version filter value. Specify this parameter to show only the available offerings that match the specified engine version.</p>
    pub engine_version: Option<String>,
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The license model filter value. Specify this parameter to show only the available offerings that match the specified license model.</p>
    pub license_model: Option<String>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The virtual private cloud (VPC) filter value. Specify this parameter to show only the available VPC or non-VPC offerings.</p>
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

/// <p>Represents the input to <a>DescribePendingMaintenanceActions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePendingMaintenanceActionsMessage {
    /// <p><p>A filter that specifies one or more resources to return pending maintenance actions for.</p> <p>Supported filters:</p> <ul> <li> <p> <code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list includes only pending maintenance actions for the DB clusters identified by these ARNs.</p> </li> <li> <p> <code>db-instance-id</code> - Accepts DB instance identifiers and DB instance ARNs. The results list includes only pending maintenance actions for the DB instances identified by these ARNs.</p> </li> </ul></p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
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

/// <p>Network information for accessing a DB cluster or DB instance. Client programs must specify a valid endpoint to access these Amazon DocumentDB resources.</p>
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
/// <p>Contains the result of a successful invocation of the <code>DescribeEngineDefaultClusterParameters</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EngineDefaults {
    /// <p>The name of the DB cluster parameter group family to return the engine parameter information for.</p>
    pub db_parameter_group_family: Option<String>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The parameters of a particular DB cluster parameter group family.</p>
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
/// <p>Detailed information about an event.</p>
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

/// <p>An event source type, accompanied by one or more event category names.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventCategoriesMap {
    /// <p>The event categories for the specified source type.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The source type that the returned categories belong to.</p>
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
/// <p>Represents the output of <a>DescribeEventCategories</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventCategoriesMessage {
    /// <p>A list of event category maps.</p>
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
/// <p>Represents the output of <a>DescribeEvents</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventsMessage {
    /// <p>Detailed information about one or more events. </p>
    pub events: Option<Vec<Event>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
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
/// <p>Represents the input to <a>FailoverDBCluster</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FailoverDBClusterMessage {
    /// <p><p>A DB cluster identifier to force a failover for. This parameter is not case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing <code>DBCluster</code>.</p> </li> </ul></p>
    pub db_cluster_identifier: Option<String>,
    /// <p>The name of the instance to promote to the primary instance.</p> <p>You must specify the instance identifier for an Amazon DocumentDB replica in the DB cluster. For example, <code>mydbcluster-replica1</code>.</p>
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
/// <p>A named set of filter values, used to return a more specific list of results. You can use a filter to match a set of resources by specific criteria, such as IDs.</p> <p>Wildcards are not supported in filters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name of the filter. Filter names are case sensitive.</p>
    pub name: String,
    /// <p>One or more filter values. Filter values are case sensitive.</p>
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

/// <p>Represents the input to <a>ListTagsForResource</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceMessage {
    /// <p>This parameter is not currently supported.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The Amazon DocumentDB resource with tags to be listed. This value is an Amazon Resource Name (ARN).</p>
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

/// <p>Represents the input to <a>ModifyDBCluster</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBClusterMessage {
    /// <p>A value that specifies whether the changes in this request and any pending changes are asynchronously applied as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the DB cluster. If this parameter is set to <code>false</code>, changes to the DB cluster are applied during the next maintenance window.</p> <p>The <code>ApplyImmediately</code> parameter affects only the <code>NewDBClusterIdentifier</code> and <code>MasterUserPassword</code> values. If you set this parameter value to <code>false</code>, the changes to the <code>NewDBClusterIdentifier</code> and <code>MasterUserPassword</code> values are applied during the next maintenance window. All other changes are applied immediately, regardless of the value of the <code>ApplyImmediately</code> parameter.</p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p><p>The number of days for which automated backups are retained. You must specify a minimum value of 1.</p> <p>Default: 1</p> <p>Constraints:</p> <ul> <li> <p>Must be a value from 1 to 35.</p> </li> </ul></p>
    pub backup_retention_period: Option<i64>,
    /// <p>The configuration setting for the log types to be enabled for export to Amazon CloudWatch Logs for a specific DB instance or DB cluster. The <code>EnableLogTypes</code> and <code>DisableLogTypes</code> arrays determine which logs are exported (or not exported) to CloudWatch Logs.</p>
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    /// <p><p>The DB cluster identifier for the cluster that is being modified. This parameter is not case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing <code>DBCluster</code>.</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p>The name of the DB cluster parameter group to use for the DB cluster.</p>
    pub db_cluster_parameter_group_name: Option<String>,
    /// <p>Specifies whether this cluster can be deleted. If <code>DeletionProtection</code> is enabled, the cluster cannot be deleted unless it is modified and <code>DeletionProtection</code> is disabled. <code>DeletionProtection</code> protects clusters from being accidentally deleted.</p>
    pub deletion_protection: Option<bool>,
    /// <p>The version number of the database engine to which you want to upgrade. Changing this parameter results in an outage. The change is applied during the next maintenance window unless the <code>ApplyImmediately</code> parameter is set to <code>true</code>.</p>
    pub engine_version: Option<String>,
    /// <p>The password for the master database user. This password can contain any printable ASCII character except forward slash (/), double quote ("), or the "at" symbol (@).</p> <p>Constraints: Must contain from 8 to 100 characters.</p>
    pub master_user_password: Option<String>,
    /// <p>The new DB cluster identifier for the DB cluster when renaming a DB cluster. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-cluster2</code> </p>
    pub new_db_cluster_identifier: Option<String>,
    /// <p>The port number on which the DB cluster accepts connections.</p> <p>Constraints: Must be a value from <code>1150</code> to <code>65535</code>. </p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p><p>The daily time range during which automated backups are created if automated backups are enabled, using the <code>BackupRetentionPeriod</code> parameter. </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. </p> <p>Constraints:</p> <ul> <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li> <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week. </p> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A list of virtual private cloud (VPC) security groups that the DB cluster will belong to.</p>
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

/// <p>Represents the input to <a>ModifyDBClusterParameterGroup</a>.</p>
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
/// <p>Represents the input to <a>ModifyDBClusterSnapshotAttribute</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBClusterSnapshotAttributeMessage {
    /// <p>The name of the DB cluster snapshot attribute to modify.</p> <p>To manage authorization for other AWS accounts to copy or restore a manual DB cluster snapshot, set this value to <code>restore</code>.</p>
    pub attribute_name: String,
    /// <p>The identifier for the DB cluster snapshot to modify the attributes for.</p>
    pub db_cluster_snapshot_identifier: String,
    /// <p>A list of DB cluster snapshot attributes to add to the attribute specified by <code>AttributeName</code>.</p> <p>To authorize other AWS accounts to copy or restore a manual DB cluster snapshot, set this list to include one or more AWS account IDs. To make the manual DB cluster snapshot restorable by any AWS account, set it to <code>all</code>. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want to be available to all AWS accounts.</p>
    pub values_to_add: Option<Vec<String>>,
    /// <p>A list of DB cluster snapshot attributes to remove from the attribute specified by <code>AttributeName</code>.</p> <p>To remove authorization for other AWS accounts to copy or restore a manual DB cluster snapshot, set this list to include one or more AWS account identifiers. To remove authorization for any AWS account to copy or restore the DB cluster snapshot, set it to <code>all</code> . If you specify <code>all</code>, an AWS account whose account ID is explicitly added to the <code>restore</code> attribute can still copy or restore a manual DB cluster snapshot.</p>
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
/// <p>Represents the input to <a>ModifyDBInstance</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBInstanceMessage {
    /// <p>Specifies whether the modifications in this request and any pending modifications are asynchronously applied as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the DB instance. </p> <p> If this parameter is set to <code>false</code>, changes to the DB instance are applied during the next maintenance window. Some parameter changes can cause an outage and are applied on the next reboot.</p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p>Indicates that minor version upgrades are applied automatically to the DB instance during the maintenance window. Changing this parameter doesn't result in an outage except in the following case, and the change is asynchronously applied as soon as possible. An outage results if this parameter is set to <code>true</code> during the maintenance window, and a newer minor version is available, and Amazon DocumentDB has enabled automatic patching for that engine version. </p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Indicates the certificate that needs to be associated with the instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p>The new compute and memory capacity of the DB instance; for example, <code>db.r5.large</code>. Not all DB instance classes are available in all AWS Regions. </p> <p>If you modify the DB instance class, an outage occurs during the change. The change is applied during the next maintenance window, unless <code>ApplyImmediately</code> is specified as <code>true</code> for this request. </p> <p>Default: Uses existing setting.</p>
    pub db_instance_class: Option<String>,
    /// <p><p>The DB instance identifier. This value is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing <code>DBInstance</code>.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p> The new DB instance identifier for the DB instance when renaming a DB instance. When you change the DB instance identifier, an instance reboot occurs immediately if you set <code>Apply Immediately</code> to <code>true</code>. It occurs during the next maintenance window if you set <code>Apply Immediately</code> to <code>false</code>. This value is stored as a lowercase string. </p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>mydbinstance</code> </p>
    pub new_db_instance_identifier: Option<String>,
    /// <p>The weekly time range (in UTC) during which system maintenance can occur, which might result in an outage. Changing this parameter doesn't result in an outage except in the following situation, and the change is asynchronously applied as soon as possible. If there are pending actions that cause a reboot, and the maintenance window is changed to include the current time, changing this parameter causes a reboot of the DB instance. If you are moving this window to the current time, there must be at least 30 minutes between the current time and end of the window to ensure that pending changes are applied.</p> <p>Default: Uses existing setting.</p> <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p> <p>Constraints: Must be at least 30 minutes.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A value that specifies the order in which an Amazon DocumentDB replica is promoted to the primary instance after a failure of the existing primary instance.</p> <p>Default: 1</p> <p>Valid values: 0-15</p>
    pub promotion_tier: Option<i64>,
}

/// Serialize `ModifyDBInstanceMessage` contents to a `SignedRequest`.
struct ModifyDBInstanceMessageSerializer;
impl ModifyDBInstanceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyDBInstanceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
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
        if let Some(ref field_value) = obj.ca_certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "CACertificateIdentifier"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.db_instance_class {
            params.put(&format!("{}{}", prefix, "DBInstanceClass"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "DBInstanceIdentifier"),
            &obj.db_instance_identifier,
        );
        if let Some(ref field_value) = obj.new_db_instance_identifier {
            params.put(
                &format!("{}{}", prefix, "NewDBInstanceIdentifier"),
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
/// <p>Represents the input to <a>ModifyDBSubnetGroup</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyDBSubnetGroupMessage {
    /// <p>The description for the DB subnet group.</p>
    pub db_subnet_group_description: Option<String>,
    /// <p>The name for the DB subnet group. This value is stored as a lowercase string. You can't modify the default subnet group. </p> <p>Constraints: Must match the name of an existing <code>DBSubnetGroup</code>. Must not be default.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: String,
    /// <p>The Amazon EC2 subnet IDs for the DB subnet group.</p>
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
/// <p>The options that are available for a DB instance.</p>
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
    /// <p>Indicates whether a DB instance is in a virtual private cloud (VPC).</p>
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
/// <p>Represents the output of <a>DescribeOrderableDBInstanceOptions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OrderableDBInstanceOptionsMessage {
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The options that are available for a particular orderable DB instance.</p>
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
/// <p>Detailed information about an individual parameter.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Parameter {
    /// <p>Specifies the valid range of values for the parameter.</p>
    pub allowed_values: Option<String>,
    /// <p>Indicates when to apply parameter updates.</p>
    pub apply_method: Option<String>,
    /// <p>Specifies the engine-specific parameters type.</p>
    pub apply_type: Option<String>,
    /// <p>Specifies the valid data type for the parameter.</p>
    pub data_type: Option<String>,
    /// <p>Provides a description of the parameter.</p>
    pub description: Option<String>,
    /// <p> Indicates whether (<code>true</code>) or not (<code>false</code>) the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed. </p>
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

/// <p>A list of the log types whose configuration is still pending. These log types are in the process of being activated or deactivated.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingCloudwatchLogsExports {
    /// <p>Log types that are in the process of being enabled. After they are enabled, these log types are exported to Amazon CloudWatch Logs.</p>
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
    /// <p>The effective date when the pending maintenance action is applied to the resource.</p>
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
/// <p>Represents the output of <a>DescribePendingMaintenanceActions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingMaintenanceActionsMessage {
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maintenance actions to be applied.</p>
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
/// <p> One or more modified settings for a DB instance. These modified settings have been requested, but haven't been applied yet.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingModifiedValues {
    /// <p> Contains the new <code>AllocatedStorage</code> size for the DB instance that will be applied or is currently being applied. </p>
    pub allocated_storage: Option<i64>,
    /// <p>Specifies the pending number of days for which automated backups are retained.</p>
    pub backup_retention_period: Option<i64>,
    /// <p>Specifies the identifier of the certificate authority (CA) certificate for the DB instance.</p>
    pub ca_certificate_identifier: Option<String>,
    /// <p> Contains the new <code>DBInstanceClass</code> for the DB instance that will be applied or is currently being applied. </p>
    pub db_instance_class: Option<String>,
    /// <p> Contains the new <code>DBInstanceIdentifier</code> for the DB instance that will be applied or is currently being applied. </p>
    pub db_instance_identifier: Option<String>,
    /// <p>The new DB subnet group for the DB instance. </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Indicates the database engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Specifies the new Provisioned IOPS value for the DB instance that will be applied or is currently being applied.</p>
    pub iops: Option<i64>,
    /// <p>The license model for the DB instance.</p> <p>Valid values: <code>license-included</code>, <code>bring-your-own-license</code>, <code>general-public-license</code> </p>
    pub license_model: Option<String>,
    /// <p>Contains the pending or currently in-progress change of the master credentials for the DB instance.</p>
    pub master_user_password: Option<String>,
    /// <p>Indicates that the Single-AZ DB instance is to change to a Multi-AZ deployment.</p>
    pub multi_az: Option<bool>,
    /// <p>A list of the log types whose configuration is still pending. These log types are in the process of being activated or deactivated.</p>
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
/// <p>Represents the input to <a>RebootDBInstance</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootDBInstanceMessage {
    /// <p><p>The DB instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing <code>DBInstance</code>.</p> </li> </ul></p>
    pub db_instance_identifier: String,
    /// <p> When <code>true</code>, the reboot is conducted through a Multi-AZ failover. </p> <p>Constraint: You can't specify <code>true</code> if the instance is not configured for Multi-AZ.</p>
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
/// <p>Represents the input to <a>RemoveTagsFromResource</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromResourceMessage {
    /// <p>The Amazon DocumentDB resource that the tags are removed from. This value is an Amazon Resource Name (ARN).</p>
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

/// <p>Represents the input to <a>ResetDBClusterParameterGroup</a>.</p>
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

/// <p>Represents the output of <a>ApplyPendingMaintenanceAction</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ResourcePendingMaintenanceActions {
    /// <p>A list that provides details about the pending maintenance actions for the resource.</p>
    pub pending_maintenance_action_details: Option<Vec<PendingMaintenanceAction>>,
    /// <p>The Amazon Resource Name (ARN) of the resource that has pending maintenance actions.</p>
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
/// <p>Represents the input to <a>RestoreDBClusterFromSnapshot</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreDBClusterFromSnapshotMessage {
    /// <p>Provides the list of Amazon EC2 Availability Zones that instances in the restored DB cluster can be created in.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The name of the DB cluster to create from the DB snapshot or DB cluster snapshot. This parameter isn't case sensitive.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>my-snapshot-id</code> </p>
    pub db_cluster_identifier: String,
    /// <p>The name of the DB subnet group to use for the new DB cluster.</p> <p>Constraints: If provided, must match the name of an existing <code>DBSubnetGroup</code>.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Specifies whether this cluster can be deleted. If <code>DeletionProtection</code> is enabled, the cluster cannot be deleted unless it is modified and <code>DeletionProtection</code> is disabled. <code>DeletionProtection</code> protects clusters from being accidentally deleted.</p>
    pub deletion_protection: Option<bool>,
    /// <p>A list of log types that must be enabled for exporting to Amazon CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>The database engine to use for the new DB cluster.</p> <p>Default: The same as source.</p> <p>Constraint: Must be compatible with the engine of the source.</p>
    pub engine: String,
    /// <p>The version of the database engine to use for the new DB cluster.</p>
    pub engine_version: Option<String>,
    /// <p><p>The AWS KMS key identifier to use when restoring an encrypted DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>The AWS KMS key identifier is the Amazon Resource Name (ARN) for the AWS KMS encryption key. If you are restoring a DB cluster with the same AWS account that owns the AWS KMS encryption key used to encrypt the new DB cluster, then you can use the AWS KMS key alias instead of the ARN for the AWS KMS encryption key.</p> <p>If you do not specify a value for the <code>KmsKeyId</code> parameter, then the following occurs:</p> <ul> <li> <p>If the DB snapshot or DB cluster snapshot in <code>SnapshotIdentifier</code> is encrypted, then the restored DB cluster is encrypted using the AWS KMS key that was used to encrypt the DB snapshot or the DB cluster snapshot.</p> </li> <li> <p>If the DB snapshot or the DB cluster snapshot in <code>SnapshotIdentifier</code> is not encrypted, then the restored DB cluster is not encrypted.</p> </li> </ul></p>
    pub kms_key_id: Option<String>,
    /// <p>The port number on which the new DB cluster accepts connections.</p> <p>Constraints: Must be a value from <code>1150</code> to <code>65535</code>.</p> <p>Default: The same port as the original DB cluster.</p>
    pub port: Option<i64>,
    /// <p><p>The identifier for the DB snapshot or DB cluster snapshot to restore from.</p> <p>You can use either the name or the Amazon Resource Name (ARN) to specify a DB cluster snapshot. However, you can use only the ARN to specify a DB snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing snapshot.</p> </li> </ul></p>
    pub snapshot_identifier: String,
    /// <p>The tags to be assigned to the restored DB cluster.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of virtual private cloud (VPC) security groups that the new DB cluster will belong to.</p>
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
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
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
/// <p>Represents the input to <a>RestoreDBClusterToPointInTime</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreDBClusterToPointInTimeMessage {
    /// <p><p>The name of the new DB cluster to be created.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub db_cluster_identifier: String,
    /// <p>The DB subnet group name to use for the new DB cluster.</p> <p>Constraints: If provided, must match the name of an existing <code>DBSubnetGroup</code>.</p> <p>Example: <code>mySubnetgroup</code> </p>
    pub db_subnet_group_name: Option<String>,
    /// <p>Specifies whether this cluster can be deleted. If <code>DeletionProtection</code> is enabled, the cluster cannot be deleted unless it is modified and <code>DeletionProtection</code> is disabled. <code>DeletionProtection</code> protects clusters from being accidentally deleted.</p>
    pub deletion_protection: Option<bool>,
    /// <p>A list of log types that must be enabled for exporting to Amazon CloudWatch Logs.</p>
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// <p>The AWS KMS key identifier to use when restoring an encrypted DB cluster from an encrypted DB cluster.</p> <p>The AWS KMS key identifier is the Amazon Resource Name (ARN) for the AWS KMS encryption key. If you are restoring a DB cluster with the same AWS account that owns the AWS KMS encryption key used to encrypt the new DB cluster, then you can use the AWS KMS key alias instead of the ARN for the AWS KMS encryption key.</p> <p>You can restore to a new DB cluster and encrypt the new DB cluster with an AWS KMS key that is different from the AWS KMS key used to encrypt the source DB cluster. The new DB cluster is encrypted with the AWS KMS key identified by the <code>KmsKeyId</code> parameter.</p> <p>If you do not specify a value for the <code>KmsKeyId</code> parameter, then the following occurs:</p> <ul> <li> <p>If the DB cluster is encrypted, then the restored DB cluster is encrypted using the AWS KMS key that was used to encrypt the source DB cluster.</p> </li> <li> <p>If the DB cluster is not encrypted, then the restored DB cluster is not encrypted.</p> </li> </ul> <p>If <code>DBClusterIdentifier</code> refers to a DB cluster that is not encrypted, then the restore request is rejected.</p>
    pub kms_key_id: Option<String>,
    /// <p>The port number on which the new DB cluster accepts connections.</p> <p>Constraints: Must be a value from <code>1150</code> to <code>65535</code>. </p> <p>Default: The default port for the engine.</p>
    pub port: Option<i64>,
    /// <p>The date and time to restore the DB cluster to.</p> <p>Valid values: A time in Universal Coordinated Time (UTC) format.</p> <p>Constraints:</p> <ul> <li> <p>Must be before the latest restorable time for the DB instance.</p> </li> <li> <p>Must be specified if the <code>UseLatestRestorableTime</code> parameter is not provided.</p> </li> <li> <p>Cannot be specified if the <code>UseLatestRestorableTime</code> parameter is <code>true</code>.</p> </li> <li> <p>Cannot be specified if the <code>RestoreType</code> parameter is <code>copy-on-write</code>.</p> </li> </ul> <p>Example: <code>2015-03-07T23:45:00Z</code> </p>
    pub restore_to_time: Option<String>,
    /// <p><p>The identifier of the source DB cluster from which to restore.</p> <p>Constraints:</p> <ul> <li> <p>Must match the identifier of an existing <code>DBCluster</code>.</p> </li> </ul></p>
    pub source_db_cluster_identifier: String,
    /// <p>The tags to be assigned to the restored DB cluster.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A value that is set to <code>true</code> to restore the DB cluster to the latest restorable backup time, and <code>false</code> otherwise. </p> <p>Default: <code>false</code> </p> <p>Constraints: Cannot be specified if the <code>RestoreToTime</code> parameter is provided.</p>
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
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.restore_to_time {
            params.put(&format!("{}{}", prefix, "RestoreToTime"), &field_value);
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDBClusterMessage {
    /// <p>The identifier of the cluster to restart. Example: <code>docdb-2019-05-28-15-24-52</code> </p>
    pub db_cluster_identifier: String,
}

/// Serialize `StartDBClusterMessage` contents to a `SignedRequest`.
struct StartDBClusterMessageSerializer;
impl StartDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StartDBClusterMessage) {
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
pub struct StartDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct StartDBClusterResultDeserializer;
impl StartDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StartDBClusterResult, XmlParseError> {
        deserialize_elements::<_, StartDBClusterResult, _>(tag_name, stack, |name, stack, obj| {
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
pub struct StopDBClusterMessage {
    /// <p>The identifier of the cluster to stop. Example: <code>docdb-2019-05-28-15-24-52</code> </p>
    pub db_cluster_identifier: String,
}

/// Serialize `StopDBClusterMessage` contents to a `SignedRequest`.
struct StopDBClusterMessageSerializer;
impl StopDBClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StopDBClusterMessage) {
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
pub struct StopDBClusterResult {
    pub db_cluster: Option<DBCluster>,
}

struct StopDBClusterResultDeserializer;
impl StopDBClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StopDBClusterResult, XmlParseError> {
        deserialize_elements::<_, StopDBClusterResult, _>(tag_name, stack, |name, stack, obj| {
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
/// <p> Detailed information about a subnet. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Subnet {
    /// <p>Specifies the Availability Zone for the subnet.</p>
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
/// <p>Metadata assigned to an Amazon DocumentDB resource consisting of a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>The required name of the tag. The string value can be from 1 to 128 Unicode characters in length and can't be prefixed with "aws:" or "rds:". The string can contain only the set of Unicode letters, digits, white space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    pub key: Option<String>,
    /// <p>The optional value of the tag. The string value can be from 1 to 256 Unicode characters in length and can't be prefixed with "aws:" or "rds:". The string can contain only the set of Unicode letters, digits, white space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
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

/// <p>Represents the output of <a>ListTagsForResource</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagListMessage {
    /// <p>A list of one or more tags.</p>
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
/// <p>The version of the database engine that a DB instance can be upgraded to.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpgradeTarget {
    /// <p>A value that indicates whether the target version is applied to any source DB instances that have <code>AutoMinorVersionUpgrade</code> set to <code>true</code>.</p>
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

/// <p>Used as a response element for queries on virtual private cloud (VPC) security group membership.</p>
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
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <code>DBSnapshotIdentifier</code> doesn't refer to an existing DB snapshot. </p>
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
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
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
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            ApplyPendingMaintenanceActionError::InvalidDBClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            ApplyPendingMaintenanceActionError::InvalidDBInstanceStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
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
            ApplyPendingMaintenanceActionError::InvalidDBClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ApplyPendingMaintenanceActionError::InvalidDBInstanceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
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
    /// <p>A DB parameter group with the same name already exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>This request would cause you to exceed the allowed number of DB parameter groups.</p>
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
    /// <p>You already have a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p> <code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The provided value isn't a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>An error occurred when accessing an AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The request would cause you to exceed the allowed number of DB snapshots.</p>
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
/// Errors returned by CreateDBCluster
#[derive(Debug, PartialEq)]
pub enum CreateDBClusterError {
    /// <p>You already have a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <code>DBClusterParameterGroupName</code> doesn't refer to an existing DB cluster parameter group. </p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p>The DB cluster can't be created because you have reached the maximum allowed quota of DB clusters.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>There is not enough storage available for the current action. You might be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available. </p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>The DB subnet group can't be deleted because it's in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p>The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC).</p>
    InvalidSubnet(String),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it is created because of changes that were made.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>An error occurred when accessing an AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The request would cause you to exceed the allowed amount of storage available across all DB instances.</p>
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
    /// <p>A DB parameter group with the same name already exists.</p>
    DBParameterGroupAlreadyExistsFault(String),
    /// <p>This request would cause you to exceed the allowed number of DB parameter groups.</p>
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
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>You already have a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p>The provided value isn't a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The request would cause you to exceed the allowed number of DB snapshots.</p>
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
    /// <p>The specified CIDR IP or Amazon EC2 security group isn't authorized for the specified DB security group.</p> <p>Amazon DocumentDB also might not be authorized to perform necessary actions on your behalf using IAM.</p>
    AuthorizationNotFoundFault(String),
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>You already have a DB instance with the given identifier.</p>
    DBInstanceAlreadyExistsFault(String),
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p> <code>DBSecurityGroupName</code> doesn't refer to an existing DB security group. </p>
    DBSecurityGroupNotFoundFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The request would cause you to exceed the allowed number of DB instances.</p>
    InstanceQuotaExceededFault(String),
    /// <p>The specified DB instance class isn't available in the specified Availability Zone.</p>
    InsufficientDBInstanceCapacityFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC).</p>
    InvalidSubnet(String),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it is created because of changes that were made.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>An error occurred when accessing an AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The request would cause you to exceed the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// <p>Storage of the specified <code>StorageType</code> can't be associated with the DB instance. </p>
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
            CreateDBInstanceError::InstanceQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::InsufficientDBInstanceCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDBInstanceError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::InvalidVPCNetworkStateFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::KMSKeyNotAccessibleFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::StorageQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            CreateDBInstanceError::StorageTypeNotSupportedFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDBInstanceError {}
/// Errors returned by CreateDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateDBSubnetGroupError {
    /// <p> <code>DBSubnetGroupName</code> is already being used by an existing DB subnet group. </p>
    DBSubnetGroupAlreadyExistsFault(String),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>The request would cause you to exceed the allowed number of DB subnet groups.</p>
    DBSubnetGroupQuotaExceededFault(String),
    /// <p>The request would cause you to exceed the allowed number of subnets in a DB subnet group.</p>
    DBSubnetQuotaExceededFault(String),
    /// <p>The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC).</p>
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
/// Errors returned by DeleteDBCluster
#[derive(Debug, PartialEq)]
pub enum DeleteDBClusterError {
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>You already have a DB cluster snapshot with the given identifier.</p>
    DBClusterSnapshotAlreadyExistsFault(String),
    /// <p>The provided value isn't a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The request would cause you to exceed the allowed number of DB snapshots.</p>
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
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use, or it is in a state that is not valid. If you are trying to delete the parameter group, you can't delete it when the parameter group is in this state.</p>
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
    /// <p> <code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The provided value isn't a valid DB cluster snapshot state.</p>
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
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <code>DBSnapshotIdentifier</code> is already being used by an existing snapshot. </p>
    DBSnapshotAlreadyExistsFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>The request would cause you to exceed the allowed number of DB snapshots.</p>
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
/// Errors returned by DeleteDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDBSubnetGroupError {
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB subnet group can't be deleted because it's in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p> The DB subnet isn't in the <i>available</i> state. </p>
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
/// Errors returned by DescribeCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeCertificatesError {
    /// <p> <code>CertificateIdentifier</code> doesn't refer to an existing certificate. </p>
    CertificateNotFoundFault(String),
}

impl DescribeCertificatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCertificatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CertificateNotFound" => {
                        return RusotoError::Service(
                            DescribeCertificatesError::CertificateNotFoundFault(
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
impl fmt::Display for DescribeCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCertificatesError::CertificateNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCertificatesError {}
/// Errors returned by DescribeDBClusterParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBClusterParameterGroupsError {
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
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
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
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
    /// <p> <code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot. </p>
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
    /// <p> <code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot. </p>
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
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
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
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
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
/// Errors returned by DescribeDBSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeDBSubnetGroupsError {
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
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
/// Errors returned by FailoverDBCluster
#[derive(Debug, PartialEq)]
pub enum FailoverDBClusterError {
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
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
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <code>DBSnapshotIdentifier</code> doesn't refer to an existing DB snapshot. </p>
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
    /// <p>You already have a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <code>DBClusterParameterGroupName</code> doesn't refer to an existing DB cluster parameter group. </p>
    DBClusterParameterGroupNotFoundFault(String),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>The state of the DB security group doesn't allow deletion.</p>
    InvalidDBSecurityGroupStateFault(String),
    /// <p>The DB subnet group can't be deleted because it's in use.</p>
    InvalidDBSubnetGroupStateFault(String),
    /// <p>The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC).</p>
    InvalidSubnet(String),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it is created because of changes that were made.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The request would cause you to exceed the allowed amount of storage available across all DB instances.</p>
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
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use, or it is in a state that is not valid. If you are trying to delete the parameter group, you can't delete it when the parameter group is in this state.</p>
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
    /// <p> <code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p>The provided value isn't a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>You have exceeded the maximum number of accounts that you can share a manual DB snapshot with. </p>
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
    /// <p>The specified CIDR IP or Amazon EC2 security group isn't authorized for the specified DB security group.</p> <p>Amazon DocumentDB also might not be authorized to perform necessary actions on your behalf using IAM.</p>
    AuthorizationNotFoundFault(String),
    /// <p> <code>CertificateIdentifier</code> doesn't refer to an existing certificate. </p>
    CertificateNotFoundFault(String),
    /// <p>You already have a DB instance with the given identifier.</p>
    DBInstanceAlreadyExistsFault(String),
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p> <code>DBSecurityGroupName</code> doesn't refer to an existing DB security group. </p>
    DBSecurityGroupNotFoundFault(String),
    /// <p>The DB upgrade failed because a resource that the DB depends on can't be modified.</p>
    DBUpgradeDependencyFailureFault(String),
    /// <p>The specified DB instance class isn't available in the specified Availability Zone.</p>
    InsufficientDBInstanceCapacityFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
    /// <p>The state of the DB security group doesn't allow deletion.</p>
    InvalidDBSecurityGroupStateFault(String),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it is created because of changes that were made.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The request would cause you to exceed the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(String),
    /// <p>Storage of the specified <code>StorageType</code> can't be associated with the DB instance. </p>
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
            ModifyDBInstanceError::InsufficientDBInstanceCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::InvalidDBSecurityGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyDBInstanceError::InvalidVPCNetworkStateFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::StorageQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            ModifyDBInstanceError::StorageTypeNotSupportedFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyDBInstanceError {}
/// Errors returned by ModifyDBSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyDBSubnetGroupError {
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DBSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The request would cause you to exceed the allowed number of subnets in a DB subnet group.</p>
    DBSubnetQuotaExceededFault(String),
    /// <p>The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC).</p>
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
/// Errors returned by RebootDBInstance
#[derive(Debug, PartialEq)]
pub enum RebootDBInstanceError {
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
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
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p> <code>DBInstanceIdentifier</code> doesn't refer to an existing DB instance. </p>
    DBInstanceNotFoundFault(String),
    /// <p> <code>DBSnapshotIdentifier</code> doesn't refer to an existing DB snapshot. </p>
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
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group. </p>
    DBParameterGroupNotFoundFault(String),
    /// <p>The DB parameter group is in use, or it is in a state that is not valid. If you are trying to delete the parameter group, you can't delete it when the parameter group is in this state.</p>
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
/// Errors returned by RestoreDBClusterFromSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreDBClusterFromSnapshotError {
    /// <p>You already have a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p>The DB cluster can't be created because you have reached the maximum allowed quota of DB clusters.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p> <code>DBSnapshotIdentifier</code> doesn't refer to an existing DB snapshot. </p>
    DBSnapshotNotFoundFault(String),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster doesn't have enough capacity for the current operation.</p>
    InsufficientDBClusterCapacityFault(String),
    /// <p>There is not enough storage available for the current action. You might be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available. </p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The provided value isn't a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The state of the DB snapshot doesn't allow deletion.</p>
    InvalidDBSnapshotStateFault(String),
    /// <p>You cannot restore from a virtual private cloud (VPC) backup to a non-VPC DB instance.</p>
    InvalidRestoreFault(String),
    /// <p>The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC).</p>
    InvalidSubnet(String),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it is created because of changes that were made.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>An error occurred when accessing an AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The request would cause you to exceed the allowed amount of storage available across all DB instances.</p>
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
    /// <p>You already have a DB cluster with the given identifier.</p>
    DBClusterAlreadyExistsFault(String),
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster can't be created because you have reached the maximum allowed quota of DB clusters.</p>
    DBClusterQuotaExceededFault(String),
    /// <p> <code>DBClusterSnapshotIdentifier</code> doesn't refer to an existing DB cluster snapshot. </p>
    DBClusterSnapshotNotFoundFault(String),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group. </p>
    DBSubnetGroupNotFoundFault(String),
    /// <p>The DB cluster doesn't have enough capacity for the current operation.</p>
    InsufficientDBClusterCapacityFault(String),
    /// <p>There is not enough storage available for the current action. You might be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available. </p>
    InsufficientStorageClusterCapacityFault(String),
    /// <p>The provided value isn't a valid DB cluster snapshot state.</p>
    InvalidDBClusterSnapshotStateFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p>The state of the DB snapshot doesn't allow deletion.</p>
    InvalidDBSnapshotStateFault(String),
    /// <p>You cannot restore from a virtual private cloud (VPC) backup to a non-VPC DB instance.</p>
    InvalidRestoreFault(String),
    /// <p>The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC).</p>
    InvalidSubnet(String),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it is created because of changes that were made.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>An error occurred when accessing an AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The request would cause you to exceed the allowed amount of storage available across all DB instances.</p>
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
            RestoreDBClusterToPointInTimeError::StorageQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RestoreDBClusterToPointInTimeError {}
/// Errors returned by StartDBCluster
#[derive(Debug, PartialEq)]
pub enum StartDBClusterError {
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
}

impl StartDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(StartDBClusterError::DBClusterNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            StartDBClusterError::InvalidDBClusterStateFault(parsed_error.message),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            StartDBClusterError::InvalidDBInstanceStateFault(parsed_error.message),
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
impl fmt::Display for StartDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDBClusterError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            StartDBClusterError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            StartDBClusterError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDBClusterError {}
/// Errors returned by StopDBCluster
#[derive(Debug, PartialEq)]
pub enum StopDBClusterError {
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster. </p>
    DBClusterNotFoundFault(String),
    /// <p>The DB cluster isn't in a valid state.</p>
    InvalidDBClusterStateFault(String),
    /// <p> The specified DB instance isn't in the <i>available</i> state. </p>
    InvalidDBInstanceStateFault(String),
}

impl StopDBClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopDBClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DBClusterNotFoundFault" => {
                        return RusotoError::Service(StopDBClusterError::DBClusterNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDBClusterStateFault" => {
                        return RusotoError::Service(
                            StopDBClusterError::InvalidDBClusterStateFault(parsed_error.message),
                        )
                    }
                    "InvalidDBInstanceState" => {
                        return RusotoError::Service(
                            StopDBClusterError::InvalidDBInstanceStateFault(parsed_error.message),
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
impl fmt::Display for StopDBClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDBClusterError::DBClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            StopDBClusterError::InvalidDBClusterStateFault(ref cause) => write!(f, "{}", cause),
            StopDBClusterError::InvalidDBInstanceStateFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopDBClusterError {}
/// Trait representing the capabilities of the Amazon DocDB API. Amazon DocDB clients implement this trait.
#[async_trait]
pub trait Docdb {
    /// <p>Adds metadata tags to an Amazon DocumentDB resource. You can use these tags with cost allocation reporting to track costs that are associated with Amazon DocumentDB resources. or in a <code>Condition</code> statement in an AWS Identity and Access Management (IAM) policy for Amazon DocumentDB.</p>
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

    /// <p>Copies a snapshot of a DB cluster.</p> <p>To copy a DB cluster snapshot from a shared manual DB cluster snapshot, <code>SourceDBClusterSnapshotIdentifier</code> must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot.</p> <p>To cancel the copy operation after it is in progress, delete the target DB cluster snapshot identified by <code>TargetDBClusterSnapshotIdentifier</code> while that DB cluster snapshot is in the <i>copying</i> status.</p>
    async fn copy_db_cluster_snapshot(
        &self,
        input: CopyDBClusterSnapshotMessage,
    ) -> Result<CopyDBClusterSnapshotResult, RusotoError<CopyDBClusterSnapshotError>>;

    /// <p>Creates a new Amazon DocumentDB DB cluster.</p>
    async fn create_db_cluster(
        &self,
        input: CreateDBClusterMessage,
    ) -> Result<CreateDBClusterResult, RusotoError<CreateDBClusterError>>;

    /// <p><p>Creates a new DB cluster parameter group.</p> <p>Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster.</p> <p>A DB cluster parameter group is initially created with the default parameters for the database engine used by instances in the DB cluster. To provide custom values for any of the parameters, you must modify the group after you create it. After you create a DB cluster parameter group, you must associate it with your DB cluster. For the new DB cluster parameter group and associated settings to take effect, you must then reboot the DB instances in the DB cluster without failover.</p> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon DocumentDB to fully complete the create action before the DB cluster parameter group is used as the default for a new DB cluster. This step is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter.</p> </important></p>
    async fn create_db_cluster_parameter_group(
        &self,
        input: CreateDBClusterParameterGroupMessage,
    ) -> Result<CreateDBClusterParameterGroupResult, RusotoError<CreateDBClusterParameterGroupError>>;

    /// <p>Creates a snapshot of a DB cluster. </p>
    async fn create_db_cluster_snapshot(
        &self,
        input: CreateDBClusterSnapshotMessage,
    ) -> Result<CreateDBClusterSnapshotResult, RusotoError<CreateDBClusterSnapshotError>>;

    /// <p>Creates a new DB instance.</p>
    async fn create_db_instance(
        &self,
        input: CreateDBInstanceMessage,
    ) -> Result<CreateDBInstanceResult, RusotoError<CreateDBInstanceError>>;

    /// <p>Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two Availability Zones in the AWS Region.</p>
    async fn create_db_subnet_group(
        &self,
        input: CreateDBSubnetGroupMessage,
    ) -> Result<CreateDBSubnetGroupResult, RusotoError<CreateDBSubnetGroupError>>;

    /// <p><p>Deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can&#39;t be recovered. Manual DB cluster snapshots of the specified DB cluster are not deleted.</p> <p/></p>
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

    /// <p>Deletes a previously provisioned DB instance. </p>
    async fn delete_db_instance(
        &self,
        input: DeleteDBInstanceMessage,
    ) -> Result<DeleteDBInstanceResult, RusotoError<DeleteDBInstanceError>>;

    /// <p><p>Deletes a DB subnet group.</p> <note> <p>The specified database subnet group must not be associated with any DB instances.</p> </note></p>
    async fn delete_db_subnet_group(
        &self,
        input: DeleteDBSubnetGroupMessage,
    ) -> Result<(), RusotoError<DeleteDBSubnetGroupError>>;

    /// <p>Returns a list of certificate authority (CA) certificates provided by Amazon RDS for this AWS account.</p>
    async fn describe_certificates(
        &self,
        input: DescribeCertificatesMessage,
    ) -> Result<CertificateMessage, RusotoError<DescribeCertificatesError>>;

    /// <p>Returns a list of <code>DBClusterParameterGroup</code> descriptions. If a <code>DBClusterParameterGroupName</code> parameter is specified, the list contains only the description of the specified DB cluster parameter group. </p>
    async fn describe_db_cluster_parameter_groups(
        &self,
        input: DescribeDBClusterParameterGroupsMessage,
    ) -> Result<DBClusterParameterGroupsMessage, RusotoError<DescribeDBClusterParameterGroupsError>>;

    /// <p>Returns the detailed parameter list for a particular DB cluster parameter group.</p>
    async fn describe_db_cluster_parameters(
        &self,
        input: DescribeDBClusterParametersMessage,
    ) -> Result<DBClusterParameterGroupDetails, RusotoError<DescribeDBClusterParametersError>>;

    /// <p>Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot.</p> <p>When you share snapshots with other AWS accounts, <code>DescribeDBClusterSnapshotAttributes</code> returns the <code>restore</code> attribute and a list of IDs for the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If <code>all</code> is included in the list of values for the <code>restore</code> attribute, then the manual DB cluster snapshot is public and can be copied or restored by all AWS accounts.</p>
    async fn describe_db_cluster_snapshot_attributes(
        &self,
        input: DescribeDBClusterSnapshotAttributesMessage,
    ) -> Result<
        DescribeDBClusterSnapshotAttributesResult,
        RusotoError<DescribeDBClusterSnapshotAttributesError>,
    >;

    /// <p>Returns information about DB cluster snapshots. This API operation supports pagination.</p>
    async fn describe_db_cluster_snapshots(
        &self,
        input: DescribeDBClusterSnapshotsMessage,
    ) -> Result<DBClusterSnapshotMessage, RusotoError<DescribeDBClusterSnapshotsError>>;

    /// <p>Returns information about provisioned Amazon DocumentDB DB clusters. This API operation supports pagination.</p>
    async fn describe_db_clusters(
        &self,
        input: DescribeDBClustersMessage,
    ) -> Result<DBClusterMessage, RusotoError<DescribeDBClustersError>>;

    /// <p>Returns a list of the available DB engines.</p>
    async fn describe_db_engine_versions(
        &self,
        input: DescribeDBEngineVersionsMessage,
    ) -> Result<DBEngineVersionMessage, RusotoError<DescribeDBEngineVersionsError>>;

    /// <p>Returns information about provisioned Amazon DocumentDB instances. This API supports pagination.</p>
    async fn describe_db_instances(
        &self,
        input: DescribeDBInstancesMessage,
    ) -> Result<DBInstanceMessage, RusotoError<DescribeDBInstancesError>>;

    /// <p>Returns a list of <code>DBSubnetGroup</code> descriptions. If a <code>DBSubnetGroupName</code> is specified, the list will contain only the descriptions of the specified <code>DBSubnetGroup</code>.</p>
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

    /// <p>Displays a list of categories for all event source types, or, if specified, for a specified source type. </p>
    async fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> Result<EventCategoriesMessage, RusotoError<DescribeEventCategoriesError>>;

    /// <p>Returns events related to DB instances, DB security groups, DB snapshots, and DB parameter groups for the past 14 days. You can obtain events specific to a particular DB instance, DB security group, DB snapshot, or DB parameter group by providing the name as a parameter. By default, the events of the past hour are returned.</p>
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

    /// <p>Forces a failover for a DB cluster.</p> <p>A failover for a DB cluster promotes one of the Amazon DocumentDB replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer).</p> <p>If the primary instance fails, Amazon DocumentDB automatically fails over to an Amazon DocumentDB replica, if one exists. You can force a failover when you want to simulate a failure of a primary instance for testing.</p>
    async fn failover_db_cluster(
        &self,
        input: FailoverDBClusterMessage,
    ) -> Result<FailoverDBClusterResult, RusotoError<FailoverDBClusterError>>;

    /// <p>Lists all tags on an Amazon DocumentDB resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> Result<TagListMessage, RusotoError<ListTagsForResourceError>>;

    /// <p>Modifies a setting for an Amazon DocumentDB DB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. </p>
    async fn modify_db_cluster(
        &self,
        input: ModifyDBClusterMessage,
    ) -> Result<ModifyDBClusterResult, RusotoError<ModifyDBClusterError>>;

    /// <p><p> Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request. </p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot or maintenance window before the change can take effect.</p> </note> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon DocumentDB to fully complete the create action before the parameter group is used as the default for a new DB cluster. This step is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter.</p> </important></p>
    async fn modify_db_cluster_parameter_group(
        &self,
        input: ModifyDBClusterParameterGroupMessage,
    ) -> Result<DBClusterParameterGroupNameMessage, RusotoError<ModifyDBClusterParameterGroupError>>;

    /// <p>Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot.</p> <p>To share a manual DB cluster snapshot with other AWS accounts, specify <code>restore</code> as the <code>AttributeName</code>, and use the <code>ValuesToAdd</code> parameter to add a list of IDs of the AWS accounts that are authorized to restore the manual DB cluster snapshot. Use the value <code>all</code> to make the manual DB cluster snapshot public, which means that it can be copied or restored by all AWS accounts. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts. If a manual DB cluster snapshot is encrypted, it can be shared, but only by specifying a list of authorized AWS account IDs for the <code>ValuesToAdd</code> parameter. You can't use <code>all</code> as a value for that parameter in this case.</p>
    async fn modify_db_cluster_snapshot_attribute(
        &self,
        input: ModifyDBClusterSnapshotAttributeMessage,
    ) -> Result<
        ModifyDBClusterSnapshotAttributeResult,
        RusotoError<ModifyDBClusterSnapshotAttributeError>,
    >;

    /// <p>Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request.</p>
    async fn modify_db_instance(
        &self,
        input: ModifyDBInstanceMessage,
    ) -> Result<ModifyDBInstanceResult, RusotoError<ModifyDBInstanceError>>;

    /// <p>Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two Availability Zones in the AWS Region.</p>
    async fn modify_db_subnet_group(
        &self,
        input: ModifyDBSubnetGroupMessage,
    ) -> Result<ModifyDBSubnetGroupResult, RusotoError<ModifyDBSubnetGroupError>>;

    /// <p>You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain changes, or if you change the DB cluster parameter group that is associated with the DB instance, you must reboot the instance for the changes to take effect. </p> <p>Rebooting a DB instance restarts the database engine service. Rebooting a DB instance results in a momentary outage, during which the DB instance status is set to <i>rebooting</i>. </p>
    async fn reboot_db_instance(
        &self,
        input: RebootDBInstanceMessage,
    ) -> Result<RebootDBInstanceResult, RusotoError<RebootDBInstanceError>>;

    /// <p>Removes metadata tags from an Amazon DocumentDB resource.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> Result<(), RusotoError<RemoveTagsFromResourceError>>;

    /// <p> Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters, submit a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB cluster parameter group, specify the <code>DBClusterParameterGroupName</code> and <code>ResetAllParameters</code> parameters. </p> <p> When you reset the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance reboot.</p>
    async fn reset_db_cluster_parameter_group(
        &self,
        input: ResetDBClusterParameterGroupMessage,
    ) -> Result<DBClusterParameterGroupNameMessage, RusotoError<ResetDBClusterParameterGroupError>>;

    /// <p>Creates a new DB cluster from a DB snapshot or DB cluster snapshot.</p> <p>If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.</p> <p>If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.</p>
    async fn restore_db_cluster_from_snapshot(
        &self,
        input: RestoreDBClusterFromSnapshotMessage,
    ) -> Result<RestoreDBClusterFromSnapshotResult, RusotoError<RestoreDBClusterFromSnapshotError>>;

    /// <p>Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before <code>LatestRestorableTime</code> for up to <code>BackupRetentionPeriod</code> days. The target DB cluster is created from the source DB cluster with the same configuration as the original DB cluster, except that the new DB cluster is created with the default DB security group. </p>
    async fn restore_db_cluster_to_point_in_time(
        &self,
        input: RestoreDBClusterToPointInTimeMessage,
    ) -> Result<RestoreDBClusterToPointInTimeResult, RusotoError<RestoreDBClusterToPointInTimeError>>;

    /// <p>Restarts the stopped cluster that is specified by <code>DBClusterIdentifier</code>. For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/db-cluster-stop-start.html">Stopping and Starting an Amazon DocumentDB Cluster</a>.</p>
    async fn start_db_cluster(
        &self,
        input: StartDBClusterMessage,
    ) -> Result<StartDBClusterResult, RusotoError<StartDBClusterError>>;

    /// <p>Stops the running cluster that is specified by <code>DBClusterIdentifier</code>. The cluster must be in the <i>available</i> state. For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/db-cluster-stop-start.html">Stopping and Starting an Amazon DocumentDB Cluster</a>.</p>
    async fn stop_db_cluster(
        &self,
        input: StopDBClusterMessage,
    ) -> Result<StopDBClusterResult, RusotoError<StopDBClusterError>>;
}
/// A client for the Amazon DocDB API.
#[derive(Clone)]
pub struct DocdbClient {
    client: Client,
    region: region::Region,
}

impl DocdbClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DocdbClient {
        DocdbClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DocdbClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DocdbClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DocdbClient {
        DocdbClient { client, region }
    }
}

#[async_trait]
impl Docdb for DocdbClient {
    /// <p>Adds metadata tags to an Amazon DocumentDB resource. You can use these tags with cost allocation reporting to track costs that are associated with Amazon DocumentDB resources. or in a <code>Condition</code> statement in an AWS Identity and Access Management (IAM) policy for Amazon DocumentDB.</p>
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

        Ok(std::mem::drop(response))
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

    /// <p>Copies a snapshot of a DB cluster.</p> <p>To copy a DB cluster snapshot from a shared manual DB cluster snapshot, <code>SourceDBClusterSnapshotIdentifier</code> must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot.</p> <p>To cancel the copy operation after it is in progress, delete the target DB cluster snapshot identified by <code>TargetDBClusterSnapshotIdentifier</code> while that DB cluster snapshot is in the <i>copying</i> status.</p>
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

    /// <p>Creates a new Amazon DocumentDB DB cluster.</p>
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

    /// <p><p>Creates a new DB cluster parameter group.</p> <p>Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster.</p> <p>A DB cluster parameter group is initially created with the default parameters for the database engine used by instances in the DB cluster. To provide custom values for any of the parameters, you must modify the group after you create it. After you create a DB cluster parameter group, you must associate it with your DB cluster. For the new DB cluster parameter group and associated settings to take effect, you must then reboot the DB instances in the DB cluster without failover.</p> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon DocumentDB to fully complete the create action before the DB cluster parameter group is used as the default for a new DB cluster. This step is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter.</p> </important></p>
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

    /// <p>Creates a snapshot of a DB cluster. </p>
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

    /// <p>Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two Availability Zones in the AWS Region.</p>
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

    /// <p><p>Deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can&#39;t be recovered. Manual DB cluster snapshots of the specified DB cluster are not deleted.</p> <p/></p>
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

        Ok(std::mem::drop(response))
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

    /// <p>Deletes a previously provisioned DB instance. </p>
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

        Ok(std::mem::drop(response))
    }

    /// <p>Returns a list of certificate authority (CA) certificates provided by Amazon RDS for this AWS account.</p>
    async fn describe_certificates(
        &self,
        input: DescribeCertificatesMessage,
    ) -> Result<CertificateMessage, RusotoError<DescribeCertificatesError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeCertificates");
        params.put("Version", "2014-10-31");
        DescribeCertificatesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeCertificatesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CertificateMessage::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CertificateMessageDeserializer::deserialize(
                "DescribeCertificatesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of <code>DBClusterParameterGroup</code> descriptions. If a <code>DBClusterParameterGroupName</code> parameter is specified, the list contains only the description of the specified DB cluster parameter group. </p>
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

    /// <p>Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot.</p> <p>When you share snapshots with other AWS accounts, <code>DescribeDBClusterSnapshotAttributes</code> returns the <code>restore</code> attribute and a list of IDs for the AWS accounts that are authorized to copy or restore the manual DB cluster snapshot. If <code>all</code> is included in the list of values for the <code>restore</code> attribute, then the manual DB cluster snapshot is public and can be copied or restored by all AWS accounts.</p>
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

    /// <p>Returns information about DB cluster snapshots. This API operation supports pagination.</p>
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

    /// <p>Returns information about provisioned Amazon DocumentDB DB clusters. This API operation supports pagination.</p>
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

    /// <p>Returns information about provisioned Amazon DocumentDB instances. This API supports pagination.</p>
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

    /// <p>Returns a list of <code>DBSubnetGroup</code> descriptions. If a <code>DBSubnetGroupName</code> is specified, the list will contain only the descriptions of the specified <code>DBSubnetGroup</code>.</p>
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

    /// <p>Displays a list of categories for all event source types, or, if specified, for a specified source type. </p>
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

    /// <p>Returns events related to DB instances, DB security groups, DB snapshots, and DB parameter groups for the past 14 days. You can obtain events specific to a particular DB instance, DB security group, DB snapshot, or DB parameter group by providing the name as a parameter. By default, the events of the past hour are returned.</p>
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

    /// <p>Forces a failover for a DB cluster.</p> <p>A failover for a DB cluster promotes one of the Amazon DocumentDB replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer).</p> <p>If the primary instance fails, Amazon DocumentDB automatically fails over to an Amazon DocumentDB replica, if one exists. You can force a failover when you want to simulate a failure of a primary instance for testing.</p>
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

    /// <p>Lists all tags on an Amazon DocumentDB resource.</p>
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

    /// <p>Modifies a setting for an Amazon DocumentDB DB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. </p>
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

    /// <p><p> Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request. </p> <note> <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot or maintenance window before the change can take effect.</p> </note> <important> <p>After you create a DB cluster parameter group, you should wait at least 5 minutes before creating your first DB cluster that uses that DB cluster parameter group as the default parameter group. This allows Amazon DocumentDB to fully complete the create action before the parameter group is used as the default for a new DB cluster. This step is especially important for parameters that are critical when creating the default database for a DB cluster, such as the character set for the default database defined by the <code>character<em>set</em>database</code> parameter.</p> </important></p>
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

    /// <p>Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot.</p> <p>To share a manual DB cluster snapshot with other AWS accounts, specify <code>restore</code> as the <code>AttributeName</code>, and use the <code>ValuesToAdd</code> parameter to add a list of IDs of the AWS accounts that are authorized to restore the manual DB cluster snapshot. Use the value <code>all</code> to make the manual DB cluster snapshot public, which means that it can be copied or restored by all AWS accounts. Do not add the <code>all</code> value for any manual DB cluster snapshots that contain private information that you don't want available to all AWS accounts. If a manual DB cluster snapshot is encrypted, it can be shared, but only by specifying a list of authorized AWS account IDs for the <code>ValuesToAdd</code> parameter. You can't use <code>all</code> as a value for that parameter in this case.</p>
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

    /// <p>Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request.</p>
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

    /// <p>Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two Availability Zones in the AWS Region.</p>
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

    /// <p>You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain changes, or if you change the DB cluster parameter group that is associated with the DB instance, you must reboot the instance for the changes to take effect. </p> <p>Rebooting a DB instance restarts the database engine service. Rebooting a DB instance results in a momentary outage, during which the DB instance status is set to <i>rebooting</i>. </p>
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

    /// <p>Removes metadata tags from an Amazon DocumentDB resource.</p>
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

        Ok(std::mem::drop(response))
    }

    /// <p> Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters, submit a list of the following: <code>ParameterName</code> and <code>ApplyMethod</code>. To reset the entire DB cluster parameter group, specify the <code>DBClusterParameterGroupName</code> and <code>ResetAllParameters</code> parameters. </p> <p> When you reset the entire group, dynamic parameters are updated immediately and static parameters are set to <code>pending-reboot</code> to take effect on the next DB instance reboot.</p>
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

    /// <p>Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before <code>LatestRestorableTime</code> for up to <code>BackupRetentionPeriod</code> days. The target DB cluster is created from the source DB cluster with the same configuration as the original DB cluster, except that the new DB cluster is created with the default DB security group. </p>
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

    /// <p>Restarts the stopped cluster that is specified by <code>DBClusterIdentifier</code>. For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/db-cluster-stop-start.html">Stopping and Starting an Amazon DocumentDB Cluster</a>.</p>
    async fn start_db_cluster(
        &self,
        input: StartDBClusterMessage,
    ) -> Result<StartDBClusterResult, RusotoError<StartDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "StartDBCluster");
        params.put("Version", "2014-10-31");
        StartDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(StartDBClusterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = StartDBClusterResult::default();
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
                StartDBClusterResultDeserializer::deserialize("StartDBClusterResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Stops the running cluster that is specified by <code>DBClusterIdentifier</code>. The cluster must be in the <i>available</i> state. For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/db-cluster-stop-start.html">Stopping and Starting an Amazon DocumentDB Cluster</a>.</p>
    async fn stop_db_cluster(
        &self,
        input: StopDBClusterMessage,
    ) -> Result<StopDBClusterResult, RusotoError<StopDBClusterError>> {
        let mut request = SignedRequest::new("POST", "rds", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "StopDBCluster");
        params.put("Version", "2014-10-31");
        StopDBClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(StopDBClusterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = StopDBClusterResult::default();
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
                StopDBClusterResultDeserializer::deserialize("StopDBClusterResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}
