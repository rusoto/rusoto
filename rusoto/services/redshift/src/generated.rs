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
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
/// <p>Describes an AWS customer account authorized to restore a snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccountWithRestoreAccess {
    /// <p>The identifier of an AWS support account authorized to restore a snapshot. For AWS support, the identifier is <code>amazon-redshift-support</code>. </p>
    pub account_alias: Option<String>,
    /// <p>The identifier of an AWS customer account authorized to restore a snapshot.</p>
    pub account_id: Option<String>,
}

struct AccountWithRestoreAccessDeserializer;
impl AccountWithRestoreAccessDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccountWithRestoreAccess, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AccountWithRestoreAccess::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccountAlias" => {
                        obj.account_alias =
                            Some(try!(StringDeserializer::deserialize("AccountAlias", stack)));
                    }
                    "AccountId" => {
                        obj.account_id =
                            Some(try!(StringDeserializer::deserialize("AccountId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AccountsWithRestoreAccessListDeserializer;
impl AccountsWithRestoreAccessListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AccountWithRestoreAccess>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "AccountWithRestoreAccess" {
                        obj.push(try!(AccountWithRestoreAccessDeserializer::deserialize(
                            "AccountWithRestoreAccess",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AuthorizeClusterSecurityGroupIngressMessage {
    /// <p>The IP range to be added the Amazon Redshift security group.</p>
    pub cidrip: Option<String>,
    /// <p>The name of the security group to which the ingress rule is added.</p>
    pub cluster_security_group_name: String,
    /// <p>The EC2 security group to be added the Amazon Redshift security group.</p>
    pub ec2_security_group_name: Option<String>,
    /// <p>The AWS account number of the owner of the security group specified by the <i>EC2SecurityGroupName</i> parameter. The AWS Access Key ID is not an acceptable value. </p> <p>Example: <code>111122223333</code> </p>
    pub ec2_security_group_owner_id: Option<String>,
}

/// Serialize `AuthorizeClusterSecurityGroupIngressMessage` contents to a `SignedRequest`.
struct AuthorizeClusterSecurityGroupIngressMessageSerializer;
impl AuthorizeClusterSecurityGroupIngressMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &AuthorizeClusterSecurityGroupIngressMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cidrip {
            params.put(
                &format!("{}{}", prefix, "CIDRIP"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ClusterSecurityGroupName"),
            &obj.cluster_security_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.ec2_security_group_name {
            params.put(
                &format!("{}{}", prefix, "EC2SecurityGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.ec2_security_group_owner_id {
            params.put(
                &format!("{}{}", prefix, "EC2SecurityGroupOwnerId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AuthorizeClusterSecurityGroupIngressResult {
    pub cluster_security_group: Option<ClusterSecurityGroup>,
}

struct AuthorizeClusterSecurityGroupIngressResultDeserializer;
impl AuthorizeClusterSecurityGroupIngressResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AuthorizeClusterSecurityGroupIngressResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AuthorizeClusterSecurityGroupIngressResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSecurityGroup" => {
                        obj.cluster_security_group =
                            Some(try!(ClusterSecurityGroupDeserializer::deserialize(
                                "ClusterSecurityGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AuthorizeSnapshotAccessMessage {
    /// <p>The identifier of the AWS customer account authorized to restore the specified snapshot.</p> <p>To share a snapshot with AWS support, specify amazon-redshift-support.</p>
    pub account_with_restore_access: String,
    /// <p>The identifier of the cluster the snapshot was created from. This parameter is required if your IAM user has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.</p>
    pub snapshot_cluster_identifier: Option<String>,
    /// <p>The identifier of the snapshot the account is authorized to restore.</p>
    pub snapshot_identifier: String,
}

/// Serialize `AuthorizeSnapshotAccessMessage` contents to a `SignedRequest`.
struct AuthorizeSnapshotAccessMessageSerializer;
impl AuthorizeSnapshotAccessMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AuthorizeSnapshotAccessMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AccountWithRestoreAccess"),
            &obj.account_with_restore_access.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.snapshot_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "SnapshotClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AuthorizeSnapshotAccessResult {
    pub snapshot: Option<Snapshot>,
}

struct AuthorizeSnapshotAccessResultDeserializer;
impl AuthorizeSnapshotAccessResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AuthorizeSnapshotAccessResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AuthorizeSnapshotAccessResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Snapshot" => {
                        obj.snapshot =
                            Some(try!(SnapshotDeserializer::deserialize("Snapshot", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes an availability zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AvailabilityZone {
    /// <p>The name of the availability zone.</p>
    pub name: Option<String>,
    pub supported_platforms: Option<Vec<SupportedPlatform>>,
}

struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AvailabilityZone, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AvailabilityZone::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Name" => {
                        obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                    }
                    "SupportedPlatforms" => {
                        obj.supported_platforms =
                            Some(try!(SupportedPlatformsListDeserializer::deserialize(
                                "SupportedPlatforms",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AvailabilityZoneListDeserializer;
impl AvailabilityZoneListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AvailabilityZone>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "AvailabilityZone" {
                        obj.push(try!(AvailabilityZoneDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BooleanOptionalDeserializer;
impl BooleanOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Cluster {
    /// <p>A Boolean value that, if <code>true</code>, indicates that major version upgrades will be applied automatically to the cluster during the maintenance window. </p>
    pub allow_version_upgrade: Option<bool>,
    /// <p>The number of days that automatic cluster snapshots are retained.</p>
    pub automated_snapshot_retention_period: Option<i64>,
    /// <p>The name of the Availability Zone in which the cluster is located.</p>
    pub availability_zone: Option<String>,
    /// <p>The date and time that the cluster was created.</p>
    pub cluster_create_time: Option<String>,
    /// <p>The unique identifier of the cluster.</p>
    pub cluster_identifier: Option<String>,
    /// <p>The nodes in the cluster.</p>
    pub cluster_nodes: Option<Vec<ClusterNode>>,
    /// <p>The list of cluster parameter groups that are associated with this cluster. Each parameter group in the list is returned with its status.</p>
    pub cluster_parameter_groups: Option<Vec<ClusterParameterGroupStatus>>,
    /// <p>The public key for the cluster.</p>
    pub cluster_public_key: Option<String>,
    /// <p>The specific revision number of the database in the cluster.</p>
    pub cluster_revision_number: Option<String>,
    /// <p>A list of cluster security group that are associated with the cluster. Each security group is represented by an element that contains <code>ClusterSecurityGroup.Name</code> and <code>ClusterSecurityGroup.Status</code> subelements. </p> <p>Cluster security groups are used when the cluster is not created in an Amazon Virtual Private Cloud (VPC). Clusters that are created in a VPC use VPC security groups, which are listed by the <b>VpcSecurityGroups</b> parameter. </p>
    pub cluster_security_groups: Option<Vec<ClusterSecurityGroupMembership>>,
    /// <p>A value that returns the destination region and retention period that are configured for cross-region snapshot copy.</p>
    pub cluster_snapshot_copy_status: Option<ClusterSnapshotCopyStatus>,
    /// <p><p> The current state of the cluster. Possible values are the following:</p> <ul> <li> <p> <code>available</code> </p> </li> <li> <p> <code>creating</code> </p> </li> <li> <p> <code>deleting</code> </p> </li> <li> <p> <code>final-snapshot</code> </p> </li> <li> <p> <code>hardware-failure</code> </p> </li> <li> <p> <code>incompatible-hsm</code> </p> </li> <li> <p> <code>incompatible-network</code> </p> </li> <li> <p> <code>incompatible-parameters</code> </p> </li> <li> <p> <code>incompatible-restore</code> </p> </li> <li> <p> <code>modifying</code> </p> </li> <li> <p> <code>rebooting</code> </p> </li> <li> <p> <code>renaming</code> </p> </li> <li> <p> <code>resizing</code> </p> </li> <li> <p> <code>rotating-keys</code> </p> </li> <li> <p> <code>storage-full</code> </p> </li> <li> <p> <code>updating-hsm</code> </p> </li> </ul></p>
    pub cluster_status: Option<String>,
    /// <p>The name of the subnet group that is associated with the cluster. This parameter is valid only when the cluster is in a VPC.</p>
    pub cluster_subnet_group_name: Option<String>,
    /// <p>The version ID of the Amazon Redshift engine that is running on the cluster.</p>
    pub cluster_version: Option<String>,
    /// <p>The name of the initial database that was created when the cluster was created. This same name is returned for the life of the cluster. If an initial database was not specified, a database named <code>dev</code>dev was created by default. </p>
    pub db_name: Option<String>,
    /// <p>The status of the elastic IP (EIP) address.</p>
    pub elastic_ip_status: Option<ElasticIpStatus>,
    /// <p>A Boolean value that, if <code>true</code>, indicates that data in the cluster is encrypted at rest.</p>
    pub encrypted: Option<bool>,
    /// <p>The connection endpoint.</p>
    pub endpoint: Option<Endpoint>,
    /// <p>An option that specifies whether to create the cluster with enhanced VPC routing enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a VPC. For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/enhanced-vpc-routing.html">Enhanced VPC Routing</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If this option is <code>true</code>, enhanced VPC routing is enabled. </p> <p>Default: false</p>
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>A value that reports whether the Amazon Redshift cluster has finished applying any hardware security module (HSM) settings changes specified in a modify cluster command.</p> <p>Values: active, applying</p>
    pub hsm_status: Option<HsmStatus>,
    /// <p>A list of AWS Identity and Access Management (IAM) roles that can be used by the cluster to access other AWS services.</p>
    pub iam_roles: Option<Vec<ClusterIamRole>>,
    /// <p>The AWS Key Management Service (AWS KMS) key ID of the encryption key used to encrypt data in the cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p>The master user name for the cluster. This name is used to connect to the database that is specified in the <b>DBName</b> parameter. </p>
    pub master_username: Option<String>,
    /// <p>The status of a modify operation, if any, initiated for the cluster.</p>
    pub modify_status: Option<String>,
    /// <p>The node type for the nodes in the cluster.</p>
    pub node_type: Option<String>,
    /// <p>The number of compute nodes in the cluster.</p>
    pub number_of_nodes: Option<i64>,
    /// <p>A value that, if present, indicates that changes to the cluster are pending. Specific pending changes are identified by subelements.</p>
    pub pending_modified_values: Option<PendingModifiedValues>,
    /// <p>The weekly time range, in Universal Coordinated Time (UTC), during which system maintenance can occur.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>A Boolean value that, if <code>true</code>, indicates that the cluster can be accessed from a public network.</p>
    pub publicly_accessible: Option<bool>,
    /// <p>A value that describes the status of a cluster restore action. This parameter returns null if the cluster was not created by restoring a snapshot.</p>
    pub restore_status: Option<RestoreStatus>,
    /// <p>The list of tags for the cluster.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The identifier of the VPC the cluster is in, if the cluster is in a VPC.</p>
    pub vpc_id: Option<String>,
    /// <p>A list of Amazon Virtual Private Cloud (Amazon VPC) security groups that are associated with the cluster. This parameter is returned only if the cluster is in a VPC.</p>
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

struct ClusterDeserializer;
impl ClusterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Cluster, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Cluster::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllowVersionUpgrade" => {
                        obj.allow_version_upgrade = Some(try!(BooleanDeserializer::deserialize(
                            "AllowVersionUpgrade",
                            stack
                        )));
                    }
                    "AutomatedSnapshotRetentionPeriod" => {
                        obj.automated_snapshot_retention_period =
                            Some(try!(IntegerDeserializer::deserialize(
                                "AutomatedSnapshotRetentionPeriod",
                                stack
                            )));
                    }
                    "AvailabilityZone" => {
                        obj.availability_zone = Some(try!(StringDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    }
                    "ClusterCreateTime" => {
                        obj.cluster_create_time = Some(try!(TStampDeserializer::deserialize(
                            "ClusterCreateTime",
                            stack
                        )));
                    }
                    "ClusterIdentifier" => {
                        obj.cluster_identifier = Some(try!(StringDeserializer::deserialize(
                            "ClusterIdentifier",
                            stack
                        )));
                    }
                    "ClusterNodes" => {
                        obj.cluster_nodes = Some(try!(ClusterNodesListDeserializer::deserialize(
                            "ClusterNodes",
                            stack
                        )));
                    }
                    "ClusterParameterGroups" => {
                        obj.cluster_parameter_groups = Some(try!(
                            ClusterParameterGroupStatusListDeserializer::deserialize(
                                "ClusterParameterGroups",
                                stack
                            )
                        ));
                    }
                    "ClusterPublicKey" => {
                        obj.cluster_public_key = Some(try!(StringDeserializer::deserialize(
                            "ClusterPublicKey",
                            stack
                        )));
                    }
                    "ClusterRevisionNumber" => {
                        obj.cluster_revision_number = Some(try!(StringDeserializer::deserialize(
                            "ClusterRevisionNumber",
                            stack
                        )));
                    }
                    "ClusterSecurityGroups" => {
                        obj.cluster_security_groups = Some(try!(
                            ClusterSecurityGroupMembershipListDeserializer::deserialize(
                                "ClusterSecurityGroups",
                                stack
                            )
                        ));
                    }
                    "ClusterSnapshotCopyStatus" => {
                        obj.cluster_snapshot_copy_status =
                            Some(try!(ClusterSnapshotCopyStatusDeserializer::deserialize(
                                "ClusterSnapshotCopyStatus",
                                stack
                            )));
                    }
                    "ClusterStatus" => {
                        obj.cluster_status = Some(try!(StringDeserializer::deserialize(
                            "ClusterStatus",
                            stack
                        )));
                    }
                    "ClusterSubnetGroupName" => {
                        obj.cluster_subnet_group_name = Some(try!(
                            StringDeserializer::deserialize("ClusterSubnetGroupName", stack)
                        ));
                    }
                    "ClusterVersion" => {
                        obj.cluster_version = Some(try!(StringDeserializer::deserialize(
                            "ClusterVersion",
                            stack
                        )));
                    }
                    "DBName" => {
                        obj.db_name = Some(try!(StringDeserializer::deserialize("DBName", stack)));
                    }
                    "ElasticIpStatus" => {
                        obj.elastic_ip_status = Some(try!(
                            ElasticIpStatusDeserializer::deserialize("ElasticIpStatus", stack)
                        ));
                    }
                    "Encrypted" => {
                        obj.encrypted =
                            Some(try!(BooleanDeserializer::deserialize("Encrypted", stack)));
                    }
                    "Endpoint" => {
                        obj.endpoint =
                            Some(try!(EndpointDeserializer::deserialize("Endpoint", stack)));
                    }
                    "EnhancedVpcRouting" => {
                        obj.enhanced_vpc_routing = Some(try!(BooleanDeserializer::deserialize(
                            "EnhancedVpcRouting",
                            stack
                        )));
                    }
                    "HsmStatus" => {
                        obj.hsm_status =
                            Some(try!(HsmStatusDeserializer::deserialize("HsmStatus", stack)));
                    }
                    "IamRoles" => {
                        obj.iam_roles = Some(try!(ClusterIamRoleListDeserializer::deserialize(
                            "IamRoles", stack
                        )));
                    }
                    "KmsKeyId" => {
                        obj.kms_key_id =
                            Some(try!(StringDeserializer::deserialize("KmsKeyId", stack)));
                    }
                    "MasterUsername" => {
                        obj.master_username = Some(try!(StringDeserializer::deserialize(
                            "MasterUsername",
                            stack
                        )));
                    }
                    "ModifyStatus" => {
                        obj.modify_status =
                            Some(try!(StringDeserializer::deserialize("ModifyStatus", stack)));
                    }
                    "NodeType" => {
                        obj.node_type =
                            Some(try!(StringDeserializer::deserialize("NodeType", stack)));
                    }
                    "NumberOfNodes" => {
                        obj.number_of_nodes = Some(try!(IntegerDeserializer::deserialize(
                            "NumberOfNodes",
                            stack
                        )));
                    }
                    "PendingModifiedValues" => {
                        obj.pending_modified_values =
                            Some(try!(PendingModifiedValuesDeserializer::deserialize(
                                "PendingModifiedValues",
                                stack
                            )));
                    }
                    "PreferredMaintenanceWindow" => {
                        obj.preferred_maintenance_window = Some(try!(
                            StringDeserializer::deserialize("PreferredMaintenanceWindow", stack)
                        ));
                    }
                    "PubliclyAccessible" => {
                        obj.publicly_accessible = Some(try!(BooleanDeserializer::deserialize(
                            "PubliclyAccessible",
                            stack
                        )));
                    }
                    "RestoreStatus" => {
                        obj.restore_status = Some(try!(RestoreStatusDeserializer::deserialize(
                            "RestoreStatus",
                            stack
                        )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    "VpcId" => {
                        obj.vpc_id = Some(try!(StringDeserializer::deserialize("VpcId", stack)));
                    }
                    "VpcSecurityGroups" => {
                        obj.vpc_security_groups = Some(try!(
                            VpcSecurityGroupMembershipListDeserializer::deserialize(
                                "VpcSecurityGroups",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Temporary credentials with authorization to log on to an Amazon Redshift database. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterCredentials {
    /// <p>A temporary password that authorizes the user name returned by <code>DbUser</code> to log on to the database <code>DbName</code>. </p>
    pub db_password: Option<String>,
    /// <p>A database user name that is authorized to log on to the database <code>DbName</code> using the password <code>DbPassword</code>. If the specified DbUser exists in the database, the new user name has the same database privileges as the the user named in DbUser. By default, the user is added to PUBLIC. If the <code>DbGroups</code> parameter is specifed, <code>DbUser</code> is added to the listed groups for any sessions created using these credentials.</p>
    pub db_user: Option<String>,
    /// <p>The date and time the password in <code>DbPassword</code> expires.</p>
    pub expiration: Option<String>,
}

struct ClusterCredentialsDeserializer;
impl ClusterCredentialsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterCredentials, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterCredentials::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DbPassword" => {
                        obj.db_password = Some(try!(SensitiveStringDeserializer::deserialize(
                            "DbPassword",
                            stack
                        )));
                    }
                    "DbUser" => {
                        obj.db_user = Some(try!(StringDeserializer::deserialize("DbUser", stack)));
                    }
                    "Expiration" => {
                        obj.expiration =
                            Some(try!(TStampDeserializer::deserialize("Expiration", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>An AWS Identity and Access Management (IAM) role that can be used by the associated Amazon Redshift cluster to access other AWS services.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterIamRole {
    /// <p><p>A value that describes the status of the IAM role&#39;s association with an Amazon Redshift cluster.</p> <p>The following are possible statuses and descriptions.</p> <ul> <li> <p> <code>in-sync</code>: The role is available for use by the cluster.</p> </li> <li> <p> <code>adding</code>: The role is in the process of being associated with the cluster.</p> </li> <li> <p> <code>removing</code>: The role is in the process of being disassociated with the cluster.</p> </li> </ul></p>
    pub apply_status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role, for example, <code>arn:aws:iam::123456789012:role/RedshiftCopyUnload</code>. </p>
    pub iam_role_arn: Option<String>,
}

struct ClusterIamRoleDeserializer;
impl ClusterIamRoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterIamRole, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterIamRole::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ApplyStatus" => {
                        obj.apply_status =
                            Some(try!(StringDeserializer::deserialize("ApplyStatus", stack)));
                    }
                    "IamRoleArn" => {
                        obj.iam_role_arn =
                            Some(try!(StringDeserializer::deserialize("IamRoleArn", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ClusterIamRoleListDeserializer;
impl ClusterIamRoleListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterIamRole>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ClusterIamRole" {
                        obj.push(try!(ClusterIamRoleDeserializer::deserialize(
                            "ClusterIamRole",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ClusterListDeserializer;
impl ClusterListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Cluster>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Cluster" {
                        obj.push(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>The identifier of a node in a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterNode {
    /// <p>Whether the node is a leader node or a compute node.</p>
    pub node_role: Option<String>,
    /// <p>The private IP address of a node within a cluster.</p>
    pub private_ip_address: Option<String>,
    /// <p>The public IP address of a node within a cluster.</p>
    pub public_ip_address: Option<String>,
}

struct ClusterNodeDeserializer;
impl ClusterNodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterNode, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterNode::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "NodeRole" => {
                        obj.node_role =
                            Some(try!(StringDeserializer::deserialize("NodeRole", stack)));
                    }
                    "PrivateIPAddress" => {
                        obj.private_ip_address = Some(try!(StringDeserializer::deserialize(
                            "PrivateIPAddress",
                            stack
                        )));
                    }
                    "PublicIPAddress" => {
                        obj.public_ip_address = Some(try!(StringDeserializer::deserialize(
                            "PublicIPAddress",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ClusterNodesListDeserializer;
impl ClusterNodesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterNode>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ClusterNodeDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Describes a parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterParameterGroup {
    /// <p>The description of the parameter group.</p>
    pub description: Option<String>,
    /// <p>The name of the cluster parameter group family that this cluster parameter group is compatible with.</p>
    pub parameter_group_family: Option<String>,
    /// <p>The name of the cluster parameter group.</p>
    pub parameter_group_name: Option<String>,
    /// <p>The list of tags for the cluster parameter group.</p>
    pub tags: Option<Vec<Tag>>,
}

struct ClusterParameterGroupDeserializer;
impl ClusterParameterGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterParameterGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterParameterGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "ParameterGroupFamily" => {
                        obj.parameter_group_family = Some(try!(StringDeserializer::deserialize(
                            "ParameterGroupFamily",
                            stack
                        )));
                    }
                    "ParameterGroupName" => {
                        obj.parameter_group_name = Some(try!(StringDeserializer::deserialize(
                            "ParameterGroupName",
                            stack
                        )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the output from the <a>DescribeClusterParameters</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterParameterGroupDetails {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>A list of <a>Parameter</a> instances. Each instance lists the parameters of one cluster parameter group. </p>
    pub parameters: Option<Vec<Parameter>>,
}

struct ClusterParameterGroupDetailsDeserializer;
impl ClusterParameterGroupDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterParameterGroupDetails, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterParameterGroupDetails::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersListDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterParameterGroupNameMessage {
    /// <p>The name of the cluster parameter group.</p>
    pub parameter_group_name: Option<String>,
    /// <p>The status of the parameter group. For example, if you made a change to a parameter group name-value pair, then the change could be pending a reboot of an associated cluster.</p>
    pub parameter_group_status: Option<String>,
}

struct ClusterParameterGroupNameMessageDeserializer;
impl ClusterParameterGroupNameMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterParameterGroupNameMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterParameterGroupNameMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ParameterGroupName" => {
                        obj.parameter_group_name = Some(try!(StringDeserializer::deserialize(
                            "ParameterGroupName",
                            stack
                        )));
                    }
                    "ParameterGroupStatus" => {
                        obj.parameter_group_status = Some(try!(StringDeserializer::deserialize(
                            "ParameterGroupStatus",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the status of a parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterParameterGroupStatus {
    /// <p>The list of parameter statuses.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    pub cluster_parameter_status_list: Option<Vec<ClusterParameterStatus>>,
    /// <p>The status of parameter updates.</p>
    pub parameter_apply_status: Option<String>,
    /// <p>The name of the cluster parameter group.</p>
    pub parameter_group_name: Option<String>,
}

struct ClusterParameterGroupStatusDeserializer;
impl ClusterParameterGroupStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterParameterGroupStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterParameterGroupStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterParameterStatusList" => {
                        obj.cluster_parameter_status_list =
                            Some(try!(ClusterParameterStatusListDeserializer::deserialize(
                                "ClusterParameterStatusList",
                                stack
                            )));
                    }
                    "ParameterApplyStatus" => {
                        obj.parameter_apply_status = Some(try!(StringDeserializer::deserialize(
                            "ParameterApplyStatus",
                            stack
                        )));
                    }
                    "ParameterGroupName" => {
                        obj.parameter_group_name = Some(try!(StringDeserializer::deserialize(
                            "ParameterGroupName",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ClusterParameterGroupStatusListDeserializer;
impl ClusterParameterGroupStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterParameterGroupStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ClusterParameterGroup" {
                        obj.push(try!(ClusterParameterGroupStatusDeserializer::deserialize(
                            "ClusterParameterGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the output from the <a>DescribeClusterParameterGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterParameterGroupsMessage {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>A list of <a>ClusterParameterGroup</a> instances. Each instance describes one cluster parameter group. </p>
    pub parameter_groups: Option<Vec<ClusterParameterGroup>>,
}

struct ClusterParameterGroupsMessageDeserializer;
impl ClusterParameterGroupsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterParameterGroupsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterParameterGroupsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "ParameterGroups" => {
                        obj.parameter_groups = Some(try!(
                            ParameterGroupListDeserializer::deserialize("ParameterGroups", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the status of a parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterParameterStatus {
    /// <p>The error that prevented the parameter from being applied to the database.</p>
    pub parameter_apply_error_description: Option<String>,
    /// <p><p>The status of the parameter that indicates whether the parameter is in sync with the database, waiting for a cluster reboot, or encountered an error when being applied.</p> <p>The following are possible statuses and descriptions.</p> <ul> <li> <p> <code>in-sync</code>: The parameter value is in sync with the database.</p> </li> <li> <p> <code>pending-reboot</code>: The parameter value will be applied after the cluster reboots.</p> </li> <li> <p> <code>applying</code>: The parameter value is being applied to the database.</p> </li> <li> <p> <code>invalid-parameter</code>: Cannot apply the parameter value because it has an invalid value or syntax.</p> </li> <li> <p> <code>apply-deferred</code>: The parameter contains static property changes. The changes are deferred until the cluster reboots.</p> </li> <li> <p> <code>apply-error</code>: Cannot connect to the cluster. The parameter change will be applied after the cluster reboots.</p> </li> <li> <p> <code>unknown-error</code>: Cannot apply the parameter change right now. The change will be applied after the cluster reboots.</p> </li> </ul></p>
    pub parameter_apply_status: Option<String>,
    /// <p>The name of the parameter.</p>
    pub parameter_name: Option<String>,
}

struct ClusterParameterStatusDeserializer;
impl ClusterParameterStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterParameterStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterParameterStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ParameterApplyErrorDescription" => {
                        obj.parameter_apply_error_description =
                            Some(try!(StringDeserializer::deserialize(
                                "ParameterApplyErrorDescription",
                                stack
                            )));
                    }
                    "ParameterApplyStatus" => {
                        obj.parameter_apply_status = Some(try!(StringDeserializer::deserialize(
                            "ParameterApplyStatus",
                            stack
                        )));
                    }
                    "ParameterName" => {
                        obj.parameter_name = Some(try!(StringDeserializer::deserialize(
                            "ParameterName",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ClusterParameterStatusListDeserializer;
impl ClusterParameterStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterParameterStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ClusterParameterStatusDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Describes a security group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterSecurityGroup {
    /// <p>The name of the cluster security group to which the operation was applied.</p>
    pub cluster_security_group_name: Option<String>,
    /// <p>A description of the security group.</p>
    pub description: Option<String>,
    /// <p>A list of EC2 security groups that are permitted to access clusters associated with this cluster security group.</p>
    pub ec2_security_groups: Option<Vec<EC2SecurityGroup>>,
    /// <p>A list of IP ranges (CIDR blocks) that are permitted to access clusters associated with this cluster security group.</p>
    pub ip_ranges: Option<Vec<IPRange>>,
    /// <p>The list of tags for the cluster security group.</p>
    pub tags: Option<Vec<Tag>>,
}

struct ClusterSecurityGroupDeserializer;
impl ClusterSecurityGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterSecurityGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterSecurityGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSecurityGroupName" => {
                        obj.cluster_security_group_name = Some(try!(
                            StringDeserializer::deserialize("ClusterSecurityGroupName", stack)
                        ));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "EC2SecurityGroups" => {
                        obj.ec2_security_groups =
                            Some(try!(EC2SecurityGroupListDeserializer::deserialize(
                                "EC2SecurityGroups",
                                stack
                            )));
                    }
                    "IPRanges" => {
                        obj.ip_ranges = Some(try!(IPRangeListDeserializer::deserialize(
                            "IPRanges", stack
                        )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a cluster security group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterSecurityGroupMembership {
    /// <p>The name of the cluster security group.</p>
    pub cluster_security_group_name: Option<String>,
    /// <p>The status of the cluster security group.</p>
    pub status: Option<String>,
}

struct ClusterSecurityGroupMembershipDeserializer;
impl ClusterSecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterSecurityGroupMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterSecurityGroupMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSecurityGroupName" => {
                        obj.cluster_security_group_name = Some(try!(
                            StringDeserializer::deserialize("ClusterSecurityGroupName", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ClusterSecurityGroupMembershipListDeserializer;
impl ClusterSecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterSecurityGroupMembership>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ClusterSecurityGroup" {
                        obj.push(try!(
                            ClusterSecurityGroupMembershipDeserializer::deserialize(
                                "ClusterSecurityGroup",
                                stack
                            )
                        ));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterSecurityGroupMessage {
    /// <p>A list of <a>ClusterSecurityGroup</a> instances. </p>
    pub cluster_security_groups: Option<Vec<ClusterSecurityGroup>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct ClusterSecurityGroupMessageDeserializer;
impl ClusterSecurityGroupMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterSecurityGroupMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterSecurityGroupMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSecurityGroups" => {
                        obj.cluster_security_groups =
                            Some(try!(ClusterSecurityGroupsDeserializer::deserialize(
                                "ClusterSecurityGroups",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `ClusterSecurityGroupNameList` contents to a `SignedRequest`.
struct ClusterSecurityGroupNameListSerializer;
impl ClusterSecurityGroupNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ClusterSecurityGroupsDeserializer;
impl ClusterSecurityGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterSecurityGroup>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ClusterSecurityGroup" {
                        obj.push(try!(ClusterSecurityGroupDeserializer::deserialize(
                            "ClusterSecurityGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Returns the destination region and retention period that are configured for cross-region snapshot copy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterSnapshotCopyStatus {
    /// <p>The destination region that snapshots are automatically copied to when cross-region snapshot copy is enabled.</p>
    pub destination_region: Option<String>,
    /// <p>The number of days that automated snapshots are retained in the destination region after they are copied from a source region.</p>
    pub retention_period: Option<i64>,
    /// <p>The name of the snapshot copy grant.</p>
    pub snapshot_copy_grant_name: Option<String>,
}

struct ClusterSnapshotCopyStatusDeserializer;
impl ClusterSnapshotCopyStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterSnapshotCopyStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterSnapshotCopyStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DestinationRegion" => {
                        obj.destination_region = Some(try!(StringDeserializer::deserialize(
                            "DestinationRegion",
                            stack
                        )));
                    }
                    "RetentionPeriod" => {
                        obj.retention_period = Some(try!(LongDeserializer::deserialize(
                            "RetentionPeriod",
                            stack
                        )));
                    }
                    "SnapshotCopyGrantName" => {
                        obj.snapshot_copy_grant_name = Some(try!(StringDeserializer::deserialize(
                            "SnapshotCopyGrantName",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a subnet group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterSubnetGroup {
    /// <p>The name of the cluster subnet group.</p>
    pub cluster_subnet_group_name: Option<String>,
    /// <p>The description of the cluster subnet group.</p>
    pub description: Option<String>,
    /// <p>The status of the cluster subnet group. Possible values are <code>Complete</code>, <code>Incomplete</code> and <code>Invalid</code>. </p>
    pub subnet_group_status: Option<String>,
    /// <p>A list of the VPC <a>Subnet</a> elements. </p>
    pub subnets: Option<Vec<Subnet>>,
    /// <p>The list of tags for the cluster subnet group.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The VPC ID of the cluster subnet group.</p>
    pub vpc_id: Option<String>,
}

struct ClusterSubnetGroupDeserializer;
impl ClusterSubnetGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterSubnetGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterSubnetGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSubnetGroupName" => {
                        obj.cluster_subnet_group_name = Some(try!(
                            StringDeserializer::deserialize("ClusterSubnetGroupName", stack)
                        ));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "SubnetGroupStatus" => {
                        obj.subnet_group_status = Some(try!(StringDeserializer::deserialize(
                            "SubnetGroupStatus",
                            stack
                        )));
                    }
                    "Subnets" => {
                        obj.subnets =
                            Some(try!(SubnetListDeserializer::deserialize("Subnets", stack)));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    "VpcId" => {
                        obj.vpc_id = Some(try!(StringDeserializer::deserialize("VpcId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the output from the <a>DescribeClusterSubnetGroups</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterSubnetGroupMessage {
    /// <p>A list of <a>ClusterSubnetGroup</a> instances. </p>
    pub cluster_subnet_groups: Option<Vec<ClusterSubnetGroup>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct ClusterSubnetGroupMessageDeserializer;
impl ClusterSubnetGroupMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterSubnetGroupMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterSubnetGroupMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSubnetGroups" => {
                        obj.cluster_subnet_groups =
                            Some(try!(ClusterSubnetGroupsDeserializer::deserialize(
                                "ClusterSubnetGroups",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ClusterSubnetGroupsDeserializer;
impl ClusterSubnetGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterSubnetGroup>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ClusterSubnetGroup" {
                        obj.push(try!(ClusterSubnetGroupDeserializer::deserialize(
                            "ClusterSubnetGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Describes a cluster version, including the parameter group family and description of the version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterVersion {
    /// <p>The name of the cluster parameter group family for the cluster.</p>
    pub cluster_parameter_group_family: Option<String>,
    /// <p>The version number used by the cluster.</p>
    pub cluster_version: Option<String>,
    /// <p>The description of the cluster version.</p>
    pub description: Option<String>,
}

struct ClusterVersionDeserializer;
impl ClusterVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterVersion, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterVersion::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterParameterGroupFamily" => {
                        obj.cluster_parameter_group_family = Some(try!(
                            StringDeserializer::deserialize("ClusterParameterGroupFamily", stack)
                        ));
                    }
                    "ClusterVersion" => {
                        obj.cluster_version = Some(try!(StringDeserializer::deserialize(
                            "ClusterVersion",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ClusterVersionListDeserializer;
impl ClusterVersionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterVersion>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ClusterVersion" {
                        obj.push(try!(ClusterVersionDeserializer::deserialize(
                            "ClusterVersion",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the output from the <a>DescribeClusterVersions</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClusterVersionsMessage {
    /// <p>A list of <code>Version</code> elements. </p>
    pub cluster_versions: Option<Vec<ClusterVersion>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct ClusterVersionsMessageDeserializer;
impl ClusterVersionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClusterVersionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClusterVersionsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterVersions" => {
                        obj.cluster_versions = Some(try!(
                            ClusterVersionListDeserializer::deserialize("ClusterVersions", stack)
                        ));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the output from the <a>DescribeClusters</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClustersMessage {
    /// <p>A list of <code>Cluster</code> objects, where each object describes one cluster. </p>
    pub clusters: Option<Vec<Cluster>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct ClustersMessageDeserializer;
impl ClustersMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ClustersMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ClustersMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Clusters" => {
                        obj.clusters = Some(try!(ClusterListDeserializer::deserialize(
                            "Clusters", stack
                        )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyClusterSnapshotMessage {
    /// <p><p>The identifier of the cluster the source snapshot was created from. This parameter is required if your IAM user has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.</p> <p>Constraints:</p> <ul> <li> <p>Must be the identifier for a valid cluster.</p> </li> </ul></p>
    pub source_snapshot_cluster_identifier: Option<String>,
    /// <p><p>The identifier for the source snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Must be the identifier for a valid automated snapshot whose state is <code>available</code>.</p> </li> </ul></p>
    pub source_snapshot_identifier: String,
    /// <p><p>The identifier given to the new manual snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank.</p> </li> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> <li> <p>Must be unique for the AWS account that is making the request.</p> </li> </ul></p>
    pub target_snapshot_identifier: String,
}

/// Serialize `CopyClusterSnapshotMessage` contents to a `SignedRequest`.
struct CopyClusterSnapshotMessageSerializer;
impl CopyClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopyClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.source_snapshot_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "SourceSnapshotClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SourceSnapshotIdentifier"),
            &obj.source_snapshot_identifier.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TargetSnapshotIdentifier"),
            &obj.target_snapshot_identifier.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyClusterSnapshotResult {
    pub snapshot: Option<Snapshot>,
}

struct CopyClusterSnapshotResultDeserializer;
impl CopyClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Snapshot" => {
                        obj.snapshot =
                            Some(try!(SnapshotDeserializer::deserialize("Snapshot", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterMessage {
    /// <p>Reserved.</p>
    pub additional_info: Option<String>,
    /// <p>If <code>true</code>, major version upgrades can be applied during the maintenance window to the Amazon Redshift engine that is running on the cluster.</p> <p>When a new major version of the Amazon Redshift engine is released, you can request that the service automatically apply upgrades during the maintenance window to the Amazon Redshift engine that is running on your cluster.</p> <p>Default: <code>true</code> </p>
    pub allow_version_upgrade: Option<bool>,
    /// <p>The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Even if automated snapshots are disabled, you can still create manual snapshots when you want with <a>CreateClusterSnapshot</a>. </p> <p>Default: <code>1</code> </p> <p>Constraints: Must be a value from 0 to 35.</p>
    pub automated_snapshot_retention_period: Option<i64>,
    /// <p>The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the cluster. For example, if you have several EC2 instances running in a specific Availability Zone, then you might want the cluster to be provisioned in the same zone in order to decrease network latency.</p> <p>Default: A random, system-chosen Availability Zone in the region that is specified by the endpoint.</p> <p>Example: <code>us-east-1d</code> </p> <p>Constraint: The specified Availability Zone must be in the same region as the current endpoint.</p>
    pub availability_zone: Option<String>,
    /// <p>A unique identifier for the cluster. You use this identifier to refer to the cluster for any subsequent cluster operations such as deleting or modifying. The identifier also appears in the Amazon Redshift console.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>Alphabetic characters must be lowercase.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> <li> <p>Must be unique for all clusters within an AWS account.</p> </li> </ul> <p>Example: <code>myexamplecluster</code> </p>
    pub cluster_identifier: String,
    /// <p><p>The name of the parameter group to be associated with this cluster.</p> <p>Default: The default Amazon Redshift cluster parameter group. For information about the default parameter group, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Working with Amazon Redshift Parameter Groups</a> </p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub cluster_parameter_group_name: Option<String>,
    /// <p>A list of security groups to be associated with this cluster.</p> <p>Default: The default cluster security group for Amazon Redshift.</p>
    pub cluster_security_groups: Option<Vec<String>>,
    /// <p>The name of a cluster subnet group to be associated with this cluster.</p> <p>If this parameter is not provided the resulting cluster will be deployed outside virtual private cloud (VPC).</p>
    pub cluster_subnet_group_name: Option<String>,
    /// <p>The type of the cluster. When cluster type is specified as</p> <ul> <li> <p> <code>single-node</code>, the <b>NumberOfNodes</b> parameter is not required.</p> </li> <li> <p> <code>multi-node</code>, the <b>NumberOfNodes</b> parameter is required.</p> </li> </ul> <p>Valid Values: <code>multi-node</code> | <code>single-node</code> </p> <p>Default: <code>multi-node</code> </p>
    pub cluster_type: Option<String>,
    /// <p>The version of the Amazon Redshift engine software that you want to deploy on the cluster.</p> <p>The version selected runs on all the nodes in the cluster.</p> <p>Constraints: Only version 1.0 is currently available.</p> <p>Example: <code>1.0</code> </p>
    pub cluster_version: Option<String>,
    /// <p><p>The name of the first database to be created when the cluster is created.</p> <p>To create additional databases after the cluster is created, connect to the cluster with a SQL client and use SQL commands to create a database. For more information, go to <a href="http://docs.aws.amazon.com/redshift/latest/dg/t_creating_database.html">Create a Database</a> in the Amazon Redshift Database Developer Guide. </p> <p>Default: <code>dev</code> </p> <p>Constraints:</p> <ul> <li> <p>Must contain 1 to 64 alphanumeric characters.</p> </li> <li> <p>Must contain only lowercase letters.</p> </li> <li> <p>Cannot be a word that is reserved by the service. A list of reserved words can be found in <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved Words</a> in the Amazon Redshift Database Developer Guide. </p> </li> </ul></p>
    pub db_name: Option<String>,
    /// <p>The Elastic IP (EIP) address for the cluster.</p> <p>Constraints: The cluster must be provisioned in EC2-VPC and publicly-accessible through an Internet gateway. For more information about provisioning clusters in EC2-VPC, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#cluster-platforms">Supported Platforms to Launch Your Cluster</a> in the Amazon Redshift Cluster Management Guide.</p>
    pub elastic_ip: Option<String>,
    /// <p>If <code>true</code>, the data in the cluster is encrypted at rest. </p> <p>Default: false</p>
    pub encrypted: Option<bool>,
    /// <p>An option that specifies whether to create the cluster with enhanced VPC routing enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a VPC. For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/enhanced-vpc-routing.html">Enhanced VPC Routing</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If this option is <code>true</code>, enhanced VPC routing is enabled. </p> <p>Default: false</p>
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>Specifies the name of the HSM client certificate the Amazon Redshift cluster uses to retrieve the data encryption keys stored in an HSM.</p>
    pub hsm_client_certificate_identifier: Option<String>,
    /// <p>Specifies the name of the HSM configuration that contains the information the Amazon Redshift cluster can use to retrieve and store keys in an HSM.</p>
    pub hsm_configuration_identifier: Option<String>,
    /// <p>A list of AWS Identity and Access Management (IAM) roles that can be used by the cluster to access other AWS services. You must supply the IAM roles in their Amazon Resource Name (ARN) format. You can supply up to 10 IAM roles in a single request.</p> <p>A cluster can have up to 10 IAM roles associated with it at any time.</p>
    pub iam_roles: Option<Vec<String>>,
    /// <p>The AWS Key Management Service (KMS) key ID of the encryption key that you want to use to encrypt data in the cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p><p>The password associated with the master user account for the cluster that is being created.</p> <p>Constraints:</p> <ul> <li> <p>Must be between 8 and 64 characters in length.</p> </li> <li> <p>Must contain at least one uppercase letter.</p> </li> <li> <p>Must contain at least one lowercase letter.</p> </li> <li> <p>Must contain one number.</p> </li> <li> <p>Can be any printable ASCII character (ASCII code 33 to 126) except &#39; (single quote), &quot; (double quote), \, /, @, or space.</p> </li> </ul></p>
    pub master_user_password: String,
    /// <p><p>The user name associated with the master user account for the cluster that is being created.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 - 128 alphanumeric characters. The user name can&#39;t be <code>PUBLIC</code>.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot be a reserved word. A list of reserved words can be found in <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved Words</a> in the Amazon Redshift Database Developer Guide. </p> </li> </ul></p>
    pub master_username: String,
    /// <p>The node type to be provisioned for the cluster. For information about node types, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#how-many-nodes"> Working with Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p> <p>Valid Values: <code>ds2.xlarge</code> | <code>ds2.8xlarge</code> | <code>ds2.xlarge</code> | <code>ds2.8xlarge</code> | <code>dc1.large</code> | <code>dc1.8xlarge</code> | <code>dc2.large</code> | <code>dc2.8xlarge</code> </p>
    pub node_type: String,
    /// <p>The number of compute nodes in the cluster. This parameter is required when the <b>ClusterType</b> parameter is specified as <code>multi-node</code>. </p> <p>For information about determining how many nodes you need, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#how-many-nodes"> Working with Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p> <p>If you don't specify this parameter, you get a single-node cluster. When requesting a multi-node cluster, you must specify the number of nodes that you want in the cluster.</p> <p>Default: <code>1</code> </p> <p>Constraints: Value must be at least 1 and no more than 100.</p>
    pub number_of_nodes: Option<i64>,
    /// <p>The port number on which the cluster accepts incoming connections.</p> <p>The cluster is accessible only via the JDBC and ODBC connection strings. Part of the connection string requires the port on which the cluster will listen for incoming connections.</p> <p>Default: <code>5439</code> </p> <p>Valid Values: <code>1150-65535</code> </p>
    pub port: Option<i64>,
    /// <p>The weekly time range (in UTC) during which automated cluster maintenance can occur.</p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p> Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week. For more information about the time blocks for each region, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#rs-maintenance-windows">Maintenance Windows</a> in Amazon Redshift Cluster Management Guide.</p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>If <code>true</code>, the cluster can be accessed from a public network. </p>
    pub publicly_accessible: Option<bool>,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of Virtual Private Cloud (VPC) security groups to be associated with the cluster.</p> <p>Default: The default VPC security group is associated with the cluster.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `CreateClusterMessage` contents to a `SignedRequest`.
struct CreateClusterMessageSerializer;
impl CreateClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.additional_info {
            params.put(
                &format!("{}{}", prefix, "AdditionalInfo"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.allow_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AllowVersionUpgrade"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.automated_snapshot_retention_period {
            params.put(
                &format!("{}{}", prefix, "AutomatedSnapshotRetentionPeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.availability_zone {
            params.put(
                &format!("{}{}", prefix, "AvailabilityZone"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "ClusterParameterGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_security_groups {
            ClusterSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ClusterSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cluster_subnet_group_name {
            params.put(
                &format!("{}{}", prefix, "ClusterSubnetGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_type {
            params.put(
                &format!("{}{}", prefix, "ClusterType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_version {
            params.put(
                &format!("{}{}", prefix, "ClusterVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.db_name {
            params.put(
                &format!("{}{}", prefix, "DBName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.elastic_ip {
            params.put(
                &format!("{}{}", prefix, "ElasticIp"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.encrypted {
            params.put(
                &format!("{}{}", prefix, "Encrypted"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.enhanced_vpc_routing {
            params.put(
                &format!("{}{}", prefix, "EnhancedVpcRouting"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.hsm_client_certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmClientCertificateIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.hsm_configuration_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmConfigurationIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.iam_roles {
            IamRoleArnListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "IamRoleArn"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(
                &format!("{}{}", prefix, "KmsKeyId"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "MasterUserPassword"),
            &obj.master_user_password.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "MasterUsername"),
            &obj.master_username.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "NodeType"),
            &obj.node_type.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.number_of_nodes {
            params.put(
                &format!("{}{}", prefix, "NumberOfNodes"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.port {
            params.put(
                &format!("{}{}", prefix, "Port"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.publicly_accessible {
            params.put(
                &format!("{}{}", prefix, "PubliclyAccessible"),
                &field_value.to_string().replace("+", "%2B"),
            );
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

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterParameterGroupMessage {
    /// <p>A description of the parameter group.</p>
    pub description: String,
    /// <p>The Amazon Redshift engine version to which the cluster parameter group applies. The cluster engine version determines the set of parameters.</p> <p>To get a list of valid parameter group family names, you can call <a>DescribeClusterParameterGroups</a>. By default, Amazon Redshift returns a list of all the parameter groups that are owned by your AWS account, including the default parameter groups for each Amazon Redshift engine version. The parameter group family names associated with the default parameter groups provide you the valid values. For example, a valid family name is "redshift-1.0". </p>
    pub parameter_group_family: String,
    /// <p><p>The name of the cluster parameter group.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters or hyphens</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> <li> <p>Must be unique withing your AWS account.</p> </li> </ul> <note> <p>This value is stored as a lower-case string.</p> </note></p>
    pub parameter_group_name: String,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateClusterParameterGroupMessage` contents to a `SignedRequest`.
struct CreateClusterParameterGroupMessageSerializer;
impl CreateClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Description"),
            &obj.description.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ParameterGroupFamily"),
            &obj.parameter_group_family.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ParameterGroupName"),
            &obj.parameter_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterParameterGroupResult {
    pub cluster_parameter_group: Option<ClusterParameterGroup>,
}

struct CreateClusterParameterGroupResultDeserializer;
impl CreateClusterParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateClusterParameterGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateClusterParameterGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterParameterGroup" => {
                        obj.cluster_parameter_group =
                            Some(try!(ClusterParameterGroupDeserializer::deserialize(
                                "ClusterParameterGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterResult {
    pub cluster: Option<Cluster>,
}

struct CreateClusterResultDeserializer;
impl CreateClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterSecurityGroupMessage {
    /// <p>The name for the security group. Amazon Redshift stores the value as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain no more than 255 alphanumeric characters or hyphens.</p> </li> <li> <p>Must not be "Default".</p> </li> <li> <p>Must be unique for all security groups that are created by your AWS account.</p> </li> </ul> <p>Example: <code>examplesecuritygroup</code> </p>
    pub cluster_security_group_name: String,
    /// <p>A description for the security group.</p>
    pub description: String,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateClusterSecurityGroupMessage` contents to a `SignedRequest`.
struct CreateClusterSecurityGroupMessageSerializer;
impl CreateClusterSecurityGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateClusterSecurityGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterSecurityGroupName"),
            &obj.cluster_security_group_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Description"),
            &obj.description.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterSecurityGroupResult {
    pub cluster_security_group: Option<ClusterSecurityGroup>,
}

struct CreateClusterSecurityGroupResultDeserializer;
impl CreateClusterSecurityGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateClusterSecurityGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateClusterSecurityGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSecurityGroup" => {
                        obj.cluster_security_group =
                            Some(try!(ClusterSecurityGroupDeserializer::deserialize(
                                "ClusterSecurityGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterSnapshotMessage {
    /// <p>The cluster identifier for which you want a snapshot.</p>
    pub cluster_identifier: String,
    /// <p>A unique identifier for the snapshot that you are requesting. This identifier must be unique for all snapshots within the AWS account.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank</p> </li> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul> <p>Example: <code>my-snapshot-id</code> </p>
    pub snapshot_identifier: String,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateClusterSnapshotMessage` contents to a `SignedRequest`.
struct CreateClusterSnapshotMessageSerializer;
impl CreateClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterSnapshotResult {
    pub snapshot: Option<Snapshot>,
}

struct CreateClusterSnapshotResultDeserializer;
impl CreateClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Snapshot" => {
                        obj.snapshot =
                            Some(try!(SnapshotDeserializer::deserialize("Snapshot", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateClusterSubnetGroupMessage {
    /// <p>The name for the subnet group. Amazon Redshift stores the value as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain no more than 255 alphanumeric characters or hyphens.</p> </li> <li> <p>Must not be "Default".</p> </li> <li> <p>Must be unique for all subnet groups that are created by your AWS account.</p> </li> </ul> <p>Example: <code>examplesubnetgroup</code> </p>
    pub cluster_subnet_group_name: String,
    /// <p>A description for the subnet group.</p>
    pub description: String,
    /// <p>An array of VPC subnet IDs. A maximum of 20 subnets can be modified in a single request.</p>
    pub subnet_ids: Vec<String>,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateClusterSubnetGroupMessage` contents to a `SignedRequest`.
struct CreateClusterSubnetGroupMessageSerializer;
impl CreateClusterSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateClusterSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterSubnetGroupName"),
            &obj.cluster_subnet_group_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Description"),
            &obj.description.replace("+", "%2B"),
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
pub struct CreateClusterSubnetGroupResult {
    pub cluster_subnet_group: Option<ClusterSubnetGroup>,
}

struct CreateClusterSubnetGroupResultDeserializer;
impl CreateClusterSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateClusterSubnetGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateClusterSubnetGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSubnetGroup" => {
                        obj.cluster_subnet_group =
                            Some(try!(ClusterSubnetGroupDeserializer::deserialize(
                                "ClusterSubnetGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateEventSubscriptionMessage {
    /// <p>A Boolean value; set to <code>true</code> to activate the subscription, set to <code>false</code> to create the subscription but not active it. </p>
    pub enabled: Option<bool>,
    /// <p>Specifies the Amazon Redshift event categories to be published by the event notification subscription.</p> <p>Values: Configuration, Management, Monitoring, Security</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>Specifies the Amazon Redshift event severity to be published by the event notification subscription.</p> <p>Values: ERROR, INFO</p>
    pub severity: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic used to transmit the event notifications. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>
    pub sns_topic_arn: String,
    /// <p>A list of one or more identifiers of Amazon Redshift source objects. All of the objects must be of the same type as was specified in the source type parameter. The event subscription will return only events generated by the specified objects. If not specified, then events are returned for all objects within the source type specified.</p> <p>Example: my-cluster-1, my-cluster-2</p> <p>Example: my-snapshot-20131010</p>
    pub source_ids: Option<Vec<String>>,
    /// <p>The type of source that will be generating the events. For example, if you want to be notified of events generated by a cluster, you would set this parameter to cluster. If this value is not specified, events are returned for all Amazon Redshift objects in your AWS account. You must specify a source type in order to specify source IDs.</p> <p>Valid values: cluster, cluster-parameter-group, cluster-security-group, and cluster-snapshot.</p>
    pub source_type: Option<String>,
    /// <p><p>The name of the event subscription to be created.</p> <p>Constraints:</p> <ul> <li> <p>Cannot be null, empty, or blank.</p> </li> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub subscription_name: String,
    /// <p>A list of tag instances.</p>
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
            params.put(
                &format!("{}{}", prefix, "Enabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.severity {
            params.put(
                &format!("{}{}", prefix, "Severity"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SnsTopicArn"),
            &obj.sns_topic_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.source_ids {
            SourceIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(
                &format!("{}{}", prefix, "SourceType"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct CreateEventSubscriptionResultDeserializer;
impl CreateEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateEventSubscriptionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateEventSubscriptionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscription" => {
                        obj.event_subscription = Some(try!(
                            EventSubscriptionDeserializer::deserialize("EventSubscription", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHsmClientCertificateMessage {
    /// <p>The identifier to be assigned to the new HSM client certificate that the cluster will use to connect to the HSM to use the database encryption keys.</p>
    pub hsm_client_certificate_identifier: String,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateHsmClientCertificateMessage` contents to a `SignedRequest`.
struct CreateHsmClientCertificateMessageSerializer;
impl CreateHsmClientCertificateMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateHsmClientCertificateMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "HsmClientCertificateIdentifier"),
            &obj.hsm_client_certificate_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHsmClientCertificateResult {
    pub hsm_client_certificate: Option<HsmClientCertificate>,
}

struct CreateHsmClientCertificateResultDeserializer;
impl CreateHsmClientCertificateResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateHsmClientCertificateResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateHsmClientCertificateResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HsmClientCertificate" => {
                        obj.hsm_client_certificate =
                            Some(try!(HsmClientCertificateDeserializer::deserialize(
                                "HsmClientCertificate",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHsmConfigurationMessage {
    /// <p>A text description of the HSM configuration to be created.</p>
    pub description: String,
    /// <p>The identifier to be assigned to the new Amazon Redshift HSM configuration.</p>
    pub hsm_configuration_identifier: String,
    /// <p>The IP address that the Amazon Redshift cluster must use to access the HSM.</p>
    pub hsm_ip_address: String,
    /// <p>The name of the partition in the HSM where the Amazon Redshift clusters will store their database encryption keys.</p>
    pub hsm_partition_name: String,
    /// <p>The password required to access the HSM partition.</p>
    pub hsm_partition_password: String,
    /// <p>The HSMs public certificate file. When using Cloud HSM, the file name is server.pem.</p>
    pub hsm_server_public_certificate: String,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateHsmConfigurationMessage` contents to a `SignedRequest`.
struct CreateHsmConfigurationMessageSerializer;
impl CreateHsmConfigurationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateHsmConfigurationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Description"),
            &obj.description.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "HsmConfigurationIdentifier"),
            &obj.hsm_configuration_identifier.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "HsmIpAddress"),
            &obj.hsm_ip_address.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "HsmPartitionName"),
            &obj.hsm_partition_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "HsmPartitionPassword"),
            &obj.hsm_partition_password.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "HsmServerPublicCertificate"),
            &obj.hsm_server_public_certificate.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHsmConfigurationResult {
    pub hsm_configuration: Option<HsmConfiguration>,
}

struct CreateHsmConfigurationResultDeserializer;
impl CreateHsmConfigurationResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateHsmConfigurationResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateHsmConfigurationResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HsmConfiguration" => {
                        obj.hsm_configuration = Some(try!(
                            HsmConfigurationDeserializer::deserialize("HsmConfiguration", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The result of the <code>CreateSnapshotCopyGrant</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateSnapshotCopyGrantMessage {
    /// <p>The unique identifier of the customer master key (CMK) to which to grant Amazon Redshift permission. If no key is specified, the default key is used.</p>
    pub kms_key_id: Option<String>,
    /// <p><p>The name of the snapshot copy grant. This name must be unique in the region for the AWS account.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>Alphabetic characters must be lowercase.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> <li> <p>Must be unique for all clusters within an AWS account.</p> </li> </ul></p>
    pub snapshot_copy_grant_name: String,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateSnapshotCopyGrantMessage` contents to a `SignedRequest`.
struct CreateSnapshotCopyGrantMessageSerializer;
impl CreateSnapshotCopyGrantMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateSnapshotCopyGrantMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.kms_key_id {
            params.put(
                &format!("{}{}", prefix, "KmsKeyId"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SnapshotCopyGrantName"),
            &obj.snapshot_copy_grant_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateSnapshotCopyGrantResult {
    pub snapshot_copy_grant: Option<SnapshotCopyGrant>,
}

struct CreateSnapshotCopyGrantResultDeserializer;
impl CreateSnapshotCopyGrantResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateSnapshotCopyGrantResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateSnapshotCopyGrantResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "SnapshotCopyGrant" => {
                        obj.snapshot_copy_grant = Some(try!(
                            SnapshotCopyGrantDeserializer::deserialize("SnapshotCopyGrant", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the output from the <code>CreateTags</code> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTagsMessage {
    /// <p>The Amazon Resource Name (ARN) to which you want to add the tag or tags. For example, <code>arn:aws:redshift:us-east-1:123456789:cluster:t1</code>. </p>
    pub resource_name: String,
    /// <p>One or more name/value pairs to add as tags to the specified resource. Each tag name is passed in with the parameter <code>Key</code> and the corresponding value is passed in with the parameter <code>Value</code>. The <code>Key</code> and <code>Value</code> parameters are separated by a comma (,). Separate multiple tags with a space. For example, <code>--tags "Key"="owner","Value"="admin" "Key"="environment","Value"="test" "Key"="version","Value"="1.0"</code>. </p>
    pub tags: Vec<Tag>,
}

/// Serialize `CreateTagsMessage` contents to a `SignedRequest`.
struct CreateTagsMessageSerializer;
impl CreateTagsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateTagsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ResourceName"),
            &obj.resource_name.replace("+", "%2B"),
        );
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), &obj.tags);
    }
}

/// Serialize `DbGroupList` contents to a `SignedRequest`.
struct DbGroupListSerializer;
impl DbGroupListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Describes the default cluster parameters for a parameter group family.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultClusterParameters {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>The name of the cluster parameter group family to which the engine default parameters apply.</p>
    pub parameter_group_family: Option<String>,
    /// <p>The list of cluster default parameters.</p>
    pub parameters: Option<Vec<Parameter>>,
}

struct DefaultClusterParametersDeserializer;
impl DefaultClusterParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefaultClusterParameters, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DefaultClusterParameters::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "ParameterGroupFamily" => {
                        obj.parameter_group_family = Some(try!(StringDeserializer::deserialize(
                            "ParameterGroupFamily",
                            stack
                        )));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersListDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteClusterMessage {
    /// <p><p>The identifier of the cluster to be deleted.</p> <p>Constraints:</p> <ul> <li> <p>Must contain lowercase characters.</p> </li> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub cluster_identifier: String,
    /// <p><p>The identifier of the final snapshot that is to be created immediately before deleting the cluster. If this parameter is provided, <i>SkipFinalClusterSnapshot</i> must be <code>false</code>. </p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub final_cluster_snapshot_identifier: Option<String>,
    /// <p>Determines whether a final snapshot of the cluster is created before Amazon Redshift deletes the cluster. If <code>true</code>, a final cluster snapshot is not created. If <code>false</code>, a final cluster snapshot is created before the cluster is deleted. </p> <note> <p>The <i>FinalClusterSnapshotIdentifier</i> parameter must be specified if <i>SkipFinalClusterSnapshot</i> is <code>false</code>.</p> </note> <p>Default: <code>false</code> </p>
    pub skip_final_cluster_snapshot: Option<bool>,
}

/// Serialize `DeleteClusterMessage` contents to a `SignedRequest`.
struct DeleteClusterMessageSerializer;
impl DeleteClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.final_cluster_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "FinalClusterSnapshotIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.skip_final_cluster_snapshot {
            params.put(
                &format!("{}{}", prefix, "SkipFinalClusterSnapshot"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteClusterParameterGroupMessage {
    /// <p><p>The name of the parameter group to be deleted.</p> <p>Constraints:</p> <ul> <li> <p>Must be the name of an existing cluster parameter group.</p> </li> <li> <p>Cannot delete a default cluster parameter group.</p> </li> </ul></p>
    pub parameter_group_name: String,
}

/// Serialize `DeleteClusterParameterGroupMessage` contents to a `SignedRequest`.
struct DeleteClusterParameterGroupMessageSerializer;
impl DeleteClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ParameterGroupName"),
            &obj.parameter_group_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteClusterResult {
    pub cluster: Option<Cluster>,
}

struct DeleteClusterResultDeserializer;
impl DeleteClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteClusterSecurityGroupMessage {
    /// <p>The name of the cluster security group to be deleted.</p>
    pub cluster_security_group_name: String,
}

/// Serialize `DeleteClusterSecurityGroupMessage` contents to a `SignedRequest`.
struct DeleteClusterSecurityGroupMessageSerializer;
impl DeleteClusterSecurityGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteClusterSecurityGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterSecurityGroupName"),
            &obj.cluster_security_group_name.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteClusterSnapshotMessage {
    /// <p>The unique identifier of the cluster the snapshot was created from. This parameter is required if your IAM user has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.</p> <p>Constraints: Must be the name of valid cluster.</p>
    pub snapshot_cluster_identifier: Option<String>,
    /// <p>The unique identifier of the manual snapshot to be deleted.</p> <p>Constraints: Must be the name of an existing snapshot that is in the <code>available</code> state.</p>
    pub snapshot_identifier: String,
}

/// Serialize `DeleteClusterSnapshotMessage` contents to a `SignedRequest`.
struct DeleteClusterSnapshotMessageSerializer;
impl DeleteClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.snapshot_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "SnapshotClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteClusterSnapshotResult {
    pub snapshot: Option<Snapshot>,
}

struct DeleteClusterSnapshotResultDeserializer;
impl DeleteClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Snapshot" => {
                        obj.snapshot =
                            Some(try!(SnapshotDeserializer::deserialize("Snapshot", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteClusterSubnetGroupMessage {
    /// <p>The name of the cluster subnet group name to be deleted.</p>
    pub cluster_subnet_group_name: String,
}

/// Serialize `DeleteClusterSubnetGroupMessage` contents to a `SignedRequest`.
struct DeleteClusterSubnetGroupMessageSerializer;
impl DeleteClusterSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteClusterSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterSubnetGroupName"),
            &obj.cluster_subnet_group_name.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteEventSubscriptionMessage {
    /// <p>The name of the Amazon Redshift event notification subscription to be deleted.</p>
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
            &obj.subscription_name.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteHsmClientCertificateMessage {
    /// <p>The identifier of the HSM client certificate to be deleted.</p>
    pub hsm_client_certificate_identifier: String,
}

/// Serialize `DeleteHsmClientCertificateMessage` contents to a `SignedRequest`.
struct DeleteHsmClientCertificateMessageSerializer;
impl DeleteHsmClientCertificateMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteHsmClientCertificateMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "HsmClientCertificateIdentifier"),
            &obj.hsm_client_certificate_identifier.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteHsmConfigurationMessage {
    /// <p>The identifier of the Amazon Redshift HSM configuration to be deleted.</p>
    pub hsm_configuration_identifier: String,
}

/// Serialize `DeleteHsmConfigurationMessage` contents to a `SignedRequest`.
struct DeleteHsmConfigurationMessageSerializer;
impl DeleteHsmConfigurationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteHsmConfigurationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "HsmConfigurationIdentifier"),
            &obj.hsm_configuration_identifier.replace("+", "%2B"),
        );
    }
}

/// <p>The result of the <code>DeleteSnapshotCopyGrant</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteSnapshotCopyGrantMessage {
    /// <p>The name of the snapshot copy grant to delete.</p>
    pub snapshot_copy_grant_name: String,
}

/// Serialize `DeleteSnapshotCopyGrantMessage` contents to a `SignedRequest`.
struct DeleteSnapshotCopyGrantMessageSerializer;
impl DeleteSnapshotCopyGrantMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteSnapshotCopyGrantMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SnapshotCopyGrantName"),
            &obj.snapshot_copy_grant_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output from the <code>DeleteTags</code> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTagsMessage {
    /// <p>The Amazon Resource Name (ARN) from which you want to remove the tag or tags. For example, <code>arn:aws:redshift:us-east-1:123456789:cluster:t1</code>. </p>
    pub resource_name: String,
    /// <p>The tag key that you want to delete.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `DeleteTagsMessage` contents to a `SignedRequest`.
struct DeleteTagsMessageSerializer;
impl DeleteTagsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteTagsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ResourceName"),
            &obj.resource_name.replace("+", "%2B"),
        );
        TagKeyListSerializer::serialize(params, &format!("{}{}", prefix, "TagKey"), &obj.tag_keys);
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeClusterParameterGroupsMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeClusterParameterGroups</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The name of a specific parameter group for which to return details. By default, details about all parameter groups and the default parameter group are returned.</p>
    pub parameter_group_name: Option<String>,
    /// <p>A tag key or keys for which you want to return all matching cluster parameter groups that are associated with the specified key or keys. For example, suppose that you have parameter groups that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the parameter groups that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching cluster parameter groups that are associated with the specified tag value or values. For example, suppose that you have parameter groups that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the parameter groups that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeClusterParameterGroupsMessage` contents to a `SignedRequest`.
struct DescribeClusterParameterGroupsMessageSerializer;
impl DescribeClusterParameterGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeClusterParameterGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "ParameterGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeClusterParametersMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeClusterParameters</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The name of a cluster parameter group for which to return details.</p>
    pub parameter_group_name: String,
    /// <p>The parameter types to return. Specify <code>user</code> to show parameters that are different form the default. Similarly, specify <code>engine-default</code> to show parameters that are the same as the default parameter group. </p> <p>Default: All parameter types returned.</p> <p>Valid Values: <code>user</code> | <code>engine-default</code> </p>
    pub source: Option<String>,
}

/// Serialize `DescribeClusterParametersMessage` contents to a `SignedRequest`.
struct DescribeClusterParametersMessageSerializer;
impl DescribeClusterParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeClusterParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ParameterGroupName"),
            &obj.parameter_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.source {
            params.put(
                &format!("{}{}", prefix, "Source"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeClusterSecurityGroupsMessage {
    /// <p>The name of a cluster security group for which you are requesting details. You can specify either the <b>Marker</b> parameter or a <b>ClusterSecurityGroupName</b> parameter, but not both. </p> <p> Example: <code>securitygroup1</code> </p>
    pub cluster_security_group_name: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeClusterSecurityGroups</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p> <p>Constraints: You can specify either the <b>ClusterSecurityGroupName</b> parameter or the <b>Marker</b> parameter, but not both. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>A tag key or keys for which you want to return all matching cluster security groups that are associated with the specified key or keys. For example, suppose that you have security groups that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the security groups that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching cluster security groups that are associated with the specified tag value or values. For example, suppose that you have security groups that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the security groups that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeClusterSecurityGroupsMessage` contents to a `SignedRequest`.
struct DescribeClusterSecurityGroupsMessageSerializer;
impl DescribeClusterSecurityGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeClusterSecurityGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cluster_security_group_name {
            params.put(
                &format!("{}{}", prefix, "ClusterSecurityGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeClusterSnapshotsMessage {
    /// <p>A value that indicates whether to return snapshots only for an existing cluster. Table-level restore can be performed only using a snapshot of an existing cluster, that is, a cluster that has not been deleted. If <code>ClusterExists</code> is set to <code>true</code>, <code>ClusterIdentifier</code> is required.</p>
    pub cluster_exists: Option<bool>,
    /// <p>The identifier of the cluster for which information about snapshots is requested.</p>
    pub cluster_identifier: Option<String>,
    /// <p>A time value that requests only snapshots created at or before the specified time. The time value is specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: <code>2012-07-16T18:00:00Z</code> </p>
    pub end_time: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeClusterSnapshots</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The AWS customer account used to create or copy the snapshot. Use this field to filter the results to snapshots owned by a particular account. To describe snapshots you own, either specify your AWS customer account, or do not specify the parameter.</p>
    pub owner_account: Option<String>,
    /// <p>The snapshot identifier of the snapshot about which to return information.</p>
    pub snapshot_identifier: Option<String>,
    /// <p>The type of snapshots for which you are requesting information. By default, snapshots of all types are returned.</p> <p>Valid Values: <code>automated</code> | <code>manual</code> </p>
    pub snapshot_type: Option<String>,
    /// <p>A value that requests only snapshots created at or after the specified time. The time value is specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: <code>2012-07-16T18:00:00Z</code> </p>
    pub start_time: Option<String>,
    /// <p>A tag key or keys for which you want to return all matching cluster snapshots that are associated with the specified key or keys. For example, suppose that you have snapshots that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the snapshots that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching cluster snapshots that are associated with the specified tag value or values. For example, suppose that you have snapshots that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the snapshots that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeClusterSnapshotsMessage` contents to a `SignedRequest`.
struct DescribeClusterSnapshotsMessageSerializer;
impl DescribeClusterSnapshotsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeClusterSnapshotsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cluster_exists {
            params.put(
                &format!("{}{}", prefix, "ClusterExists"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "ClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(
                &format!("{}{}", prefix, "EndTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.owner_account {
            params.put(
                &format!("{}{}", prefix, "OwnerAccount"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "SnapshotIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.snapshot_type {
            params.put(
                &format!("{}{}", prefix, "SnapshotType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(
                &format!("{}{}", prefix, "StartTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeClusterSubnetGroupsMessage {
    /// <p>The name of the cluster subnet group for which information is requested.</p>
    pub cluster_subnet_group_name: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeClusterSubnetGroups</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>A tag key or keys for which you want to return all matching cluster subnet groups that are associated with the specified key or keys. For example, suppose that you have subnet groups that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the subnet groups that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching cluster subnet groups that are associated with the specified tag value or values. For example, suppose that you have subnet groups that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the subnet groups that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeClusterSubnetGroupsMessage` contents to a `SignedRequest`.
struct DescribeClusterSubnetGroupsMessageSerializer;
impl DescribeClusterSubnetGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeClusterSubnetGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cluster_subnet_group_name {
            params.put(
                &format!("{}{}", prefix, "ClusterSubnetGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeClusterVersionsMessage {
    /// <p><p>The name of a specific cluster parameter group family to return details for.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub cluster_parameter_group_family: Option<String>,
    /// <p>The specific cluster version to return.</p> <p>Example: <code>1.0</code> </p>
    pub cluster_version: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeClusterVersions</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
}

/// Serialize `DescribeClusterVersionsMessage` contents to a `SignedRequest`.
struct DescribeClusterVersionsMessageSerializer;
impl DescribeClusterVersionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeClusterVersionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cluster_parameter_group_family {
            params.put(
                &format!("{}{}", prefix, "ClusterParameterGroupFamily"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_version {
            params.put(
                &format!("{}{}", prefix, "ClusterVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeClustersMessage {
    /// <p>The unique identifier of a cluster whose properties you are requesting. This parameter is case sensitive.</p> <p>The default is that all clusters defined for an account are returned.</p>
    pub cluster_identifier: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeClusters</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p> <p>Constraints: You can specify either the <b>ClusterIdentifier</b> parameter or the <b>Marker</b> parameter, but not both. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>A tag key or keys for which you want to return all matching clusters that are associated with the specified key or keys. For example, suppose that you have clusters that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the clusters that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching clusters that are associated with the specified tag value or values. For example, suppose that you have clusters that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the clusters that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeClustersMessage` contents to a `SignedRequest`.
struct DescribeClustersMessageSerializer;
impl DescribeClustersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeClustersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "ClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDefaultClusterParametersMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeDefaultClusterParameters</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The name of the cluster parameter group family.</p>
    pub parameter_group_family: String,
}

/// Serialize `DescribeDefaultClusterParametersMessage` contents to a `SignedRequest`.
struct DescribeDefaultClusterParametersMessageSerializer;
impl DescribeDefaultClusterParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeDefaultClusterParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ParameterGroupFamily"),
            &obj.parameter_group_family.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeDefaultClusterParametersResult {
    pub default_cluster_parameters: Option<DefaultClusterParameters>,
}

struct DescribeDefaultClusterParametersResultDeserializer;
impl DescribeDefaultClusterParametersResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeDefaultClusterParametersResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeDefaultClusterParametersResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultClusterParameters" => {
                        obj.default_cluster_parameters =
                            Some(try!(DefaultClusterParametersDeserializer::deserialize(
                                "DefaultClusterParameters",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEventCategoriesMessage {
    /// <p>The source type, such as cluster or parameter group, to which the described event categories apply.</p> <p>Valid values: cluster, cluster-snapshot, cluster-parameter-group, and cluster-security-group.</p>
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

        if let Some(ref field_value) = obj.source_type {
            params.put(
                &format!("{}{}", prefix, "SourceType"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEventSubscriptionsMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a DescribeEventSubscriptions request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The name of the Amazon Redshift event notification subscription to be described.</p>
    pub subscription_name: Option<String>,
    /// <p>A tag key or keys for which you want to return all matching event notification subscriptions that are associated with the specified key or keys. For example, suppose that you have subscriptions that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the subscriptions that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching event notification subscriptions that are associated with the specified tag value or values. For example, suppose that you have subscriptions that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the subscriptions that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeEventSubscriptionsMessage` contents to a `SignedRequest`.
struct DescribeEventSubscriptionsMessageSerializer;
impl DescribeEventSubscriptionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventSubscriptionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.subscription_name {
            params.put(
                &format!("{}{}", prefix, "SubscriptionName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEventsMessage {
    /// <p>The number of minutes prior to the time of the request for which to retrieve events. For example, if the request is sent at 18:00 and you specify a duration of 60, then only events which have occurred after 17:00 will be returned.</p> <p>Default: <code>60</code> </p>
    pub duration: Option<i64>,
    /// <p>The end of the time interval for which to retrieve events, specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: <code>2009-07-08T18:00Z</code> </p>
    pub end_time: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeEvents</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p><p>The identifier of the event source for which events will be returned. If this parameter is not specified, then all sources are included in the response.</p> <p>Constraints:</p> <p>If <i>SourceIdentifier</i> is supplied, <i>SourceType</i> must also be provided.</p> <ul> <li> <p>Specify a cluster identifier when <i>SourceType</i> is <code>cluster</code>.</p> </li> <li> <p>Specify a cluster security group name when <i>SourceType</i> is <code>cluster-security-group</code>.</p> </li> <li> <p>Specify a cluster parameter group name when <i>SourceType</i> is <code>cluster-parameter-group</code>.</p> </li> <li> <p>Specify a cluster snapshot identifier when <i>SourceType</i> is <code>cluster-snapshot</code>.</p> </li> </ul></p>
    pub source_identifier: Option<String>,
    /// <p><p>The event source to retrieve events for. If no value is specified, all events are returned.</p> <p>Constraints:</p> <p>If <i>SourceType</i> is supplied, <i>SourceIdentifier</i> must also be provided.</p> <ul> <li> <p>Specify <code>cluster</code> when <i>SourceIdentifier</i> is a cluster identifier.</p> </li> <li> <p>Specify <code>cluster-security-group</code> when <i>SourceIdentifier</i> is a cluster security group name.</p> </li> <li> <p>Specify <code>cluster-parameter-group</code> when <i>SourceIdentifier</i> is a cluster parameter group name.</p> </li> <li> <p>Specify <code>cluster-snapshot</code> when <i>SourceIdentifier</i> is a cluster snapshot identifier.</p> </li> </ul></p>
    pub source_type: Option<String>,
    /// <p>The beginning of the time interval to retrieve events for, specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <p>Example: <code>2009-07-08T18:00Z</code> </p>
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
            params.put(
                &format!("{}{}", prefix, "Duration"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(
                &format!("{}{}", prefix, "EndTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_identifier {
            params.put(
                &format!("{}{}", prefix, "SourceIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(
                &format!("{}{}", prefix, "SourceType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(
                &format!("{}{}", prefix, "StartTime"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeHsmClientCertificatesMessage {
    /// <p>The identifier of a specific HSM client certificate for which you want information. If no identifier is specified, information is returned for all HSM client certificates owned by your AWS customer account.</p>
    pub hsm_client_certificate_identifier: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeHsmClientCertificates</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>A tag key or keys for which you want to return all matching HSM client certificates that are associated with the specified key or keys. For example, suppose that you have HSM client certificates that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the HSM client certificates that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching HSM client certificates that are associated with the specified tag value or values. For example, suppose that you have HSM client certificates that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the HSM client certificates that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeHsmClientCertificatesMessage` contents to a `SignedRequest`.
struct DescribeHsmClientCertificatesMessageSerializer;
impl DescribeHsmClientCertificatesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeHsmClientCertificatesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.hsm_client_certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmClientCertificateIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeHsmConfigurationsMessage {
    /// <p>The identifier of a specific Amazon Redshift HSM configuration to be described. If no identifier is specified, information is returned for all HSM configurations owned by your AWS customer account.</p>
    pub hsm_configuration_identifier: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeHsmConfigurations</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>A tag key or keys for which you want to return all matching HSM configurations that are associated with the specified key or keys. For example, suppose that you have HSM configurations that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the HSM configurations that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching HSM configurations that are associated with the specified tag value or values. For example, suppose that you have HSM configurations that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the HSM configurations that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeHsmConfigurationsMessage` contents to a `SignedRequest`.
struct DescribeHsmConfigurationsMessageSerializer;
impl DescribeHsmConfigurationsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeHsmConfigurationsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.hsm_configuration_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmConfigurationIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoggingStatusMessage {
    /// <p>The identifier of the cluster from which to get the logging status.</p> <p>Example: <code>examplecluster</code> </p>
    pub cluster_identifier: String,
}

/// Serialize `DescribeLoggingStatusMessage` contents to a `SignedRequest`.
struct DescribeLoggingStatusMessageSerializer;
impl DescribeLoggingStatusMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoggingStatusMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeOrderableClusterOptionsMessage {
    /// <p>The version filter value. Specify this parameter to show only the available offerings matching the specified version.</p> <p>Default: All versions.</p> <p>Constraints: Must be one of the version returned from <a>DescribeClusterVersions</a>.</p>
    pub cluster_version: Option<String>,
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeOrderableClusterOptions</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The node type filter value. Specify this parameter to show only the available offerings matching the specified node type.</p>
    pub node_type: Option<String>,
}

/// Serialize `DescribeOrderableClusterOptionsMessage` contents to a `SignedRequest`.
struct DescribeOrderableClusterOptionsMessageSerializer;
impl DescribeOrderableClusterOptionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeOrderableClusterOptionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cluster_version {
            params.put(
                &format!("{}{}", prefix, "ClusterVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.node_type {
            params.put(
                &format!("{}{}", prefix, "NodeType"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeReservedNodeOfferingsMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeReservedNodeOfferings</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The unique identifier for the offering.</p>
    pub reserved_node_offering_id: Option<String>,
}

/// Serialize `DescribeReservedNodeOfferingsMessage` contents to a `SignedRequest`.
struct DescribeReservedNodeOfferingsMessageSerializer;
impl DescribeReservedNodeOfferingsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeReservedNodeOfferingsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.reserved_node_offering_id {
            params.put(
                &format!("{}{}", prefix, "ReservedNodeOfferingId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeReservedNodesMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <a>DescribeReservedNodes</a> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>Identifier for the node reservation.</p>
    pub reserved_node_id: Option<String>,
}

/// Serialize `DescribeReservedNodesMessage` contents to a `SignedRequest`.
struct DescribeReservedNodesMessageSerializer;
impl DescribeReservedNodesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeReservedNodesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.reserved_node_id {
            params.put(
                &format!("{}{}", prefix, "ReservedNodeId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeResizeMessage {
    /// <p>The unique identifier of a cluster whose resize progress you are requesting. This parameter is case-sensitive.</p> <p>By default, resize operations for all clusters defined for an AWS account are returned.</p>
    pub cluster_identifier: String,
}

/// Serialize `DescribeResizeMessage` contents to a `SignedRequest`.
struct DescribeResizeMessageSerializer;
impl DescribeResizeMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeResizeMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
    }
}

/// <p>The result of the <code>DescribeSnapshotCopyGrants</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeSnapshotCopyGrantsMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeSnapshotCopyGrant</code> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p> <p>Constraints: You can specify either the <b>SnapshotCopyGrantName</b> parameter or the <b>Marker</b> parameter, but not both. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p> <p>Default: <code>100</code> </p> <p>Constraints: minimum 20, maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The name of the snapshot copy grant.</p>
    pub snapshot_copy_grant_name: Option<String>,
    /// <p>A tag key or keys for which you want to return all matching resources that are associated with the specified key or keys. For example, suppose that you have resources tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with all resources that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching resources that are associated with the specified value or values. For example, suppose that you have resources tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with all resources that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeSnapshotCopyGrantsMessage` contents to a `SignedRequest`.
struct DescribeSnapshotCopyGrantsMessageSerializer;
impl DescribeSnapshotCopyGrantsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeSnapshotCopyGrantsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.snapshot_copy_grant_name {
            params.put(
                &format!("{}{}", prefix, "SnapshotCopyGrantName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTableRestoreStatusMessage {
    /// <p>The Amazon Redshift cluster that the table is being restored to.</p>
    pub cluster_identifier: Option<String>,
    /// <p>An optional pagination token provided by a previous <code>DescribeTableRestoreStatus</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by the <code>MaxRecords</code> parameter.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    pub max_records: Option<i64>,
    /// <p>The identifier of the table restore request to return status for. If you don't specify a <code>TableRestoreRequestId</code> value, then <code>DescribeTableRestoreStatus</code> returns the status of all in-progress table restore requests.</p>
    pub table_restore_request_id: Option<String>,
}

/// Serialize `DescribeTableRestoreStatusMessage` contents to a `SignedRequest`.
struct DescribeTableRestoreStatusMessageSerializer;
impl DescribeTableRestoreStatusMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTableRestoreStatusMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "ClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.table_restore_request_id {
            params.put(
                &format!("{}{}", prefix, "TableRestoreRequestId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTagsMessage {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>marker</code> parameter and retrying the command. If the <code>marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>The maximum number or response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned <code>marker</code> value. </p>
    pub max_records: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) for which you want to describe the tag or tags. For example, <code>arn:aws:redshift:us-east-1:123456789:cluster:t1</code>. </p>
    pub resource_name: Option<String>,
    /// <p>The type of resource with which you want to view tags. Valid resource types are: </p> <ul> <li> <p>Cluster</p> </li> <li> <p>CIDR/IP</p> </li> <li> <p>EC2 security group</p> </li> <li> <p>Snapshot</p> </li> <li> <p>Cluster security group</p> </li> <li> <p>Subnet group</p> </li> <li> <p>HSM connection</p> </li> <li> <p>HSM certificate</p> </li> <li> <p>Parameter group</p> </li> <li> <p>Snapshot copy grant</p> </li> </ul> <p>For more information about Amazon Redshift resource types and constructing ARNs, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/redshift-iam-access-control-overview.html#redshift-iam-access-control-specify-actions">Specifying Policy Elements: Actions, Effects, Resources, and Principals</a> in the Amazon Redshift Cluster Management Guide. </p>
    pub resource_type: Option<String>,
    /// <p>A tag key or keys for which you want to return all matching resources that are associated with the specified key or keys. For example, suppose that you have resources tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with all resources that have either or both of these tag keys associated with them.</p>
    pub tag_keys: Option<Vec<String>>,
    /// <p>A tag value or values for which you want to return all matching resources that are associated with the specified value or values. For example, suppose that you have resources tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with all resources that have either or both of these tag values associated with them.</p>
    pub tag_values: Option<Vec<String>>,
}

/// Serialize `DescribeTagsMessage` contents to a `SignedRequest`.
struct DescribeTagsMessageSerializer;
impl DescribeTagsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTagsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resource_name {
            params.put(
                &format!("{}{}", prefix, "ResourceName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resource_type {
            params.put(
                &format!("{}{}", prefix, "ResourceType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tag_keys {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagKey"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tag_values {
            TagValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagValue"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisableLoggingMessage {
    /// <p>The identifier of the cluster on which logging is to be stopped.</p> <p>Example: <code>examplecluster</code> </p>
    pub cluster_identifier: String,
}

/// Serialize `DisableLoggingMessage` contents to a `SignedRequest`.
struct DisableLoggingMessageSerializer;
impl DisableLoggingMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DisableLoggingMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisableSnapshotCopyMessage {
    /// <p>The unique identifier of the source cluster that you want to disable copying of snapshots to a destination region.</p> <p>Constraints: Must be the valid name of an existing cluster that has cross-region snapshot copy enabled.</p>
    pub cluster_identifier: String,
}

/// Serialize `DisableSnapshotCopyMessage` contents to a `SignedRequest`.
struct DisableSnapshotCopyMessageSerializer;
impl DisableSnapshotCopyMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DisableSnapshotCopyMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisableSnapshotCopyResult {
    pub cluster: Option<Cluster>,
}

struct DisableSnapshotCopyResultDeserializer;
impl DisableSnapshotCopyResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DisableSnapshotCopyResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DisableSnapshotCopyResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DoubleDeserializer;
impl DoubleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DoubleOptionalDeserializer;
impl DoubleOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes an Amazon EC2 security group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EC2SecurityGroup {
    /// <p>The name of the EC2 Security Group.</p>
    pub ec2_security_group_name: Option<String>,
    /// <p>The AWS ID of the owner of the EC2 security group specified in the <code>EC2SecurityGroupName</code> field. </p>
    pub ec2_security_group_owner_id: Option<String>,
    /// <p>The status of the EC2 security group.</p>
    pub status: Option<String>,
    /// <p>The list of tags for the EC2 security group.</p>
    pub tags: Option<Vec<Tag>>,
}

struct EC2SecurityGroupDeserializer;
impl EC2SecurityGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EC2SecurityGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EC2SecurityGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EC2SecurityGroupName" => {
                        obj.ec2_security_group_name = Some(try!(StringDeserializer::deserialize(
                            "EC2SecurityGroupName",
                            stack
                        )));
                    }
                    "EC2SecurityGroupOwnerId" => {
                        obj.ec2_security_group_owner_id = Some(try!(
                            StringDeserializer::deserialize("EC2SecurityGroupOwnerId", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EC2SecurityGroupListDeserializer;
impl EC2SecurityGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EC2SecurityGroup>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EC2SecurityGroup" {
                        obj.push(try!(EC2SecurityGroupDeserializer::deserialize(
                            "EC2SecurityGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Describes the status of the elastic IP (EIP) address.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElasticIpStatus {
    /// <p>The elastic IP (EIP) address for the cluster.</p>
    pub elastic_ip: Option<String>,
    /// <p>The status of the elastic IP (EIP) address.</p>
    pub status: Option<String>,
}

struct ElasticIpStatusDeserializer;
impl ElasticIpStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ElasticIpStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ElasticIpStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ElasticIp" => {
                        obj.elastic_ip =
                            Some(try!(StringDeserializer::deserialize("ElasticIp", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableLoggingMessage {
    /// <p><p>The name of an existing S3 bucket where the log files are to be stored.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the same region as the cluster</p> </li> <li> <p>The cluster must have read bucket and put object permissions</p> </li> </ul></p>
    pub bucket_name: String,
    /// <p>The identifier of the cluster on which logging is to be started.</p> <p>Example: <code>examplecluster</code> </p>
    pub cluster_identifier: String,
    /// <p><p>The prefix applied to the log file names.</p> <p>Constraints:</p> <ul> <li> <p>Cannot exceed 512 characters</p> </li> <li> <p>Cannot contain spaces( ), double quotes (&quot;), single quotes (&#39;), a backslash (), or control characters. The hexadecimal codes for invalid characters are: </p> <ul> <li> <p>x00 to x20</p> </li> <li> <p>x22</p> </li> <li> <p>x27</p> </li> <li> <p>x5c</p> </li> <li> <p>x7f or larger</p> </li> </ul> </li> </ul></p>
    pub s3_key_prefix: Option<String>,
}

/// Serialize `EnableLoggingMessage` contents to a `SignedRequest`.
struct EnableLoggingMessageSerializer;
impl EnableLoggingMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EnableLoggingMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "BucketName"),
            &obj.bucket_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.s3_key_prefix {
            params.put(
                &format!("{}{}", prefix, "S3KeyPrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableSnapshotCopyMessage {
    /// <p>The unique identifier of the source cluster to copy snapshots from.</p> <p>Constraints: Must be the valid name of an existing cluster that does not already have cross-region snapshot copy enabled.</p>
    pub cluster_identifier: String,
    /// <p>The destination region that you want to copy snapshots to.</p> <p>Constraints: Must be the name of a valid region. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#redshift_region">Regions and Endpoints</a> in the Amazon Web Services General Reference. </p>
    pub destination_region: String,
    /// <p>The number of days to retain automated snapshots in the destination region after they are copied from the source region.</p> <p>Default: 7.</p> <p>Constraints: Must be at least 1 and no more than 35.</p>
    pub retention_period: Option<i64>,
    /// <p>The name of the snapshot copy grant to use when snapshots of an AWS KMS-encrypted cluster are copied to the destination region.</p>
    pub snapshot_copy_grant_name: Option<String>,
}

/// Serialize `EnableSnapshotCopyMessage` contents to a `SignedRequest`.
struct EnableSnapshotCopyMessageSerializer;
impl EnableSnapshotCopyMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EnableSnapshotCopyMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "DestinationRegion"),
            &obj.destination_region.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.retention_period {
            params.put(
                &format!("{}{}", prefix, "RetentionPeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.snapshot_copy_grant_name {
            params.put(
                &format!("{}{}", prefix, "SnapshotCopyGrantName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableSnapshotCopyResult {
    pub cluster: Option<Cluster>,
}

struct EnableSnapshotCopyResultDeserializer;
impl EnableSnapshotCopyResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnableSnapshotCopyResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnableSnapshotCopyResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a connection endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Endpoint {
    /// <p>The DNS address of the Cluster.</p>
    pub address: Option<String>,
    /// <p>The port that the database engine is listening on.</p>
    pub port: Option<i64>,
}

struct EndpointDeserializer;
impl EndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Endpoint, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Endpoint::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Address" => {
                        obj.address = Some(try!(StringDeserializer::deserialize("Address", stack)));
                    }
                    "Port" => {
                        obj.port = Some(try!(IntegerDeserializer::deserialize("Port", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes an event.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Event {
    /// <p>The date and time of the event.</p>
    pub date: Option<String>,
    /// <p>A list of the event categories.</p> <p>Values: Configuration, Management, Monitoring, Security</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The identifier of the event.</p>
    pub event_id: Option<String>,
    /// <p>The text of this event.</p>
    pub message: Option<String>,
    /// <p>The severity of the event.</p> <p>Values: ERROR, INFO</p>
    pub severity: Option<String>,
    /// <p>The identifier for the source of the event.</p>
    pub source_identifier: Option<String>,
    /// <p>The source type for this event.</p>
    pub source_type: Option<String>,
}

struct EventDeserializer;
impl EventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Event, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Event::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Date" => {
                        obj.date = Some(try!(TStampDeserializer::deserialize("Date", stack)));
                    }
                    "EventCategories" => {
                        obj.event_categories = Some(try!(
                            EventCategoriesListDeserializer::deserialize("EventCategories", stack)
                        ));
                    }
                    "EventId" => {
                        obj.event_id =
                            Some(try!(StringDeserializer::deserialize("EventId", stack)));
                    }
                    "Message" => {
                        obj.message = Some(try!(StringDeserializer::deserialize("Message", stack)));
                    }
                    "Severity" => {
                        obj.severity =
                            Some(try!(StringDeserializer::deserialize("Severity", stack)));
                    }
                    "SourceIdentifier" => {
                        obj.source_identifier = Some(try!(StringDeserializer::deserialize(
                            "SourceIdentifier",
                            stack
                        )));
                    }
                    "SourceType" => {
                        obj.source_type = Some(try!(SourceTypeDeserializer::deserialize(
                            "SourceType",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventCategoriesListDeserializer;
impl EventCategoriesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EventCategory" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "EventCategory",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// <p>Describes event categories.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventCategoriesMap {
    /// <p>The events in the event category.</p>
    pub events: Option<Vec<EventInfoMap>>,
    /// <p>The source type, such as cluster or cluster-snapshot, that the returned categories belong to.</p>
    pub source_type: Option<String>,
}

struct EventCategoriesMapDeserializer;
impl EventCategoriesMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventCategoriesMap, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventCategoriesMap::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Events" => {
                        obj.events = Some(try!(EventInfoMapListDeserializer::deserialize(
                            "Events", stack
                        )));
                    }
                    "SourceType" => {
                        obj.source_type =
                            Some(try!(StringDeserializer::deserialize("SourceType", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventCategoriesMapListDeserializer;
impl EventCategoriesMapListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventCategoriesMap>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EventCategoriesMap" {
                        obj.push(try!(EventCategoriesMapDeserializer::deserialize(
                            "EventCategoriesMap",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventCategoriesMessage {
    /// <p>A list of event categories descriptions.</p>
    pub event_categories_map_list: Option<Vec<EventCategoriesMap>>,
}

struct EventCategoriesMessageDeserializer;
impl EventCategoriesMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventCategoriesMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventCategoriesMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventCategoriesMapList" => {
                        obj.event_categories_map_list =
                            Some(try!(EventCategoriesMapListDeserializer::deserialize(
                                "EventCategoriesMapList",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes event information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventInfoMap {
    /// <p>The category of an Amazon Redshift event.</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>The description of an Amazon Redshift event.</p>
    pub event_description: Option<String>,
    /// <p>The identifier of an Amazon Redshift event.</p>
    pub event_id: Option<String>,
    /// <p>The severity of the event.</p> <p>Values: ERROR, INFO</p>
    pub severity: Option<String>,
}

struct EventInfoMapDeserializer;
impl EventInfoMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventInfoMap, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventInfoMap::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventCategories" => {
                        obj.event_categories = Some(try!(
                            EventCategoriesListDeserializer::deserialize("EventCategories", stack)
                        ));
                    }
                    "EventDescription" => {
                        obj.event_description = Some(try!(StringDeserializer::deserialize(
                            "EventDescription",
                            stack
                        )));
                    }
                    "EventId" => {
                        obj.event_id =
                            Some(try!(StringDeserializer::deserialize("EventId", stack)));
                    }
                    "Severity" => {
                        obj.severity =
                            Some(try!(StringDeserializer::deserialize("Severity", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventInfoMapListDeserializer;
impl EventInfoMapListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventInfoMap>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EventInfoMap" {
                        obj.push(try!(EventInfoMapDeserializer::deserialize(
                            "EventInfoMap",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Event>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Event" {
                        obj.push(try!(EventDeserializer::deserialize("Event", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Describes event subscriptions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventSubscription {
    /// <p>The name of the Amazon Redshift event notification subscription.</p>
    pub cust_subscription_id: Option<String>,
    /// <p>The AWS customer account associated with the Amazon Redshift event notification subscription.</p>
    pub customer_aws_id: Option<String>,
    /// <p>A Boolean value indicating whether the subscription is enabled. <code>true</code> indicates the subscription is enabled.</p>
    pub enabled: Option<bool>,
    /// <p>The list of Amazon Redshift event categories specified in the event notification subscription.</p> <p>Values: Configuration, Management, Monitoring, Security</p>
    pub event_categories_list: Option<Vec<String>>,
    /// <p>The event severity specified in the Amazon Redshift event notification subscription.</p> <p>Values: ERROR, INFO</p>
    pub severity: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic used by the event notification subscription.</p>
    pub sns_topic_arn: Option<String>,
    /// <p>A list of the sources that publish events to the Amazon Redshift event notification subscription.</p>
    pub source_ids_list: Option<Vec<String>>,
    /// <p>The source type of the events returned the Amazon Redshift event notification, such as cluster, or cluster-snapshot.</p>
    pub source_type: Option<String>,
    /// <p><p>The status of the Amazon Redshift event notification subscription.</p> <p>Constraints:</p> <ul> <li> <p>Can be one of the following: active | no-permission | topic-not-exist</p> </li> <li> <p>The status &quot;no-permission&quot; indicates that Amazon Redshift no longer has permission to post to the Amazon SNS topic. The status &quot;topic-not-exist&quot; indicates that the topic was deleted after the subscription was created.</p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The date and time the Amazon Redshift event notification subscription was created.</p>
    pub subscription_creation_time: Option<String>,
    /// <p>The list of tags for the event subscription.</p>
    pub tags: Option<Vec<Tag>>,
}

struct EventSubscriptionDeserializer;
impl EventSubscriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventSubscription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventSubscription::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CustSubscriptionId" => {
                        obj.cust_subscription_id = Some(try!(StringDeserializer::deserialize(
                            "CustSubscriptionId",
                            stack
                        )));
                    }
                    "CustomerAwsId" => {
                        obj.customer_aws_id = Some(try!(StringDeserializer::deserialize(
                            "CustomerAwsId",
                            stack
                        )));
                    }
                    "Enabled" => {
                        obj.enabled =
                            Some(try!(BooleanDeserializer::deserialize("Enabled", stack)));
                    }
                    "EventCategoriesList" => {
                        obj.event_categories_list =
                            Some(try!(EventCategoriesListDeserializer::deserialize(
                                "EventCategoriesList",
                                stack
                            )));
                    }
                    "Severity" => {
                        obj.severity =
                            Some(try!(StringDeserializer::deserialize("Severity", stack)));
                    }
                    "SnsTopicArn" => {
                        obj.sns_topic_arn =
                            Some(try!(StringDeserializer::deserialize("SnsTopicArn", stack)));
                    }
                    "SourceIdsList" => {
                        obj.source_ids_list = Some(try!(SourceIdsListDeserializer::deserialize(
                            "SourceIdsList",
                            stack
                        )));
                    }
                    "SourceType" => {
                        obj.source_type =
                            Some(try!(StringDeserializer::deserialize("SourceType", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "SubscriptionCreationTime" => {
                        obj.subscription_creation_time = Some(try!(
                            TStampDeserializer::deserialize("SubscriptionCreationTime", stack)
                        ));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventSubscriptionsListDeserializer;
impl EventSubscriptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventSubscription>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "EventSubscription" {
                        obj.push(try!(EventSubscriptionDeserializer::deserialize(
                            "EventSubscription",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventSubscriptionsMessage {
    /// <p>A list of event subscriptions.</p>
    pub event_subscriptions_list: Option<Vec<EventSubscription>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct EventSubscriptionsMessageDeserializer;
impl EventSubscriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventSubscriptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventSubscriptionsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscriptionsList" => {
                        obj.event_subscriptions_list =
                            Some(try!(EventSubscriptionsListDeserializer::deserialize(
                                "EventSubscriptionsList",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventsMessage {
    /// <p>A list of <code>Event</code> instances. </p>
    pub events: Option<Vec<Event>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct EventsMessageDeserializer;
impl EventsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Events" => {
                        obj.events =
                            Some(try!(EventListDeserializer::deserialize("Events", stack)));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request parameters to get cluster credentials.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetClusterCredentialsMessage {
    /// <p>Create a database user with the name specified for the user named in <code>DbUser</code> if one does not exist.</p>
    pub auto_create: Option<bool>,
    /// <p>The unique identifier of the cluster that contains the database for which your are requesting credentials. This parameter is case sensitive.</p>
    pub cluster_identifier: String,
    /// <p><p>A list of the names of existing database groups that the user named in <code>DbUser</code> will join for the current session, in addition to any group memberships for an existing user. If not specified, a new user is added only to PUBLIC.</p> <p>Database group name constraints</p> <ul> <li> <p>Must be 1 to 64 alphanumeric characters or hyphens</p> </li> <li> <p>Must contain only lowercase letters, numbers, underscore, plus sign, period (dot), at symbol (@), or hyphen.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Must not contain a colon ( : ) or slash ( / ). </p> </li> <li> <p>Cannot be a reserved word. A list of reserved words can be found in <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved Words</a> in the Amazon Redshift Database Developer Guide.</p> </li> </ul></p>
    pub db_groups: Option<Vec<String>>,
    /// <p><p>The name of a database that <code>DbUser</code> is authorized to log on to. If <code>DbName</code> is not specified, <code>DbUser</code> can log on to any existing database.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 64 alphanumeric characters or hyphens</p> </li> <li> <p>Must contain only lowercase letters, numbers, underscore, plus sign, period (dot), at symbol (@), or hyphen.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Must not contain a colon ( : ) or slash ( / ). </p> </li> <li> <p>Cannot be a reserved word. A list of reserved words can be found in <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved Words</a> in the Amazon Redshift Database Developer Guide.</p> </li> </ul></p>
    pub db_name: Option<String>,
    /// <p><p>The name of a database user. If a user name matching <code>DbUser</code> exists in the database, the temporary user credentials have the same permissions as the existing user. If <code>DbUser</code> doesn&#39;t exist in the database and <code>Autocreate</code> is <code>True</code>, a new user is created using the value for <code>DbUser</code> with PUBLIC permissions. If a database user matching the value for <code>DbUser</code> doesn&#39;t exist and <code>Autocreate</code> is <code>False</code>, then the command succeeds but the connection attempt will fail because the user doesn&#39;t exist in the database.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_USER.html">CREATE USER</a> in the Amazon Redshift Database Developer Guide. </p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 64 alphanumeric characters or hyphens. The user name can&#39;t be <code>PUBLIC</code>.</p> </li> <li> <p>Must contain only lowercase letters, numbers, underscore, plus sign, period (dot), at symbol (@), or hyphen.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Must not contain a colon ( : ) or slash ( / ). </p> </li> <li> <p>Cannot be a reserved word. A list of reserved words can be found in <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_pg_keywords.html">Reserved Words</a> in the Amazon Redshift Database Developer Guide.</p> </li> </ul></p>
    pub db_user: String,
    /// <p>The number of seconds until the returned temporary password expires.</p> <p>Constraint: minimum 900, maximum 3600.</p> <p>Default: 900</p>
    pub duration_seconds: Option<i64>,
}

/// Serialize `GetClusterCredentialsMessage` contents to a `SignedRequest`.
struct GetClusterCredentialsMessageSerializer;
impl GetClusterCredentialsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetClusterCredentialsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auto_create {
            params.put(
                &format!("{}{}", prefix, "AutoCreate"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.db_groups {
            DbGroupListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DbGroup"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.db_name {
            params.put(
                &format!("{}{}", prefix, "DbName"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DbUser"),
            &obj.db_user.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.duration_seconds {
            params.put(
                &format!("{}{}", prefix, "DurationSeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>Returns information about an HSM client certificate. The certificate is stored in a secure Hardware Storage Module (HSM), and used by the Amazon Redshift cluster to encrypt data files.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HsmClientCertificate {
    /// <p>The identifier of the HSM client certificate.</p>
    pub hsm_client_certificate_identifier: Option<String>,
    /// <p>The public key that the Amazon Redshift cluster will use to connect to the HSM. You must register the public key in the HSM.</p>
    pub hsm_client_certificate_public_key: Option<String>,
    /// <p>The list of tags for the HSM client certificate.</p>
    pub tags: Option<Vec<Tag>>,
}

struct HsmClientCertificateDeserializer;
impl HsmClientCertificateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HsmClientCertificate, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HsmClientCertificate::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HsmClientCertificateIdentifier" => {
                        obj.hsm_client_certificate_identifier =
                            Some(try!(StringDeserializer::deserialize(
                                "HsmClientCertificateIdentifier",
                                stack
                            )));
                    }
                    "HsmClientCertificatePublicKey" => {
                        obj.hsm_client_certificate_public_key = Some(try!(
                            StringDeserializer::deserialize("HsmClientCertificatePublicKey", stack)
                        ));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HsmClientCertificateListDeserializer;
impl HsmClientCertificateListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HsmClientCertificate>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "HsmClientCertificate" {
                        obj.push(try!(HsmClientCertificateDeserializer::deserialize(
                            "HsmClientCertificate",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HsmClientCertificateMessage {
    /// <p>A list of the identifiers for one or more HSM client certificates used by Amazon Redshift clusters to store and retrieve database encryption keys in an HSM.</p>
    pub hsm_client_certificates: Option<Vec<HsmClientCertificate>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct HsmClientCertificateMessageDeserializer;
impl HsmClientCertificateMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HsmClientCertificateMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HsmClientCertificateMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HsmClientCertificates" => {
                        obj.hsm_client_certificates =
                            Some(try!(HsmClientCertificateListDeserializer::deserialize(
                                "HsmClientCertificates",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Returns information about an HSM configuration, which is an object that describes to Amazon Redshift clusters the information they require to connect to an HSM where they can store database encryption keys.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HsmConfiguration {
    /// <p>A text description of the HSM configuration.</p>
    pub description: Option<String>,
    /// <p>The name of the Amazon Redshift HSM configuration.</p>
    pub hsm_configuration_identifier: Option<String>,
    /// <p>The IP address that the Amazon Redshift cluster must use to access the HSM.</p>
    pub hsm_ip_address: Option<String>,
    /// <p>The name of the partition in the HSM where the Amazon Redshift clusters will store their database encryption keys.</p>
    pub hsm_partition_name: Option<String>,
    /// <p>The list of tags for the HSM configuration.</p>
    pub tags: Option<Vec<Tag>>,
}

struct HsmConfigurationDeserializer;
impl HsmConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HsmConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HsmConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "HsmConfigurationIdentifier" => {
                        obj.hsm_configuration_identifier = Some(try!(
                            StringDeserializer::deserialize("HsmConfigurationIdentifier", stack)
                        ));
                    }
                    "HsmIpAddress" => {
                        obj.hsm_ip_address =
                            Some(try!(StringDeserializer::deserialize("HsmIpAddress", stack)));
                    }
                    "HsmPartitionName" => {
                        obj.hsm_partition_name = Some(try!(StringDeserializer::deserialize(
                            "HsmPartitionName",
                            stack
                        )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HsmConfigurationListDeserializer;
impl HsmConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HsmConfiguration>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "HsmConfiguration" {
                        obj.push(try!(HsmConfigurationDeserializer::deserialize(
                            "HsmConfiguration",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HsmConfigurationMessage {
    /// <p>A list of <code>HsmConfiguration</code> objects.</p>
    pub hsm_configurations: Option<Vec<HsmConfiguration>>,
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
}

struct HsmConfigurationMessageDeserializer;
impl HsmConfigurationMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HsmConfigurationMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HsmConfigurationMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HsmConfigurations" => {
                        obj.hsm_configurations =
                            Some(try!(HsmConfigurationListDeserializer::deserialize(
                                "HsmConfigurations",
                                stack
                            )));
                    }
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the status of changes to HSM settings.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HsmStatus {
    /// <p>Specifies the name of the HSM client certificate the Amazon Redshift cluster uses to retrieve the data encryption keys stored in an HSM.</p>
    pub hsm_client_certificate_identifier: Option<String>,
    /// <p>Specifies the name of the HSM configuration that contains the information the Amazon Redshift cluster can use to retrieve and store keys in an HSM.</p>
    pub hsm_configuration_identifier: Option<String>,
    /// <p>Reports whether the Amazon Redshift cluster has finished applying any HSM settings changes specified in a modify cluster command.</p> <p>Values: active, applying</p>
    pub status: Option<String>,
}

struct HsmStatusDeserializer;
impl HsmStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HsmStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HsmStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HsmClientCertificateIdentifier" => {
                        obj.hsm_client_certificate_identifier =
                            Some(try!(StringDeserializer::deserialize(
                                "HsmClientCertificateIdentifier",
                                stack
                            )));
                    }
                    "HsmConfigurationIdentifier" => {
                        obj.hsm_configuration_identifier = Some(try!(
                            StringDeserializer::deserialize("HsmConfigurationIdentifier", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes an IP range used in a security group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IPRange {
    /// <p>The IP range in Classless Inter-Domain Routing (CIDR) notation.</p>
    pub cidrip: Option<String>,
    /// <p>The status of the IP range, for example, "authorized".</p>
    pub status: Option<String>,
    /// <p>The list of tags for the IP range.</p>
    pub tags: Option<Vec<Tag>>,
}

struct IPRangeDeserializer;
impl IPRangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IPRange, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IPRange::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CIDRIP" => {
                        obj.cidrip = Some(try!(StringDeserializer::deserialize("CIDRIP", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IPRangeListDeserializer;
impl IPRangeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<IPRange>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "IPRange" {
                        obj.push(try!(IPRangeDeserializer::deserialize("IPRange", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `IamRoleArnList` contents to a `SignedRequest`.
struct IamRoleArnListSerializer;
impl IamRoleArnListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ImportTablesCompletedDeserializer;
impl ImportTablesCompletedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(StringDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ImportTablesInProgressDeserializer;
impl ImportTablesInProgressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(StringDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ImportTablesNotStartedDeserializer;
impl ImportTablesNotStartedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(StringDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IntegerOptionalDeserializer;
impl IntegerOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the status of logging for a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoggingStatus {
    /// <p>The name of the S3 bucket where the log files are stored.</p>
    pub bucket_name: Option<String>,
    /// <p>The message indicating that logs failed to be delivered.</p>
    pub last_failure_message: Option<String>,
    /// <p>The last time when logs failed to be delivered.</p>
    pub last_failure_time: Option<String>,
    /// <p>The last time that logs were delivered.</p>
    pub last_successful_delivery_time: Option<String>,
    /// <p> <code>true</code> if logging is on, <code>false</code> if logging is off.</p>
    pub logging_enabled: Option<bool>,
    /// <p>The prefix applied to the log file names.</p>
    pub s3_key_prefix: Option<String>,
}

struct LoggingStatusDeserializer;
impl LoggingStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoggingStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoggingStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "BucketName" => {
                        obj.bucket_name =
                            Some(try!(StringDeserializer::deserialize("BucketName", stack)));
                    }
                    "LastFailureMessage" => {
                        obj.last_failure_message = Some(try!(StringDeserializer::deserialize(
                            "LastFailureMessage",
                            stack
                        )));
                    }
                    "LastFailureTime" => {
                        obj.last_failure_time = Some(try!(TStampDeserializer::deserialize(
                            "LastFailureTime",
                            stack
                        )));
                    }
                    "LastSuccessfulDeliveryTime" => {
                        obj.last_successful_delivery_time = Some(try!(
                            TStampDeserializer::deserialize("LastSuccessfulDeliveryTime", stack)
                        ));
                    }
                    "LoggingEnabled" => {
                        obj.logging_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "LoggingEnabled",
                            stack
                        )));
                    }
                    "S3KeyPrefix" => {
                        obj.s3_key_prefix =
                            Some(try!(StringDeserializer::deserialize("S3KeyPrefix", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct LongDeserializer;
impl LongDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct LongOptionalDeserializer;
impl LongOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyClusterIamRolesMessage {
    /// <p>Zero or more IAM roles to associate with the cluster. The roles must be in their Amazon Resource Name (ARN) format. You can associate up to 10 IAM roles with a single cluster in a single request.</p>
    pub add_iam_roles: Option<Vec<String>>,
    /// <p>The unique identifier of the cluster for which you want to associate or disassociate IAM roles.</p>
    pub cluster_identifier: String,
    /// <p>Zero or more IAM roles in ARN format to disassociate from the cluster. You can disassociate up to 10 IAM roles from a single cluster in a single request.</p>
    pub remove_iam_roles: Option<Vec<String>>,
}

/// Serialize `ModifyClusterIamRolesMessage` contents to a `SignedRequest`.
struct ModifyClusterIamRolesMessageSerializer;
impl ModifyClusterIamRolesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyClusterIamRolesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.add_iam_roles {
            IamRoleArnListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "IamRoleArn"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.remove_iam_roles {
            IamRoleArnListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "IamRoleArn"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyClusterIamRolesResult {
    pub cluster: Option<Cluster>,
}

struct ModifyClusterIamRolesResultDeserializer;
impl ModifyClusterIamRolesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyClusterIamRolesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyClusterIamRolesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyClusterMessage {
    /// <p>If <code>true</code>, major version upgrades will be applied automatically to the cluster during the maintenance window. </p> <p>Default: <code>false</code> </p>
    pub allow_version_upgrade: Option<bool>,
    /// <p>The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Even if automated snapshots are disabled, you can still create manual snapshots when you want with <a>CreateClusterSnapshot</a>. </p> <p>If you decrease the automated snapshot retention period from its current value, existing automated snapshots that fall outside of the new retention period will be immediately deleted.</p> <p>Default: Uses existing setting.</p> <p>Constraints: Must be a value from 0 to 35.</p>
    pub automated_snapshot_retention_period: Option<i64>,
    /// <p>The unique identifier of the cluster to be modified.</p> <p>Example: <code>examplecluster</code> </p>
    pub cluster_identifier: String,
    /// <p>The name of the cluster parameter group to apply to this cluster. This change is applied only after the cluster is rebooted. To reboot a cluster use <a>RebootCluster</a>. </p> <p>Default: Uses existing setting.</p> <p>Constraints: The cluster parameter group must be in the same parameter group family that matches the cluster version.</p>
    pub cluster_parameter_group_name: Option<String>,
    /// <p><p>A list of cluster security groups to be authorized on this cluster. This change is asynchronously applied as soon as possible.</p> <p>Security groups currently associated with the cluster, and not in the list of groups to apply, will be revoked from the cluster.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters or hyphens</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub cluster_security_groups: Option<Vec<String>>,
    /// <p>The new cluster type.</p> <p>When you submit your cluster resize request, your existing cluster goes into a read-only mode. After Amazon Redshift provisions a new cluster based on your resize requirements, there will be outage for a period while the old cluster is deleted and your connection is switched to the new cluster. You can use <a>DescribeResize</a> to track the progress of the resize request. </p> <p>Valid Values: <code> multi-node | single-node </code> </p>
    pub cluster_type: Option<String>,
    /// <p>The new version number of the Amazon Redshift engine to upgrade to.</p> <p>For major version upgrades, if a non-default cluster parameter group is currently in use, a new cluster parameter group in the cluster parameter group family for the new version must be specified. The new cluster parameter group can be the default for that cluster parameter group family. For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>Example: <code>1.0</code> </p>
    pub cluster_version: Option<String>,
    /// <p>The Elastic IP (EIP) address for the cluster.</p> <p>Constraints: The cluster must be provisioned in EC2-VPC and publicly-accessible through an Internet gateway. For more information about provisioning clusters in EC2-VPC, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#cluster-platforms">Supported Platforms to Launch Your Cluster</a> in the Amazon Redshift Cluster Management Guide.</p>
    pub elastic_ip: Option<String>,
    /// <p>An option that specifies whether to create the cluster with enhanced VPC routing enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a VPC. For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/enhanced-vpc-routing.html">Enhanced VPC Routing</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If this option is <code>true</code>, enhanced VPC routing is enabled. </p> <p>Default: false</p>
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>Specifies the name of the HSM client certificate the Amazon Redshift cluster uses to retrieve the data encryption keys stored in an HSM.</p>
    pub hsm_client_certificate_identifier: Option<String>,
    /// <p>Specifies the name of the HSM configuration that contains the information the Amazon Redshift cluster can use to retrieve and store keys in an HSM.</p>
    pub hsm_configuration_identifier: Option<String>,
    /// <p><p>The new password for the cluster master user. This change is asynchronously applied as soon as possible. Between the time of the request and the completion of the request, the <code>MasterUserPassword</code> element exists in the <code>PendingModifiedValues</code> element of the operation response. </p> <note> <p>Operations never return the password, so this operation provides a way to regain access to the master user account for a cluster if the password is lost.</p> </note> <p>Default: Uses existing setting.</p> <p>Constraints:</p> <ul> <li> <p>Must be between 8 and 64 characters in length.</p> </li> <li> <p>Must contain at least one uppercase letter.</p> </li> <li> <p>Must contain at least one lowercase letter.</p> </li> <li> <p>Must contain one number.</p> </li> <li> <p>Can be any printable ASCII character (ASCII code 33 to 126) except &#39; (single quote), &quot; (double quote), \, /, @, or space.</p> </li> </ul></p>
    pub master_user_password: Option<String>,
    /// <p>The new identifier for the cluster.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>Alphabetic characters must be lowercase.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> <li> <p>Must be unique for all clusters within an AWS account.</p> </li> </ul> <p>Example: <code>examplecluster</code> </p>
    pub new_cluster_identifier: Option<String>,
    /// <p>The new node type of the cluster. If you specify a new node type, you must also specify the number of nodes parameter.</p> <p>When you submit your request to resize a cluster, Amazon Redshift sets access permissions for the cluster to read-only. After Amazon Redshift provisions a new cluster according to your resize requirements, there will be a temporary outage while the old cluster is deleted and your connection is switched to the new cluster. When the new connection is complete, the original access permissions for the cluster are restored. You can use <a>DescribeResize</a> to track the progress of the resize request. </p> <p>Valid Values: <code>ds2.xlarge</code> | <code>ds2.8xlarge</code> | <code>dc1.large</code> | <code>dc1.8xlarge</code> | <code>dc2.large</code> | <code>dc2.8xlarge</code> </p>
    pub node_type: Option<String>,
    /// <p>The new number of nodes of the cluster. If you specify a new number of nodes, you must also specify the node type parameter.</p> <p>When you submit your request to resize a cluster, Amazon Redshift sets access permissions for the cluster to read-only. After Amazon Redshift provisions a new cluster according to your resize requirements, there will be a temporary outage while the old cluster is deleted and your connection is switched to the new cluster. When the new connection is complete, the original access permissions for the cluster are restored. You can use <a>DescribeResize</a> to track the progress of the resize request. </p> <p>Valid Values: Integer greater than <code>0</code>.</p>
    pub number_of_nodes: Option<i64>,
    /// <p>The weekly time range (in UTC) during which system maintenance can occur, if necessary. If system maintenance is necessary during the window, it may result in an outage.</p> <p>This maintenance window change is made immediately. If the new maintenance window indicates the current time, there must be at least 120 minutes between the current time and end of the window in order to ensure that pending changes are applied.</p> <p>Default: Uses existing setting.</p> <p>Format: ddd:hh24:mi-ddd:hh24:mi, for example <code>wed:07:30-wed:08:00</code>.</p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Must be at least 30 minutes.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>If <code>true</code>, the cluster can be accessed from a public network. Only clusters in VPCs can be set to be publicly available.</p>
    pub publicly_accessible: Option<bool>,
    /// <p>A list of virtual private cloud (VPC) security groups to be associated with the cluster.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `ModifyClusterMessage` contents to a `SignedRequest`.
struct ModifyClusterMessageSerializer;
impl ModifyClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allow_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AllowVersionUpgrade"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.automated_snapshot_retention_period {
            params.put(
                &format!("{}{}", prefix, "AutomatedSnapshotRetentionPeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "ClusterParameterGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_security_groups {
            ClusterSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ClusterSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cluster_type {
            params.put(
                &format!("{}{}", prefix, "ClusterType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_version {
            params.put(
                &format!("{}{}", prefix, "ClusterVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.elastic_ip {
            params.put(
                &format!("{}{}", prefix, "ElasticIp"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.enhanced_vpc_routing {
            params.put(
                &format!("{}{}", prefix, "EnhancedVpcRouting"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.hsm_client_certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmClientCertificateIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.hsm_configuration_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmConfigurationIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.master_user_password {
            params.put(
                &format!("{}{}", prefix, "MasterUserPassword"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.new_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "NewClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.node_type {
            params.put(
                &format!("{}{}", prefix, "NodeType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.number_of_nodes {
            params.put(
                &format!("{}{}", prefix, "NumberOfNodes"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.publicly_accessible {
            params.put(
                &format!("{}{}", prefix, "PubliclyAccessible"),
                &field_value.to_string().replace("+", "%2B"),
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

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyClusterParameterGroupMessage {
    /// <p>The name of the parameter group to be modified.</p>
    pub parameter_group_name: String,
    /// <p>An array of parameters to be modified. A maximum of 20 parameters can be modified in a single request.</p> <p>For each parameter to be modified, you must supply at least the parameter name and parameter value; other name-value pairs of the parameter are optional.</p> <p>For the workload management (WLM) configuration, you must supply all the name-value pairs in the wlm_json_configuration parameter.</p>
    pub parameters: Vec<Parameter>,
}

/// Serialize `ModifyClusterParameterGroupMessage` contents to a `SignedRequest`.
struct ModifyClusterParameterGroupMessageSerializer;
impl ModifyClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ParameterGroupName"),
            &obj.parameter_group_name.replace("+", "%2B"),
        );
        ParametersListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Parameter"),
            &obj.parameters,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyClusterResult {
    pub cluster: Option<Cluster>,
}

struct ModifyClusterResultDeserializer;
impl ModifyClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyClusterSubnetGroupMessage {
    /// <p>The name of the subnet group to be modified.</p>
    pub cluster_subnet_group_name: String,
    /// <p>A text description of the subnet group to be modified.</p>
    pub description: Option<String>,
    /// <p>An array of VPC subnet IDs. A maximum of 20 subnets can be modified in a single request.</p>
    pub subnet_ids: Vec<String>,
}

/// Serialize `ModifyClusterSubnetGroupMessage` contents to a `SignedRequest`.
struct ModifyClusterSubnetGroupMessageSerializer;
impl ModifyClusterSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyClusterSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterSubnetGroupName"),
            &obj.cluster_subnet_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        SubnetIdentifierListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SubnetIdentifier"),
            &obj.subnet_ids,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyClusterSubnetGroupResult {
    pub cluster_subnet_group: Option<ClusterSubnetGroup>,
}

struct ModifyClusterSubnetGroupResultDeserializer;
impl ModifyClusterSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyClusterSubnetGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyClusterSubnetGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSubnetGroup" => {
                        obj.cluster_subnet_group =
                            Some(try!(ClusterSubnetGroupDeserializer::deserialize(
                                "ClusterSubnetGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyEventSubscriptionMessage {
    /// <p>A Boolean value indicating if the subscription is enabled. <code>true</code> indicates the subscription is enabled </p>
    pub enabled: Option<bool>,
    /// <p>Specifies the Amazon Redshift event categories to be published by the event notification subscription.</p> <p>Values: Configuration, Management, Monitoring, Security</p>
    pub event_categories: Option<Vec<String>>,
    /// <p>Specifies the Amazon Redshift event severity to be published by the event notification subscription.</p> <p>Values: ERROR, INFO</p>
    pub severity: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic to be used by the event notification subscription.</p>
    pub sns_topic_arn: Option<String>,
    /// <p>A list of one or more identifiers of Amazon Redshift source objects. All of the objects must be of the same type as was specified in the source type parameter. The event subscription will return only events generated by the specified objects. If not specified, then events are returned for all objects within the source type specified.</p> <p>Example: my-cluster-1, my-cluster-2</p> <p>Example: my-snapshot-20131010</p>
    pub source_ids: Option<Vec<String>>,
    /// <p>The type of source that will be generating the events. For example, if you want to be notified of events generated by a cluster, you would set this parameter to cluster. If this value is not specified, events are returned for all Amazon Redshift objects in your AWS account. You must specify a source type in order to specify source IDs.</p> <p>Valid values: cluster, cluster-parameter-group, cluster-security-group, and cluster-snapshot.</p>
    pub source_type: Option<String>,
    /// <p>The name of the modified Amazon Redshift event notification subscription.</p>
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
            params.put(
                &format!("{}{}", prefix, "Enabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.event_categories {
            EventCategoriesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EventCategory"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.severity {
            params.put(
                &format!("{}{}", prefix, "Severity"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.sns_topic_arn {
            params.put(
                &format!("{}{}", prefix, "SnsTopicArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_ids {
            SourceIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_type {
            params.put(
                &format!("{}{}", prefix, "SourceType"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionName"),
            &obj.subscription_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyEventSubscriptionResult {
    pub event_subscription: Option<EventSubscription>,
}

struct ModifyEventSubscriptionResultDeserializer;
impl ModifyEventSubscriptionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyEventSubscriptionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyEventSubscriptionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventSubscription" => {
                        obj.event_subscription = Some(try!(
                            EventSubscriptionDeserializer::deserialize("EventSubscription", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifySnapshotCopyRetentionPeriodMessage {
    /// <p>The unique identifier of the cluster for which you want to change the retention period for automated snapshots that are copied to a destination region.</p> <p>Constraints: Must be the valid name of an existing cluster that has cross-region snapshot copy enabled.</p>
    pub cluster_identifier: String,
    /// <p>The number of days to retain automated snapshots in the destination region after they are copied from the source region.</p> <p>If you decrease the retention period for automated snapshots that are copied to a destination region, Amazon Redshift will delete any existing automated snapshots that were copied to the destination region and that fall outside of the new retention period.</p> <p>Constraints: Must be at least 1 and no more than 35.</p>
    pub retention_period: i64,
}

/// Serialize `ModifySnapshotCopyRetentionPeriodMessage` contents to a `SignedRequest`.
struct ModifySnapshotCopyRetentionPeriodMessageSerializer;
impl ModifySnapshotCopyRetentionPeriodMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifySnapshotCopyRetentionPeriodMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "RetentionPeriod"),
            &obj.retention_period.to_string().replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifySnapshotCopyRetentionPeriodResult {
    pub cluster: Option<Cluster>,
}

struct ModifySnapshotCopyRetentionPeriodResultDeserializer;
impl ModifySnapshotCopyRetentionPeriodResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifySnapshotCopyRetentionPeriodResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifySnapshotCopyRetentionPeriodResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes an orderable cluster option.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OrderableClusterOption {
    /// <p>A list of availability zones for the orderable cluster.</p>
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The cluster type, for example <code>multi-node</code>. </p>
    pub cluster_type: Option<String>,
    /// <p>The version of the orderable cluster.</p>
    pub cluster_version: Option<String>,
    /// <p>The node type for the orderable cluster.</p>
    pub node_type: Option<String>,
}

struct OrderableClusterOptionDeserializer;
impl OrderableClusterOptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OrderableClusterOption, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OrderableClusterOption::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AvailabilityZones" => {
                        obj.availability_zones =
                            Some(try!(AvailabilityZoneListDeserializer::deserialize(
                                "AvailabilityZones",
                                stack
                            )));
                    }
                    "ClusterType" => {
                        obj.cluster_type =
                            Some(try!(StringDeserializer::deserialize("ClusterType", stack)));
                    }
                    "ClusterVersion" => {
                        obj.cluster_version = Some(try!(StringDeserializer::deserialize(
                            "ClusterVersion",
                            stack
                        )));
                    }
                    "NodeType" => {
                        obj.node_type =
                            Some(try!(StringDeserializer::deserialize("NodeType", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct OrderableClusterOptionsListDeserializer;
impl OrderableClusterOptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OrderableClusterOption>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "OrderableClusterOption" {
                        obj.push(try!(OrderableClusterOptionDeserializer::deserialize(
                            "OrderableClusterOption",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the output from the <a>DescribeOrderableClusterOptions</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OrderableClusterOptionsMessage {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>An <code>OrderableClusterOption</code> structure containing information about orderable options for the cluster.</p>
    pub orderable_cluster_options: Option<Vec<OrderableClusterOption>>,
}

struct OrderableClusterOptionsMessageDeserializer;
impl OrderableClusterOptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OrderableClusterOptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OrderableClusterOptionsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "OrderableClusterOptions" => {
                        obj.orderable_cluster_options =
                            Some(try!(OrderableClusterOptionsListDeserializer::deserialize(
                                "OrderableClusterOptions",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a parameter in a cluster parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Parameter {
    /// <p>The valid range of values for the parameter.</p>
    pub allowed_values: Option<String>,
    /// <p>Specifies how to apply the WLM configuration parameter. Some properties can be applied dynamically, while other properties require that any associated clusters be rebooted for the configuration changes to be applied. For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    pub apply_type: Option<String>,
    /// <p>The data type of the parameter.</p>
    pub data_type: Option<String>,
    /// <p>A description of the parameter.</p>
    pub description: Option<String>,
    /// <p>If <code>true</code>, the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed. </p>
    pub is_modifiable: Option<bool>,
    /// <p>The earliest engine version to which the parameter can apply.</p>
    pub minimum_engine_version: Option<String>,
    /// <p>The name of the parameter.</p>
    pub parameter_name: Option<String>,
    /// <p>The value of the parameter.</p>
    pub parameter_value: Option<String>,
    /// <p>The source of the parameter value, such as "engine-default" or "user".</p>
    pub source: Option<String>,
}

struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Parameter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Parameter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllowedValues" => {
                        obj.allowed_values = Some(try!(StringDeserializer::deserialize(
                            "AllowedValues",
                            stack
                        )));
                    }
                    "ApplyType" => {
                        obj.apply_type = Some(try!(ParameterApplyTypeDeserializer::deserialize(
                            "ApplyType",
                            stack
                        )));
                    }
                    "DataType" => {
                        obj.data_type =
                            Some(try!(StringDeserializer::deserialize("DataType", stack)));
                    }
                    "Description" => {
                        obj.description =
                            Some(try!(StringDeserializer::deserialize("Description", stack)));
                    }
                    "IsModifiable" => {
                        obj.is_modifiable = Some(try!(BooleanDeserializer::deserialize(
                            "IsModifiable",
                            stack
                        )));
                    }
                    "MinimumEngineVersion" => {
                        obj.minimum_engine_version = Some(try!(StringDeserializer::deserialize(
                            "MinimumEngineVersion",
                            stack
                        )));
                    }
                    "ParameterName" => {
                        obj.parameter_name = Some(try!(StringDeserializer::deserialize(
                            "ParameterName",
                            stack
                        )));
                    }
                    "ParameterValue" => {
                        obj.parameter_value = Some(try!(StringDeserializer::deserialize(
                            "ParameterValue",
                            stack
                        )));
                    }
                    "Source" => {
                        obj.source = Some(try!(StringDeserializer::deserialize("Source", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            params.put(
                &format!("{}{}", prefix, "AllowedValues"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.apply_type {
            params.put(
                &format!("{}{}", prefix, "ApplyType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.data_type {
            params.put(
                &format!("{}{}", prefix, "DataType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.is_modifiable {
            params.put(
                &format!("{}{}", prefix, "IsModifiable"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.minimum_engine_version {
            params.put(
                &format!("{}{}", prefix, "MinimumEngineVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.parameter_name {
            params.put(
                &format!("{}{}", prefix, "ParameterName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.parameter_value {
            params.put(
                &format!("{}{}", prefix, "ParameterValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source {
            params.put(
                &format!("{}{}", prefix, "Source"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct ParameterApplyTypeDeserializer;
impl ParameterApplyTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ParameterGroupListDeserializer;
impl ParameterGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ClusterParameterGroup>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ClusterParameterGroup" {
                        obj.push(try!(ClusterParameterGroupDeserializer::deserialize(
                            "ClusterParameterGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ParametersListDeserializer;
impl ParametersListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Parameter>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Parameter" {
                        obj.push(try!(ParameterDeserializer::deserialize("Parameter", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// <p>Describes cluster attributes that are in a pending state. A change to one or more the attributes was requested and is in progress or will be applied.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PendingModifiedValues {
    /// <p>The pending or in-progress change of the automated snapshot retention period.</p>
    pub automated_snapshot_retention_period: Option<i64>,
    /// <p>The pending or in-progress change of the new identifier for the cluster.</p>
    pub cluster_identifier: Option<String>,
    /// <p>The pending or in-progress change of the cluster type.</p>
    pub cluster_type: Option<String>,
    /// <p>The pending or in-progress change of the service version.</p>
    pub cluster_version: Option<String>,
    /// <p>An option that specifies whether to create the cluster with enhanced VPC routing enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a VPC. For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/enhanced-vpc-routing.html">Enhanced VPC Routing</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If this option is <code>true</code>, enhanced VPC routing is enabled. </p> <p>Default: false</p>
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>The pending or in-progress change of the master user password for the cluster.</p>
    pub master_user_password: Option<String>,
    /// <p>The pending or in-progress change of the cluster's node type.</p>
    pub node_type: Option<String>,
    /// <p>The pending or in-progress change of the number of nodes in the cluster.</p>
    pub number_of_nodes: Option<i64>,
    /// <p>The pending or in-progress change of the ability to connect to the cluster from the public network.</p>
    pub publicly_accessible: Option<bool>,
}

struct PendingModifiedValuesDeserializer;
impl PendingModifiedValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingModifiedValues, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PendingModifiedValues::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AutomatedSnapshotRetentionPeriod" => {
                        obj.automated_snapshot_retention_period =
                            Some(try!(IntegerOptionalDeserializer::deserialize(
                                "AutomatedSnapshotRetentionPeriod",
                                stack
                            )));
                    }
                    "ClusterIdentifier" => {
                        obj.cluster_identifier = Some(try!(StringDeserializer::deserialize(
                            "ClusterIdentifier",
                            stack
                        )));
                    }
                    "ClusterType" => {
                        obj.cluster_type =
                            Some(try!(StringDeserializer::deserialize("ClusterType", stack)));
                    }
                    "ClusterVersion" => {
                        obj.cluster_version = Some(try!(StringDeserializer::deserialize(
                            "ClusterVersion",
                            stack
                        )));
                    }
                    "EnhancedVpcRouting" => {
                        obj.enhanced_vpc_routing = Some(try!(
                            BooleanOptionalDeserializer::deserialize("EnhancedVpcRouting", stack)
                        ));
                    }
                    "MasterUserPassword" => {
                        obj.master_user_password = Some(try!(StringDeserializer::deserialize(
                            "MasterUserPassword",
                            stack
                        )));
                    }
                    "NodeType" => {
                        obj.node_type =
                            Some(try!(StringDeserializer::deserialize("NodeType", stack)));
                    }
                    "NumberOfNodes" => {
                        obj.number_of_nodes = Some(try!(IntegerOptionalDeserializer::deserialize(
                            "NumberOfNodes",
                            stack
                        )));
                    }
                    "PubliclyAccessible" => {
                        obj.publicly_accessible = Some(try!(
                            BooleanOptionalDeserializer::deserialize("PubliclyAccessible", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PurchaseReservedNodeOfferingMessage {
    /// <p>The number of reserved nodes that you want to purchase.</p> <p>Default: <code>1</code> </p>
    pub node_count: Option<i64>,
    /// <p>The unique identifier of the reserved node offering you want to purchase.</p>
    pub reserved_node_offering_id: String,
}

/// Serialize `PurchaseReservedNodeOfferingMessage` contents to a `SignedRequest`.
struct PurchaseReservedNodeOfferingMessageSerializer;
impl PurchaseReservedNodeOfferingMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PurchaseReservedNodeOfferingMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.node_count {
            params.put(
                &format!("{}{}", prefix, "NodeCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReservedNodeOfferingId"),
            &obj.reserved_node_offering_id.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PurchaseReservedNodeOfferingResult {
    pub reserved_node: Option<ReservedNode>,
}

struct PurchaseReservedNodeOfferingResultDeserializer;
impl PurchaseReservedNodeOfferingResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PurchaseReservedNodeOfferingResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PurchaseReservedNodeOfferingResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ReservedNode" => {
                        obj.reserved_node = Some(try!(ReservedNodeDeserializer::deserialize(
                            "ReservedNode",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RebootClusterMessage {
    /// <p>The cluster identifier.</p>
    pub cluster_identifier: String,
}

/// Serialize `RebootClusterMessage` contents to a `SignedRequest`.
struct RebootClusterMessageSerializer;
impl RebootClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RebootClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RebootClusterResult {
    pub cluster: Option<Cluster>,
}

struct RebootClusterResultDeserializer;
impl RebootClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RebootClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RebootClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a recurring charge.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RecurringCharge {
    /// <p>The amount charged per the period of time specified by the recurring charge frequency.</p>
    pub recurring_charge_amount: Option<f64>,
    /// <p>The frequency at which the recurring charge amount is applied.</p>
    pub recurring_charge_frequency: Option<String>,
}

struct RecurringChargeDeserializer;
impl RecurringChargeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecurringCharge, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RecurringCharge::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "RecurringChargeAmount" => {
                        obj.recurring_charge_amount = Some(try!(DoubleDeserializer::deserialize(
                            "RecurringChargeAmount",
                            stack
                        )));
                    }
                    "RecurringChargeFrequency" => {
                        obj.recurring_charge_frequency = Some(try!(
                            StringDeserializer::deserialize("RecurringChargeFrequency", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct RecurringChargeListDeserializer;
impl RecurringChargeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RecurringCharge>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "RecurringCharge" {
                        obj.push(try!(RecurringChargeDeserializer::deserialize(
                            "RecurringCharge",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Describes a reserved node. You can call the <a>DescribeReservedNodeOfferings</a> API to obtain the available reserved node offerings. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReservedNode {
    /// <p>The currency code for the reserved cluster.</p>
    pub currency_code: Option<String>,
    /// <p>The duration of the node reservation in seconds.</p>
    pub duration: Option<i64>,
    /// <p>The fixed cost Amazon Redshift charges you for this reserved node.</p>
    pub fixed_price: Option<f64>,
    /// <p>The number of reserved compute nodes.</p>
    pub node_count: Option<i64>,
    /// <p>The node type of the reserved node.</p>
    pub node_type: Option<String>,
    /// <p>The anticipated utilization of the reserved node, as defined in the reserved node offering.</p>
    pub offering_type: Option<String>,
    /// <p>The recurring charges for the reserved node.</p>
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>The unique identifier for the reservation.</p>
    pub reserved_node_id: Option<String>,
    /// <p>The identifier for the reserved node offering.</p>
    pub reserved_node_offering_id: Option<String>,
    pub reserved_node_offering_type: Option<String>,
    /// <p>The time the reservation started. You purchase a reserved node offering for a duration. This is the start time of that duration.</p>
    pub start_time: Option<String>,
    /// <p><p>The state of the reserved compute node.</p> <p>Possible Values:</p> <ul> <li> <p>pending-payment-This reserved node has recently been purchased, and the sale has been approved, but payment has not yet been confirmed.</p> </li> <li> <p>active-This reserved node is owned by the caller and is available for use.</p> </li> <li> <p>payment-failed-Payment failed for the purchase attempt.</p> </li> </ul></p>
    pub state: Option<String>,
    /// <p>The hourly rate Amazon Redshift charges you for this reserved node.</p>
    pub usage_price: Option<f64>,
}

struct ReservedNodeDeserializer;
impl ReservedNodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedNode, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedNode::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CurrencyCode" => {
                        obj.currency_code =
                            Some(try!(StringDeserializer::deserialize("CurrencyCode", stack)));
                    }
                    "Duration" => {
                        obj.duration =
                            Some(try!(IntegerDeserializer::deserialize("Duration", stack)));
                    }
                    "FixedPrice" => {
                        obj.fixed_price =
                            Some(try!(DoubleDeserializer::deserialize("FixedPrice", stack)));
                    }
                    "NodeCount" => {
                        obj.node_count =
                            Some(try!(IntegerDeserializer::deserialize("NodeCount", stack)));
                    }
                    "NodeType" => {
                        obj.node_type =
                            Some(try!(StringDeserializer::deserialize("NodeType", stack)));
                    }
                    "OfferingType" => {
                        obj.offering_type =
                            Some(try!(StringDeserializer::deserialize("OfferingType", stack)));
                    }
                    "RecurringCharges" => {
                        obj.recurring_charges = Some(try!(
                            RecurringChargeListDeserializer::deserialize("RecurringCharges", stack)
                        ));
                    }
                    "ReservedNodeId" => {
                        obj.reserved_node_id = Some(try!(StringDeserializer::deserialize(
                            "ReservedNodeId",
                            stack
                        )));
                    }
                    "ReservedNodeOfferingId" => {
                        obj.reserved_node_offering_id = Some(try!(
                            StringDeserializer::deserialize("ReservedNodeOfferingId", stack)
                        ));
                    }
                    "ReservedNodeOfferingType" => {
                        obj.reserved_node_offering_type =
                            Some(try!(ReservedNodeOfferingTypeDeserializer::deserialize(
                                "ReservedNodeOfferingType",
                                stack
                            )));
                    }
                    "StartTime" => {
                        obj.start_time =
                            Some(try!(TStampDeserializer::deserialize("StartTime", stack)));
                    }
                    "State" => {
                        obj.state = Some(try!(StringDeserializer::deserialize("State", stack)));
                    }
                    "UsagePrice" => {
                        obj.usage_price =
                            Some(try!(DoubleDeserializer::deserialize("UsagePrice", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ReservedNodeListDeserializer;
impl ReservedNodeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReservedNode>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ReservedNode" {
                        obj.push(try!(ReservedNodeDeserializer::deserialize(
                            "ReservedNode",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Describes a reserved node offering.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReservedNodeOffering {
    /// <p>The currency code for the compute nodes offering.</p>
    pub currency_code: Option<String>,
    /// <p>The duration, in seconds, for which the offering will reserve the node.</p>
    pub duration: Option<i64>,
    /// <p>The upfront fixed charge you will pay to purchase the specific reserved node offering.</p>
    pub fixed_price: Option<f64>,
    /// <p>The node type offered by the reserved node offering.</p>
    pub node_type: Option<String>,
    /// <p>The anticipated utilization of the reserved node, as defined in the reserved node offering.</p>
    pub offering_type: Option<String>,
    /// <p>The charge to your account regardless of whether you are creating any clusters using the node offering. Recurring charges are only in effect for heavy-utilization reserved nodes.</p>
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>The offering identifier.</p>
    pub reserved_node_offering_id: Option<String>,
    pub reserved_node_offering_type: Option<String>,
    /// <p>The rate you are charged for each hour the cluster that is using the offering is running.</p>
    pub usage_price: Option<f64>,
}

struct ReservedNodeOfferingDeserializer;
impl ReservedNodeOfferingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedNodeOffering, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedNodeOffering::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CurrencyCode" => {
                        obj.currency_code =
                            Some(try!(StringDeserializer::deserialize("CurrencyCode", stack)));
                    }
                    "Duration" => {
                        obj.duration =
                            Some(try!(IntegerDeserializer::deserialize("Duration", stack)));
                    }
                    "FixedPrice" => {
                        obj.fixed_price =
                            Some(try!(DoubleDeserializer::deserialize("FixedPrice", stack)));
                    }
                    "NodeType" => {
                        obj.node_type =
                            Some(try!(StringDeserializer::deserialize("NodeType", stack)));
                    }
                    "OfferingType" => {
                        obj.offering_type =
                            Some(try!(StringDeserializer::deserialize("OfferingType", stack)));
                    }
                    "RecurringCharges" => {
                        obj.recurring_charges = Some(try!(
                            RecurringChargeListDeserializer::deserialize("RecurringCharges", stack)
                        ));
                    }
                    "ReservedNodeOfferingId" => {
                        obj.reserved_node_offering_id = Some(try!(
                            StringDeserializer::deserialize("ReservedNodeOfferingId", stack)
                        ));
                    }
                    "ReservedNodeOfferingType" => {
                        obj.reserved_node_offering_type =
                            Some(try!(ReservedNodeOfferingTypeDeserializer::deserialize(
                                "ReservedNodeOfferingType",
                                stack
                            )));
                    }
                    "UsagePrice" => {
                        obj.usage_price =
                            Some(try!(DoubleDeserializer::deserialize("UsagePrice", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ReservedNodeOfferingListDeserializer;
impl ReservedNodeOfferingListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReservedNodeOffering>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ReservedNodeOffering" {
                        obj.push(try!(ReservedNodeOfferingDeserializer::deserialize(
                            "ReservedNodeOffering",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ReservedNodeOfferingTypeDeserializer;
impl ReservedNodeOfferingTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReservedNodeOfferingsMessage {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>A list of <code>ReservedNodeOffering</code> objects.</p>
    pub reserved_node_offerings: Option<Vec<ReservedNodeOffering>>,
}

struct ReservedNodeOfferingsMessageDeserializer;
impl ReservedNodeOfferingsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedNodeOfferingsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedNodeOfferingsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "ReservedNodeOfferings" => {
                        obj.reserved_node_offerings =
                            Some(try!(ReservedNodeOfferingListDeserializer::deserialize(
                                "ReservedNodeOfferings",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReservedNodesMessage {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>The list of <code>ReservedNode</code> objects.</p>
    pub reserved_nodes: Option<Vec<ReservedNode>>,
}

struct ReservedNodesMessageDeserializer;
impl ReservedNodesMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedNodesMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedNodesMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "ReservedNodes" => {
                        obj.reserved_nodes = Some(try!(ReservedNodeListDeserializer::deserialize(
                            "ReservedNodes",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResetClusterParameterGroupMessage {
    /// <p>The name of the cluster parameter group to be reset.</p>
    pub parameter_group_name: String,
    /// <p>An array of names of parameters to be reset. If <i>ResetAllParameters</i> option is not used, then at least one parameter name must be supplied. </p> <p>Constraints: A maximum of 20 parameters can be reset in a single request.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>If <code>true</code>, all parameters in the specified parameter group will be reset to their default values. </p> <p>Default: <code>true</code> </p>
    pub reset_all_parameters: Option<bool>,
}

/// Serialize `ResetClusterParameterGroupMessage` contents to a `SignedRequest`.
struct ResetClusterParameterGroupMessageSerializer;
impl ResetClusterParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ResetClusterParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ParameterGroupName"),
            &obj.parameter_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.parameters {
            ParametersListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameter"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.reset_all_parameters {
            params.put(
                &format!("{}{}", prefix, "ResetAllParameters"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>Describes the result of a cluster resize operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResizeProgressMessage {
    /// <p>The average rate of the resize operation over the last few minutes, measured in megabytes per second. After the resize operation completes, this value shows the average rate of the entire resize operation.</p>
    pub avg_resize_rate_in_mega_bytes_per_second: Option<f64>,
    /// <p>The amount of seconds that have elapsed since the resize operation began. After the resize operation completes, this value shows the total actual time, in seconds, for the resize operation.</p>
    pub elapsed_time_in_seconds: Option<i64>,
    /// <p>The estimated time remaining, in seconds, until the resize operation is complete. This value is calculated based on the average resize rate and the estimated amount of data remaining to be processed. Once the resize operation is complete, this value will be 0.</p>
    pub estimated_time_to_completion_in_seconds: Option<i64>,
    /// <p>The names of tables that have been completely imported .</p> <p>Valid Values: List of table names.</p>
    pub import_tables_completed: Option<Vec<String>>,
    /// <p>The names of tables that are being currently imported.</p> <p>Valid Values: List of table names.</p>
    pub import_tables_in_progress: Option<Vec<String>>,
    /// <p>The names of tables that have not been yet imported.</p> <p>Valid Values: List of table names</p>
    pub import_tables_not_started: Option<Vec<String>>,
    /// <p>While the resize operation is in progress, this value shows the current amount of data, in megabytes, that has been processed so far. When the resize operation is complete, this value shows the total amount of data, in megabytes, on the cluster, which may be more or less than TotalResizeDataInMegaBytes (the estimated total amount of data before resize).</p>
    pub progress_in_mega_bytes: Option<i64>,
    /// <p>The status of the resize operation.</p> <p>Valid Values: <code>NONE</code> | <code>IN_PROGRESS</code> | <code>FAILED</code> | <code>SUCCEEDED</code> </p>
    pub status: Option<String>,
    /// <p>The cluster type after the resize operation is complete.</p> <p>Valid Values: <code>multi-node</code> | <code>single-node</code> </p>
    pub target_cluster_type: Option<String>,
    /// <p>The node type that the cluster will have after the resize operation is complete.</p>
    pub target_node_type: Option<String>,
    /// <p>The number of nodes that the cluster will have after the resize operation is complete.</p>
    pub target_number_of_nodes: Option<i64>,
    /// <p>The estimated total amount of data, in megabytes, on the cluster before the resize operation began.</p>
    pub total_resize_data_in_mega_bytes: Option<i64>,
}

struct ResizeProgressMessageDeserializer;
impl ResizeProgressMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResizeProgressMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResizeProgressMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AvgResizeRateInMegaBytesPerSecond" => {
                        obj.avg_resize_rate_in_mega_bytes_per_second =
                            Some(try!(DoubleOptionalDeserializer::deserialize(
                                "AvgResizeRateInMegaBytesPerSecond",
                                stack
                            )));
                    }
                    "ElapsedTimeInSeconds" => {
                        obj.elapsed_time_in_seconds = Some(try!(
                            LongOptionalDeserializer::deserialize("ElapsedTimeInSeconds", stack)
                        ));
                    }
                    "EstimatedTimeToCompletionInSeconds" => {
                        obj.estimated_time_to_completion_in_seconds =
                            Some(try!(LongOptionalDeserializer::deserialize(
                                "EstimatedTimeToCompletionInSeconds",
                                stack
                            )));
                    }
                    "ImportTablesCompleted" => {
                        obj.import_tables_completed =
                            Some(try!(ImportTablesCompletedDeserializer::deserialize(
                                "ImportTablesCompleted",
                                stack
                            )));
                    }
                    "ImportTablesInProgress" => {
                        obj.import_tables_in_progress =
                            Some(try!(ImportTablesInProgressDeserializer::deserialize(
                                "ImportTablesInProgress",
                                stack
                            )));
                    }
                    "ImportTablesNotStarted" => {
                        obj.import_tables_not_started =
                            Some(try!(ImportTablesNotStartedDeserializer::deserialize(
                                "ImportTablesNotStarted",
                                stack
                            )));
                    }
                    "ProgressInMegaBytes" => {
                        obj.progress_in_mega_bytes = Some(try!(
                            LongOptionalDeserializer::deserialize("ProgressInMegaBytes", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "TargetClusterType" => {
                        obj.target_cluster_type = Some(try!(StringDeserializer::deserialize(
                            "TargetClusterType",
                            stack
                        )));
                    }
                    "TargetNodeType" => {
                        obj.target_node_type = Some(try!(StringDeserializer::deserialize(
                            "TargetNodeType",
                            stack
                        )));
                    }
                    "TargetNumberOfNodes" => {
                        obj.target_number_of_nodes = Some(try!(
                            IntegerOptionalDeserializer::deserialize("TargetNumberOfNodes", stack)
                        ));
                    }
                    "TotalResizeDataInMegaBytes" => {
                        obj.total_resize_data_in_mega_bytes =
                            Some(try!(LongOptionalDeserializer::deserialize(
                                "TotalResizeDataInMegaBytes",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct RestorableNodeTypeListDeserializer;
impl RestorableNodeTypeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "NodeType" {
                        obj.push(try!(StringDeserializer::deserialize("NodeType", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreFromClusterSnapshotMessage {
    /// <p>Reserved.</p>
    pub additional_info: Option<String>,
    /// <p>If <code>true</code>, major version upgrades can be applied during the maintenance window to the Amazon Redshift engine that is running on the cluster. </p> <p>Default: <code>true</code> </p>
    pub allow_version_upgrade: Option<bool>,
    /// <p>The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Even if automated snapshots are disabled, you can still create manual snapshots when you want with <a>CreateClusterSnapshot</a>. </p> <p>Default: The value selected for the cluster from which the snapshot was taken.</p> <p>Constraints: Must be a value from 0 to 35.</p>
    pub automated_snapshot_retention_period: Option<i64>,
    /// <p>The Amazon EC2 Availability Zone in which to restore the cluster.</p> <p>Default: A random, system-chosen Availability Zone.</p> <p>Example: <code>us-east-1a</code> </p>
    pub availability_zone: Option<String>,
    /// <p><p>The identifier of the cluster that will be created from restoring the snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>Alphabetic characters must be lowercase.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> <li> <p>Must be unique for all clusters within an AWS account.</p> </li> </ul></p>
    pub cluster_identifier: String,
    /// <p><p>The name of the parameter group to be associated with this cluster.</p> <p>Default: The default Amazon Redshift cluster parameter group. For information about the default parameter group, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Working with Amazon Redshift Parameter Groups</a>.</p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub cluster_parameter_group_name: Option<String>,
    /// <p>A list of security groups to be associated with this cluster.</p> <p>Default: The default cluster security group for Amazon Redshift.</p> <p>Cluster security groups only apply to clusters outside of VPCs.</p>
    pub cluster_security_groups: Option<Vec<String>>,
    /// <p>The name of the subnet group where you want to cluster restored.</p> <p>A snapshot of cluster in VPC can be restored only in VPC. Therefore, you must provide subnet group name where you want the cluster restored.</p>
    pub cluster_subnet_group_name: Option<String>,
    /// <p>The elastic IP (EIP) address for the cluster.</p>
    pub elastic_ip: Option<String>,
    /// <p>An option that specifies whether to create the cluster with enhanced VPC routing enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a VPC. For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/enhanced-vpc-routing.html">Enhanced VPC Routing</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If this option is <code>true</code>, enhanced VPC routing is enabled. </p> <p>Default: false</p>
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>Specifies the name of the HSM client certificate the Amazon Redshift cluster uses to retrieve the data encryption keys stored in an HSM.</p>
    pub hsm_client_certificate_identifier: Option<String>,
    /// <p>Specifies the name of the HSM configuration that contains the information the Amazon Redshift cluster can use to retrieve and store keys in an HSM.</p>
    pub hsm_configuration_identifier: Option<String>,
    /// <p>A list of AWS Identity and Access Management (IAM) roles that can be used by the cluster to access other AWS services. You must supply the IAM roles in their Amazon Resource Name (ARN) format. You can supply up to 10 IAM roles in a single request.</p> <p>A cluster can have up to 10 IAM roles associated at any time.</p>
    pub iam_roles: Option<Vec<String>>,
    /// <p>The AWS Key Management Service (KMS) key ID of the encryption key that you want to use to encrypt data in the cluster that you restore from a shared snapshot.</p>
    pub kms_key_id: Option<String>,
    /// <p>The node type that the restored cluster will be provisioned with.</p> <p>Default: The node type of the cluster from which the snapshot was taken. You can modify this if you are using any DS node type. In that case, you can choose to restore into another DS node type of the same size. For example, you can restore ds1.8xlarge into ds2.8xlarge, or ds1.xlarge into ds2.xlarge. If you have a DC instance type, you must restore into that same instance type and size. In other words, you can only restore a dc1.large instance type into another dc1.large instance type or dc2.large instance type. You can't restore dc1.8xlarge to dc2.8xlarge. First restore to a dc1.8xlareg cluster, then resize to a dc2.8large cluster. For more information about node types, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#rs-about-clusters-and-nodes"> About Clusters and Nodes</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    pub node_type: Option<String>,
    /// <p>The AWS customer account used to create or copy the snapshot. Required if you are restoring a snapshot you do not own, optional if you own the snapshot.</p>
    pub owner_account: Option<String>,
    /// <p>The port number on which the cluster accepts connections.</p> <p>Default: The same port as the original cluster.</p> <p>Constraints: Must be between <code>1115</code> and <code>65535</code>.</p>
    pub port: Option<i64>,
    /// <p>The weekly time range (in UTC) during which automated cluster maintenance can occur.</p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p> Default: The value selected for the cluster from which the snapshot was taken. For more information about the time blocks for each region, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html#rs-maintenance-windows">Maintenance Windows</a> in Amazon Redshift Cluster Management Guide. </p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Minimum 30-minute window.</p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>If <code>true</code>, the cluster can be accessed from a public network. </p>
    pub publicly_accessible: Option<bool>,
    /// <p>The name of the cluster the source snapshot was created from. This parameter is required if your IAM user has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.</p>
    pub snapshot_cluster_identifier: Option<String>,
    /// <p>The name of the snapshot from which to create the new cluster. This parameter isn't case sensitive.</p> <p>Example: <code>my-snapshot-id</code> </p>
    pub snapshot_identifier: String,
    /// <p>A list of Virtual Private Cloud (VPC) security groups to be associated with the cluster.</p> <p>Default: The default VPC security group is associated with the cluster.</p> <p>VPC security groups only apply to clusters in VPCs.</p>
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// Serialize `RestoreFromClusterSnapshotMessage` contents to a `SignedRequest`.
struct RestoreFromClusterSnapshotMessageSerializer;
impl RestoreFromClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RestoreFromClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.additional_info {
            params.put(
                &format!("{}{}", prefix, "AdditionalInfo"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.allow_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AllowVersionUpgrade"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.automated_snapshot_retention_period {
            params.put(
                &format!("{}{}", prefix, "AutomatedSnapshotRetentionPeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.availability_zone {
            params.put(
                &format!("{}{}", prefix, "AvailabilityZone"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.cluster_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "ClusterParameterGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.cluster_security_groups {
            ClusterSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ClusterSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cluster_subnet_group_name {
            params.put(
                &format!("{}{}", prefix, "ClusterSubnetGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.elastic_ip {
            params.put(
                &format!("{}{}", prefix, "ElasticIp"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.enhanced_vpc_routing {
            params.put(
                &format!("{}{}", prefix, "EnhancedVpcRouting"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.hsm_client_certificate_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmClientCertificateIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.hsm_configuration_identifier {
            params.put(
                &format!("{}{}", prefix, "HsmConfigurationIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.iam_roles {
            IamRoleArnListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "IamRoleArn"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(
                &format!("{}{}", prefix, "KmsKeyId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.node_type {
            params.put(
                &format!("{}{}", prefix, "NodeType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.owner_account {
            params.put(
                &format!("{}{}", prefix, "OwnerAccount"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.port {
            params.put(
                &format!("{}{}", prefix, "Port"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.publicly_accessible {
            params.put(
                &format!("{}{}", prefix, "PubliclyAccessible"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.snapshot_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "SnapshotClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier.replace("+", "%2B"),
        );
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
pub struct RestoreFromClusterSnapshotResult {
    pub cluster: Option<Cluster>,
}

struct RestoreFromClusterSnapshotResultDeserializer;
impl RestoreFromClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreFromClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RestoreFromClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the status of a cluster restore action. Returns null if the cluster was not created by restoring a snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreStatus {
    /// <p>The number of megabytes per second being transferred from the backup storage. Returns the average rate for a completed backup.</p>
    pub current_restore_rate_in_mega_bytes_per_second: Option<f64>,
    /// <p>The amount of time an in-progress restore has been running, or the amount of time it took a completed restore to finish.</p>
    pub elapsed_time_in_seconds: Option<i64>,
    /// <p>The estimate of the time remaining before the restore will complete. Returns 0 for a completed restore.</p>
    pub estimated_time_to_completion_in_seconds: Option<i64>,
    /// <p>The number of megabytes that have been transferred from snapshot storage.</p>
    pub progress_in_mega_bytes: Option<i64>,
    /// <p>The size of the set of snapshot data used to restore the cluster.</p>
    pub snapshot_size_in_mega_bytes: Option<i64>,
    /// <p>The status of the restore action. Returns starting, restoring, completed, or failed.</p>
    pub status: Option<String>,
}

struct RestoreStatusDeserializer;
impl RestoreStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RestoreStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CurrentRestoreRateInMegaBytesPerSecond" => {
                        obj.current_restore_rate_in_mega_bytes_per_second =
                            Some(try!(DoubleDeserializer::deserialize(
                                "CurrentRestoreRateInMegaBytesPerSecond",
                                stack
                            )));
                    }
                    "ElapsedTimeInSeconds" => {
                        obj.elapsed_time_in_seconds = Some(try!(LongDeserializer::deserialize(
                            "ElapsedTimeInSeconds",
                            stack
                        )));
                    }
                    "EstimatedTimeToCompletionInSeconds" => {
                        obj.estimated_time_to_completion_in_seconds =
                            Some(try!(LongDeserializer::deserialize(
                                "EstimatedTimeToCompletionInSeconds",
                                stack
                            )));
                    }
                    "ProgressInMegaBytes" => {
                        obj.progress_in_mega_bytes = Some(try!(LongDeserializer::deserialize(
                            "ProgressInMegaBytes",
                            stack
                        )));
                    }
                    "SnapshotSizeInMegaBytes" => {
                        obj.snapshot_size_in_mega_bytes = Some(try!(
                            LongDeserializer::deserialize("SnapshotSizeInMegaBytes", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreTableFromClusterSnapshotMessage {
    /// <p>The identifier of the Amazon Redshift cluster to restore the table to.</p>
    pub cluster_identifier: String,
    /// <p>The name of the table to create as a result of the current request.</p>
    pub new_table_name: String,
    /// <p>The identifier of the snapshot to restore the table from. This snapshot must have been created from the Amazon Redshift cluster specified by the <code>ClusterIdentifier</code> parameter.</p>
    pub snapshot_identifier: String,
    /// <p>The name of the source database that contains the table to restore from.</p>
    pub source_database_name: String,
    /// <p>The name of the source schema that contains the table to restore from. If you do not specify a <code>SourceSchemaName</code> value, the default is <code>public</code>.</p>
    pub source_schema_name: Option<String>,
    /// <p>The name of the source table to restore from.</p>
    pub source_table_name: String,
    /// <p>The name of the database to restore the table to.</p>
    pub target_database_name: Option<String>,
    /// <p>The name of the schema to restore the table to.</p>
    pub target_schema_name: Option<String>,
}

/// Serialize `RestoreTableFromClusterSnapshotMessage` contents to a `SignedRequest`.
struct RestoreTableFromClusterSnapshotMessageSerializer;
impl RestoreTableFromClusterSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RestoreTableFromClusterSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "NewTableName"),
            &obj.new_table_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SourceDatabaseName"),
            &obj.source_database_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.source_schema_name {
            params.put(
                &format!("{}{}", prefix, "SourceSchemaName"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SourceTableName"),
            &obj.source_table_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.target_database_name {
            params.put(
                &format!("{}{}", prefix, "TargetDatabaseName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.target_schema_name {
            params.put(
                &format!("{}{}", prefix, "TargetSchemaName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreTableFromClusterSnapshotResult {
    pub table_restore_status: Option<TableRestoreStatus>,
}

struct RestoreTableFromClusterSnapshotResultDeserializer;
impl RestoreTableFromClusterSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreTableFromClusterSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RestoreTableFromClusterSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "TableRestoreStatus" => {
                        obj.table_restore_status =
                            Some(try!(TableRestoreStatusDeserializer::deserialize(
                                "TableRestoreStatus",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RevokeClusterSecurityGroupIngressMessage {
    /// <p>The IP range for which to revoke access. This range must be a valid Classless Inter-Domain Routing (CIDR) block of IP addresses. If <code>CIDRIP</code> is specified, <code>EC2SecurityGroupName</code> and <code>EC2SecurityGroupOwnerId</code> cannot be provided. </p>
    pub cidrip: Option<String>,
    /// <p>The name of the security Group from which to revoke the ingress rule.</p>
    pub cluster_security_group_name: String,
    /// <p>The name of the EC2 Security Group whose access is to be revoked. If <code>EC2SecurityGroupName</code> is specified, <code>EC2SecurityGroupOwnerId</code> must also be provided and <code>CIDRIP</code> cannot be provided. </p>
    pub ec2_security_group_name: Option<String>,
    /// <p>The AWS account number of the owner of the security group specified in the <code>EC2SecurityGroupName</code> parameter. The AWS access key ID is not an acceptable value. If <code>EC2SecurityGroupOwnerId</code> is specified, <code>EC2SecurityGroupName</code> must also be provided. and <code>CIDRIP</code> cannot be provided. </p> <p>Example: <code>111122223333</code> </p>
    pub ec2_security_group_owner_id: Option<String>,
}

/// Serialize `RevokeClusterSecurityGroupIngressMessage` contents to a `SignedRequest`.
struct RevokeClusterSecurityGroupIngressMessageSerializer;
impl RevokeClusterSecurityGroupIngressMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RevokeClusterSecurityGroupIngressMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cidrip {
            params.put(
                &format!("{}{}", prefix, "CIDRIP"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ClusterSecurityGroupName"),
            &obj.cluster_security_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.ec2_security_group_name {
            params.put(
                &format!("{}{}", prefix, "EC2SecurityGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.ec2_security_group_owner_id {
            params.put(
                &format!("{}{}", prefix, "EC2SecurityGroupOwnerId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RevokeClusterSecurityGroupIngressResult {
    pub cluster_security_group: Option<ClusterSecurityGroup>,
}

struct RevokeClusterSecurityGroupIngressResultDeserializer;
impl RevokeClusterSecurityGroupIngressResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RevokeClusterSecurityGroupIngressResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RevokeClusterSecurityGroupIngressResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterSecurityGroup" => {
                        obj.cluster_security_group =
                            Some(try!(ClusterSecurityGroupDeserializer::deserialize(
                                "ClusterSecurityGroup",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RevokeSnapshotAccessMessage {
    /// <p>The identifier of the AWS customer account that can no longer restore the specified snapshot.</p>
    pub account_with_restore_access: String,
    /// <p>The identifier of the cluster the snapshot was created from. This parameter is required if your IAM user has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.</p>
    pub snapshot_cluster_identifier: Option<String>,
    /// <p>The identifier of the snapshot that the account can no longer access.</p>
    pub snapshot_identifier: String,
}

/// Serialize `RevokeSnapshotAccessMessage` contents to a `SignedRequest`.
struct RevokeSnapshotAccessMessageSerializer;
impl RevokeSnapshotAccessMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RevokeSnapshotAccessMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AccountWithRestoreAccess"),
            &obj.account_with_restore_access.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.snapshot_cluster_identifier {
            params.put(
                &format!("{}{}", prefix, "SnapshotClusterIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SnapshotIdentifier"),
            &obj.snapshot_identifier.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RevokeSnapshotAccessResult {
    pub snapshot: Option<Snapshot>,
}

struct RevokeSnapshotAccessResultDeserializer;
impl RevokeSnapshotAccessResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RevokeSnapshotAccessResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RevokeSnapshotAccessResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Snapshot" => {
                        obj.snapshot =
                            Some(try!(SnapshotDeserializer::deserialize("Snapshot", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RotateEncryptionKeyMessage {
    /// <p>The unique identifier of the cluster that you want to rotate the encryption keys for.</p> <p>Constraints: Must be the name of valid cluster that has encryption enabled.</p>
    pub cluster_identifier: String,
}

/// Serialize `RotateEncryptionKeyMessage` contents to a `SignedRequest`.
struct RotateEncryptionKeyMessageSerializer;
impl RotateEncryptionKeyMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RotateEncryptionKeyMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ClusterIdentifier"),
            &obj.cluster_identifier.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RotateEncryptionKeyResult {
    pub cluster: Option<Cluster>,
}

struct RotateEncryptionKeyResultDeserializer;
impl RotateEncryptionKeyResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RotateEncryptionKeyResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RotateEncryptionKeyResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cluster" => {
                        obj.cluster =
                            Some(try!(ClusterDeserializer::deserialize("Cluster", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SensitiveStringDeserializer;
impl SensitiveStringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Snapshot {
    /// <p>A list of the AWS customer accounts authorized to restore the snapshot. Returns <code>null</code> if no accounts are authorized. Visible only to the snapshot owner. </p>
    pub accounts_with_restore_access: Option<Vec<AccountWithRestoreAccess>>,
    /// <p>The size of the incremental backup.</p>
    pub actual_incremental_backup_size_in_mega_bytes: Option<f64>,
    /// <p>The Availability Zone in which the cluster was created.</p>
    pub availability_zone: Option<String>,
    /// <p>The number of megabytes that have been transferred to the snapshot backup.</p>
    pub backup_progress_in_mega_bytes: Option<f64>,
    /// <p>The time (UTC) when the cluster was originally created.</p>
    pub cluster_create_time: Option<String>,
    /// <p>The identifier of the cluster for which the snapshot was taken.</p>
    pub cluster_identifier: Option<String>,
    /// <p>The version ID of the Amazon Redshift engine that is running on the cluster.</p>
    pub cluster_version: Option<String>,
    /// <p>The number of megabytes per second being transferred to the snapshot backup. Returns <code>0</code> for a completed backup. </p>
    pub current_backup_rate_in_mega_bytes_per_second: Option<f64>,
    /// <p>The name of the database that was created when the cluster was created.</p>
    pub db_name: Option<String>,
    /// <p>The amount of time an in-progress snapshot backup has been running, or the amount of time it took a completed backup to finish.</p>
    pub elapsed_time_in_seconds: Option<i64>,
    /// <p>If <code>true</code>, the data in the snapshot is encrypted at rest.</p>
    pub encrypted: Option<bool>,
    /// <p>A boolean that indicates whether the snapshot data is encrypted using the HSM keys of the source cluster. <code>true</code> indicates that the data is encrypted using HSM keys.</p>
    pub encrypted_with_hsm: Option<bool>,
    /// <p>An option that specifies whether to create the cluster with enhanced VPC routing enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a VPC. For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/enhanced-vpc-routing.html">Enhanced VPC Routing</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If this option is <code>true</code>, enhanced VPC routing is enabled. </p> <p>Default: false</p>
    pub enhanced_vpc_routing: Option<bool>,
    /// <p>The estimate of the time remaining before the snapshot backup will complete. Returns <code>0</code> for a completed backup. </p>
    pub estimated_seconds_to_completion: Option<i64>,
    /// <p>The AWS Key Management Service (KMS) key ID of the encryption key that was used to encrypt data in the cluster from which the snapshot was taken.</p>
    pub kms_key_id: Option<String>,
    /// <p>The master user name for the cluster.</p>
    pub master_username: Option<String>,
    /// <p>The node type of the nodes in the cluster.</p>
    pub node_type: Option<String>,
    /// <p>The number of nodes in the cluster.</p>
    pub number_of_nodes: Option<i64>,
    /// <p>For manual snapshots, the AWS customer account used to create or copy the snapshot. For automatic snapshots, the owner of the cluster. The owner can perform all snapshot actions, such as sharing a manual snapshot.</p>
    pub owner_account: Option<String>,
    /// <p>The port that the cluster is listening on.</p>
    pub port: Option<i64>,
    /// <p>The list of node types that this cluster snapshot is able to restore into.</p>
    pub restorable_node_types: Option<Vec<String>>,
    /// <p>The time (UTC) when Amazon Redshift began the snapshot. A snapshot contains a copy of the cluster data as of this exact time.</p>
    pub snapshot_create_time: Option<String>,
    /// <p>The snapshot identifier that is provided in the request.</p>
    pub snapshot_identifier: Option<String>,
    /// <p>The snapshot type. Snapshots created using <a>CreateClusterSnapshot</a> and <a>CopyClusterSnapshot</a> will be of type "manual". </p>
    pub snapshot_type: Option<String>,
    /// <p>The source region from which the snapshot was copied.</p>
    pub source_region: Option<String>,
    /// <p><p>The snapshot status. The value of the status depends on the API operation used. </p> <ul> <li> <p> <a>CreateClusterSnapshot</a> and <a>CopyClusterSnapshot</a> returns status as &quot;creating&quot;. </p> </li> <li> <p> <a>DescribeClusterSnapshots</a> returns status as &quot;creating&quot;, &quot;available&quot;, &quot;final snapshot&quot;, or &quot;failed&quot;.</p> </li> <li> <p> <a>DeleteClusterSnapshot</a> returns status as &quot;deleted&quot;.</p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The list of tags for the cluster snapshot.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The size of the complete set of backup data that would be used to restore the cluster.</p>
    pub total_backup_size_in_mega_bytes: Option<f64>,
    /// <p>The VPC identifier of the cluster if the snapshot is from a cluster in a VPC. Otherwise, this field is not in the output.</p>
    pub vpc_id: Option<String>,
}

struct SnapshotDeserializer;
impl SnapshotDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Snapshot, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Snapshot::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AccountsWithRestoreAccess" => {
                        obj.accounts_with_restore_access = Some(try!(
                            AccountsWithRestoreAccessListDeserializer::deserialize(
                                "AccountsWithRestoreAccess",
                                stack
                            )
                        ));
                    }
                    "ActualIncrementalBackupSizeInMegaBytes" => {
                        obj.actual_incremental_backup_size_in_mega_bytes =
                            Some(try!(DoubleDeserializer::deserialize(
                                "ActualIncrementalBackupSizeInMegaBytes",
                                stack
                            )));
                    }
                    "AvailabilityZone" => {
                        obj.availability_zone = Some(try!(StringDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    }
                    "BackupProgressInMegaBytes" => {
                        obj.backup_progress_in_mega_bytes = Some(try!(
                            DoubleDeserializer::deserialize("BackupProgressInMegaBytes", stack)
                        ));
                    }
                    "ClusterCreateTime" => {
                        obj.cluster_create_time = Some(try!(TStampDeserializer::deserialize(
                            "ClusterCreateTime",
                            stack
                        )));
                    }
                    "ClusterIdentifier" => {
                        obj.cluster_identifier = Some(try!(StringDeserializer::deserialize(
                            "ClusterIdentifier",
                            stack
                        )));
                    }
                    "ClusterVersion" => {
                        obj.cluster_version = Some(try!(StringDeserializer::deserialize(
                            "ClusterVersion",
                            stack
                        )));
                    }
                    "CurrentBackupRateInMegaBytesPerSecond" => {
                        obj.current_backup_rate_in_mega_bytes_per_second =
                            Some(try!(DoubleDeserializer::deserialize(
                                "CurrentBackupRateInMegaBytesPerSecond",
                                stack
                            )));
                    }
                    "DBName" => {
                        obj.db_name = Some(try!(StringDeserializer::deserialize("DBName", stack)));
                    }
                    "ElapsedTimeInSeconds" => {
                        obj.elapsed_time_in_seconds = Some(try!(LongDeserializer::deserialize(
                            "ElapsedTimeInSeconds",
                            stack
                        )));
                    }
                    "Encrypted" => {
                        obj.encrypted =
                            Some(try!(BooleanDeserializer::deserialize("Encrypted", stack)));
                    }
                    "EncryptedWithHSM" => {
                        obj.encrypted_with_hsm = Some(try!(BooleanDeserializer::deserialize(
                            "EncryptedWithHSM",
                            stack
                        )));
                    }
                    "EnhancedVpcRouting" => {
                        obj.enhanced_vpc_routing = Some(try!(BooleanDeserializer::deserialize(
                            "EnhancedVpcRouting",
                            stack
                        )));
                    }
                    "EstimatedSecondsToCompletion" => {
                        obj.estimated_seconds_to_completion = Some(try!(
                            LongDeserializer::deserialize("EstimatedSecondsToCompletion", stack)
                        ));
                    }
                    "KmsKeyId" => {
                        obj.kms_key_id =
                            Some(try!(StringDeserializer::deserialize("KmsKeyId", stack)));
                    }
                    "MasterUsername" => {
                        obj.master_username = Some(try!(StringDeserializer::deserialize(
                            "MasterUsername",
                            stack
                        )));
                    }
                    "NodeType" => {
                        obj.node_type =
                            Some(try!(StringDeserializer::deserialize("NodeType", stack)));
                    }
                    "NumberOfNodes" => {
                        obj.number_of_nodes = Some(try!(IntegerDeserializer::deserialize(
                            "NumberOfNodes",
                            stack
                        )));
                    }
                    "OwnerAccount" => {
                        obj.owner_account =
                            Some(try!(StringDeserializer::deserialize("OwnerAccount", stack)));
                    }
                    "Port" => {
                        obj.port = Some(try!(IntegerDeserializer::deserialize("Port", stack)));
                    }
                    "RestorableNodeTypes" => {
                        obj.restorable_node_types =
                            Some(try!(RestorableNodeTypeListDeserializer::deserialize(
                                "RestorableNodeTypes",
                                stack
                            )));
                    }
                    "SnapshotCreateTime" => {
                        obj.snapshot_create_time = Some(try!(TStampDeserializer::deserialize(
                            "SnapshotCreateTime",
                            stack
                        )));
                    }
                    "SnapshotIdentifier" => {
                        obj.snapshot_identifier = Some(try!(StringDeserializer::deserialize(
                            "SnapshotIdentifier",
                            stack
                        )));
                    }
                    "SnapshotType" => {
                        obj.snapshot_type =
                            Some(try!(StringDeserializer::deserialize("SnapshotType", stack)));
                    }
                    "SourceRegion" => {
                        obj.source_region =
                            Some(try!(StringDeserializer::deserialize("SourceRegion", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    "TotalBackupSizeInMegaBytes" => {
                        obj.total_backup_size_in_mega_bytes = Some(try!(
                            DoubleDeserializer::deserialize("TotalBackupSizeInMegaBytes", stack)
                        ));
                    }
                    "VpcId" => {
                        obj.vpc_id = Some(try!(StringDeserializer::deserialize("VpcId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The snapshot copy grant that grants Amazon Redshift permission to encrypt copied snapshots with the specified customer master key (CMK) from AWS KMS in the destination region.</p> <p> For more information about managing snapshot copy grants, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-db-encryption.html">Amazon Redshift Database Encryption</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SnapshotCopyGrant {
    /// <p>The unique identifier of the customer master key (CMK) in AWS KMS to which Amazon Redshift is granted permission.</p>
    pub kms_key_id: Option<String>,
    /// <p>The name of the snapshot copy grant.</p>
    pub snapshot_copy_grant_name: Option<String>,
    /// <p>A list of tag instances.</p>
    pub tags: Option<Vec<Tag>>,
}

struct SnapshotCopyGrantDeserializer;
impl SnapshotCopyGrantDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SnapshotCopyGrant, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SnapshotCopyGrant::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "KmsKeyId" => {
                        obj.kms_key_id =
                            Some(try!(StringDeserializer::deserialize("KmsKeyId", stack)));
                    }
                    "SnapshotCopyGrantName" => {
                        obj.snapshot_copy_grant_name = Some(try!(StringDeserializer::deserialize(
                            "SnapshotCopyGrantName",
                            stack
                        )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SnapshotCopyGrantListDeserializer;
impl SnapshotCopyGrantListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SnapshotCopyGrant>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "SnapshotCopyGrant" {
                        obj.push(try!(SnapshotCopyGrantDeserializer::deserialize(
                            "SnapshotCopyGrant",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SnapshotCopyGrantMessage {
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeSnapshotCopyGrant</code> request exceed the value specified in <code>MaxRecords</code>, AWS returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p> <p>Constraints: You can specify either the <b>SnapshotCopyGrantName</b> parameter or the <b>Marker</b> parameter, but not both. </p>
    pub marker: Option<String>,
    /// <p>The list of <code>SnapshotCopyGrant</code> objects.</p>
    pub snapshot_copy_grants: Option<Vec<SnapshotCopyGrant>>,
}

struct SnapshotCopyGrantMessageDeserializer;
impl SnapshotCopyGrantMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SnapshotCopyGrantMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SnapshotCopyGrantMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "SnapshotCopyGrants" => {
                        obj.snapshot_copy_grants =
                            Some(try!(SnapshotCopyGrantListDeserializer::deserialize(
                                "SnapshotCopyGrants",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SnapshotListDeserializer;
impl SnapshotListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Snapshot>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Snapshot" {
                        obj.push(try!(SnapshotDeserializer::deserialize("Snapshot", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Contains the output from the <a>DescribeClusterSnapshots</a> action. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SnapshotMessage {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>A list of <a>Snapshot</a> instances. </p>
    pub snapshots: Option<Vec<Snapshot>>,
}

struct SnapshotMessageDeserializer;
impl SnapshotMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SnapshotMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SnapshotMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "Snapshots" => {
                        obj.snapshots = Some(try!(SnapshotListDeserializer::deserialize(
                            "Snapshots",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SourceIdsListDeserializer;
impl SourceIdsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "SourceId" {
                        obj.push(try!(StringDeserializer::deserialize("SourceId", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a subnet.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Subnet {
    pub subnet_availability_zone: Option<AvailabilityZone>,
    /// <p>The identifier of the subnet.</p>
    pub subnet_identifier: Option<String>,
    /// <p>The status of the subnet.</p>
    pub subnet_status: Option<String>,
}

struct SubnetDeserializer;
impl SubnetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Subnet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Subnet::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "SubnetAvailabilityZone" => {
                        obj.subnet_availability_zone =
                            Some(try!(AvailabilityZoneDeserializer::deserialize(
                                "SubnetAvailabilityZone",
                                stack
                            )));
                    }
                    "SubnetIdentifier" => {
                        obj.subnet_identifier = Some(try!(StringDeserializer::deserialize(
                            "SubnetIdentifier",
                            stack
                        )));
                    }
                    "SubnetStatus" => {
                        obj.subnet_status =
                            Some(try!(StringDeserializer::deserialize("SubnetStatus", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Subnet>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Subnet" {
                        obj.push(try!(SubnetDeserializer::deserialize("Subnet", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>A list of supported platforms for orderable clusters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SupportedPlatform {
    pub name: Option<String>,
}

struct SupportedPlatformDeserializer;
impl SupportedPlatformDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SupportedPlatform, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SupportedPlatform::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Name" => {
                        obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SupportedPlatformsListDeserializer;
impl SupportedPlatformsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SupportedPlatform>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "SupportedPlatform" {
                        obj.push(try!(SupportedPlatformDeserializer::deserialize(
                            "SupportedPlatform",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct TStampDeserializer;
impl TStampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes the status of a <a>RestoreTableFromClusterSnapshot</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TableRestoreStatus {
    /// <p>The identifier of the Amazon Redshift cluster that the table is being restored to.</p>
    pub cluster_identifier: Option<String>,
    /// <p>A description of the status of the table restore request. Status values include <code>SUCCEEDED</code>, <code>FAILED</code>, <code>CANCELED</code>, <code>PENDING</code>, <code>IN_PROGRESS</code>.</p>
    pub message: Option<String>,
    /// <p>The name of the table to create as a result of the table restore request.</p>
    pub new_table_name: Option<String>,
    /// <p>The amount of data restored to the new table so far, in megabytes (MB).</p>
    pub progress_in_mega_bytes: Option<i64>,
    /// <p>The time that the table restore request was made, in Universal Coordinated Time (UTC).</p>
    pub request_time: Option<String>,
    /// <p>The identifier of the snapshot that the table is being restored from.</p>
    pub snapshot_identifier: Option<String>,
    /// <p>The name of the source database that contains the table being restored.</p>
    pub source_database_name: Option<String>,
    /// <p>The name of the source schema that contains the table being restored.</p>
    pub source_schema_name: Option<String>,
    /// <p>The name of the source table being restored.</p>
    pub source_table_name: Option<String>,
    /// <p>A value that describes the current state of the table restore request.</p> <p>Valid Values: <code>SUCCEEDED</code>, <code>FAILED</code>, <code>CANCELED</code>, <code>PENDING</code>, <code>IN_PROGRESS</code> </p>
    pub status: Option<String>,
    /// <p>The unique identifier for the table restore request.</p>
    pub table_restore_request_id: Option<String>,
    /// <p>The name of the database to restore the table to.</p>
    pub target_database_name: Option<String>,
    /// <p>The name of the schema to restore the table to.</p>
    pub target_schema_name: Option<String>,
    /// <p>The total amount of data to restore to the new table, in megabytes (MB).</p>
    pub total_data_in_mega_bytes: Option<i64>,
}

struct TableRestoreStatusDeserializer;
impl TableRestoreStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TableRestoreStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TableRestoreStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ClusterIdentifier" => {
                        obj.cluster_identifier = Some(try!(StringDeserializer::deserialize(
                            "ClusterIdentifier",
                            stack
                        )));
                    }
                    "Message" => {
                        obj.message = Some(try!(StringDeserializer::deserialize("Message", stack)));
                    }
                    "NewTableName" => {
                        obj.new_table_name =
                            Some(try!(StringDeserializer::deserialize("NewTableName", stack)));
                    }
                    "ProgressInMegaBytes" => {
                        obj.progress_in_mega_bytes = Some(try!(
                            LongOptionalDeserializer::deserialize("ProgressInMegaBytes", stack)
                        ));
                    }
                    "RequestTime" => {
                        obj.request_time =
                            Some(try!(TStampDeserializer::deserialize("RequestTime", stack)));
                    }
                    "SnapshotIdentifier" => {
                        obj.snapshot_identifier = Some(try!(StringDeserializer::deserialize(
                            "SnapshotIdentifier",
                            stack
                        )));
                    }
                    "SourceDatabaseName" => {
                        obj.source_database_name = Some(try!(StringDeserializer::deserialize(
                            "SourceDatabaseName",
                            stack
                        )));
                    }
                    "SourceSchemaName" => {
                        obj.source_schema_name = Some(try!(StringDeserializer::deserialize(
                            "SourceSchemaName",
                            stack
                        )));
                    }
                    "SourceTableName" => {
                        obj.source_table_name = Some(try!(StringDeserializer::deserialize(
                            "SourceTableName",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(TableRestoreStatusTypeDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "TableRestoreRequestId" => {
                        obj.table_restore_request_id = Some(try!(StringDeserializer::deserialize(
                            "TableRestoreRequestId",
                            stack
                        )));
                    }
                    "TargetDatabaseName" => {
                        obj.target_database_name = Some(try!(StringDeserializer::deserialize(
                            "TargetDatabaseName",
                            stack
                        )));
                    }
                    "TargetSchemaName" => {
                        obj.target_schema_name = Some(try!(StringDeserializer::deserialize(
                            "TargetSchemaName",
                            stack
                        )));
                    }
                    "TotalDataInMegaBytes" => {
                        obj.total_data_in_mega_bytes = Some(try!(
                            LongOptionalDeserializer::deserialize("TotalDataInMegaBytes", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TableRestoreStatusListDeserializer;
impl TableRestoreStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TableRestoreStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "TableRestoreStatus" {
                        obj.push(try!(TableRestoreStatusDeserializer::deserialize(
                            "TableRestoreStatus",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TableRestoreStatusMessage {
    /// <p>A pagination token that can be used in a subsequent <a>DescribeTableRestoreStatus</a> request.</p>
    pub marker: Option<String>,
    /// <p>A list of status details for one or more table restore requests.</p>
    pub table_restore_status_details: Option<Vec<TableRestoreStatus>>,
}

struct TableRestoreStatusMessageDeserializer;
impl TableRestoreStatusMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TableRestoreStatusMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TableRestoreStatusMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "TableRestoreStatusDetails" => {
                        obj.table_restore_status_details =
                            Some(try!(TableRestoreStatusListDeserializer::deserialize(
                                "TableRestoreStatusDetails",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TableRestoreStatusTypeDeserializer;
impl TableRestoreStatusTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A tag consisting of a name/value pair for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>The key, or name, for the resource tag.</p>
    pub key: Option<String>,
    /// <p>The value for the resource tag.</p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Tag::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Key" => {
                        obj.key = Some(try!(StringDeserializer::deserialize("Key", stack)));
                    }
                    "Value" => {
                        obj.value = Some(try!(StringDeserializer::deserialize("Value", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            params.put(
                &format!("{}{}", prefix, "Key"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.value {
            params.put(
                &format!("{}{}", prefix, "Value"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `TagKeyList` contents to a `SignedRequest`.
struct TagKeyListSerializer;
impl TagKeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Tag" {
                        obj.push(try!(TagDeserializer::deserialize("Tag", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// Serialize `TagValueList` contents to a `SignedRequest`.
struct TagValueListSerializer;
impl TagValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>A tag and its associated resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TaggedResource {
    /// <p>The Amazon Resource Name (ARN) with which the tag is associated. For example, <code>arn:aws:redshift:us-east-1:123456789:cluster:t1</code>.</p>
    pub resource_name: Option<String>,
    /// <p>The type of resource with which the tag is associated. Valid resource types are: </p> <ul> <li> <p>Cluster</p> </li> <li> <p>CIDR/IP</p> </li> <li> <p>EC2 security group</p> </li> <li> <p>Snapshot</p> </li> <li> <p>Cluster security group</p> </li> <li> <p>Subnet group</p> </li> <li> <p>HSM connection</p> </li> <li> <p>HSM certificate</p> </li> <li> <p>Parameter group</p> </li> </ul> <p>For more information about Amazon Redshift resource types and constructing ARNs, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/redshift-iam-access-control-overview.html#redshift-iam-access-control-specify-actions">Constructing an Amazon Redshift Amazon Resource Name (ARN)</a> in the Amazon Redshift Cluster Management Guide. </p>
    pub resource_type: Option<String>,
    /// <p>The tag for the resource.</p>
    pub tag: Option<Tag>,
}

struct TaggedResourceDeserializer;
impl TaggedResourceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TaggedResource, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TaggedResource::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ResourceName" => {
                        obj.resource_name =
                            Some(try!(StringDeserializer::deserialize("ResourceName", stack)));
                    }
                    "ResourceType" => {
                        obj.resource_type =
                            Some(try!(StringDeserializer::deserialize("ResourceType", stack)));
                    }
                    "Tag" => {
                        obj.tag = Some(try!(TagDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TaggedResourceListDeserializer;
impl TaggedResourceListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TaggedResource>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "TaggedResource" {
                        obj.push(try!(TaggedResourceDeserializer::deserialize(
                            "TaggedResource",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TaggedResourceListMessage {
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned marker value in the <code>Marker</code> parameter and retrying the command. If the <code>Marker</code> field is empty, all response records have been retrieved for the request. </p>
    pub marker: Option<String>,
    /// <p>A list of tags with their associated resources.</p>
    pub tagged_resources: Option<Vec<TaggedResource>>,
}

struct TaggedResourceListMessageDeserializer;
impl TaggedResourceListMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TaggedResourceListMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TaggedResourceListMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Marker" => {
                        obj.marker = Some(try!(StringDeserializer::deserialize("Marker", stack)));
                    }
                    "TaggedResources" => {
                        obj.tagged_resources = Some(try!(
                            TaggedResourceListDeserializer::deserialize("TaggedResources", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

/// <p>Describes the members of a VPC security group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VpcSecurityGroupMembership {
    /// <p>The status of the VPC security group.</p>
    pub status: Option<String>,
    /// <p>The identifier of the VPC security group.</p>
    pub vpc_security_group_id: Option<String>,
}

struct VpcSecurityGroupMembershipDeserializer;
impl VpcSecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VpcSecurityGroupMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = VpcSecurityGroupMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "VpcSecurityGroupId" => {
                        obj.vpc_security_group_id = Some(try!(StringDeserializer::deserialize(
                            "VpcSecurityGroupId",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct VpcSecurityGroupMembershipListDeserializer;
impl VpcSecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<VpcSecurityGroupMembership>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "VpcSecurityGroup" {
                        obj.push(try!(VpcSecurityGroupMembershipDeserializer::deserialize(
                            "VpcSecurityGroup",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// Errors returned by AuthorizeClusterSecurityGroupIngress
#[derive(Debug, PartialEq)]
pub enum AuthorizeClusterSecurityGroupIngressError {
    /// <p>The specified CIDR block or EC2 security group is already authorized for the specified cluster security group.</p>
    AuthorizationAlreadyExistsFault(String),
    /// <p>The authorization quota for the cluster security group has been reached.</p>
    AuthorizationQuotaExceededFault(String),
    /// <p>The cluster security group name does not refer to an existing cluster security group.</p>
    ClusterSecurityGroupNotFoundFault(String),
    /// <p>The state of the cluster security group is not <code>available</code>. </p>
    InvalidClusterSecurityGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AuthorizeClusterSecurityGroupIngressError {
    pub fn from_body(body: &str) -> AuthorizeClusterSecurityGroupIngressError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationAlreadyExists" => {
                    AuthorizeClusterSecurityGroupIngressError::AuthorizationAlreadyExistsFault(
                        String::from(parsed_error.message),
                    )
                }
                "AuthorizationQuotaExceeded" => {
                    AuthorizeClusterSecurityGroupIngressError::AuthorizationQuotaExceededFault(
                        String::from(parsed_error.message),
                    )
                }
                "ClusterSecurityGroupNotFound" => {
                    AuthorizeClusterSecurityGroupIngressError::ClusterSecurityGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterSecurityGroupState" => {
                    AuthorizeClusterSecurityGroupIngressError::InvalidClusterSecurityGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => AuthorizeClusterSecurityGroupIngressError::Unknown(String::from(body)),
            },
            Err(_) => AuthorizeClusterSecurityGroupIngressError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for AuthorizeClusterSecurityGroupIngressError {
    fn from(err: XmlParseError) -> AuthorizeClusterSecurityGroupIngressError {
        let XmlParseError(message) = err;
        AuthorizeClusterSecurityGroupIngressError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AuthorizeClusterSecurityGroupIngressError {
    fn from(err: CredentialsError) -> AuthorizeClusterSecurityGroupIngressError {
        AuthorizeClusterSecurityGroupIngressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AuthorizeClusterSecurityGroupIngressError {
    fn from(err: HttpDispatchError) -> AuthorizeClusterSecurityGroupIngressError {
        AuthorizeClusterSecurityGroupIngressError::HttpDispatch(err)
    }
}
impl From<io::Error> for AuthorizeClusterSecurityGroupIngressError {
    fn from(err: io::Error) -> AuthorizeClusterSecurityGroupIngressError {
        AuthorizeClusterSecurityGroupIngressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AuthorizeClusterSecurityGroupIngressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AuthorizeClusterSecurityGroupIngressError {
    fn description(&self) -> &str {
        match *self {
            AuthorizeClusterSecurityGroupIngressError::AuthorizationAlreadyExistsFault(
                ref cause,
            ) => cause,
            AuthorizeClusterSecurityGroupIngressError::AuthorizationQuotaExceededFault(
                ref cause,
            ) => cause,
            AuthorizeClusterSecurityGroupIngressError::ClusterSecurityGroupNotFoundFault(
                ref cause,
            ) => cause,
            AuthorizeClusterSecurityGroupIngressError::InvalidClusterSecurityGroupStateFault(
                ref cause,
            ) => cause,
            AuthorizeClusterSecurityGroupIngressError::Validation(ref cause) => cause,
            AuthorizeClusterSecurityGroupIngressError::Credentials(ref err) => err.description(),
            AuthorizeClusterSecurityGroupIngressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AuthorizeClusterSecurityGroupIngressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AuthorizeSnapshotAccess
#[derive(Debug, PartialEq)]
pub enum AuthorizeSnapshotAccessError {
    /// <p>The specified CIDR block or EC2 security group is already authorized for the specified cluster security group.</p>
    AuthorizationAlreadyExistsFault(String),
    /// <p>The authorization quota for the cluster security group has been reached.</p>
    AuthorizationQuotaExceededFault(String),
    /// <p>The snapshot identifier does not refer to an existing cluster snapshot.</p>
    ClusterSnapshotNotFoundFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>The specified cluster snapshot is not in the <code>available</code> state, or other accounts are authorized to access the snapshot. </p>
    InvalidClusterSnapshotStateFault(String),
    /// <p>The encryption key has exceeded its grant limit in AWS KMS.</p>
    LimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AuthorizeSnapshotAccessError {
    pub fn from_body(body: &str) -> AuthorizeSnapshotAccessError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationAlreadyExists" => {
                    AuthorizeSnapshotAccessError::AuthorizationAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "AuthorizationQuotaExceeded" => {
                    AuthorizeSnapshotAccessError::AuthorizationQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSnapshotNotFound" => {
                    AuthorizeSnapshotAccessError::ClusterSnapshotNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "DependentServiceRequestThrottlingFault" => {
                    AuthorizeSnapshotAccessError::DependentServiceRequestThrottlingFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterSnapshotState" => {
                    AuthorizeSnapshotAccessError::InvalidClusterSnapshotStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "LimitExceededFault" => AuthorizeSnapshotAccessError::LimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => AuthorizeSnapshotAccessError::Unknown(String::from(body)),
            },
            Err(_) => AuthorizeSnapshotAccessError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for AuthorizeSnapshotAccessError {
    fn from(err: XmlParseError) -> AuthorizeSnapshotAccessError {
        let XmlParseError(message) = err;
        AuthorizeSnapshotAccessError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AuthorizeSnapshotAccessError {
    fn from(err: CredentialsError) -> AuthorizeSnapshotAccessError {
        AuthorizeSnapshotAccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AuthorizeSnapshotAccessError {
    fn from(err: HttpDispatchError) -> AuthorizeSnapshotAccessError {
        AuthorizeSnapshotAccessError::HttpDispatch(err)
    }
}
impl From<io::Error> for AuthorizeSnapshotAccessError {
    fn from(err: io::Error) -> AuthorizeSnapshotAccessError {
        AuthorizeSnapshotAccessError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AuthorizeSnapshotAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AuthorizeSnapshotAccessError {
    fn description(&self) -> &str {
        match *self {
            AuthorizeSnapshotAccessError::AuthorizationAlreadyExistsFault(ref cause) => cause,
            AuthorizeSnapshotAccessError::AuthorizationQuotaExceededFault(ref cause) => cause,
            AuthorizeSnapshotAccessError::ClusterSnapshotNotFoundFault(ref cause) => cause,
            AuthorizeSnapshotAccessError::DependentServiceRequestThrottlingFault(ref cause) => {
                cause
            }
            AuthorizeSnapshotAccessError::InvalidClusterSnapshotStateFault(ref cause) => cause,
            AuthorizeSnapshotAccessError::LimitExceededFault(ref cause) => cause,
            AuthorizeSnapshotAccessError::Validation(ref cause) => cause,
            AuthorizeSnapshotAccessError::Credentials(ref err) => err.description(),
            AuthorizeSnapshotAccessError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AuthorizeSnapshotAccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CopyClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum CopyClusterSnapshotError {
    /// <p>The value specified as a snapshot identifier is already used by an existing snapshot.</p>
    ClusterSnapshotAlreadyExistsFault(String),
    /// <p>The snapshot identifier does not refer to an existing cluster snapshot.</p>
    ClusterSnapshotNotFoundFault(String),
    /// <p>The request would result in the user exceeding the allowed number of cluster snapshots.</p>
    ClusterSnapshotQuotaExceededFault(String),
    /// <p>The specified cluster snapshot is not in the <code>available</code> state, or other accounts are authorized to access the snapshot. </p>
    InvalidClusterSnapshotStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CopyClusterSnapshotError {
    pub fn from_body(body: &str) -> CopyClusterSnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSnapshotAlreadyExists" => {
                    CopyClusterSnapshotError::ClusterSnapshotAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSnapshotNotFound" => {
                    CopyClusterSnapshotError::ClusterSnapshotNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSnapshotQuotaExceeded" => {
                    CopyClusterSnapshotError::ClusterSnapshotQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterSnapshotState" => {
                    CopyClusterSnapshotError::InvalidClusterSnapshotStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CopyClusterSnapshotError::Unknown(String::from(body)),
            },
            Err(_) => CopyClusterSnapshotError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CopyClusterSnapshotError {
    fn from(err: XmlParseError) -> CopyClusterSnapshotError {
        let XmlParseError(message) = err;
        CopyClusterSnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CopyClusterSnapshotError {
    fn from(err: CredentialsError) -> CopyClusterSnapshotError {
        CopyClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyClusterSnapshotError {
    fn from(err: HttpDispatchError) -> CopyClusterSnapshotError {
        CopyClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyClusterSnapshotError {
    fn from(err: io::Error) -> CopyClusterSnapshotError {
        CopyClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CopyClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CopyClusterSnapshotError::ClusterSnapshotAlreadyExistsFault(ref cause) => cause,
            CopyClusterSnapshotError::ClusterSnapshotNotFoundFault(ref cause) => cause,
            CopyClusterSnapshotError::ClusterSnapshotQuotaExceededFault(ref cause) => cause,
            CopyClusterSnapshotError::InvalidClusterSnapshotStateFault(ref cause) => cause,
            CopyClusterSnapshotError::Validation(ref cause) => cause,
            CopyClusterSnapshotError::Credentials(ref err) => err.description(),
            CopyClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CopyClusterSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>The account already has a cluster with the given identifier.</p>
    ClusterAlreadyExistsFault(String),
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// <p>The request would exceed the allowed number of cluster instances for this account. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterQuotaExceededFault(String),
    /// <p>The cluster security group name does not refer to an existing cluster security group.</p>
    ClusterSecurityGroupNotFoundFault(String),
    /// <p>The cluster subnet group name does not refer to an existing cluster subnet group.</p>
    ClusterSubnetGroupNotFoundFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>There is no Amazon Redshift HSM client certificate with the specified identifier.</p>
    HsmClientCertificateNotFoundFault(String),
    /// <p>There is no Amazon Redshift HSM configuration with the specified identifier.</p>
    HsmConfigurationNotFoundFault(String),
    /// <p>The number of nodes specified exceeds the allotted capacity of the cluster.</p>
    InsufficientClusterCapacityFault(String),
    /// <p>The cluster subnet group cannot be deleted because it is in use.</p>
    InvalidClusterSubnetGroupStateFault(String),
    /// <p>The Elastic IP (EIP) is invalid or cannot be found.</p>
    InvalidElasticIpFault(String),
    /// <p>The requested subnet is not valid, or not all of the subnets are in the same VPC.</p>
    InvalidSubnet(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The cluster subnet group does not cover all Availability Zones.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The encryption key has exceeded its grant limit in AWS KMS.</p>
    LimitExceededFault(String),
    /// <p>The operation would exceed the number of nodes allowed for a cluster.</p>
    NumberOfNodesPerClusterLimitExceededFault(String),
    /// <p>The operation would exceed the number of nodes allotted to the account. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    NumberOfNodesQuotaExceededFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateClusterError {
    pub fn from_body(body: &str) -> CreateClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterAlreadyExists" => CreateClusterError::ClusterAlreadyExistsFault(
                    String::from(parsed_error.message),
                ),
                "ClusterParameterGroupNotFound" => {
                    CreateClusterError::ClusterParameterGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterQuotaExceeded" => CreateClusterError::ClusterQuotaExceededFault(
                    String::from(parsed_error.message),
                ),
                "ClusterSecurityGroupNotFound" => {
                    CreateClusterError::ClusterSecurityGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSubnetGroupNotFoundFault" => {
                    CreateClusterError::ClusterSubnetGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "DependentServiceRequestThrottlingFault" => {
                    CreateClusterError::DependentServiceRequestThrottlingFault(String::from(
                        parsed_error.message,
                    ))
                }
                "HsmClientCertificateNotFoundFault" => {
                    CreateClusterError::HsmClientCertificateNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "HsmConfigurationNotFoundFault" => {
                    CreateClusterError::HsmConfigurationNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientClusterCapacity" => {
                    CreateClusterError::InsufficientClusterCapacityFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterSubnetGroupStateFault" => {
                    CreateClusterError::InvalidClusterSubnetGroupStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidElasticIpFault" => {
                    CreateClusterError::InvalidElasticIpFault(String::from(parsed_error.message))
                }
                "InvalidSubnet" => {
                    CreateClusterError::InvalidSubnet(String::from(parsed_error.message))
                }
                "InvalidTagFault" => {
                    CreateClusterError::InvalidTagFault(String::from(parsed_error.message))
                }
                "InvalidVPCNetworkStateFault" => CreateClusterError::InvalidVPCNetworkStateFault(
                    String::from(parsed_error.message),
                ),
                "LimitExceededFault" => {
                    CreateClusterError::LimitExceededFault(String::from(parsed_error.message))
                }
                "NumberOfNodesPerClusterLimitExceeded" => {
                    CreateClusterError::NumberOfNodesPerClusterLimitExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "NumberOfNodesQuotaExceeded" => {
                    CreateClusterError::NumberOfNodesQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "TagLimitExceededFault" => {
                    CreateClusterError::TagLimitExceededFault(String::from(parsed_error.message))
                }
                "UnauthorizedOperation" => {
                    CreateClusterError::UnauthorizedOperation(String::from(parsed_error.message))
                }
                _ => CreateClusterError::Unknown(String::from(body)),
            },
            Err(_) => CreateClusterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateClusterError {
    fn from(err: XmlParseError) -> CreateClusterError {
        let XmlParseError(message) = err;
        CreateClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateClusterError {
    fn from(err: CredentialsError) -> CreateClusterError {
        CreateClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterError {
    fn from(err: HttpDispatchError) -> CreateClusterError {
        CreateClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterError {
    fn from(err: io::Error) -> CreateClusterError {
        CreateClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::ClusterAlreadyExistsFault(ref cause) => cause,
            CreateClusterError::ClusterParameterGroupNotFoundFault(ref cause) => cause,
            CreateClusterError::ClusterQuotaExceededFault(ref cause) => cause,
            CreateClusterError::ClusterSecurityGroupNotFoundFault(ref cause) => cause,
            CreateClusterError::ClusterSubnetGroupNotFoundFault(ref cause) => cause,
            CreateClusterError::DependentServiceRequestThrottlingFault(ref cause) => cause,
            CreateClusterError::HsmClientCertificateNotFoundFault(ref cause) => cause,
            CreateClusterError::HsmConfigurationNotFoundFault(ref cause) => cause,
            CreateClusterError::InsufficientClusterCapacityFault(ref cause) => cause,
            CreateClusterError::InvalidClusterSubnetGroupStateFault(ref cause) => cause,
            CreateClusterError::InvalidElasticIpFault(ref cause) => cause,
            CreateClusterError::InvalidSubnet(ref cause) => cause,
            CreateClusterError::InvalidTagFault(ref cause) => cause,
            CreateClusterError::InvalidVPCNetworkStateFault(ref cause) => cause,
            CreateClusterError::LimitExceededFault(ref cause) => cause,
            CreateClusterError::NumberOfNodesPerClusterLimitExceededFault(ref cause) => cause,
            CreateClusterError::NumberOfNodesQuotaExceededFault(ref cause) => cause,
            CreateClusterError::TagLimitExceededFault(ref cause) => cause,
            CreateClusterError::UnauthorizedOperation(ref cause) => cause,
            CreateClusterError::Validation(ref cause) => cause,
            CreateClusterError::Credentials(ref err) => err.description(),
            CreateClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateClusterParameterGroupError {
    /// <p>A cluster parameter group with the same name already exists.</p>
    ClusterParameterGroupAlreadyExistsFault(String),
    /// <p>The request would result in the user exceeding the allowed number of cluster parameter groups. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterParameterGroupQuotaExceededFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateClusterParameterGroupError {
    pub fn from_body(body: &str) -> CreateClusterParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterParameterGroupAlreadyExists" => {
                    CreateClusterParameterGroupError::ClusterParameterGroupAlreadyExistsFault(
                        String::from(parsed_error.message),
                    )
                }
                "ClusterParameterGroupQuotaExceeded" => {
                    CreateClusterParameterGroupError::ClusterParameterGroupQuotaExceededFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagFault" => CreateClusterParameterGroupError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                "TagLimitExceededFault" => CreateClusterParameterGroupError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateClusterParameterGroupError::Unknown(String::from(body)),
            },
            Err(_) => CreateClusterParameterGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateClusterParameterGroupError {
    fn from(err: XmlParseError) -> CreateClusterParameterGroupError {
        let XmlParseError(message) = err;
        CreateClusterParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateClusterParameterGroupError {
    fn from(err: CredentialsError) -> CreateClusterParameterGroupError {
        CreateClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> CreateClusterParameterGroupError {
        CreateClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterParameterGroupError {
    fn from(err: io::Error) -> CreateClusterParameterGroupError {
        CreateClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterParameterGroupError::ClusterParameterGroupAlreadyExistsFault(
                ref cause,
            ) => cause,
            CreateClusterParameterGroupError::ClusterParameterGroupQuotaExceededFault(
                ref cause,
            ) => cause,
            CreateClusterParameterGroupError::InvalidTagFault(ref cause) => cause,
            CreateClusterParameterGroupError::TagLimitExceededFault(ref cause) => cause,
            CreateClusterParameterGroupError::Validation(ref cause) => cause,
            CreateClusterParameterGroupError::Credentials(ref err) => err.description(),
            CreateClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateClusterParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateClusterSecurityGroup
#[derive(Debug, PartialEq)]
pub enum CreateClusterSecurityGroupError {
    /// <p>A cluster security group with the same name already exists.</p>
    ClusterSecurityGroupAlreadyExistsFault(String),
    /// <p>The request would result in the user exceeding the allowed number of cluster security groups. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterSecurityGroupQuotaExceededFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateClusterSecurityGroupError {
    pub fn from_body(body: &str) -> CreateClusterSecurityGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSecurityGroupAlreadyExists" => {
                    CreateClusterSecurityGroupError::ClusterSecurityGroupAlreadyExistsFault(
                        String::from(parsed_error.message),
                    )
                }
                "QuotaExceeded.ClusterSecurityGroup" => {
                    CreateClusterSecurityGroupError::ClusterSecurityGroupQuotaExceededFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagFault" => CreateClusterSecurityGroupError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                "TagLimitExceededFault" => CreateClusterSecurityGroupError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateClusterSecurityGroupError::Unknown(String::from(body)),
            },
            Err(_) => CreateClusterSecurityGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateClusterSecurityGroupError {
    fn from(err: XmlParseError) -> CreateClusterSecurityGroupError {
        let XmlParseError(message) = err;
        CreateClusterSecurityGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateClusterSecurityGroupError {
    fn from(err: CredentialsError) -> CreateClusterSecurityGroupError {
        CreateClusterSecurityGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterSecurityGroupError {
    fn from(err: HttpDispatchError) -> CreateClusterSecurityGroupError {
        CreateClusterSecurityGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterSecurityGroupError {
    fn from(err: io::Error) -> CreateClusterSecurityGroupError {
        CreateClusterSecurityGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClusterSecurityGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterSecurityGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterSecurityGroupError::ClusterSecurityGroupAlreadyExistsFault(ref cause) => {
                cause
            }
            CreateClusterSecurityGroupError::ClusterSecurityGroupQuotaExceededFault(ref cause) => {
                cause
            }
            CreateClusterSecurityGroupError::InvalidTagFault(ref cause) => cause,
            CreateClusterSecurityGroupError::TagLimitExceededFault(ref cause) => cause,
            CreateClusterSecurityGroupError::Validation(ref cause) => cause,
            CreateClusterSecurityGroupError::Credentials(ref err) => err.description(),
            CreateClusterSecurityGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateClusterSecurityGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateClusterSnapshotError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The value specified as a snapshot identifier is already used by an existing snapshot.</p>
    ClusterSnapshotAlreadyExistsFault(String),
    /// <p>The request would result in the user exceeding the allowed number of cluster snapshots.</p>
    ClusterSnapshotQuotaExceededFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateClusterSnapshotError {
    pub fn from_body(body: &str) -> CreateClusterSnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => CreateClusterSnapshotError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "ClusterSnapshotAlreadyExists" => {
                    CreateClusterSnapshotError::ClusterSnapshotAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSnapshotQuotaExceeded" => {
                    CreateClusterSnapshotError::ClusterSnapshotQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterState" => CreateClusterSnapshotError::InvalidClusterStateFault(
                    String::from(parsed_error.message),
                ),
                "InvalidTagFault" => {
                    CreateClusterSnapshotError::InvalidTagFault(String::from(parsed_error.message))
                }
                "TagLimitExceededFault" => CreateClusterSnapshotError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateClusterSnapshotError::Unknown(String::from(body)),
            },
            Err(_) => CreateClusterSnapshotError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateClusterSnapshotError {
    fn from(err: XmlParseError) -> CreateClusterSnapshotError {
        let XmlParseError(message) = err;
        CreateClusterSnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateClusterSnapshotError {
    fn from(err: CredentialsError) -> CreateClusterSnapshotError {
        CreateClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterSnapshotError {
    fn from(err: HttpDispatchError) -> CreateClusterSnapshotError {
        CreateClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterSnapshotError {
    fn from(err: io::Error) -> CreateClusterSnapshotError {
        CreateClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterSnapshotError::ClusterNotFoundFault(ref cause) => cause,
            CreateClusterSnapshotError::ClusterSnapshotAlreadyExistsFault(ref cause) => cause,
            CreateClusterSnapshotError::ClusterSnapshotQuotaExceededFault(ref cause) => cause,
            CreateClusterSnapshotError::InvalidClusterStateFault(ref cause) => cause,
            CreateClusterSnapshotError::InvalidTagFault(ref cause) => cause,
            CreateClusterSnapshotError::TagLimitExceededFault(ref cause) => cause,
            CreateClusterSnapshotError::Validation(ref cause) => cause,
            CreateClusterSnapshotError::Credentials(ref err) => err.description(),
            CreateClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateClusterSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateClusterSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateClusterSubnetGroupError {
    /// <p>A <i>ClusterSubnetGroupName</i> is already used by an existing cluster subnet group. </p>
    ClusterSubnetGroupAlreadyExistsFault(String),
    /// <p>The request would result in user exceeding the allowed number of cluster subnet groups. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterSubnetGroupQuotaExceededFault(String),
    /// <p>The request would result in user exceeding the allowed number of subnets in a cluster subnet groups. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterSubnetQuotaExceededFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>The requested subnet is not valid, or not all of the subnets are in the same VPC.</p>
    InvalidSubnet(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateClusterSubnetGroupError {
    pub fn from_body(body: &str) -> CreateClusterSubnetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSubnetGroupAlreadyExists" => {
                    CreateClusterSubnetGroupError::ClusterSubnetGroupAlreadyExistsFault(
                        String::from(parsed_error.message),
                    )
                }
                "ClusterSubnetGroupQuotaExceeded" => {
                    CreateClusterSubnetGroupError::ClusterSubnetGroupQuotaExceededFault(
                        String::from(parsed_error.message),
                    )
                }
                "ClusterSubnetQuotaExceededFault" => {
                    CreateClusterSubnetGroupError::ClusterSubnetQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "DependentServiceRequestThrottlingFault" => {
                    CreateClusterSubnetGroupError::DependentServiceRequestThrottlingFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidSubnet" => {
                    CreateClusterSubnetGroupError::InvalidSubnet(String::from(parsed_error.message))
                }
                "InvalidTagFault" => CreateClusterSubnetGroupError::InvalidTagFault(String::from(
                    parsed_error.message,
                )),
                "TagLimitExceededFault" => CreateClusterSubnetGroupError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                "UnauthorizedOperation" => CreateClusterSubnetGroupError::UnauthorizedOperation(
                    String::from(parsed_error.message),
                ),
                _ => CreateClusterSubnetGroupError::Unknown(String::from(body)),
            },
            Err(_) => CreateClusterSubnetGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateClusterSubnetGroupError {
    fn from(err: XmlParseError) -> CreateClusterSubnetGroupError {
        let XmlParseError(message) = err;
        CreateClusterSubnetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateClusterSubnetGroupError {
    fn from(err: CredentialsError) -> CreateClusterSubnetGroupError {
        CreateClusterSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterSubnetGroupError {
    fn from(err: HttpDispatchError) -> CreateClusterSubnetGroupError {
        CreateClusterSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterSubnetGroupError {
    fn from(err: io::Error) -> CreateClusterSubnetGroupError {
        CreateClusterSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClusterSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterSubnetGroupError::ClusterSubnetGroupAlreadyExistsFault(ref cause) => cause,
            CreateClusterSubnetGroupError::ClusterSubnetGroupQuotaExceededFault(ref cause) => cause,
            CreateClusterSubnetGroupError::ClusterSubnetQuotaExceededFault(ref cause) => cause,
            CreateClusterSubnetGroupError::DependentServiceRequestThrottlingFault(ref cause) => {
                cause
            }
            CreateClusterSubnetGroupError::InvalidSubnet(ref cause) => cause,
            CreateClusterSubnetGroupError::InvalidTagFault(ref cause) => cause,
            CreateClusterSubnetGroupError::TagLimitExceededFault(ref cause) => cause,
            CreateClusterSubnetGroupError::UnauthorizedOperation(ref cause) => cause,
            CreateClusterSubnetGroupError::Validation(ref cause) => cause,
            CreateClusterSubnetGroupError::Credentials(ref err) => err.description(),
            CreateClusterSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateClusterSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEventSubscription
#[derive(Debug, PartialEq)]
pub enum CreateEventSubscriptionError {
    /// <p>The request would exceed the allowed number of event subscriptions for this account. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    EventSubscriptionQuotaExceededFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>Amazon SNS has responded that there is a problem with the specified Amazon SNS topic.</p>
    SNSInvalidTopicFault(String),
    /// <p>You do not have permission to publish to the specified Amazon SNS topic.</p>
    SNSNoAuthorizationFault(String),
    /// <p>An Amazon SNS topic with the specified Amazon Resource Name (ARN) does not exist.</p>
    SNSTopicArnNotFoundFault(String),
    /// <p>The specified Amazon Redshift event source could not be found.</p>
    SourceNotFoundFault(String),
    /// <p>There is already an existing event notification subscription with the specified name.</p>
    SubscriptionAlreadyExistFault(String),
    /// <p>The value specified for the event category was not one of the allowed values, or it specified a category that does not apply to the specified source type. The allowed values are Configuration, Management, Monitoring, and Security.</p>
    SubscriptionCategoryNotFoundFault(String),
    /// <p>An Amazon Redshift event with the specified event ID does not exist.</p>
    SubscriptionEventIdNotFoundFault(String),
    /// <p>The value specified for the event severity was not one of the allowed values, or it specified a severity that does not apply to the specified source type. The allowed values are ERROR and INFO.</p>
    SubscriptionSeverityNotFoundFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateEventSubscriptionError {
    pub fn from_body(body: &str) -> CreateEventSubscriptionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "EventSubscriptionQuotaExceeded" => {
                    CreateEventSubscriptionError::EventSubscriptionQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidTagFault" => CreateEventSubscriptionError::InvalidTagFault(String::from(
                    parsed_error.message,
                )),
                "SNSInvalidTopic" => CreateEventSubscriptionError::SNSInvalidTopicFault(
                    String::from(parsed_error.message),
                ),
                "SNSNoAuthorization" => CreateEventSubscriptionError::SNSNoAuthorizationFault(
                    String::from(parsed_error.message),
                ),
                "SNSTopicArnNotFound" => CreateEventSubscriptionError::SNSTopicArnNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "SourceNotFound" => CreateEventSubscriptionError::SourceNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "SubscriptionAlreadyExist" => {
                    CreateEventSubscriptionError::SubscriptionAlreadyExistFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SubscriptionCategoryNotFound" => {
                    CreateEventSubscriptionError::SubscriptionCategoryNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SubscriptionEventIdNotFound" => {
                    CreateEventSubscriptionError::SubscriptionEventIdNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SubscriptionSeverityNotFound" => {
                    CreateEventSubscriptionError::SubscriptionSeverityNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "TagLimitExceededFault" => CreateEventSubscriptionError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateEventSubscriptionError::Unknown(String::from(body)),
            },
            Err(_) => CreateEventSubscriptionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateEventSubscriptionError {
    fn from(err: XmlParseError) -> CreateEventSubscriptionError {
        let XmlParseError(message) = err;
        CreateEventSubscriptionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateEventSubscriptionError {
    fn from(err: CredentialsError) -> CreateEventSubscriptionError {
        CreateEventSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEventSubscriptionError {
    fn from(err: HttpDispatchError) -> CreateEventSubscriptionError {
        CreateEventSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEventSubscriptionError {
    fn from(err: io::Error) -> CreateEventSubscriptionError {
        CreateEventSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            CreateEventSubscriptionError::EventSubscriptionQuotaExceededFault(ref cause) => cause,
            CreateEventSubscriptionError::InvalidTagFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSTopicArnNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::SourceNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::SubscriptionAlreadyExistFault(ref cause) => cause,
            CreateEventSubscriptionError::SubscriptionCategoryNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::SubscriptionEventIdNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::SubscriptionSeverityNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::TagLimitExceededFault(ref cause) => cause,
            CreateEventSubscriptionError::Validation(ref cause) => cause,
            CreateEventSubscriptionError::Credentials(ref err) => err.description(),
            CreateEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEventSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHsmClientCertificate
#[derive(Debug, PartialEq)]
pub enum CreateHsmClientCertificateError {
    /// <p>There is already an existing Amazon Redshift HSM client certificate with the specified identifier.</p>
    HsmClientCertificateAlreadyExistsFault(String),
    /// <p>The quota for HSM client certificates has been reached. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    HsmClientCertificateQuotaExceededFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHsmClientCertificateError {
    pub fn from_body(body: &str) -> CreateHsmClientCertificateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HsmClientCertificateAlreadyExistsFault" => {
                    CreateHsmClientCertificateError::HsmClientCertificateAlreadyExistsFault(
                        String::from(parsed_error.message),
                    )
                }
                "HsmClientCertificateQuotaExceededFault" => {
                    CreateHsmClientCertificateError::HsmClientCertificateQuotaExceededFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagFault" => CreateHsmClientCertificateError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                "TagLimitExceededFault" => CreateHsmClientCertificateError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateHsmClientCertificateError::Unknown(String::from(body)),
            },
            Err(_) => CreateHsmClientCertificateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateHsmClientCertificateError {
    fn from(err: XmlParseError) -> CreateHsmClientCertificateError {
        let XmlParseError(message) = err;
        CreateHsmClientCertificateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateHsmClientCertificateError {
    fn from(err: CredentialsError) -> CreateHsmClientCertificateError {
        CreateHsmClientCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHsmClientCertificateError {
    fn from(err: HttpDispatchError) -> CreateHsmClientCertificateError {
        CreateHsmClientCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHsmClientCertificateError {
    fn from(err: io::Error) -> CreateHsmClientCertificateError {
        CreateHsmClientCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHsmClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHsmClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            CreateHsmClientCertificateError::HsmClientCertificateAlreadyExistsFault(ref cause) => {
                cause
            }
            CreateHsmClientCertificateError::HsmClientCertificateQuotaExceededFault(ref cause) => {
                cause
            }
            CreateHsmClientCertificateError::InvalidTagFault(ref cause) => cause,
            CreateHsmClientCertificateError::TagLimitExceededFault(ref cause) => cause,
            CreateHsmClientCertificateError::Validation(ref cause) => cause,
            CreateHsmClientCertificateError::Credentials(ref err) => err.description(),
            CreateHsmClientCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateHsmClientCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHsmConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateHsmConfigurationError {
    /// <p>There is already an existing Amazon Redshift HSM configuration with the specified identifier.</p>
    HsmConfigurationAlreadyExistsFault(String),
    /// <p>The quota for HSM configurations has been reached. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    HsmConfigurationQuotaExceededFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHsmConfigurationError {
    pub fn from_body(body: &str) -> CreateHsmConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HsmConfigurationAlreadyExistsFault" => {
                    CreateHsmConfigurationError::HsmConfigurationAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "HsmConfigurationQuotaExceededFault" => {
                    CreateHsmConfigurationError::HsmConfigurationQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidTagFault" => {
                    CreateHsmConfigurationError::InvalidTagFault(String::from(parsed_error.message))
                }
                "TagLimitExceededFault" => CreateHsmConfigurationError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateHsmConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => CreateHsmConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateHsmConfigurationError {
    fn from(err: XmlParseError) -> CreateHsmConfigurationError {
        let XmlParseError(message) = err;
        CreateHsmConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateHsmConfigurationError {
    fn from(err: CredentialsError) -> CreateHsmConfigurationError {
        CreateHsmConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHsmConfigurationError {
    fn from(err: HttpDispatchError) -> CreateHsmConfigurationError {
        CreateHsmConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHsmConfigurationError {
    fn from(err: io::Error) -> CreateHsmConfigurationError {
        CreateHsmConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHsmConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHsmConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateHsmConfigurationError::HsmConfigurationAlreadyExistsFault(ref cause) => cause,
            CreateHsmConfigurationError::HsmConfigurationQuotaExceededFault(ref cause) => cause,
            CreateHsmConfigurationError::InvalidTagFault(ref cause) => cause,
            CreateHsmConfigurationError::TagLimitExceededFault(ref cause) => cause,
            CreateHsmConfigurationError::Validation(ref cause) => cause,
            CreateHsmConfigurationError::Credentials(ref err) => err.description(),
            CreateHsmConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateHsmConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSnapshotCopyGrant
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotCopyGrantError {
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The encryption key has exceeded its grant limit in AWS KMS.</p>
    LimitExceededFault(String),
    /// <p>The snapshot copy grant can't be created because a grant with the same name already exists.</p>
    SnapshotCopyGrantAlreadyExistsFault(String),
    /// <p>The AWS account has exceeded the maximum number of snapshot copy grants in this region.</p>
    SnapshotCopyGrantQuotaExceededFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSnapshotCopyGrantError {
    pub fn from_body(body: &str) -> CreateSnapshotCopyGrantError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DependentServiceRequestThrottlingFault" => {
                    CreateSnapshotCopyGrantError::DependentServiceRequestThrottlingFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagFault" => CreateSnapshotCopyGrantError::InvalidTagFault(String::from(
                    parsed_error.message,
                )),
                "LimitExceededFault" => CreateSnapshotCopyGrantError::LimitExceededFault(
                    String::from(parsed_error.message),
                ),
                "SnapshotCopyGrantAlreadyExistsFault" => {
                    CreateSnapshotCopyGrantError::SnapshotCopyGrantAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SnapshotCopyGrantQuotaExceededFault" => {
                    CreateSnapshotCopyGrantError::SnapshotCopyGrantQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "TagLimitExceededFault" => CreateSnapshotCopyGrantError::TagLimitExceededFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateSnapshotCopyGrantError::Unknown(String::from(body)),
            },
            Err(_) => CreateSnapshotCopyGrantError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateSnapshotCopyGrantError {
    fn from(err: XmlParseError) -> CreateSnapshotCopyGrantError {
        let XmlParseError(message) = err;
        CreateSnapshotCopyGrantError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateSnapshotCopyGrantError {
    fn from(err: CredentialsError) -> CreateSnapshotCopyGrantError {
        CreateSnapshotCopyGrantError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSnapshotCopyGrantError {
    fn from(err: HttpDispatchError) -> CreateSnapshotCopyGrantError {
        CreateSnapshotCopyGrantError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSnapshotCopyGrantError {
    fn from(err: io::Error) -> CreateSnapshotCopyGrantError {
        CreateSnapshotCopyGrantError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSnapshotCopyGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSnapshotCopyGrantError {
    fn description(&self) -> &str {
        match *self {
            CreateSnapshotCopyGrantError::DependentServiceRequestThrottlingFault(ref cause) => {
                cause
            }
            CreateSnapshotCopyGrantError::InvalidTagFault(ref cause) => cause,
            CreateSnapshotCopyGrantError::LimitExceededFault(ref cause) => cause,
            CreateSnapshotCopyGrantError::SnapshotCopyGrantAlreadyExistsFault(ref cause) => cause,
            CreateSnapshotCopyGrantError::SnapshotCopyGrantQuotaExceededFault(ref cause) => cause,
            CreateSnapshotCopyGrantError::TagLimitExceededFault(ref cause) => cause,
            CreateSnapshotCopyGrantError::Validation(ref cause) => cause,
            CreateSnapshotCopyGrantError::Credentials(ref err) => err.description(),
            CreateSnapshotCopyGrantError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSnapshotCopyGrantError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The request exceeds the limit of 10 tags for the resource.</p>
    TagLimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTagsError {
    pub fn from_body(body: &str) -> CreateTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTagFault" => {
                    CreateTagsError::InvalidTagFault(String::from(parsed_error.message))
                }
                "ResourceNotFoundFault" => {
                    CreateTagsError::ResourceNotFoundFault(String::from(parsed_error.message))
                }
                "TagLimitExceededFault" => {
                    CreateTagsError::TagLimitExceededFault(String::from(parsed_error.message))
                }
                _ => CreateTagsError::Unknown(String::from(body)),
            },
            Err(_) => CreateTagsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateTagsError {
    fn from(err: XmlParseError) -> CreateTagsError {
        let XmlParseError(message) = err;
        CreateTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateTagsError {
    fn from(err: CredentialsError) -> CreateTagsError {
        CreateTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTagsError {
    fn from(err: HttpDispatchError) -> CreateTagsError {
        CreateTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTagsError {
    fn from(err: io::Error) -> CreateTagsError {
        CreateTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateTagsError::InvalidTagFault(ref cause) => cause,
            CreateTagsError::ResourceNotFoundFault(ref cause) => cause,
            CreateTagsError::TagLimitExceededFault(ref cause) => cause,
            CreateTagsError::Validation(ref cause) => cause,
            CreateTagsError::Credentials(ref err) => err.description(),
            CreateTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The value specified as a snapshot identifier is already used by an existing snapshot.</p>
    ClusterSnapshotAlreadyExistsFault(String),
    /// <p>The request would result in the user exceeding the allowed number of cluster snapshots.</p>
    ClusterSnapshotQuotaExceededFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteClusterError {
    pub fn from_body(body: &str) -> DeleteClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => {
                    DeleteClusterError::ClusterNotFoundFault(String::from(parsed_error.message))
                }
                "ClusterSnapshotAlreadyExists" => {
                    DeleteClusterError::ClusterSnapshotAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSnapshotQuotaExceeded" => {
                    DeleteClusterError::ClusterSnapshotQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterState" => {
                    DeleteClusterError::InvalidClusterStateFault(String::from(parsed_error.message))
                }
                _ => DeleteClusterError::Unknown(String::from(body)),
            },
            Err(_) => DeleteClusterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteClusterError {
    fn from(err: XmlParseError) -> DeleteClusterError {
        let XmlParseError(message) = err;
        DeleteClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteClusterError {
    fn from(err: CredentialsError) -> DeleteClusterError {
        DeleteClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterError {
    fn from(err: HttpDispatchError) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterError {
    fn from(err: io::Error) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterError::ClusterNotFoundFault(ref cause) => cause,
            DeleteClusterError::ClusterSnapshotAlreadyExistsFault(ref cause) => cause,
            DeleteClusterError::ClusterSnapshotQuotaExceededFault(ref cause) => cause,
            DeleteClusterError::InvalidClusterStateFault(ref cause) => cause,
            DeleteClusterError::Validation(ref cause) => cause,
            DeleteClusterError::Credentials(ref err) => err.description(),
            DeleteClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteClusterParameterGroupError {
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// <p>The cluster parameter group action can not be completed because another task is in progress that involves the parameter group. Wait a few moments and try the operation again.</p>
    InvalidClusterParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteClusterParameterGroupError {
    pub fn from_body(body: &str) -> DeleteClusterParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterParameterGroupNotFound" => {
                    DeleteClusterParameterGroupError::ClusterParameterGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterParameterGroupState" => {
                    DeleteClusterParameterGroupError::InvalidClusterParameterGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => DeleteClusterParameterGroupError::Unknown(String::from(body)),
            },
            Err(_) => DeleteClusterParameterGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteClusterParameterGroupError {
    fn from(err: XmlParseError) -> DeleteClusterParameterGroupError {
        let XmlParseError(message) = err;
        DeleteClusterParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteClusterParameterGroupError {
    fn from(err: CredentialsError) -> DeleteClusterParameterGroupError {
        DeleteClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> DeleteClusterParameterGroupError {
        DeleteClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterParameterGroupError {
    fn from(err: io::Error) -> DeleteClusterParameterGroupError {
        DeleteClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterParameterGroupError::ClusterParameterGroupNotFoundFault(ref cause) => {
                cause
            }
            DeleteClusterParameterGroupError::InvalidClusterParameterGroupStateFault(ref cause) => {
                cause
            }
            DeleteClusterParameterGroupError::Validation(ref cause) => cause,
            DeleteClusterParameterGroupError::Credentials(ref err) => err.description(),
            DeleteClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteClusterParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClusterSecurityGroup
#[derive(Debug, PartialEq)]
pub enum DeleteClusterSecurityGroupError {
    /// <p>The cluster security group name does not refer to an existing cluster security group.</p>
    ClusterSecurityGroupNotFoundFault(String),
    /// <p>The state of the cluster security group is not <code>available</code>. </p>
    InvalidClusterSecurityGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteClusterSecurityGroupError {
    pub fn from_body(body: &str) -> DeleteClusterSecurityGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSecurityGroupNotFound" => {
                    DeleteClusterSecurityGroupError::ClusterSecurityGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterSecurityGroupState" => {
                    DeleteClusterSecurityGroupError::InvalidClusterSecurityGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => DeleteClusterSecurityGroupError::Unknown(String::from(body)),
            },
            Err(_) => DeleteClusterSecurityGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteClusterSecurityGroupError {
    fn from(err: XmlParseError) -> DeleteClusterSecurityGroupError {
        let XmlParseError(message) = err;
        DeleteClusterSecurityGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteClusterSecurityGroupError {
    fn from(err: CredentialsError) -> DeleteClusterSecurityGroupError {
        DeleteClusterSecurityGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterSecurityGroupError {
    fn from(err: HttpDispatchError) -> DeleteClusterSecurityGroupError {
        DeleteClusterSecurityGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterSecurityGroupError {
    fn from(err: io::Error) -> DeleteClusterSecurityGroupError {
        DeleteClusterSecurityGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClusterSecurityGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterSecurityGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterSecurityGroupError::ClusterSecurityGroupNotFoundFault(ref cause) => cause,
            DeleteClusterSecurityGroupError::InvalidClusterSecurityGroupStateFault(ref cause) => {
                cause
            }
            DeleteClusterSecurityGroupError::Validation(ref cause) => cause,
            DeleteClusterSecurityGroupError::Credentials(ref err) => err.description(),
            DeleteClusterSecurityGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteClusterSecurityGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteClusterSnapshotError {
    /// <p>The snapshot identifier does not refer to an existing cluster snapshot.</p>
    ClusterSnapshotNotFoundFault(String),
    /// <p>The specified cluster snapshot is not in the <code>available</code> state, or other accounts are authorized to access the snapshot. </p>
    InvalidClusterSnapshotStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteClusterSnapshotError {
    pub fn from_body(body: &str) -> DeleteClusterSnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSnapshotNotFound" => {
                    DeleteClusterSnapshotError::ClusterSnapshotNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterSnapshotState" => {
                    DeleteClusterSnapshotError::InvalidClusterSnapshotStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteClusterSnapshotError::Unknown(String::from(body)),
            },
            Err(_) => DeleteClusterSnapshotError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteClusterSnapshotError {
    fn from(err: XmlParseError) -> DeleteClusterSnapshotError {
        let XmlParseError(message) = err;
        DeleteClusterSnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteClusterSnapshotError {
    fn from(err: CredentialsError) -> DeleteClusterSnapshotError {
        DeleteClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterSnapshotError {
    fn from(err: HttpDispatchError) -> DeleteClusterSnapshotError {
        DeleteClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterSnapshotError {
    fn from(err: io::Error) -> DeleteClusterSnapshotError {
        DeleteClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterSnapshotError::ClusterSnapshotNotFoundFault(ref cause) => cause,
            DeleteClusterSnapshotError::InvalidClusterSnapshotStateFault(ref cause) => cause,
            DeleteClusterSnapshotError::Validation(ref cause) => cause,
            DeleteClusterSnapshotError::Credentials(ref err) => err.description(),
            DeleteClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteClusterSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClusterSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteClusterSubnetGroupError {
    /// <p>The cluster subnet group name does not refer to an existing cluster subnet group.</p>
    ClusterSubnetGroupNotFoundFault(String),
    /// <p>The cluster subnet group cannot be deleted because it is in use.</p>
    InvalidClusterSubnetGroupStateFault(String),
    /// <p>The state of the subnet is invalid.</p>
    InvalidClusterSubnetStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteClusterSubnetGroupError {
    pub fn from_body(body: &str) -> DeleteClusterSubnetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSubnetGroupNotFoundFault" => {
                    DeleteClusterSubnetGroupError::ClusterSubnetGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterSubnetGroupStateFault" => {
                    DeleteClusterSubnetGroupError::InvalidClusterSubnetGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterSubnetStateFault" => {
                    DeleteClusterSubnetGroupError::InvalidClusterSubnetStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteClusterSubnetGroupError::Unknown(String::from(body)),
            },
            Err(_) => DeleteClusterSubnetGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteClusterSubnetGroupError {
    fn from(err: XmlParseError) -> DeleteClusterSubnetGroupError {
        let XmlParseError(message) = err;
        DeleteClusterSubnetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteClusterSubnetGroupError {
    fn from(err: CredentialsError) -> DeleteClusterSubnetGroupError {
        DeleteClusterSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterSubnetGroupError {
    fn from(err: HttpDispatchError) -> DeleteClusterSubnetGroupError {
        DeleteClusterSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterSubnetGroupError {
    fn from(err: io::Error) -> DeleteClusterSubnetGroupError {
        DeleteClusterSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClusterSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterSubnetGroupError::ClusterSubnetGroupNotFoundFault(ref cause) => cause,
            DeleteClusterSubnetGroupError::InvalidClusterSubnetGroupStateFault(ref cause) => cause,
            DeleteClusterSubnetGroupError::InvalidClusterSubnetStateFault(ref cause) => cause,
            DeleteClusterSubnetGroupError::Validation(ref cause) => cause,
            DeleteClusterSubnetGroupError::Credentials(ref err) => err.description(),
            DeleteClusterSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteClusterSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEventSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteEventSubscriptionError {
    /// <p>The subscription request is invalid because it is a duplicate request. This subscription request is already in progress.</p>
    InvalidSubscriptionStateFault(String),
    /// <p>An Amazon Redshift event notification subscription with the specified name does not exist.</p>
    SubscriptionNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteEventSubscriptionError {
    pub fn from_body(body: &str) -> DeleteEventSubscriptionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidSubscriptionStateFault" => {
                    DeleteEventSubscriptionError::InvalidSubscriptionStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SubscriptionNotFound" => DeleteEventSubscriptionError::SubscriptionNotFoundFault(
                    String::from(parsed_error.message),
                ),
                _ => DeleteEventSubscriptionError::Unknown(String::from(body)),
            },
            Err(_) => DeleteEventSubscriptionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteEventSubscriptionError {
    fn from(err: XmlParseError) -> DeleteEventSubscriptionError {
        let XmlParseError(message) = err;
        DeleteEventSubscriptionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteEventSubscriptionError {
    fn from(err: CredentialsError) -> DeleteEventSubscriptionError {
        DeleteEventSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEventSubscriptionError {
    fn from(err: HttpDispatchError) -> DeleteEventSubscriptionError {
        DeleteEventSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEventSubscriptionError {
    fn from(err: io::Error) -> DeleteEventSubscriptionError {
        DeleteEventSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventSubscriptionError::InvalidSubscriptionStateFault(ref cause) => cause,
            DeleteEventSubscriptionError::SubscriptionNotFoundFault(ref cause) => cause,
            DeleteEventSubscriptionError::Validation(ref cause) => cause,
            DeleteEventSubscriptionError::Credentials(ref err) => err.description(),
            DeleteEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEventSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHsmClientCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteHsmClientCertificateError {
    /// <p>There is no Amazon Redshift HSM client certificate with the specified identifier.</p>
    HsmClientCertificateNotFoundFault(String),
    /// <p>The specified HSM client certificate is not in the <code>available</code> state, or it is still in use by one or more Amazon Redshift clusters.</p>
    InvalidHsmClientCertificateStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteHsmClientCertificateError {
    pub fn from_body(body: &str) -> DeleteHsmClientCertificateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HsmClientCertificateNotFoundFault" => {
                    DeleteHsmClientCertificateError::HsmClientCertificateNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidHsmClientCertificateStateFault" => {
                    DeleteHsmClientCertificateError::InvalidHsmClientCertificateStateFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => DeleteHsmClientCertificateError::Unknown(String::from(body)),
            },
            Err(_) => DeleteHsmClientCertificateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteHsmClientCertificateError {
    fn from(err: XmlParseError) -> DeleteHsmClientCertificateError {
        let XmlParseError(message) = err;
        DeleteHsmClientCertificateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteHsmClientCertificateError {
    fn from(err: CredentialsError) -> DeleteHsmClientCertificateError {
        DeleteHsmClientCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteHsmClientCertificateError {
    fn from(err: HttpDispatchError) -> DeleteHsmClientCertificateError {
        DeleteHsmClientCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteHsmClientCertificateError {
    fn from(err: io::Error) -> DeleteHsmClientCertificateError {
        DeleteHsmClientCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteHsmClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHsmClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteHsmClientCertificateError::HsmClientCertificateNotFoundFault(ref cause) => cause,
            DeleteHsmClientCertificateError::InvalidHsmClientCertificateStateFault(ref cause) => {
                cause
            }
            DeleteHsmClientCertificateError::Validation(ref cause) => cause,
            DeleteHsmClientCertificateError::Credentials(ref err) => err.description(),
            DeleteHsmClientCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteHsmClientCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHsmConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteHsmConfigurationError {
    /// <p>There is no Amazon Redshift HSM configuration with the specified identifier.</p>
    HsmConfigurationNotFoundFault(String),
    /// <p>The specified HSM configuration is not in the <code>available</code> state, or it is still in use by one or more Amazon Redshift clusters.</p>
    InvalidHsmConfigurationStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteHsmConfigurationError {
    pub fn from_body(body: &str) -> DeleteHsmConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HsmConfigurationNotFoundFault" => {
                    DeleteHsmConfigurationError::HsmConfigurationNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidHsmConfigurationStateFault" => {
                    DeleteHsmConfigurationError::InvalidHsmConfigurationStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteHsmConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteHsmConfigurationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteHsmConfigurationError {
    fn from(err: XmlParseError) -> DeleteHsmConfigurationError {
        let XmlParseError(message) = err;
        DeleteHsmConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteHsmConfigurationError {
    fn from(err: CredentialsError) -> DeleteHsmConfigurationError {
        DeleteHsmConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteHsmConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteHsmConfigurationError {
        DeleteHsmConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteHsmConfigurationError {
    fn from(err: io::Error) -> DeleteHsmConfigurationError {
        DeleteHsmConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteHsmConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHsmConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteHsmConfigurationError::HsmConfigurationNotFoundFault(ref cause) => cause,
            DeleteHsmConfigurationError::InvalidHsmConfigurationStateFault(ref cause) => cause,
            DeleteHsmConfigurationError::Validation(ref cause) => cause,
            DeleteHsmConfigurationError::Credentials(ref err) => err.description(),
            DeleteHsmConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteHsmConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSnapshotCopyGrant
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotCopyGrantError {
    /// <p>The snapshot copy grant can't be deleted because it is used by one or more clusters.</p>
    InvalidSnapshotCopyGrantStateFault(String),
    /// <p>The specified snapshot copy grant can't be found. Make sure that the name is typed correctly and that the grant exists in the destination region.</p>
    SnapshotCopyGrantNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSnapshotCopyGrantError {
    pub fn from_body(body: &str) -> DeleteSnapshotCopyGrantError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidSnapshotCopyGrantStateFault" => {
                    DeleteSnapshotCopyGrantError::InvalidSnapshotCopyGrantStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SnapshotCopyGrantNotFoundFault" => {
                    DeleteSnapshotCopyGrantError::SnapshotCopyGrantNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteSnapshotCopyGrantError::Unknown(String::from(body)),
            },
            Err(_) => DeleteSnapshotCopyGrantError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteSnapshotCopyGrantError {
    fn from(err: XmlParseError) -> DeleteSnapshotCopyGrantError {
        let XmlParseError(message) = err;
        DeleteSnapshotCopyGrantError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteSnapshotCopyGrantError {
    fn from(err: CredentialsError) -> DeleteSnapshotCopyGrantError {
        DeleteSnapshotCopyGrantError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSnapshotCopyGrantError {
    fn from(err: HttpDispatchError) -> DeleteSnapshotCopyGrantError {
        DeleteSnapshotCopyGrantError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSnapshotCopyGrantError {
    fn from(err: io::Error) -> DeleteSnapshotCopyGrantError {
        DeleteSnapshotCopyGrantError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSnapshotCopyGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSnapshotCopyGrantError {
    fn description(&self) -> &str {
        match *self {
            DeleteSnapshotCopyGrantError::InvalidSnapshotCopyGrantStateFault(ref cause) => cause,
            DeleteSnapshotCopyGrantError::SnapshotCopyGrantNotFoundFault(ref cause) => cause,
            DeleteSnapshotCopyGrantError::Validation(ref cause) => cause,
            DeleteSnapshotCopyGrantError::Credentials(ref err) => err.description(),
            DeleteSnapshotCopyGrantError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSnapshotCopyGrantError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagsError {
    pub fn from_body(body: &str) -> DeleteTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTagFault" => {
                    DeleteTagsError::InvalidTagFault(String::from(parsed_error.message))
                }
                "ResourceNotFoundFault" => {
                    DeleteTagsError::ResourceNotFoundFault(String::from(parsed_error.message))
                }
                _ => DeleteTagsError::Unknown(String::from(body)),
            },
            Err(_) => DeleteTagsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteTagsError {
    fn from(err: XmlParseError) -> DeleteTagsError {
        let XmlParseError(message) = err;
        DeleteTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteTagsError {
    fn from(err: CredentialsError) -> DeleteTagsError {
        DeleteTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsError {
    fn from(err: HttpDispatchError) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsError {
    fn from(err: io::Error) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::InvalidTagFault(ref cause) => cause,
            DeleteTagsError::ResourceNotFoundFault(ref cause) => cause,
            DeleteTagsError::Validation(ref cause) => cause,
            DeleteTagsError::Credentials(ref err) => err.description(),
            DeleteTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusterParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeClusterParameterGroupsError {
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClusterParameterGroupsError {
    pub fn from_body(body: &str) -> DescribeClusterParameterGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterParameterGroupNotFound" => {
                    DescribeClusterParameterGroupsError::ClusterParameterGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagFault" => DescribeClusterParameterGroupsError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeClusterParameterGroupsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeClusterParameterGroupsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeClusterParameterGroupsError {
    fn from(err: XmlParseError) -> DescribeClusterParameterGroupsError {
        let XmlParseError(message) = err;
        DescribeClusterParameterGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeClusterParameterGroupsError {
    fn from(err: CredentialsError) -> DescribeClusterParameterGroupsError {
        DescribeClusterParameterGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterParameterGroupsError {
    fn from(err: HttpDispatchError) -> DescribeClusterParameterGroupsError {
        DescribeClusterParameterGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterParameterGroupsError {
    fn from(err: io::Error) -> DescribeClusterParameterGroupsError {
        DescribeClusterParameterGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterParameterGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterParameterGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterParameterGroupsError::ClusterParameterGroupNotFoundFault(ref cause) => {
                cause
            }
            DescribeClusterParameterGroupsError::InvalidTagFault(ref cause) => cause,
            DescribeClusterParameterGroupsError::Validation(ref cause) => cause,
            DescribeClusterParameterGroupsError::Credentials(ref err) => err.description(),
            DescribeClusterParameterGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeClusterParameterGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusterParameters
#[derive(Debug, PartialEq)]
pub enum DescribeClusterParametersError {
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClusterParametersError {
    pub fn from_body(body: &str) -> DescribeClusterParametersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterParameterGroupNotFound" => {
                    DescribeClusterParametersError::ClusterParameterGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => DescribeClusterParametersError::Unknown(String::from(body)),
            },
            Err(_) => DescribeClusterParametersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeClusterParametersError {
    fn from(err: XmlParseError) -> DescribeClusterParametersError {
        let XmlParseError(message) = err;
        DescribeClusterParametersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeClusterParametersError {
    fn from(err: CredentialsError) -> DescribeClusterParametersError {
        DescribeClusterParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterParametersError {
    fn from(err: HttpDispatchError) -> DescribeClusterParametersError {
        DescribeClusterParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterParametersError {
    fn from(err: io::Error) -> DescribeClusterParametersError {
        DescribeClusterParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterParametersError::ClusterParameterGroupNotFoundFault(ref cause) => cause,
            DescribeClusterParametersError::Validation(ref cause) => cause,
            DescribeClusterParametersError::Credentials(ref err) => err.description(),
            DescribeClusterParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeClusterParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusterSecurityGroups
#[derive(Debug, PartialEq)]
pub enum DescribeClusterSecurityGroupsError {
    /// <p>The cluster security group name does not refer to an existing cluster security group.</p>
    ClusterSecurityGroupNotFoundFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClusterSecurityGroupsError {
    pub fn from_body(body: &str) -> DescribeClusterSecurityGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSecurityGroupNotFound" => {
                    DescribeClusterSecurityGroupsError::ClusterSecurityGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagFault" => DescribeClusterSecurityGroupsError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeClusterSecurityGroupsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeClusterSecurityGroupsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeClusterSecurityGroupsError {
    fn from(err: XmlParseError) -> DescribeClusterSecurityGroupsError {
        let XmlParseError(message) = err;
        DescribeClusterSecurityGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeClusterSecurityGroupsError {
    fn from(err: CredentialsError) -> DescribeClusterSecurityGroupsError {
        DescribeClusterSecurityGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterSecurityGroupsError {
    fn from(err: HttpDispatchError) -> DescribeClusterSecurityGroupsError {
        DescribeClusterSecurityGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterSecurityGroupsError {
    fn from(err: io::Error) -> DescribeClusterSecurityGroupsError {
        DescribeClusterSecurityGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterSecurityGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterSecurityGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterSecurityGroupsError::ClusterSecurityGroupNotFoundFault(ref cause) => {
                cause
            }
            DescribeClusterSecurityGroupsError::InvalidTagFault(ref cause) => cause,
            DescribeClusterSecurityGroupsError::Validation(ref cause) => cause,
            DescribeClusterSecurityGroupsError::Credentials(ref err) => err.description(),
            DescribeClusterSecurityGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeClusterSecurityGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusterSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeClusterSnapshotsError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The snapshot identifier does not refer to an existing cluster snapshot.</p>
    ClusterSnapshotNotFoundFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClusterSnapshotsError {
    pub fn from_body(body: &str) -> DescribeClusterSnapshotsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => DescribeClusterSnapshotsError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "ClusterSnapshotNotFound" => {
                    DescribeClusterSnapshotsError::ClusterSnapshotNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidTagFault" => DescribeClusterSnapshotsError::InvalidTagFault(String::from(
                    parsed_error.message,
                )),
                _ => DescribeClusterSnapshotsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeClusterSnapshotsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeClusterSnapshotsError {
    fn from(err: XmlParseError) -> DescribeClusterSnapshotsError {
        let XmlParseError(message) = err;
        DescribeClusterSnapshotsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeClusterSnapshotsError {
    fn from(err: CredentialsError) -> DescribeClusterSnapshotsError {
        DescribeClusterSnapshotsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterSnapshotsError {
    fn from(err: HttpDispatchError) -> DescribeClusterSnapshotsError {
        DescribeClusterSnapshotsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterSnapshotsError {
    fn from(err: io::Error) -> DescribeClusterSnapshotsError {
        DescribeClusterSnapshotsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterSnapshotsError::ClusterNotFoundFault(ref cause) => cause,
            DescribeClusterSnapshotsError::ClusterSnapshotNotFoundFault(ref cause) => cause,
            DescribeClusterSnapshotsError::InvalidTagFault(ref cause) => cause,
            DescribeClusterSnapshotsError::Validation(ref cause) => cause,
            DescribeClusterSnapshotsError::Credentials(ref err) => err.description(),
            DescribeClusterSnapshotsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeClusterSnapshotsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusterSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeClusterSubnetGroupsError {
    /// <p>The cluster subnet group name does not refer to an existing cluster subnet group.</p>
    ClusterSubnetGroupNotFoundFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClusterSubnetGroupsError {
    pub fn from_body(body: &str) -> DescribeClusterSubnetGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSubnetGroupNotFoundFault" => {
                    DescribeClusterSubnetGroupsError::ClusterSubnetGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidTagFault" => DescribeClusterSubnetGroupsError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeClusterSubnetGroupsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeClusterSubnetGroupsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeClusterSubnetGroupsError {
    fn from(err: XmlParseError) -> DescribeClusterSubnetGroupsError {
        let XmlParseError(message) = err;
        DescribeClusterSubnetGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeClusterSubnetGroupsError {
    fn from(err: CredentialsError) -> DescribeClusterSubnetGroupsError {
        DescribeClusterSubnetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterSubnetGroupsError {
    fn from(err: HttpDispatchError) -> DescribeClusterSubnetGroupsError {
        DescribeClusterSubnetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterSubnetGroupsError {
    fn from(err: io::Error) -> DescribeClusterSubnetGroupsError {
        DescribeClusterSubnetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterSubnetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterSubnetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterSubnetGroupsError::ClusterSubnetGroupNotFoundFault(ref cause) => cause,
            DescribeClusterSubnetGroupsError::InvalidTagFault(ref cause) => cause,
            DescribeClusterSubnetGroupsError::Validation(ref cause) => cause,
            DescribeClusterSubnetGroupsError::Credentials(ref err) => err.description(),
            DescribeClusterSubnetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeClusterSubnetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusterVersions
#[derive(Debug, PartialEq)]
pub enum DescribeClusterVersionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClusterVersionsError {
    pub fn from_body(body: &str) -> DescribeClusterVersionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeClusterVersionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeClusterVersionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeClusterVersionsError {
    fn from(err: XmlParseError) -> DescribeClusterVersionsError {
        let XmlParseError(message) = err;
        DescribeClusterVersionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeClusterVersionsError {
    fn from(err: CredentialsError) -> DescribeClusterVersionsError {
        DescribeClusterVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterVersionsError {
    fn from(err: HttpDispatchError) -> DescribeClusterVersionsError {
        DescribeClusterVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterVersionsError {
    fn from(err: io::Error) -> DescribeClusterVersionsError {
        DescribeClusterVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterVersionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterVersionsError::Validation(ref cause) => cause,
            DescribeClusterVersionsError::Credentials(ref err) => err.description(),
            DescribeClusterVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeClusterVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClustersError {
    pub fn from_body(body: &str) -> DescribeClustersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => {
                    DescribeClustersError::ClusterNotFoundFault(String::from(parsed_error.message))
                }
                "InvalidTagFault" => {
                    DescribeClustersError::InvalidTagFault(String::from(parsed_error.message))
                }
                _ => DescribeClustersError::Unknown(String::from(body)),
            },
            Err(_) => DescribeClustersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeClustersError {
    fn from(err: XmlParseError) -> DescribeClustersError {
        let XmlParseError(message) = err;
        DescribeClustersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeClustersError {
    fn from(err: CredentialsError) -> DescribeClustersError {
        DescribeClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClustersError {
    fn from(err: HttpDispatchError) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClustersError {
    fn from(err: io::Error) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClustersError::ClusterNotFoundFault(ref cause) => cause,
            DescribeClustersError::InvalidTagFault(ref cause) => cause,
            DescribeClustersError::Validation(ref cause) => cause,
            DescribeClustersError::Credentials(ref err) => err.description(),
            DescribeClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDefaultClusterParameters
#[derive(Debug, PartialEq)]
pub enum DescribeDefaultClusterParametersError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDefaultClusterParametersError {
    pub fn from_body(body: &str) -> DescribeDefaultClusterParametersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeDefaultClusterParametersError::Unknown(String::from(body)),
            },
            Err(_) => DescribeDefaultClusterParametersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeDefaultClusterParametersError {
    fn from(err: XmlParseError) -> DescribeDefaultClusterParametersError {
        let XmlParseError(message) = err;
        DescribeDefaultClusterParametersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeDefaultClusterParametersError {
    fn from(err: CredentialsError) -> DescribeDefaultClusterParametersError {
        DescribeDefaultClusterParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDefaultClusterParametersError {
    fn from(err: HttpDispatchError) -> DescribeDefaultClusterParametersError {
        DescribeDefaultClusterParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDefaultClusterParametersError {
    fn from(err: io::Error) -> DescribeDefaultClusterParametersError {
        DescribeDefaultClusterParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDefaultClusterParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDefaultClusterParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeDefaultClusterParametersError::Validation(ref cause) => cause,
            DescribeDefaultClusterParametersError::Credentials(ref err) => err.description(),
            DescribeDefaultClusterParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDefaultClusterParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventCategories
#[derive(Debug, PartialEq)]
pub enum DescribeEventCategoriesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEventCategoriesError {
    pub fn from_body(body: &str) -> DescribeEventCategoriesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeEventCategoriesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEventCategoriesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEventCategoriesError {
    fn from(err: XmlParseError) -> DescribeEventCategoriesError {
        let XmlParseError(message) = err;
        DescribeEventCategoriesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEventCategoriesError {
    fn from(err: CredentialsError) -> DescribeEventCategoriesError {
        DescribeEventCategoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventCategoriesError {
    fn from(err: HttpDispatchError) -> DescribeEventCategoriesError {
        DescribeEventCategoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventCategoriesError {
    fn from(err: io::Error) -> DescribeEventCategoriesError {
        DescribeEventCategoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventCategoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventCategoriesError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventCategoriesError::Validation(ref cause) => cause,
            DescribeEventCategoriesError::Credentials(ref err) => err.description(),
            DescribeEventCategoriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventCategoriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventSubscriptions
#[derive(Debug, PartialEq)]
pub enum DescribeEventSubscriptionsError {
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>An Amazon Redshift event notification subscription with the specified name does not exist.</p>
    SubscriptionNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEventSubscriptionsError {
    pub fn from_body(body: &str) -> DescribeEventSubscriptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTagFault" => DescribeEventSubscriptionsError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                "SubscriptionNotFound" => {
                    DescribeEventSubscriptionsError::SubscriptionNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeEventSubscriptionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEventSubscriptionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEventSubscriptionsError {
    fn from(err: XmlParseError) -> DescribeEventSubscriptionsError {
        let XmlParseError(message) = err;
        DescribeEventSubscriptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEventSubscriptionsError {
    fn from(err: CredentialsError) -> DescribeEventSubscriptionsError {
        DescribeEventSubscriptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventSubscriptionsError {
    fn from(err: HttpDispatchError) -> DescribeEventSubscriptionsError {
        DescribeEventSubscriptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventSubscriptionsError {
    fn from(err: io::Error) -> DescribeEventSubscriptionsError {
        DescribeEventSubscriptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventSubscriptionsError::InvalidTagFault(ref cause) => cause,
            DescribeEventSubscriptionsError::SubscriptionNotFoundFault(ref cause) => cause,
            DescribeEventSubscriptionsError::Validation(ref cause) => cause,
            DescribeEventSubscriptionsError::Credentials(ref err) => err.description(),
            DescribeEventSubscriptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventSubscriptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEventsError {
    pub fn from_body(body: &str) -> DescribeEventsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeEventsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEventsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeEventsError {
    fn from(err: XmlParseError) -> DescribeEventsError {
        let XmlParseError(message) = err;
        DescribeEventsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEventsError {
    fn from(err: CredentialsError) -> DescribeEventsError {
        DescribeEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventsError {
    fn from(err: HttpDispatchError) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventsError {
    fn from(err: io::Error) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeHsmClientCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeHsmClientCertificatesError {
    /// <p>There is no Amazon Redshift HSM client certificate with the specified identifier.</p>
    HsmClientCertificateNotFoundFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeHsmClientCertificatesError {
    pub fn from_body(body: &str) -> DescribeHsmClientCertificatesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HsmClientCertificateNotFoundFault" => {
                    DescribeHsmClientCertificatesError::HsmClientCertificateNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagFault" => DescribeHsmClientCertificatesError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeHsmClientCertificatesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeHsmClientCertificatesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeHsmClientCertificatesError {
    fn from(err: XmlParseError) -> DescribeHsmClientCertificatesError {
        let XmlParseError(message) = err;
        DescribeHsmClientCertificatesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeHsmClientCertificatesError {
    fn from(err: CredentialsError) -> DescribeHsmClientCertificatesError {
        DescribeHsmClientCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeHsmClientCertificatesError {
    fn from(err: HttpDispatchError) -> DescribeHsmClientCertificatesError {
        DescribeHsmClientCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeHsmClientCertificatesError {
    fn from(err: io::Error) -> DescribeHsmClientCertificatesError {
        DescribeHsmClientCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeHsmClientCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHsmClientCertificatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeHsmClientCertificatesError::HsmClientCertificateNotFoundFault(ref cause) => {
                cause
            }
            DescribeHsmClientCertificatesError::InvalidTagFault(ref cause) => cause,
            DescribeHsmClientCertificatesError::Validation(ref cause) => cause,
            DescribeHsmClientCertificatesError::Credentials(ref err) => err.description(),
            DescribeHsmClientCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeHsmClientCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeHsmConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeHsmConfigurationsError {
    /// <p>There is no Amazon Redshift HSM configuration with the specified identifier.</p>
    HsmConfigurationNotFoundFault(String),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeHsmConfigurationsError {
    pub fn from_body(body: &str) -> DescribeHsmConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HsmConfigurationNotFoundFault" => {
                    DescribeHsmConfigurationsError::HsmConfigurationNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidTagFault" => DescribeHsmConfigurationsError::InvalidTagFault(String::from(
                    parsed_error.message,
                )),
                _ => DescribeHsmConfigurationsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeHsmConfigurationsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeHsmConfigurationsError {
    fn from(err: XmlParseError) -> DescribeHsmConfigurationsError {
        let XmlParseError(message) = err;
        DescribeHsmConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeHsmConfigurationsError {
    fn from(err: CredentialsError) -> DescribeHsmConfigurationsError {
        DescribeHsmConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeHsmConfigurationsError {
    fn from(err: HttpDispatchError) -> DescribeHsmConfigurationsError {
        DescribeHsmConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeHsmConfigurationsError {
    fn from(err: io::Error) -> DescribeHsmConfigurationsError {
        DescribeHsmConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeHsmConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHsmConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeHsmConfigurationsError::HsmConfigurationNotFoundFault(ref cause) => cause,
            DescribeHsmConfigurationsError::InvalidTagFault(ref cause) => cause,
            DescribeHsmConfigurationsError::Validation(ref cause) => cause,
            DescribeHsmConfigurationsError::Credentials(ref err) => err.description(),
            DescribeHsmConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeHsmConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoggingStatus
#[derive(Debug, PartialEq)]
pub enum DescribeLoggingStatusError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLoggingStatusError {
    pub fn from_body(body: &str) -> DescribeLoggingStatusError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => DescribeLoggingStatusError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeLoggingStatusError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLoggingStatusError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeLoggingStatusError {
    fn from(err: XmlParseError) -> DescribeLoggingStatusError {
        let XmlParseError(message) = err;
        DescribeLoggingStatusError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLoggingStatusError {
    fn from(err: CredentialsError) -> DescribeLoggingStatusError {
        DescribeLoggingStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoggingStatusError {
    fn from(err: HttpDispatchError) -> DescribeLoggingStatusError {
        DescribeLoggingStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoggingStatusError {
    fn from(err: io::Error) -> DescribeLoggingStatusError {
        DescribeLoggingStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoggingStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoggingStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoggingStatusError::ClusterNotFoundFault(ref cause) => cause,
            DescribeLoggingStatusError::Validation(ref cause) => cause,
            DescribeLoggingStatusError::Credentials(ref err) => err.description(),
            DescribeLoggingStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoggingStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOrderableClusterOptions
#[derive(Debug, PartialEq)]
pub enum DescribeOrderableClusterOptionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeOrderableClusterOptionsError {
    pub fn from_body(body: &str) -> DescribeOrderableClusterOptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeOrderableClusterOptionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeOrderableClusterOptionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeOrderableClusterOptionsError {
    fn from(err: XmlParseError) -> DescribeOrderableClusterOptionsError {
        let XmlParseError(message) = err;
        DescribeOrderableClusterOptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeOrderableClusterOptionsError {
    fn from(err: CredentialsError) -> DescribeOrderableClusterOptionsError {
        DescribeOrderableClusterOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeOrderableClusterOptionsError {
    fn from(err: HttpDispatchError) -> DescribeOrderableClusterOptionsError {
        DescribeOrderableClusterOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeOrderableClusterOptionsError {
    fn from(err: io::Error) -> DescribeOrderableClusterOptionsError {
        DescribeOrderableClusterOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeOrderableClusterOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrderableClusterOptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrderableClusterOptionsError::Validation(ref cause) => cause,
            DescribeOrderableClusterOptionsError::Credentials(ref err) => err.description(),
            DescribeOrderableClusterOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeOrderableClusterOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReservedNodeOfferings
#[derive(Debug, PartialEq)]
pub enum DescribeReservedNodeOfferingsError {
    /// <p>Your request cannot be completed because a dependent internal service is temporarily unavailable. Wait 30 to 60 seconds and try again.</p>
    DependentServiceUnavailableFault(String),
    /// <p>Specified offering does not exist.</p>
    ReservedNodeOfferingNotFoundFault(String),
    /// <p>The requested operation isn't supported.</p>
    UnsupportedOperationFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeReservedNodeOfferingsError {
    pub fn from_body(body: &str) -> DescribeReservedNodeOfferingsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DependentServiceUnavailableFault" => {
                    DescribeReservedNodeOfferingsError::DependentServiceUnavailableFault(
                        String::from(parsed_error.message),
                    )
                }
                "ReservedNodeOfferingNotFound" => {
                    DescribeReservedNodeOfferingsError::ReservedNodeOfferingNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "UnsupportedOperation" => {
                    DescribeReservedNodeOfferingsError::UnsupportedOperationFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeReservedNodeOfferingsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeReservedNodeOfferingsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeReservedNodeOfferingsError {
    fn from(err: XmlParseError) -> DescribeReservedNodeOfferingsError {
        let XmlParseError(message) = err;
        DescribeReservedNodeOfferingsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeReservedNodeOfferingsError {
    fn from(err: CredentialsError) -> DescribeReservedNodeOfferingsError {
        DescribeReservedNodeOfferingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReservedNodeOfferingsError {
    fn from(err: HttpDispatchError) -> DescribeReservedNodeOfferingsError {
        DescribeReservedNodeOfferingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeReservedNodeOfferingsError {
    fn from(err: io::Error) -> DescribeReservedNodeOfferingsError {
        DescribeReservedNodeOfferingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeReservedNodeOfferingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReservedNodeOfferingsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReservedNodeOfferingsError::DependentServiceUnavailableFault(ref cause) => {
                cause
            }
            DescribeReservedNodeOfferingsError::ReservedNodeOfferingNotFoundFault(ref cause) => {
                cause
            }
            DescribeReservedNodeOfferingsError::UnsupportedOperationFault(ref cause) => cause,
            DescribeReservedNodeOfferingsError::Validation(ref cause) => cause,
            DescribeReservedNodeOfferingsError::Credentials(ref err) => err.description(),
            DescribeReservedNodeOfferingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReservedNodeOfferingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReservedNodes
#[derive(Debug, PartialEq)]
pub enum DescribeReservedNodesError {
    /// <p>Your request cannot be completed because a dependent internal service is temporarily unavailable. Wait 30 to 60 seconds and try again.</p>
    DependentServiceUnavailableFault(String),
    /// <p>The specified reserved compute node not found.</p>
    ReservedNodeNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeReservedNodesError {
    pub fn from_body(body: &str) -> DescribeReservedNodesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DependentServiceUnavailableFault" => {
                    DescribeReservedNodesError::DependentServiceUnavailableFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ReservedNodeNotFound" => DescribeReservedNodesError::ReservedNodeNotFoundFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeReservedNodesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeReservedNodesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeReservedNodesError {
    fn from(err: XmlParseError) -> DescribeReservedNodesError {
        let XmlParseError(message) = err;
        DescribeReservedNodesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeReservedNodesError {
    fn from(err: CredentialsError) -> DescribeReservedNodesError {
        DescribeReservedNodesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReservedNodesError {
    fn from(err: HttpDispatchError) -> DescribeReservedNodesError {
        DescribeReservedNodesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeReservedNodesError {
    fn from(err: io::Error) -> DescribeReservedNodesError {
        DescribeReservedNodesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeReservedNodesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReservedNodesError {
    fn description(&self) -> &str {
        match *self {
            DescribeReservedNodesError::DependentServiceUnavailableFault(ref cause) => cause,
            DescribeReservedNodesError::ReservedNodeNotFoundFault(ref cause) => cause,
            DescribeReservedNodesError::Validation(ref cause) => cause,
            DescribeReservedNodesError::Credentials(ref err) => err.description(),
            DescribeReservedNodesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReservedNodesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeResize
#[derive(Debug, PartialEq)]
pub enum DescribeResizeError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>A resize operation for the specified cluster is not found.</p>
    ResizeNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeResizeError {
    pub fn from_body(body: &str) -> DescribeResizeError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => {
                    DescribeResizeError::ClusterNotFoundFault(String::from(parsed_error.message))
                }
                "ResizeNotFound" => {
                    DescribeResizeError::ResizeNotFoundFault(String::from(parsed_error.message))
                }
                _ => DescribeResizeError::Unknown(String::from(body)),
            },
            Err(_) => DescribeResizeError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeResizeError {
    fn from(err: XmlParseError) -> DescribeResizeError {
        let XmlParseError(message) = err;
        DescribeResizeError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeResizeError {
    fn from(err: CredentialsError) -> DescribeResizeError {
        DescribeResizeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeResizeError {
    fn from(err: HttpDispatchError) -> DescribeResizeError {
        DescribeResizeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeResizeError {
    fn from(err: io::Error) -> DescribeResizeError {
        DescribeResizeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeResizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeResizeError {
    fn description(&self) -> &str {
        match *self {
            DescribeResizeError::ClusterNotFoundFault(ref cause) => cause,
            DescribeResizeError::ResizeNotFoundFault(ref cause) => cause,
            DescribeResizeError::Validation(ref cause) => cause,
            DescribeResizeError::Credentials(ref err) => err.description(),
            DescribeResizeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeResizeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSnapshotCopyGrants
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotCopyGrantsError {
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The specified snapshot copy grant can't be found. Make sure that the name is typed correctly and that the grant exists in the destination region.</p>
    SnapshotCopyGrantNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSnapshotCopyGrantsError {
    pub fn from_body(body: &str) -> DescribeSnapshotCopyGrantsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTagFault" => DescribeSnapshotCopyGrantsError::InvalidTagFault(
                    String::from(parsed_error.message),
                ),
                "SnapshotCopyGrantNotFoundFault" => {
                    DescribeSnapshotCopyGrantsError::SnapshotCopyGrantNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeSnapshotCopyGrantsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeSnapshotCopyGrantsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeSnapshotCopyGrantsError {
    fn from(err: XmlParseError) -> DescribeSnapshotCopyGrantsError {
        let XmlParseError(message) = err;
        DescribeSnapshotCopyGrantsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeSnapshotCopyGrantsError {
    fn from(err: CredentialsError) -> DescribeSnapshotCopyGrantsError {
        DescribeSnapshotCopyGrantsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSnapshotCopyGrantsError {
    fn from(err: HttpDispatchError) -> DescribeSnapshotCopyGrantsError {
        DescribeSnapshotCopyGrantsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSnapshotCopyGrantsError {
    fn from(err: io::Error) -> DescribeSnapshotCopyGrantsError {
        DescribeSnapshotCopyGrantsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSnapshotCopyGrantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSnapshotCopyGrantsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSnapshotCopyGrantsError::InvalidTagFault(ref cause) => cause,
            DescribeSnapshotCopyGrantsError::SnapshotCopyGrantNotFoundFault(ref cause) => cause,
            DescribeSnapshotCopyGrantsError::Validation(ref cause) => cause,
            DescribeSnapshotCopyGrantsError::Credentials(ref err) => err.description(),
            DescribeSnapshotCopyGrantsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSnapshotCopyGrantsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTableRestoreStatus
#[derive(Debug, PartialEq)]
pub enum DescribeTableRestoreStatusError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The specified <code>TableRestoreRequestId</code> value was not found.</p>
    TableRestoreNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTableRestoreStatusError {
    pub fn from_body(body: &str) -> DescribeTableRestoreStatusError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => DescribeTableRestoreStatusError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "TableRestoreNotFoundFault" => {
                    DescribeTableRestoreStatusError::TableRestoreNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeTableRestoreStatusError::Unknown(String::from(body)),
            },
            Err(_) => DescribeTableRestoreStatusError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeTableRestoreStatusError {
    fn from(err: XmlParseError) -> DescribeTableRestoreStatusError {
        let XmlParseError(message) = err;
        DescribeTableRestoreStatusError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeTableRestoreStatusError {
    fn from(err: CredentialsError) -> DescribeTableRestoreStatusError {
        DescribeTableRestoreStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTableRestoreStatusError {
    fn from(err: HttpDispatchError) -> DescribeTableRestoreStatusError {
        DescribeTableRestoreStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTableRestoreStatusError {
    fn from(err: io::Error) -> DescribeTableRestoreStatusError {
        DescribeTableRestoreStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTableRestoreStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTableRestoreStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeTableRestoreStatusError::ClusterNotFoundFault(ref cause) => cause,
            DescribeTableRestoreStatusError::TableRestoreNotFoundFault(ref cause) => cause,
            DescribeTableRestoreStatusError::Validation(ref cause) => cause,
            DescribeTableRestoreStatusError::Credentials(ref err) => err.description(),
            DescribeTableRestoreStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTableRestoreStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The tag is invalid.</p>
    InvalidTagFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTagsError {
    pub fn from_body(body: &str) -> DescribeTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTagFault" => {
                    DescribeTagsError::InvalidTagFault(String::from(parsed_error.message))
                }
                "ResourceNotFoundFault" => {
                    DescribeTagsError::ResourceNotFoundFault(String::from(parsed_error.message))
                }
                _ => DescribeTagsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeTagsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeTagsError {
    fn from(err: XmlParseError) -> DescribeTagsError {
        let XmlParseError(message) = err;
        DescribeTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeTagsError {
    fn from(err: CredentialsError) -> DescribeTagsError {
        DescribeTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagsError {
    fn from(err: HttpDispatchError) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagsError {
    fn from(err: io::Error) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::InvalidTagFault(ref cause) => cause,
            DescribeTagsError::ResourceNotFoundFault(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableLogging
#[derive(Debug, PartialEq)]
pub enum DisableLoggingError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableLoggingError {
    pub fn from_body(body: &str) -> DisableLoggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => {
                    DisableLoggingError::ClusterNotFoundFault(String::from(parsed_error.message))
                }
                _ => DisableLoggingError::Unknown(String::from(body)),
            },
            Err(_) => DisableLoggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DisableLoggingError {
    fn from(err: XmlParseError) -> DisableLoggingError {
        let XmlParseError(message) = err;
        DisableLoggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DisableLoggingError {
    fn from(err: CredentialsError) -> DisableLoggingError {
        DisableLoggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableLoggingError {
    fn from(err: HttpDispatchError) -> DisableLoggingError {
        DisableLoggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableLoggingError {
    fn from(err: io::Error) -> DisableLoggingError {
        DisableLoggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableLoggingError {
    fn description(&self) -> &str {
        match *self {
            DisableLoggingError::ClusterNotFoundFault(ref cause) => cause,
            DisableLoggingError::Validation(ref cause) => cause,
            DisableLoggingError::Credentials(ref err) => err.description(),
            DisableLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableLoggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableSnapshotCopy
#[derive(Debug, PartialEq)]
pub enum DisableSnapshotCopyError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// <p>The cluster already has cross-region snapshot copy disabled.</p>
    SnapshotCopyAlreadyDisabledFault(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableSnapshotCopyError {
    pub fn from_body(body: &str) -> DisableSnapshotCopyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => DisableSnapshotCopyError::ClusterNotFoundFault(String::from(
                    parsed_error.message,
                )),
                "InvalidClusterState" => DisableSnapshotCopyError::InvalidClusterStateFault(
                    String::from(parsed_error.message),
                ),
                "SnapshotCopyAlreadyDisabledFault" => {
                    DisableSnapshotCopyError::SnapshotCopyAlreadyDisabledFault(String::from(
                        parsed_error.message,
                    ))
                }
                "UnauthorizedOperation" => DisableSnapshotCopyError::UnauthorizedOperation(
                    String::from(parsed_error.message),
                ),
                _ => DisableSnapshotCopyError::Unknown(String::from(body)),
            },
            Err(_) => DisableSnapshotCopyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DisableSnapshotCopyError {
    fn from(err: XmlParseError) -> DisableSnapshotCopyError {
        let XmlParseError(message) = err;
        DisableSnapshotCopyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DisableSnapshotCopyError {
    fn from(err: CredentialsError) -> DisableSnapshotCopyError {
        DisableSnapshotCopyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableSnapshotCopyError {
    fn from(err: HttpDispatchError) -> DisableSnapshotCopyError {
        DisableSnapshotCopyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableSnapshotCopyError {
    fn from(err: io::Error) -> DisableSnapshotCopyError {
        DisableSnapshotCopyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableSnapshotCopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableSnapshotCopyError {
    fn description(&self) -> &str {
        match *self {
            DisableSnapshotCopyError::ClusterNotFoundFault(ref cause) => cause,
            DisableSnapshotCopyError::InvalidClusterStateFault(ref cause) => cause,
            DisableSnapshotCopyError::SnapshotCopyAlreadyDisabledFault(ref cause) => cause,
            DisableSnapshotCopyError::UnauthorizedOperation(ref cause) => cause,
            DisableSnapshotCopyError::Validation(ref cause) => cause,
            DisableSnapshotCopyError::Credentials(ref err) => err.description(),
            DisableSnapshotCopyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableSnapshotCopyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableLogging
#[derive(Debug, PartialEq)]
pub enum EnableLoggingError {
    /// <p>Could not find the specified S3 bucket.</p>
    BucketNotFoundFault(String),
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The cluster does not have read bucket or put object permissions on the S3 bucket specified when enabling logging.</p>
    InsufficientS3BucketPolicyFault(String),
    /// <p>The S3 bucket name is invalid. For more information about naming rules, go to <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html">Bucket Restrictions and Limitations</a> in the Amazon Simple Storage Service (S3) Developer Guide.</p>
    InvalidS3BucketNameFault(String),
    /// <p>The string specified for the logging S3 key prefix does not comply with the documented constraints.</p>
    InvalidS3KeyPrefixFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableLoggingError {
    pub fn from_body(body: &str) -> EnableLoggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BucketNotFoundFault" => {
                    EnableLoggingError::BucketNotFoundFault(String::from(parsed_error.message))
                }
                "ClusterNotFound" => {
                    EnableLoggingError::ClusterNotFoundFault(String::from(parsed_error.message))
                }
                "InsufficientS3BucketPolicyFault" => {
                    EnableLoggingError::InsufficientS3BucketPolicyFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidS3BucketNameFault" => {
                    EnableLoggingError::InvalidS3BucketNameFault(String::from(parsed_error.message))
                }
                "InvalidS3KeyPrefixFault" => {
                    EnableLoggingError::InvalidS3KeyPrefixFault(String::from(parsed_error.message))
                }
                _ => EnableLoggingError::Unknown(String::from(body)),
            },
            Err(_) => EnableLoggingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for EnableLoggingError {
    fn from(err: XmlParseError) -> EnableLoggingError {
        let XmlParseError(message) = err;
        EnableLoggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for EnableLoggingError {
    fn from(err: CredentialsError) -> EnableLoggingError {
        EnableLoggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableLoggingError {
    fn from(err: HttpDispatchError) -> EnableLoggingError {
        EnableLoggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableLoggingError {
    fn from(err: io::Error) -> EnableLoggingError {
        EnableLoggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableLoggingError {
    fn description(&self) -> &str {
        match *self {
            EnableLoggingError::BucketNotFoundFault(ref cause) => cause,
            EnableLoggingError::ClusterNotFoundFault(ref cause) => cause,
            EnableLoggingError::InsufficientS3BucketPolicyFault(ref cause) => cause,
            EnableLoggingError::InvalidS3BucketNameFault(ref cause) => cause,
            EnableLoggingError::InvalidS3KeyPrefixFault(ref cause) => cause,
            EnableLoggingError::Validation(ref cause) => cause,
            EnableLoggingError::Credentials(ref err) => err.description(),
            EnableLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableLoggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableSnapshotCopy
#[derive(Debug, PartialEq)]
pub enum EnableSnapshotCopyError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>Cross-region snapshot copy was temporarily disabled. Try your request again.</p>
    CopyToRegionDisabledFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>The specified options are incompatible.</p>
    IncompatibleOrderableOptions(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// <p>The encryption key has exceeded its grant limit in AWS KMS.</p>
    LimitExceededFault(String),
    /// <p>The cluster already has cross-region snapshot copy enabled.</p>
    SnapshotCopyAlreadyEnabledFault(String),
    /// <p>The specified snapshot copy grant can't be found. Make sure that the name is typed correctly and that the grant exists in the destination region.</p>
    SnapshotCopyGrantNotFoundFault(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// <p>The specified region is incorrect or does not exist.</p>
    UnknownSnapshotCopyRegionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableSnapshotCopyError {
    pub fn from_body(body: &str) -> EnableSnapshotCopyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => EnableSnapshotCopyError::ClusterNotFoundFault(String::from(
                    parsed_error.message,
                )),
                "CopyToRegionDisabledFault" => EnableSnapshotCopyError::CopyToRegionDisabledFault(
                    String::from(parsed_error.message),
                ),
                "DependentServiceRequestThrottlingFault" => {
                    EnableSnapshotCopyError::DependentServiceRequestThrottlingFault(String::from(
                        parsed_error.message,
                    ))
                }
                "IncompatibleOrderableOptions" => {
                    EnableSnapshotCopyError::IncompatibleOrderableOptions(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterState" => EnableSnapshotCopyError::InvalidClusterStateFault(
                    String::from(parsed_error.message),
                ),
                "LimitExceededFault" => {
                    EnableSnapshotCopyError::LimitExceededFault(String::from(parsed_error.message))
                }
                "SnapshotCopyAlreadyEnabledFault" => {
                    EnableSnapshotCopyError::SnapshotCopyAlreadyEnabledFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SnapshotCopyGrantNotFoundFault" => {
                    EnableSnapshotCopyError::SnapshotCopyGrantNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "UnauthorizedOperation" => EnableSnapshotCopyError::UnauthorizedOperation(
                    String::from(parsed_error.message),
                ),
                "UnknownSnapshotCopyRegionFault" => {
                    EnableSnapshotCopyError::UnknownSnapshotCopyRegionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => EnableSnapshotCopyError::Unknown(String::from(body)),
            },
            Err(_) => EnableSnapshotCopyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for EnableSnapshotCopyError {
    fn from(err: XmlParseError) -> EnableSnapshotCopyError {
        let XmlParseError(message) = err;
        EnableSnapshotCopyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for EnableSnapshotCopyError {
    fn from(err: CredentialsError) -> EnableSnapshotCopyError {
        EnableSnapshotCopyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableSnapshotCopyError {
    fn from(err: HttpDispatchError) -> EnableSnapshotCopyError {
        EnableSnapshotCopyError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableSnapshotCopyError {
    fn from(err: io::Error) -> EnableSnapshotCopyError {
        EnableSnapshotCopyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableSnapshotCopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableSnapshotCopyError {
    fn description(&self) -> &str {
        match *self {
            EnableSnapshotCopyError::ClusterNotFoundFault(ref cause) => cause,
            EnableSnapshotCopyError::CopyToRegionDisabledFault(ref cause) => cause,
            EnableSnapshotCopyError::DependentServiceRequestThrottlingFault(ref cause) => cause,
            EnableSnapshotCopyError::IncompatibleOrderableOptions(ref cause) => cause,
            EnableSnapshotCopyError::InvalidClusterStateFault(ref cause) => cause,
            EnableSnapshotCopyError::LimitExceededFault(ref cause) => cause,
            EnableSnapshotCopyError::SnapshotCopyAlreadyEnabledFault(ref cause) => cause,
            EnableSnapshotCopyError::SnapshotCopyGrantNotFoundFault(ref cause) => cause,
            EnableSnapshotCopyError::UnauthorizedOperation(ref cause) => cause,
            EnableSnapshotCopyError::UnknownSnapshotCopyRegionFault(ref cause) => cause,
            EnableSnapshotCopyError::Validation(ref cause) => cause,
            EnableSnapshotCopyError::Credentials(ref err) => err.description(),
            EnableSnapshotCopyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableSnapshotCopyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClusterCredentials
#[derive(Debug, PartialEq)]
pub enum GetClusterCredentialsError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The requested operation isn't supported.</p>
    UnsupportedOperationFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetClusterCredentialsError {
    pub fn from_body(body: &str) -> GetClusterCredentialsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => GetClusterCredentialsError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "UnsupportedOperation" => GetClusterCredentialsError::UnsupportedOperationFault(
                    String::from(parsed_error.message),
                ),
                _ => GetClusterCredentialsError::Unknown(String::from(body)),
            },
            Err(_) => GetClusterCredentialsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetClusterCredentialsError {
    fn from(err: XmlParseError) -> GetClusterCredentialsError {
        let XmlParseError(message) = err;
        GetClusterCredentialsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetClusterCredentialsError {
    fn from(err: CredentialsError) -> GetClusterCredentialsError {
        GetClusterCredentialsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetClusterCredentialsError {
    fn from(err: HttpDispatchError) -> GetClusterCredentialsError {
        GetClusterCredentialsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetClusterCredentialsError {
    fn from(err: io::Error) -> GetClusterCredentialsError {
        GetClusterCredentialsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetClusterCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClusterCredentialsError {
    fn description(&self) -> &str {
        match *self {
            GetClusterCredentialsError::ClusterNotFoundFault(ref cause) => cause,
            GetClusterCredentialsError::UnsupportedOperationFault(ref cause) => cause,
            GetClusterCredentialsError::Validation(ref cause) => cause,
            GetClusterCredentialsError::Credentials(ref err) => err.description(),
            GetClusterCredentialsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetClusterCredentialsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyCluster
#[derive(Debug, PartialEq)]
pub enum ModifyClusterError {
    /// <p>The account already has a cluster with the given identifier.</p>
    ClusterAlreadyExistsFault(String),
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// <p>The cluster security group name does not refer to an existing cluster security group.</p>
    ClusterSecurityGroupNotFoundFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>There is no Amazon Redshift HSM client certificate with the specified identifier.</p>
    HsmClientCertificateNotFoundFault(String),
    /// <p>There is no Amazon Redshift HSM configuration with the specified identifier.</p>
    HsmConfigurationNotFoundFault(String),
    /// <p>The number of nodes specified exceeds the allotted capacity of the cluster.</p>
    InsufficientClusterCapacityFault(String),
    /// <p>The state of the cluster security group is not <code>available</code>. </p>
    InvalidClusterSecurityGroupStateFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// <p>The Elastic IP (EIP) is invalid or cannot be found.</p>
    InvalidElasticIpFault(String),
    /// <p>The encryption key has exceeded its grant limit in AWS KMS.</p>
    LimitExceededFault(String),
    /// <p>The operation would exceed the number of nodes allowed for a cluster.</p>
    NumberOfNodesPerClusterLimitExceededFault(String),
    /// <p>The operation would exceed the number of nodes allotted to the account. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    NumberOfNodesQuotaExceededFault(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// <p>A request option was specified that is not supported.</p>
    UnsupportedOptionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyClusterError {
    pub fn from_body(body: &str) -> ModifyClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterAlreadyExists" => ModifyClusterError::ClusterAlreadyExistsFault(
                    String::from(parsed_error.message),
                ),
                "ClusterNotFound" => {
                    ModifyClusterError::ClusterNotFoundFault(String::from(parsed_error.message))
                }
                "ClusterParameterGroupNotFound" => {
                    ModifyClusterError::ClusterParameterGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSecurityGroupNotFound" => {
                    ModifyClusterError::ClusterSecurityGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "DependentServiceRequestThrottlingFault" => {
                    ModifyClusterError::DependentServiceRequestThrottlingFault(String::from(
                        parsed_error.message,
                    ))
                }
                "HsmClientCertificateNotFoundFault" => {
                    ModifyClusterError::HsmClientCertificateNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "HsmConfigurationNotFoundFault" => {
                    ModifyClusterError::HsmConfigurationNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientClusterCapacity" => {
                    ModifyClusterError::InsufficientClusterCapacityFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterSecurityGroupState" => {
                    ModifyClusterError::InvalidClusterSecurityGroupStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterState" => {
                    ModifyClusterError::InvalidClusterStateFault(String::from(parsed_error.message))
                }
                "InvalidElasticIpFault" => {
                    ModifyClusterError::InvalidElasticIpFault(String::from(parsed_error.message))
                }
                "LimitExceededFault" => {
                    ModifyClusterError::LimitExceededFault(String::from(parsed_error.message))
                }
                "NumberOfNodesPerClusterLimitExceeded" => {
                    ModifyClusterError::NumberOfNodesPerClusterLimitExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "NumberOfNodesQuotaExceeded" => {
                    ModifyClusterError::NumberOfNodesQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "UnauthorizedOperation" => {
                    ModifyClusterError::UnauthorizedOperation(String::from(parsed_error.message))
                }
                "UnsupportedOptionFault" => {
                    ModifyClusterError::UnsupportedOptionFault(String::from(parsed_error.message))
                }
                _ => ModifyClusterError::Unknown(String::from(body)),
            },
            Err(_) => ModifyClusterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyClusterError {
    fn from(err: XmlParseError) -> ModifyClusterError {
        let XmlParseError(message) = err;
        ModifyClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyClusterError {
    fn from(err: CredentialsError) -> ModifyClusterError {
        ModifyClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyClusterError {
    fn from(err: HttpDispatchError) -> ModifyClusterError {
        ModifyClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyClusterError {
    fn from(err: io::Error) -> ModifyClusterError {
        ModifyClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyClusterError {
    fn description(&self) -> &str {
        match *self {
            ModifyClusterError::ClusterAlreadyExistsFault(ref cause) => cause,
            ModifyClusterError::ClusterNotFoundFault(ref cause) => cause,
            ModifyClusterError::ClusterParameterGroupNotFoundFault(ref cause) => cause,
            ModifyClusterError::ClusterSecurityGroupNotFoundFault(ref cause) => cause,
            ModifyClusterError::DependentServiceRequestThrottlingFault(ref cause) => cause,
            ModifyClusterError::HsmClientCertificateNotFoundFault(ref cause) => cause,
            ModifyClusterError::HsmConfigurationNotFoundFault(ref cause) => cause,
            ModifyClusterError::InsufficientClusterCapacityFault(ref cause) => cause,
            ModifyClusterError::InvalidClusterSecurityGroupStateFault(ref cause) => cause,
            ModifyClusterError::InvalidClusterStateFault(ref cause) => cause,
            ModifyClusterError::InvalidElasticIpFault(ref cause) => cause,
            ModifyClusterError::LimitExceededFault(ref cause) => cause,
            ModifyClusterError::NumberOfNodesPerClusterLimitExceededFault(ref cause) => cause,
            ModifyClusterError::NumberOfNodesQuotaExceededFault(ref cause) => cause,
            ModifyClusterError::UnauthorizedOperation(ref cause) => cause,
            ModifyClusterError::UnsupportedOptionFault(ref cause) => cause,
            ModifyClusterError::Validation(ref cause) => cause,
            ModifyClusterError::Credentials(ref err) => err.description(),
            ModifyClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ModifyClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyClusterIamRoles
#[derive(Debug, PartialEq)]
pub enum ModifyClusterIamRolesError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyClusterIamRolesError {
    pub fn from_body(body: &str) -> ModifyClusterIamRolesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => ModifyClusterIamRolesError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "InvalidClusterState" => ModifyClusterIamRolesError::InvalidClusterStateFault(
                    String::from(parsed_error.message),
                ),
                _ => ModifyClusterIamRolesError::Unknown(String::from(body)),
            },
            Err(_) => ModifyClusterIamRolesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyClusterIamRolesError {
    fn from(err: XmlParseError) -> ModifyClusterIamRolesError {
        let XmlParseError(message) = err;
        ModifyClusterIamRolesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyClusterIamRolesError {
    fn from(err: CredentialsError) -> ModifyClusterIamRolesError {
        ModifyClusterIamRolesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyClusterIamRolesError {
    fn from(err: HttpDispatchError) -> ModifyClusterIamRolesError {
        ModifyClusterIamRolesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyClusterIamRolesError {
    fn from(err: io::Error) -> ModifyClusterIamRolesError {
        ModifyClusterIamRolesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyClusterIamRolesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyClusterIamRolesError {
    fn description(&self) -> &str {
        match *self {
            ModifyClusterIamRolesError::ClusterNotFoundFault(ref cause) => cause,
            ModifyClusterIamRolesError::InvalidClusterStateFault(ref cause) => cause,
            ModifyClusterIamRolesError::Validation(ref cause) => cause,
            ModifyClusterIamRolesError::Credentials(ref err) => err.description(),
            ModifyClusterIamRolesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyClusterIamRolesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum ModifyClusterParameterGroupError {
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// <p>The cluster parameter group action can not be completed because another task is in progress that involves the parameter group. Wait a few moments and try the operation again.</p>
    InvalidClusterParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyClusterParameterGroupError {
    pub fn from_body(body: &str) -> ModifyClusterParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterParameterGroupNotFound" => {
                    ModifyClusterParameterGroupError::ClusterParameterGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterParameterGroupState" => {
                    ModifyClusterParameterGroupError::InvalidClusterParameterGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => ModifyClusterParameterGroupError::Unknown(String::from(body)),
            },
            Err(_) => ModifyClusterParameterGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyClusterParameterGroupError {
    fn from(err: XmlParseError) -> ModifyClusterParameterGroupError {
        let XmlParseError(message) = err;
        ModifyClusterParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyClusterParameterGroupError {
    fn from(err: CredentialsError) -> ModifyClusterParameterGroupError {
        ModifyClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> ModifyClusterParameterGroupError {
        ModifyClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyClusterParameterGroupError {
    fn from(err: io::Error) -> ModifyClusterParameterGroupError {
        ModifyClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyClusterParameterGroupError::ClusterParameterGroupNotFoundFault(ref cause) => {
                cause
            }
            ModifyClusterParameterGroupError::InvalidClusterParameterGroupStateFault(ref cause) => {
                cause
            }
            ModifyClusterParameterGroupError::Validation(ref cause) => cause,
            ModifyClusterParameterGroupError::Credentials(ref err) => err.description(),
            ModifyClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyClusterParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyClusterSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyClusterSubnetGroupError {
    /// <p>The cluster subnet group name does not refer to an existing cluster subnet group.</p>
    ClusterSubnetGroupNotFoundFault(String),
    /// <p>The request would result in user exceeding the allowed number of subnets in a cluster subnet groups. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterSubnetQuotaExceededFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>The requested subnet is not valid, or not all of the subnets are in the same VPC.</p>
    InvalidSubnet(String),
    /// <p>A specified subnet is already in use by another cluster.</p>
    SubnetAlreadyInUse(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyClusterSubnetGroupError {
    pub fn from_body(body: &str) -> ModifyClusterSubnetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterSubnetGroupNotFoundFault" => {
                    ModifyClusterSubnetGroupError::ClusterSubnetGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSubnetQuotaExceededFault" => {
                    ModifyClusterSubnetGroupError::ClusterSubnetQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "DependentServiceRequestThrottlingFault" => {
                    ModifyClusterSubnetGroupError::DependentServiceRequestThrottlingFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidSubnet" => {
                    ModifyClusterSubnetGroupError::InvalidSubnet(String::from(parsed_error.message))
                }
                "SubnetAlreadyInUse" => ModifyClusterSubnetGroupError::SubnetAlreadyInUse(
                    String::from(parsed_error.message),
                ),
                "UnauthorizedOperation" => ModifyClusterSubnetGroupError::UnauthorizedOperation(
                    String::from(parsed_error.message),
                ),
                _ => ModifyClusterSubnetGroupError::Unknown(String::from(body)),
            },
            Err(_) => ModifyClusterSubnetGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyClusterSubnetGroupError {
    fn from(err: XmlParseError) -> ModifyClusterSubnetGroupError {
        let XmlParseError(message) = err;
        ModifyClusterSubnetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyClusterSubnetGroupError {
    fn from(err: CredentialsError) -> ModifyClusterSubnetGroupError {
        ModifyClusterSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyClusterSubnetGroupError {
    fn from(err: HttpDispatchError) -> ModifyClusterSubnetGroupError {
        ModifyClusterSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyClusterSubnetGroupError {
    fn from(err: io::Error) -> ModifyClusterSubnetGroupError {
        ModifyClusterSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyClusterSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyClusterSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyClusterSubnetGroupError::ClusterSubnetGroupNotFoundFault(ref cause) => cause,
            ModifyClusterSubnetGroupError::ClusterSubnetQuotaExceededFault(ref cause) => cause,
            ModifyClusterSubnetGroupError::DependentServiceRequestThrottlingFault(ref cause) => {
                cause
            }
            ModifyClusterSubnetGroupError::InvalidSubnet(ref cause) => cause,
            ModifyClusterSubnetGroupError::SubnetAlreadyInUse(ref cause) => cause,
            ModifyClusterSubnetGroupError::UnauthorizedOperation(ref cause) => cause,
            ModifyClusterSubnetGroupError::Validation(ref cause) => cause,
            ModifyClusterSubnetGroupError::Credentials(ref err) => err.description(),
            ModifyClusterSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyClusterSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyEventSubscription
#[derive(Debug, PartialEq)]
pub enum ModifyEventSubscriptionError {
    /// <p>The subscription request is invalid because it is a duplicate request. This subscription request is already in progress.</p>
    InvalidSubscriptionStateFault(String),
    /// <p>Amazon SNS has responded that there is a problem with the specified Amazon SNS topic.</p>
    SNSInvalidTopicFault(String),
    /// <p>You do not have permission to publish to the specified Amazon SNS topic.</p>
    SNSNoAuthorizationFault(String),
    /// <p>An Amazon SNS topic with the specified Amazon Resource Name (ARN) does not exist.</p>
    SNSTopicArnNotFoundFault(String),
    /// <p>The specified Amazon Redshift event source could not be found.</p>
    SourceNotFoundFault(String),
    /// <p>The value specified for the event category was not one of the allowed values, or it specified a category that does not apply to the specified source type. The allowed values are Configuration, Management, Monitoring, and Security.</p>
    SubscriptionCategoryNotFoundFault(String),
    /// <p>An Amazon Redshift event with the specified event ID does not exist.</p>
    SubscriptionEventIdNotFoundFault(String),
    /// <p>An Amazon Redshift event notification subscription with the specified name does not exist.</p>
    SubscriptionNotFoundFault(String),
    /// <p>The value specified for the event severity was not one of the allowed values, or it specified a severity that does not apply to the specified source type. The allowed values are ERROR and INFO.</p>
    SubscriptionSeverityNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyEventSubscriptionError {
    pub fn from_body(body: &str) -> ModifyEventSubscriptionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidSubscriptionStateFault" => {
                    ModifyEventSubscriptionError::InvalidSubscriptionStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SNSInvalidTopic" => ModifyEventSubscriptionError::SNSInvalidTopicFault(
                    String::from(parsed_error.message),
                ),
                "SNSNoAuthorization" => ModifyEventSubscriptionError::SNSNoAuthorizationFault(
                    String::from(parsed_error.message),
                ),
                "SNSTopicArnNotFound" => ModifyEventSubscriptionError::SNSTopicArnNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "SourceNotFound" => ModifyEventSubscriptionError::SourceNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "SubscriptionCategoryNotFound" => {
                    ModifyEventSubscriptionError::SubscriptionCategoryNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SubscriptionEventIdNotFound" => {
                    ModifyEventSubscriptionError::SubscriptionEventIdNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SubscriptionNotFound" => ModifyEventSubscriptionError::SubscriptionNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "SubscriptionSeverityNotFound" => {
                    ModifyEventSubscriptionError::SubscriptionSeverityNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ModifyEventSubscriptionError::Unknown(String::from(body)),
            },
            Err(_) => ModifyEventSubscriptionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifyEventSubscriptionError {
    fn from(err: XmlParseError) -> ModifyEventSubscriptionError {
        let XmlParseError(message) = err;
        ModifyEventSubscriptionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyEventSubscriptionError {
    fn from(err: CredentialsError) -> ModifyEventSubscriptionError {
        ModifyEventSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyEventSubscriptionError {
    fn from(err: HttpDispatchError) -> ModifyEventSubscriptionError {
        ModifyEventSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyEventSubscriptionError {
    fn from(err: io::Error) -> ModifyEventSubscriptionError {
        ModifyEventSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            ModifyEventSubscriptionError::InvalidSubscriptionStateFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSTopicArnNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::SourceNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::SubscriptionCategoryNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::SubscriptionEventIdNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::SubscriptionNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::SubscriptionSeverityNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::Validation(ref cause) => cause,
            ModifyEventSubscriptionError::Credentials(ref err) => err.description(),
            ModifyEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyEventSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifySnapshotCopyRetentionPeriod
#[derive(Debug, PartialEq)]
pub enum ModifySnapshotCopyRetentionPeriodError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// <p>Cross-region snapshot copy was temporarily disabled. Try your request again.</p>
    SnapshotCopyDisabledFault(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifySnapshotCopyRetentionPeriodError {
    pub fn from_body(body: &str) -> ModifySnapshotCopyRetentionPeriodError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => ModifySnapshotCopyRetentionPeriodError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "InvalidClusterState" => {
                    ModifySnapshotCopyRetentionPeriodError::InvalidClusterStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "SnapshotCopyDisabledFault" => {
                    ModifySnapshotCopyRetentionPeriodError::SnapshotCopyDisabledFault(String::from(
                        parsed_error.message,
                    ))
                }
                "UnauthorizedOperation" => {
                    ModifySnapshotCopyRetentionPeriodError::UnauthorizedOperation(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ModifySnapshotCopyRetentionPeriodError::Unknown(String::from(body)),
            },
            Err(_) => ModifySnapshotCopyRetentionPeriodError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ModifySnapshotCopyRetentionPeriodError {
    fn from(err: XmlParseError) -> ModifySnapshotCopyRetentionPeriodError {
        let XmlParseError(message) = err;
        ModifySnapshotCopyRetentionPeriodError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifySnapshotCopyRetentionPeriodError {
    fn from(err: CredentialsError) -> ModifySnapshotCopyRetentionPeriodError {
        ModifySnapshotCopyRetentionPeriodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifySnapshotCopyRetentionPeriodError {
    fn from(err: HttpDispatchError) -> ModifySnapshotCopyRetentionPeriodError {
        ModifySnapshotCopyRetentionPeriodError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifySnapshotCopyRetentionPeriodError {
    fn from(err: io::Error) -> ModifySnapshotCopyRetentionPeriodError {
        ModifySnapshotCopyRetentionPeriodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifySnapshotCopyRetentionPeriodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifySnapshotCopyRetentionPeriodError {
    fn description(&self) -> &str {
        match *self {
            ModifySnapshotCopyRetentionPeriodError::ClusterNotFoundFault(ref cause) => cause,
            ModifySnapshotCopyRetentionPeriodError::InvalidClusterStateFault(ref cause) => cause,
            ModifySnapshotCopyRetentionPeriodError::SnapshotCopyDisabledFault(ref cause) => cause,
            ModifySnapshotCopyRetentionPeriodError::UnauthorizedOperation(ref cause) => cause,
            ModifySnapshotCopyRetentionPeriodError::Validation(ref cause) => cause,
            ModifySnapshotCopyRetentionPeriodError::Credentials(ref err) => err.description(),
            ModifySnapshotCopyRetentionPeriodError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifySnapshotCopyRetentionPeriodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PurchaseReservedNodeOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseReservedNodeOfferingError {
    /// <p>User already has a reservation with the given identifier.</p>
    ReservedNodeAlreadyExistsFault(String),
    /// <p>Specified offering does not exist.</p>
    ReservedNodeOfferingNotFoundFault(String),
    /// <p>Request would exceed the user's compute node quota. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ReservedNodeQuotaExceededFault(String),
    /// <p>The requested operation isn't supported.</p>
    UnsupportedOperationFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PurchaseReservedNodeOfferingError {
    pub fn from_body(body: &str) -> PurchaseReservedNodeOfferingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ReservedNodeAlreadyExists" => {
                    PurchaseReservedNodeOfferingError::ReservedNodeAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ReservedNodeOfferingNotFound" => {
                    PurchaseReservedNodeOfferingError::ReservedNodeOfferingNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "ReservedNodeQuotaExceeded" => {
                    PurchaseReservedNodeOfferingError::ReservedNodeQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "UnsupportedOperation" => {
                    PurchaseReservedNodeOfferingError::UnsupportedOperationFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => PurchaseReservedNodeOfferingError::Unknown(String::from(body)),
            },
            Err(_) => PurchaseReservedNodeOfferingError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PurchaseReservedNodeOfferingError {
    fn from(err: XmlParseError) -> PurchaseReservedNodeOfferingError {
        let XmlParseError(message) = err;
        PurchaseReservedNodeOfferingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PurchaseReservedNodeOfferingError {
    fn from(err: CredentialsError) -> PurchaseReservedNodeOfferingError {
        PurchaseReservedNodeOfferingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PurchaseReservedNodeOfferingError {
    fn from(err: HttpDispatchError) -> PurchaseReservedNodeOfferingError {
        PurchaseReservedNodeOfferingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PurchaseReservedNodeOfferingError {
    fn from(err: io::Error) -> PurchaseReservedNodeOfferingError {
        PurchaseReservedNodeOfferingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PurchaseReservedNodeOfferingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PurchaseReservedNodeOfferingError {
    fn description(&self) -> &str {
        match *self {
            PurchaseReservedNodeOfferingError::ReservedNodeAlreadyExistsFault(ref cause) => cause,
            PurchaseReservedNodeOfferingError::ReservedNodeOfferingNotFoundFault(ref cause) => {
                cause
            }
            PurchaseReservedNodeOfferingError::ReservedNodeQuotaExceededFault(ref cause) => cause,
            PurchaseReservedNodeOfferingError::UnsupportedOperationFault(ref cause) => cause,
            PurchaseReservedNodeOfferingError::Validation(ref cause) => cause,
            PurchaseReservedNodeOfferingError::Credentials(ref err) => err.description(),
            PurchaseReservedNodeOfferingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PurchaseReservedNodeOfferingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootCluster
#[derive(Debug, PartialEq)]
pub enum RebootClusterError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RebootClusterError {
    pub fn from_body(body: &str) -> RebootClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => {
                    RebootClusterError::ClusterNotFoundFault(String::from(parsed_error.message))
                }
                "InvalidClusterState" => {
                    RebootClusterError::InvalidClusterStateFault(String::from(parsed_error.message))
                }
                _ => RebootClusterError::Unknown(String::from(body)),
            },
            Err(_) => RebootClusterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RebootClusterError {
    fn from(err: XmlParseError) -> RebootClusterError {
        let XmlParseError(message) = err;
        RebootClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RebootClusterError {
    fn from(err: CredentialsError) -> RebootClusterError {
        RebootClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootClusterError {
    fn from(err: HttpDispatchError) -> RebootClusterError {
        RebootClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebootClusterError {
    fn from(err: io::Error) -> RebootClusterError {
        RebootClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RebootClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootClusterError {
    fn description(&self) -> &str {
        match *self {
            RebootClusterError::ClusterNotFoundFault(ref cause) => cause,
            RebootClusterError::InvalidClusterStateFault(ref cause) => cause,
            RebootClusterError::Validation(ref cause) => cause,
            RebootClusterError::Credentials(ref err) => err.description(),
            RebootClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RebootClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetClusterParameterGroup
#[derive(Debug, PartialEq)]
pub enum ResetClusterParameterGroupError {
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// <p>The cluster parameter group action can not be completed because another task is in progress that involves the parameter group. Wait a few moments and try the operation again.</p>
    InvalidClusterParameterGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResetClusterParameterGroupError {
    pub fn from_body(body: &str) -> ResetClusterParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterParameterGroupNotFound" => {
                    ResetClusterParameterGroupError::ClusterParameterGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterParameterGroupState" => {
                    ResetClusterParameterGroupError::InvalidClusterParameterGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => ResetClusterParameterGroupError::Unknown(String::from(body)),
            },
            Err(_) => ResetClusterParameterGroupError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ResetClusterParameterGroupError {
    fn from(err: XmlParseError) -> ResetClusterParameterGroupError {
        let XmlParseError(message) = err;
        ResetClusterParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ResetClusterParameterGroupError {
    fn from(err: CredentialsError) -> ResetClusterParameterGroupError {
        ResetClusterParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetClusterParameterGroupError {
    fn from(err: HttpDispatchError) -> ResetClusterParameterGroupError {
        ResetClusterParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResetClusterParameterGroupError {
    fn from(err: io::Error) -> ResetClusterParameterGroupError {
        ResetClusterParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResetClusterParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetClusterParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ResetClusterParameterGroupError::ClusterParameterGroupNotFoundFault(ref cause) => cause,
            ResetClusterParameterGroupError::InvalidClusterParameterGroupStateFault(ref cause) => {
                cause
            }
            ResetClusterParameterGroupError::Validation(ref cause) => cause,
            ResetClusterParameterGroupError::Credentials(ref err) => err.description(),
            ResetClusterParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ResetClusterParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreFromClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreFromClusterSnapshotError {
    /// <p>The owner of the specified snapshot has not authorized your account to access the snapshot.</p>
    AccessToSnapshotDeniedFault(String),
    /// <p>The account already has a cluster with the given identifier.</p>
    ClusterAlreadyExistsFault(String),
    /// <p>The parameter group name does not refer to an existing parameter group.</p>
    ClusterParameterGroupNotFoundFault(String),
    /// <p>The request would exceed the allowed number of cluster instances for this account. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterQuotaExceededFault(String),
    /// <p>The cluster security group name does not refer to an existing cluster security group.</p>
    ClusterSecurityGroupNotFoundFault(String),
    /// <p>The snapshot identifier does not refer to an existing cluster snapshot.</p>
    ClusterSnapshotNotFoundFault(String),
    /// <p>The cluster subnet group name does not refer to an existing cluster subnet group.</p>
    ClusterSubnetGroupNotFoundFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>There is no Amazon Redshift HSM client certificate with the specified identifier.</p>
    HsmClientCertificateNotFoundFault(String),
    /// <p>There is no Amazon Redshift HSM configuration with the specified identifier.</p>
    HsmConfigurationNotFoundFault(String),
    /// <p>The number of nodes specified exceeds the allotted capacity of the cluster.</p>
    InsufficientClusterCapacityFault(String),
    /// <p>The specified cluster snapshot is not in the <code>available</code> state, or other accounts are authorized to access the snapshot. </p>
    InvalidClusterSnapshotStateFault(String),
    /// <p>The cluster subnet group cannot be deleted because it is in use.</p>
    InvalidClusterSubnetGroupStateFault(String),
    /// <p>The Elastic IP (EIP) is invalid or cannot be found.</p>
    InvalidElasticIpFault(String),
    /// <p>The restore is invalid.</p>
    InvalidRestoreFault(String),
    /// <p>The requested subnet is not valid, or not all of the subnets are in the same VPC.</p>
    InvalidSubnet(String),
    /// <p>The cluster subnet group does not cover all Availability Zones.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The encryption key has exceeded its grant limit in AWS KMS.</p>
    LimitExceededFault(String),
    /// <p>The operation would exceed the number of nodes allowed for a cluster.</p>
    NumberOfNodesPerClusterLimitExceededFault(String),
    /// <p>The operation would exceed the number of nodes allotted to the account. For information about increasing your quota, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    NumberOfNodesQuotaExceededFault(String),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RestoreFromClusterSnapshotError {
    pub fn from_body(body: &str) -> RestoreFromClusterSnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessToSnapshotDenied" => {
                    RestoreFromClusterSnapshotError::AccessToSnapshotDeniedFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterAlreadyExists" => {
                    RestoreFromClusterSnapshotError::ClusterAlreadyExistsFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterParameterGroupNotFound" => {
                    RestoreFromClusterSnapshotError::ClusterParameterGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "ClusterQuotaExceeded" => {
                    RestoreFromClusterSnapshotError::ClusterQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSecurityGroupNotFound" => {
                    RestoreFromClusterSnapshotError::ClusterSecurityGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "ClusterSnapshotNotFound" => {
                    RestoreFromClusterSnapshotError::ClusterSnapshotNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ClusterSubnetGroupNotFoundFault" => {
                    RestoreFromClusterSnapshotError::ClusterSubnetGroupNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "DependentServiceRequestThrottlingFault" => {
                    RestoreFromClusterSnapshotError::DependentServiceRequestThrottlingFault(
                        String::from(parsed_error.message),
                    )
                }
                "HsmClientCertificateNotFoundFault" => {
                    RestoreFromClusterSnapshotError::HsmClientCertificateNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "HsmConfigurationNotFoundFault" => {
                    RestoreFromClusterSnapshotError::HsmConfigurationNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientClusterCapacity" => {
                    RestoreFromClusterSnapshotError::InsufficientClusterCapacityFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterSnapshotState" => {
                    RestoreFromClusterSnapshotError::InvalidClusterSnapshotStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterSubnetGroupStateFault" => {
                    RestoreFromClusterSnapshotError::InvalidClusterSubnetGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidElasticIpFault" => RestoreFromClusterSnapshotError::InvalidElasticIpFault(
                    String::from(parsed_error.message),
                ),
                "InvalidRestore" => RestoreFromClusterSnapshotError::InvalidRestoreFault(
                    String::from(parsed_error.message),
                ),
                "InvalidSubnet" => RestoreFromClusterSnapshotError::InvalidSubnet(String::from(
                    parsed_error.message,
                )),
                "InvalidVPCNetworkStateFault" => {
                    RestoreFromClusterSnapshotError::InvalidVPCNetworkStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "LimitExceededFault" => RestoreFromClusterSnapshotError::LimitExceededFault(
                    String::from(parsed_error.message),
                ),
                "NumberOfNodesPerClusterLimitExceeded" => {
                    RestoreFromClusterSnapshotError::NumberOfNodesPerClusterLimitExceededFault(
                        String::from(parsed_error.message),
                    )
                }
                "NumberOfNodesQuotaExceeded" => {
                    RestoreFromClusterSnapshotError::NumberOfNodesQuotaExceededFault(String::from(
                        parsed_error.message,
                    ))
                }
                "UnauthorizedOperation" => RestoreFromClusterSnapshotError::UnauthorizedOperation(
                    String::from(parsed_error.message),
                ),
                _ => RestoreFromClusterSnapshotError::Unknown(String::from(body)),
            },
            Err(_) => RestoreFromClusterSnapshotError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RestoreFromClusterSnapshotError {
    fn from(err: XmlParseError) -> RestoreFromClusterSnapshotError {
        let XmlParseError(message) = err;
        RestoreFromClusterSnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RestoreFromClusterSnapshotError {
    fn from(err: CredentialsError) -> RestoreFromClusterSnapshotError {
        RestoreFromClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreFromClusterSnapshotError {
    fn from(err: HttpDispatchError) -> RestoreFromClusterSnapshotError {
        RestoreFromClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreFromClusterSnapshotError {
    fn from(err: io::Error) -> RestoreFromClusterSnapshotError {
        RestoreFromClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreFromClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreFromClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            RestoreFromClusterSnapshotError::AccessToSnapshotDeniedFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::ClusterAlreadyExistsFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::ClusterParameterGroupNotFoundFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::ClusterQuotaExceededFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::ClusterSecurityGroupNotFoundFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::ClusterSnapshotNotFoundFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::ClusterSubnetGroupNotFoundFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::DependentServiceRequestThrottlingFault(ref cause) => {
                cause
            }
            RestoreFromClusterSnapshotError::HsmClientCertificateNotFoundFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::HsmConfigurationNotFoundFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::InsufficientClusterCapacityFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::InvalidClusterSnapshotStateFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::InvalidClusterSubnetGroupStateFault(ref cause) => {
                cause
            }
            RestoreFromClusterSnapshotError::InvalidElasticIpFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::InvalidRestoreFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::InvalidSubnet(ref cause) => cause,
            RestoreFromClusterSnapshotError::InvalidVPCNetworkStateFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::LimitExceededFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::NumberOfNodesPerClusterLimitExceededFault(
                ref cause,
            ) => cause,
            RestoreFromClusterSnapshotError::NumberOfNodesQuotaExceededFault(ref cause) => cause,
            RestoreFromClusterSnapshotError::UnauthorizedOperation(ref cause) => cause,
            RestoreFromClusterSnapshotError::Validation(ref cause) => cause,
            RestoreFromClusterSnapshotError::Credentials(ref err) => err.description(),
            RestoreFromClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreFromClusterSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreTableFromClusterSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreTableFromClusterSnapshotError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The snapshot identifier does not refer to an existing cluster snapshot.</p>
    ClusterSnapshotNotFoundFault(String),
    /// <p>You have exceeded the allowed number of table restore requests. Wait for your current table restore requests to complete before making a new request.</p>
    InProgressTableRestoreQuotaExceededFault(String),
    /// <p>The specified cluster snapshot is not in the <code>available</code> state, or other accounts are authorized to access the snapshot. </p>
    InvalidClusterSnapshotStateFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// <p>The value specified for the <code>sourceDatabaseName</code>, <code>sourceSchemaName</code>, or <code>sourceTableName</code> parameter, or a combination of these, doesn't exist in the snapshot.</p>
    InvalidTableRestoreArgumentFault(String),
    /// <p>The requested operation isn't supported.</p>
    UnsupportedOperationFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RestoreTableFromClusterSnapshotError {
    pub fn from_body(body: &str) -> RestoreTableFromClusterSnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => RestoreTableFromClusterSnapshotError::ClusterNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "ClusterSnapshotNotFound" => {
                    RestoreTableFromClusterSnapshotError::ClusterSnapshotNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InProgressTableRestoreQuotaExceededFault" => {
                    RestoreTableFromClusterSnapshotError::InProgressTableRestoreQuotaExceededFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterSnapshotState" => {
                    RestoreTableFromClusterSnapshotError::InvalidClusterSnapshotStateFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterState" => {
                    RestoreTableFromClusterSnapshotError::InvalidClusterStateFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidTableRestoreArgument" => {
                    RestoreTableFromClusterSnapshotError::InvalidTableRestoreArgumentFault(
                        String::from(parsed_error.message),
                    )
                }
                "UnsupportedOperation" => {
                    RestoreTableFromClusterSnapshotError::UnsupportedOperationFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => RestoreTableFromClusterSnapshotError::Unknown(String::from(body)),
            },
            Err(_) => RestoreTableFromClusterSnapshotError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RestoreTableFromClusterSnapshotError {
    fn from(err: XmlParseError) -> RestoreTableFromClusterSnapshotError {
        let XmlParseError(message) = err;
        RestoreTableFromClusterSnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RestoreTableFromClusterSnapshotError {
    fn from(err: CredentialsError) -> RestoreTableFromClusterSnapshotError {
        RestoreTableFromClusterSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreTableFromClusterSnapshotError {
    fn from(err: HttpDispatchError) -> RestoreTableFromClusterSnapshotError {
        RestoreTableFromClusterSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreTableFromClusterSnapshotError {
    fn from(err: io::Error) -> RestoreTableFromClusterSnapshotError {
        RestoreTableFromClusterSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreTableFromClusterSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreTableFromClusterSnapshotError {
    fn description(&self) -> &str {
        match *self {
            RestoreTableFromClusterSnapshotError::ClusterNotFoundFault(ref cause) => cause,
            RestoreTableFromClusterSnapshotError::ClusterSnapshotNotFoundFault(ref cause) => cause,
            RestoreTableFromClusterSnapshotError::InProgressTableRestoreQuotaExceededFault(
                ref cause,
            ) => cause,
            RestoreTableFromClusterSnapshotError::InvalidClusterSnapshotStateFault(ref cause) => {
                cause
            }
            RestoreTableFromClusterSnapshotError::InvalidClusterStateFault(ref cause) => cause,
            RestoreTableFromClusterSnapshotError::InvalidTableRestoreArgumentFault(ref cause) => {
                cause
            }
            RestoreTableFromClusterSnapshotError::UnsupportedOperationFault(ref cause) => cause,
            RestoreTableFromClusterSnapshotError::Validation(ref cause) => cause,
            RestoreTableFromClusterSnapshotError::Credentials(ref err) => err.description(),
            RestoreTableFromClusterSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreTableFromClusterSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RevokeClusterSecurityGroupIngress
#[derive(Debug, PartialEq)]
pub enum RevokeClusterSecurityGroupIngressError {
    /// <p>The specified CIDR IP range or EC2 security group is not authorized for the specified cluster security group.</p>
    AuthorizationNotFoundFault(String),
    /// <p>The cluster security group name does not refer to an existing cluster security group.</p>
    ClusterSecurityGroupNotFoundFault(String),
    /// <p>The state of the cluster security group is not <code>available</code>. </p>
    InvalidClusterSecurityGroupStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RevokeClusterSecurityGroupIngressError {
    pub fn from_body(body: &str) -> RevokeClusterSecurityGroupIngressError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationNotFound" => {
                    RevokeClusterSecurityGroupIngressError::AuthorizationNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "ClusterSecurityGroupNotFound" => {
                    RevokeClusterSecurityGroupIngressError::ClusterSecurityGroupNotFoundFault(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidClusterSecurityGroupState" => {
                    RevokeClusterSecurityGroupIngressError::InvalidClusterSecurityGroupStateFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => RevokeClusterSecurityGroupIngressError::Unknown(String::from(body)),
            },
            Err(_) => RevokeClusterSecurityGroupIngressError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RevokeClusterSecurityGroupIngressError {
    fn from(err: XmlParseError) -> RevokeClusterSecurityGroupIngressError {
        let XmlParseError(message) = err;
        RevokeClusterSecurityGroupIngressError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RevokeClusterSecurityGroupIngressError {
    fn from(err: CredentialsError) -> RevokeClusterSecurityGroupIngressError {
        RevokeClusterSecurityGroupIngressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RevokeClusterSecurityGroupIngressError {
    fn from(err: HttpDispatchError) -> RevokeClusterSecurityGroupIngressError {
        RevokeClusterSecurityGroupIngressError::HttpDispatch(err)
    }
}
impl From<io::Error> for RevokeClusterSecurityGroupIngressError {
    fn from(err: io::Error) -> RevokeClusterSecurityGroupIngressError {
        RevokeClusterSecurityGroupIngressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RevokeClusterSecurityGroupIngressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeClusterSecurityGroupIngressError {
    fn description(&self) -> &str {
        match *self {
            RevokeClusterSecurityGroupIngressError::AuthorizationNotFoundFault(ref cause) => cause,
            RevokeClusterSecurityGroupIngressError::ClusterSecurityGroupNotFoundFault(
                ref cause,
            ) => cause,
            RevokeClusterSecurityGroupIngressError::InvalidClusterSecurityGroupStateFault(
                ref cause,
            ) => cause,
            RevokeClusterSecurityGroupIngressError::Validation(ref cause) => cause,
            RevokeClusterSecurityGroupIngressError::Credentials(ref err) => err.description(),
            RevokeClusterSecurityGroupIngressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RevokeClusterSecurityGroupIngressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RevokeSnapshotAccess
#[derive(Debug, PartialEq)]
pub enum RevokeSnapshotAccessError {
    /// <p>The owner of the specified snapshot has not authorized your account to access the snapshot.</p>
    AccessToSnapshotDeniedFault(String),
    /// <p>The specified CIDR IP range or EC2 security group is not authorized for the specified cluster security group.</p>
    AuthorizationNotFoundFault(String),
    /// <p>The snapshot identifier does not refer to an existing cluster snapshot.</p>
    ClusterSnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RevokeSnapshotAccessError {
    pub fn from_body(body: &str) -> RevokeSnapshotAccessError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessToSnapshotDenied" => RevokeSnapshotAccessError::AccessToSnapshotDeniedFault(
                    String::from(parsed_error.message),
                ),
                "AuthorizationNotFound" => RevokeSnapshotAccessError::AuthorizationNotFoundFault(
                    String::from(parsed_error.message),
                ),
                "ClusterSnapshotNotFound" => {
                    RevokeSnapshotAccessError::ClusterSnapshotNotFoundFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => RevokeSnapshotAccessError::Unknown(String::from(body)),
            },
            Err(_) => RevokeSnapshotAccessError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RevokeSnapshotAccessError {
    fn from(err: XmlParseError) -> RevokeSnapshotAccessError {
        let XmlParseError(message) = err;
        RevokeSnapshotAccessError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RevokeSnapshotAccessError {
    fn from(err: CredentialsError) -> RevokeSnapshotAccessError {
        RevokeSnapshotAccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RevokeSnapshotAccessError {
    fn from(err: HttpDispatchError) -> RevokeSnapshotAccessError {
        RevokeSnapshotAccessError::HttpDispatch(err)
    }
}
impl From<io::Error> for RevokeSnapshotAccessError {
    fn from(err: io::Error) -> RevokeSnapshotAccessError {
        RevokeSnapshotAccessError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RevokeSnapshotAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeSnapshotAccessError {
    fn description(&self) -> &str {
        match *self {
            RevokeSnapshotAccessError::AccessToSnapshotDeniedFault(ref cause) => cause,
            RevokeSnapshotAccessError::AuthorizationNotFoundFault(ref cause) => cause,
            RevokeSnapshotAccessError::ClusterSnapshotNotFoundFault(ref cause) => cause,
            RevokeSnapshotAccessError::Validation(ref cause) => cause,
            RevokeSnapshotAccessError::Credentials(ref err) => err.description(),
            RevokeSnapshotAccessError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RevokeSnapshotAccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RotateEncryptionKey
#[derive(Debug, PartialEq)]
pub enum RotateEncryptionKeyError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(String),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(String),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RotateEncryptionKeyError {
    pub fn from_body(body: &str) -> RotateEncryptionKeyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ClusterNotFound" => RotateEncryptionKeyError::ClusterNotFoundFault(String::from(
                    parsed_error.message,
                )),
                "DependentServiceRequestThrottlingFault" => {
                    RotateEncryptionKeyError::DependentServiceRequestThrottlingFault(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidClusterState" => RotateEncryptionKeyError::InvalidClusterStateFault(
                    String::from(parsed_error.message),
                ),
                _ => RotateEncryptionKeyError::Unknown(String::from(body)),
            },
            Err(_) => RotateEncryptionKeyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RotateEncryptionKeyError {
    fn from(err: XmlParseError) -> RotateEncryptionKeyError {
        let XmlParseError(message) = err;
        RotateEncryptionKeyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RotateEncryptionKeyError {
    fn from(err: CredentialsError) -> RotateEncryptionKeyError {
        RotateEncryptionKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RotateEncryptionKeyError {
    fn from(err: HttpDispatchError) -> RotateEncryptionKeyError {
        RotateEncryptionKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for RotateEncryptionKeyError {
    fn from(err: io::Error) -> RotateEncryptionKeyError {
        RotateEncryptionKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RotateEncryptionKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RotateEncryptionKeyError {
    fn description(&self) -> &str {
        match *self {
            RotateEncryptionKeyError::ClusterNotFoundFault(ref cause) => cause,
            RotateEncryptionKeyError::DependentServiceRequestThrottlingFault(ref cause) => cause,
            RotateEncryptionKeyError::InvalidClusterStateFault(ref cause) => cause,
            RotateEncryptionKeyError::Validation(ref cause) => cause,
            RotateEncryptionKeyError::Credentials(ref err) => err.description(),
            RotateEncryptionKeyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RotateEncryptionKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Redshift API. Amazon Redshift clients implement this trait.
pub trait Redshift {
    /// <p>Adds an inbound (ingress) rule to an Amazon Redshift security group. Depending on whether the application accessing your cluster is running on the Internet or an Amazon EC2 instance, you can authorize inbound access to either a Classless Interdomain Routing (CIDR)/Internet Protocol (IP) range or to an Amazon EC2 security group. You can add as many as 20 ingress rules to an Amazon Redshift security group.</p> <p>If you authorize access to an Amazon EC2 security group, specify <i>EC2SecurityGroupName</i> and <i>EC2SecurityGroupOwnerId</i>. The Amazon EC2 security group and Amazon Redshift cluster must be in the same AWS region. </p> <p>If you authorize access to a CIDR/IP address range, specify <i>CIDRIP</i>. For an overview of CIDR blocks, see the Wikipedia article on <a href="http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>. </p> <p>You must also associate the security group with a cluster so that clients running on these IP addresses or the EC2 instance are authorized to connect to the cluster. For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Working with Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn authorize_cluster_security_group_ingress(
        &self,
        input: AuthorizeClusterSecurityGroupIngressMessage,
    ) -> RusotoFuture<
        AuthorizeClusterSecurityGroupIngressResult,
        AuthorizeClusterSecurityGroupIngressError,
    >;

    /// <p>Authorizes the specified AWS customer account to restore the specified snapshot.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn authorize_snapshot_access(
        &self,
        input: AuthorizeSnapshotAccessMessage,
    ) -> RusotoFuture<AuthorizeSnapshotAccessResult, AuthorizeSnapshotAccessError>;

    /// <p>Copies the specified automated cluster snapshot to a new manual cluster snapshot. The source must be an automated snapshot and it must be in the available state.</p> <p>When you delete a cluster, Amazon Redshift deletes any automated snapshots of the cluster. Also, when the retention period of the snapshot expires, Amazon Redshift automatically deletes it. If you want to keep an automated snapshot for a longer period, you can make a manual copy of the snapshot. Manual snapshots are retained until you delete them.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn copy_cluster_snapshot(
        &self,
        input: CopyClusterSnapshotMessage,
    ) -> RusotoFuture<CopyClusterSnapshotResult, CopyClusterSnapshotError>;

    /// <p>Creates a new cluster.</p> <p>To create the cluster in Virtual Private Cloud (VPC), you must provide a cluster subnet group name. The cluster subnet group identifies the subnets of your VPC that Amazon Redshift uses when creating the cluster. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster(
        &self,
        input: CreateClusterMessage,
    ) -> RusotoFuture<CreateClusterResult, CreateClusterError>;

    /// <p>Creates an Amazon Redshift parameter group.</p> <p>Creating parameter groups is independent of creating clusters. You can associate a cluster with a parameter group when you create the cluster. You can also associate an existing cluster with a parameter group after the cluster is created by using <a>ModifyCluster</a>. </p> <p>Parameters in the parameter group define specific behavior that applies to the databases you create on the cluster. For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_parameter_group(
        &self,
        input: CreateClusterParameterGroupMessage,
    ) -> RusotoFuture<CreateClusterParameterGroupResult, CreateClusterParameterGroupError>;

    /// <p>Creates a new Amazon Redshift security group. You use security groups to control access to non-VPC clusters.</p> <p> For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_security_group(
        &self,
        input: CreateClusterSecurityGroupMessage,
    ) -> RusotoFuture<CreateClusterSecurityGroupResult, CreateClusterSecurityGroupError>;

    /// <p>Creates a manual snapshot of the specified cluster. The cluster must be in the <code>available</code> state. </p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_snapshot(
        &self,
        input: CreateClusterSnapshotMessage,
    ) -> RusotoFuture<CreateClusterSnapshotResult, CreateClusterSnapshotError>;

    /// <p>Creates a new Amazon Redshift subnet group. You must provide a list of one or more subnets in your existing Amazon Virtual Private Cloud (Amazon VPC) when creating Amazon Redshift subnet group.</p> <p> For information about subnet groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-cluster-subnet-groups.html">Amazon Redshift Cluster Subnet Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_subnet_group(
        &self,
        input: CreateClusterSubnetGroupMessage,
    ) -> RusotoFuture<CreateClusterSubnetGroupResult, CreateClusterSubnetGroupError>;

    /// <p>Creates an Amazon Redshift event notification subscription. This action requires an ARN (Amazon Resource Name) of an Amazon SNS topic created by either the Amazon Redshift console, the Amazon SNS console, or the Amazon SNS API. To obtain an ARN with Amazon SNS, you must create a topic in Amazon SNS and subscribe to the topic. The ARN is displayed in the SNS console.</p> <p>You can specify the source type, and lists of Amazon Redshift source IDs, event categories, and event severities. Notifications will be sent for all events you want that match those criteria. For example, you can specify source type = cluster, source ID = my-cluster-1 and mycluster2, event categories = Availability, Backup, and severity = ERROR. The subscription will only send notifications for those ERROR events in the Availability and Backup categories for the specified clusters.</p> <p>If you specify both the source type and source IDs, such as source type = cluster and source identifier = my-cluster-1, notifications will be sent for all the cluster events for my-cluster-1. If you specify a source type but do not specify a source identifier, you will receive notice of the events for the objects of that type in your AWS account. If you do not specify either the SourceType nor the SourceIdentifier, you will be notified of events generated from all Amazon Redshift sources belonging to your AWS account. You must specify a source type if you specify a source ID.</p>
    fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> RusotoFuture<CreateEventSubscriptionResult, CreateEventSubscriptionError>;

    /// <p>Creates an HSM client certificate that an Amazon Redshift cluster will use to connect to the client's HSM in order to store and retrieve the keys used to encrypt the cluster databases.</p> <p>The command returns a public key, which you must store in the HSM. In addition to creating the HSM certificate, you must create an Amazon Redshift HSM configuration that provides a cluster the information needed to store and use encryption keys in the HSM. For more information, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-HSM.html">Hardware Security Modules</a> in the Amazon Redshift Cluster Management Guide.</p>
    fn create_hsm_client_certificate(
        &self,
        input: CreateHsmClientCertificateMessage,
    ) -> RusotoFuture<CreateHsmClientCertificateResult, CreateHsmClientCertificateError>;

    /// <p>Creates an HSM configuration that contains the information required by an Amazon Redshift cluster to store and use database encryption keys in a Hardware Security Module (HSM). After creating the HSM configuration, you can specify it as a parameter when creating a cluster. The cluster will then store its encryption keys in the HSM.</p> <p>In addition to creating an HSM configuration, you must also create an HSM client certificate. For more information, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-HSM.html">Hardware Security Modules</a> in the Amazon Redshift Cluster Management Guide.</p>
    fn create_hsm_configuration(
        &self,
        input: CreateHsmConfigurationMessage,
    ) -> RusotoFuture<CreateHsmConfigurationResult, CreateHsmConfigurationError>;

    /// <p>Creates a snapshot copy grant that permits Amazon Redshift to use a customer master key (CMK) from AWS Key Management Service (AWS KMS) to encrypt copied snapshots in a destination region.</p> <p> For more information about managing snapshot copy grants, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-db-encryption.html">Amazon Redshift Database Encryption</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn create_snapshot_copy_grant(
        &self,
        input: CreateSnapshotCopyGrantMessage,
    ) -> RusotoFuture<CreateSnapshotCopyGrantResult, CreateSnapshotCopyGrantError>;

    /// <p>Adds one or more tags to a specified resource.</p> <p>A resource can have up to 10 tags. If you try to create more than 10 tags for a resource, you will receive an error and the attempt will fail.</p> <p>If you specify a key that already exists for the resource, the value for that key will be updated with the new value.</p>
    fn create_tags(&self, input: CreateTagsMessage) -> RusotoFuture<(), CreateTagsError>;

    /// <p>Deletes a previously provisioned cluster. A successful response from the web service indicates that the request was received correctly. Use <a>DescribeClusters</a> to monitor the status of the deletion. The delete operation cannot be canceled or reverted once submitted. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you want to shut down the cluster and retain it for future use, set <i>SkipFinalClusterSnapshot</i> to <code>false</code> and specify a name for <i>FinalClusterSnapshotIdentifier</i>. You can later restore this snapshot to resume using the cluster. If a final cluster snapshot is requested, the status of the cluster will be "final-snapshot" while the snapshot is being taken, then it's "deleting" once Amazon Redshift begins deleting the cluster. </p> <p> For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterMessage,
    ) -> RusotoFuture<DeleteClusterResult, DeleteClusterError>;

    /// <p><p>Deletes a specified Amazon Redshift parameter group.</p> <note> <p>You cannot delete a parameter group if it is associated with a cluster.</p> </note></p>
    fn delete_cluster_parameter_group(
        &self,
        input: DeleteClusterParameterGroupMessage,
    ) -> RusotoFuture<(), DeleteClusterParameterGroupError>;

    /// <p>Deletes an Amazon Redshift security group.</p> <note> <p>You cannot delete a security group that is associated with any clusters. You cannot delete the default security group.</p> </note> <p> For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn delete_cluster_security_group(
        &self,
        input: DeleteClusterSecurityGroupMessage,
    ) -> RusotoFuture<(), DeleteClusterSecurityGroupError>;

    /// <p>Deletes the specified manual snapshot. The snapshot must be in the <code>available</code> state, with no other users authorized to access the snapshot. </p> <p>Unlike automated snapshots, manual snapshots are retained even after you delete your cluster. Amazon Redshift does not delete your manual snapshots. You must delete manual snapshot explicitly to avoid getting charged. If other accounts are authorized to access the snapshot, you must revoke all of the authorizations before you can delete the snapshot.</p>
    fn delete_cluster_snapshot(
        &self,
        input: DeleteClusterSnapshotMessage,
    ) -> RusotoFuture<DeleteClusterSnapshotResult, DeleteClusterSnapshotError>;

    /// <p>Deletes the specified cluster subnet group.</p>
    fn delete_cluster_subnet_group(
        &self,
        input: DeleteClusterSubnetGroupMessage,
    ) -> RusotoFuture<(), DeleteClusterSubnetGroupError>;

    /// <p>Deletes an Amazon Redshift event notification subscription.</p>
    fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> RusotoFuture<(), DeleteEventSubscriptionError>;

    /// <p>Deletes the specified HSM client certificate.</p>
    fn delete_hsm_client_certificate(
        &self,
        input: DeleteHsmClientCertificateMessage,
    ) -> RusotoFuture<(), DeleteHsmClientCertificateError>;

    /// <p>Deletes the specified Amazon Redshift HSM configuration.</p>
    fn delete_hsm_configuration(
        &self,
        input: DeleteHsmConfigurationMessage,
    ) -> RusotoFuture<(), DeleteHsmConfigurationError>;

    /// <p>Deletes the specified snapshot copy grant.</p>
    fn delete_snapshot_copy_grant(
        &self,
        input: DeleteSnapshotCopyGrantMessage,
    ) -> RusotoFuture<(), DeleteSnapshotCopyGrantError>;

    /// <p>Deletes a tag or tags from a resource. You must provide the ARN of the resource from which you want to delete the tag or tags.</p>
    fn delete_tags(&self, input: DeleteTagsMessage) -> RusotoFuture<(), DeleteTagsError>;

    /// <p>Returns a list of Amazon Redshift parameter groups, including parameter groups you created and the default parameter group. For each parameter group, the response includes the parameter group name, description, and parameter group family name. You can optionally specify a name to retrieve the description of a specific parameter group.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all parameter groups that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all parameter groups that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, parameter groups are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_parameter_groups(
        &self,
        input: DescribeClusterParameterGroupsMessage,
    ) -> RusotoFuture<ClusterParameterGroupsMessage, DescribeClusterParameterGroupsError>;

    /// <p>Returns a detailed list of parameters contained within the specified Amazon Redshift parameter group. For each parameter the response includes information such as parameter name, description, data type, value, whether the parameter value is modifiable, and so on.</p> <p>You can specify <i>source</i> filter to retrieve parameters of only specific type. For example, to retrieve parameters that were modified by a user action such as from <a>ModifyClusterParameterGroup</a>, you can specify <i>source</i> equal to <i>user</i>.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_cluster_parameters(
        &self,
        input: DescribeClusterParametersMessage,
    ) -> RusotoFuture<ClusterParameterGroupDetails, DescribeClusterParametersError>;

    /// <p>Returns information about Amazon Redshift security groups. If the name of a security group is specified, the response will contain only information about only that security group.</p> <p> For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all security groups that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all security groups that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, security groups are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_security_groups(
        &self,
        input: DescribeClusterSecurityGroupsMessage,
    ) -> RusotoFuture<ClusterSecurityGroupMessage, DescribeClusterSecurityGroupsError>;

    /// <p>Returns one or more snapshot objects, which contain metadata about your cluster snapshots. By default, this operation returns information about all snapshots of all clusters that are owned by you AWS customer account. No information is returned for snapshots owned by inactive AWS customer accounts.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all snapshots that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all snapshots that have any combination of those values are returned. Only snapshots that you own are returned in the response; shared snapshots are not returned with the tag key and tag value request parameters.</p> <p>If both tag keys and values are omitted from the request, snapshots are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_snapshots(
        &self,
        input: DescribeClusterSnapshotsMessage,
    ) -> RusotoFuture<SnapshotMessage, DescribeClusterSnapshotsError>;

    /// <p>Returns one or more cluster subnet group objects, which contain metadata about your cluster subnet groups. By default, this operation returns information about all cluster subnet groups that are defined in you AWS account.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all subnet groups that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all subnet groups that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, subnet groups are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_subnet_groups(
        &self,
        input: DescribeClusterSubnetGroupsMessage,
    ) -> RusotoFuture<ClusterSubnetGroupMessage, DescribeClusterSubnetGroupsError>;

    /// <p>Returns descriptions of the available Amazon Redshift cluster versions. You can call this operation even before creating any clusters to learn more about the Amazon Redshift versions. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_cluster_versions(
        &self,
        input: DescribeClusterVersionsMessage,
    ) -> RusotoFuture<ClusterVersionsMessage, DescribeClusterVersionsError>;

    /// <p>Returns properties of provisioned clusters including general cluster properties, cluster database properties, maintenance and backup properties, and security and access properties. This operation supports pagination. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all clusters that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all clusters that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, clusters are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersMessage,
    ) -> RusotoFuture<ClustersMessage, DescribeClustersError>;

    /// <p>Returns a list of parameter settings for the specified parameter group family.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_default_cluster_parameters(
        &self,
        input: DescribeDefaultClusterParametersMessage,
    ) -> RusotoFuture<DescribeDefaultClusterParametersResult, DescribeDefaultClusterParametersError>;

    /// <p>Displays a list of event categories for all event source types, or for a specified source type. For a list of the event categories and source types, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-event-notifications.html">Amazon Redshift Event Notifications</a>.</p>
    fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> RusotoFuture<EventCategoriesMessage, DescribeEventCategoriesError>;

    /// <p>Lists descriptions of all the Amazon Redshift event notification subscriptions for a customer account. If you specify a subscription name, lists the description for that subscription.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all event notification subscriptions that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all subscriptions that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, subscriptions are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> RusotoFuture<EventSubscriptionsMessage, DescribeEventSubscriptionsError>;

    /// <p>Returns events related to clusters, security groups, snapshots, and parameter groups for the past 14 days. Events specific to a particular cluster, security group, snapshot or parameter group can be obtained by providing the name as a parameter. By default, the past hour of events are returned.</p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<EventsMessage, DescribeEventsError>;

    /// <p>Returns information about the specified HSM client certificate. If no certificate ID is specified, returns information about all the HSM certificates owned by your AWS customer account.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all HSM client certificates that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all HSM client certificates that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, HSM client certificates are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_hsm_client_certificates(
        &self,
        input: DescribeHsmClientCertificatesMessage,
    ) -> RusotoFuture<HsmClientCertificateMessage, DescribeHsmClientCertificatesError>;

    /// <p>Returns information about the specified Amazon Redshift HSM configuration. If no configuration ID is specified, returns information about all the HSM configurations owned by your AWS customer account.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all HSM connections that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all HSM connections that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, HSM connections are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_hsm_configurations(
        &self,
        input: DescribeHsmConfigurationsMessage,
    ) -> RusotoFuture<HsmConfigurationMessage, DescribeHsmConfigurationsError>;

    /// <p>Describes whether information, such as queries and connection attempts, is being logged for the specified Amazon Redshift cluster.</p>
    fn describe_logging_status(
        &self,
        input: DescribeLoggingStatusMessage,
    ) -> RusotoFuture<LoggingStatus, DescribeLoggingStatusError>;

    /// <p>Returns a list of orderable cluster options. Before you create a new cluster you can use this operation to find what options are available, such as the EC2 Availability Zones (AZ) in the specific AWS region that you can specify, and the node types you can request. The node types differ by available storage, memory, CPU and price. With the cost involved you might want to obtain a list of cluster options in the specific region and specify values when creating a cluster. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_orderable_cluster_options(
        &self,
        input: DescribeOrderableClusterOptionsMessage,
    ) -> RusotoFuture<OrderableClusterOptionsMessage, DescribeOrderableClusterOptionsError>;

    /// <p>Returns a list of the available reserved node offerings by Amazon Redshift with their descriptions including the node type, the fixed and recurring costs of reserving the node and duration the node will be reserved for you. These descriptions help you determine which reserve node offering you want to purchase. You then use the unique offering ID in you call to <a>PurchaseReservedNodeOffering</a> to reserve one or more nodes for your Amazon Redshift cluster. </p> <p> For more information about reserved node offerings, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/purchase-reserved-node-instance.html">Purchasing Reserved Nodes</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_reserved_node_offerings(
        &self,
        input: DescribeReservedNodeOfferingsMessage,
    ) -> RusotoFuture<ReservedNodeOfferingsMessage, DescribeReservedNodeOfferingsError>;

    /// <p>Returns the descriptions of the reserved nodes.</p>
    fn describe_reserved_nodes(
        &self,
        input: DescribeReservedNodesMessage,
    ) -> RusotoFuture<ReservedNodesMessage, DescribeReservedNodesError>;

    /// <p>Returns information about the last resize operation for the specified cluster. If no resize operation has ever been initiated for the specified cluster, a <code>HTTP 404</code> error is returned. If a resize operation was initiated and completed, the status of the resize remains as <code>SUCCEEDED</code> until the next resize. </p> <p>A resize operation can be requested using <a>ModifyCluster</a> and specifying a different number or type of nodes for the cluster. </p>
    fn describe_resize(
        &self,
        input: DescribeResizeMessage,
    ) -> RusotoFuture<ResizeProgressMessage, DescribeResizeError>;

    /// <p>Returns a list of snapshot copy grants owned by the AWS account in the destination region.</p> <p> For more information about managing snapshot copy grants, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-db-encryption.html">Amazon Redshift Database Encryption</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn describe_snapshot_copy_grants(
        &self,
        input: DescribeSnapshotCopyGrantsMessage,
    ) -> RusotoFuture<SnapshotCopyGrantMessage, DescribeSnapshotCopyGrantsError>;

    /// <p>Lists the status of one or more table restore requests made using the <a>RestoreTableFromClusterSnapshot</a> API action. If you don't specify a value for the <code>TableRestoreRequestId</code> parameter, then <code>DescribeTableRestoreStatus</code> returns the status of all table restore requests ordered by the date and time of the request in ascending order. Otherwise <code>DescribeTableRestoreStatus</code> returns the status of the table specified by <code>TableRestoreRequestId</code>.</p>
    fn describe_table_restore_status(
        &self,
        input: DescribeTableRestoreStatusMessage,
    ) -> RusotoFuture<TableRestoreStatusMessage, DescribeTableRestoreStatusError>;

    /// <p>Returns a list of tags. You can return tags from a specific resource by specifying an ARN, or you can return all tags for a given type of resource, such as clusters, snapshots, and so on.</p> <p>The following are limitations for <code>DescribeTags</code>: </p> <ul> <li> <p>You cannot specify an ARN and a resource-type value together in the same request.</p> </li> <li> <p>You cannot use the <code>MaxRecords</code> and <code>Marker</code> parameters together with the ARN parameter.</p> </li> <li> <p>The <code>MaxRecords</code> parameter can be a range from 10 to 50 results to return in a request.</p> </li> </ul> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all resources that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all resources that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, resources are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsMessage,
    ) -> RusotoFuture<TaggedResourceListMessage, DescribeTagsError>;

    /// <p>Stops logging information, such as queries and connection attempts, for the specified Amazon Redshift cluster.</p>
    fn disable_logging(
        &self,
        input: DisableLoggingMessage,
    ) -> RusotoFuture<LoggingStatus, DisableLoggingError>;

    /// <p>Disables the automatic copying of snapshots from one region to another region for a specified cluster.</p> <p>If your cluster and its snapshots are encrypted using a customer master key (CMK) from AWS KMS, use <a>DeleteSnapshotCopyGrant</a> to delete the grant that grants Amazon Redshift permission to the CMK in the destination region. </p>
    fn disable_snapshot_copy(
        &self,
        input: DisableSnapshotCopyMessage,
    ) -> RusotoFuture<DisableSnapshotCopyResult, DisableSnapshotCopyError>;

    /// <p>Starts logging information, such as queries and connection attempts, for the specified Amazon Redshift cluster.</p>
    fn enable_logging(
        &self,
        input: EnableLoggingMessage,
    ) -> RusotoFuture<LoggingStatus, EnableLoggingError>;

    /// <p>Enables the automatic copy of snapshots from one region to another region for a specified cluster.</p>
    fn enable_snapshot_copy(
        &self,
        input: EnableSnapshotCopyMessage,
    ) -> RusotoFuture<EnableSnapshotCopyResult, EnableSnapshotCopyError>;

    /// <p>Returns a database user name and temporary password with temporary authorization to log on to an Amazon Redshift database. The action returns the database user name prefixed with <code>IAM:</code> if <code>AutoCreate</code> is <code>False</code> or <code>IAMA:</code> if <code>AutoCreate</code> is <code>True</code>. You can optionally specify one or more database user groups that the user will join at log on. By default, the temporary credentials expire in 900 seconds. You can optionally specify a duration between 900 seconds (15 minutes) and 3600 seconds (60 minutes). For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/generating-user-credentials.html">Using IAM Authentication to Generate Database User Credentials</a> in the Amazon Redshift Cluster Management Guide.</p> <p>The AWS Identity and Access Management (IAM)user or role that executes GetClusterCredentials must have an IAM policy attached that allows access to all necessary actions and resources. For more information about permissions, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/redshift-iam-access-control-identity-based.html#redshift-policy-resources.getclustercredentials-resources">Resource Policies for GetClusterCredentials</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If the <code>DbGroups</code> parameter is specified, the IAM policy must allow the <code>redshift:JoinGroup</code> action with access to the listed <code>dbgroups</code>. </p> <p>In addition, if the <code>AutoCreate</code> parameter is set to <code>True</code>, then the policy must include the <code>redshift:CreateClusterUser</code> privilege.</p> <p>If the <code>DbName</code> parameter is specified, the IAM policy must allow access to the resource <code>dbname</code> for the specified database name. </p>
    fn get_cluster_credentials(
        &self,
        input: GetClusterCredentialsMessage,
    ) -> RusotoFuture<ClusterCredentials, GetClusterCredentialsError>;

    /// <p>Modifies the settings for a cluster. For example, you can add another security or parameter group, update the preferred maintenance window, or change the master user password. Resetting a cluster password or modifying the security groups associated with a cluster do not need a reboot. However, modifying a parameter group requires a reboot for parameters to take effect. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>You can also change node type and the number of nodes to scale up or down the cluster. When resizing a cluster, you must specify both the number of nodes and the node type even if one of the parameters does not change.</p>
    fn modify_cluster(
        &self,
        input: ModifyClusterMessage,
    ) -> RusotoFuture<ModifyClusterResult, ModifyClusterError>;

    /// <p>Modifies the list of AWS Identity and Access Management (IAM) roles that can be used by the cluster to access other AWS services.</p> <p>A cluster can have up to 10 IAM roles associated at any time.</p>
    fn modify_cluster_iam_roles(
        &self,
        input: ModifyClusterIamRolesMessage,
    ) -> RusotoFuture<ModifyClusterIamRolesResult, ModifyClusterIamRolesError>;

    /// <p>Modifies the parameters of a parameter group.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn modify_cluster_parameter_group(
        &self,
        input: ModifyClusterParameterGroupMessage,
    ) -> RusotoFuture<ClusterParameterGroupNameMessage, ModifyClusterParameterGroupError>;

    /// <p>Modifies a cluster subnet group to include the specified list of VPC subnets. The operation replaces the existing list of subnets with the new list of subnets.</p>
    fn modify_cluster_subnet_group(
        &self,
        input: ModifyClusterSubnetGroupMessage,
    ) -> RusotoFuture<ModifyClusterSubnetGroupResult, ModifyClusterSubnetGroupError>;

    /// <p>Modifies an existing Amazon Redshift event notification subscription.</p>
    fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> RusotoFuture<ModifyEventSubscriptionResult, ModifyEventSubscriptionError>;

    /// <p>Modifies the number of days to retain automated snapshots in the destination region after they are copied from the source region.</p>
    fn modify_snapshot_copy_retention_period(
        &self,
        input: ModifySnapshotCopyRetentionPeriodMessage,
    ) -> RusotoFuture<ModifySnapshotCopyRetentionPeriodResult, ModifySnapshotCopyRetentionPeriodError>;

    /// <p>Allows you to purchase reserved nodes. Amazon Redshift offers a predefined set of reserved node offerings. You can purchase one or more of the offerings. You can call the <a>DescribeReservedNodeOfferings</a> API to obtain the available reserved node offerings. You can call this API by providing a specific reserved node offering and the number of nodes you want to reserve. </p> <p> For more information about reserved node offerings, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/purchase-reserved-node-instance.html">Purchasing Reserved Nodes</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn purchase_reserved_node_offering(
        &self,
        input: PurchaseReservedNodeOfferingMessage,
    ) -> RusotoFuture<PurchaseReservedNodeOfferingResult, PurchaseReservedNodeOfferingError>;

    /// <p>Reboots a cluster. This action is taken as soon as possible. It results in a momentary outage to the cluster, during which the cluster status is set to <code>rebooting</code>. A cluster event is created when the reboot is completed. Any pending cluster modifications (see <a>ModifyCluster</a>) are applied at this reboot. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn reboot_cluster(
        &self,
        input: RebootClusterMessage,
    ) -> RusotoFuture<RebootClusterResult, RebootClusterError>;

    /// <p>Sets one or more parameters of the specified parameter group to their default values and sets the source values of the parameters to "engine-default". To reset the entire parameter group specify the <i>ResetAllParameters</i> parameter. For parameter changes to take effect you must reboot any associated clusters. </p>
    fn reset_cluster_parameter_group(
        &self,
        input: ResetClusterParameterGroupMessage,
    ) -> RusotoFuture<ClusterParameterGroupNameMessage, ResetClusterParameterGroupError>;

    /// <p>Creates a new cluster from a snapshot. By default, Amazon Redshift creates the resulting cluster with the same configuration as the original cluster from which the snapshot was created, except that the new cluster is created with the default cluster security and parameter groups. After Amazon Redshift creates the cluster, you can use the <a>ModifyCluster</a> API to associate a different security group and different parameter group with the restored cluster. If you are using a DS node type, you can also choose to change to another DS node type of the same size during restore.</p> <p>If you restore a cluster into a VPC, you must provide a cluster subnet group where you want the cluster restored.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn restore_from_cluster_snapshot(
        &self,
        input: RestoreFromClusterSnapshotMessage,
    ) -> RusotoFuture<RestoreFromClusterSnapshotResult, RestoreFromClusterSnapshotError>;

    /// <p>Creates a new table from a table in an Amazon Redshift cluster snapshot. You must create the new table within the Amazon Redshift cluster that the snapshot was taken from.</p> <p>You cannot use <code>RestoreTableFromClusterSnapshot</code> to restore a table with the same name as an existing table in an Amazon Redshift cluster. That is, you cannot overwrite an existing table in a cluster with a restored table. If you want to replace your original table with a new, restored table, then rename or drop your original table before you call <code>RestoreTableFromClusterSnapshot</code>. When you have renamed your original table, then you can pass the original name of the table as the <code>NewTableName</code> parameter value in the call to <code>RestoreTableFromClusterSnapshot</code>. This way, you can replace the original table with the table created from the snapshot.</p>
    fn restore_table_from_cluster_snapshot(
        &self,
        input: RestoreTableFromClusterSnapshotMessage,
    ) -> RusotoFuture<RestoreTableFromClusterSnapshotResult, RestoreTableFromClusterSnapshotError>;

    /// <p>Revokes an ingress rule in an Amazon Redshift security group for a previously authorized IP range or Amazon EC2 security group. To add an ingress rule, see <a>AuthorizeClusterSecurityGroupIngress</a>. For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn revoke_cluster_security_group_ingress(
        &self,
        input: RevokeClusterSecurityGroupIngressMessage,
    ) -> RusotoFuture<RevokeClusterSecurityGroupIngressResult, RevokeClusterSecurityGroupIngressError>;

    /// <p>Removes the ability of the specified AWS customer account to restore the specified snapshot. If the account is currently restoring the snapshot, the restore will run to completion.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn revoke_snapshot_access(
        &self,
        input: RevokeSnapshotAccessMessage,
    ) -> RusotoFuture<RevokeSnapshotAccessResult, RevokeSnapshotAccessError>;

    /// <p>Rotates the encryption keys for a cluster.</p>
    fn rotate_encryption_key(
        &self,
        input: RotateEncryptionKeyMessage,
    ) -> RusotoFuture<RotateEncryptionKeyResult, RotateEncryptionKeyError>;
}
/// A client for the Amazon Redshift API.
pub struct RedshiftClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl RedshiftClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> RedshiftClient {
        RedshiftClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> RedshiftClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        RedshiftClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Redshift for RedshiftClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds an inbound (ingress) rule to an Amazon Redshift security group. Depending on whether the application accessing your cluster is running on the Internet or an Amazon EC2 instance, you can authorize inbound access to either a Classless Interdomain Routing (CIDR)/Internet Protocol (IP) range or to an Amazon EC2 security group. You can add as many as 20 ingress rules to an Amazon Redshift security group.</p> <p>If you authorize access to an Amazon EC2 security group, specify <i>EC2SecurityGroupName</i> and <i>EC2SecurityGroupOwnerId</i>. The Amazon EC2 security group and Amazon Redshift cluster must be in the same AWS region. </p> <p>If you authorize access to a CIDR/IP address range, specify <i>CIDRIP</i>. For an overview of CIDR blocks, see the Wikipedia article on <a href="http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>. </p> <p>You must also associate the security group with a cluster so that clients running on these IP addresses or the EC2 instance are authorized to connect to the cluster. For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Working with Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn authorize_cluster_security_group_ingress(
        &self,
        input: AuthorizeClusterSecurityGroupIngressMessage,
    ) -> RusotoFuture<
        AuthorizeClusterSecurityGroupIngressResult,
        AuthorizeClusterSecurityGroupIngressError,
    > {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AuthorizeClusterSecurityGroupIngress");
        params.put("Version", "2012-12-01");
        AuthorizeClusterSecurityGroupIngressMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AuthorizeClusterSecurityGroupIngressError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AuthorizeClusterSecurityGroupIngressResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        AuthorizeClusterSecurityGroupIngressResultDeserializer::deserialize(
                            "AuthorizeClusterSecurityGroupIngressResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Authorizes the specified AWS customer account to restore the specified snapshot.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn authorize_snapshot_access(
        &self,
        input: AuthorizeSnapshotAccessMessage,
    ) -> RusotoFuture<AuthorizeSnapshotAccessResult, AuthorizeSnapshotAccessError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AuthorizeSnapshotAccess");
        params.put("Version", "2012-12-01");
        AuthorizeSnapshotAccessMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AuthorizeSnapshotAccessError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AuthorizeSnapshotAccessResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AuthorizeSnapshotAccessResultDeserializer::deserialize(
                        "AuthorizeSnapshotAccessResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Copies the specified automated cluster snapshot to a new manual cluster snapshot. The source must be an automated snapshot and it must be in the available state.</p> <p>When you delete a cluster, Amazon Redshift deletes any automated snapshots of the cluster. Also, when the retention period of the snapshot expires, Amazon Redshift automatically deletes it. If you want to keep an automated snapshot for a longer period, you can make a manual copy of the snapshot. Manual snapshots are retained until you delete them.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn copy_cluster_snapshot(
        &self,
        input: CopyClusterSnapshotMessage,
    ) -> RusotoFuture<CopyClusterSnapshotResult, CopyClusterSnapshotError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopyClusterSnapshot");
        params.put("Version", "2012-12-01");
        CopyClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CopyClusterSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CopyClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CopyClusterSnapshotResultDeserializer::deserialize(
                        "CopyClusterSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new cluster.</p> <p>To create the cluster in Virtual Private Cloud (VPC), you must provide a cluster subnet group name. The cluster subnet group identifies the subnets of your VPC that Amazon Redshift uses when creating the cluster. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster(
        &self,
        input: CreateClusterMessage,
    ) -> RusotoFuture<CreateClusterResult, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateCluster");
        params.put("Version", "2012-12-01");
        CreateClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateClusterResultDeserializer::deserialize(
                        "CreateClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an Amazon Redshift parameter group.</p> <p>Creating parameter groups is independent of creating clusters. You can associate a cluster with a parameter group when you create the cluster. You can also associate an existing cluster with a parameter group after the cluster is created by using <a>ModifyCluster</a>. </p> <p>Parameters in the parameter group define specific behavior that applies to the databases you create on the cluster. For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_parameter_group(
        &self,
        input: CreateClusterParameterGroupMessage,
    ) -> RusotoFuture<CreateClusterParameterGroupResult, CreateClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateClusterParameterGroup");
        params.put("Version", "2012-12-01");
        CreateClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateClusterParameterGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateClusterParameterGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateClusterParameterGroupResultDeserializer::deserialize(
                        "CreateClusterParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new Amazon Redshift security group. You use security groups to control access to non-VPC clusters.</p> <p> For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_security_group(
        &self,
        input: CreateClusterSecurityGroupMessage,
    ) -> RusotoFuture<CreateClusterSecurityGroupResult, CreateClusterSecurityGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateClusterSecurityGroup");
        params.put("Version", "2012-12-01");
        CreateClusterSecurityGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateClusterSecurityGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateClusterSecurityGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateClusterSecurityGroupResultDeserializer::deserialize(
                        "CreateClusterSecurityGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a manual snapshot of the specified cluster. The cluster must be in the <code>available</code> state. </p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_snapshot(
        &self,
        input: CreateClusterSnapshotMessage,
    ) -> RusotoFuture<CreateClusterSnapshotResult, CreateClusterSnapshotError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateClusterSnapshot");
        params.put("Version", "2012-12-01");
        CreateClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateClusterSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateClusterSnapshotResultDeserializer::deserialize(
                        "CreateClusterSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new Amazon Redshift subnet group. You must provide a list of one or more subnets in your existing Amazon Virtual Private Cloud (Amazon VPC) when creating Amazon Redshift subnet group.</p> <p> For information about subnet groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-cluster-subnet-groups.html">Amazon Redshift Cluster Subnet Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn create_cluster_subnet_group(
        &self,
        input: CreateClusterSubnetGroupMessage,
    ) -> RusotoFuture<CreateClusterSubnetGroupResult, CreateClusterSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateClusterSubnetGroup");
        params.put("Version", "2012-12-01");
        CreateClusterSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateClusterSubnetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateClusterSubnetGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateClusterSubnetGroupResultDeserializer::deserialize(
                        "CreateClusterSubnetGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an Amazon Redshift event notification subscription. This action requires an ARN (Amazon Resource Name) of an Amazon SNS topic created by either the Amazon Redshift console, the Amazon SNS console, or the Amazon SNS API. To obtain an ARN with Amazon SNS, you must create a topic in Amazon SNS and subscribe to the topic. The ARN is displayed in the SNS console.</p> <p>You can specify the source type, and lists of Amazon Redshift source IDs, event categories, and event severities. Notifications will be sent for all events you want that match those criteria. For example, you can specify source type = cluster, source ID = my-cluster-1 and mycluster2, event categories = Availability, Backup, and severity = ERROR. The subscription will only send notifications for those ERROR events in the Availability and Backup categories for the specified clusters.</p> <p>If you specify both the source type and source IDs, such as source type = cluster and source identifier = my-cluster-1, notifications will be sent for all the cluster events for my-cluster-1. If you specify a source type but do not specify a source identifier, you will receive notice of the events for the objects of that type in your AWS account. If you do not specify either the SourceType nor the SourceIdentifier, you will be notified of events generated from all Amazon Redshift sources belonging to your AWS account. You must specify a source type if you specify a source ID.</p>
    fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> RusotoFuture<CreateEventSubscriptionResult, CreateEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateEventSubscription");
        params.put("Version", "2012-12-01");
        CreateEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateEventSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateEventSubscriptionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateEventSubscriptionResultDeserializer::deserialize(
                        "CreateEventSubscriptionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an HSM client certificate that an Amazon Redshift cluster will use to connect to the client's HSM in order to store and retrieve the keys used to encrypt the cluster databases.</p> <p>The command returns a public key, which you must store in the HSM. In addition to creating the HSM certificate, you must create an Amazon Redshift HSM configuration that provides a cluster the information needed to store and use encryption keys in the HSM. For more information, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-HSM.html">Hardware Security Modules</a> in the Amazon Redshift Cluster Management Guide.</p>
    fn create_hsm_client_certificate(
        &self,
        input: CreateHsmClientCertificateMessage,
    ) -> RusotoFuture<CreateHsmClientCertificateResult, CreateHsmClientCertificateError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateHsmClientCertificate");
        params.put("Version", "2012-12-01");
        CreateHsmClientCertificateMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHsmClientCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateHsmClientCertificateResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateHsmClientCertificateResultDeserializer::deserialize(
                        "CreateHsmClientCertificateResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an HSM configuration that contains the information required by an Amazon Redshift cluster to store and use database encryption keys in a Hardware Security Module (HSM). After creating the HSM configuration, you can specify it as a parameter when creating a cluster. The cluster will then store its encryption keys in the HSM.</p> <p>In addition to creating an HSM configuration, you must also create an HSM client certificate. For more information, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-HSM.html">Hardware Security Modules</a> in the Amazon Redshift Cluster Management Guide.</p>
    fn create_hsm_configuration(
        &self,
        input: CreateHsmConfigurationMessage,
    ) -> RusotoFuture<CreateHsmConfigurationResult, CreateHsmConfigurationError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateHsmConfiguration");
        params.put("Version", "2012-12-01");
        CreateHsmConfigurationMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHsmConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateHsmConfigurationResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateHsmConfigurationResultDeserializer::deserialize(
                        "CreateHsmConfigurationResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a snapshot copy grant that permits Amazon Redshift to use a customer master key (CMK) from AWS Key Management Service (AWS KMS) to encrypt copied snapshots in a destination region.</p> <p> For more information about managing snapshot copy grants, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-db-encryption.html">Amazon Redshift Database Encryption</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn create_snapshot_copy_grant(
        &self,
        input: CreateSnapshotCopyGrantMessage,
    ) -> RusotoFuture<CreateSnapshotCopyGrantResult, CreateSnapshotCopyGrantError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateSnapshotCopyGrant");
        params.put("Version", "2012-12-01");
        CreateSnapshotCopyGrantMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSnapshotCopyGrantError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateSnapshotCopyGrantResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateSnapshotCopyGrantResultDeserializer::deserialize(
                        "CreateSnapshotCopyGrantResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds one or more tags to a specified resource.</p> <p>A resource can have up to 10 tags. If you try to create more than 10 tags for a resource, you will receive an error and the attempt will fail.</p> <p>If you specify a key that already exists for the resource, the value for that key will be updated with the new value.</p>
    fn create_tags(&self, input: CreateTagsMessage) -> RusotoFuture<(), CreateTagsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateTags");
        params.put("Version", "2012-12-01");
        CreateTagsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a previously provisioned cluster. A successful response from the web service indicates that the request was received correctly. Use <a>DescribeClusters</a> to monitor the status of the deletion. The delete operation cannot be canceled or reverted once submitted. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you want to shut down the cluster and retain it for future use, set <i>SkipFinalClusterSnapshot</i> to <code>false</code> and specify a name for <i>FinalClusterSnapshotIdentifier</i>. You can later restore this snapshot to resume using the cluster. If a final cluster snapshot is requested, the status of the cluster will be "final-snapshot" while the snapshot is being taken, then it's "deleting" once Amazon Redshift begins deleting the cluster. </p> <p> For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterMessage,
    ) -> RusotoFuture<DeleteClusterResult, DeleteClusterError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteCluster");
        params.put("Version", "2012-12-01");
        DeleteClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteClusterResultDeserializer::deserialize(
                        "DeleteClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes a specified Amazon Redshift parameter group.</p> <note> <p>You cannot delete a parameter group if it is associated with a cluster.</p> </note></p>
    fn delete_cluster_parameter_group(
        &self,
        input: DeleteClusterParameterGroupMessage,
    ) -> RusotoFuture<(), DeleteClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteClusterParameterGroup");
        params.put("Version", "2012-12-01");
        DeleteClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClusterParameterGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an Amazon Redshift security group.</p> <note> <p>You cannot delete a security group that is associated with any clusters. You cannot delete the default security group.</p> </note> <p> For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn delete_cluster_security_group(
        &self,
        input: DeleteClusterSecurityGroupMessage,
    ) -> RusotoFuture<(), DeleteClusterSecurityGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteClusterSecurityGroup");
        params.put("Version", "2012-12-01");
        DeleteClusterSecurityGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClusterSecurityGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified manual snapshot. The snapshot must be in the <code>available</code> state, with no other users authorized to access the snapshot. </p> <p>Unlike automated snapshots, manual snapshots are retained even after you delete your cluster. Amazon Redshift does not delete your manual snapshots. You must delete manual snapshot explicitly to avoid getting charged. If other accounts are authorized to access the snapshot, you must revoke all of the authorizations before you can delete the snapshot.</p>
    fn delete_cluster_snapshot(
        &self,
        input: DeleteClusterSnapshotMessage,
    ) -> RusotoFuture<DeleteClusterSnapshotResult, DeleteClusterSnapshotError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteClusterSnapshot");
        params.put("Version", "2012-12-01");
        DeleteClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClusterSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteClusterSnapshotResultDeserializer::deserialize(
                        "DeleteClusterSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified cluster subnet group.</p>
    fn delete_cluster_subnet_group(
        &self,
        input: DeleteClusterSubnetGroupMessage,
    ) -> RusotoFuture<(), DeleteClusterSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteClusterSubnetGroup");
        params.put("Version", "2012-12-01");
        DeleteClusterSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClusterSubnetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an Amazon Redshift event notification subscription.</p>
    fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> RusotoFuture<(), DeleteEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteEventSubscription");
        params.put("Version", "2012-12-01");
        DeleteEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEventSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified HSM client certificate.</p>
    fn delete_hsm_client_certificate(
        &self,
        input: DeleteHsmClientCertificateMessage,
    ) -> RusotoFuture<(), DeleteHsmClientCertificateError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteHsmClientCertificate");
        params.put("Version", "2012-12-01");
        DeleteHsmClientCertificateMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteHsmClientCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified Amazon Redshift HSM configuration.</p>
    fn delete_hsm_configuration(
        &self,
        input: DeleteHsmConfigurationMessage,
    ) -> RusotoFuture<(), DeleteHsmConfigurationError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteHsmConfiguration");
        params.put("Version", "2012-12-01");
        DeleteHsmConfigurationMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteHsmConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified snapshot copy grant.</p>
    fn delete_snapshot_copy_grant(
        &self,
        input: DeleteSnapshotCopyGrantMessage,
    ) -> RusotoFuture<(), DeleteSnapshotCopyGrantError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteSnapshotCopyGrant");
        params.put("Version", "2012-12-01");
        DeleteSnapshotCopyGrantMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSnapshotCopyGrantError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a tag or tags from a resource. You must provide the ARN of the resource from which you want to delete the tag or tags.</p>
    fn delete_tags(&self, input: DeleteTagsMessage) -> RusotoFuture<(), DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTags");
        params.put("Version", "2012-12-01");
        DeleteTagsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of Amazon Redshift parameter groups, including parameter groups you created and the default parameter group. For each parameter group, the response includes the parameter group name, description, and parameter group family name. You can optionally specify a name to retrieve the description of a specific parameter group.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all parameter groups that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all parameter groups that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, parameter groups are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_parameter_groups(
        &self,
        input: DescribeClusterParameterGroupsMessage,
    ) -> RusotoFuture<ClusterParameterGroupsMessage, DescribeClusterParameterGroupsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeClusterParameterGroups");
        params.put("Version", "2012-12-01");
        DescribeClusterParameterGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClusterParameterGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterParameterGroupsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterParameterGroupsMessageDeserializer::deserialize(
                        "DescribeClusterParameterGroupsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a detailed list of parameters contained within the specified Amazon Redshift parameter group. For each parameter the response includes information such as parameter name, description, data type, value, whether the parameter value is modifiable, and so on.</p> <p>You can specify <i>source</i> filter to retrieve parameters of only specific type. For example, to retrieve parameters that were modified by a user action such as from <a>ModifyClusterParameterGroup</a>, you can specify <i>source</i> equal to <i>user</i>.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_cluster_parameters(
        &self,
        input: DescribeClusterParametersMessage,
    ) -> RusotoFuture<ClusterParameterGroupDetails, DescribeClusterParametersError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeClusterParameters");
        params.put("Version", "2012-12-01");
        DescribeClusterParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClusterParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterParameterGroupDetails::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterParameterGroupDetailsDeserializer::deserialize(
                        "DescribeClusterParametersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about Amazon Redshift security groups. If the name of a security group is specified, the response will contain only information about only that security group.</p> <p> For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all security groups that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all security groups that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, security groups are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_security_groups(
        &self,
        input: DescribeClusterSecurityGroupsMessage,
    ) -> RusotoFuture<ClusterSecurityGroupMessage, DescribeClusterSecurityGroupsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeClusterSecurityGroups");
        params.put("Version", "2012-12-01");
        DescribeClusterSecurityGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClusterSecurityGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterSecurityGroupMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterSecurityGroupMessageDeserializer::deserialize(
                        "DescribeClusterSecurityGroupsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns one or more snapshot objects, which contain metadata about your cluster snapshots. By default, this operation returns information about all snapshots of all clusters that are owned by you AWS customer account. No information is returned for snapshots owned by inactive AWS customer accounts.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all snapshots that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all snapshots that have any combination of those values are returned. Only snapshots that you own are returned in the response; shared snapshots are not returned with the tag key and tag value request parameters.</p> <p>If both tag keys and values are omitted from the request, snapshots are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_snapshots(
        &self,
        input: DescribeClusterSnapshotsMessage,
    ) -> RusotoFuture<SnapshotMessage, DescribeClusterSnapshotsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeClusterSnapshots");
        params.put("Version", "2012-12-01");
        DescribeClusterSnapshotsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClusterSnapshotsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SnapshotMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SnapshotMessageDeserializer::deserialize(
                        "DescribeClusterSnapshotsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns one or more cluster subnet group objects, which contain metadata about your cluster subnet groups. By default, this operation returns information about all cluster subnet groups that are defined in you AWS account.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all subnet groups that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all subnet groups that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, subnet groups are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_cluster_subnet_groups(
        &self,
        input: DescribeClusterSubnetGroupsMessage,
    ) -> RusotoFuture<ClusterSubnetGroupMessage, DescribeClusterSubnetGroupsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeClusterSubnetGroups");
        params.put("Version", "2012-12-01");
        DescribeClusterSubnetGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClusterSubnetGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterSubnetGroupMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterSubnetGroupMessageDeserializer::deserialize(
                        "DescribeClusterSubnetGroupsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns descriptions of the available Amazon Redshift cluster versions. You can call this operation even before creating any clusters to learn more about the Amazon Redshift versions. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_cluster_versions(
        &self,
        input: DescribeClusterVersionsMessage,
    ) -> RusotoFuture<ClusterVersionsMessage, DescribeClusterVersionsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeClusterVersions");
        params.put("Version", "2012-12-01");
        DescribeClusterVersionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClusterVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterVersionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterVersionsMessageDeserializer::deserialize(
                        "DescribeClusterVersionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns properties of provisioned clusters including general cluster properties, cluster database properties, maintenance and backup properties, and security and access properties. This operation supports pagination. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all clusters that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all clusters that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, clusters are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersMessage,
    ) -> RusotoFuture<ClustersMessage, DescribeClustersError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeClusters");
        params.put("Version", "2012-12-01");
        DescribeClustersMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClustersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClustersMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClustersMessageDeserializer::deserialize(
                        "DescribeClustersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of parameter settings for the specified parameter group family.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_default_cluster_parameters(
        &self,
        input: DescribeDefaultClusterParametersMessage,
    ) -> RusotoFuture<DescribeDefaultClusterParametersResult, DescribeDefaultClusterParametersError>
    {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeDefaultClusterParameters");
        params.put("Version", "2012-12-01");
        DescribeDefaultClusterParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDefaultClusterParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeDefaultClusterParametersResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeDefaultClusterParametersResultDeserializer::deserialize(
                            "DescribeDefaultClusterParametersResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Displays a list of event categories for all event source types, or for a specified source type. For a list of the event categories and source types, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-event-notifications.html">Amazon Redshift Event Notifications</a>.</p>
    fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> RusotoFuture<EventCategoriesMessage, DescribeEventCategoriesError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEventCategories");
        params.put("Version", "2012-12-01");
        DescribeEventCategoriesMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventCategoriesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EventCategoriesMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventCategoriesMessageDeserializer::deserialize(
                        "DescribeEventCategoriesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists descriptions of all the Amazon Redshift event notification subscriptions for a customer account. If you specify a subscription name, lists the description for that subscription.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all event notification subscriptions that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all subscriptions that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, subscriptions are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> RusotoFuture<EventSubscriptionsMessage, DescribeEventSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEventSubscriptions");
        params.put("Version", "2012-12-01");
        DescribeEventSubscriptionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventSubscriptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EventSubscriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventSubscriptionsMessageDeserializer::deserialize(
                        "DescribeEventSubscriptionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns events related to clusters, security groups, snapshots, and parameter groups for the past 14 days. Events specific to a particular cluster, security group, snapshot or parameter group can be obtained by providing the name as a parameter. By default, the past hour of events are returned.</p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<EventsMessage, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEvents");
        params.put("Version", "2012-12-01");
        DescribeEventsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EventsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventsMessageDeserializer::deserialize(
                        "DescribeEventsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified HSM client certificate. If no certificate ID is specified, returns information about all the HSM certificates owned by your AWS customer account.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all HSM client certificates that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all HSM client certificates that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, HSM client certificates are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_hsm_client_certificates(
        &self,
        input: DescribeHsmClientCertificatesMessage,
    ) -> RusotoFuture<HsmClientCertificateMessage, DescribeHsmClientCertificatesError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeHsmClientCertificates");
        params.put("Version", "2012-12-01");
        DescribeHsmClientCertificatesMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeHsmClientCertificatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = HsmClientCertificateMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(HsmClientCertificateMessageDeserializer::deserialize(
                        "DescribeHsmClientCertificatesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified Amazon Redshift HSM configuration. If no configuration ID is specified, returns information about all the HSM configurations owned by your AWS customer account.</p> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all HSM connections that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all HSM connections that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, HSM connections are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_hsm_configurations(
        &self,
        input: DescribeHsmConfigurationsMessage,
    ) -> RusotoFuture<HsmConfigurationMessage, DescribeHsmConfigurationsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeHsmConfigurations");
        params.put("Version", "2012-12-01");
        DescribeHsmConfigurationsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeHsmConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = HsmConfigurationMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(HsmConfigurationMessageDeserializer::deserialize(
                        "DescribeHsmConfigurationsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes whether information, such as queries and connection attempts, is being logged for the specified Amazon Redshift cluster.</p>
    fn describe_logging_status(
        &self,
        input: DescribeLoggingStatusMessage,
    ) -> RusotoFuture<LoggingStatus, DescribeLoggingStatusError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoggingStatus");
        params.put("Version", "2012-12-01");
        DescribeLoggingStatusMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLoggingStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = LoggingStatus::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(LoggingStatusDeserializer::deserialize(
                        "DescribeLoggingStatusResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of orderable cluster options. Before you create a new cluster you can use this operation to find what options are available, such as the EC2 Availability Zones (AZ) in the specific AWS region that you can specify, and the node types you can request. The node types differ by available storage, memory, CPU and price. With the cost involved you might want to obtain a list of cluster options in the specific region and specify values when creating a cluster. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_orderable_cluster_options(
        &self,
        input: DescribeOrderableClusterOptionsMessage,
    ) -> RusotoFuture<OrderableClusterOptionsMessage, DescribeOrderableClusterOptionsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeOrderableClusterOptions");
        params.put("Version", "2012-12-01");
        DescribeOrderableClusterOptionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeOrderableClusterOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = OrderableClusterOptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(OrderableClusterOptionsMessageDeserializer::deserialize(
                        "DescribeOrderableClusterOptionsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of the available reserved node offerings by Amazon Redshift with their descriptions including the node type, the fixed and recurring costs of reserving the node and duration the node will be reserved for you. These descriptions help you determine which reserve node offering you want to purchase. You then use the unique offering ID in you call to <a>PurchaseReservedNodeOffering</a> to reserve one or more nodes for your Amazon Redshift cluster. </p> <p> For more information about reserved node offerings, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/purchase-reserved-node-instance.html">Purchasing Reserved Nodes</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn describe_reserved_node_offerings(
        &self,
        input: DescribeReservedNodeOfferingsMessage,
    ) -> RusotoFuture<ReservedNodeOfferingsMessage, DescribeReservedNodeOfferingsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReservedNodeOfferings");
        params.put("Version", "2012-12-01");
        DescribeReservedNodeOfferingsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReservedNodeOfferingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ReservedNodeOfferingsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ReservedNodeOfferingsMessageDeserializer::deserialize(
                        "DescribeReservedNodeOfferingsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the descriptions of the reserved nodes.</p>
    fn describe_reserved_nodes(
        &self,
        input: DescribeReservedNodesMessage,
    ) -> RusotoFuture<ReservedNodesMessage, DescribeReservedNodesError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReservedNodes");
        params.put("Version", "2012-12-01");
        DescribeReservedNodesMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReservedNodesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ReservedNodesMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ReservedNodesMessageDeserializer::deserialize(
                        "DescribeReservedNodesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the last resize operation for the specified cluster. If no resize operation has ever been initiated for the specified cluster, a <code>HTTP 404</code> error is returned. If a resize operation was initiated and completed, the status of the resize remains as <code>SUCCEEDED</code> until the next resize. </p> <p>A resize operation can be requested using <a>ModifyCluster</a> and specifying a different number or type of nodes for the cluster. </p>
    fn describe_resize(
        &self,
        input: DescribeResizeMessage,
    ) -> RusotoFuture<ResizeProgressMessage, DescribeResizeError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeResize");
        params.put("Version", "2012-12-01");
        DescribeResizeMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeResizeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ResizeProgressMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ResizeProgressMessageDeserializer::deserialize(
                        "DescribeResizeResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of snapshot copy grants owned by the AWS account in the destination region.</p> <p> For more information about managing snapshot copy grants, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-db-encryption.html">Amazon Redshift Database Encryption</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn describe_snapshot_copy_grants(
        &self,
        input: DescribeSnapshotCopyGrantsMessage,
    ) -> RusotoFuture<SnapshotCopyGrantMessage, DescribeSnapshotCopyGrantsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeSnapshotCopyGrants");
        params.put("Version", "2012-12-01");
        DescribeSnapshotCopyGrantsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSnapshotCopyGrantsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SnapshotCopyGrantMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SnapshotCopyGrantMessageDeserializer::deserialize(
                        "DescribeSnapshotCopyGrantsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the status of one or more table restore requests made using the <a>RestoreTableFromClusterSnapshot</a> API action. If you don't specify a value for the <code>TableRestoreRequestId</code> parameter, then <code>DescribeTableRestoreStatus</code> returns the status of all table restore requests ordered by the date and time of the request in ascending order. Otherwise <code>DescribeTableRestoreStatus</code> returns the status of the table specified by <code>TableRestoreRequestId</code>.</p>
    fn describe_table_restore_status(
        &self,
        input: DescribeTableRestoreStatusMessage,
    ) -> RusotoFuture<TableRestoreStatusMessage, DescribeTableRestoreStatusError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTableRestoreStatus");
        params.put("Version", "2012-12-01");
        DescribeTableRestoreStatusMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTableRestoreStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = TableRestoreStatusMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TableRestoreStatusMessageDeserializer::deserialize(
                        "DescribeTableRestoreStatusResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of tags. You can return tags from a specific resource by specifying an ARN, or you can return all tags for a given type of resource, such as clusters, snapshots, and so on.</p> <p>The following are limitations for <code>DescribeTags</code>: </p> <ul> <li> <p>You cannot specify an ARN and a resource-type value together in the same request.</p> </li> <li> <p>You cannot use the <code>MaxRecords</code> and <code>Marker</code> parameters together with the ARN parameter.</p> </li> <li> <p>The <code>MaxRecords</code> parameter can be a range from 10 to 50 results to return in a request.</p> </li> </ul> <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all resources that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all resources that have any combination of those values are returned.</p> <p>If both tag keys and values are omitted from the request, resources are returned regardless of whether they have tag keys or values associated with them.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsMessage,
    ) -> RusotoFuture<TaggedResourceListMessage, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTags");
        params.put("Version", "2012-12-01");
        DescribeTagsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = TaggedResourceListMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TaggedResourceListMessageDeserializer::deserialize(
                        "DescribeTagsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops logging information, such as queries and connection attempts, for the specified Amazon Redshift cluster.</p>
    fn disable_logging(
        &self,
        input: DisableLoggingMessage,
    ) -> RusotoFuture<LoggingStatus, DisableLoggingError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableLogging");
        params.put("Version", "2012-12-01");
        DisableLoggingMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableLoggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = LoggingStatus::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(LoggingStatusDeserializer::deserialize(
                        "DisableLoggingResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables the automatic copying of snapshots from one region to another region for a specified cluster.</p> <p>If your cluster and its snapshots are encrypted using a customer master key (CMK) from AWS KMS, use <a>DeleteSnapshotCopyGrant</a> to delete the grant that grants Amazon Redshift permission to the CMK in the destination region. </p>
    fn disable_snapshot_copy(
        &self,
        input: DisableSnapshotCopyMessage,
    ) -> RusotoFuture<DisableSnapshotCopyResult, DisableSnapshotCopyError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableSnapshotCopy");
        params.put("Version", "2012-12-01");
        DisableSnapshotCopyMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableSnapshotCopyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DisableSnapshotCopyResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DisableSnapshotCopyResultDeserializer::deserialize(
                        "DisableSnapshotCopyResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts logging information, such as queries and connection attempts, for the specified Amazon Redshift cluster.</p>
    fn enable_logging(
        &self,
        input: EnableLoggingMessage,
    ) -> RusotoFuture<LoggingStatus, EnableLoggingError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableLogging");
        params.put("Version", "2012-12-01");
        EnableLoggingMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableLoggingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = LoggingStatus::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(LoggingStatusDeserializer::deserialize(
                        "EnableLoggingResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables the automatic copy of snapshots from one region to another region for a specified cluster.</p>
    fn enable_snapshot_copy(
        &self,
        input: EnableSnapshotCopyMessage,
    ) -> RusotoFuture<EnableSnapshotCopyResult, EnableSnapshotCopyError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableSnapshotCopy");
        params.put("Version", "2012-12-01");
        EnableSnapshotCopyMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableSnapshotCopyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnableSnapshotCopyResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EnableSnapshotCopyResultDeserializer::deserialize(
                        "EnableSnapshotCopyResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a database user name and temporary password with temporary authorization to log on to an Amazon Redshift database. The action returns the database user name prefixed with <code>IAM:</code> if <code>AutoCreate</code> is <code>False</code> or <code>IAMA:</code> if <code>AutoCreate</code> is <code>True</code>. You can optionally specify one or more database user groups that the user will join at log on. By default, the temporary credentials expire in 900 seconds. You can optionally specify a duration between 900 seconds (15 minutes) and 3600 seconds (60 minutes). For more information, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/generating-user-credentials.html">Using IAM Authentication to Generate Database User Credentials</a> in the Amazon Redshift Cluster Management Guide.</p> <p>The AWS Identity and Access Management (IAM)user or role that executes GetClusterCredentials must have an IAM policy attached that allows access to all necessary actions and resources. For more information about permissions, see <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/redshift-iam-access-control-identity-based.html#redshift-policy-resources.getclustercredentials-resources">Resource Policies for GetClusterCredentials</a> in the Amazon Redshift Cluster Management Guide.</p> <p>If the <code>DbGroups</code> parameter is specified, the IAM policy must allow the <code>redshift:JoinGroup</code> action with access to the listed <code>dbgroups</code>. </p> <p>In addition, if the <code>AutoCreate</code> parameter is set to <code>True</code>, then the policy must include the <code>redshift:CreateClusterUser</code> privilege.</p> <p>If the <code>DbName</code> parameter is specified, the IAM policy must allow access to the resource <code>dbname</code> for the specified database name. </p>
    fn get_cluster_credentials(
        &self,
        input: GetClusterCredentialsMessage,
    ) -> RusotoFuture<ClusterCredentials, GetClusterCredentialsError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetClusterCredentials");
        params.put("Version", "2012-12-01");
        GetClusterCredentialsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetClusterCredentialsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterCredentials::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterCredentialsDeserializer::deserialize(
                        "GetClusterCredentialsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the settings for a cluster. For example, you can add another security or parameter group, update the preferred maintenance window, or change the master user password. Resetting a cluster password or modifying the security groups associated with a cluster do not need a reboot. However, modifying a parameter group requires a reboot for parameters to take effect. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p> <p>You can also change node type and the number of nodes to scale up or down the cluster. When resizing a cluster, you must specify both the number of nodes and the node type even if one of the parameters does not change.</p>
    fn modify_cluster(
        &self,
        input: ModifyClusterMessage,
    ) -> RusotoFuture<ModifyClusterResult, ModifyClusterError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyCluster");
        params.put("Version", "2012-12-01");
        ModifyClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyClusterResultDeserializer::deserialize(
                        "ModifyClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the list of AWS Identity and Access Management (IAM) roles that can be used by the cluster to access other AWS services.</p> <p>A cluster can have up to 10 IAM roles associated at any time.</p>
    fn modify_cluster_iam_roles(
        &self,
        input: ModifyClusterIamRolesMessage,
    ) -> RusotoFuture<ModifyClusterIamRolesResult, ModifyClusterIamRolesError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyClusterIamRoles");
        params.put("Version", "2012-12-01");
        ModifyClusterIamRolesMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyClusterIamRolesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyClusterIamRolesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyClusterIamRolesResultDeserializer::deserialize(
                        "ModifyClusterIamRolesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the parameters of a parameter group.</p> <p> For more information about parameters and parameter groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-parameter-groups.html">Amazon Redshift Parameter Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn modify_cluster_parameter_group(
        &self,
        input: ModifyClusterParameterGroupMessage,
    ) -> RusotoFuture<ClusterParameterGroupNameMessage, ModifyClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyClusterParameterGroup");
        params.put("Version", "2012-12-01");
        ModifyClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyClusterParameterGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterParameterGroupNameMessageDeserializer::deserialize(
                        "ModifyClusterParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies a cluster subnet group to include the specified list of VPC subnets. The operation replaces the existing list of subnets with the new list of subnets.</p>
    fn modify_cluster_subnet_group(
        &self,
        input: ModifyClusterSubnetGroupMessage,
    ) -> RusotoFuture<ModifyClusterSubnetGroupResult, ModifyClusterSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyClusterSubnetGroup");
        params.put("Version", "2012-12-01");
        ModifyClusterSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyClusterSubnetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyClusterSubnetGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyClusterSubnetGroupResultDeserializer::deserialize(
                        "ModifyClusterSubnetGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies an existing Amazon Redshift event notification subscription.</p>
    fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> RusotoFuture<ModifyEventSubscriptionResult, ModifyEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyEventSubscription");
        params.put("Version", "2012-12-01");
        ModifyEventSubscriptionMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyEventSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyEventSubscriptionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyEventSubscriptionResultDeserializer::deserialize(
                        "ModifyEventSubscriptionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the number of days to retain automated snapshots in the destination region after they are copied from the source region.</p>
    fn modify_snapshot_copy_retention_period(
        &self,
        input: ModifySnapshotCopyRetentionPeriodMessage,
    ) -> RusotoFuture<ModifySnapshotCopyRetentionPeriodResult, ModifySnapshotCopyRetentionPeriodError>
    {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifySnapshotCopyRetentionPeriod");
        params.put("Version", "2012-12-01");
        ModifySnapshotCopyRetentionPeriodMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifySnapshotCopyRetentionPeriodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifySnapshotCopyRetentionPeriodResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ModifySnapshotCopyRetentionPeriodResultDeserializer::deserialize(
                            "ModifySnapshotCopyRetentionPeriodResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Allows you to purchase reserved nodes. Amazon Redshift offers a predefined set of reserved node offerings. You can purchase one or more of the offerings. You can call the <a>DescribeReservedNodeOfferings</a> API to obtain the available reserved node offerings. You can call this API by providing a specific reserved node offering and the number of nodes you want to reserve. </p> <p> For more information about reserved node offerings, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/purchase-reserved-node-instance.html">Purchasing Reserved Nodes</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn purchase_reserved_node_offering(
        &self,
        input: PurchaseReservedNodeOfferingMessage,
    ) -> RusotoFuture<PurchaseReservedNodeOfferingResult, PurchaseReservedNodeOfferingError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PurchaseReservedNodeOffering");
        params.put("Version", "2012-12-01");
        PurchaseReservedNodeOfferingMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PurchaseReservedNodeOfferingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PurchaseReservedNodeOfferingResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PurchaseReservedNodeOfferingResultDeserializer::deserialize(
                        "PurchaseReservedNodeOfferingResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Reboots a cluster. This action is taken as soon as possible. It results in a momentary outage to the cluster, during which the cluster status is set to <code>rebooting</code>. A cluster event is created when the reboot is completed. Any pending cluster modifications (see <a>ModifyCluster</a>) are applied at this reboot. For more information about managing clusters, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html">Amazon Redshift Clusters</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn reboot_cluster(
        &self,
        input: RebootClusterMessage,
    ) -> RusotoFuture<RebootClusterResult, RebootClusterError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RebootCluster");
        params.put("Version", "2012-12-01");
        RebootClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RebootClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RebootClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RebootClusterResultDeserializer::deserialize(
                        "RebootClusterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets one or more parameters of the specified parameter group to their default values and sets the source values of the parameters to "engine-default". To reset the entire parameter group specify the <i>ResetAllParameters</i> parameter. For parameter changes to take effect you must reboot any associated clusters. </p>
    fn reset_cluster_parameter_group(
        &self,
        input: ResetClusterParameterGroupMessage,
    ) -> RusotoFuture<ClusterParameterGroupNameMessage, ResetClusterParameterGroupError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResetClusterParameterGroup");
        params.put("Version", "2012-12-01");
        ResetClusterParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResetClusterParameterGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ClusterParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ClusterParameterGroupNameMessageDeserializer::deserialize(
                        "ResetClusterParameterGroupResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new cluster from a snapshot. By default, Amazon Redshift creates the resulting cluster with the same configuration as the original cluster from which the snapshot was created, except that the new cluster is created with the default cluster security and parameter groups. After Amazon Redshift creates the cluster, you can use the <a>ModifyCluster</a> API to associate a different security group and different parameter group with the restored cluster. If you are using a DS node type, you can also choose to change to another DS node type of the same size during restore.</p> <p>If you restore a cluster into a VPC, you must provide a cluster subnet group where you want the cluster restored.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn restore_from_cluster_snapshot(
        &self,
        input: RestoreFromClusterSnapshotMessage,
    ) -> RusotoFuture<RestoreFromClusterSnapshotResult, RestoreFromClusterSnapshotError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RestoreFromClusterSnapshot");
        params.put("Version", "2012-12-01");
        RestoreFromClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RestoreFromClusterSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RestoreFromClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RestoreFromClusterSnapshotResultDeserializer::deserialize(
                        "RestoreFromClusterSnapshotResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new table from a table in an Amazon Redshift cluster snapshot. You must create the new table within the Amazon Redshift cluster that the snapshot was taken from.</p> <p>You cannot use <code>RestoreTableFromClusterSnapshot</code> to restore a table with the same name as an existing table in an Amazon Redshift cluster. That is, you cannot overwrite an existing table in a cluster with a restored table. If you want to replace your original table with a new, restored table, then rename or drop your original table before you call <code>RestoreTableFromClusterSnapshot</code>. When you have renamed your original table, then you can pass the original name of the table as the <code>NewTableName</code> parameter value in the call to <code>RestoreTableFromClusterSnapshot</code>. This way, you can replace the original table with the table created from the snapshot.</p>
    fn restore_table_from_cluster_snapshot(
        &self,
        input: RestoreTableFromClusterSnapshotMessage,
    ) -> RusotoFuture<RestoreTableFromClusterSnapshotResult, RestoreTableFromClusterSnapshotError>
    {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RestoreTableFromClusterSnapshot");
        params.put("Version", "2012-12-01");
        RestoreTableFromClusterSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RestoreTableFromClusterSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RestoreTableFromClusterSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        RestoreTableFromClusterSnapshotResultDeserializer::deserialize(
                            "RestoreTableFromClusterSnapshotResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Revokes an ingress rule in an Amazon Redshift security group for a previously authorized IP range or Amazon EC2 security group. To add an ingress rule, see <a>AuthorizeClusterSecurityGroupIngress</a>. For information about managing security groups, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    fn revoke_cluster_security_group_ingress(
        &self,
        input: RevokeClusterSecurityGroupIngressMessage,
    ) -> RusotoFuture<RevokeClusterSecurityGroupIngressResult, RevokeClusterSecurityGroupIngressError>
    {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RevokeClusterSecurityGroupIngress");
        params.put("Version", "2012-12-01");
        RevokeClusterSecurityGroupIngressMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RevokeClusterSecurityGroupIngressError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RevokeClusterSecurityGroupIngressResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        RevokeClusterSecurityGroupIngressResultDeserializer::deserialize(
                            "RevokeClusterSecurityGroupIngressResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the ability of the specified AWS customer account to restore the specified snapshot. If the account is currently restoring the snapshot, the restore will run to completion.</p> <p> For more information about working with snapshots, go to <a href="http://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
    fn revoke_snapshot_access(
        &self,
        input: RevokeSnapshotAccessMessage,
    ) -> RusotoFuture<RevokeSnapshotAccessResult, RevokeSnapshotAccessError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RevokeSnapshotAccess");
        params.put("Version", "2012-12-01");
        RevokeSnapshotAccessMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RevokeSnapshotAccessError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RevokeSnapshotAccessResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RevokeSnapshotAccessResultDeserializer::deserialize(
                        "RevokeSnapshotAccessResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Rotates the encryption keys for a cluster.</p>
    fn rotate_encryption_key(
        &self,
        input: RotateEncryptionKeyMessage,
    ) -> RusotoFuture<RotateEncryptionKeyResult, RotateEncryptionKeyError> {
        let mut request = SignedRequest::new("POST", "redshift", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RotateEncryptionKey");
        params.put("Version", "2012-12-01");
        RotateEncryptionKeyMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RotateEncryptionKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RotateEncryptionKeyResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RotateEncryptionKeyResultDeserializer::deserialize(
                        "RotateEncryptionKeyResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_valid_redshift_authorize_cluster_security_group_ingress() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-authorize-cluster-security-group-ingress.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = AuthorizeClusterSecurityGroupIngressMessage::default();
        let result = client
            .authorize_cluster_security_group_ingress(request)
            .sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_copy_cluster_snapshot() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-copy-cluster-snapshot.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CopyClusterSnapshotMessage::default();
        let result = client.copy_cluster_snapshot(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_create_cluster_parameter_group() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-create-cluster-parameter-group.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateClusterParameterGroupMessage::default();
        let result = client.create_cluster_parameter_group(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_create_cluster_security_group() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-create-cluster-security-group.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateClusterSecurityGroupMessage::default();
        let result = client.create_cluster_security_group(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_create_cluster_snapshot() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-create-cluster-snapshot.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateClusterSnapshotMessage::default();
        let result = client.create_cluster_snapshot(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_create_cluster_subnet_group() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-create-cluster-subnet-group.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateClusterSubnetGroupMessage::default();
        let result = client.create_cluster_subnet_group(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_create_cluster() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-create-cluster.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateClusterMessage::default();
        let result = client.create_cluster(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_delete_cluster_parameter_group() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-delete-cluster-parameter-group.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteClusterParameterGroupMessage::default();
        let result = client.delete_cluster_parameter_group(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_delete_cluster_snapshot() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-delete-cluster-snapshot.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteClusterSnapshotMessage::default();
        let result = client.delete_cluster_snapshot(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_delete_cluster() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-delete-cluster.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteClusterMessage::default();
        let result = client.delete_cluster(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_cluster_parameter_groups() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-cluster-parameter-groups.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeClusterParameterGroupsMessage::default();
        let result = client.describe_cluster_parameter_groups(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_cluster_parameters() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-cluster-parameters.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeClusterParametersMessage::default();
        let result = client.describe_cluster_parameters(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_cluster_security_groups() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-cluster-security-groups.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeClusterSecurityGroupsMessage::default();
        let result = client.describe_cluster_security_groups(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_cluster_snapshots() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-cluster-snapshots.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeClusterSnapshotsMessage::default();
        let result = client.describe_cluster_snapshots(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_cluster_subnet_groups() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-cluster-subnet-groups.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeClusterSubnetGroupsMessage::default();
        let result = client.describe_cluster_subnet_groups(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_cluster_versions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-cluster-versions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeClusterVersionsMessage::default();
        let result = client.describe_cluster_versions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_clusters() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-clusters.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeClustersMessage::default();
        let result = client.describe_clusters(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_events() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-events.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeEventsMessage::default();
        let result = client.describe_events(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_orderable_cluster_options() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-orderable-cluster-options.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeOrderableClusterOptionsMessage::default();
        let result = client.describe_orderable_cluster_options(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_reserved_node_offerings() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-reserved-node-offerings.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeReservedNodeOfferingsMessage::default();
        let result = client.describe_reserved_node_offerings(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_reserved_nodes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-reserved-nodes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeReservedNodesMessage::default();
        let result = client.describe_reserved_nodes(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_describe_resize() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-describe-resize.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeResizeMessage::default();
        let result = client.describe_resize(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_modify_cluster_parameter_group() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-modify-cluster-parameter-group.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ModifyClusterParameterGroupMessage::default();
        let result = client.modify_cluster_parameter_group(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_purchase_reserved_node_offering() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-purchase-reserved-node-offering.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = PurchaseReservedNodeOfferingMessage::default();
        let result = client.purchase_reserved_node_offering(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_reboot_cluster() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-reboot-cluster.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = RebootClusterMessage::default();
        let result = client.reboot_cluster(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_reset_cluster_parameter_group() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-reset-cluster-parameter-group.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ResetClusterParameterGroupMessage::default();
        let result = client.reset_cluster_parameter_group(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_restore_from_cluster_snapshot() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-restore-from-cluster-snapshot.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = RestoreFromClusterSnapshotMessage::default();
        let result = client.restore_from_cluster_snapshot(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_redshift_revoke_cluster_security_group_ingress() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "redshift-revoke-cluster-security-group-ingress.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = RevokeClusterSecurityGroupIngressMessage::default();
        let result = client.revoke_cluster_security_group_ingress(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
